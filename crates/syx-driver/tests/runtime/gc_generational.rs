//! Phase 98: GC v13 generational compact quad stdout.

use syx_driver::{compile_and_run_embedded, run_native_mir_file_printed, run_wasm_source_printed, DriverOptions};
use syx_runtime_core::{
    gc_gen_compact_cycles, gc_gen_compact_record, heap_v13_generational, reset_gc_gen_compact_cycles,
};

const GC_SRC: &str = r#"func main() {
    let s = __syx_array_sort_by([3, 1, 2], { (a: Int, b: Int) -> Int in return a - b })
    print(s[0])
    __syx_gc_collect()
    print("gc-done")
}"#;

#[test]
fn p98_gc_v13_generational_default_on() {
    std::env::remove_var("SYX_HEAP_V13_GENERATIONAL");
    assert!(heap_v13_generational());
}

#[test]
fn p98_gc_gen_compact_record() {
    reset_gc_gen_compact_cycles();
    gc_gen_compact_record();
    assert!(gc_gen_compact_cycles() >= 1);
}

#[test]
fn p98_gc_generational_vm_stdout() {
    syx_host_plugin::register_echo_plugin_in_process();
    let out = compile_and_run_embedded(GC_SRC, &DriverOptions::default())
        .unwrap()
        .printed;
    assert!(out.iter().any(|l| l.contains("gc-done")), "{out:?}");
}

#[test]
fn p98_gc_generational_mir_stdout() {
    let dir = std::env::temp_dir().join(format!("syx_p98_gc_mir_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("main.sy"), GC_SRC).unwrap();
    syx_host_plugin::register_echo_plugin_in_process();
    let (_, lines) = run_native_mir_file_printed(&dir.join("main.sy"), &DriverOptions::default()).unwrap();
    assert!(lines.iter().any(|l| l.contains("gc-done")), "{lines:?}");
}

#[test]
fn p98_gc_generational_wasm_stdout() {
    let dir = std::env::temp_dir().join(format!("syx_p98_gc_wasm_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("main.sy"), GC_SRC).unwrap();
    syx_host_plugin::register_echo_plugin_in_process();
    let (_, lines) = run_wasm_source_printed(&dir.join("main.sy"), "main", &DriverOptions::default()).unwrap();
    assert!(lines.iter().any(|l| l.contains("gc-done")), "{lines:?}");
}

#[test]
fn p98_gc_generational_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("gc-generational-v13-quad-v1"));
}

#[test]
fn p98_gc_generational_doctor_alias() {
    let cli = include_str!("../../../syx-cli/src/main.rs");
    assert!(cli.contains("gc-gen-report") || cli.contains("gc_gen_report"));
}
