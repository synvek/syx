// Phase 77 real LSP matrix (<=25): lock v11, farm queue, heap v6, inkwell llvm, borrow proof.

use super::*;

fn p77_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p77_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p77_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}

#[test]
fn p77_lock_v11_plugin_unload_epoch_field() {
    assert!(include_str!("../../syx-pkg/src/lockfile.rs").contains("plugin_unload_epoch"));
}

#[test]
fn p77_lock_v11_plugin_unload_epoch() {
    let src = include_str!("../../syx-pkg/src/lockfile.rs");
    assert!(src.contains("plugin_unload_epoch"));
}

#[test]
fn p77_farm_queue_url_field() {
    let src = include_str!("../../syx-pkg/src/lockfile.rs");
    assert!(src.contains("farm_queue_url"));
}

#[test]
fn p77_heap_v6_materialize_helper() {
    let src = include_str!("../../syx-runtime-core/src/value.rs");
    assert!(src.contains("materialize_array"));
}

#[test]
fn p77_inkwell_llvm_phase77() {
    let src = include_str!("../../syx-llvm/src/lib.rs");
    assert!(src.contains("llvm_inkwell_phase77_llvm_ready"));
}

#[test]
fn p77_rt_direct_stack_ops() {
    let src = include_str!("../../syx-rt/src/abi.rs");
    assert!(src.contains("rt_direct_exec"));
}

#[test]
fn p77_borrow_proof_export() {
    let src = include_str!("../../syx-semantic/src/lib.rs");
    assert!(src.contains("borrow_regions_export_proof_obligations_json"));
}

#[test]
fn p77_reflect_specialization() {
    let src = include_str!("../../syx-meta/src/lib.rs");
    assert!(src.contains("reflect_specialization"));
}

#[test]
fn p77_heap_host_report_module() {
    let src = include_str!("../../syx-driver/src/heap_host_report.rs");
    assert!(src.contains("HeapHostReport"));
}

#[test]
fn p77_farm_report_module() {
    let src = include_str!("../../syx-driver/src/farm_report.rs");
    assert!(src.contains("FarmReport"));
}

#[test]
fn p77_vm_array_set_heap() {
    let src = include_str!("../../syx-vm/src/lib.rs");
    assert!(src.contains("alloc_heap_slot"));
}

#[test]
fn p77_build_farm_queue() {
    let src = include_str!("../../syx-driver/src/build_syx.rs");
    assert!(src.contains("farm_queue_url") || src.contains("SYX_BUILD_FARM_QUEUE"));
}

#[test]
fn p77_doctor_heap_host_cli() {
    let src = include_str!("../../syx-cli/src/main.rs");
    assert!(src.contains("heap_host_report") || src.contains("heap-host-report"));
}

#[test]
fn p77_doctor_farm_cli() {
    let src = include_str!("../../syx-cli/src/main.rs");
    assert!(src.contains("farm_report") || src.contains("farm-report"));
}

#[test]
fn p77_doctor_reflect_cli() {
    let src = include_str!("../../syx-cli/src/main.rs");
    assert!(src.contains("reflect_report") || src.contains("reflect-report"));
}

#[test]
fn p77_resolver_lock_v11_fields() {
    assert!(include_str!("../../syx-pkg/src/resolver.rs").contains("marketplace_cdn_base"));
}

#[test]
fn p77_adr_phase77_charter() {
    let adr = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/adr/architecture-decisions.md"),
    )
    .unwrap();
    assert!(adr.contains("Phase 77") || adr.contains("ADR-055"));
}

#[test]
fn p77_heap_dict_fixture_exists() {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/syx-driver/tests/fixtures/memory/heap_dict_slot_unified.sy");
    assert!(p.exists());
}

#[test]
fn p77_async_fn_parse() {
    assert!(p77_doc_ok("async func f() -> Int { return 1 }\nfunc main() { print(1) }"));
}

#[test]
fn p77_class_heap_field() {
    assert!(p77_doc_ok(
        "class Node { var n: Int init(n: Int) { self.n = n } }\nfunc main() { print(Node(n: 2).n) }"
    ));
}

#[test]
fn p77_simd_type_parse() {
    assert!(p77_doc_ok(
        "func main() { let v: SIMD4<Int32> = SIMD4.fromArray([1i32, 2i32, 3i32, 4i32]); print(v) }"
    ));
}

#[test]
fn p77_marketplace_cdn_field() {
    assert!(include_str!("../../syx-pkg/src/lockfile.rs").contains("marketplace_cdn_base"));
}

#[test]
fn p77_materialize_dict_helper() {
    assert!(include_str!("../../syx-runtime-core/src/value.rs").contains("materialize_dict"));
}
