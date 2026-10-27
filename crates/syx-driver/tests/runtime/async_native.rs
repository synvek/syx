//! Phase 98: async nested await strict stdout order gate.

use std::fs;
use std::process::Command;

use syx_driver::{build_native_executable, compile_and_run_embedded, DriverOptions};

const ASYNC_NESTED_SRC: &str = r#"async func inner() {
    print("inner-step")
}

async func outer() {
    print("outer-step")
    await inner()
}

async func main() {
    print("main-start")
    await outer()
    print("main-done")
}"#;

fn run_vm(src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
}

fn run_native_strict(src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!("syx_p98_async_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), src).unwrap();
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .unwrap();
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe).unwrap();
    let output = Command::new(&exe).output().unwrap();
    let _ = fs::remove_dir_all(&dir);
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect()
}

#[test]
fn p98_async_native_resume_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("native-async-resume-v4:nested-await-stdout-order"));
}

#[test]
fn p98_async_native_vm_nested_stdout_order() {
    let lines = run_vm(ASYNC_NESTED_SRC);
    let start = lines.iter().position(|l| l.contains("main-start")).expect("start");
    let outer = lines.iter().position(|l| l.contains("outer-step")).expect("outer");
    let inner = lines.iter().position(|l| l.contains("inner-step")).expect("inner");
    let done = lines.iter().position(|l| l.contains("main-done")).expect("done");
    assert!(start < outer && outer < inner && inner < done, "{lines:?}");
}

#[test]
fn p98_async_native_strict_nested_stdout_order() {
    let lines = run_native_strict(ASYNC_NESTED_SRC);
    let start = lines.iter().position(|l| l.contains("main-start")).expect("start");
    let outer = lines.iter().position(|l| l.contains("outer-step")).expect("outer");
    let inner = lines.iter().position(|l| l.contains("inner-step")).expect("inner");
    let done = lines.iter().position(|l| l.contains("main-done")).expect("done");
    assert!(start < outer && outer < inner && inner < done, "{lines:?}");
}
