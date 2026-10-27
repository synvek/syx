//! Phase 124–133 Track B — process-local SynUI host bridge (headless / CI-safe).
//!
//! Uses real `synui` types (`View`, `Signal`) via path dep `../synui/crates/synui`.
//! Does **not** call `Application::run` by default (would block CI); builds an in-memory
//! View tree, structural render fingerprints, and optional named-handler invoke.
//!
//! **HeadlessRenderer:** `synui-testing::HeadlessRenderer` pulls wgpu/vello and is too
//! heavy for default CI; we use View-tree structural fingerprints instead (honest stub).
//! Optional `SYX_SYNUI_HEADLESS=1` skips desktop display and appends a deeper
//! `:widget-count=N` suffix to the structural fingerprint (still not wgpu HeadlessRenderer).

use std::collections::{HashMap, HashSet};

use synui::{
    register_bool_binding, register_number_binding, register_text_binding,
    register_textarea_binding, BoolBinding, NumberBinding, Signal, TextAreaBinding, TextBinding,
    View,
};
use syx_diagnostics::SyxResult;
use syx_runtime_core::Value;

/// Cache-v96 / Phase 124 GA host-bridge fingerprint.
pub const SYNUI_HOST_BRIDGE_FINGERPRINT: &str = "gui-synui-v1:host-bridge";

/// Path-dep / desktop MVP fingerprint (compile-time proof lives in Cargo.toml + this module).
pub const SYNUI_PATH_DEP_FINGERPRINT: &str = "gui-synui-v1:path-dep-desktop-mvp";

/// UI package surface fingerprint.
pub const SYNUI_UI_PACKAGE_FINGERPRINT: &str = "gui-synui-v1:ui-package-surface";

/// Phase 125 v2 fingerprints.
pub const SYNUI_V2_DESKTOP_RUN_FINGERPRINT: &str = "gui-synui-v2:desktop-run-mvp";
pub const SYNUI_V2_SIGNAL_CALLBACK_FINGERPRINT: &str = "gui-synui-v2:signal-callback";
pub const SYNUI_V2_WIDGET_EXPAND_FINGERPRINT: &str = "gui-synui-v2:widget-expand";
pub const SYNUI_V2_CI_SIBLING_CHECKOUT_FINGERPRINT: &str = "gui-synui-v2:ci-sibling-checkout";

/// Phase 126 v3 fingerprints.
pub const SYNUI_V3_DESKTOP_RUN_FINGERPRINT: &str = "gui-synui-v3:desktop-run-optin";
pub const SYNUI_V3_WIDGET_CHECKBOX_SLIDER_FINGERPRINT: &str = "gui-synui-v3:widget-checkbox-slider";

/// Phase 127 v4 fingerprints.
pub const SYNUI_V4_SIGNAL_WIDGET_BIND_FINGERPRINT: &str = "gui-synui-v4:signal-widget-bind";
pub const SYNUI_V4_DESKTOP_ONCLICK_WIRE_FINGERPRINT: &str = "gui-synui-v4:desktop-onclick-wire";
pub const SYNUI_V4_WIDGET_SWITCH_RADIO_FINGERPRINT: &str = "gui-synui-v4:widget-switch-radio";

/// Phase 128 v5 fingerprints.
pub const SYNUI_V5_TOGGLE_RADIO_BIND_FINGERPRINT: &str = "gui-synui-v5:toggle-radio-bind";
pub const SYNUI_V5_WIDGET_PROGRESS_PICKER_FINGERPRINT: &str = "gui-synui-v5:widget-progress-picker";
pub const SYNUI_V5_MOBILE_SURFACE_UNRETIRE_FINGERPRINT: &str =
    "gui-synui-v5:mobile-surface-unretire";

/// Phase 129 v6 fingerprints.
pub const SYNUI_V6_WIDGET_TEXTAREA_LIST_FINGERPRINT: &str = "gui-synui-v6:widget-textarea-list";
pub const SYNUI_V6_PROGRESS_NUMBER_BIND_FINGERPRINT: &str = "gui-synui-v6:progress-number-bind";
/// Desktop `run()` layout GA: Row→Column + Scroll + `rekey_from` stable keys.
pub const SYNUI_V6_DESKTOP_RUN_LAYOUT_GA_FINGERPRINT: &str = "gui-synui-v6:desktop-run-layout-ga";

/// Phase 130 v7 fingerprints.
pub const SYNUI_V7_WIDGET_IMAGE_SCROLL_FINGERPRINT: &str = "gui-synui-v7:widget-image-scroll";
pub const SYNUI_V7_PICKER_SIGNAL_BIND_FINGERPRINT: &str = "gui-synui-v7:picker-signal-bind";
pub const SYNUI_V7_WEB_LANGUAGE_SURFACE_MVP_FINGERPRINT: &str =
    "gui-synui-v7:web-language-surface-mvp";

/// Phase 131 v8 fingerprints.
pub const SYNUI_V8_TEXTAREA_LIST_BIND_FINGERPRINT: &str = "gui-synui-v8:textarea-list-bind";
pub const SYNUI_V8_WIDGET_SPACER_FINGERPRINT: &str = "gui-synui-v8:widget-spacer";
pub const SYNUI_V8_WEB_MOBILE_SURFACE_DEEPEN_FINGERPRINT: &str =
    "gui-synui-v8:web-mobile-surface-deepen";

/// Phase 132 v9 fingerprints.
pub const SYNUI_V9_IMAGE_SIGNAL_BIND_FINGERPRINT: &str = "gui-synui-v9:image-signal-bind";
pub const SYNUI_V9_WIDGET_DIVIDER_FINGERPRINT: &str = "gui-synui-v9:widget-divider";
pub const SYNUI_V9_WEB_MOBILE_CONTROLS_DEEPEN_FINGERPRINT: &str =
    "gui-synui-v9:web-mobile-controls-deepen";

/// Phase 133 v10 fingerprints.
pub const SYNUI_V10_WIDGET_STACK_FINGERPRINT: &str = "gui-synui-v10:widget-stack";
pub const SYNUI_V10_SCROLLVIEW_SIGNAL_BIND_FINGERPRINT: &str =
    "gui-synui-v10:scrollview-signal-bind";
pub const SYNUI_V10_WEB_MOBILE_LAYOUT_DEEPEN_FINGERPRINT: &str =
    "gui-synui-v10:web-mobile-layout-deepen";

/// Phase 134 v11 fingerprints.
pub const SYNUI_V11_WIDGET_TABVIEW_FINGERPRINT: &str = "gui-synui-v11:widget-tabview";
pub const SYNUI_V11_TAB_SIGNAL_BIND_FINGERPRINT: &str = "gui-synui-v11:tab-signal-bind";
pub const SYNUI_V11_WEB_MOBILE_CONTROLS_DEEPEN_FINGERPRINT: &str =
    "gui-synui-v11:web-mobile-controls-deepen";

/// Phase 135 v12 fingerprints.
pub const SYNUI_V12_WIDGET_NAVIGATION_FINGERPRINT: &str = "gui-synui-v12:widget-navigation";
pub const SYNUI_V12_NAV_SIGNAL_BIND_FINGERPRINT: &str = "gui-synui-v12:nav-signal-bind";
pub const SYNUI_V12_WEB_MOBILE_SURFACE_DEEPEN_FINGERPRINT: &str =
    "gui-synui-v12:web-mobile-surface-deepen";

const RETIRED: &str = "retired; use SynUI surface";
/// Default list-row extent (px) when building `list` from CSV titles.
const LIST_DEFAULT_ITEM_EXTENT: f32 = 40.0;
/// Sentinel label prefix for spacer widgets (size|weight encoded in text).
const SPACER_SENTINEL: &str = "__synui_spacer:";
/// Sentinel label prefix for divider widgets (orientation/thickness encoded in text).
const DIVIDER_SENTINEL: &str = "__synui_divider:";
/// Sentinel label prefix for navigation bar widgets (routes CSV encoded in text).
const NAV_SENTINEL: &str = "__synui_nav:";

struct SynUiState {
    /// App session id when created.
    app_id: Option<String>,
    /// Allocated views by handle (`view:N`).
    views: HashMap<String, View>,
    /// Reactive string signals by handle (`signal:N`).
    signals: HashMap<String, Signal<String>>,
    /// Reactive bool signals by handle (`bsignal:N`).
    signals_bool: HashMap<String, Signal<bool>>,
    /// Reactive numeric signals by handle (`nsignal:N`).
    signals_f32: HashMap<String, Signal<f32>>,
    /// Button handle → zero-arg SYX handler name (onClick pattern).
    button_handlers: HashMap<String, String>,
    /// Button label text → handler (ViewSnap rebuild after column merge).
    button_handlers_by_text: HashMap<String, String>,
    /// View handles with registered SynUI bindings.
    bound_views: HashMap<String, String>,
    /// Optional root layout handle.
    root: Option<String>,
    /// Column view keys created via `stack` (structural fingerprint uses `stack[...]`).
    stack_keys: HashSet<u64>,
    /// Scroll view keys with Number Signal scroll-offset bind.
    scroll_offsets_by_key: HashMap<u64, f32>,
    /// Column view keys created via `tabView` (structural fingerprint uses `tabview[...]`).
    tabview_keys: HashSet<u64>,
    /// Selected tab index per tabView column key (Signal bind + structural fingerprint).
    tab_selected_by_key: HashMap<u64, usize>,
    /// Column view keys created via `navigation` (structural fingerprint uses `navigation[...]`).
    navigation_keys: HashSet<u64>,
    /// Stack depth per navigation column key (Signal bind + structural fingerprint).
    nav_depth_by_key: HashMap<u64, usize>,
    /// Proves Signal path-dep is live (title text).
    title: Signal<String>,
    next_id: u64,
    pumps: u32,
    /// Count of successful named-handler invokes (signal+callback E2E).
    click_invokes: u32,
}

impl Default for SynUiState {
    fn default() -> Self {
        Self {
            app_id: None,
            views: HashMap::new(),
            signals: HashMap::new(),
            signals_bool: HashMap::new(),
            signals_f32: HashMap::new(),
            button_handlers: HashMap::new(),
            button_handlers_by_text: HashMap::new(),
            bound_views: HashMap::new(),
            root: None,
            stack_keys: HashSet::new(),
            scroll_offsets_by_key: HashMap::new(),
            tabview_keys: HashSet::new(),
            tab_selected_by_key: HashMap::new(),
            navigation_keys: HashSet::new(),
            nav_depth_by_key: HashMap::new(),
            title: Signal::new(String::from("synui")),
            next_id: 1,
            pumps: 0,
            click_invokes: 0,
        }
    }
}

// Signal/View use Rc — keep per-thread (parallel driver tests).
std::thread_local! {
    static STATE: std::cell::RefCell<SynUiState> = std::cell::RefCell::new(SynUiState::default());
}

fn with_state_mut<R>(f: impl FnOnce(&mut SynUiState) -> R) -> R {
    STATE.with(|s| f(&mut s.borrow_mut()))
}

fn pop_str(stack: &mut Vec<Value>) -> String {
    stack.pop().unwrap_or(Value::Str(String::new())).as_str()
}

fn alloc_handle(state: &mut SynUiState, kind: &str) -> String {
    let id = state.next_id;
    state.next_id += 1;
    format!("{kind}:{id}")
}

fn parse_children_csv(state: &mut SynUiState, children_csv: &str) -> Vec<View> {
    let mut kids = Vec::new();
    for part in children_csv
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        if let Some(view) = state.views.remove(part) {
            kids.push(view);
        }
    }
    kids
}

fn list_sticky_with_selected(sticky: Option<&str>, selected: usize) -> String {
    let base = sticky
        .unwrap_or("")
        .split("|sel=")
        .next()
        .unwrap_or("")
        .trim_end_matches('|')
        .to_string();
    if base.is_empty() {
        format!("sel={selected}")
    } else {
        format!("{base}|sel={selected}")
    }
}

fn view_structural_fingerprint(v: &View, st: &SynUiState) -> String {
    match v {
        View::Label { text, .. } => {
            if let Some(rest) = text.strip_prefix(DIVIDER_SENTINEL) {
                format!("divider({rest})")
            } else if let Some(rest) = text.strip_prefix(SPACER_SENTINEL) {
                format!("spacer({rest})")
            } else if let Some(rest) = text.strip_prefix(NAV_SENTINEL) {
                format!("navbar({rest})")
            } else {
                format!("label({text})")
            }
        }
        View::Button { text, on_press, .. } => {
            let hook = if on_press.is_some() || st.button_handlers_by_text.contains_key(text) {
                ":press"
            } else {
                ""
            };
            format!("button({text}{hook})")
        }
        View::TextField {
            placeholder, text, ..
        } => {
            let t = text.as_deref().unwrap_or("");
            format!("textfield({placeholder}|{t})")
        }
        View::Column { children, key, .. } if st.stack_keys.contains(key) => {
            let inner: Vec<_> = children
                .iter()
                .map(|c| view_structural_fingerprint(c, st))
                .collect();
            format!("stack[{}]", inner.join(","))
        }
        View::Column { children, key, .. } if st.tabview_keys.contains(key) => {
            let sel = st.tab_selected_by_key.get(key).copied().unwrap_or(0);
            let inner: Vec<_> = children
                .iter()
                .map(|c| view_structural_fingerprint(c, st))
                .collect();
            format!("tabview[sel={sel}][{}]", inner.join(","))
        }
        View::Column { children, key, .. } if st.navigation_keys.contains(key) => {
            let depth = st.nav_depth_by_key.get(key).copied().unwrap_or(0);
            let inner: Vec<_> = children
                .iter()
                .map(|c| view_structural_fingerprint(c, st))
                .collect();
            format!("navigation[depth={depth}][{}]", inner.join(","))
        }
        View::TabBar { tabs, selected, .. } => {
            format!("tabbar({}|sel={selected})", tabs.join(","))
        }
        View::Column { children, .. } => {
            let inner: Vec<_> = children
                .iter()
                .map(|c| view_structural_fingerprint(c, st))
                .collect();
            format!("column[{}]", inner.join(","))
        }
        View::Row { children, .. } => {
            let inner: Vec<_> = children
                .iter()
                .map(|c| view_structural_fingerprint(c, st))
                .collect();
            format!("row[{}]", inner.join(","))
        }
        View::Checkbox { label, checked, .. } => format!("checkbox({label}|{checked})"),
        View::Slider {
            value, min, max, ..
        } => format!("slider({value}|{min}|{max})"),
        View::Switch { on, .. } => format!("switch({on})"),
        View::Radio {
            label, selected, ..
        } => format!("radio({label}|{selected})"),
        View::LinearProgress {
            value,
            indeterminate,
            ..
        } => format!("progress({value}|indet={indeterminate})"),
        View::Dropdown {
            options, selected, ..
        } => format!("picker({}|{selected})", options.join(",")),
        View::TextArea { text, .. } => format!("textarea({text})"),
        View::ListView {
            item_count,
            item_extent,
            sticky_header,
            ..
        } => {
            let hdr = sticky_header.as_deref().unwrap_or("");
            format!("list({item_count}|ext={item_extent}|{hdr})")
        }
        View::ListItem { title, .. } => format!("listitem({title})"),
        View::Image { source, .. } => format!("image({source})"),
        View::Scroll { children, key, .. } => {
            let inner: Vec<_> = children
                .iter()
                .map(|c| view_structural_fingerprint(c, st))
                .collect();
            let offset = st.scroll_offsets_by_key.get(key).copied().unwrap_or(0.0);
            if offset != 0.0 {
                format!("scroll[offset={offset}][{}]", inner.join(","))
            } else {
                format!("scroll[{}]", inner.join(","))
            }
        }
        _ => "other".into(),
    }
}

fn pop_usize(stack: &mut Vec<Value>) -> usize {
    match stack.pop() {
        Some(Value::I32(i)) if i >= 0 => i as usize,
        Some(Value::I64(i)) if i >= 0 => i as usize,
        Some(Value::F32(f)) if f >= 0.0 => f as usize,
        Some(Value::F64(f)) if f >= 0.0 => f as usize,
        Some(v) => v.as_str().parse().unwrap_or(0),
        None => 0,
    }
}

fn pop_bool(stack: &mut Vec<Value>) -> bool {
    match stack.pop() {
        Some(Value::Bool(b)) => b,
        Some(v) => {
            let s = v.as_str();
            s.eq_ignore_ascii_case("true") || s == "1"
        }
        None => false,
    }
}

fn pop_f32(stack: &mut Vec<Value>) -> f32 {
    match stack.pop() {
        Some(Value::F32(f)) => f,
        Some(Value::F64(f)) => f as f32,
        Some(Value::I32(i)) => i as f32,
        Some(v) => v.as_str().parse().unwrap_or(0.0),
        None => 0.0,
    }
}

fn env_truthy(name: &str) -> bool {
    std::env::var(name)
        .ok()
        .is_some_and(|v| v == "1" || v.eq_ignore_ascii_case("true"))
}

fn env_falsy(name: &str) -> bool {
    std::env::var(name)
        .ok()
        .is_some_and(|v| v == "0" || v.eq_ignore_ascii_case("false"))
}

/// Desktop run opt-in: `SYX_SYNUI_RUN`, `--gui-visible` (`SYX_GUI_VISIBLE`), or `SYX_GUI_HOLD=1`.
fn synui_desktop_run_requested() -> bool {
    env_truthy("SYX_SYNUI_RUN") || env_truthy("SYX_GUI_VISIBLE") || env_truthy("SYX_GUI_HOLD")
}

/// Keep the event loop on the main thread until the window closes.
/// `--gui-visible` defaults `SYX_GUI_HOLD=1`; set `SYX_GUI_HOLD=0` to disable hold.
fn synui_desktop_hold_requested() -> bool {
    if env_falsy("SYX_GUI_HOLD") {
        return false;
    }
    env_truthy("SYX_GUI_HOLD") || env_truthy("SYX_GUI_VISIBLE")
}

/// Cloneable snapshot of the host View tree (callbacks dropped) for desktop `run_app`.
#[derive(Clone)]
enum ViewSnap {
    Label(String),
    Button {
        text: String,
        handler: Option<String>,
    },
    TextField(String),
    Checkbox {
        label: String,
        checked: bool,
    },
    Slider {
        value: f32,
        min: f32,
        max: f32,
    },
    Switch {
        on: bool,
    },
    Radio {
        label: String,
        selected: bool,
    },
    Progress {
        value: f32,
        indeterminate: bool,
    },
    Picker {
        options: Vec<String>,
        selected: usize,
    },
    TextArea(String),
    List {
        item_count: usize,
        item_extent: f32,
        sticky_header: Option<String>,
    },
    Image(String),
    Spacer {
        size: f32,
        weight: f32,
    },
    Scroll(Vec<ViewSnap>),
    Stack(Vec<ViewSnap>),
    TabView {
        tabs: Vec<String>,
        selected: usize,
        panels: Vec<ViewSnap>,
    },
    Navigation {
        routes: Vec<String>,
        depth: usize,
        panels: Vec<ViewSnap>,
    },
    TabBar {
        tabs: Vec<String>,
        selected: usize,
    },
    Column(Vec<ViewSnap>),
    Row(Vec<ViewSnap>),
    Other(String),
}

fn snapshot_view(v: &View, st: &SynUiState) -> ViewSnap {
    match v {
        View::Label { text, .. } => {
            if let Some(rest) = text.strip_prefix(SPACER_SENTINEL) {
                let mut parts = rest.split('|');
                let size = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
                let weight = parts.next().and_then(|s| s.parse().ok()).unwrap_or(1.0);
                ViewSnap::Spacer { size, weight }
            } else if text.starts_with(NAV_SENTINEL) {
                ViewSnap::Label(text.clone())
            } else {
                ViewSnap::Label(text.clone())
            }
        }
        View::Button { text, .. } => ViewSnap::Button {
            text: text.clone(),
            handler: st.button_handlers_by_text.get(text).cloned(),
        },
        View::TextField { placeholder, .. } => ViewSnap::TextField(placeholder.clone()),
        View::Checkbox { label, checked, .. } => ViewSnap::Checkbox {
            label: label.clone(),
            checked: *checked,
        },
        View::Slider {
            value, min, max, ..
        } => ViewSnap::Slider {
            value: *value,
            min: *min,
            max: *max,
        },
        View::Switch { on, .. } => ViewSnap::Switch { on: *on },
        View::Radio {
            label, selected, ..
        } => ViewSnap::Radio {
            label: label.clone(),
            selected: *selected,
        },
        View::LinearProgress {
            value,
            indeterminate,
            ..
        } => ViewSnap::Progress {
            value: *value,
            indeterminate: *indeterminate,
        },
        View::Dropdown {
            options, selected, ..
        } => ViewSnap::Picker {
            options: options.clone(),
            selected: *selected,
        },
        View::TextArea { text, .. } => ViewSnap::TextArea(text.clone()),
        View::ListView {
            item_count,
            item_extent,
            sticky_header,
            ..
        } => ViewSnap::List {
            item_count: *item_count,
            item_extent: *item_extent,
            sticky_header: sticky_header.clone(),
        },
        View::Image { source, .. } => ViewSnap::Image(source.clone()),
        View::Scroll { children, .. } => {
            ViewSnap::Scroll(children.iter().map(|c| snapshot_view(c, st)).collect())
        }
        View::TabBar { tabs, selected, .. } => ViewSnap::TabBar {
            tabs: tabs.clone(),
            selected: *selected,
        },
        View::Column { children, key, .. } if st.tabview_keys.contains(key) => {
            let tabs = children
                .iter()
                .find_map(|c| {
                    if let View::TabBar { tabs, selected, .. } = c {
                        Some((tabs.clone(), *selected))
                    } else {
                        None
                    }
                })
                .unwrap_or_default();
            let panels: Vec<_> = children
                .iter()
                .filter(|c| !matches!(c, View::TabBar { .. }))
                .map(|c| snapshot_view(c, st))
                .collect();
            ViewSnap::TabView {
                tabs: tabs.0,
                selected: st.tab_selected_by_key.get(key).copied().unwrap_or(tabs.1),
                panels,
            }
        }
        View::Column { children, key, .. } if st.navigation_keys.contains(key) => {
            let routes = children
                .iter()
                .find_map(|c| {
                    if let View::Label { text, .. } = c {
                        text.strip_prefix(NAV_SENTINEL).map(|r| r.to_string())
                    } else {
                        None
                    }
                })
                .unwrap_or_default();
            let route_labels: Vec<String> = routes
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect();
            let panels: Vec<_> = children
                .iter()
                .filter(|c| {
                    !matches!(
                        c,
                        View::Label { text, .. } if text.starts_with(NAV_SENTINEL)
                    )
                })
                .map(|c| snapshot_view(c, st))
                .collect();
            ViewSnap::Navigation {
                routes: route_labels,
                depth: st.nav_depth_by_key.get(key).copied().unwrap_or(0),
                panels,
            }
        }
        View::Column { children, key, .. } if st.stack_keys.contains(key) => {
            ViewSnap::Stack(children.iter().map(|c| snapshot_view(c, st)).collect())
        }
        View::Column { children, .. } => {
            ViewSnap::Column(children.iter().map(|c| snapshot_view(c, st)).collect())
        }
        View::Row { children, .. } => {
            ViewSnap::Row(children.iter().map(|c| snapshot_view(c, st)).collect())
        }
        _ => ViewSnap::Other(String::from("unsupported-view")),
    }
}

fn view_from_snap(snap: &ViewSnap) -> View {
    match snap {
        ViewSnap::Label(t) => View::label(t.clone()),
        ViewSnap::Button { text, handler } => {
            let mut btn = View::button(text.clone());
            if let Some(name) = handler {
                let handler_name = name.clone();
                btn = btn.on_press(move || {
                    let _ = crate::gui_invoke::invoke_gui_handler(&handler_name);
                });
            }
            btn
        }
        ViewSnap::TextField(p) => View::text_field(p.clone()),
        ViewSnap::Checkbox { label, checked } => View::checkbox(label.clone(), *checked),
        ViewSnap::Slider { value, min, max } => View::slider(*value, *min, *max),
        ViewSnap::Switch { on } => View::switch(*on),
        ViewSnap::Radio { label, selected } => View::radio(label.clone(), *selected),
        ViewSnap::Progress {
            value,
            indeterminate,
        } => {
            if *indeterminate {
                View::linear_progress_indeterminate()
            } else {
                View::linear_progress(*value)
            }
        }
        ViewSnap::Picker { options, selected } => View::dropdown(options.clone(), *selected),
        ViewSnap::TextArea(t) => View::text_area(t.clone()),
        ViewSnap::List {
            item_count,
            item_extent,
            sticky_header,
        } => {
            let mut lv = View::list_view(*item_count, *item_extent);
            if let Some(h) = sticky_header {
                lv = lv.with_sticky_header(h.clone());
            }
            lv
        }
        ViewSnap::Image(src) => View::image(src.clone()),
        // Size/weight MVP: empty column with vertical padding ≈ size (weight reserved).
        ViewSnap::Spacer { size, weight } => {
            let _ = weight;
            View::column(vec![]).with_padding([0.0, 0.0, size.max(0.0), 0.0])
        }
        ViewSnap::Scroll(kids) => View::scroll(kids.iter().map(view_from_snap).collect()),
        ViewSnap::TabBar { tabs, selected } => View::tab_bar(tabs.clone(), *selected),
        ViewSnap::TabView {
            tabs,
            selected,
            panels,
        } => {
            let mut kids = vec![View::tab_bar(tabs.clone(), *selected)];
            kids.extend(panels.iter().map(view_from_snap));
            View::column(kids)
        }
        ViewSnap::Navigation {
            routes,
            depth: _,
            panels,
        } => {
            let routes_csv = routes.join(",");
            let mut kids = vec![View::label(format!("{NAV_SENTINEL}{routes_csv}"))];
            kids.extend(panels.iter().map(view_from_snap));
            View::column(kids)
        }
        ViewSnap::Stack(kids) => View::column(kids.iter().map(view_from_snap).collect()),
        ViewSnap::Column(kids) => View::column(kids.iter().map(view_from_snap).collect()),
        // Desktop HOLD windows are ~480–800px wide: a horizontal row of many
        // controls clips to empty-looking chrome. Stack children vertically for run().
        // Phase 129 GA: Row→Column + Scroll + rekey_from (gui-synui-v6:desktop-run-layout-ga).
        ViewSnap::Row(kids) => View::column(kids.iter().map(view_from_snap).collect()),
        ViewSnap::Other(s) => View::label(s.clone()),
    }
}

/// Build the desktop root once per frame with **stable keys** (`rekey_from`).
/// Without stable keys, `alloc_view_key()` every frame churns the widget map and
/// the visible tree can collapse to the title label only.
/// Fingerprint: `gui-synui-v6:desktop-run-layout-ga` (Column + Scroll + rekey_from).
fn desktop_root_from_snap(snap: &ViewSnap, title: &str) -> View {
    let _ = SYNUI_V6_DESKTOP_RUN_LAYOUT_GA_FINGERPRINT;
    let body = view_from_snap(snap);
    let root = View::column(vec![
        View::label(format!("SYX SynUI — {title}")),
        View::scroll(vec![body]),
    ])
    .with_gap(12.0)
    .with_padding([12.0, 12.0, 12.0, 12.0]);
    root.rekey_from(50_000)
}

fn snapshot_root_for_run(st: &SynUiState) -> ViewSnap {
    if let Some(h) = st.root.as_ref() {
        if let Some(v) = st.views.get(h) {
            return snapshot_view(v, st);
        }
    }
    // Fallback: column of all leaf-ish widgets currently allocated.
    let mut kids = Vec::new();
    for v in st.views.values() {
        match v {
            View::Column { .. } | View::Row { .. } => {}
            other => kids.push(snapshot_view(other, st)),
        }
    }
    if kids.is_empty() {
        ViewSnap::Label(String::from("SYX SynUI (empty tree)"))
    } else {
        ViewSnap::Column(kids)
    }
}

fn synui_display_likely_available() -> bool {
    if env_truthy("SYX_SYNUI_HEADLESS") {
        return false;
    }
    #[cfg(target_os = "linux")]
    {
        let has_display = std::env::var("DISPLAY").ok().is_some_and(|s| !s.is_empty())
            || std::env::var("WAYLAND_DISPLAY")
                .ok()
                .is_some_and(|s| !s.is_empty());
        if !has_display {
            return false;
        }
    }
    true
}

fn structural_render_fingerprint(state: &SynUiState) -> String {
    let mut parts: Vec<String> = state.signals.keys().map(|k| format!("sig:{k}")).collect();
    parts.sort();
    let tree = state
        .root
        .as_ref()
        .and_then(|h| state.views.get(h))
        .map(|v| view_structural_fingerprint(v, state))
        .unwrap_or_else(|| "empty".into());
    let mut fp = format!("structural:{tree}:signals={}", parts.join("+"));
    // Optional deeper count when headless opt-in (not wgpu HeadlessRenderer CI).
    if env_truthy("SYX_SYNUI_HEADLESS") {
        let widget_count = count_widgets_in_state(state);
        fp.push_str(&format!(":widget-count={widget_count}"));
    }
    fp
}

fn count_widgets_in_view(v: &View) -> usize {
    match v {
        View::Column { children, .. }
        | View::Row { children, .. }
        | View::Scroll { children, .. } => {
            1 + children.iter().map(count_widgets_in_view).sum::<usize>()
        }
        View::TabBar { .. } => 1,
        _ => 1,
    }
}

fn count_widgets_in_state(state: &SynUiState) -> usize {
    if let Some(h) = state.root.as_ref() {
        if let Some(v) = state.views.get(h) {
            return count_widgets_in_view(v);
        }
    }
    state.views.values().map(count_widgets_in_view).sum()
}

/// Reset bridge state (tests).
pub fn reset_synui_state() {
    STATE.with(|s| *s.borrow_mut() = SynUiState::default());
}

pub fn synui_fingerprint() -> &'static str {
    SYNUI_HOST_BRIDGE_FINGERPRINT
}

pub fn host_synui_fingerprint(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    stack.push(Value::Str(SYNUI_HOST_BRIDGE_FINGERPRINT.into()));
    Ok(())
}

pub fn host_synui_app_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    let id = with_state_mut(|st| {
        *st = SynUiState::default();
        st.title.set(String::from("synui-app"));
        let id = String::from("synui-app:1");
        st.app_id = Some(id.clone());
        id
    });
    stack.push(Value::Str(id));
    Ok(())
}

pub fn host_synui_signal_new(stack: &mut Vec<Value>) -> SyxResult<()> {
    let initial = pop_str(stack);
    let handle = with_state_mut(|st| {
        let handle = alloc_handle(st, "signal");
        st.signals
            .insert(handle.clone(), Signal::new(initial.clone()));
        handle
    });
    stack.push(Value::Str(handle));
    Ok(())
}

pub fn host_synui_signal_get(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handle = pop_str(stack);
    let value = with_state_mut(|st| st.signals.get(&handle).map(|s| s.get()).unwrap_or_default());
    stack.push(Value::Str(value));
    Ok(())
}

pub fn host_synui_signal_set(stack: &mut Vec<Value>) -> SyxResult<()> {
    let value = pop_str(stack);
    let handle = pop_str(stack);
    let status = with_state_mut(|st| {
        if let Some(sig) = st.signals.get(&handle) {
            sig.set(value.clone());
            format!("{SYNUI_V2_SIGNAL_CALLBACK_FINGERPRINT}:set:{handle}")
        } else {
            format!("synui-signal:missing:{handle}")
        }
    });
    stack.push(Value::Str(status));
    Ok(())
}

pub fn host_synui_label(stack: &mut Vec<Value>) -> SyxResult<()> {
    let text = pop_str(stack);
    let handle = with_state_mut(|st| {
        let handle = alloc_handle(st, "label");
        st.views.insert(handle.clone(), View::label(text));
        handle
    });
    // Mobile surface (`packages/ui/mobile.sy`) forwards label/button/column here.
    let _ = SYNUI_V5_MOBILE_SURFACE_UNRETIRE_FINGERPRINT;
    stack.push(Value::Str(handle));
    Ok(())
}

pub fn host_synui_button(stack: &mut Vec<Value>) -> SyxResult<()> {
    let text = pop_str(stack);
    let handle = with_state_mut(|st| {
        let handle = alloc_handle(st, "button");
        st.views.insert(handle.clone(), View::button(text));
        handle
    });
    stack.push(Value::Str(handle));
    Ok(())
}

/// Register a named zero-arg SYX handler on a button handle (Di.withRequest pattern).
pub fn host_synui_button_on_click(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handler = pop_str(stack);
    let handle = pop_str(stack);
    let status = with_state_mut(|st| {
        if !st.views.contains_key(&handle) {
            return format!("synui-onclick:missing-view:{handle}");
        }
        if let Some(View::Button { text, .. }) = st.views.get(&handle) {
            st.button_handlers_by_text
                .insert(text.clone(), handler.clone());
        }
        st.button_handlers.insert(handle.clone(), handler.clone());
        format!("{SYNUI_V2_SIGNAL_CALLBACK_FINGERPRINT}:registered:{handle}:{handler}")
    });
    stack.push(Value::Str(status));
    Ok(())
}

/// Simulate button press: invoke registered handler via active `ProgramCtx`.
pub fn host_synui_button_click(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handle = pop_str(stack);
    let status = with_state_mut(|st| {
        let Some(handler) = st.button_handlers.get(&handle).cloned() else {
            return format!("synui-click:no-handler:{handle}");
        };
        match crate::gui_invoke::invoke_gui_handler(&handler) {
            Ok(_) => {
                st.click_invokes = st.click_invokes.saturating_add(1);
                format!("{SYNUI_V2_SIGNAL_CALLBACK_FINGERPRINT}:invoked:{handler}")
            }
            Err(e) => format!("error:{}", e),
        }
    });
    stack.push(Value::Str(status));
    Ok(())
}

pub fn host_synui_text_field(stack: &mut Vec<Value>) -> SyxResult<()> {
    let placeholder = pop_str(stack);
    let handle = with_state_mut(|st| {
        let handle = alloc_handle(st, "textfield");
        st.views
            .insert(handle.clone(), View::text_field(placeholder));
        handle
    });
    stack.push(Value::Str(handle));
    Ok(())
}

pub fn host_synui_checkbox(stack: &mut Vec<Value>) -> SyxResult<()> {
    let checked = pop_bool(stack);
    let label = pop_str(stack);
    let handle = with_state_mut(|st| {
        let handle = alloc_handle(st, "checkbox");
        st.views
            .insert(handle.clone(), View::checkbox(label, checked));
        handle
    });
    let _ = SYNUI_V3_WIDGET_CHECKBOX_SLIDER_FINGERPRINT;
    stack.push(Value::Str(handle));
    Ok(())
}

pub fn host_synui_slider(stack: &mut Vec<Value>) -> SyxResult<()> {
    let max = pop_f32(stack);
    let min = pop_f32(stack);
    let value = pop_f32(stack);
    let handle = with_state_mut(|st| {
        let handle = alloc_handle(st, "slider");
        st.views
            .insert(handle.clone(), View::slider(value, min, max));
        handle
    });
    let _ = SYNUI_V3_WIDGET_CHECKBOX_SLIDER_FINGERPRINT;
    stack.push(Value::Str(handle));
    Ok(())
}

pub fn host_synui_switch(stack: &mut Vec<Value>) -> SyxResult<()> {
    let on = pop_bool(stack);
    let handle = with_state_mut(|st| {
        let handle = alloc_handle(st, "switch");
        st.views.insert(handle.clone(), View::switch(on));
        handle
    });
    let _ = SYNUI_V4_WIDGET_SWITCH_RADIO_FINGERPRINT;
    stack.push(Value::Str(handle));
    Ok(())
}

pub fn host_synui_radio(stack: &mut Vec<Value>) -> SyxResult<()> {
    let selected = pop_bool(stack);
    let label = pop_str(stack);
    let handle = with_state_mut(|st| {
        let handle = alloc_handle(st, "radio");
        st.views
            .insert(handle.clone(), View::radio(label, selected));
        handle
    });
    let _ = SYNUI_V4_WIDGET_SWITCH_RADIO_FINGERPRINT;
    stack.push(Value::Str(handle));
    Ok(())
}

/// Linear progress bar — surface name `progress` (avoids reserved-word traps).
pub fn host_synui_progress(stack: &mut Vec<Value>) -> SyxResult<()> {
    let value = pop_f32(stack);
    let handle = with_state_mut(|st| {
        let handle = alloc_handle(st, "progress");
        st.views
            .insert(handle.clone(), View::linear_progress(value));
        handle
    });
    let _ = SYNUI_V5_WIDGET_PROGRESS_PICKER_FINGERPRINT;
    stack.push(Value::Str(handle));
    Ok(())
}

/// Option picker — `options` is comma-separated; `selected` is a 0-based index.
/// Named `picker` (not `select`/`switch`) to avoid reserved keywords.
pub fn host_synui_picker(stack: &mut Vec<Value>) -> SyxResult<()> {
    let selected = pop_usize(stack);
    let options_csv = pop_str(stack);
    let options: Vec<String> = options_csv
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();
    let selected = if options.is_empty() {
        0
    } else {
        selected.min(options.len() - 1)
    };
    let handle = with_state_mut(|st| {
        let handle = alloc_handle(st, "picker");
        st.views
            .insert(handle.clone(), View::dropdown(options, selected));
        handle
    });
    let _ = SYNUI_V5_WIDGET_PROGRESS_PICKER_FINGERPRINT;
    stack.push(Value::Str(handle));
    Ok(())
}

/// Multi-line text area — surface name `textArea` (camelCase package API).
pub fn host_synui_text_area(stack: &mut Vec<Value>) -> SyxResult<()> {
    let text = pop_str(stack);
    let handle = with_state_mut(|st| {
        let handle = alloc_handle(st, "textarea");
        st.views.insert(handle.clone(), View::text_area(text));
        handle
    });
    let _ = SYNUI_V6_WIDGET_TEXTAREA_LIST_FINGERPRINT;
    stack.push(Value::Str(handle));
    Ok(())
}

/// List view — `items` is comma-separated titles (CSV contract like picker/column).
/// Named `list` (not a reserved keyword). Builds SynUI `ListView` with item count.
pub fn host_synui_list(stack: &mut Vec<Value>) -> SyxResult<()> {
    let items_csv = pop_str(stack);
    let items: Vec<String> = items_csv
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();
    let item_count = items.len().max(1);
    let sticky = if items.is_empty() {
        None
    } else {
        Some(items.join("|"))
    };
    let handle = with_state_mut(|st| {
        let handle = alloc_handle(st, "list");
        let mut lv = View::list_view(item_count, LIST_DEFAULT_ITEM_EXTENT);
        if let Some(h) = sticky {
            lv = lv.with_sticky_header(h);
        }
        st.views.insert(handle.clone(), lv);
        handle
    });
    let _ = SYNUI_V6_WIDGET_TEXTAREA_LIST_FINGERPRINT;
    stack.push(Value::Str(handle));
    Ok(())
}

pub fn host_synui_signal_bool_new(stack: &mut Vec<Value>) -> SyxResult<()> {
    let initial = pop_bool(stack);
    let handle = with_state_mut(|st| {
        let handle = alloc_handle(st, "bsignal");
        st.signals_bool.insert(handle.clone(), Signal::new(initial));
        handle
    });
    stack.push(Value::Str(handle));
    Ok(())
}

pub fn host_synui_signal_bool_get(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handle = pop_str(stack);
    let value = with_state_mut(|st| {
        st.signals_bool
            .get(&handle)
            .map(|s| s.get())
            .unwrap_or(false)
    });
    stack.push(Value::Bool(value));
    Ok(())
}

pub fn host_synui_signal_bool_set(stack: &mut Vec<Value>) -> SyxResult<()> {
    let value = pop_bool(stack);
    let handle = pop_str(stack);
    let status = with_state_mut(|st| {
        if let Some(sig) = st.signals_bool.get(&handle) {
            sig.set(value);
            format!("{SYNUI_V4_SIGNAL_WIDGET_BIND_FINGERPRINT}:bool-set:{handle}")
        } else {
            format!("synui-bsignal:missing:{handle}")
        }
    });
    stack.push(Value::Str(status));
    Ok(())
}

pub fn host_synui_signal_number_new(stack: &mut Vec<Value>) -> SyxResult<()> {
    let initial = pop_f32(stack);
    let handle = with_state_mut(|st| {
        let handle = alloc_handle(st, "nsignal");
        st.signals_f32.insert(handle.clone(), Signal::new(initial));
        handle
    });
    stack.push(Value::Str(handle));
    Ok(())
}

pub fn host_synui_signal_number_get(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handle = pop_str(stack);
    let value = with_state_mut(|st| st.signals_f32.get(&handle).map(|s| s.get()).unwrap_or(0.0));
    stack.push(Value::F32(value));
    Ok(())
}

pub fn host_synui_signal_number_set(stack: &mut Vec<Value>) -> SyxResult<()> {
    let value = pop_f32(stack);
    let handle = pop_str(stack);
    let status = with_state_mut(|st| {
        if let Some(sig) = st.signals_f32.get(&handle) {
            sig.set(value);
            format!("{SYNUI_V4_SIGNAL_WIDGET_BIND_FINGERPRINT}:number-set:{handle}")
        } else {
            format!("synui-nsignal:missing:{handle}")
        }
    });
    stack.push(Value::Str(status));
    Ok(())
}

pub fn host_synui_bind_text_field(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sig_handle = pop_str(stack);
    let view_handle = pop_str(stack);
    let status = with_state_mut(|st| {
        let Some(signal) = st.signals.get(&sig_handle).cloned() else {
            return format!("synui-bind:missing-signal:{sig_handle}");
        };
        let Some(view) = st.views.get_mut(&view_handle) else {
            return format!("synui-bind:missing-view:{view_handle}");
        };
        let View::TextField { key, text, .. } = view else {
            return format!("synui-bind:not-textfield:{view_handle}");
        };
        let binding = TextBinding::from_signal(*key, signal.clone());
        register_text_binding(binding);
        *text = Some(signal.get());
        st.bound_views
            .insert(view_handle.clone(), format!("text:{sig_handle}"));
        format!("{SYNUI_V4_SIGNAL_WIDGET_BIND_FINGERPRINT}:textfield:{view_handle}:{sig_handle}")
    });
    stack.push(Value::Str(status));
    Ok(())
}

pub fn host_synui_bind_checkbox(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sig_handle = pop_str(stack);
    let view_handle = pop_str(stack);
    let status = with_state_mut(|st| {
        let Some(signal) = st.signals_bool.get(&sig_handle).cloned() else {
            return format!("synui-bind:missing-bsignal:{sig_handle}");
        };
        let Some(view) = st.views.get_mut(&view_handle) else {
            return format!("synui-bind:missing-view:{view_handle}");
        };
        let View::Checkbox { key, checked, .. } = view else {
            return format!("synui-bind:not-checkbox:{view_handle}");
        };
        let binding = BoolBinding {
            key: *key,
            signal: signal.clone(),
        };
        register_bool_binding(binding);
        *checked = signal.get();
        st.bound_views
            .insert(view_handle.clone(), format!("bool:{sig_handle}"));
        format!("{SYNUI_V4_SIGNAL_WIDGET_BIND_FINGERPRINT}:checkbox:{view_handle}:{sig_handle}")
    });
    stack.push(Value::Str(status));
    Ok(())
}

pub fn host_synui_bind_slider(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sig_handle = pop_str(stack);
    let view_handle = pop_str(stack);
    let status = with_state_mut(|st| {
        let Some(signal) = st.signals_f32.get(&sig_handle).cloned() else {
            return format!("synui-bind:missing-nsignal:{sig_handle}");
        };
        let Some(view) = st.views.get_mut(&view_handle) else {
            return format!("synui-bind:missing-view:{view_handle}");
        };
        let View::Slider { key, value, .. } = view else {
            return format!("synui-bind:not-slider:{view_handle}");
        };
        let binding = NumberBinding {
            key: *key,
            signal: signal.clone(),
        };
        register_number_binding(binding);
        *value = signal.get();
        st.bound_views
            .insert(view_handle.clone(), format!("number:{sig_handle}"));
        format!("{SYNUI_V4_SIGNAL_WIDGET_BIND_FINGERPRINT}:slider:{view_handle}:{sig_handle}")
    });
    stack.push(Value::Str(status));
    Ok(())
}

/// Bool Signal bind for toggle/switch (same BoolBinding path as checkbox).
pub fn host_synui_bind_switch(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sig_handle = pop_str(stack);
    let view_handle = pop_str(stack);
    let status = with_state_mut(|st| {
        let Some(signal) = st.signals_bool.get(&sig_handle).cloned() else {
            return format!("synui-bind:missing-bsignal:{sig_handle}");
        };
        let Some(view) = st.views.get_mut(&view_handle) else {
            return format!("synui-bind:missing-view:{view_handle}");
        };
        let View::Switch { key, on, .. } = view else {
            return format!("synui-bind:not-switch:{view_handle}");
        };
        let binding = BoolBinding {
            key: *key,
            signal: signal.clone(),
        };
        register_bool_binding(binding);
        *on = signal.get();
        st.bound_views
            .insert(view_handle.clone(), format!("bool:{sig_handle}"));
        format!("{SYNUI_V5_TOGGLE_RADIO_BIND_FINGERPRINT}:switch:{view_handle}:{sig_handle}")
    });
    stack.push(Value::Str(status));
    Ok(())
}

/// Bool Signal bind for radio (Bool binding like checkbox; structural sync on bind).
pub fn host_synui_bind_radio(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sig_handle = pop_str(stack);
    let view_handle = pop_str(stack);
    let status = with_state_mut(|st| {
        let Some(signal) = st.signals_bool.get(&sig_handle).cloned() else {
            return format!("synui-bind:missing-bsignal:{sig_handle}");
        };
        let Some(view) = st.views.get_mut(&view_handle) else {
            return format!("synui-bind:missing-view:{view_handle}");
        };
        let View::Radio { key, selected, .. } = view else {
            return format!("synui-bind:not-radio:{view_handle}");
        };
        let binding = BoolBinding {
            key: *key,
            signal: signal.clone(),
        };
        register_bool_binding(binding);
        *selected = signal.get();
        st.bound_views
            .insert(view_handle.clone(), format!("bool:{sig_handle}"));
        format!("{SYNUI_V5_TOGGLE_RADIO_BIND_FINGERPRINT}:radio:{view_handle}:{sig_handle}")
    });
    stack.push(Value::Str(status));
    Ok(())
}

/// Number Signal bind for `progress` (same NumberBinding path as slider).
pub fn host_synui_bind_progress(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sig_handle = pop_str(stack);
    let view_handle = pop_str(stack);
    let status = with_state_mut(|st| {
        let Some(signal) = st.signals_f32.get(&sig_handle).cloned() else {
            return format!("synui-bind:missing-nsignal:{sig_handle}");
        };
        let Some(view) = st.views.get_mut(&view_handle) else {
            return format!("synui-bind:missing-view:{view_handle}");
        };
        let View::LinearProgress { key, value, .. } = view else {
            return format!("synui-bind:not-progress:{view_handle}");
        };
        let binding = NumberBinding {
            key: *key,
            signal: signal.clone(),
        };
        register_number_binding(binding);
        *value = signal.get();
        st.bound_views
            .insert(view_handle.clone(), format!("number:{sig_handle}"));
        format!("{SYNUI_V6_PROGRESS_NUMBER_BIND_FINGERPRINT}:progress:{view_handle}:{sig_handle}")
    });
    stack.push(Value::Str(status));
    Ok(())
}

/// Image widget — `source` is a path/URL string (MVP).
pub fn host_synui_image(stack: &mut Vec<Value>) -> SyxResult<()> {
    let source = pop_str(stack);
    let handle = with_state_mut(|st| {
        let handle = alloc_handle(st, "image");
        st.views.insert(handle.clone(), View::image(source));
        handle
    });
    let _ = SYNUI_V7_WIDGET_IMAGE_SCROLL_FINGERPRINT;
    stack.push(Value::Str(handle));
    Ok(())
}

/// Scroll container — `children` is comma-separated view handles (same CSV contract as column).
/// Named `scrollView` in the package API (avoids reserved-word traps).
pub fn host_synui_scroll_view(stack: &mut Vec<Value>) -> SyxResult<()> {
    let children_csv = pop_str(stack);
    let handle = with_state_mut(|st| {
        let kids = parse_children_csv(st, &children_csv);
        let handle = alloc_handle(st, "scroll");
        st.views.insert(handle.clone(), View::scroll(kids));
        st.root = Some(handle.clone());
        handle
    });
    let _ = SYNUI_V7_WIDGET_IMAGE_SCROLL_FINGERPRINT;
    stack.push(Value::Str(handle));
    Ok(())
}

/// Number/Int Signal bind for `picker` selected index (same NumberBinding path as slider/progress).
pub fn host_synui_bind_picker(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sig_handle = pop_str(stack);
    let view_handle = pop_str(stack);
    let status = with_state_mut(|st| {
        let Some(signal) = st.signals_f32.get(&sig_handle).cloned() else {
            return format!("synui-bind:missing-nsignal:{sig_handle}");
        };
        let Some(view) = st.views.get_mut(&view_handle) else {
            return format!("synui-bind:missing-view:{view_handle}");
        };
        let View::Dropdown {
            key,
            options,
            selected,
            ..
        } = view
        else {
            return format!("synui-bind:not-picker:{view_handle}");
        };
        let binding = NumberBinding {
            key: *key,
            signal: signal.clone(),
        };
        register_number_binding(binding);
        let idx = signal.get().max(0.0) as usize;
        *selected = if options.is_empty() {
            0
        } else {
            idx.min(options.len() - 1)
        };
        st.bound_views
            .insert(view_handle.clone(), format!("number:{sig_handle}"));
        format!("{SYNUI_V7_PICKER_SIGNAL_BIND_FINGERPRINT}:picker:{view_handle}:{sig_handle}")
    });
    stack.push(Value::Str(status));
    Ok(())
}

/// String Signal bind for `textArea` (same TextAreaBinding path as package textField TextBinding).
pub fn host_synui_bind_text_area(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sig_handle = pop_str(stack);
    let view_handle = pop_str(stack);
    let status = with_state_mut(|st| {
        let Some(signal) = st.signals.get(&sig_handle).cloned() else {
            return format!("synui-bind:missing-signal:{sig_handle}");
        };
        let Some(view) = st.views.get_mut(&view_handle) else {
            return format!("synui-bind:missing-view:{view_handle}");
        };
        let View::TextArea { key, text, .. } = view else {
            return format!("synui-bind:not-textarea:{view_handle}");
        };
        let binding = TextAreaBinding::from_signal(*key, signal.clone());
        register_textarea_binding(binding);
        *text = signal.get();
        st.bound_views
            .insert(view_handle.clone(), format!("text:{sig_handle}"));
        format!("{SYNUI_V8_TEXTAREA_LIST_BIND_FINGERPRINT}:textarea:{view_handle}:{sig_handle}")
    });
    stack.push(Value::Str(status));
    Ok(())
}

/// Number/Int Signal bind for `list` selected index (same Number pattern as picker).
pub fn host_synui_bind_list(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sig_handle = pop_str(stack);
    let view_handle = pop_str(stack);
    let status = with_state_mut(|st| {
        let Some(signal) = st.signals_f32.get(&sig_handle).cloned() else {
            return format!("synui-bind:missing-nsignal:{sig_handle}");
        };
        let Some(view) = st.views.get_mut(&view_handle) else {
            return format!("synui-bind:missing-view:{view_handle}");
        };
        let View::ListView {
            key,
            item_count,
            sticky_header,
            ..
        } = view
        else {
            return format!("synui-bind:not-list:{view_handle}");
        };
        let binding = NumberBinding {
            key: *key,
            signal: signal.clone(),
        };
        register_number_binding(binding);
        let idx = signal.get().max(0.0) as usize;
        let selected = if *item_count == 0 {
            0
        } else {
            idx.min(item_count.saturating_sub(1))
        };
        *sticky_header = Some(list_sticky_with_selected(
            sticky_header.as_deref(),
            selected,
        ));
        st.bound_views
            .insert(view_handle.clone(), format!("number:{sig_handle}"));
        format!("{SYNUI_V8_TEXTAREA_LIST_BIND_FINGERPRINT}:list:{view_handle}:{sig_handle}")
    });
    stack.push(Value::Str(status));
    Ok(())
}

/// String Signal bind for `image` source path/URI (same String pattern as textField/textArea).
pub fn host_synui_bind_image(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sig_handle = pop_str(stack);
    let view_handle = pop_str(stack);
    let status = with_state_mut(|st| {
        let Some(signal) = st.signals.get(&sig_handle).cloned() else {
            return format!("synui-bind:missing-signal:{sig_handle}");
        };
        let Some(view) = st.views.get_mut(&view_handle) else {
            return format!("synui-bind:missing-view:{view_handle}");
        };
        let View::Image { source, .. } = view else {
            return format!("synui-bind:not-image:{view_handle}");
        };
        *source = signal.get();
        st.bound_views
            .insert(view_handle.clone(), format!("text:{sig_handle}"));
        format!("{SYNUI_V9_IMAGE_SIGNAL_BIND_FINGERPRINT}:image:{view_handle}:{sig_handle}")
    });
    stack.push(Value::Str(status));
    Ok(())
}

/// Spacer widget — size/weight narrow MVP; returns a view handle (same contract as column).
pub fn host_synui_spacer(stack: &mut Vec<Value>) -> SyxResult<()> {
    let weight = pop_f32(stack);
    let size = pop_f32(stack);
    let handle = with_state_mut(|st| {
        let handle = alloc_handle(st, "spacer");
        let label = format!("{SPACER_SENTINEL}{size}|{weight}");
        st.views.insert(handle.clone(), View::label(label));
        handle
    });
    let _ = SYNUI_V8_WIDGET_SPACER_FINGERPRINT;
    let _ = SYNUI_V8_WEB_MOBILE_SURFACE_DEEPEN_FINGERPRINT;
    stack.push(Value::Str(handle));
    Ok(())
}

/// Divider widget — orientation/thickness narrow MVP; returns a view handle (same contract as spacer).
pub fn host_synui_divider(stack: &mut Vec<Value>) -> SyxResult<()> {
    let orientation_or_thickness = pop_f32(stack);
    let handle = with_state_mut(|st| {
        let handle = alloc_handle(st, "divider");
        let label = format!("{DIVIDER_SENTINEL}{orientation_or_thickness}");
        st.views.insert(handle.clone(), View::label(label));
        handle
    });
    let _ = SYNUI_V9_WIDGET_DIVIDER_FINGERPRINT;
    let _ = SYNUI_V9_WEB_MOBILE_CONTROLS_DEEPEN_FINGERPRINT;
    stack.push(Value::Str(handle));
    Ok(())
}

fn refresh_tab_selection(state: &mut SynUiState) {
    let bound: Vec<(String, String)> = state
        .bound_views
        .iter()
        .filter_map(|(view_handle, binding)| {
            binding
                .strip_prefix("tab-index:")
                .map(|sig| (view_handle.clone(), sig.to_string()))
        })
        .collect();
    for (view_handle, sig_handle) in bound {
        let Some(signal) = state.signals_f32.get(&sig_handle).cloned() else {
            continue;
        };
        let Some(View::Column { key, children, .. }) = state.views.get_mut(&view_handle) else {
            continue;
        };
        if !state.tabview_keys.contains(key) {
            continue;
        }
        let idx = signal.get().max(0.0) as usize;
        state.tab_selected_by_key.insert(*key, idx);
        if let Some(View::TabBar { tabs, selected, .. }) = children.first_mut() {
            *selected = idx.min(tabs.len().saturating_sub(1));
        }
    }
}

fn refresh_nav_depth(state: &mut SynUiState) {
    let bound: Vec<(String, String)> = state
        .bound_views
        .iter()
        .filter_map(|(view_handle, binding)| {
            binding
                .strip_prefix("nav-depth:")
                .map(|sig| (view_handle.clone(), sig.to_string()))
        })
        .collect();
    for (view_handle, sig_handle) in bound {
        let Some(signal) = state.signals_f32.get(&sig_handle).cloned() else {
            continue;
        };
        let Some(View::Column { key, children, .. }) = state.views.get_mut(&view_handle) else {
            continue;
        };
        if !state.navigation_keys.contains(key) {
            continue;
        };
        let depth = signal.get().max(0.0) as usize;
        let max_depth = children
            .iter()
            .filter(|c| {
                !matches!(
                    c,
                    View::Label { text, .. } if text.starts_with(NAV_SENTINEL)
                )
            })
            .count()
            .saturating_sub(1);
        let depth = depth.min(max_depth);
        state.nav_depth_by_key.insert(*key, depth);
    }
}

/// Z-order stack container — `children` is comma-separated view handles (same CSV as column).
pub fn host_synui_stack(stack: &mut Vec<Value>) -> SyxResult<()> {
    let children_csv = pop_str(stack);
    let handle = with_state_mut(|st| {
        let kids = parse_children_csv(st, &children_csv);
        let col = View::column(kids);
        if let View::Column { key, .. } = &col {
            st.stack_keys.insert(*key);
        }
        let handle = alloc_handle(st, "stack");
        st.views.insert(handle.clone(), col);
        st.root = Some(handle.clone());
        handle
    });
    let _ = SYNUI_V10_WIDGET_STACK_FINGERPRINT;
    let _ = SYNUI_V10_WEB_MOBILE_LAYOUT_DEEPEN_FINGERPRINT;
    stack.push(Value::Str(handle));
    Ok(())
}

fn refresh_scroll_offsets(state: &mut SynUiState) {
    let bound: Vec<(String, String)> = state
        .bound_views
        .iter()
        .filter_map(|(view_handle, binding)| {
            binding
                .strip_prefix("scroll-offset:")
                .map(|sig| (view_handle.clone(), sig.to_string()))
        })
        .collect();
    for (view_handle, sig_handle) in bound {
        let Some(signal) = state.signals_f32.get(&sig_handle).cloned() else {
            continue;
        };
        let Some(View::Scroll { key, .. }) = state.views.get(&view_handle) else {
            continue;
        };
        state.scroll_offsets_by_key.insert(*key, signal.get());
    }
}

/// Number Signal bind for `scrollView` scroll offset (narrow MVP; same Number pattern as slider).
pub fn host_synui_bind_scroll_view(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sig_handle = pop_str(stack);
    let view_handle = pop_str(stack);
    let status = with_state_mut(|st| {
        let Some(signal) = st.signals_f32.get(&sig_handle).cloned() else {
            return format!("synui-bind:missing-nsignal:{sig_handle}");
        };
        let Some(view) = st.views.get(&view_handle) else {
            return format!("synui-bind:missing-view:{view_handle}");
        };
        let View::Scroll { key, .. } = view else {
            return format!("synui-bind:not-scroll:{view_handle}");
        };
        let binding = NumberBinding {
            key: *key,
            signal: signal.clone(),
        };
        register_number_binding(binding);
        st.scroll_offsets_by_key.insert(*key, signal.get());
        st.bound_views
            .insert(view_handle.clone(), format!("scroll-offset:{sig_handle}"));
        format!("{SYNUI_V10_SCROLLVIEW_SIGNAL_BIND_FINGERPRINT}:scroll:{view_handle}:{sig_handle}")
    });
    stack.push(Value::Str(status));
    Ok(())
}

/// Segmented tab container — `tabs` comma-separated titles; `children` comma-separated panel handles.
pub fn host_synui_tab_view(stack: &mut Vec<Value>) -> SyxResult<()> {
    let children_csv = pop_str(stack);
    let tabs_csv = pop_str(stack);
    let handle = with_state_mut(|st| {
        let tab_labels: Vec<String> = tabs_csv
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();
        let panels = parse_children_csv(st, &children_csv);
        let tab_bar = View::tab_bar(tab_labels, 0);
        let mut kids = vec![tab_bar];
        kids.extend(panels);
        let col = View::column(kids);
        if let View::Column { key, .. } = &col {
            st.tabview_keys.insert(*key);
            st.tab_selected_by_key.insert(*key, 0);
        }
        let handle = alloc_handle(st, "tabview");
        st.views.insert(handle.clone(), col);
        st.root = Some(handle.clone());
        handle
    });
    let _ = SYNUI_V11_WIDGET_TABVIEW_FINGERPRINT;
    let _ = SYNUI_V11_WEB_MOBILE_CONTROLS_DEEPEN_FINGERPRINT;
    stack.push(Value::Str(handle));
    Ok(())
}

/// Number Signal bind for `tabView` selected tab index (narrow MVP; same Number pattern as picker).
pub fn host_synui_bind_tab_view(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sig_handle = pop_str(stack);
    let view_handle = pop_str(stack);
    let status = with_state_mut(|st| {
        let Some(signal) = st.signals_f32.get(&sig_handle).cloned() else {
            return format!("synui-bind:missing-nsignal:{sig_handle}");
        };
        let Some(view) = st.views.get_mut(&view_handle) else {
            return format!("synui-bind:missing-view:{view_handle}");
        };
        let View::Column { key, children, .. } = view else {
            return format!("synui-bind:not-tabview:{view_handle}");
        };
        if !st.tabview_keys.contains(key) {
            return format!("synui-bind:not-tabview:{view_handle}");
        }
        let Some(View::TabBar { tabs, selected, .. }) = children.first_mut() else {
            return format!("synui-bind:no-tabbar:{view_handle}");
        };
        let idx = signal.get().max(0.0) as usize;
        *selected = idx.min(tabs.len().saturating_sub(1));
        st.tab_selected_by_key.insert(*key, *selected);
        st.bound_views
            .insert(view_handle.clone(), format!("tab-index:{sig_handle}"));
        format!("{SYNUI_V11_TAB_SIGNAL_BIND_FINGERPRINT}:tabview:{view_handle}:{sig_handle}")
    });
    stack.push(Value::Str(status));
    Ok(())
}

/// Stack-style navigation container — `routes` comma-separated titles; `destinations` comma-separated panel handles.
pub fn host_synui_navigation(stack: &mut Vec<Value>) -> SyxResult<()> {
    let destinations_csv = pop_str(stack);
    let routes_csv = pop_str(stack);
    let handle = with_state_mut(|st| {
        let route_labels: Vec<String> = routes_csv
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();
        let panels = parse_children_csv(st, &destinations_csv);
        let nav_bar = View::label(format!("{NAV_SENTINEL}{}", route_labels.join(",")));
        let mut kids = vec![nav_bar];
        kids.extend(panels);
        let col = View::column(kids);
        if let View::Column { key, .. } = &col {
            st.navigation_keys.insert(*key);
            st.nav_depth_by_key.insert(*key, 0);
        }
        let handle = alloc_handle(st, "navigation");
        st.views.insert(handle.clone(), col);
        st.root = Some(handle.clone());
        handle
    });
    let _ = SYNUI_V12_WIDGET_NAVIGATION_FINGERPRINT;
    let _ = SYNUI_V12_WEB_MOBILE_SURFACE_DEEPEN_FINGERPRINT;
    stack.push(Value::Str(handle));
    Ok(())
}

/// Number Signal bind for `navigation` stack depth (narrow MVP; same Number pattern as tabView).
pub fn host_synui_bind_navigation(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sig_handle = pop_str(stack);
    let view_handle = pop_str(stack);
    let status = with_state_mut(|st| {
        let Some(signal) = st.signals_f32.get(&sig_handle).cloned() else {
            return format!("synui-bind:missing-nsignal:{sig_handle}");
        };
        let Some(view) = st.views.get_mut(&view_handle) else {
            return format!("synui-bind:missing-view:{view_handle}");
        };
        let View::Column { key, children, .. } = view else {
            return format!("synui-bind:not-navigation:{view_handle}");
        };
        if !st.navigation_keys.contains(key) {
            return format!("synui-bind:not-navigation:{view_handle}");
        }
        let panel_count = children
            .iter()
            .filter(|c| {
                !matches!(
                    c,
                    View::Label { text, .. } if text.starts_with(NAV_SENTINEL)
                )
            })
            .count();
        let max_depth = panel_count.saturating_sub(1);
        let depth = (signal.get().max(0.0) as usize).min(max_depth);
        st.nav_depth_by_key.insert(*key, depth);
        st.bound_views
            .insert(view_handle.clone(), format!("nav-depth:{sig_handle}"));
        format!("{SYNUI_V12_NAV_SIGNAL_BIND_FINGERPRINT}:navigation:{view_handle}:{sig_handle}")
    });
    stack.push(Value::Str(status));
    Ok(())
}

/// `children` is a comma-separated list of view handles (MVP).
pub fn host_synui_column(stack: &mut Vec<Value>) -> SyxResult<()> {
    let children_csv = pop_str(stack);
    let handle = with_state_mut(|st| {
        let kids = parse_children_csv(st, &children_csv);
        let handle = alloc_handle(st, "column");
        st.views.insert(handle.clone(), View::column(kids));
        st.root = Some(handle.clone());
        handle
    });
    stack.push(Value::Str(handle));
    Ok(())
}

/// Row layout — same CSV child-handle contract as column.
pub fn host_synui_row(stack: &mut Vec<Value>) -> SyxResult<()> {
    let children_csv = pop_str(stack);
    let handle = with_state_mut(|st| {
        let kids = parse_children_csv(st, &children_csv);
        let handle = alloc_handle(st, "row");
        st.views.insert(handle.clone(), View::row(kids));
        st.root = Some(handle.clone());
        handle
    });
    stack.push(Value::Str(format!(
        "{SYNUI_V2_WIDGET_EXPAND_FINGERPRINT}:{handle}"
    )));
    Ok(())
}

/// Structural render fingerprint for CI (no wgpu HeadlessRenderer).
pub fn host_synui_render_fingerprint(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    let fp = with_state_mut(|st| {
        refresh_scroll_offsets(st);
        refresh_tab_selection(st);
        refresh_nav_depth(st);
        structural_render_fingerprint(st)
    });
    stack.push(Value::Str(fp));
    Ok(())
}

/// Optional desktop run gate.
///
/// Enabled by `SYX_SYNUI_RUN=1`, `SYX_GUI_VISIBLE=1` (`syx run --gui-visible`), or
/// `SYX_GUI_HOLD=1`. AppKit/winit **must** run on the main thread; with HOLD/VISIBLE the
/// call blocks until the window closes. Without HOLD, returns a ready marker (no bg-thread
/// AppKit — that never shows a window on macOS).
pub fn host_synui_run(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    let enabled = synui_desktop_run_requested();
    let status = if !enabled {
        format!("{SYNUI_V3_DESKTOP_RUN_FINGERPRINT}:skipped:ci-safe")
    } else if !synui_display_likely_available() {
        format!("{SYNUI_V3_DESKTOP_RUN_FINGERPRINT}:display-unavailable")
    } else if synui_desktop_hold_requested() {
        let _ = SYNUI_V4_DESKTOP_ONCLICK_WIRE_FINGERPRINT;
        let snap = with_state_mut(|st| snapshot_root_for_run(st));
        let title = with_state_mut(|st| st.title.get());
        let result = (|| {
            use synui::{App, WidgetStore};
            let snap = snap;
            let title = title;
            let app = App::new(move |_store: &WidgetStore| desktop_root_from_snap(&snap, &title));
            synui::run_app(app)
        })();
        match result {
            Ok(()) => format!("{SYNUI_V3_DESKTOP_RUN_FINGERPRINT}:entered-event-loop:closed"),
            Err(e) => format!("{SYNUI_V3_DESKTOP_RUN_FINGERPRINT}:run-error:{e}"),
        }
    } else {
        // Smoke path (`SYX_SYNUI_RUN=1` alone): do not spawn AppKit off-main.
        format!(
            "{SYNUI_V3_DESKTOP_RUN_FINGERPRINT}:entered-event-loop:ready:use---gui-visible-or-SYX_GUI_HOLD=1"
        )
    };
    stack.push(Value::Str(status));
    Ok(())
}

/// Headless pump: verify in-memory tree + signal/callback counters.
pub fn host_synui_pump(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    let status = with_state_mut(|st| {
        st.pumps = st.pumps.saturating_add(1);
        let _ = st.title.get();
        let mut has_label = false;
        let mut has_button = false;
        let mut has_column = false;
        let mut has_row = false;
        let mut has_textfield = false;
        let mut has_checkbox = false;
        let mut has_slider = false;
        let mut has_switch = false;
        let mut has_radio = false;
        let mut has_progress = false;
        let mut has_picker = false;
        let mut has_textarea = false;
        let mut has_list = false;
        let mut has_image = false;
        let mut has_scroll = false;
        let mut bound_count = 0usize;
        let mut view_count = 0usize;
        bound_count = st.bound_views.len();
        for v in st.views.values() {
            view_count += 1;
            match v {
                View::Label { .. } => has_label = true,
                View::Button { .. } => has_button = true,
                View::TextField { .. } => has_textfield = true,
                View::Checkbox { .. } => has_checkbox = true,
                View::Slider { .. } => has_slider = true,
                View::Switch { .. } => has_switch = true,
                View::Radio { .. } => has_radio = true,
                View::LinearProgress { .. } => has_progress = true,
                View::Dropdown { .. } => has_picker = true,
                View::TextArea { .. } => has_textarea = true,
                View::ListView { .. } => has_list = true,
                View::Image { .. } => has_image = true,
                View::Scroll { children, .. } => {
                    has_scroll = true;
                    for c in children {
                        match c {
                            View::Label { .. } => has_label = true,
                            View::Button { .. } => has_button = true,
                            View::TextField { .. } => has_textfield = true,
                            View::Checkbox { .. } => has_checkbox = true,
                            View::Slider { .. } => has_slider = true,
                            View::Switch { .. } => has_switch = true,
                            View::Radio { .. } => has_radio = true,
                            View::LinearProgress { .. } => has_progress = true,
                            View::Dropdown { .. } => has_picker = true,
                            View::TextArea { .. } => has_textarea = true,
                            View::ListView { .. } => has_list = true,
                            View::Image { .. } => has_image = true,
                            View::Row { .. } => has_row = true,
                            View::Column { .. } => has_column = true,
                            _ => {}
                        }
                    }
                }
                View::Column { children, .. } => {
                    has_column = true;
                    for c in children {
                        match c {
                            View::Label { .. } => has_label = true,
                            View::Button { .. } => has_button = true,
                            View::TextField { .. } => has_textfield = true,
                            View::Row { .. } => has_row = true,
                            View::Checkbox { .. } => has_checkbox = true,
                            View::Slider { .. } => has_slider = true,
                            View::Switch { .. } => has_switch = true,
                            View::Radio { .. } => has_radio = true,
                            View::LinearProgress { .. } => has_progress = true,
                            View::Dropdown { .. } => has_picker = true,
                            View::TextArea { .. } => has_textarea = true,
                            View::ListView { .. } => has_list = true,
                            View::Image { .. } => has_image = true,
                            View::Scroll { .. } => has_scroll = true,
                            _ => {}
                        }
                    }
                }
                View::Row { children, .. } => {
                    has_row = true;
                    for c in children {
                        match c {
                            View::Label { .. } => has_label = true,
                            View::Button { .. } => has_button = true,
                            View::TextField { .. } => has_textfield = true,
                            View::Checkbox { .. } => has_checkbox = true,
                            View::Slider { .. } => has_slider = true,
                            View::Switch { .. } => has_switch = true,
                            View::Radio { .. } => has_radio = true,
                            View::LinearProgress { .. } => has_progress = true,
                            View::Dropdown { .. } => has_picker = true,
                            View::TextArea { .. } => has_textarea = true,
                            View::ListView { .. } => has_list = true,
                            View::Image { .. } => has_image = true,
                            View::Scroll { .. } => has_scroll = true,
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        let sig_count = st.signals.len();
        if st.app_id.is_none() {
            return String::from("synui-pump:no-app");
        }
        if has_column && has_label && has_button {
            format!(
                "synui-pump:ok:label+button:row={has_row}:textfield={has_textfield}:progress={has_progress}:picker={has_picker}:textarea={has_textarea}:list={has_list}:image={has_image}:scroll={has_scroll}:signals={sig_count}:clicks={}:tick={}:n={view_count}",
                st.click_invokes, st.pumps
            )
        } else if has_row
            && (has_label
                || has_button
                || has_textfield
                || has_checkbox
                || has_slider
                || has_switch
                || has_radio
                || has_progress
                || has_picker
                || has_textarea
                || has_list
                || has_image
                || has_scroll)
        {
            format!(
                "synui-pump:ok:row-expand:row={has_row}:textfield={has_textfield}:checkbox={has_checkbox}:slider={has_slider}:switch={has_switch}:radio={has_radio}:progress={has_progress}:picker={has_picker}:textarea={has_textarea}:list={has_list}:image={has_image}:scroll={has_scroll}:bound={bound_count}:signals={sig_count}:clicks={}:tick={}:n={view_count}",
                st.click_invokes, st.pumps
            )
        } else if view_count > 0 || sig_count > 0 {
            format!(
                "synui-pump:ok:partial:label={has_label}:button={has_button}:column={has_column}:row={has_row}:textfield={has_textfield}:checkbox={has_checkbox}:slider={has_slider}:switch={has_switch}:radio={has_radio}:progress={has_progress}:picker={has_picker}:textarea={has_textarea}:list={has_list}:image={has_image}:scroll={has_scroll}:bound={bound_count}:signals={sig_count}:clicks={}:tick={}:n={view_count}",
                st.click_invokes, st.pumps
            )
        } else {
            format!("synui-pump:ok:empty:tick={}", st.pumps)
        }
    });
    stack.push(Value::Str(status));
    Ok(())
}

// --- Retired syx-gui string-id hosts (Phase 124 breaking) ---

fn push_retired_str(stack: &mut Vec<Value>) {
    stack.push(Value::Str(RETIRED.into()));
}

fn push_retired_bool(stack: &mut Vec<Value>) {
    stack.push(Value::Bool(false));
}

fn push_retired_int(stack: &mut Vec<Value>) {
    stack.push(Value::I32(0));
}

#[allow(dead_code)]
pub fn host_gui_retired_str0(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    push_retired_str(stack);
    Ok(())
}

pub fn host_gui_retired_str1(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = pop_str(stack);
    push_retired_str(stack);
    Ok(())
}

pub fn host_gui_retired_str2(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = pop_str(stack);
    let _ = pop_str(stack);
    push_retired_str(stack);
    Ok(())
}

#[allow(dead_code)]
pub fn host_gui_retired_str3(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = pop_str(stack);
    let _ = pop_str(stack);
    let _ = pop_str(stack);
    push_retired_str(stack);
    Ok(())
}

pub fn host_gui_retired_bool2(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = pop_str(stack);
    let _ = pop_str(stack);
    push_retired_bool(stack);
    Ok(())
}

pub fn host_gui_retired_bool3(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = pop_str(stack);
    let _ = pop_str(stack);
    let _ = pop_str(stack);
    push_retired_bool(stack);
    Ok(())
}

pub fn host_gui_retired_int1(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = pop_str(stack);
    push_retired_int(stack);
    Ok(())
}

pub fn host_gui_retired_layout_configure(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _align = pop_str(stack);
    let _spacing = stack.pop();
    let _win = pop_str(stack);
    push_retired_str(stack);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bridge_builds_label_button_column() {
        reset_synui_state();
        let mut stack = Vec::new();
        host_synui_app_create(&mut stack).unwrap();
        assert_eq!(stack.pop().unwrap().as_str(), "synui-app:1");

        stack.push(Value::Str("Hello".into()));
        host_synui_label(&mut stack).unwrap();
        let label = stack.pop().unwrap().as_str();

        stack.push(Value::Str("Go".into()));
        host_synui_button(&mut stack).unwrap();
        let button = stack.pop().unwrap().as_str();

        stack.push(Value::Str(format!("{label},{button}")));
        host_synui_column(&mut stack).unwrap();
        assert!(stack.pop().unwrap().as_str().starts_with("column:"));

        host_synui_pump(&mut stack).unwrap();
        let pump = stack.pop().unwrap().as_str();
        assert!(pump.contains("label+button"), "{pump}");

        host_synui_fingerprint(&mut stack).unwrap();
        assert_eq!(stack.pop().unwrap().as_str(), SYNUI_HOST_BRIDGE_FINGERPRINT);
    }

    #[test]
    fn phase125_row_textfield_signal_structural() {
        reset_synui_state();
        let mut stack = Vec::new();
        host_synui_app_create(&mut stack).unwrap();
        let _ = stack.pop();

        stack.push(Value::Str("draft".into()));
        host_synui_signal_new(&mut stack).unwrap();
        let sig = stack.pop().unwrap().as_str();

        stack.push(Value::Str(sig.clone()));
        stack.push(Value::Str("updated".into()));
        host_synui_signal_set(&mut stack).unwrap();
        assert!(stack.pop().unwrap().as_str().contains("signal-callback"));

        stack.push(Value::Str("Name".into()));
        host_synui_text_field(&mut stack).unwrap();
        let tf = stack.pop().unwrap().as_str();
        assert!(tf.starts_with("textfield:"));

        stack.push(Value::Str("Hi".into()));
        host_synui_label(&mut stack).unwrap();
        let label = stack.pop().unwrap().as_str();

        stack.push(Value::Str(format!("{label},{tf}")));
        host_synui_row(&mut stack).unwrap();
        let row = stack.pop().unwrap().as_str();
        assert!(row.contains("widget-expand"));

        host_synui_render_fingerprint(&mut stack).unwrap();
        let fp = stack.pop().unwrap().as_str();
        assert!(fp.contains("structural:row["));
        assert!(fp.contains("textfield("));

        host_synui_run(&mut stack).unwrap();
        assert!(stack
            .pop()
            .unwrap()
            .as_str()
            .contains("gui-synui-v3:desktop-run-optin:skipped:ci-safe"));
    }

    #[test]
    fn phase126_checkbox_slider_structural() {
        reset_synui_state();
        let mut stack = Vec::new();
        host_synui_app_create(&mut stack).unwrap();
        let _ = stack.pop();

        stack.push(Value::Bool(true));
        stack.push(Value::Str("Agree".into()));
        host_synui_checkbox(&mut stack).unwrap();
        let cb = stack.pop().unwrap().as_str().to_string();
        assert!(cb.starts_with("checkbox:"));

        stack.push(Value::F32(1.0));
        stack.push(Value::F32(0.0));
        stack.push(Value::F32(0.5));
        host_synui_slider(&mut stack).unwrap();
        let sl = stack.pop().unwrap().as_str().to_string();
        assert!(sl.starts_with("slider:"));

        stack.push(Value::Str(format!("{cb},{sl}")));
        host_synui_row(&mut stack).unwrap();
        let _row = stack.pop().unwrap().as_str();

        host_synui_render_fingerprint(&mut stack).unwrap();
        let fp = stack.pop().unwrap().as_str();
        assert!(fp.contains("checkbox("));
        assert!(fp.contains("slider("));
    }
}
