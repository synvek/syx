//! Phase 91: Native LLVM CallHost stack + MIR closure executor stdout hard.

use std::fs;
use std::process::Command;

use syx_driver::{build_native_executable, compile_and_run_embedded, DriverOptions};

fn run_native_strict(src: &str) -> (i32, String) {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_p98_stack_{}",
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
    build_native_executable(&dir, &DriverOptions::default(), &exe).unwrap();
    let output = Command::new(&exe).output().unwrap();
    (
        output.status.code().unwrap_or(1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
    )
}

#[test]
fn p98_native_llvm_stack_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("native-llvm-stack-v1"));
}

#[test]
fn p98_native_llvm_stack_mir_closure_executor() {
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    assert!(abi.contains("execute_native_mir_closure"));
    assert!(abi.contains("set_mir_closure_executor"));
}

#[test]
fn p98_native_llvm_stack_sort_by_strict_stdout() {
    let src = r#"func main() {
    let nums = [3, 1, 2]
    let sorted = __syx_array_sort_by(nums, { (a, b) in return a - b })
    print(sorted[0])
    print(sorted[2])
}"#;
    let (code, stdout) = run_native_strict(src);
    assert_eq!(code, 0, "stdout: {stdout}");
    assert!(stdout.contains('1'), "stdout: {stdout}");
    assert!(stdout.contains('3'), "stdout: {stdout}");
}

#[test]
fn p98_native_llvm_stack_vm_parity() {
    syx_host_plugin::register_echo_plugin_in_process();
    let src = r#"func main() {
    let nums = [3, 1, 2]
    let sorted = __syx_array_sort_by(nums, { (a, b) in return a - b })
    print(sorted[0])
}"#;
    let out = compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n");
    assert!(out.contains('1'), "{out}");
}
