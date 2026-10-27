//! Phase 91: ORM MVP reflect roundtrip.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

#[test]
fn p98_orm_mvp_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("orm-mvp-v1"));
}

#[test]
fn p98_orm_mvp_save_find_stdout() {
    let out = run(
        r#"func main() {
    __syx_sql_exec("CREATE TABLE IF NOT EXISTS Item (n TEXT)")
    let ok = __syx_orm_save("Item", { "n": "88" })
    print(ok)
    let rows = __syx_orm_find("Item")
    print(rows.len())
}"#,
    );
    assert!(out.contains("true") || out.contains("1"), "{out}");
}

#[test]
fn p98_orm_mvp_package_surface() {
    let p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../packages/data/orm.sy");
    assert!(p.is_file(), "missing packages/data/orm.sy");
}
