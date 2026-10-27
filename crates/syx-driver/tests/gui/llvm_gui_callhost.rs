//! Phase 98: GUI CallHost MIR → LLVM IR → clang link + run exit 0 (hello.sy pivot).

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use syx_driver::{build_native_executable, emit_llvm_text, DriverOptions};

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn hello_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/hello.sy")
}

#[test]
fn p98_llvm_gui_callhost_hello_ir_clang() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = hello_path();
    let ir = emit_llvm_text(&path, &opts()).unwrap();
    assert!(!ir.is_empty(), "hello.sy LLVM IR must be non-empty");
    let dir = std::env::temp_dir().join(format!("syx_p98_gui_clang_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let ir_path = dir.join("out.ll");
    fs::write(&ir_path, &ir).unwrap();
    let obj_path = dir.join("out.o");
    let output = Command::new("clang")
        .args(["-c", "-o"])
        .arg(&obj_path)
        .arg(&ir_path)
        .output()
        .expect("clang must be available");
    assert!(
        output.status.success(),
        "clang -c failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn p98_llvm_gui_callhost_hello_link_run_exit_zero() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = hello_path();
    let dir = std::env::temp_dir().join(format!("syx_p98_gui_run_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::copy(&path, dir.join("hello.sy")).unwrap();
    fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"hello.sy\"\n").unwrap();
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let exe = dir.join("app");
    build_native_executable(&dir, &opts(), &exe).expect("native link");
    let output = Command::new(&exe).output().unwrap();
    assert!(
        output.status.success(),
        "native run failed: {:?}",
        output.status
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "Hello, World\n",
        "hello.sy native stdout must exact"
    );
    let _ = fs::remove_dir_all(&dir);
}
