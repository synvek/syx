//! Phase 91: Native LLVM full GA — sync main + CallHost stack + MIR closure executor.

use std::fs;
use syx_driver::{compile_and_run_embedded, DriverOptions};

#[path = "../support/native_aot.rs"]
mod native_aot;

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
    let src = r#"func main() {
    let s = __syx_array_sort_by([3, 1, 2], { (a: Int, b: Int) -> Int in return a - b })
    print(s[0])
}"#;
    let Some((code, stdout)) = native_aot::try_run_native_strict(src, "llvm_full") else {
        return;
    };
    assert_eq!(code, 0, "{stdout}");
    assert!(stdout.contains('1'), "{stdout}");
}
