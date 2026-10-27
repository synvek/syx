//! EC-M2-1 SQLite service narrow slice nail.
// was: ec_m2_1_sql_service

use std::path::PathBuf;

use syx_driver::{compile_and_run_embedded, run_file_printed, DriverOptions};

fn example_main() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/sql_tool/main.sy")
}

fn facade_src(body: &str) -> String {
    let sql = include_str!("../../../../packages/data/sql.sy");
    format!("{sql}\n{body}")
}

fn run_embed(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

#[test]
fn ec_m2_1_sql_service_source_uses_official_sql() {
    let example = include_str!("../../../../examples/sql_tool/main.sy");
    assert!(
        example.contains("import Sql")
            && example.contains("Sql.open")
            && example.contains("execOn")
            && example.contains("queryOn"),
        "example must use import Sql + open/execOn/queryOn: {example}"
    );
    assert!(
        example.contains("try") && example.contains("catch"),
        "example must try/catch: {example}"
    );
    assert!(
        !example.contains("__syx_sql_"),
        "example must not call bare sql hosts: {example}"
    );
    let sql = include_str!("../../../../packages/data/sql.sy");
    assert!(
        sql.contains("SqlError") && sql.contains("throws"),
        "Sql facade throws SqlError"
    );
    let emb = include_str!("../../src/embedded_std.rs");
    assert!(
        emb.contains("(\"Sql\", SQL_SOURCE)") || emb.contains("\"Sql\""),
        "Sql must be importable in embedded_std"
    );
}

#[test]
fn ec_m2_1_sql_service_memory_crud_ok() {
    // Wave A W63 / ADR-407 / A-03: Sql service memory CRUD product exact (example unchanged).
    let (code, printed) =
        run_file_printed(&example_main(), &DriverOptions::default()).expect("run");
    let out = printed.join("\n");
    assert_eq!(code, 0, "sql_tool must succeed: {out}");
    assert_eq!(out, "ok\n1\nclosed", "sql_tool memory CRUD must exact: {out}");
}

#[test]
fn ec_m2_1_sql_service_invalid_close_caught() {
    // Wave A W47 / ADR-391 / A-03: Sql.close invalid-handle product exact.
    let out = run_embed(&facade_src(
        r#"
func main() {
  try {
    Sql.close(connId: 999999)
    print("UNEXPECTED")
  } catch (err) {
    print("caught")
  }
}
"#,
    ));
    assert_eq!(out, "caught", "invalid close must exact: {out}");
}
