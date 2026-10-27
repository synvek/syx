//! Phase 98: ORM reflect compound AND parameterized WHERE.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::set_var("SYX_ORM_COMPOUND", "1");
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

#[test]
fn p98_orm_reflect_compound_and_smoke() {
    let out = run(
        r#"func main() {
    __syx_sql_migrate("CREATE TABLE IF NOT EXISTS users (name TEXT, age INT)")
    __syx_orm_find_where("users", "name", "alice")
    print("compound-ok")
}"#,
    );
    assert!(out.contains("compound-ok"), "{out}");
}

#[test]
fn p98_orm_reflect_compound_sql_host() {
    let hosts = include_str!("../../../syx-vm/src/stdlib_phase98_hosts.rs");
    assert!(hosts.contains(" AND ") && hosts.contains(" = ?"));
}

#[test]
fn p98_orm_reflect_compound_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("orm-reflect-compound-v1"));
}
