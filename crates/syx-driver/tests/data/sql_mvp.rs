//! Phase 91: SQL MVP in-memory loopback E2E.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

#[test]
fn p98_sql_mvp_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("sql-mvp-v1"));
}

#[test]
fn p98_sql_mvp_host_surface() {
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(reg.contains("SqlQuery") && reg.contains("__syx_sql_query"));
}

#[test]
fn p98_sql_mvp_create_insert_select_stdout() {
    let out = run(
        r#"func main() {
    __syx_sql_exec("CREATE TABLE t (id INT, name TEXT)")
    __syx_sql_exec("INSERT INTO t VALUES (1, 'a')")
    let rows = __syx_sql_query("SELECT * FROM t")
    print(rows.len())
}"#,
    );
    assert!(out.contains("1"), "{out}");
}

#[test]
fn p98_sql_mvp_fixture_exists() {
    let p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/data/sql_mvp.sy");
    assert!(p.is_file());
}
