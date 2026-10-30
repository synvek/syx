//! 01c Connection pool MVP nail.
// was: m2_s8_01c_connection_pool

use std::fs;
use std::path::PathBuf;

use syx_driver::{compile_and_run_embedded, DriverOptions};

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
    let p = std::env::temp_dir().join(format!("syx_m2_s8_01c_{name}.db"));
    let _ = fs::remove_file(&p);
    p
}

fn dsn_escape(p: &PathBuf) -> String {
    p.to_string_lossy().replace('\\', "\\\\")
}

#[test]
fn m2_s8_01c_max_size_exhaust_and_reuse() {
    // Wave A W63 / ADR-407 / A-03: Sql pool exhaust/reuse product exact (SAME label, no conn id).
    let db = tmp_db("reuse");
    let dsn = dsn_escape(&db);
    // Bare hosts for exhaust/reuse loop (nested catch binding is fragile in embed).
    let out = run(&facade_src(&format!(
        r#"
func main() {{
  let p = __syx_sql_pool_create("{dsn}", 2)
  let a = __syx_sql_pool_acquire(p)
  let b = __syx_sql_pool_acquire(p)
  print("GOT2")
  try {{
    let c = __syx_sql_pool_acquire(p)
    print("BAD_THIRD")
    print(c)
  }} catch (err1) {{
    print("EXHAUST")
  }}
  __syx_sql_pool_release(p, a)
  let d = __syx_sql_pool_acquire(p)
  print("REUSE")
  if d == a {{
    print("SAME")
  }} else {{
    print("DIFF")
  }}
  __syx_sql_pool_release(p, b)
  __syx_sql_pool_release(p, d)
  __syx_sql_pool_close(p)
  print("OK")
}}
"#
    )));
    assert_eq!(
        out, "GOT2\nEXHAUST\nREUSE\nSAME\nOK",
        "pool exhaust and reuse SAME must exact: {out}"
    );
    let _ = fs::remove_file(&db);
}

#[test]
fn m2_s8_01c_interleaved_exec_on_file_dsn() {
    // Wave A W61 / ADR-405 / A-03: Sql pool interleaved exec product exact (drop print(err)).
    let db = tmp_db("interleave");
    let dsn = dsn_escape(&db);
    let out = run(&facade_src(&format!(
        r#"
func main() {{
  try {{
    let p = Sql.poolCreate(dsn: "{dsn}", maxSize: 2)
    let a = Sql.poolAcquire(poolId: p)
    let b = Sql.poolAcquire(poolId: p)
    Sql.execOn(connId: a, sql: "CREATE TABLE t(x INTEGER)")
    Sql.execOn(connId: a, sql: "INSERT INTO t(x) VALUES (1)")
    Sql.execOn(connId: b, sql: "INSERT INTO t(x) VALUES (2)")
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
    assert_eq!(out, "OK\n2", "interleaved execOn on file DSN must exact: {out}");
    let _ = fs::remove_file(&db);
}

#[test]
fn m2_s8_01c_close_and_double_release_fail() {
    // Wave A W61 / ADR-405 / A-03: Sql pool double-release / close-after product exact.
    let db = tmp_db("close");
    let dsn = dsn_escape(&db);
    let out = run(&facade_src(&format!(
        r#"
func main() {{
  let p = __syx_sql_pool_create("{dsn}", 1)
  let a = __syx_sql_pool_acquire(p)
  __syx_sql_pool_release(p, a)
  try {{
    __syx_sql_pool_release(p, a)
    print("DOUBLE_OK")
  }} catch (err1) {{
    print("DOUBLE_FAIL")
  }}
  __syx_sql_pool_close(p)
  try {{
    let x = __syx_sql_pool_acquire(p)
    print("AFTER_OK")
    print(x)
  }} catch (err2) {{
    print("AFTER_FAIL")
  }}
  print("DONE")
}}
"#
    )));
    assert_eq!(
        out, "DOUBLE_FAIL\nAFTER_FAIL\nDONE",
        "double release and acquire after close must exact: {out}"
    );
    let _ = fs::remove_file(&db);
}

#[test]
fn m2_s8_01c_docs_and_capability() {
    let sql = include_str!("../../../../packages/data/sql.sy");
    assert!(
        sql.contains("poolCreate")
            && sql.contains("__syx_sql_pool_acquire")
            && sql.contains("ADR-245"),
        "Sql pool facade required"
    );
    let hosts = include_str!("../../../syx-vm/src/stdlib_phase89_hosts.rs");
    assert!(
        hosts.contains("SQL_POOLS")
            && hosts.contains("host_sql_pool_acquire")
            && hosts.contains("exhausted"),
        "pool store + non-blocking exhaust required"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("poolCreate")
            && guide.contains("ADR-245")
            && guide.contains("poolRelease"),
        "UsageGuide must document pool vs open/close"
    );
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "data.sqlite.pool");
    assert!(
        block.contains("status = \"implemented\""),
        "data.sqlite.pool must be implemented: {block}"
    );
    assert!(
        block.contains("m2_s8_01c_connection_pool"),
        "tests must list nail"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m2_s8_01c_connection_pool_tier = \"signed-ADR-245\""),
        "inventory must note signed-ADR-245"
    );
}

