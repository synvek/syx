//! Phase 91: Native LLVM CallHost stack + MIR closure executor stdout hard.

#[path = "../support/native_aot.rs"]
mod native_aot;

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run_native_strict(src: &str) -> Option<(i32, String)> {
    native_aot::try_run_native_strict(src, "llvm_stack")
}

#[test]
fn p98_native_llvm_stack_sort_by_strict_stdout() {
    let src = r#"func main() {
    let nums = [3, 1, 2]
    let sorted = __syx_array_sort_by(nums, { (a, b) in return a - b })
    print(sorted[0])
    print(sorted[2])
}"#;
    let Some((code, stdout)) = run_native_strict(src) else {
        return;
    };
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
