//! 03b pool / concurrent txn threshold nail.
// was: m2_s8_03b_pool_txn

use std::fs;
use std::path::PathBuf;

use syx_driver::{compile_and_run_embedded, DriverOptions};
use syx_vm::{reset_rusqlite_store, sql_conn_live_count};

fn product_block<'a>(caps: &'a str, id: &str) -> &'a str {
    let needle = format!("id = \"{id}\"");
    let rest = caps
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing capability {id}"));
    rest.split("[[capability]]").next().unwrap_or(rest)
}

fn run(src: &str) -> String {
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Ok(r) => r.printed.join("\n"),
        Err(e) => panic!("compile/run failed: {e:?}\n---src---\n{src}"),
    }
}

fn facade_src(body: &str) -> String {
    let sql = include_str!("../../../../packages/data/sql.sy");
    format!("{sql}\n{body}")
}

fn tmp_db(name: &str) -> PathBuf {
    let p = std::env::temp_dir().join(format!("syx_m2_s8_03b_{name}.db"));
    let _ = fs::remove_file(&p);
    p
}

fn dsn_escape(p: &PathBuf) -> String {
    p.to_string_lossy().replace('\\', "\\\\")
}

#[test]
fn m2_s8_03b_dual_checkout_sequential_txn() {
    // Wave A W60 / ADR-404 / A-03: Sql pool dual-checkout sequential txn product exact (drop print(err)).
    let db = tmp_db("seq");
    let dsn = dsn_escape(&db);
    let out = run(&facade_src(&format!(
        r#"
func main() {{
  try {{
    let p = Sql.poolCreate(dsn: "{dsn}", maxSize: 2)
    let a = Sql.poolAcquire(poolId: p)
    let b = Sql.poolAcquire(poolId: p)
    Sql.execOn(connId: a, sql: "CREATE TABLE t(x INTEGER)")
    Sql.beginTransactionOn(connId: a)
    Sql.execOn(connId: a, sql: "INSERT INTO t(x) VALUES (1)")
    Sql.commitTransactionOn(connId: a)
    Sql.beginTransactionOn(connId: b)
    Sql.execOn(connId: b, sql: "INSERT INTO t(x) VALUES (2)")
    Sql.commitTransactionOn(connId: b)
    let rows = Sql.queryOn(connId: a, sql: "SELECT x FROM t ORDER BY x")
    print("OK")
    print(rows.len())
    Sql.poolRelease(poolId: p, connId: a)
    Sql.poolRelease(poolId: p, connId: b)
    Sql.poolClose(poolId: p)
  }} catch (err) {{
    print("ERR")
  }}
}}
"#
    )));
    assert_eq!(out, "OK\n2", "sequential dual-checkout txn must exact: {out}");
    let _ = fs::remove_file(&db);
}

#[test]
fn m2_s8_03b_second_immediate_busy() {
    // Wave A W59 / ADR-403 / A-03: Sql pool second IMMEDIATE begin busy product exact.
    let db = tmp_db("busy");
    let dsn = dsn_escape(&db);
    // Bare hosts avoid fragile nested catch binding on busy path.
    let out = run(&facade_src(&format!(
        r#"
func main() {{
  let p = __syx_sql_pool_create("{dsn}", 2)
  let a = __syx_sql_pool_acquire(p)
  let b = __syx_sql_pool_acquire(p)
  __syx_sql_begin_transaction_on(a)
  try {{
    __syx_sql_begin_transaction_on(b)
    print("BUSY_OK")
  }} catch (err1) {{
    print("BUSY_FAIL")
  }}
  __syx_sql_commit_transaction_on(a)
  try {{
    __syx_sql_begin_transaction_on(b)
    print("AFTER_OK")
  }} catch (err2) {{
    print("AFTER_FAIL")
  }}
  __syx_sql_commit_transaction_on(b)
  __syx_sql_pool_release(p, a)
  __syx_sql_pool_release(p, b)
  __syx_sql_pool_close(p)
  print("DONE")
}}
"#
    )));
    assert_eq!(
        out, "BUSY_FAIL\nAFTER_OK\nDONE",
        "second IMMEDIATE begin busy must exact: {out}"
    );
    let _ = fs::remove_file(&db);
}

#[test]
fn m2_s8_03b_release_mid_txn_throws() {
    // Wave A W59 / ADR-403 / A-03: Sql pool release mid-txn product exact.
    let db = tmp_db("mid");
    let dsn = dsn_escape(&db);
    let out = run(&facade_src(&format!(
        r#"
func main() {{
  let p = __syx_sql_pool_create("{dsn}", 1)
  let a = __syx_sql_pool_acquire(p)
  __syx_sql_begin_transaction_on(a)
  try {{
    __syx_sql_pool_release(p, a)
    print("MID_OK")
  }} catch (err1) {{
    print("MID_FAIL")
  }}
  __syx_sql_commit_transaction_on(a)
  try {{
    __syx_sql_pool_release(p, a)
    print("AFTER_COMMIT_OK")
  }} catch (err2) {{
    print("AFTER_COMMIT_FAIL")
  }}
  let b = __syx_sql_pool_acquire(p)
  __syx_sql_begin_transaction_on(b)
  try {{
    __syx_sql_pool_release(p, b)
    print("MID2_OK")
  }} catch (err3) {{
    print("MID2_FAIL")
  }}
  __syx_sql_rollback_transaction_on(b)
  try {{
    __syx_sql_pool_release(p, b)
    print("AFTER_ROLLBACK_OK")
  }} catch (err4) {{
    print("AFTER_ROLLBACK_FAIL")
  }}
  __syx_sql_pool_close(p)
  print("DONE")
}}
"#
    )));
    assert_eq!(
        out,
        "MID_FAIL\nAFTER_COMMIT_OK\nMID2_FAIL\nAFTER_ROLLBACK_OK\nDONE",
        "release mid-txn must exact: {out}"
    );
    let _ = fs::remove_file(&db);
}

#[test]
fn m2_s8_03b_pool_close_live_count() {
    // Wave A W60 / ADR-404 / A-03: Sql pool close-after acquire product exact (keep live_count).
    reset_rusqlite_store();
    let baseline = sql_conn_live_count();
    let db = tmp_db("leak");
    let dsn = dsn_escape(&db);
    let out = run(&facade_src(&format!(
        r#"
func main() {{
  let p = __syx_sql_pool_create("{dsn}", 2)
  let a = __syx_sql_pool_acquire(p)
  __syx_sql_begin_transaction_on(a)
  __syx_sql_pool_close(p)
  try {{
    let x = __syx_sql_pool_acquire(p)
    print("AFTER_OK")
    print(x)
  }} catch (err1) {{
    print("AFTER_FAIL")
  }}
  print("DONE")
}}
"#
    )));
    assert_eq!(out, "AFTER_FAIL\nDONE", "acquire after poolClose must exact: {out}");
    assert_eq!(
        sql_conn_live_count(),
        baseline,
        "poolClose must not leave pooled conns in sql_conn_live_count"
    );
    reset_rusqlite_store();
    let _ = fs::remove_file(&db);
}

#[test]
fn m2_s8_03b_nested_begin_throws() {
    // Wave A W47 / ADR-391 / A-03: nested beginTransactionOn product exact.
    let db = tmp_db("nested");
    let dsn = dsn_escape(&db);
    let out = run(&facade_src(&format!(
        r#"
func main() {{
  try {{
    let p = Sql.poolCreate(dsn: "{dsn}", maxSize: 1)
    let a = Sql.poolAcquire(poolId: p)
    Sql.beginTransactionOn(connId: a)
    try {{
      Sql.beginTransactionOn(connId: a)
      print("UNEXPECTED_OK")
    }} catch (err) {{
      print("caught")
    }}
    Sql.rollbackTransactionOn(connId: a)
    Sql.poolRelease(poolId: p, connId: a)
    Sql.poolClose(poolId: p)
  }} catch (err) {{
    print("OUTER")
    print(err)
  }}
}}
"#
    )));
    assert_eq!(
        out, "caught",
        "nested beginTransactionOn must exact: {out}"
    );
    let _ = fs::remove_file(&db);
}

#[test]
fn m2_s8_03b_docs_and_capability() {
    let host = include_str!("../../../syx-vm/src/stdlib_phase89_hosts.rs");
    assert!(
        host.contains("SQL_TXN_OPEN")
            && host.contains("open transaction")
            && host.contains("ADR-247"),
        "host must track SQL_TXN_OPEN and reject mid-txn release"
    );
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "data.sqlite.pool");
    assert!(
        block.contains("status = \"implemented\""),
        "data.sqlite.pool must be implemented: {block}"
    );
    assert!(
        block.contains("m2_s8_03b_pool_txn") && block.contains("ADR-247"),
        "capability must deepen for ADR-247: {block}"
    );
    assert!(
        !block.contains("≠ S8-03b") && !block.contains("!= S8-03b"),
        "notes must not still exclude S8-03b: {block}"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("open txn")
            || guide.contains("open transaction")
            || guide.contains("mid-txn")
            || guide.contains("IMMEDIATE"),
        "UsageGuide must document pool×txn threshold"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m2_s8_03b_pool_txn_tier = \"signed-ADR-247\""),
        "inventory must note signed-ADR-247"
    );
}

#[test]
fn m2_s8_03b_fence_no_ec_m2_4z_or_m2_exit() {
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    let s8_03b = roadmap
        .lines()
        .find(|l| l.starts_with("| M2-S8-03b |"))
        .expect("M2-S8-03b row");
    assert!(
        s8_03b.contains("| [x] |"),
        "M2-S8-03b must be checked: {s8_03b}"
    );
    assert!(
        roadmap.contains("S8 叶子收口") || roadmap.contains("S8 leaf"),
        "roadmap must note S8 leaf closeout"
    );
    assert!(
        roadmap.contains("W104") && roadmap.contains("M2-S1-03"),
        "roadmap must list W104 / M2-S1-03 as ADR-247 next lock"
    );
    // S1-03 may be closed by a later window (ADR-248); ADR-247 still locks it historically.
    assert!(
        !roadmap.contains("EC-M2-4-Z` `[x]`")
            && (roadmap.contains("**M2 已退出**")
                || roadmap.contains("ADR-259")
                || roadmap.contains("| **exited** |")),
        "W115 may claim M2 exited; must not claim EC-M2-4-Z via backtick pattern"
    );
}
