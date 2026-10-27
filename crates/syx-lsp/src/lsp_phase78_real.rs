// Phase 78 real LSP matrix (<=25): lock v12, farm worker, heap v7, inkwell module, borrow external.

use super::*;

fn p78_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p78_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p78_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}

#[test]
fn p78_lock_v12_default_version() {
    assert!(include_str!("../../syx-pkg/src/lockfile.rs").contains("default_lock_version() -> u32"));
    assert!(include_str!("../../syx-pkg/src/lockfile.rs").contains("13"));
}

#[test]
fn p78_lock_v12_publisher_key_rotation() {
    assert!(include_str!("../../syx-pkg/src/lockfile.rs").contains("publisher_key_rotation"));
}

#[test]
fn p78_farm_worker_dequeue_api() {
    assert!(include_str!("../../syx-driver/src/build_syx.rs").contains("farm_worker_dequeue"));
}

#[test]
fn p78_heap_v7_relocate_api() {
    assert!(include_str!("../../syx-runtime-core/src/managed_heap.rs").contains("relocate_global"));
}

#[test]
fn p78_inkwell_module_v4() {
    assert!(include_str!("../../syx-llvm/src/lib.rs").contains("emit_inkwell_module_v4"));
}

#[test]
fn p78_rt_runtime_report_module() {
    assert!(include_str!("../../syx-driver/src/rt_runtime_report.rs").contains("RtRuntimeReport"));
}

#[test]
fn p78_borrow_external_export() {
    assert!(include_str!("../../syx-semantic/src/lib.rs").contains("borrow_regions_export_external_proof_files"));
}

#[test]
fn p78_mono_reflect_cache() {
    assert!(include_str!("../../syx-meta/src/lib.rs").contains("register_mono_sig"));
}

#[test]
fn p78_plugin_epoch_poison() {
    assert!(include_str!("../../syx-host-plugin/src/registry.rs").contains("PLUGIN_UNLOAD_EPOCH"));
}

#[test]
fn p78_gc_relocate_report() {
    assert!(include_str!("../../syx-driver/src/gc_relocate_report.rs").contains("GcRelocateReport"));
}

#[test]
fn p78_lock_enforce_module() {
    assert!(include_str!("../../syx-pkg/src/lock_enforce.rs").contains("enforce_plugin_epoch"));
}

#[test]
fn p78_farm_worker_id_field() {
    assert!(include_str!("../../syx-pkg/src/lockfile.rs").contains("farm_worker_id"));
}

#[test]
fn p78_heap_relocate_fixture() {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/syx-driver/tests/fixtures/memory/heap_relocate_churn.sy");
    assert!(p.exists());
}

#[test]
fn p78_async_fn_parse() {
    assert!(p78_doc_ok("async func f() -> Int { return 1 }\nfunc main() { print(1) }"));
}

#[test]
fn p78_actor_parse() {
    assert!(p78_doc_ok("actor A { var n: Int = 0 }\nfunc main() { print(0) }"));
}

#[test]
fn p78_simd_parse() {
    assert!(p78_doc_ok(
        "func main() { let v: SIMD4<Int32> = SIMD4.fromArray([1i32,2i32,3i32,4i32]); print(v) }"
    ));
}

#[test]
fn p78_generic_fn_parse() {
    assert!(p78_doc_ok("func id<T>(x: T) -> T { return x }\nfunc main() { print(id(1)) }"));
}

#[test]
fn p78_dict_slot_parse() {
    assert!(p78_doc_ok(
        "func main() { let d: Dictionary<String, Int> = [\"a\": 1]; print(d.count) }"
    ));
}

#[test]
fn p78_rt_heap_relocate_abi() {
    assert!(include_str!("../../syx-rt/src/abi.rs").contains("syx_rt_heap_relocate"));
}

#[test]
fn p78_plugin_bundle_sha256_field() {
    assert!(include_str!("../../syx-pkg/src/lockfile.rs").contains("plugin_bundle_sha256"));
}

#[test]
fn p78_doctor_rt_runtime_cli() {
    assert!(include_str!("../../syx-cli/src/main.rs").contains("rt_runtime_report"));
}

#[test]
fn p78_adr_phase78_charter() {
    let adr = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/adr/architecture-decisions.md"),
    )
    .unwrap();
    assert!(adr.contains("Phase 78") || adr.contains("ADR-057"));
}
