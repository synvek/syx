//! ADR-216 / M2 W72: EC-M2-4 Sql native/WASM GA third deepening (SqlExecOn llvm=real).
//! Wave A W272 / ADR-616: Sql core HostFn wasm8 (Open/Close/Exec/Query/Prepare/Bind+ExecOn/QueryOn).
//! Wave A W273 / ADR-617: Sql residual HostFn wasm9 收官 (PrepareOn/Bind*On+txn×3/+On×3).
//! Wave A W292 / ADR-636: data.sqlite product native=real (status still partial).
// was: ec_m2_4_sql_native_wasm_ga3

use std::fs;
use std::process::Command;

use syx_driver::{build_native_executable, compile_and_run_embedded, DriverOptions};
use syx_vm::reset_rusqlite_store;

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
    fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
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
