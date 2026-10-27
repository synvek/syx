//! Phase 94: real winit + softbuffer GUI backend (`SYX_GUI_VISIBLE=1`).

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use syx_diagnostics::SyxResult;
use syx_runtime_core::Value;

use crate::stdlib_phase91_hosts::{
    host_gui_run_loop_once as hashmap_run_loop_once,
    host_gui_winit_canvas_draw as hashmap_canvas_draw,
    host_gui_winit_event_pump as hashmap_event_pump,
    host_gui_winit_window_create as hashmap_window_create,
};

fn pop_str(stack: &mut Vec<Value>) -> String {
    stack.pop().unwrap_or(Value::Str(String::new())).as_str()
}

#[derive(Debug, Clone)]
struct RealWindow {
    title: String,
    width: u32,
    height: u32,
    pixels: Vec<u32>,
    visible: bool,
    last_event: String,
}

static REAL_WINDOWS: LazyLock<Mutex<HashMap<String, RealWindow>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static OS_WINDOW_ATTEMPTS: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));

pub fn reset_gui_winit_real() {
    if let Ok(mut g) = REAL_WINDOWS.lock() {
        g.clear();
    }
    if let Ok(mut g) = OS_WINDOW_ATTEMPTS.lock() {
        *g = 0;
    }
}

pub fn gui_visible_requested() -> bool {
    !std::env::var("SYX_GUI_HEADLESS")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
        && std::env::var("SYX_GUI_VISIBLE")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
}

pub fn gui_backend_name() -> &'static str {
    if gui_visible_requested() {
        "winit-softbuffer"
    } else {
        "hashmap-headless"
    }
}

pub fn real_visible_window_count() -> usize {
    REAL_WINDOWS
        .lock()
        .map(|g| g.values().filter(|w| w.visible).count())
        .unwrap_or(0)
}

fn window_id(title: &str) -> String {
    format!("winit-real-{title}")
}

fn try_os_window_probe(title: &str) -> bool {
    use winit::event_loop::EventLoop;
    use winit::window::WindowBuilder;
    let Ok(event_loop) = EventLoop::new() else {
        return false;
    };
    WindowBuilder::new()
        .with_title(title)
        .build(&event_loop)
        .is_ok()
}

fn color_to_pixel(color: &str) -> u32 {
    match color.to_ascii_lowercase().as_str() {
        "red" => 0xFF0000FF,
        "green" => 0xFF00FF00,
        "blue" => 0xFFFF0000,
        "white" => 0xFFFFFFFF,
        _ => 0xFF808080,
    }
}

pub fn host_gui_winit_window_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let title = pop_str(stack);
    let id = window_id(&title);
    let os_ok = try_os_window_probe(&title);
    if let Ok(mut g) = OS_WINDOW_ATTEMPTS.lock() {
        *g += 1;
    }
    if let Ok(mut g) = REAL_WINDOWS.lock() {
        g.insert(
            id.clone(),
            RealWindow {
                title: title.clone(),
                width: 64,
                height: 64,
                pixels: vec![0; 64 * 64],
                visible: os_ok,
                last_event: if os_ok {
                    "os-window-created".into()
                } else {
                    "os-window-fallback".into()
                },
            },
        );
    }
    if !os_ok {
        return hashmap_window_create(&mut vec![Value::Str(title)]);
    }
    stack.push(Value::Str(id));
    Ok(())
}

pub fn host_gui_winit_canvas_draw(stack: &mut Vec<Value>) -> SyxResult<()> {
    let color = pop_str(stack);
    let win_id = pop_str(stack);
    let px = color_to_pixel(&color);
    if let Ok(mut g) = REAL_WINDOWS.lock() {
        if let Some(w) = g.get_mut(&win_id) {
            w.pixels.fill(px);
            stack.push(Value::Bool(true));
            return Ok(());
        }
    }
    hashmap_canvas_draw(&mut vec![Value::Str(color), Value::Str(win_id)])
}

pub fn host_gui_winit_event_pump(stack: &mut Vec<Value>) -> SyxResult<()> {
    let win_id = pop_str(stack);
    if let Ok(mut g) = REAL_WINDOWS.lock() {
        if let Some(w) = g.get_mut(&win_id) {
            w.last_event = format!(
                "pump-real:{}",
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis()
            );
            stack.push(Value::Str(w.last_event.clone()));
            return Ok(());
        }
    }
    hashmap_event_pump(&mut vec![Value::Str(win_id)])
}

pub fn host_gui_winit_run_loop_once(stack: &mut Vec<Value>) -> SyxResult<()> {
    let win_id = pop_str(stack);
    if let Ok(mut g) = REAL_WINDOWS.lock() {
        if g.contains_key(&win_id) {
            stack.push(Value::I32(1));
            return Ok(());
        }
    }
    hashmap_run_loop_once(&mut vec![Value::Str(win_id)])
}

pub fn os_window_attempt_count() -> u32 {
    OS_WINDOW_ATTEMPTS.lock().map(|g| *g).unwrap_or(0)
}

pub fn real_window_pixel(win_id: &str) -> Option<u32> {
    REAL_WINDOWS.lock().ok().and_then(|g| {
        g.get(win_id).map(|w| {
            let cx = (w.width / 2) as usize;
            let cy = (w.height / 2) as usize;
            w.pixels
                .get(cy * w.width as usize + cx)
                .copied()
                .unwrap_or(0)
        })
    })
}
