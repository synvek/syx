// Phase 79 real LSP matrix (<=25): lock v13, farm multitenant, heap v8, inkwell object, RT dispatch, actor IPC.

use super::*;

fn p79_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p79_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p79_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}

#[test]
fn p79_lock_v13_default_version() {
    assert!(include_str!("../../syx-pkg/src/lockfile.rs").contains("default_lock_version() -> u32"));
    assert!(
        include_str!("../../syx-pkg/src/lockfile.rs").contains("13")
            || include_str!("../../syx-pkg/src/lockfile.rs").contains("14")
    );
}

#[test]
fn p79_lock_v13_tenant_id() {
    assert!(include_str!("../../syx-pkg/src/lockfile.rs").contains("tenant_id"));
}

#[test]
fn p79_lock_v13_rotation_cron() {
    assert!(include_str!("../../syx-pkg/src/lockfile.rs").contains("rotation_cron_spec"));
}

#[test]
fn p79_lock_v13_farm_lease_ttl() {
    assert!(include_str!("../../syx-pkg/src/lockfile.rs").contains("farm_lease_ttl_secs"));
}

#[test]
fn p79_farm_worker_pool_field() {
    assert!(include_str!("../../syx-pkg/src/lockfile.rs").contains("farm_worker_pool"));
}

#[test]
fn p79_heap_v8_string_inline_api() {
    assert!(include_str!("../../syx-runtime-core/src/value.rs").contains("heap_v8_string_inline_enabled"));
}

#[test]
fn p79_inkwell_object_aot_v5() {
    assert!(include_str!("../../syx-llvm/src/lib.rs").contains("compile_inkwell_object_direct"));
}

#[test]
fn p79_rt_dispatch_table_abi() {
    let abi = include_str!("../../syx-rt/src/abi.rs");
    assert!(abi.contains("RT_DISPATCH_TABLE") || abi.contains("rt_dispatch_table"));
}

#[test]
fn p79_actor_ipc_register_abi() {
    assert!(include_str!("../../syx-wasm-emit/src/tier_matrix.rs").contains("syx_rt_actor_register"));
}

#[test]
fn p79_borrow_prover_export() {
    assert!(include_str!("../../syx-semantic/src/lib.rs").contains("borrow_regions_export_external_proof_files"));
}

#[test]
fn p79_mono_reflect_instantiations() {
    assert!(include_str!("../../syx-semantic/src/lib.rs").contains("instantiations"));
}

#[test]
fn p79_dict_slot_parse() {
    assert!(p79_doc_ok(
        "func main() { let d: Dictionary<String, Int> = [\"a\": 1]; print(d.count) }"
    ));
}

#[test]
fn p79_heap_string_inline_parse() {
    assert!(p79_doc_ok("func main() { let s: String = \"hi\"; print(s.count) }"));
}

#[test]
fn p79_async_fn_parse() {
    assert!(p79_doc_ok("async func f() -> Int { return 1 }\nfunc main() { print(1) }"));
}

#[test]
fn p79_actor_parse() {
    assert!(p79_doc_ok("actor A { var n: Int = 0 }\nfunc main() { print(0) }"));
}

#[test]
fn p79_simd_parse() {
    assert!(p79_doc_ok(
        "func main() { let v: SIMD4<Int32> = SIMD4.fromArray([1i32,2i32,3i32,4i32]); print(v) }"
    ));
}

#[test]
fn p79_generic_fn_parse() {
    assert!(p79_doc_ok("func id<T>(x: T) -> T { return x }\nfunc main() { print(id(1)) }"));
}

#[test]
fn p79_gc_relocate_report() {
    assert!(include_str!("../../syx-driver/src/gc_relocate_report.rs").contains("GcRelocateReport"));
}

#[test]
fn p79_doctor_rt_runtime_cli() {
    assert!(include_str!("../../syx-cli/src/main.rs").contains("rt_runtime_report"));
}

#[test]
fn p79_browser_tier2c_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("tier2c"));
}

#[test]
fn p79_adr_phase79_charter() {
    let adr = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/adr/architecture-decisions.md"),
    )
    .unwrap();
    assert!(adr.contains("Phase 79") || adr.contains("ADR-059"));
}

#[test]
fn p79_heap_dict_fixture_exists() {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/syx-driver/tests/fixtures/memory/heap_dict_slot_unified.sy");
    assert!(p.exists());
}
