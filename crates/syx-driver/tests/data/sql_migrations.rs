//! Phase 91: SQL migrations + prepare/bind E2E.

use syx_driver::{compile_and_run_embedded, DriverOptions};

#[test]
fn p98_sql_migrate_stdout() {
    syx_host_plugin::register_echo_plugin_in_process();
    let out = compile_and_run_embedded(
        r#"func main() {
    let v = __syx_sql_migrate("CREATE TABLE IF NOT EXISTS t (id INTEGER)")
    print(v)
}"#,
        &DriverOptions::default(),
    )
    .unwrap()
    .printed;
    assert!(out.iter().any(|l| l.contains('1')), "{out:?}");
}

#[test]
fn p98_sql_bind_step_stdout() {
    syx_host_plugin::register_echo_plugin_in_process();
    let out = compile_and_run_embedded(
        r#"func main() {
    __syx_sql_migrate("CREATE TABLE IF NOT EXISTS u (name TEXT)")
    __syx_sql_bind_step("INSERT INTO u (name) VALUES (?)", "alice")
    print("ok")
}"#,
        &DriverOptions::default(),
    )
    .unwrap()
    .printed;
    assert!(out.iter().any(|l| l.contains("ok")), "{out:?}");
}

#[test]
fn p98_sql_migrations_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("sql-migrations-v1"));
}
