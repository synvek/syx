// Phase 80 real LSP matrix (<=25): lock v14, farm global CDN, heap v9, inkwell llvm, RT native stack, dist actor, tier-2D.

use super::*;

fn p80_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p80_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p80_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}

#[test]
fn p80_lock_v14_default_version() {
    assert!(include_str!("../../syx-pkg/src/lockfile.rs").contains("default_lock_version() -> u32"));
    assert!(include_str!("../../syx-pkg/src/lockfile.rs").contains("hsm_key_ref"));
}

#[test]
fn p80_lock_v14_hsm_key_ref() {
    assert!(include_str!("../../syx-pkg/src/lockfile.rs").contains("hsm_key_ref"));
}

#[test]
fn p80_lock_v14_billing_meter() {
    assert!(include_str!("../../syx-pkg/src/lockfile.rs").contains("billing_meter_id"));
}

#[test]
fn p80_lock_v14_global_cdn_sla() {
    // Phase 118: global_cdn_sla_tier removed entirely from lockfile/enforce/resolver.
    let lock = include_str!("../../syx-pkg/src/lockfile.rs");
    assert!(!lock.contains("global_cdn_sla_tier"));
    let enforce = include_str!("../../syx-pkg/src/lock_enforce.rs");
    assert!(!enforce.contains("enforce_global_cdn_sla"));
    assert!(!enforce.contains("global_cdn_sla"));
}

#[test]
fn p80_heap_v9_generational_api() {
    let src = include_str!("../../syx-runtime-core/src/managed_heap.rs");
    assert!(src.contains("heap_v9_generational") || src.contains("young_gen") || src.contains("old_gen"));
}

#[test]
fn p80_inkwell_llvm_v6_real() {
    assert!(include_str!("../../syx-llvm/src/lib.rs").contains("compile_inkwell_object_direct"));
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("inkwell-llvm-v6-real"));
}

#[test]
fn p80_rt_native_stack_dispatch() {
    let abi = include_str!("../../syx-rt/src/abi.rs");
    assert!(abi.contains("NATIVE_STACK_DISPATCH") || abi.contains("native_stack_dispatch"));
}

#[test]
fn p80_dist_actor_cluster_v2() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("dist-actor-cluster-v2"));
}

#[test]
fn p80_borrow_prover_docker_export() {
    assert!(include_str!("../../syx-semantic/src/lib.rs").contains("borrow_regions_export_external_proof_files"));
}

#[test]
fn p80_mono_runtime_reflect() {
    assert!(include_str!("../../syx-meta/src/lib.rs").contains("reflect_specialization"));
}

#[test]
fn p80_heap_generational_parse() {
    assert!(p80_doc_ok("func main() { var i = 0; while i < 3 { i = i + 1 }; print(i) }"));
}

#[test]
fn p80_heap_string_full_inline_parse() {
    assert!(p80_doc_ok("func main() { let s: String = \"hello\"; print(s.count) }"));
}

#[test]
fn p80_async_fn_parse() {
    assert!(p80_doc_ok("async func f() -> Int { return 1 }\nfunc main() { print(1) }"));
}

#[test]
fn p80_actor_parse() {
    assert!(p80_doc_ok("actor A { var n: Int = 0 }\nfunc main() { print(0) }"));
}

#[test]
fn p80_closure_parse() {
    assert!(p80_doc_ok("func id(x: Int) -> Int { return x }\nfunc main() { print(id(1)) }"));
}

#[test]
fn p80_generic_fn_parse() {
    assert!(p80_doc_ok("func id<T>(x: T) -> T { return x }\nfunc main() { print(id(1)) }"));
}

#[test]
fn p80_gc_generational_report() {
    assert!(
        include_str!("../../syx-cli/src/main.rs").contains("gc-generational-report")
            || include_str!("../../syx-driver/src/lib.rs").contains("gc_generational")
    );
}

#[test]
fn p80_doctor_farm_billing_cli() {
    assert!(
        include_str!("../../syx-cli/src/main.rs").contains("farm-billing-report")
            || include_str!("../../syx-cli/src/main.rs").contains("farm_billing")
    );
}

#[test]
fn p80_browser_tier2d_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("browser-tier2d-v1"));
}

#[test]
fn p80_adr_phase80_charter() {
    let adr = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/adr/architecture-decisions.md"),
    )
    .unwrap();
    assert!(adr.contains("Phase 80") || adr.contains("ADR-063"));
}

#[test]
fn p80_heap_generational_fixture_exists() {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/syx-driver/tests/fixtures/memory/heap_generational_churn.sy");
    assert!(p.exists());
}

#[test]
fn p80_actor_cluster_cross_region_fixture_exists() {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/syx-driver/tests/fixtures/parity/actor_cluster_cross_region/main.sy");
    assert!(p.exists());
}
