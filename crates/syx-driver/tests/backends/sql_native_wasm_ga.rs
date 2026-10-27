//! ADR-216 / M2 W72: EC-M2-4 Sql native/WASM GA third deepening (SqlExecOn llvm=real).
//! Wave A W272 / ADR-616: Sql core HostFn wasm8 (Open/Close/Exec/Query/Prepare/Bind+ExecOn/QueryOn).
//! Wave A W273 / ADR-617: Sql residual HostFn wasm9 收官 (PrepareOn/Bind*On+txn×3/+On×3).
//! Wave A W292 / ADR-636: data.sqlite product native=real (status still partial).
// was: ec_m2_4_sql_native_wasm_ga3

use std::fs;
use std::process::Command;

use syx_driver::{build_native_executable, compile_and_run_embedded, DriverOptions};
use syx_vm::reset_rusqlite_store;

const W272_WASM_REAL: &[&str] = &[
    "SqlOpen",
    "SqlClose",
    "SqlExec",
    "SqlQuery",
    "SqlPrepare",
    "SqlBindStep",
    "SqlExecOn",
    "SqlQueryOn",
];

const W273_WASM_REAL: &[&str] = &[
    "SqlPrepareOn",
    "SqlBindStepOn",
    "SqlBindPreparedOn",
    "SqlBeginTransaction",
    "SqlCommitTransaction",
    "SqlRollback",
    "SqlBeginTransactionOn",
    "SqlCommitTransactionOn",
    "SqlRollbackTransactionOn",
];

fn host_block<'a>(toml: &'a str, name: &str) -> &'a str {
    let needle = format!("name = \"{name}\"");
    let rest = toml
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing host override {name}"));
    rest.split("[[host]]").next().unwrap_or(rest)
}

fn product_block<'a>(caps: &'a str, id: &str) -> &'a str {
    let needle = format!("id = \"{id}\"");
    let rest = caps
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing capability {id}"));
    rest.split("[[capability]]").next().unwrap_or(rest)
}

fn run_native_strict(src: &str) -> (i32, String) {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_m2_w72_sql_ga3_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), src).unwrap();
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .unwrap();
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native exe");
    (
        output.status.code().unwrap_or(1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
    )
}

#[test]
fn ec_m2_4_sql_native_wasm_ga3_sql_exec_on_columns() {
    // Wave A W72 / ADR-216 llvm + Wave A W272 / ADR-616 wasm=real
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "SqlExecOn");
    assert!(
        block.contains("vm = \"real\""),
        "SqlExecOn must keep vm=real: {block}"
    );
    assert!(
        block.contains("llvm = \"real\""),
        "SqlExecOn must declare llvm=real (ADR-216): {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "SqlExecOn must declare wasm=real (ADR-616): {block}"
    );
    assert!(
        block.contains("ADR-616"),
        "SqlExecOn notes must cite ADR-616: {block}"
    );
}

#[test]
fn ec_m2_4_sql_native_wasm_ga3_prior_ga_hosts_remain_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in ["SqlOpen", "SqlClose"] {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must keep llvm=real: {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-616): {block}"
        );
    }
}

#[test]
fn ec_m2_4_sql_hosts_wasm_real_w272() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in W272_WASM_REAL {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must keep llvm=real: {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-616): {block}"
        );
        assert!(
            block.contains("ADR-616"),
            "{name} notes must cite ADR-616: {block}"
        );
    }
}

#[test]
fn ec_m2_4_sql_hosts_wasm_real_w273() {
    // Wave A W273 / ADR-617 — Sql residual HostFn wasm 收官 (no remaining Sql host-assisted).
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in W273_WASM_REAL {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must keep llvm=real: {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-617): {block}"
        );
        assert!(
            block.contains("ADR-617"),
            "{name} notes must cite ADR-617: {block}"
        );
    }
}

#[test]
fn ec_m2_4_sql_native_wasm_ga3_product_columns() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "data.sqlite");
    assert!(
        block.contains("native = \"real\""),
        "data.sqlite must keep native=real (ADR-636): {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "data.sqlite must declare wasm=real (ADR-624): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "data.sqlite must keep status=partial (≠ implemented): {block}"
    );
    assert!(
        block.contains("ADR-636") && block.contains("W292"),
        "data.sqlite notes must cite ADR-636+W292: {block}"
    );
    assert!(
        block.contains("ec_m2_4_sql_native_wasm_ga3"),
        "data.sqlite tests must include ec_m2_4_sql_native_wasm_ga3: {block}"
    );
}

#[test]
fn ec_m2_4_sql_native_wasm_ga3_native_sql_exec_on_exec() {
    // SqlOpen/SqlClose already llvm=real; SqlExecOn is the GA3 HostFn under test.
    let src = r#"func main() {
    let c = __syx_sql_open(":memory:")
    __syx_sql_exec_on(c, "CREATE TABLE t (v TEXT)")
    let n = __syx_sql_exec_on(c, "INSERT INTO t VALUES ('x')")
    print(n)
    __syx_sql_close(c)
    return 0
}"#;
    let vm_out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("vm run")
        .printed
        .join("\n");
    let (code, native_out) = run_native_strict(src);
    assert_eq!(code, 0, "native exit nonzero; stdout={native_out}");
    let vm_line = vm_out.lines().next().unwrap_or("").trim();
    let native_line = native_out.lines().next().unwrap_or("").trim();
    assert_eq!(
        native_line, vm_line,
        "native SqlExecOn must match VM; vm={vm_line:?} native={native_line:?}"
    );
    assert_eq!(native_line, "1", "INSERT changed-row count must be 1: {native_line}");
}

#[test]
fn ec_m2_4_sql_native_wasm_ga_native_sql_open_exec() {
    // Wave A W149 / ADR-493 / A-03
    let src = r#"func main() {
    print(__syx_sql_open(":memory:"))
    return 0
}"#;
    reset_rusqlite_store();
    let vm_out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("vm run")
        .printed
        .join("\n");
    let (code, native_out) = run_native_strict(src);
    assert_eq!(code, 0, "native exit nonzero; stdout={native_out}");
    let vm_line = vm_out.lines().next().unwrap_or("").trim();
    let native_line = native_out.lines().next().unwrap_or("").trim();
    assert_eq!(
        native_line, vm_line,
        "native SqlOpen must match VM; vm={vm_line:?} native={native_line:?}"
    );
    assert_eq!(native_line, "1", "first conn id must be 1: {native_line}");
}

#[test]
fn ec_m2_4_sql_native_wasm_ga_native_sql_close_exec() {
    // Wave A W149 / ADR-493 / A-03
    let src = r#"func main() {
    let c = __syx_sql_open(":memory:")
    __syx_sql_close(c)
    print("ok")
    return 0
}"#;
    reset_rusqlite_store();
    let vm_out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("vm run")
        .printed
        .join("\n");
    let (code, native_out) = run_native_strict(src);
    assert_eq!(code, 0, "native exit nonzero; stdout={native_out}");
    let vm_line = vm_out.lines().next().unwrap_or("").trim();
    let native_line = native_out.lines().next().unwrap_or("").trim();
    assert_eq!(
        native_line, vm_line,
        "native SqlClose must match VM; vm={vm_line:?} native={native_line:?}"
    );
    assert_eq!(native_line, "ok", "close must succeed: {native_line}");
}

#[test]
fn ec_m2_4_sql_native_wasm_ga3_honesty_fences() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in ["OrmSave", "SqlMigrate", "SqlBindMulti"] {
        assert!(
            !toml.contains(&format!("name = \"{name}\"")),
            "{name} must remain uncovered (fail-closed default); found override"
        );
    }

    let stubs = include_str!("../../../../crates/syx-rt/src/llvm_stubs.rs");
    assert!(
        !stubs.contains("syx_rt_sql"),
        "llvm_stubs must not claim dedicated syx_rt_sql AOT (no Sql POP stub)"
    );
}

#[test]
fn ec_m2_4_sql_native_wasm_ga3_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("ec_m2_4_sql_native_wasm_ga3_tier"),
        "inventory must contain ec_m2_4_sql_native_wasm_ga3_tier: {inv}"
    );
    assert!(
        inv.contains("ADR-216") || inv.contains("signed-ADR-216"),
        "inventory must note ADR-216: {inv}"
    );
}

#[test]
fn ec_m2_4_sql_native_wasm_ga3_ci_wires() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test evergreen"),
        "ci.yml must wire ec_m2_4_sql_native_wasm_ga3"
    );
}
