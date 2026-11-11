//! Orm.saveOn multi-Connection reflective INSERT.
// was: m2_error_orm_conn
//! ADR-167 / M2 W23: Orm.findWhereOn multi-Connection SELECT WHERE.
//! ADR-168 / M2 W24: Orm.updateWhereOn multi-Connection UPDATE WHERE.
//! ADR-169 / M2 W25: Orm.deleteWhereOn multi-Connection DELETE WHERE.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

fn run_err(src: &str) -> String {
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Ok(r) => format!("UNEXPECTED_OK:{}", r.printed.join("\n")),
        // Wave A W122 / ADR-466: Display (not Debug) for product exact nails.
        Err(e) => format!("{e}"),
    }
}

fn facade_src(body: &str) -> String {
    let sql = include_str!("../../../../packages/data/sql.sy");
    let orm = include_str!("../../../../packages/data/orm.sy");
    format!("{sql}\n{orm}\n{body}")
}

#[test]
fn m2_error_orm_save_on_isolation() {
    // Wave A W38 / ADR-382 / A-03: saveOn isolation product exact.
    let out = run(&facade_src(
        r#"
func main() {
  do {
    let a = Sql.open(dsn: ":memory:")
    let b = Sql.open(dsn: ":memory:")
    let ok = saveOn(connId: a, table: "users", record: { "name": "alice" })
    let ra = Sql.queryOn(connId: a, sql: "SELECT name FROM users WHERE name = 'alice'")
    let leakB = Sql.queryOn(connId: b, sql: "SELECT name FROM users WHERE name = 'alice'")
    print("ok")
    print(ok)
    print(ra.len())
    print(leakB.len())
    Sql.close(connId: a)
    Sql.close(connId: b)
  } catch (err) {
    print("UNEXPECTED")
  }
}
"#,
    ));
    assert_eq!(out, "ok\ntrue\n1\n0", "saveOn isolation must exact: {out}");
}

#[test]
fn m2_error_orm_save_on_unknown_conn_display_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_orm_save_on(999999, "users", { "name": "x" })
}
"#,
    );
    // Wave A W122 / ADR-466 / A-03: Orm saveOn unknown-connection Display product exact.
    assert_eq!(
        err, "[R003] sql connection 999999 not found",
        "Orm saveOn unknown-connection must exact: {err}"
    );
}

#[test]
fn m2_error_orm_find_where_on_unknown_conn_display_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_orm_find_where_on(999999, "users", "name", "x")
}
"#,
    );
    // Wave A W122 / ADR-466 / A-03: Orm findWhereOn unknown-connection Display product exact.
    assert_eq!(
        err, "[R003] sql connection 999999 not found",
        "Orm findWhereOn unknown-connection must exact: {err}"
    );
}

#[test]
fn m2_error_orm_save_on_record_not_object_display_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_orm_save_on(999999, "users", 0)
}
"#,
    );
    // Wave A W123 / ADR-467 / A-03: Orm saveOn record-not-object Display product exact.
    assert_eq!(
        err, "[R003] orm saveOn failed: record is not an object",
        "Orm saveOn record-not-object must exact: {err}"
    );
}

#[test]
fn m2_error_orm_save_on_no_columns_display_throws() {
    let err = run_err(
        r#"
func main() {
  let record = __syx_json_decode("{}")
  let _ = __syx_orm_save_on(999999, "users", record)
}
"#,
    );
    // Wave A W123 / ADR-467 / A-03: Orm saveOn no-columns Display product exact.
    assert_eq!(
        err, "[R003] orm saveOn failed: no columns in record",
        "Orm saveOn no-columns must exact: {err}"
    );
}

#[test]
fn m2_error_orm_update_where_on_unknown_conn_display_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_orm_update_where_on(999999, "users", "name", "x", "name", "y")
}
"#,
    );
    // Wave A W124 / ADR-468 / A-03: Orm updateWhereOn unknown-connection Display product exact.
    assert_eq!(
        err, "[R003] sql connection 999999 not found",
        "Orm updateWhereOn unknown-connection must exact: {err}"
    );
}

#[test]
fn m2_error_orm_delete_where_on_unknown_conn_display_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_orm_delete_where_on(999999, "users", "name", "x")
}
"#,
    );
    // Wave A W124 / ADR-468 / A-03: Orm deleteWhereOn unknown-connection Display product exact.
    assert_eq!(
        err, "[R003] sql connection 999999 not found",
        "Orm deleteWhereOn unknown-connection must exact: {err}"
    );
}

#[test]
fn m2_error_orm_save_on_invalid_conn_throws() {
    // Wave A W39 / ADR-383 / A-03: saveOn invalid-conn product exact.
    let out = run(&facade_src(
        r#"
func main() {
  do {
    let ok = saveOn(connId: 999999, table: "users", record: { "name": "x" })
    print("UNEXPECTED_OK")
    print(ok)
  } catch (err) {
    print("caught")
  }
}
"#,
    ));
    assert_eq!(out, "caught", "saveOn invalid conn must exact: {out}");
}

#[test]
fn m2_error_orm_find_where_on_isolation() {
    // Wave A W37 / ADR-381 / A-03: findWhereOn isolation product exact.
    let out = run(&facade_src(
        r#"
func main() {
  do {
    let a = Sql.open(dsn: ":memory:")
    let b = Sql.open(dsn: ":memory:")
    saveOn(connId: a, table: "users", record: { "name": "alice" })
    saveOn(connId: b, table: "users", record: { "name": "bob" })
    let ra = findWhereOn(connId: a, table: "users", col: "name", val: "alice")
    let leakB = findWhereOn(connId: b, table: "users", col: "name", val: "alice")
    print("ok")
    print(ra.len())
    print(leakB.len())
    Sql.close(connId: a)
    Sql.close(connId: b)
  } catch (err) {
    print("UNEXPECTED")
  }
}
"#,
    ));
    assert_eq!(out, "ok\n1\n0", "findWhereOn isolation must exact: {out}");
}

#[test]
fn m2_error_orm_find_where_on_invalid_conn_throws() {
    // Wave A W39 / ADR-383 / A-03: findWhereOn invalid-conn product exact.
    let out = run(&facade_src(
        r#"
func main() {
  do {
    let rows = findWhereOn(connId: 999999, table: "users", col: "name", val: "x")
    print("UNEXPECTED_OK")
    print(rows.len())
  } catch (err) {
    print("caught")
  }
}
"#,
    ));
    assert_eq!(out, "caught", "findWhereOn invalid conn must exact: {out}");
}

#[test]
fn m2_error_orm_update_where_on_isolation() {
    // Wave A W37 / ADR-381 / A-03: updateWhereOn isolation product exact.
    let out = run(&facade_src(
        r#"
func main() {
  do {
    let a = Sql.open(dsn: ":memory:")
    let b = Sql.open(dsn: ":memory:")
    saveOn(connId: a, table: "users", record: { "name": "alice" })
    saveOn(connId: b, table: "users", record: { "name": "bob" })
    let n = updateWhereOn(connId: a, table: "users", col: "name", val: "alice",
                          newCol: "name", newVal: "alice2")
    let a2 = findWhereOn(connId: a, table: "users", col: "name", val: "alice2")
    let leakB = findWhereOn(connId: b, table: "users", col: "name", val: "alice2")
    let aOld = findWhereOn(connId: a, table: "users", col: "name", val: "alice")
    print("ok")
    print(n)
    print(a2.len())
    print(leakB.len())
    print(aOld.len())
    Sql.close(connId: a)
    Sql.close(connId: b)
  } catch (err) {
    print("UNEXPECTED")
  }
}
"#,
    ));
    assert_eq!(
        out, "ok\n1\n1\n0\n0",
        "updateWhereOn isolation must exact: {out}"
    );
}

#[test]
fn m2_error_orm_update_where_on_invalid_conn_throws() {
    // Wave A W40 / ADR-384 / A-03: updateWhereOn invalid-conn product exact.
    let out = run(&facade_src(
        r#"
func main() {
  do {
    let n = updateWhereOn(connId: 999999, table: "users", col: "name", val: "x",
                          newCol: "name", newVal: "y")
    print("UNEXPECTED_OK")
    print(n)
  } catch (err) {
    print("caught")
  }
}
"#,
    ));
    assert_eq!(
        out, "caught",
        "updateWhereOn invalid conn must exact: {out}"
    );
}

#[test]
fn m2_error_orm_delete_where_on_isolation() {
    // Wave A W38 / ADR-382 / A-03: deleteWhereOn isolation product exact.
    let out = run(&facade_src(
        r#"
func main() {
  do {
    let a = Sql.open(dsn: ":memory:")
    let b = Sql.open(dsn: ":memory:")
    saveOn(connId: a, table: "users", record: { "name": "alice" })
    saveOn(connId: b, table: "users", record: { "name": "bob" })
    let n = deleteWhereOn(connId: a, table: "users", col: "name", val: "alice")
    let aGone = findWhereOn(connId: a, table: "users", col: "name", val: "alice")
    let bBob = findWhereOn(connId: b, table: "users", col: "name", val: "bob")
    print("ok")
    print(n)
    print(aGone.len())
    print(bBob.len())
    Sql.close(connId: a)
    Sql.close(connId: b)
  } catch (err) {
    print("UNEXPECTED")
  }
}
"#,
    ));
    assert_eq!(
        out, "ok\n1\n0\n1",
        "deleteWhereOn isolation must exact: {out}"
    );
}

#[test]
fn m2_error_orm_delete_where_on_invalid_conn_throws() {
    // Wave A W40 / ADR-384 / A-03: deleteWhereOn invalid-conn product exact.
    let out = run(&facade_src(
        r#"
func main() {
  do {
    let n = deleteWhereOn(connId: 999999, table: "users", col: "name", val: "x")
    print("UNEXPECTED_OK")
    print(n)
  } catch (err) {
    print("caught")
  }
}
"#,
    ));
    assert_eq!(
        out, "caught",
        "deleteWhereOn invalid conn must exact: {out}"
    );
}
