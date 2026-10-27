//! Sql.open/close/execOn/queryOn multi-Connection first nail.
// was: m2_error_sql_conn
//! ADR-162 / M2 W18: Sql.prepareOn schema isolation on multi-Connection.
//! ADR-163 / M2 W19: Sql.bindStepOn parameterized write isolation on multi-Connection.
//! ADR-164 / M2 W20: Sql.bindPreparedOn stmt↔conn binding.
//! ADR-165 / M2 W21: Sql begin/commit/rollback TransactionOn.

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
        // Wave A W111 / ADR-455: Display (not Debug) for product exact nails.
        Err(e) => format!("{e}"),
    }
}

fn facade_src(body: &str) -> String {
    let sql = include_str!("../../../../packages/data/sql.sy");
    format!("{sql}\n{body}")
}

#[test]
fn m2_error_sql_conn_facade_source_and_hosts() {
    let src = include_str!("../../../../packages/data/sql.sy");
    assert!(src.contains("SqlError"), "SqlError required");
    for needle in [
        "open(dsn: String) -> Int throws",
        "close(connId: Int) throws",
        "execOn(connId: Int, sql: String) -> Int throws",
        "queryOn(connId: Int, sql: String) -> Array throws",
        "prepareOn(connId: Int, sql: String) -> Int throws",
        "bindStepOn(connId: Int, sql: String, value: String) -> Int throws",
        "bindPreparedOn(connId: Int, stmtId: Int, value: String) -> Int throws",
        "beginTransactionOn(connId: Int) throws",
        "commitTransactionOn(connId: Int) throws",
        "rollbackTransactionOn(connId: Int) throws",
    ] {
        assert!(
            src.contains(needle) || src.contains("throws"),
            "Sql multi-conn surface must throws ({needle}): {src}"
        );
        assert!(
            src.contains(needle.split('(').next().unwrap_or(needle)),
            "missing method in facade: {needle}"
        );
    }

    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    let sql_idx = reg
        .find("type_name: \"Sql\"")
        .expect("Sql TypeMethod block");
    let slice = &reg[sql_idx..];
    for method in [
        "open",
        "close",
        "execOn",
        "queryOn",
        "prepareOn",
        "bindStepOn",
        "bindPreparedOn",
        "beginTransactionOn",
        "commitTransactionOn",
        "rollbackTransactionOn",
    ] {
        assert!(
            slice.contains(&format!("method: \"{method}\""))
                && slice
                    .split(&format!("method: \"{method}\""))
                    .nth(1)
                    .is_some_and(|s| s.contains("host: None")),
            "Sql.{method} must host: None: {slice}"
        );
    }

    let host = include_str!("../../../syx-vm/src/stdlib_phase89_hosts.rs");
    assert!(
        host.contains("RUSQLITE_CONNS") && host.contains("HashMap"),
        "host must use HashMap conn store: missing RUSQLITE_CONNS"
    );
    assert!(
        !host.contains("RUSQLITE_CONN:") && !host.contains("Mutex<Option<rusqlite::Connection>>"),
        "must not keep single-slot Option<Connection> as sole store"
    );
    assert!(
        host.contains("host_sql_open")
            && host.contains("host_sql_close")
            && host.contains("host_sql_exec_on")
            && host.contains("host_sql_query_on")
            && host.contains("host_sql_prepare_on")
            && host.contains("host_sql_bind_step_on")
            && host.contains("host_sql_bind_prepared_on")
            && host.contains("host_sql_begin_transaction_on")
            && host.contains("host_sql_commit_transaction_on")
            && host.contains("host_sql_rollback_transaction_on"),
        "open/close/exec_on/query_on/prepare_on/bind_*/txn_on hosts required"
    );
}

#[test]
fn m2_error_sql_close_invalid_throws() {
    // Wave A W26 / ADR-370 / A-03: invalid-close product exact (not message/code soft).
    let out = run(&facade_src(
        r#"
func main() {
  try {
    Sql.close(connId: 999999)
    print("UNEXPECTED_OK")
  } catch (err) {
    print("caught")
  }
}
"#,
    ));
    assert_eq!(out, "caught", "invalid close must exact-catch: {out}");
}

#[test]
fn m2_error_sql_close_unknown_connection_throws() {
    let err = run_err(
        r#"
func main() {
  __syx_sql_close(999999)
}
"#,
    );
    // Wave A W111 / ADR-455 / A-03: Sql close unknown-connection Display product exact.
    assert_eq!(
        err, "[R003] sql close failed: unknown connection 999999",
        "Sql close unknown-connection must exact: {err}"
    );
}

#[test]
fn m2_error_sql_exec_on_unknown_connection_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_sql_exec_on(999999, "SELECT 1")
}
"#,
    );
    // Wave A W111 / ADR-455 / A-03: Sql execOn unknown-connection Display product exact.
    assert_eq!(
        err, "[R003] sql connection 999999 not found",
        "Sql execOn unknown-connection must exact: {err}"
    );
}

#[test]
fn m2_error_sql_query_on_unknown_connection_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_sql_query_on(999999, "SELECT 1")
}
"#,
    );
    // Wave A W112 / ADR-456 / A-03: Sql queryOn unknown-connection Display product exact.
    assert_eq!(
        err, "[R003] sql connection 999999 not found",
        "Sql queryOn unknown-connection must exact: {err}"
    );
}

#[test]
fn m2_error_sql_prepare_on_unknown_connection_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_sql_prepare_on(999999, "SELECT 1")
}
"#,
    );
    // Wave A W112 / ADR-456 / A-03: Sql prepareOn unknown-connection Display product exact.
    assert_eq!(
        err, "[R003] sql connection 999999 not found",
        "Sql prepareOn unknown-connection must exact: {err}"
    );
}

#[test]
fn m2_error_sql_begin_transaction_on_unknown_connection_throws() {
    let err = run_err(
        r#"
func main() {
  __syx_sql_begin_transaction_on(999999)
}
"#,
    );
    // Wave A W113 / ADR-457 / A-03: Sql beginTransactionOn unknown-connection Display product exact.
    assert_eq!(
        err, "[R003] sql connection 999999 not found",
        "Sql beginTransactionOn unknown-connection must exact: {err}"
    );
}

#[test]
fn m2_error_sql_commit_transaction_on_unknown_connection_throws() {
    let err = run_err(
        r#"
func main() {
  __syx_sql_commit_transaction_on(999999)
}
"#,
    );
    // Wave A W113 / ADR-457 / A-03: Sql commitTransactionOn unknown-connection Display product exact.
    assert_eq!(
        err, "[R003] sql connection 999999 not found",
        "Sql commitTransactionOn unknown-connection must exact: {err}"
    );
}

#[test]
fn m2_error_sql_rollback_transaction_on_unknown_connection_throws() {
    let err = run_err(
        r#"
func main() {
  __syx_sql_rollback_transaction_on(999999)
}
"#,
    );
    // Wave A W114 / ADR-458 / A-03: Sql rollbackTransactionOn unknown-connection Display product exact.
    assert_eq!(
        err, "[R003] sql connection 999999 not found",
        "Sql rollbackTransactionOn unknown-connection must exact: {err}"
    );
}

#[test]
fn m2_error_sql_bind_step_on_unknown_connection_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_sql_bind_step_on(999999, "SELECT 1", "x")
}
"#,
    );
    // Wave A W114 / ADR-458 / A-03: Sql bindStepOn unknown-connection Display product exact.
    assert_eq!(
        err, "[R003] sql connection 999999 not found",
        "Sql bindStepOn unknown-connection must exact: {err}"
    );
}

#[test]
fn m2_error_sql_bind_prepared_on_unknown_stmt_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_sql_bind_prepared_on(999999, 999999, "x")
}
"#,
    );
    // Wave A W115 / ADR-459 / A-03: Sql bindPreparedOn unknown-stmt Display product exact.
    assert_eq!(
        err, "[R003] sql bindPreparedOn failed: unknown stmt 999999",
        "Sql bindPreparedOn unknown-stmt must exact: {err}"
    );
}

#[test]
fn m2_error_sql_pool_acquire_unknown_pool_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_sql_pool_acquire(999999)
}
"#,
    );
    // Wave A W115 / ADR-459 / A-03: Sql poolAcquire unknown-pool Display product exact.
    assert_eq!(
        err, "[R003] sql poolAcquire failed: unknown pool 999999",
        "Sql poolAcquire unknown-pool must exact: {err}"
    );
}

#[test]
fn m2_error_sql_pool_acquire_unknown_pool_display_throws() {
    // Wave A W179 / ADR-523 / A-03 — Sql.poolAcquire unknown-pool facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  let _ = Sql.poolAcquire(poolId: 999999)
}
"#,
    ));
    assert_eq!(
        err, "[R007] sql poolAcquire failed",
        "poolAcquire unknown-pool Display must exact: {err}"
    );
}

#[test]
fn m2_error_sql_pool_acquire_after_close_display_throws() {
    // Wave A W179 / ADR-523 / A-03 — Sql.poolAcquire after-close facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  let p = Sql.poolCreate(dsn: ":memory:", maxSize: 1)
  Sql.poolClose(poolId: p)
  let _ = Sql.poolAcquire(poolId: p)
}
"#,
    ));
    assert_eq!(
        err, "[R007] sql poolAcquire failed",
        "poolAcquire-after-close Display must exact: {err}"
    );
}

#[test]
fn m2_error_sql_pool_release_unknown_pool_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_sql_pool_release(999999, 999999)
}
"#,
    );
    // Wave A W116 / ADR-460 / A-03: Sql poolRelease unknown-pool Display product exact.
    assert_eq!(
        err, "[R003] sql poolRelease failed: unknown pool 999999",
        "Sql poolRelease unknown-pool must exact: {err}"
    );
}

#[test]
fn m2_error_sql_pool_release_unknown_pool_display_throws() {
    // Wave A W178 / ADR-522 / A-03 — Sql.poolRelease unknown-pool facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  Sql.poolRelease(poolId: 999999, connId: 999999)
}
"#,
    ));
    assert_eq!(
        err, "[R007] sql poolRelease failed",
        "poolRelease unknown-pool Display must exact: {err}"
    );
}

#[test]
fn m2_error_sql_pool_release_double_release_display_throws() {
    // Wave A W178 / ADR-522 / A-03 — Sql.poolRelease double-release facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  let p = Sql.poolCreate(dsn: ":memory:", maxSize: 1)
  let c = Sql.poolAcquire(poolId: p)
  Sql.poolRelease(poolId: p, connId: c)
  Sql.poolRelease(poolId: p, connId: c)
}
"#,
    ));
    assert_eq!(
        err, "[R007] sql poolRelease failed",
        "poolRelease double-release Display must exact: {err}"
    );
}

#[test]
fn m2_error_sql_pool_close_unknown_pool_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_sql_pool_close(999999)
}
"#,
    );
    // Wave A W116 / ADR-460 / A-03: Sql poolClose unknown-pool Display product exact.
    assert_eq!(
        err, "[R003] sql poolClose failed: unknown pool 999999",
        "Sql poolClose unknown-pool must exact: {err}"
    );
}

#[test]
fn m2_error_sql_pool_close_unknown_pool_display_throws() {
    // Wave A W177 / ADR-521 / A-03 — Sql.poolClose unknown-pool facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  Sql.poolClose(poolId: 999999)
}
"#,
    ));
    assert_eq!(
        err, "[R007] sql poolClose failed",
        "poolClose unknown-pool Display must exact: {err}"
    );
}

#[test]
fn m2_error_sql_pool_close_after_close_display_throws() {
    // Wave A W177 / ADR-521 / A-03 — Sql.poolClose close-after-close facade Display.
    let err = run_err(&facade_src(
        r#"
func main() {
  let p = Sql.poolCreate(dsn: ":memory:", maxSize: 1)
  Sql.poolClose(poolId: p)
  Sql.poolClose(poolId: p)
}
"#,
    ));
    assert_eq!(
        err, "[R007] sql poolClose failed",
        "poolClose-after-close Display must exact: {err}"
    );
}

#[test]
fn m2_error_sql_two_memory_isolation() {
    // Wave A W36 / ADR-380 / A-03: two-memory isolation product exact (no conn-id print).
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let a = Sql.open(dsn: ":memory:")
    let b = Sql.open(dsn: ":memory:")
    Sql.execOn(connId: a, sql: "CREATE TABLE t (v TEXT)")
    Sql.execOn(connId: b, sql: "CREATE TABLE t (v TEXT)")
    Sql.execOn(connId: a, sql: "INSERT INTO t (v) VALUES ('alice')")
    Sql.execOn(connId: b, sql: "INSERT INTO t (v) VALUES ('bob')")
    let ra = Sql.queryOn(connId: a, sql: "SELECT v FROM t WHERE v = 'alice'")
    let rb = Sql.queryOn(connId: b, sql: "SELECT v FROM t WHERE v = 'bob'")
    let leakA = Sql.queryOn(connId: a, sql: "SELECT v FROM t WHERE v = 'bob'")
    let leakB = Sql.queryOn(connId: b, sql: "SELECT v FROM t WHERE v = 'alice'")
    print("ok")
    print(ra.len())
    print(rb.len())
    print(leakA.len())
    print(leakB.len())
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
        "two-memory isolation must exact: {out}"
    );
}

#[test]
fn m2_error_sql_prepare_on_schema_isolation() {
    // Wave A W31 / ADR-375 / A-03: prepareOn schema-isolation product exact.
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let a = Sql.open(dsn: ":memory:")
    let b = Sql.open(dsn: ":memory:")
    Sql.execOn(connId: a, sql: "CREATE TABLE t (v TEXT)")
    let id = Sql.prepareOn(connId: a, sql: "INSERT INTO t (v) VALUES (?)")
    print("ok")
    try {
      let bad = Sql.prepareOn(connId: b, sql: "INSERT INTO t (v) VALUES (?)")
      print("UNEXPECTED_OK")
      print(bad)
    } catch (e2) {
      print("caught")
    }
    Sql.close(connId: a)
    Sql.close(connId: b)
  } catch (e1) {
    print("OUTER")
  }
}
"#,
    ));
    assert_eq!(
        out, "ok\ncaught",
        "prepareOn schema isolation must exact: {out}"
    );
}

#[test]
fn m2_error_sql_prepare_on_invalid_throws() {
    // Wave A W29 / ADR-373 / A-03: prepareOn-invalid product exact (not message/code soft).
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let a = Sql.open(dsn: ":memory:")
    try {
      let id = Sql.prepareOn(connId: a, sql: "INSERT INTO nosuch_m2_w18 (v) VALUES (?)")
      print("UNEXPECTED_OK")
      print(id)
    } catch (err) {
      print("caught")
    }
    Sql.close(connId: a)
  } catch (err) {
    print("OUTER")
  }
}
"#,
    ));
    assert_eq!(out, "caught", "prepareOn missing table must exact-catch: {out}");
}

#[test]
fn m2_error_sql_bind_step_on_schema_isolation() {
    // Wave A W30 / ADR-374 / A-03: bindStepOn schema-isolation product exact.
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let a = Sql.open(dsn: ":memory:")
    let b = Sql.open(dsn: ":memory:")
    Sql.execOn(connId: a, sql: "CREATE TABLE t (v TEXT)")
    let n = Sql.bindStepOn(connId: a, sql: "INSERT INTO t (v) VALUES (?)", value: "x")
    print("ok")
    print(n)
    try {
      let bad = Sql.bindStepOn(connId: b, sql: "INSERT INTO t (v) VALUES (?)", value: "y")
      print("UNEXPECTED_OK")
      print(bad)
    } catch (e2) {
      print("caught")
    }
    Sql.close(connId: a)
    Sql.close(connId: b)
  } catch (e1) {
    print("OUTER")
  }
}
"#,
    ));
    assert_eq!(
        out, "ok\n1\ncaught",
        "bindStepOn schema isolation must exact: {out}"
    );
}

#[test]
fn m2_error_sql_bind_step_on_write_isolation() {
    // Wave A W31 / ADR-375 / A-03: bindStepOn write-isolation product exact.
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let a = Sql.open(dsn: ":memory:")
    let b = Sql.open(dsn: ":memory:")
    Sql.execOn(connId: a, sql: "CREATE TABLE t (v TEXT)")
    Sql.execOn(connId: b, sql: "CREATE TABLE t (v TEXT)")
    let na = Sql.bindStepOn(connId: a, sql: "INSERT INTO t (v) VALUES (?)", value: "alice")
    let nb = Sql.bindStepOn(connId: b, sql: "INSERT INTO t (v) VALUES (?)", value: "bob")
    let ra = Sql.queryOn(connId: a, sql: "SELECT v FROM t WHERE v = 'alice'")
    let rb = Sql.queryOn(connId: b, sql: "SELECT v FROM t WHERE v = 'bob'")
    let leakA = Sql.queryOn(connId: a, sql: "SELECT v FROM t WHERE v = 'bob'")
    let leakB = Sql.queryOn(connId: b, sql: "SELECT v FROM t WHERE v = 'alice'")
    print("ok")
    print(na)
    print(nb)
    print(ra.len())
    print(rb.len())
    print(leakA.len())
    print(leakB.len())
    Sql.close(connId: a)
    Sql.close(connId: b)
  } catch (err) {
    print("UNEXPECTED")
  }
}
"#,
    ));
    assert_eq!(
        out, "ok\n1\n1\n1\n1\n0\n0",
        "bindStepOn write isolation must exact: {out}"
    );
}

#[test]
fn m2_error_sql_bind_prepared_on_conn_mismatch() {
    // Wave A W30 / ADR-374 / A-03: bindPreparedOn conn-mismatch product exact.
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let a = Sql.open(dsn: ":memory:")
    let b = Sql.open(dsn: ":memory:")
    Sql.execOn(connId: a, sql: "CREATE TABLE t (v TEXT)")
    let stmtId = Sql.prepareOn(connId: a, sql: "INSERT INTO t (v) VALUES (?)")
    print("ok")
    try {
      let bad = Sql.bindPreparedOn(connId: b, stmtId: stmtId, value: "x")
      print("UNEXPECTED_OK")
      print(bad)
    } catch (e2) {
      print("caught")
    }
    Sql.close(connId: a)
    Sql.close(connId: b)
  } catch (e1) {
    print("OUTER")
  }
}
"#,
    ));
    assert_eq!(
        out, "ok\ncaught",
        "bindPreparedOn conn mismatch must exact: {out}"
    );
}

#[test]
fn m2_error_sql_bind_prepared_on_conn_mismatch_display_throws() {
    // Wave A W135 / ADR-479 / A-03: bindPreparedOn conn-mismatch Display product exact.
    let err = run_err(&facade_src(
        r#"
func main() {
  let a = Sql.open(dsn: ":memory:")
  let b = Sql.open(dsn: ":memory:")
  Sql.execOn(connId: a, sql: "CREATE TABLE t (v TEXT)")
  let stmtId = Sql.prepareOn(connId: a, sql: "INSERT INTO t (v) VALUES (?)")
  let _ = Sql.bindPreparedOn(connId: b, stmtId: stmtId, value: "x")
}
"#,
    ));
    assert_eq!(
        err, "[R007] sql bindPreparedOn failed",
        "bindPreparedOn conn mismatch Display must exact: {err}"
    );
}

#[test]
fn m2_error_sql_bind_prepared_on_positive() {
    // Wave A W33 / ADR-377 / A-03: bindPreparedOn positive product exact.
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let a = Sql.open(dsn: ":memory:")
    let b = Sql.open(dsn: ":memory:")
    Sql.execOn(connId: a, sql: "CREATE TABLE t (v TEXT)")
    Sql.execOn(connId: b, sql: "CREATE TABLE t (v TEXT)")
    let stmtId = Sql.prepareOn(connId: a, sql: "INSERT INTO t (v) VALUES (?)")
    let n = Sql.bindPreparedOn(connId: a, stmtId: stmtId, value: "alice")
    let ra = Sql.queryOn(connId: a, sql: "SELECT v FROM t WHERE v = 'alice'")
    let leakB = Sql.queryOn(connId: b, sql: "SELECT v FROM t WHERE v = 'alice'")
    print("ok")
    print(n)
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
    assert_eq!(
        out, "ok\n1\n1\n0",
        "bindPreparedOn positive must exact: {out}"
    );
}

#[test]
fn m2_error_sql_txn_on_commit_isolation() {
    // Wave A W32 / ADR-376 / A-03: txnOn commit-isolation product exact.
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let a = Sql.open(dsn: ":memory:")
    let b = Sql.open(dsn: ":memory:")
    Sql.execOn(connId: a, sql: "CREATE TABLE t (v TEXT)")
    Sql.execOn(connId: b, sql: "CREATE TABLE t (v TEXT)")
    Sql.beginTransactionOn(connId: a)
    Sql.execOn(connId: a, sql: "INSERT INTO t (v) VALUES ('alice')")
    Sql.commitTransactionOn(connId: a)
    let ra = Sql.queryOn(connId: a, sql: "SELECT v FROM t WHERE v = 'alice'")
    let leakB = Sql.queryOn(connId: b, sql: "SELECT v FROM t WHERE v = 'alice'")
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
    assert_eq!(
        out, "ok\n1\n0",
        "txnOn commit isolation must exact: {out}"
    );
}

#[test]
fn m2_error_sql_txn_on_rollback() {
    // Wave A W32 / ADR-376 / A-03: txnOn rollback product exact.
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let a = Sql.open(dsn: ":memory:")
    Sql.execOn(connId: a, sql: "CREATE TABLE t (v TEXT)")
    Sql.beginTransactionOn(connId: a)
    Sql.execOn(connId: a, sql: "INSERT INTO t (v) VALUES ('alice')")
    Sql.rollbackTransactionOn(connId: a)
    let ra = Sql.queryOn(connId: a, sql: "SELECT v FROM t WHERE v = 'alice'")
    print("ok")
    print(ra.len())
    Sql.close(connId: a)
  } catch (err) {
    print("UNEXPECTED")
  }
}
"#,
    ));
    assert_eq!(out, "ok\n0", "txnOn rollback must exact: {out}");
}

#[test]
fn m2_error_sql_txn_on_nested_begin_throws() {
    // Wave A W29 / ADR-373 / A-03: nested beginTransactionOn product exact (not message/code soft).
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let a = Sql.open(dsn: ":memory:")
    Sql.beginTransactionOn(connId: a)
    try {
      Sql.beginTransactionOn(connId: a)
      print("UNEXPECTED_OK")
    } catch (e2) {
      print("caught")
    }
    try {
      Sql.rollbackTransactionOn(connId: a)
    } catch (e3) {
    }
    Sql.close(connId: a)
  } catch (e1) {
    print("OUTER")
  }
}
"#,
    ));
    assert_eq!(out, "caught", "nested beginTransactionOn must exact-catch: {out}");
}
