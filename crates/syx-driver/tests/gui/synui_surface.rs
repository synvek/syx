//! Phase 135 Track B — SynUI v12: navigation, nav Signal bind, Web/Mobile surface deepen.

use std::sync::Mutex;

use syx_driver::{compile_and_run_embedded, DriverOptions};
use syx_vm::{
    reset_synui_state, SYNUI_V12_NAV_SIGNAL_BIND_FINGERPRINT,
    SYNUI_V12_WEB_MOBILE_SURFACE_DEEPEN_FINGERPRINT, SYNUI_V12_WIDGET_NAVIGATION_FINGERPRINT,
};

static P135_SYNUI_ENV_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn p135_synui_v12_fingerprint_constants() {
    assert_eq!(
        SYNUI_V12_WIDGET_NAVIGATION_FINGERPRINT,
        "gui-synui-v12:widget-navigation"
    );
    assert_eq!(
        SYNUI_V12_NAV_SIGNAL_BIND_FINGERPRINT,
        "gui-synui-v12:nav-signal-bind"
    );
    assert_eq!(
        SYNUI_V12_WEB_MOBILE_SURFACE_DEEPEN_FINGERPRINT,
        "gui-synui-v12:web-mobile-surface-deepen"
    );
    let src = include_str!("../../../syx-vm/src/synui_hosts.rs");
    assert!(src.contains("gui-synui-v12:widget-navigation"));
    assert!(src.contains("gui-synui-v12:nav-signal-bind"));
    assert!(src.contains("gui-synui-v12:web-mobile-surface-deepen"));
}

#[test]
fn p135_synui_v12_ui_package_surface() {
    let ui = include_str!("../../../../packages/ui/mod.sy");
    assert!(ui.contains("__syx_synui_navigation"));
    assert!(ui.contains("__syx_synui_bind_navigation"));
    assert!(ui.contains("public func navigation("));
    assert!(ui.contains("public func bindNavigation("));
    assert!(ui.contains("gui-synui-v12:widget-navigation"));
    assert!(ui.contains("gui-synui-v12:nav-signal-bind"));
    assert!(ui.contains("gui-synui-v12:web-mobile-surface-deepen"));
}

#[test]
fn p135_synui_navigation_e2e_embedded() {
    let _guard = P135_SYNUI_ENV_LOCK.lock().unwrap();
    reset_synui_state();
    let out = compile_and_run_embedded(
        r#"
func main() {
    let _a = __syx_synui_app_create()
    let home = __syx_synui_label("home-panel")
    let settings = __syx_synui_label("settings-panel")
    let nav = __syx_synui_navigation("Home,Settings", "\(home),\(settings)")
    let _ = nav
    print(__syx_synui_render_fingerprint())
    print(__syx_synui_pump())
}
"#,
        &DriverOptions::default(),
    )
    .expect("compile_and_run_embedded synui v12 navigation")
    .printed
    .join("\n");

    assert!(
        out.contains("gui-synui-v12:widget-navigation") || out.contains("navigation["),
        "{out}"
    );
    assert!(
        out.contains("structural:navigation[depth=0][navbar(Home,Settings)"),
        "structural fingerprint must use navigation container: {out}"
    );
}

#[test]
fn p135_synui_nav_bind_e2e_embedded() {
    let _guard = P135_SYNUI_ENV_LOCK.lock().unwrap();
    reset_synui_state();
    let out = compile_and_run_embedded(
        r#"
func main() {
    let _a = __syx_synui_app_create()
    let navSig = __syx_synui_signal_number_new(0.0)
    print(__syx_synui_signal_number_set(navSig, 1.0))
    let home = __syx_synui_label("home-panel")
    let settings = __syx_synui_label("settings-panel")
    let nav = __syx_synui_navigation("Home,Settings", "\(home),\(settings)")
    print(__syx_synui_bind_navigation(nav, navSig))
    let _ = __syx_synui_column("\(nav)")
    print(__syx_synui_render_fingerprint())
    print(__syx_synui_pump())
}
"#,
        &DriverOptions::default(),
    )
    .expect("compile_and_run_embedded synui v12 nav bind")
    .printed
    .join("\n");

    assert!(out.contains("gui-synui-v12:nav-signal-bind"), "{out}");
    assert!(
        out.contains("navigation[depth=1]"),
        "bound navigation must reflect Number signal stack depth: {out}"
    );
}

#[test]
fn p135_synui_web_mobile_surface_deepen() {
    let web = include_str!("../../../../packages/ui/web.sy");
    assert!(web.contains("__syx_synui_navigation"));
    assert!(web.contains("__syx_synui_bind_navigation"));
    assert!(web.contains("__syx_synui_tab_view"));
    assert!(web.contains("__syx_synui_bind_tab_view"));
    assert!(web.contains("__syx_synui_render_fingerprint"));
    assert!(web.contains("public func navigation("));
    assert!(web.contains("public func bindNavigation("));
    assert!(web.contains("public func renderFingerprint()"));
    assert!(web.contains("gui-synui-v12:web-mobile-surface-deepen"));
    assert!(web.contains("public func pump()"));
    assert!(web.contains("public func run()"));

    let mobile = include_str!("../../../../packages/ui/mobile.sy");
    assert!(mobile.contains("__syx_synui_navigation"));
    assert!(mobile.contains("__syx_synui_bind_navigation"));
    assert!(mobile.contains("__syx_synui_tab_view"));
    assert!(mobile.contains("__syx_synui_bind_tab_view"));
    assert!(mobile.contains("__syx_synui_render_fingerprint"));
    assert!(mobile.contains("gui-synui-v12:web-mobile-surface-deepen"));
    assert!(mobile.contains("Phase 136+"));
}

#[test]
fn p135_synui_web_mobile_surface_e2e_embedded() {
    let _guard = P135_SYNUI_ENV_LOCK.lock().unwrap();
    reset_synui_state();
    let out = compile_and_run_embedded(
        r#"
func main() {
    let _a = __syx_synui_app_create()
    let navSig = __syx_synui_signal_number_new(0.0)
    print(__syx_synui_signal_number_set(navSig, 1.0))
    let home = __syx_synui_label("web-mobile-home")
    let settings = __syx_synui_label("web-mobile-settings")
    let nav = __syx_synui_navigation("Home,Settings", "\(home),\(settings)")
    print(__syx_synui_bind_navigation(nav, navSig))
    let tabSig = __syx_synui_signal_number_new(0.0)
    print(__syx_synui_signal_number_set(tabSig, 1.0))
    let tabHome = __syx_synui_label("tab-home")
    let tabSettings = __syx_synui_label("tab-settings")
    let tv = __syx_synui_tab_view("Home,Settings", "\(tabHome),\(tabSettings)")
    print(__syx_synui_bind_tab_view(tv, tabSig))
    let _ = __syx_synui_column("\(nav),\(tv)")
    print(__syx_synui_render_fingerprint())
    print(__syx_synui_pump())
}
"#,
        &DriverOptions::default(),
    )
    .expect("compile_and_run_embedded synui v12 web/mobile surface deepen")
    .printed
    .join("\n");

    assert!(out.contains("navigation[depth=1]"), "{out}");
    assert!(
        out.contains("tabview[sel=1]") || out.contains("tabbar("),
        "{out}"
    );
    assert!(out.contains("synui-pump:ok"), "{out}");
}

#[test]
fn p135_synui_host_registry_v12() {
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(reg.contains("SynuiNavigation"));
    assert!(reg.contains("SynuiBindNavigation"));
    assert!(reg.contains("__syx_synui_navigation"));
    assert!(reg.contains("__syx_synui_bind_navigation"));
}

#[test]
fn p135_synui_guidemo_surface() {
    let demo = include_str!("../../../../examples/gui_demo.sy");
    assert!(demo.contains("navigation("));
    assert!(demo.contains("bindNavigation("));
}

#[test]
fn p135_synui_headless_widget_count_inherit() {
    let _guard = P135_SYNUI_ENV_LOCK.lock().unwrap();
    reset_synui_state();
    unsafe {
        std::env::set_var("SYX_SYNUI_HEADLESS", "1");
    }
    let out = compile_and_run_embedded(
        r#"
func main() {
    let _a = __syx_synui_app_create()
    let a = __syx_synui_label("a")
    let b = __syx_synui_label("b")
    let nav = __syx_synui_navigation("A,B", "\(a),\(b)")
    let _ = __syx_synui_column("\(nav)")
    print(__syx_synui_render_fingerprint())
}
"#,
        &DriverOptions::default(),
    )
    .expect("compile_and_run_embedded synui v12 headless count")
    .printed
    .join("\n");
    unsafe {
        std::env::remove_var("SYX_SYNUI_HEADLESS");
    }
    assert!(
        out.contains("widget-count="),
        "SYX_SYNUI_HEADLESS=1 must deepen structural fingerprint: {out}"
    );
    assert!(
        out.contains("navigation[") || out.contains("navbar("),
        "v12 navigation must appear in structural tree: {out}"
    );
}
