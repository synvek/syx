//! Phase 91: Native LLVM full GA — sync main + CallHost stack + MIR closure executor.

use std::fs;
use std::process::Command;
use syx_driver::{build_native_executable, compile_and_run_embedded, DriverOptions};

#[test]
fn p98_native_llvm_full_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("native-llvm-locals-v5:alloca-ssa-value-ga"));
}

#[test]
fn p98_native_llvm_full_mir_closure_executor() {
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    assert!(abi.contains("execute_native_mir_closure"));
    assert!(abi.contains("set_mir_closure_preferred"));
}

#[test]
fn p98_native_llvm_full_sort_by_vm_stdout() {
    syx_host_plugin::register_echo_plugin_in_process();
    let out = compile_and_run_embedded(
        r#"func main() {
    let s = __syx_array_sort_by([3, 1, 2], { (a: Int, b: Int) -> Int in return a - b })
    print(s[0])
    print(s[2])
}"#,
        &DriverOptions::default(),
    )
    .unwrap()
    .printed;
    assert!(out.iter().any(|l| l.contains("1")), "{out:?}");
}

#[test]
fn p98_native_llvm_full_sort_by_mir_stdout() {
    syx_host_plugin::register_echo_plugin_in_process();
    let dir = std::env::temp_dir().join(format!("syx_p98_native_mir_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join("main.sy"),
        r#"func main() {
    let s = __syx_array_sort_by([3, 1, 2], { (a: Int, b: Int) -> Int in return a - b })
    print(s[0])
    print(s[2])
}"#,
    )
    .unwrap();
    let (_, lines) =
        syx_driver::run_native_mir_file_printed(&dir.join("main.sy"), &DriverOptions::default())
            .unwrap();
    let _ = fs::remove_dir_all(&dir);
    assert!(lines.iter().any(|l| l.contains("1")), "{lines:?}");
}

#[test]
fn p98_native_llvm_full_strict_native_sort() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!("syx_p98_native_strict_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join("main.sy"),
        r#"func main() {
    let s = __syx_array_sort_by([3, 1, 2], { (a: Int, b: Int) -> Int in return a - b })
    print(s[0])
}"#,
    )
    .unwrap();
    fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe).unwrap();
    let output = Command::new(&exe).output().unwrap();
    let _ = fs::remove_dir_all(&dir);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains('1'), "{stdout}");
}
