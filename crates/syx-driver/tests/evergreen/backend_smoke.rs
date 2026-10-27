//! SYX 37.0 backend — LLVM-only, object ABI, native smoke.
// was: backend

use std::path::PathBuf;

use syx_driver::{
    build_native_executable_file, cranelift_backend_retired, emit_clif_module_text,
    run_file_printed, run_native_mir_file_printed, DriverOptions, NativeBackend,
};

fn fixture(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

#[test]
fn llvm_default_backend() {
    assert_eq!(DriverOptions::default().native_backend, NativeBackend::Llvm);
    assert_eq!(NativeBackend::parse("llvm").unwrap(), NativeBackend::Llvm);
}

#[test]
fn cranelift_rejected() {
    assert!(cranelift_backend_retired());
    assert!(NativeBackend::parse("cranelift").is_err());
    assert!(NativeBackend::parse("cf").is_err());
}

#[test]
fn config_llvm_only() {
    assert_eq!(
        syx_config::parse_backend("llvm"),
        Some(syx_config::NativeBackendChoice::Llvm)
    );
    assert!(syx_config::parse_backend("cranelift").is_none());
}

#[test]
fn object_drop_rt_abi() {
    let src = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../syx-rt/src/abi.rs"),
    )
    .unwrap();
    // Wave A W534 / ADR-878: handle C ABI deleted; table API remains in runtime-core.
    assert!(!src.contains("pub extern \"C\" fn syx_rt_object_drop"));
    assert!(!src.contains("pub extern \"C\" fn syx_rt_object_retain"));
    assert!(!src.contains("pub extern \"C\" fn syx_rt_object_release"));
}

#[test]
fn emit_clif_retired_stub() {
    let mir = syx_mir::MirModule { functions: vec![],
 ..Default::default()
};
    let text = emit_clif_module_text(&mir);
    assert!(text.contains("retired"));
}

#[test]
fn seccomp_subprocess_default() {
    assert!(!syx_macro::macro_in_process_enabled());
}

#[test]
fn wasm_plugin_proxy() {
    assert!(syx_wasm_run::wasm_guest_plugin_proxy_enabled());
}

#[test]
fn mono_witness_matrix() {
    let preds = syx_mono::witness_constraint_matrix(
        &[smol_str::SmolStr::new("T")],
        &["Codable"],
    );
    assert_eq!(preds.len(), 1);
}

#[test]
fn vm_gc_object_ref() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture("gc/gc_object_ref.sy");
    let options = DriverOptions::default();
    syx_driver::compile_file_with_options(&path, &options).expect("compile");
    let (code, lines) = run_file_printed(&path, &options).expect("run");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "dropped"));
}

#[test]
fn native_mir_assign_host() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture("operators/assign_host.sy");
    let (code, lines) =
        run_native_mir_file_printed(&path, &DriverOptions::default()).expect("native mir");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "15"));
}

#[test]
fn native_llvm_hof_smoke() {
    let path = fixture("parity/hof.sy");
    let source = std::fs::read_to_string(&path).unwrap();
    let dir = std::env::temp_dir().join(format!("syx_native_hof_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("main.sy"), source).unwrap();
    std::fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .unwrap();
    let exe = dir.join("app");
    build_native_executable_file(&dir.join("main.sy"), &DriverOptions::default(), &exe)
        .expect("build");
    let output = std::process::Command::new(&exe).output().unwrap();
    assert_eq!(output.status.code().unwrap_or(1), 0);
}
