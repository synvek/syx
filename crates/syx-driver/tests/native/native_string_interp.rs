//! Phase 101 Track B+C: native string interp exact wasm + locals v8 frame slab gates.

use std::fs;
use std::process::Command;

use syx_driver::{
    build_native_executable, compile_and_run_embedded, run_native_mir_file_printed,
    run_wasm_source_printed, DriverOptions,
};

fn opts() -> DriverOptions {
    DriverOptions::default()
}

const SRC: &str = r#"func main() {
    let name = "SYX"
    print("Hello, \(name)!")
}
"#;

const EXACT: &str = "Hello, SYX!";

#[test]
fn p101_native_string_interp_fingerprint() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("wasm-concat-exact-v1:hello-syx-quad"));
    assert!(lib.contains("native-llvm-locals-v10:no-tls-slab"));
    assert!(lib.contains("native-llvm-locals-v10-e2e:guidemo-line-parity"));
}

#[test]
fn p98_string_interp_vm_exact_stdout() {
    syx_host_plugin::register_echo_plugin_in_process();
    let out = compile_and_run_embedded(SRC, &opts())
        .unwrap()
        .printed
        .join("\n");
    assert!(out.contains(EXACT), "vm: {out}");
}

#[test]
fn p98_string_interp_native_exact_stdout() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_p98_str_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), SRC).unwrap();
    fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
    let exe = dir.join("app");
    build_native_executable(&dir, &opts(), &exe).unwrap();
    let output = Command::new(&exe).output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(EXACT), "native stdout: {stdout}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn p98_string_interp_mir_wasm_quad() {
    syx_host_plugin::register_echo_plugin_in_process();
    let dir = std::env::temp_dir().join(format!("syx_p98_str_q_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), SRC).unwrap();
    fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
    let path = dir.join("main.sy");
    let (_, mir) = run_native_mir_file_printed(&path, &opts()).unwrap();
    assert!(mir.iter().any(|l| l.contains(EXACT)), "mir: {mir:?}");
    let (_, wasm) = run_wasm_source_printed(&path, "main", &opts()).unwrap();
    assert!(
        wasm.iter().any(|l| l == EXACT),
        "wasm must print exact {EXACT:?}, got: {wasm:?}"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn p103_native_llvm_locals_v9_fingerprint() {
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    assert!(
        abi.contains("syx_rt_frame_local_ptr")
            && abi.contains("syx_rt_pop_into_local_ptr")
            && !abi.contains("fn local_bits_insert"),
        "locals v9 must use frame Value pointers, not TLS bit indices"
    );
    assert!(include_str!("../../src/lib.rs").contains("native-llvm-locals-v10:no-tls-slab"));
    assert!(
        include_str!("../../src/lib.rs").contains("native-llvm-locals-v10-e2e:guidemo-line-parity")
    );
    assert!(include_str!("../../src/lib.rs")
        .contains("wasm-closure-guest-v1:no-mir-exec-closure-fnindex"));
}

#[test]
fn p98_native_llvm_locals_abi_surface() {
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    assert!(abi.contains("syx_rt_load_local") && abi.contains("syx_rt_store_local"));
}
