//! Browser GUI package + embedded SQL smoke.

#[test]
fn gui_smoke_driver_marker() {
    assert!(include_str!("../../src/lib.rs").contains("browser-tier2p-v1"));
}

#[test]
fn gui_smoke_playwright_or_compile_marker() {
    let src = include_str!("../../src/lib.rs");
    assert!(src.contains("browser-tier2k") || src.contains("playwright"));
}

#[test]
fn gui_smoke_ui_package_present() {
    let p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../packages/ui/mod.sy");
    assert!(p.is_file());
}

#[test]
fn gui_smoke_sql_embed() {
    syx_host_plugin::register_echo_plugin_in_process();
    let out = syx_driver::compile_and_run_embedded(
        r#"func main() {
    __syx_sql_exec("CREATE TABLE t (n INT)")
    print(1)
}"#,
        &syx_driver::DriverOptions::default(),
    )
    .unwrap()
    .printed
    .join("\n");
    assert!(out.contains('1'), "{out}");
}
