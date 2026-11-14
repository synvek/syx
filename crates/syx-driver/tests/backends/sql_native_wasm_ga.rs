//! ADR-216 / M2 W72: EC-M2-4 Sql native/WASM GA third deepening (SqlExecOn llvm=real).
//! Wave A W272 / ADR-616: Sql core HostFn wasm8 (Open/Close/Exec/Query/Prepare/Bind+ExecOn/QueryOn).
//! Wave A W273 / ADR-617: Sql residual HostFn wasm9 收官 (PrepareOn/Bind*On+txn×3/+On×3).
//! Wave A W292 / ADR-636: data.sqlite product native=real (status still partial).
// was: ec_m2_4_sql_native_wasm_ga3


use syx_driver::{compile_and_run_embedded, DriverOptions};

use crate::native_strict::run_native_strict_or_mirror_vm;
use syx_vm::reset_rusqlite_store;

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
    let (code, native_out) = run_native_strict_or_mirror_vm(src, &vm_out, "sql_ga");
    assert_eq!(code, 0, "native exit nonzero; stdout={native_out}");
    let vm_line = vm_out.lines().next().unwrap_or("").trim();
    let native_line = native_out.lines().next().unwrap_or("").trim();
    assert_eq!(
        native_line, vm_line,
        "native SqlExecOn must match VM; vm={vm_line:?} native={native_line:?}"
    );
    assert_eq!(
        native_line, "1",
        "INSERT changed-row count must be 1: {native_line}"
    );
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
    let (code, native_out) = run_native_strict_or_mirror_vm(src, &vm_out, "sql_ga");
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
    let (code, native_out) = run_native_strict_or_mirror_vm(src, &vm_out, "sql_ga");
    assert_eq!(code, 0, "native exit nonzero; stdout={native_out}");
    let vm_line = vm_out.lines().next().unwrap_or("").trim();
    let native_line = native_out.lines().next().unwrap_or("").trim();
    assert_eq!(
        native_line, vm_line,
        "native SqlClose must match VM; vm={vm_line:?} native={native_line:?}"
    );
    assert_eq!(native_line, "ok", "close must succeed: {native_line}");
}
