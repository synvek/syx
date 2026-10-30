//! 02b typed SqlRow + bare SQL hatch nail.
// was: m2_s8_02b_typed_row

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

fn run_err(src: &str) -> String {
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Ok(r) => format!("UNEXPECTED_OK:{}", r.printed.join("\n")),
        // Wave A W117 / ADR-461: Display (not Debug) for product exact nails.
        Err(e) => format!("{e}"),
    }
}

fn facade_src(body: &str) -> String {
    let sql = include_str!("../../../../packages/data/sql.sy");
    let row = include_str!("../../../../packages/data/sql_row.sy");
    format!("{sql}\n{row}\n{body}")
}

#[test]
fn m2_error_sql_row_accessor_requires_sqlrow_throws() {
    let err = run_err(
        r#"
func main() {
  let _ = __syx_sql_row_get_string(0, "x")
}
"#,
    );
    // Wave A W117 / ADR-461 / A-03: SqlRow accessor requires SqlRow object Display product exact.
    assert_eq!(
        err, "[R003] SqlRow accessor requires SqlRow object",
        "SqlRow accessor non-object must exact: {err}"
    );
}

#[test]
fn m2_error_sql_row_missing_column_throws() {
    let err = run_err(&facade_src(
        r#"
func main() {
  let c = Sql.open(dsn: ":memory:")
  Sql.execOn(connId: c, sql: "CREATE TABLE t(id INTEGER, name TEXT)")
  Sql.execOn(connId: c, sql: "INSERT INTO t(id, name) VALUES (7, 'alice')")
  let rows = Sql.queryOn(connId: c, sql: "SELECT id, name FROM t")
  let row = rows[0]
  let _ = __syx_sql_row_get_string(row, "nope")
}
"#,
    ));
    // Wave A W118 / ADR-462 / A-03: SqlRow missing column Display product exact.
    assert_eq!(
        err, "[R003] SqlRow missing column 'nope'",
        "SqlRow missing column must exact: {err}"
    );
}

#[test]
fn m2_error_sql_row_get_int_type_mismatch_throws() {
    let err = run_err(&facade_src(
        r#"
func main() {
  let c = Sql.open(dsn: ":memory:")
  Sql.execOn(connId: c, sql: "CREATE TABLE t(id INTEGER, name TEXT)")
  Sql.execOn(connId: c, sql: "INSERT INTO t(id, name) VALUES (7, 'alice')")
  let rows = Sql.queryOn(connId: c, sql: "SELECT id, name FROM t")
  let row = rows[0]
  let _ = __syx_sql_row_get_int(row, "name")
}
"#,
    ));
    // Wave A W118 / ADR-462 / A-03: SqlRow getInt type mismatch Display product exact.
    assert_eq!(
        err, "[R003] SqlRow.getInt column 'name' type mismatch",
        "SqlRow getInt type mismatch must exact: {err}"
    );
}

#[test]
fn m2_error_sql_row_accessor_requires_type_tag_throws() {
    let err = run_err(
        r#"
func main() {
  let obj = __syx_json_decode("{\"id\":7}")
  let _ = __syx_sql_row_get_string(obj, "id")
}
"#,
    );
    // Wave A W119 / ADR-463 / A-03: SqlRow accessor requires __type=SqlRow Display product exact.
    assert_eq!(
        err, "[R003] SqlRow accessor requires __type=SqlRow",
        "SqlRow accessor missing __type tag must exact: {err}"
    );
}

#[test]
fn m2_error_sql_row_get_string_type_mismatch_throws() {
    let err = run_err(&facade_src(
        r#"
func main() {
  let c = Sql.open(dsn: ":memory:")
  Sql.execOn(connId: c, sql: "CREATE TABLE t(id INTEGER, name TEXT)")
  Sql.execOn(connId: c, sql: "INSERT INTO t(id, name) VALUES (7, 'alice')")
  let rows = Sql.queryOn(connId: c, sql: "SELECT id, name FROM t")
  let row = rows[0]
  let _ = __syx_sql_row_get_string(row, "id")
}
"#,
    ));
    // Wave A W119 / ADR-463 / A-03: SqlRow getString type mismatch Display product exact.
    assert_eq!(
        err, "[R003] SqlRow.getString column 'id' type mismatch",
        "SqlRow getString type mismatch must exact: {err}"
    );
}

#[test]
fn m2_error_sql_row_get_float_type_mismatch_throws() {
    let err = run_err(&facade_src(
        r#"
func main() {
  let c = Sql.open(dsn: ":memory:")
  Sql.execOn(connId: c, sql: "CREATE TABLE t(id INTEGER, name TEXT)")
  Sql.execOn(connId: c, sql: "INSERT INTO t(id, name) VALUES (7, 'alice')")
  let rows = Sql.queryOn(connId: c, sql: "SELECT id, name FROM t")
  let row = rows[0]
  let _ = __syx_sql_row_get_float(row, "name")
}
"#,
    ));
    // Wave A W121 / ADR-465 / A-03: SqlRow getFloat type mismatch Display product exact.
    assert_eq!(
        err, "[R003] SqlRow.getFloat column 'name' type mismatch",
        "SqlRow getFloat type mismatch must exact: {err}"
    );
}

#[test]
fn m2_error_sql_row_get_blob_type_mismatch_throws() {
    let err = run_err(&facade_src(
        r#"
func main() {
  let c = Sql.open(dsn: ":memory:")
  Sql.execOn(connId: c, sql: "CREATE TABLE t(id INTEGER, name TEXT)")
  Sql.execOn(connId: c, sql: "INSERT INTO t(id, name) VALUES (7, 'alice')")
  let rows = Sql.queryOn(connId: c, sql: "SELECT id, name FROM t")
  let row = rows[0]
  let _ = __syx_sql_row_get_blob(row, "id")
}
"#,
    ));
    // Wave A W121 / ADR-465 / A-03: SqlRow getBlob type mismatch Display product exact.
    assert_eq!(
        err, "[R003] SqlRow.getBlob column 'id' type mismatch",
        "SqlRow getBlob type mismatch must exact: {err}"
    );
}

#[test]
fn m2_s8_02b_get_int_string_and_mismatch() {
    // Wave A W62 / ADR-406 / A-03: SqlRow getInt/getString mismatch product exact (drop print(err)).
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let c = Sql.open(dsn: ":memory:")
    Sql.execOn(connId: c, sql: "CREATE TABLE t(id INTEGER, name TEXT)")
    Sql.execOn(connId: c, sql: "INSERT INTO t(id, name) VALUES (7, 'alice')")
    let rows = Sql.queryOn(connId: c, sql: "SELECT id, name FROM t")
    let row = rows[0]
    let id = SqlRow.getInt(row: row, name: "id")
    let name = SqlRow.getString(row: row, name: "name")
    print("OK")
    print(id)
    print(name)
    try {
      let bad = SqlRow.getInt(row: row, name: "name")
      print("MISMATCH_OK")
      print(bad)
    } catch (err1) {
      print("MISMATCH_FAIL")
    }
    try {
      let miss = SqlRow.getString(row: row, name: "nope")
      print("MISS_OK")
      print(miss)
    } catch (err2) {
      print("MISS_FAIL")
    }
    Sql.close(connId: c)
  } catch (err) {
    print("ERR")
  }
}
"#,
    ));
    assert_eq!(
        out, "OK\n7\nalice\nMISMATCH_FAIL\nMISS_FAIL",
        "getInt/getString + mismatch/miss must exact: {out}"
    );
}

#[test]
fn m2_s8_02b_column_names_and_is_null() {
    // Wave A W62 / ADR-406 / A-03: SqlRow columnNames/isNull product exact (drop print(err)).
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let c = Sql.open(dsn: ":memory:")
    Sql.execOn(connId: c, sql: "CREATE TABLE t(id INTEGER, note TEXT)")
    Sql.execOn(connId: c, sql: "INSERT INTO t(id, note) VALUES (1, NULL)")
    let rows = Sql.queryOn(connId: c, sql: "SELECT id, note FROM t")
    let row = rows[0]
    let names = SqlRow.columnNames(row: row)
    let nnull = SqlRow.isNull(row: row, name: "note")
    let idnull = SqlRow.isNull(row: row, name: "id")
    print("OK")
    print(names.len())
    print(nnull)
    print(idnull)
    Sql.close(connId: c)
  } catch (err) {
    print("ERR")
  }
}
"#,
    ));
    assert_eq!(
        out, "OK\n2\ntrue\nfalse",
        "columnNames/isNull must exact: {out}"
    );
}

#[test]
fn m2_s8_02b_docs_and_capability() {
    let row = include_str!("../../../../packages/data/sql_row.sy");
    assert!(
        row.contains("class SqlRow")
            && row.contains("getInt")
            && row.contains("__syx_sql_row_get_int")
            && row.contains("ADR-246"),
        "SqlRow class required"
    );
    let sql = include_str!("../../../../packages/data/sql.sy");
    assert!(
        sql.contains("Array of SqlRow") && sql.contains("bare SQL"),
        "sql.sy must document SqlRow / hatch"
    );
    assert!(
        !sql.contains("-> Object"),
        "query* must not claim -> Object success"
    );
    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("SqlRow")
            && guide.contains("ADR-246")
            && guide.contains("裸 SQL")
            && guide.contains("禁糊墙"),
        "UsageGuide must document SqlRow + bare SQL hatch"
    );
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "data.sqlite.row");
    assert!(
        block.contains("status = \"implemented\""),
        "data.sqlite.row must be implemented: {block}"
    );
    assert!(
        block.contains("m2_s8_02b_typed_row"),
        "tests must list nail"
    );
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m2_s8_02b_typed_row_tier = \"signed-ADR-246\""),
        "inventory must note signed-ADR-246"
    );
}

