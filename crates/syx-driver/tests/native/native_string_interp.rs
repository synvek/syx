//! Phase 101 Track B+C: native string interp exact wasm + locals v8 frame slab gates.

use std::fs;

use syx_driver::{
    compile_and_run_embedded, run_native_mir_file_printed, run_wasm_source_printed, DriverOptions,
};

#[path = "../support/native_aot.rs"]
mod native_aot;

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
    let Some((code, stdout)) = native_aot::try_run_native_strict(SRC, "string_interp") else {
        return;
    };
    assert_eq!(code, 0, "native stdout: {stdout}");
    assert!(stdout.contains(EXACT), "native stdout: {stdout}");
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
