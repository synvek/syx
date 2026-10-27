//! Stub SynUI hosts when `gui-synui` is disabled (default / server builds).
//!
//! Keeps the same public surface as [`crate::synui_hosts`] so VM dispatch and
//! fingerprint re-exports compile without linking the heavy `synui` path dep.

use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_runtime_core::Value;

/// Cache-v96 / Phase 124 GA host-bridge fingerprint.
pub const SYNUI_HOST_BRIDGE_FINGERPRINT: &str = "gui-synui-v1:host-bridge";
/// Path-dep / desktop MVP fingerprint.
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
const DISABLED: &str = "gui-synui:feature-disabled";

fn feature_disabled() -> SyxResult<()> {
    Err(SyxDiagnostic::error(
        "SynUI GUI is not enabled in this build; rebuild with `--features gui-synui` \
         (requires sibling checkout `../synui`)",
    )
    .with_code("V080"))
}

pub fn reset_synui_state() {}

pub fn synui_fingerprint() -> &'static str {
    DISABLED
}

pub fn host_synui_fingerprint(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    stack.push(Value::Str(DISABLED.into()));
    Ok(())
}

macro_rules! stub_synui {
    ($($name:ident),+ $(,)?) => {
        $(
            pub fn $name(_stack: &mut Vec<Value>) -> SyxResult<()> {
                feature_disabled()
            }
        )+
    };
}

stub_synui! {
    host_synui_app_create,
    host_synui_signal_new,
    host_synui_signal_get,
    host_synui_signal_set,
    host_synui_label,
    host_synui_button,
    host_synui_button_on_click,
    host_synui_button_click,
    host_synui_text_field,
    host_synui_checkbox,
    host_synui_slider,
    host_synui_switch,
    host_synui_radio,
    host_synui_progress,
    host_synui_picker,
    host_synui_text_area,
    host_synui_list,
    host_synui_signal_bool_new,
    host_synui_signal_bool_get,
    host_synui_signal_bool_set,
    host_synui_signal_number_new,
    host_synui_signal_number_get,
    host_synui_signal_number_set,
    host_synui_bind_text_field,
    host_synui_bind_checkbox,
    host_synui_bind_slider,
    host_synui_bind_switch,
    host_synui_bind_radio,
    host_synui_bind_progress,
    host_synui_image,
    host_synui_scroll_view,
    host_synui_bind_picker,
    host_synui_bind_text_area,
    host_synui_bind_list,
    host_synui_bind_image,
    host_synui_spacer,
    host_synui_divider,
    host_synui_stack,
    host_synui_bind_scroll_view,
    host_synui_tab_view,
    host_synui_bind_tab_view,
    host_synui_navigation,
    host_synui_bind_navigation,
    host_synui_column,
    host_synui_row,
    host_synui_render_fingerprint,
    host_synui_run,
    host_synui_pump,
}

fn pop_str(stack: &mut Vec<Value>) -> String {
    match stack.pop() {
        Some(v) => v.as_str(),
        None => String::new(),
    }
}

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
