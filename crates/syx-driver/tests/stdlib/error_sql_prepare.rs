//! Sql facade throws SqlError — prepare/bind/exec/query/txn.
// was: m2_error_sql_prepare

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

fn facade_src(body: &str) -> String {
    let sql = include_str!("../../../../packages/data/sql.sy");
    format!("{sql}\n{body}")
}

#[test]
fn m2_error_sql_facade_source_has_sqlerror_and_throws() {
    let src = include_str!("../../../../packages/data/sql.sy");
    assert!(src.contains("SqlError"), "SqlError type required");
    assert!(
        src.contains("prepare(sql: String) -> Int throws") || src.contains("-> Int throws"),
        "prepare must throws: {src}"
    );
    assert!(
        src.contains("bindStep(sql: String, value: String) -> Int throws")
            || src.contains("func bindStep(sql: String, value: String) -> Int throws"),
        "bindStep must throws: {src}"
    );
    assert!(
        src.contains("exec(sql: String) -> Int throws")
            || src.contains("func exec(sql: String) -> Int throws"),
        "exec must throws: {src}"
    );
    assert!(
        src.contains("query(sql: String) -> Array throws")
            || src.contains("func query(sql: String) -> Array throws"),
        "query must throws: {src}"
    );
    assert!(
        src.contains("func beginTransaction() throws")
            || src.contains("beginTransaction() throws"),
        "beginTransaction must throws: {src}"
    );
    assert!(
        src.contains("func commitTransaction() throws")
            || src.contains("commitTransaction() throws"),
        "commitTransaction must throws: {src}"
    );
    assert!(
        src.contains("func rollbackTransaction() throws")
            || src.contains("rollbackTransaction() throws"),
        "rollbackTransaction must throws: {src}"
    );
    let host = include_str!("../../../syx-vm/src/stdlib_phase89_hosts.rs");
    assert!(
        !host.contains("prep:{sql}") && !host.contains("format!(\"prep:"),
        "host must not paste prep:{{sql}}: found in phase89 hosts"
    );
    let exec_fn = host
        .split("pub fn host_sql_exec_rusqlite")
        .nth(1)
        .expect("host_sql_exec_rusqlite");
    let exec_body = exec_fn.split("pub fn ").next().unwrap_or(exec_fn);
    assert!(
        !exec_body.contains(".unwrap_or(0)"),
        "host_sql_exec must not .unwrap_or(0) sentinel: {exec_body}"
    );
    assert!(
        exec_body.contains("with_code(\"R003\")"),
        "host_sql_exec must Err with R003: {exec_body}"
    );
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    let sql_idx = reg
        .find("type_name: \"Sql\"")
        .expect("Sql TypeMethod block");
    let slice = &reg[sql_idx..];
    for method in [
        "exec",
        "query",
        "prepare",
        "bindStep",
        "beginTransaction",
        "commitTransaction",
        "rollbackTransaction",
    ] {
        assert!(
            slice.contains(&format!("method: \"{method}\""))
                && slice
                    .split(&format!("method: \"{method}\""))
                    .nth(1)
                    .is_some_and(|s| s.contains("host: None")),
            "Sql.{method} must host: None"
        );
    }
}

#[test]
fn m2_error_sql_prepare_invalid_throws() {
    // Wave A W26 / ADR-370 / A-03: invalid-prepare product exact (not message/code soft).
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let id = Sql.prepare(sql: "NOT VALID SQL !!!")
    print("UNEXPECTED_OK")
    print(id)
  } catch (err) {
    print("caught")
  }
}
"#,
    ));
    assert_eq!(out, "caught", "invalid prepare must exact-catch: {out}");
}

#[test]
fn m2_error_sql_prepare_valid_returns_int() {
    // Wave A W35 / ADR-379 / A-03: prepare valid product exact (no stmt-id print).
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let _ = Sql.prepare(sql: "SELECT 1")
    print("ok")
  } catch (err) {
    print("UNEXPECTED")
  }
}
"#,
    ));
    assert_eq!(out, "ok", "valid prepare must exact-ok: {out}");
}

#[test]
fn m2_error_sql_bind_step_invalid_throws() {
    // Wave A W27 / ADR-371 / A-03: invalid-bindStep product exact (not message/code soft).
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let n = Sql.bindStep(sql: "NOT VALID !!!", value: "x")
    print("UNEXPECTED_OK")
    print(n)
  } catch (err) {
    print("caught")
  }
}
"#,
    ));
    assert_eq!(out, "caught", "invalid bindStep must exact-catch: {out}");
}

#[test]
fn m2_error_sql_bind_step_valid_returns_int() {
    // Wave A W34 / ADR-378 / A-03: bindStep valid product exact.
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let _ = __syx_sql_migrate("CREATE TABLE IF NOT EXISTS u (name TEXT)")
    let n = Sql.bindStep(sql: "INSERT INTO u (name) VALUES (?)", value: "alice")
    print("ok")
    print(n)
  } catch (err) {
    print("UNEXPECTED")
  }
}
"#,
    ));
    assert_eq!(out, "ok\n1", "valid bindStep must exact: {out}");
}

#[test]
fn m2_error_sql_exec_invalid_throws() {
    // Wave A W27 / ADR-371 / A-03: invalid-exec product exact (not message/code soft).
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let n = Sql.exec(sql: "NOT VALID !!!")
    print("UNEXPECTED_OK")
    print(n)
  } catch (err) {
    print("caught")
  }
}
"#,
    ));
    assert_eq!(out, "caught", "invalid exec must exact-catch: {out}");
}

#[test]
fn m2_error_sql_query_invalid_throws() {
    // Wave A W28 / ADR-372 / A-03: invalid-query product exact (not message/code soft).
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let rows = Sql.query(sql: "NOT VALID !!!")
    print("UNEXPECTED_OK")
    print(rows)
  } catch (err) {
    print("caught")
  }
}
"#,
    ));
    assert_eq!(out, "caught", "invalid query must exact-catch: {out}");
}

#[test]
fn m2_error_sql_exec_valid_returns_int() {
    // Wave A W35 / ADR-379 / A-03: exec valid product exact.
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let n = Sql.exec(sql: "CREATE TABLE IF NOT EXISTS m2w11_t (id INTEGER)")
    print("ok")
    print(n)
  } catch (err) {
    print("UNEXPECTED")
  }
}
"#,
    ));
    assert_eq!(out, "ok\n1", "valid exec must exact: {out}");
}

#[test]
fn m2_error_sql_begin_transaction_ok() {
    // Wave A W33 / ADR-377 / A-03: beginTransaction ok product exact.
    let out = run(&facade_src(
        r#"
func main() {
  try {
    Sql.beginTransaction()
    print("ok")
    Sql.rollbackTransaction()
  } catch (err) {
    print("UNEXPECTED")
  }
}
"#,
    ));
    assert_eq!(out, "ok", "beginTransaction must exact-ok: {out}");
}

#[test]
fn m2_error_sql_nested_begin_transaction_throws() {
    // Wave A W28 / ADR-372 / A-03: nested-begin product exact (not message/code soft).
    // Nested BEGIN IMMEDIATE fails under rusqlite — reliable SqlError nail (vs COMMIT no-op).
    let out = run(&facade_src(
        r#"
func main() {
  try {
    Sql.beginTransaction()
  } catch (e1) {
    print("UNEXPECTED_BEGIN1")
    return
  }
  try {
    Sql.beginTransaction()
    print("UNEXPECTED_OK")
  } catch (e2) {
    print("caught")
  }
  try {
    Sql.rollbackTransaction()
  } catch (e3) {
  }
}
"#,
    ));
    assert_eq!(out, "caught", "nested begin must exact-catch: {out}");
}

#[test]
fn m2_error_sql_rollback_ok_after_begin() {
    // Wave A W34 / ADR-378 / A-03: rollback after begin product exact.
    let out = run(&facade_src(
        r#"
func main() {
  try {
    Sql.beginTransaction()
    Sql.rollbackTransaction()
    print("ok")
  } catch (err) {
    print("UNEXPECTED")
  }
}
"#,
    ));
    assert_eq!(out, "ok", "begin+rollback must exact-ok: {out}");
}

#[test]
fn m2_error_sql_commit_without_begin_throws() {
    // Wave A W36 / ADR-380 / A-03: COMMIT-without-BEGIN product exact (host throws; not SQLite no-op).
    // Nested-begin throw is already exact via m2_error_sql_nested_begin_transaction_throws.
    let out = run(&facade_src(
        r#"
func main() {
  try {
    Sql.commitTransaction()
    print("COMMIT_NOOP")
  } catch (err) {
    print("caught")
  }
}
"#,
    ));
    assert_eq!(
        out, "caught",
        "commit without begin must exact-catch: {out}"
    );
}
