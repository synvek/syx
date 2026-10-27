//! HostFn Q-S1 Sql matrix nails (vm embed + native/wasm exact lines).

use std::fs;
use std::process::Command;

use syx_driver::{
    build_native_executable, compile_and_run_embedded, run_wasm_source_printed, DriverOptions,
};
use syx_vm::reset_rusqlite_store;

fn run(src: &str) -> Result<Vec<String>, String> {
    compile_and_run_embedded(src, &DriverOptions::default())
        .map(|r| r.printed)
        .map_err(|e| format!("{e:?}"))
}

fn facade_src(body: &str) -> String {
    let sql = include_str!("../../../../packages/data/sql.sy");
    format!("{sql}\n{body}")
}


fn host_block<'a>(toml: &'a str, name: &str) -> &'a str {
    let marker = format!("name = \"{name}\"");
    let start = toml
        .find(&marker)
        .unwrap_or_else(|| panic!("missing host {name}"));
    let rest = &toml[start..];
    let end = rest.find("\n[[host]]").unwrap_or(rest.len());
    &rest[..end]
}


#[test]
fn hostfn_sql_matrix_toml_lists_sql_batch() {
    let matrix = include_str!("../../../../packages/internal/hostfn_qs1_matrix.toml");
    crate::matrix_fingerprint::assert_live_matrix_fingerprint();
    assert!(
        matrix.contains("host = \"SqlOpen\"") && matrix.contains("host = \"SqlClose\""),
        "matrix missing SqlOpen/SqlClose"
    );
    assert!(
        matrix.contains("SqlClose") && matrix.contains("llvm = \"covered\""),
        "W15 SqlClose llvm cell must be present"
    );
    assert!(
        matrix.contains("SqlOpen") && matrix.contains("wasm = \"covered\""),
        "W27 SqlOpen wasm cell must be present"
    );
    assert!(
        matrix.contains("SqlClose") && matrix.contains("wasm = \"covered\""),
        "W28 SqlClose wasm cell must be present"
    );
    assert!(
        matrix.contains("vm = \"covered\"") && matrix.contains("mir = \"covered\""),
        "Sql batch must claim vm+mir covered"
    );
}

#[test]
fn hostfn_sql_close_invalid_fail_exact() {
    let out = run(&facade_src(
        r#"
func main() {
  try {
    Sql.close(connId: 999999)
    print("UNEXPECTED")
  } catch (err) {
    print("failed")
    print(err.code)
  }
}
"#,
    ))
    .expect("invalid close must catch");
    assert_eq!(out, vec!["failed".to_string(), "9".to_string()]);
}

#[test]
fn hostfn_sql_open_memory_close_exact() {
    let out = run(&facade_src(
        r#"
func main() {
  try {
    let c = Sql.open(dsn: ":memory:")
    print("opened")
    Sql.close(connId: c)
    print("closed")
  } catch (err) {
    print("caught")
    print(err.message)
  }
}
"#,
    ))
    .expect("memory open+close");
    assert_eq!(out, vec!["opened".to_string(), "closed".to_string()]);
}

/// Wave A W14 / ADR-358 / A-05 — SqlOpen llvm cell (native CallHost success path).
/// Exact open+close markers — err.code ObjectGet on llvm remains deepen (W16+).
#[cfg(unix)]
#[test]
fn hostfn_sql_open_memory_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let src = facade_src(
        r#"
func main() {
  try {
    let c = Sql.open(dsn: ":memory:")
    print("opened")
    Sql.close(connId: c)
    print("closed")
  } catch (err) {
    print("caught")
  }
  return 0
}
"#,
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_sql_llvm_open_memory_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    fs::write(dir.join("main.sy"), &src).expect("write main");
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .expect("write toml");
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<String> = stdout
        .lines()
        .map(|l| l.to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(
        lines,
        vec!["opened".to_string(), "closed".to_string()],
        "SqlOpen llvm open-memory must exact-success: {lines:?}"
    );
}

/// Wave A W27 / ADR-371 / A-05 — SqlOpen wasm cell (AOT CallHost success path).
/// Exact open+close markers — err.code ObjectGet remains deepen (W28+).
#[cfg(unix)]
#[test]
fn hostfn_sql_open_memory_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let src = facade_src(
        r#"
func main() {
  try {
    let c = Sql.open(dsn: ":memory:")
    print("opened")
    Sql.close(connId: c)
    print("closed")
  } catch (err) {
    print("caught")
  }
  return 0
}
"#,
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_sql_wasm_open_memory_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, &src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm sql open-memory failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .collect();
    assert_eq!(
        lines,
        vec!["opened".to_string(), "closed".to_string()],
        "SqlOpen wasm open-memory must exact-success: {lines:?}"
    );
}

/// Wave A W15 / ADR-359 / A-05 — SqlClose llvm cell (native CallHost try/catch).
/// Wave A W132 / ADR-476 / A-03: IOError.code ObjectGet native llvm exact.
#[cfg(unix)]
#[test]
fn hostfn_sql_close_invalid_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let src = facade_src(
        r#"
func main() {
  try {
    Sql.close(connId: 999999)
    print("UNEXPECTED")
  } catch (err) {
    print("failed")
    print(err.code)
  }
  return 0
}
"#,
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_sql_llvm_close_invalid_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    fs::write(dir.join("main.sy"), &src).expect("write main");
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .expect("write toml");
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<String> = stdout
        .lines()
        .map(|l| l.to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(
        lines,
        vec!["failed".to_string(), "9".to_string()],
        "SqlClose llvm invalid-close must exact-catch with code: {lines:?}"
    );
}

/// Wave A W28 / ADR-372 / A-05 — SqlClose wasm cell (AOT CallHost try/catch).
/// Wave A W130 / ADR-474 / A-03: IOError.code ObjectGet wasm exact.
#[cfg(unix)]
#[test]
fn hostfn_sql_close_invalid_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let src = facade_src(
        r#"
func main() {
  try {
    Sql.close(connId: 999999)
    print("UNEXPECTED")
  } catch (err) {
    print("failed")
    print(err.code)
  }
  return 0
}
"#,
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_sql_wasm_close_invalid_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, &src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm sql close-invalid failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .collect();
    assert_eq!(
        lines,
        vec!["failed".to_string(), "9".to_string()],
        "SqlClose wasm invalid-close must exact-catch with code: {lines:?}"
    );
}

#[test]
fn hostfn_sql_open_close_host_wasm_real() {
    // Wave A W272 / ADR-616 — Open/Close inventory wasm=real (reuse W27/W28 nails).
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for (name, llvm_adr) in [("SqlOpen", "ADR-206"), ("SqlClose", "ADR-211")] {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must declare llvm=real ({llvm_adr}): {block}"
        );
        assert!(
            block.contains("status = \"partial\""),
            "{name} must keep status=partial: {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-616): {block}"
        );
        assert!(
            block.contains("ADR-616") && block.contains(llvm_adr),
            "{name} notes must cite ADR-616 + {llvm_adr}: {block}"
        );
    }
}

#[test]
fn hostfn_sql_plan_a05_mentions_sql_batch() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    let a05 = roadmap
        .lines()
        .find(|l| l.starts_with("| A-05 |"))
        .expect("A-05 row");
    assert!(
        a05.contains("| [~] |") || a05.contains("| [x] |"),
        "A-05 must stay in progress or done: {a05}"
    );
    crate::matrix_fingerprint::assert_live_matrix_fingerprint();
}

/// Wave A W250 / ADR-594 / A-05 — SqlQueryOn llvm cell (native CallHost :memory: query).
#[cfg(unix)]
#[test]
fn hostfn_sql_query_on_memory_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let src = r#"
func main() {
  let c = __syx_sql_open(":memory:")
  let e1 = __syx_sql_exec_on(c, "CREATE TABLE t(x INT)")
  let e2 = __syx_sql_exec_on(c, "INSERT INTO t(x) VALUES (42)")
  let rows = __syx_sql_query_on(c, "SELECT x FROM t")
  print(rows.count)
  __syx_sql_close(c)
  return 0
}
"#;
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_sql_llvm_query_on_memory_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    fs::write(dir.join("main.sy"), src).expect("write main");
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .expect("write toml");
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<String> = stdout
        .lines()
        .map(|l| l.to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(
        lines,
        vec!["1".to_string()],
        "SqlQueryOn llvm :memory: query must exact-success: {lines:?}"
    );
}

/// Wave A W272 / ADR-616 — SqlExecOn + SqlQueryOn wasm :memory: narrow.
#[cfg(unix)]
#[test]
fn hostfn_sql_query_on_memory_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let src = r#"
func main() {
  let c = __syx_sql_open(":memory:")
  let e1 = __syx_sql_exec_on(c, "CREATE TABLE t(x INT)")
  let e2 = __syx_sql_exec_on(c, "INSERT INTO t(x) VALUES (42)")
  let rows = __syx_sql_query_on(c, "SELECT x FROM t")
  print(rows.count)
  __syx_sql_close(c)
  return 0
}
"#;
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_sql_wasm_query_on_memory_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm sql query_on failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines.into_iter().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        lines,
        vec!["1".to_string()],
        "SqlExecOn/QueryOn wasm :memory: query must exact-success: {lines:?}"
    );
}

/// Wave A W250 / ADR-594 / A-05 — Begin/Commit/Rollback TransactionOn llvm cells.
#[cfg(unix)]
#[test]
fn hostfn_sql_txn_commit_rollback_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let src = r#"
func main() {
  let c = __syx_sql_open(":memory:")
  let e0 = __syx_sql_exec_on(c, "CREATE TABLE t(x INT)")
  __syx_sql_begin_transaction_on(c)
  let e1 = __syx_sql_exec_on(c, "INSERT INTO t(x) VALUES (1)")
  __syx_sql_commit_transaction_on(c)
  let rows = __syx_sql_query_on(c, "SELECT x FROM t")
  print(rows.count)
  __syx_sql_close(c)

  let c2 = __syx_sql_open(":memory:")
  let e2 = __syx_sql_exec_on(c2, "CREATE TABLE t(x INT)")
  __syx_sql_begin_transaction_on(c2)
  let e3 = __syx_sql_exec_on(c2, "INSERT INTO t(x) VALUES (2)")
  __syx_sql_rollback_transaction_on(c2)
  let rows2 = __syx_sql_query_on(c2, "SELECT x FROM t")
  print(rows2.count)
  __syx_sql_close(c2)
  return 0
}
"#;
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_sql_llvm_txn_commit_rollback_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    fs::write(dir.join("main.sy"), src).expect("write main");
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .expect("write toml");
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<String> = stdout
        .lines()
        .map(|l| l.to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(
        lines,
        vec!["1".to_string(), "0".to_string()],
        "Sql txn On llvm commit/rollback must exact 1/0: {lines:?}"
    );
}

/// Wave A W273 / ADR-617 — Begin/Commit/RollbackTransactionOn wasm.
#[cfg(unix)]
#[test]
fn hostfn_sql_txn_commit_rollback_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let src = r#"
func main() {
  let c = __syx_sql_open(":memory:")
  let e0 = __syx_sql_exec_on(c, "CREATE TABLE t(x INT)")
  __syx_sql_begin_transaction_on(c)
  let e1 = __syx_sql_exec_on(c, "INSERT INTO t(x) VALUES (1)")
  __syx_sql_commit_transaction_on(c)
  let rows = __syx_sql_query_on(c, "SELECT x FROM t")
  print(rows.count)
  __syx_sql_close(c)

  let c2 = __syx_sql_open(":memory:")
  let e2 = __syx_sql_exec_on(c2, "CREATE TABLE t(x INT)")
  __syx_sql_begin_transaction_on(c2)
  let e3 = __syx_sql_exec_on(c2, "INSERT INTO t(x) VALUES (2)")
  __syx_sql_rollback_transaction_on(c2)
  let rows2 = __syx_sql_query_on(c2, "SELECT x FROM t")
  print(rows2.count)
  __syx_sql_close(c2)
  return 0
}
"#;
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_sql_wasm_txn_commit_rollback_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm sql txn On failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines.into_iter().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        lines,
        vec!["1".to_string(), "0".to_string()],
        "Sql txn On wasm commit/rollback must exact 1/0: {lines:?}"
    );
}

#[test]
fn hostfn_sql_query_on_host_llvm_real() {
    // Wave A W250 / ADR-594 llvm + Wave A W272 / ADR-616 wasm=real
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlQueryOn");
    assert!(
        block.contains("llvm = \"real\""),
        "SqlQueryOn must declare llvm=real (ADR-594): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "SqlQueryOn must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "SqlQueryOn must declare wasm=real (ADR-616): {block}"
    );
    assert!(
        block.contains("ADR-616") && block.contains("ADR-594"),
        "SqlQueryOn notes must cite ADR-616 + ADR-594: {block}"
    );
}

#[test]
fn hostfn_sql_exec_on_host_wasm_real() {
    // Wave A W272 / ADR-616 — ExecOn covered by qs1_sql_query_on_memory_wasm
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlExecOn");
    assert!(
        block.contains("llvm = \"real\""),
        "SqlExecOn must declare llvm=real (ADR-216): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "SqlExecOn must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "SqlExecOn must declare wasm=real (ADR-616): {block}"
    );
    assert!(
        block.contains("ADR-616") && block.contains("ADR-216"),
        "SqlExecOn notes must cite ADR-616 + ADR-216: {block}"
    );
}

#[test]
fn hostfn_sql_begin_transaction_on_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlBeginTransactionOn");
    assert!(
        block.contains("llvm = \"real\""),
        "SqlBeginTransactionOn must declare llvm=real (ADR-594): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "SqlBeginTransactionOn must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "SqlBeginTransactionOn must declare wasm=real (ADR-617): {block}"
    );
    assert!(
        block.contains("ADR-617") && block.contains("ADR-594"),
        "SqlBeginTransactionOn notes must cite ADR-617 + ADR-594: {block}"
    );
}

#[test]
fn hostfn_sql_commit_transaction_on_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlCommitTransactionOn");
    assert!(
        block.contains("llvm = \"real\""),
        "SqlCommitTransactionOn must declare llvm=real (ADR-594): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "SqlCommitTransactionOn must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "SqlCommitTransactionOn must declare wasm=real (ADR-617): {block}"
    );
    assert!(
        block.contains("ADR-617") && block.contains("ADR-594"),
        "SqlCommitTransactionOn notes must cite ADR-617 + ADR-594: {block}"
    );
}

#[test]
fn hostfn_sql_rollback_transaction_on_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlRollbackTransactionOn");
    assert!(
        block.contains("llvm = \"real\""),
        "SqlRollbackTransactionOn must declare llvm=real (ADR-594): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "SqlRollbackTransactionOn must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "SqlRollbackTransactionOn must declare wasm=real (ADR-617): {block}"
    );
    assert!(
        block.contains("ADR-617") && block.contains("ADR-594"),
        "SqlRollbackTransactionOn notes must cite ADR-617 + ADR-594: {block}"
    );
}

/// Wave A W251 / ADR-595 / A-05 — PrepareOn+BindStepOn+BindPreparedOn+SqlRowGetInt llvm.
#[cfg(unix)]
#[test]
fn hostfn_sql_prepare_bind_row_get_int_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let src = r#"
func main() {
  let c = __syx_sql_open(":memory:")
  let e0 = __syx_sql_exec_on(c, "CREATE TABLE t(x INT)")
  let n1 = __syx_sql_bind_step_on(c, "INSERT INTO t(x) VALUES (?)", "41")
  print(n1)
  let stmt = __syx_sql_prepare_on(c, "INSERT INTO t(x) VALUES (?)")
  let n2 = __syx_sql_bind_prepared_on(c, stmt, "42")
  print(n2)
  let rows = __syx_sql_query_on(c, "SELECT x FROM t WHERE x = 42")
  print(rows.count)
  let row = rows[0]
  print(__syx_sql_row_get_int(row, "x"))
  __syx_sql_close(c)
  return 0
}
"#;
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_sql_llvm_prepare_bind_get_int_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    fs::write(dir.join("main.sy"), src).expect("write main");
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .expect("write toml");
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<String> = stdout
        .lines()
        .map(|l| l.to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(
        lines,
        vec![
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
            "42".to_string(),
        ],
        "Sql prepare/bind/getInt llvm must exact 1/1/1/42: {lines:?}"
    );
}

/// Wave A W273 / ADR-617 — PrepareOn+BindStepOn+BindPreparedOn wasm.
#[cfg(unix)]
#[test]
fn hostfn_sql_prepare_bind_row_get_int_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let src = r#"
func main() {
  let c = __syx_sql_open(":memory:")
  let e0 = __syx_sql_exec_on(c, "CREATE TABLE t(x INT)")
  let n1 = __syx_sql_bind_step_on(c, "INSERT INTO t(x) VALUES (?)", "41")
  print(n1)
  let stmt = __syx_sql_prepare_on(c, "INSERT INTO t(x) VALUES (?)")
  let n2 = __syx_sql_bind_prepared_on(c, stmt, "42")
  print(n2)
  let rows = __syx_sql_query_on(c, "SELECT x FROM t WHERE x = 42")
  print(rows.count)
  let row = rows[0]
  print(__syx_sql_row_get_int(row, "x"))
  __syx_sql_close(c)
  return 0
}
"#;
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_sql_wasm_prepare_bind_get_int_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm sql prepare/bind failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines.into_iter().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        lines,
        vec![
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
            "42".to_string(),
        ],
        "Sql prepare/bind/getInt wasm must exact 1/1/1/42: {lines:?}"
    );
}

#[test]
fn hostfn_sql_prepare_on_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlPrepareOn");
    assert!(
        block.contains("llvm = \"real\""),
        "SqlPrepareOn must declare llvm=real (ADR-595): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "SqlPrepareOn must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "SqlPrepareOn must declare wasm=real (ADR-617): {block}"
    );
    assert!(
        block.contains("ADR-617") && block.contains("ADR-595"),
        "SqlPrepareOn notes must cite ADR-617 + ADR-595: {block}"
    );
}

#[test]
fn hostfn_sql_bind_step_on_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlBindStepOn");
    assert!(
        block.contains("llvm = \"real\""),
        "SqlBindStepOn must declare llvm=real (ADR-595): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "SqlBindStepOn must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "SqlBindStepOn must declare wasm=real (ADR-617): {block}"
    );
    assert!(
        block.contains("ADR-617") && block.contains("ADR-595"),
        "SqlBindStepOn notes must cite ADR-617 + ADR-595: {block}"
    );
}

#[test]
fn hostfn_sql_bind_prepared_on_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlBindPreparedOn");
    assert!(
        block.contains("llvm = \"real\""),
        "SqlBindPreparedOn must declare llvm=real (ADR-595): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "SqlBindPreparedOn must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "SqlBindPreparedOn must declare wasm=real (ADR-617): {block}"
    );
    assert!(
        block.contains("ADR-617") && block.contains("ADR-595"),
        "SqlBindPreparedOn notes must cite ADR-617 + ADR-595: {block}"
    );
}

#[test]
fn hostfn_sql_row_get_int_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlRowGetInt");
    assert!(
        block.contains("llvm = \"real\""),
        "SqlRowGetInt must declare llvm=real (ADR-595): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "SqlRowGetInt must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "SqlRowGetInt must declare wasm=real (ADR-713): {block}"
    );
    assert!(
        block.contains("ADR-713") && block.contains("ADR-595") && block.contains("ADR-246"),
        "SqlRowGetInt notes must cite ADR-713 + ADR-595 + ADR-246: {block}"
    );
}

/// Wave A W252 / ADR-596 / A-05 — SqlPool Create/Acquire/Release/Close llvm (file DSN).
/// Reuse evidenced by printed conn ids (LLVM `==` on HostFn Ints unreliable).
#[cfg(unix)]
#[test]
fn hostfn_sql_pool_lifecycle_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let db = std::env::temp_dir().join(format!(
        "syx_hostfn_sql_llvm_pool_{}.db",
        std::process::id()
    ));
    let _ = fs::remove_file(&db);
    let dsn = db.to_string_lossy().replace('\\', "\\\\");
    let src = format!(
        r#"
func main() {{
  let p = __syx_sql_pool_create("{dsn}", 2)
  let a = __syx_sql_pool_acquire(p)
  let b = __syx_sql_pool_acquire(p)
  print("GOT2")
  __syx_sql_pool_release(p, a)
  let d = __syx_sql_pool_acquire(p)
  print(a)
  print(d)
  __syx_sql_pool_release(p, b)
  __syx_sql_pool_release(p, d)
  __syx_sql_pool_close(p)
  print("OK")
  return 0
}}
"#,
        dsn = dsn
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_sql_llvm_pool_lifecycle_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    fs::write(dir.join("main.sy"), &src).expect("write main");
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .expect("write toml");
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<String> = stdout
        .lines()
        .map(|l| l.to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let _ = fs::remove_dir_all(&dir);
    let _ = fs::remove_file(&db);
    assert_eq!(lines.len(), 4, "pool lifecycle must print 4 lines: {lines:?}");
    assert_eq!(lines[0], "GOT2", "pool acquire×2 must GOT2: {lines:?}");
    assert_eq!(
        lines[1], lines[2],
        "pool release+reacquire must reuse same conn id: {lines:?}"
    );
    assert_eq!(lines[3], "OK", "pool close must OK: {lines:?}");
}

/// Wave A W372 / ADR-716 — SqlPool Create/Acquire/Release/Close wasm (file DSN).
/// Mirror `qs1_sql_pool_lifecycle_native_llvm` exact (GOT2 / reuse id / OK).
#[test]
fn hostfn_sql_pool_lifecycle_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let db = std::env::temp_dir().join(format!(
        "syx_hostfn_sql_wasm_pool_{}.db",
        std::process::id()
    ));
    let _ = fs::remove_file(&db);
    let dsn = db.to_string_lossy().replace('\\', "\\\\");
    let src = format!(
        r#"
func main() {{
  let p = __syx_sql_pool_create("{dsn}", 2)
  let a = __syx_sql_pool_acquire(p)
  let b = __syx_sql_pool_acquire(p)
  print("GOT2")
  __syx_sql_pool_release(p, a)
  let d = __syx_sql_pool_acquire(p)
  print(a)
  print(d)
  __syx_sql_pool_release(p, b)
  __syx_sql_pool_release(p, d)
  __syx_sql_pool_close(p)
  print("OK")
  return 0
}}
"#,
        dsn = dsn
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_sql_wasm_pool_lifecycle_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, &src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm sql pool lifecycle failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    let _ = fs::remove_file(&db);
    let lines: Vec<String> = lines.into_iter().filter(|l| !l.is_empty()).collect();
    assert_eq!(lines.len(), 4, "pool lifecycle wasm must print 4 lines: {lines:?}");
    assert_eq!(lines[0], "GOT2", "pool acquire×2 wasm must GOT2: {lines:?}");
    assert_eq!(
        lines[1], lines[2],
        "pool release+reacquire wasm must reuse same conn id: {lines:?}"
    );
    assert_eq!(lines[3], "OK", "pool close wasm must OK: {lines:?}");
}

#[test]
fn hostfn_sql_pool_create_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlPoolCreate");
    assert!(
        block.contains("llvm = \"real\""),
        "SqlPoolCreate must declare llvm=real (ADR-596): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "SqlPoolCreate must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "SqlPoolCreate must declare wasm=real (ADR-716): {block}"
    );
    assert!(
        block.contains("ADR-716") && block.contains("ADR-596") && block.contains("ADR-245"),
        "SqlPoolCreate notes must cite ADR-716 + ADR-596 + ADR-245: {block}"
    );
}

#[test]
fn hostfn_sql_pool_acquire_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlPoolAcquire");
    assert!(
        block.contains("llvm = \"real\""),
        "SqlPoolAcquire must declare llvm=real (ADR-596): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "SqlPoolAcquire must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "SqlPoolAcquire must declare wasm=real (ADR-716): {block}"
    );
    assert!(
        block.contains("ADR-716") && block.contains("ADR-596") && block.contains("ADR-245"),
        "SqlPoolAcquire notes must cite ADR-716 + ADR-596 + ADR-245: {block}"
    );
}

#[test]
fn hostfn_sql_pool_release_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlPoolRelease");
    assert!(
        block.contains("llvm = \"real\""),
        "SqlPoolRelease must declare llvm=real (ADR-596): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "SqlPoolRelease must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "SqlPoolRelease must declare wasm=real (ADR-716): {block}"
    );
    assert!(
        block.contains("ADR-716") && block.contains("ADR-596") && block.contains("ADR-245"),
        "SqlPoolRelease notes must cite ADR-716 + ADR-596 + ADR-245: {block}"
    );
}

#[test]
fn hostfn_sql_pool_close_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlPoolClose");
    assert!(
        block.contains("llvm = \"real\""),
        "SqlPoolClose must declare llvm=real (ADR-596): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "SqlPoolClose must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "SqlPoolClose must declare wasm=real (ADR-716): {block}"
    );
    assert!(
        block.contains("ADR-716") && block.contains("ADR-596") && block.contains("ADR-245"),
        "SqlPoolClose notes must cite ADR-716 + ADR-596 + ADR-245: {block}"
    );
}

/// Wave A W253 / ADR-597 / A-05 — remaining SqlRow accessors llvm.
#[cfg(unix)]
#[test]
fn hostfn_sql_row_accessors_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let src = r#"
func main() {
  let c = __syx_sql_open(":memory:")
  let e0 = __syx_sql_exec_on(c, "CREATE TABLE t(name TEXT, score REAL, bin BLOB, note TEXT)")
  let e1 = __syx_sql_exec_on(c, "INSERT INTO t(name, score, bin, note) VALUES ('alice', 1.5, X'0102', NULL)")
  let rows = __syx_sql_query_on(c, "SELECT name, score, bin, note FROM t")
  let row = rows[0]
  print(__syx_sql_row_get_string(row, "name"))
  print(__syx_sql_row_get_float(row, "score"))
  let blob = __syx_sql_row_get_blob(row, "bin")
  print(blob.count)
  print(__syx_sql_row_is_null(row, "note"))
  print(__syx_sql_row_is_null(row, "name"))
  let names = __syx_sql_row_column_names(row)
  print(names.count)
  __syx_sql_close(c)
  return 0
}
"#;
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_sql_llvm_row_accessors_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    fs::write(dir.join("main.sy"), src).expect("write main");
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .expect("write toml");
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<String> = stdout
        .lines()
        .map(|l| l.to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(
        lines,
        vec![
            "alice".to_string(),
            "1.5".to_string(),
            "2".to_string(),
            "true".to_string(),
            "false".to_string(),
            "4".to_string(),
        ],
        "SqlRow accessors llvm must exact alice/1.5/2/true/false/4: {lines:?}"
    );
}

/// Wave A W370 / ADR-714 — remaining SqlRow accessors wasm (mirror native exact).
#[cfg(unix)]
#[test]
fn hostfn_sql_row_accessors_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let src = r#"
func main() {
  let c = __syx_sql_open(":memory:")
  let e0 = __syx_sql_exec_on(c, "CREATE TABLE t(name TEXT, score REAL, bin BLOB, note TEXT)")
  let e1 = __syx_sql_exec_on(c, "INSERT INTO t(name, score, bin, note) VALUES ('alice', 1.5, X'0102', NULL)")
  let rows = __syx_sql_query_on(c, "SELECT name, score, bin, note FROM t")
  let row = rows[0]
  print(__syx_sql_row_get_string(row, "name"))
  print(__syx_sql_row_get_float(row, "score"))
  let blob = __syx_sql_row_get_blob(row, "bin")
  print(blob.count)
  print(__syx_sql_row_is_null(row, "note"))
  print(__syx_sql_row_is_null(row, "name"))
  let names = __syx_sql_row_column_names(row)
  print(names.count)
  __syx_sql_close(c)
  return 0
}
"#;
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_sql_wasm_row_accessors_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm sql row accessors failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines.into_iter().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        lines,
        vec![
            "alice".to_string(),
            "1.5".to_string(),
            "2".to_string(),
            "true".to_string(),
            "false".to_string(),
            "4".to_string(),
        ],
        "SqlRow accessors wasm must exact alice/1.5/2/true/false/4: {lines:?}"
    );
}

#[test]
fn hostfn_sql_row_get_string_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlRowGetString");
    assert!(block.contains("llvm = \"real\""), "SqlRowGetString llvm=real: {block}");
    assert!(block.contains("status = \"partial\""), "SqlRowGetString status=partial: {block}");
    assert!(block.contains("wasm = \"real\""), "SqlRowGetString wasm=real (ADR-714): {block}");
    assert!(
        block.contains("ADR-714") && block.contains("ADR-597") && block.contains("ADR-246"),
        "SqlRowGetString notes ADR-714+597+246: {block}"
    );
}

#[test]
fn hostfn_sql_row_get_float_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlRowGetFloat");
    assert!(block.contains("llvm = \"real\""), "SqlRowGetFloat llvm=real: {block}");
    assert!(block.contains("status = \"partial\""), "SqlRowGetFloat status=partial: {block}");
    assert!(block.contains("wasm = \"real\""), "SqlRowGetFloat wasm=real (ADR-714): {block}");
    assert!(
        block.contains("ADR-714") && block.contains("ADR-597") && block.contains("ADR-246"),
        "SqlRowGetFloat notes ADR-714+597+246: {block}"
    );
}

#[test]
fn hostfn_sql_row_get_blob_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlRowGetBlob");
    assert!(block.contains("llvm = \"real\""), "SqlRowGetBlob llvm=real: {block}");
    assert!(block.contains("status = \"partial\""), "SqlRowGetBlob status=partial: {block}");
    assert!(block.contains("wasm = \"real\""), "SqlRowGetBlob wasm=real (ADR-714): {block}");
    assert!(
        block.contains("ADR-714") && block.contains("ADR-597") && block.contains("ADR-246"),
        "SqlRowGetBlob notes ADR-714+597+246: {block}"
    );
}

#[test]
fn hostfn_sql_row_is_null_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlRowIsNull");
    assert!(block.contains("llvm = \"real\""), "SqlRowIsNull llvm=real: {block}");
    assert!(block.contains("status = \"partial\""), "SqlRowIsNull status=partial: {block}");
    assert!(block.contains("wasm = \"real\""), "SqlRowIsNull wasm=real (ADR-714): {block}");
    assert!(
        block.contains("ADR-714") && block.contains("ADR-597") && block.contains("ADR-246"),
        "SqlRowIsNull notes ADR-714+597+246: {block}"
    );
}

#[test]
fn hostfn_sql_row_column_names_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlRowColumnNames");
    assert!(block.contains("llvm = \"real\""), "SqlRowColumnNames llvm=real: {block}");
    assert!(block.contains("status = \"partial\""), "SqlRowColumnNames status=partial: {block}");
    assert!(block.contains("wasm = \"real\""), "SqlRowColumnNames wasm=real (ADR-714): {block}");
    assert!(
        block.contains("ADR-714") && block.contains("ADR-597") && block.contains("ADR-246"),
        "SqlRowColumnNames notes ADR-714+597+246: {block}"
    );
}

/// Wave A W254 / ADR-598 / A-05 — legacy conn-0 SqlExec/Query/Prepare/BindStep llvm.
#[cfg(unix)]
#[test]
fn hostfn_sql_legacy_crud_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let src = r#"
func main() {
  let e0 = __syx_sql_exec("CREATE TABLE t(id INT, name TEXT)")
  let e1 = __syx_sql_exec("INSERT INTO t VALUES (1, 'alice')")
  let rows = __syx_sql_query("SELECT * FROM t")
  print(e0)
  print(e1)
  print(rows.count)
  let stmt = __syx_sql_prepare("INSERT INTO t(name) VALUES (?)")
  print(stmt)
  let changed = __syx_sql_bind_step("INSERT INTO t(name) VALUES (?)", "bob")
  print(changed)
  let rows2 = __syx_sql_query("SELECT * FROM t")
  print(rows2.count)
  return 0
}
"#;
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_sql_llvm_legacy_crud_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    fs::write(dir.join("main.sy"), src).expect("write main");
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .expect("write toml");
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<String> = stdout
        .lines()
        .map(|l| l.to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(
        lines.len(),
        6,
        "legacy Sql CRUD llvm must print 6 lines: {lines:?}"
    );
    assert_eq!(lines[0], "0", "CREATE changed count: {lines:?}");
    assert_eq!(lines[1], "1", "INSERT changed count: {lines:?}");
    assert_eq!(lines[2], "1", "first query row count: {lines:?}");
    assert!(
        lines[3].parse::<i32>().unwrap_or(0) >= 1,
        "prepare stmt id must be >= 1: {lines:?}"
    );
    assert_eq!(lines[4], "1", "bindStep changed count: {lines:?}");
    assert_eq!(lines[5], "2", "second query row count: {lines:?}");
}

/// Wave A W272 / ADR-616 — legacy conn-0 SqlExec/Query/Prepare/BindStep wasm.
#[cfg(unix)]
#[test]
fn hostfn_sql_legacy_crud_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    reset_rusqlite_store();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let src = r#"
func main() {
  let e0 = __syx_sql_exec("CREATE TABLE t(id INT, name TEXT)")
  let e1 = __syx_sql_exec("INSERT INTO t VALUES (1, 'alice')")
  let rows = __syx_sql_query("SELECT * FROM t")
  print(e0)
  print(e1)
  print(rows.count)
  let stmt = __syx_sql_prepare("INSERT INTO t(name) VALUES (?)")
  print(stmt)
  let changed = __syx_sql_bind_step("INSERT INTO t(name) VALUES (?)", "bob")
  print(changed)
  let rows2 = __syx_sql_query("SELECT * FROM t")
  print(rows2.count)
  return 0
}
"#;
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_sql_wasm_legacy_crud_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm sql legacy crud failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines.into_iter().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        lines.len(),
        6,
        "legacy Sql CRUD wasm must print 6 lines: {lines:?}"
    );
    assert_eq!(lines[0], "0", "CREATE changed count: {lines:?}");
    assert_eq!(lines[1], "1", "INSERT changed count: {lines:?}");
    assert_eq!(lines[2], "1", "first query row count: {lines:?}");
    assert!(
        lines[3].parse::<i32>().unwrap_or(0) >= 1,
        "prepare stmt id must be >= 1: {lines:?}"
    );
    assert_eq!(lines[4], "1", "bindStep changed count: {lines:?}");
    assert_eq!(lines[5], "2", "second query row count: {lines:?}");
}

#[test]
fn hostfn_sql_exec_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlExec");
    assert!(block.contains("llvm = \"real\""), "SqlExec llvm=real: {block}");
    assert!(block.contains("status = \"partial\""), "SqlExec status=partial: {block}");
    assert!(block.contains("wasm = \"real\""), "SqlExec wasm=real (ADR-616): {block}");
    assert!(
        block.contains("ADR-616") && block.contains("ADR-598"),
        "SqlExec notes ADR-616+598: {block}"
    );
}

#[test]
fn hostfn_sql_query_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlQuery");
    assert!(block.contains("llvm = \"real\""), "SqlQuery llvm=real: {block}");
    assert!(block.contains("status = \"partial\""), "SqlQuery status=partial: {block}");
    assert!(block.contains("wasm = \"real\""), "SqlQuery wasm=real (ADR-616): {block}");
    assert!(
        block.contains("ADR-616") && block.contains("ADR-598"),
        "SqlQuery notes ADR-616+598: {block}"
    );
}

#[test]
fn hostfn_sql_prepare_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlPrepare");
    assert!(block.contains("llvm = \"real\""), "SqlPrepare llvm=real: {block}");
    assert!(block.contains("status = \"partial\""), "SqlPrepare status=partial: {block}");
    assert!(block.contains("wasm = \"real\""), "SqlPrepare wasm=real (ADR-616): {block}");
    assert!(
        block.contains("ADR-616") && block.contains("ADR-598"),
        "SqlPrepare notes ADR-616+598: {block}"
    );
}

#[test]
fn hostfn_sql_bind_step_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlBindStep");
    assert!(block.contains("llvm = \"real\""), "SqlBindStep llvm=real: {block}");
    assert!(block.contains("status = \"partial\""), "SqlBindStep status=partial: {block}");
    assert!(block.contains("wasm = \"real\""), "SqlBindStep wasm=real (ADR-616): {block}");
    assert!(
        block.contains("ADR-616") && block.contains("ADR-598"),
        "SqlBindStep notes ADR-616+598: {block}"
    );
}

/// Wave A W255 / ADR-599 / A-05 — legacy conn-0 SqlBegin/Commit/Rollback llvm.
#[cfg(unix)]
#[test]
fn hostfn_sql_legacy_txn_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let src = r#"
func main() {
  __syx_sql_exec("CREATE TABLE t(x INT)")
  print(__syx_sql_begin_transaction())
  __syx_sql_exec("INSERT INTO t(x) VALUES (1)")
  print(__syx_sql_commit_transaction())
  let rows = __syx_sql_query("SELECT x FROM t")
  print(rows.count)
  print(__syx_sql_begin_transaction())
  __syx_sql_exec("INSERT INTO t(x) VALUES (2)")
  print(__syx_sql_rollback_transaction())
  let rows2 = __syx_sql_query("SELECT x FROM t")
  print(rows2.count)
  return 0
}
"#;
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_sql_llvm_legacy_txn_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    fs::write(dir.join("main.sy"), src).expect("write main");
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .expect("write toml");
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<String> = stdout
        .lines()
        .map(|l| l.to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(
        lines,
        vec![
            "true".to_string(),
            "true".to_string(),
            "1".to_string(),
            "true".to_string(),
            "true".to_string(),
            "1".to_string(),
        ],
        "legacy Sql txn llvm must exact true/true/1/true/true/1: {lines:?}"
    );
}

/// Wave A W273 / ADR-617 — legacy Begin/Commit/Rollback wasm.
#[cfg(unix)]
#[test]
fn hostfn_sql_legacy_txn_wasm() {
    syx_host_plugin::register_echo_plugin_in_process();
    reset_rusqlite_store();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let src = r#"
func main() {
  __syx_sql_exec("CREATE TABLE t(x INT)")
  print(__syx_sql_begin_transaction())
  __syx_sql_exec("INSERT INTO t(x) VALUES (1)")
  print(__syx_sql_commit_transaction())
  let rows = __syx_sql_query("SELECT x FROM t")
  print(rows.count)
  print(__syx_sql_begin_transaction())
  __syx_sql_exec("INSERT INTO t(x) VALUES (2)")
  print(__syx_sql_rollback_transaction())
  let rows2 = __syx_sql_query("SELECT x FROM t")
  print(rows2.count)
  return 0
}
"#;
    let dir = std::env::temp_dir().join(format!(
        "syx_hostfn_sql_wasm_legacy_txn_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm sql legacy txn failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = lines.into_iter().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        lines,
        vec![
            "true".to_string(),
            "true".to_string(),
            "1".to_string(),
            "true".to_string(),
            "true".to_string(),
            "1".to_string(),
        ],
        "legacy Sql txn wasm must exact true/true/1/true/true/1: {lines:?}"
    );
}

#[test]
fn hostfn_sql_begin_transaction_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlBeginTransaction");
    assert!(block.contains("llvm = \"real\""), "SqlBeginTransaction llvm=real: {block}");
    assert!(block.contains("status = \"partial\""), "SqlBeginTransaction status=partial: {block}");
    assert!(block.contains("wasm = \"real\""), "SqlBeginTransaction wasm=real (ADR-617): {block}");
    assert!(
        block.contains("ADR-617") && block.contains("ADR-599"),
        "SqlBeginTransaction notes ADR-617+599: {block}"
    );
}

#[test]
fn hostfn_sql_commit_transaction_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlCommitTransaction");
    assert!(block.contains("llvm = \"real\""), "SqlCommitTransaction llvm=real: {block}");
    assert!(block.contains("status = \"partial\""), "SqlCommitTransaction status=partial: {block}");
    assert!(block.contains("wasm = \"real\""), "SqlCommitTransaction wasm=real (ADR-617): {block}");
    assert!(
        block.contains("ADR-617") && block.contains("ADR-599"),
        "SqlCommitTransaction notes ADR-617+599: {block}"
    );
}

#[test]
fn hostfn_sql_rollback_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlRollback");
    assert!(block.contains("llvm = \"real\""), "SqlRollback llvm=real: {block}");
    assert!(block.contains("status = \"partial\""), "SqlRollback status=partial: {block}");
    assert!(block.contains("wasm = \"real\""), "SqlRollback wasm=real (ADR-617): {block}");
    assert!(
        block.contains("ADR-617") && block.contains("ADR-599"),
        "SqlRollback notes ADR-617+599: {block}"
    );
}
