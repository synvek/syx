//! EC-M2-3 Sql explicit connection live-count leak first nail.
// was: ec_m2_3_sql_leak

use syx_driver::{compile_and_run_embedded, DriverOptions};
use syx_vm::{
    reset_rusqlite_store, sql_conn_force_close, sql_conn_live_count,
};

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
fn ec_m2_3_sql_leak_facade_source_has_open_close_throws() {
    let src = include_str!("../../../../packages/data/sql.sy");
    assert!(
        src.contains("open(dsn: String) -> Int throws")
            || (src.contains("func open") && src.contains("throws")),
        "open must throws: {src}"
    );
    assert!(
        src.contains("close(connId: Int) throws")
            || src.contains("func close(connId: Int) throws"),
        "close must throws"
    );
    let hosts = include_str!("../../../syx-vm/src/stdlib_phase89_hosts.rs");
    assert!(
        hosts.contains("pub fn sql_conn_live_count"),
        "sql_conn_live_count must be exported for nails"
    );
}

#[test]
fn ec_m2_3_sql_leak_open_close_returns_to_baseline() {
    // Wave A W52 / ADR-396 / A-03: Sql open+close product exact (drop print(c)).
    reset_rusqlite_store();
    let baseline = sql_conn_live_count();
    assert_eq!(baseline, 0, "reset must clear explicit conns");

    let out = run(&facade_src(
        r#"
func main() {
  try {
    let c = Sql.open(dsn: ":memory:")
    print("OPENED")
    let _ = Sql.execOn(connId: c, sql: "CREATE TABLE t (v TEXT)")
    Sql.close(connId: c)
    print("CLOSED")
  } catch (err) {
    print("caught")
  }
}
"#,
    ));
    assert_eq!(out, "OPENED\nCLOSED", "open+close must exact: {out}");
    assert_eq!(
        sql_conn_live_count(),
        baseline,
        "open+close must return live count to baseline"
    );
}

#[test]
fn ec_m2_3_sql_leak_open_without_close_increments() {
    reset_rusqlite_store();
    let baseline = sql_conn_live_count();
    assert_eq!(baseline, 0, "reset must clear explicit conns");

    let out = run(&facade_src(
        r#"
func main() {
  try {
    let c = Sql.open(dsn: ":memory:")
    print("LEAKED")
    print(c)
  } catch (err) {
    print("caught")
    print(err.message)
  }
}
"#,
    ));
    assert!(out.contains("LEAKED"), "open without close must succeed: {out}");
    assert!(!out.contains("caught"), "open must not throw: {out}");

    let after = sql_conn_live_count();
    assert_eq!(
        after,
        baseline + 1,
        "open without close must leave live_count == baseline + 1 (baseline={baseline}, after={after})"
    );

    // Nail cleanup: force-close the leaked id so later tests stay isolated (--test-threads=1).
    let id: i32 = out
        .lines()
        .filter_map(|l| l.trim().parse().ok())
        .next()
        .expect("printed conn id");
    assert!(id >= 1, "explicit conn id must be ≥ 1, got {id}");
    assert!(
        sql_conn_force_close(id),
        "force_close must drop leaked conn {id}"
    );
    assert_eq!(
        sql_conn_live_count(),
        baseline,
        "cleanup must restore baseline"
    );
}

#[test]
fn ec_m2_3_sql_leak_implicit_conn0_not_counted() {
    // Wave A W53 / ADR-397 / A-03: Sql implicit conn0 product exact.
    reset_rusqlite_store();
    assert_eq!(sql_conn_live_count(), 0);
    // Legacy static Sql.exec lazy-creates conn 0; it must not inflate the leak count.
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let _ = Sql.exec(sql: "CREATE TABLE t0 (v TEXT)")
    print("OK")
  } catch (err) {
    print("caught")
  }
}
"#,
    ));
    assert_eq!(out, "OK", "legacy exec must exact: {out}");
    assert_eq!(
        sql_conn_live_count(),
        0,
        "implicit conn 0 must not count toward sql_conn_live_count"
    );
    reset_rusqlite_store();
}
