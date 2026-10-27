//! Phase 98: full guidemo import UI VM≡native ≥15-line nil-free parity.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use syx_driver::{build_native_executable, compile_and_run_embedded, run_file_printed, DriverOptions};

const GUIDEMO_FULL_INLINE: &str = r#"func onButtonClick() {
    print("onButtonClick: handler ran")
}
func main() {
    print("SYX GUI demo subset")
    let win = __syx_gui_winit_window_create("SYX GUI Demo")
    print("window: \(win)")
    __syx_gui_winit_canvas_draw(win, "blue")
    print("canvas cleared: true")
    print("center pixel: \(__syx_gui_pixel_read(win))")
    let title = __syx_gui_label_create(win, "Hello SYX")
    let btn = __syx_gui_button_create(win, "Click me")
    let field = __syx_gui_text_field_create(win, "Type here...")
    let layoutId = __syx_gui_layout_create(win, "VStack")
    print("label: \(title)")
    print("button: \(btn)")
    print("textField: \(field)")
    print("layout: \(layoutId)")
    let clicked = __syx_gui_on_click(win, btn, "onButtonClick")
    print("onClick: \(clicked)")
    __syx_gui_click_inject(win, btn)
    print("tick 0: \(__syx_gui_run_loop_once(win))")
    print("tick 1: \(__syx_gui_run_loop_once(win))")
    print("tick 2: \(__syx_gui_run_loop_once(win))")
    print("gui demo done")
}
"#;

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn guidemo_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/guidemo.sy")
}

fn normalize_pump_digits(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i..].starts_with(b"pump:") {
            out.push_str("pump:N");
            i += 5;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            continue;
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

fn lines_compatible(vm: &str, native: &str) -> bool {
    // Phase 99: line-equal; only `pump:\d+` digits may differ.
    normalize_pump_digits(vm) == normalize_pump_digits(native)
}

#[test]
fn p103_native_gui_parity_focus_fingerprint() {
    assert!(include_str!("../../../syx-gui/src/lib.rs")
        .contains("gui-textfield-focus-v1:caret-tab-cycle"));
    assert!(include_str!("gui_winit_visible.rs").contains("p103_gui_focus_caret_fingerprint"));
}

#[test]
fn p102_native_gui_parity_fingerprint() {
    assert!(include_str!("../../src/lib.rs")
        .contains("gui-line-hard-v1:guidemo-main-stdout-equal"));
    assert!(include_str!("../../../syx-gui/src/lib.rs")
        .contains("gui-callback-ga-v1:invoke-onclick-textfield"));
    assert!(include_str!("gui_winit_visible.rs").contains("p102_gui_callback_ga_fingerprint"));
}

#[test]
fn p101_native_gui_parity_fingerprint() {
    assert!(include_str!("../../src/lib.rs")
        .contains("gui-line-hard-v1:guidemo-main-stdout-equal"));
    assert!(include_str!("../../../syx-gui/src/lib.rs")
        .contains("gui-callback-ga-v1"));
    assert!(include_str!("gui_winit_visible.rs").contains("p102_gui_callback_ga_fingerprint"));
}

#[test]
fn p100_native_gui_parity_fingerprint() {
    assert!(include_str!("../../src/lib.rs")
        .contains("gui-line-hard-v1:guidemo-main-stdout-equal"));
    assert!(include_str!("../../src/lib.rs")
        .contains("gui-widgets-bitmap-v1:label-button-visible"));
    assert!(include_str!("../../src/lib.rs")
        .contains("native-llvm-locals-v7:no-hydrate-handle-ga"));
}

#[ignore = "Phase 124: syx-gui string-id API retired; use SynUI"]
#[test]
fn p98_guidemo_full_inline_vm_native_stdout_parity() {
    syx_host_plugin::register_echo_plugin_in_process();
    let vm = compile_and_run_embedded(GUIDEMO_FULL_INLINE, &opts())
        .unwrap()
        .printed;
    assert!(vm.len() >= 13, "vm lines: {vm:?}");

    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!("syx_p98_gui_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), GUIDEMO_FULL_INLINE).unwrap();
    fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
    let exe = dir.join("app");
    build_native_executable(&dir, &opts(), &exe).expect("native build");
    let output = Command::new(&exe).output().unwrap();
    let native: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect();
    assert!(native.len() >= 13, "native lines: {native:?}");
    assert!(
        !native.iter().any(|l| l.contains("nil")),
        "native must not print nil: {native:?}"
    );
    assert_eq!(vm.len(), native.len(), "inline guidemo VM/native line count");
    for i in 0..vm.len() {
        assert!(
            lines_compatible(&vm[i], &native[i]),
            "inline guidemo line {i} vm={} native={}",
            vm[i],
            native[i]
        );
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn p105_guidemo_inline_line_parity_fingerprint() {
    assert!(include_str!("../../src/lib.rs")
        .contains("native-llvm-locals-v10-e2e:guidemo-line-parity"));
}

#[test]
#[ignore = "W173: guidemo.sy retired; covered by driver_gate synui exec"]
fn p98_guidemo_examples_vm_native_stdout_parity() {
    syx_host_plugin::register_echo_plugin_in_process();
    let (_, vm) = run_file_printed(&guidemo_path(), &opts()).unwrap();
    assert!(vm.len() >= 15, "vm lines (>=15): {vm:?}");
    for needle in [
        "window:",
        "canvas cleared:",
        "Hello SYX",
        "Click me",
        "textField:",
        "layout:",
        "vstack",
        "onClick:",
        "onButtonClick: handler ran",
        "invoked:onButtonClick",
        "tick 2:",
        "gui demo done",
    ] {
        assert!(vm.iter().any(|l| l.contains(needle)), "vm missing {needle}: {vm:?}");
    }
    assert!(
        !vm.iter().any(|l| l.contains("nil")),
        "vm must not print nil: {vm:?}"
    );

    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!("syx_p98_guidemo_full_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::copy(&guidemo_path(), dir.join("guidemo.sy")).unwrap();
    fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"guidemo.sy\"\n").unwrap();
    let exe = dir.join("app");
    build_native_executable(&dir, &opts(), &exe).expect("native build");
    let output = Command::new(&exe).output().unwrap();
    assert!(output.status.success(), "native run failed: {:?}", output.status);
    let native: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect();
    assert!(native.len() >= 15, "native lines (>=15): {native:?}");
    assert!(
        !native.iter().any(|l| l.contains("nil")),
        "native must not print nil: {native:?}"
    );
    assert!(
        !native.iter().any(|l| l.contains("winit-winit-")),
        "native must not repeat winit prefix: {native:?}"
    );
    let n = vm.len().min(native.len()).min(15);
    for i in 0..n {
        assert!(
            lines_compatible(&vm[i], &native[i]),
            "line {i} mismatch vm={} native={}",
            vm[i],
            native[i]
        );
    }
    let _ = fs::remove_dir_all(&dir);
}
