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
fn m2_error_sql_prepare_invalid_throws() {
    // Wave A W26 / ADR-370 / A-03: invalid-prepare product exact (not message/code soft).
    let out = run(&facade_src(
        r#"
func main() {
  do {
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
  do {
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
  do {
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
  do {
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
  do {
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
  do {
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
  do {
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
  do {
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
  do {
    Sql.beginTransaction()
  } catch (e1) {
    print("UNEXPECTED_BEGIN1")
    return
  }
  do {
    Sql.beginTransaction()
    print("UNEXPECTED_OK")
  } catch (e2) {
    print("caught")
  }
  do {
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
  do {
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
  do {
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
