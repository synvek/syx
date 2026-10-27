//! Phase 91: rusqlite embedded SQL E2E.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

#[test]
fn p98_sql_rusqlite_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("sql-rusqlite-v1"));
}

#[test]
fn p98_sql_rusqlite_create_insert_select() {
    let out = run(
        r#"func main() {
    __syx_sql_exec("CREATE TABLE users (id INT, name TEXT)")
    __syx_sql_exec("INSERT INTO users VALUES (1, 'alice')")
    let rows = __syx_sql_query("SELECT * FROM users")
    print(rows.len())
}"#,
    );
    assert!(out.contains("1"), "{out}");
}

#[test]
fn p98_sql_rusqlite_prepare_surface() {
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(reg.contains("SqlPrepare") && reg.contains("__syx_sql_prepare"));
    // ADR-148: prepare returns Int (not status-string paste).
    assert!(
        reg.contains("HostFnId::SqlPrepare => \"Int\""),
        "SqlPrepare return type must be Int: check registry host_fn_return_type"
    );
    let host = include_str!("../../../syx-vm/src/stdlib_phase89_hosts.rs");
    assert!(
        !host.contains("prep:{sql}") && !host.contains("format!(\"prep:"),
        "host_sql_prepare must not return prep:{{sql}}"
    );
}

#[test]
fn p98_sql_rusqlite_host_module() {
    assert!(include_str!("../../../syx-vm/src/stdlib_phase98_hosts.rs").contains("rusqlite"));
}
