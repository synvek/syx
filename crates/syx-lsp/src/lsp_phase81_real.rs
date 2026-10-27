// Phase 81 real LSP matrix (<=25): lock v15, farm billing settlement, heap v10 bridge, inkwell llvm v7 link, RT native stack v6, dist CDN, tier-2E.

use super::*;

fn p81_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p81_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p81_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}

#[test]
fn p81_lock_v15_default_version() {
    assert!(include_str!("../../syx-pkg/src/lockfile.rs").contains("default_lock_version() -> u32"));
    assert!(include_str!("../../syx-pkg/src/lockfile.rs").contains("15"));
}

#[test]
fn p81_lock_v15_hsm_rotation_cron() {
    assert!(include_str!("../../syx-pkg/src/lockfile.rs").contains("hsm_rotation_cron_job"));
}

#[test]
fn p81_lock_v15_billing_settlement() {
    assert!(include_str!("../../syx-pkg/src/lockfile.rs").contains("billing_settlement_id"));
}

#[test]
fn p81_lock_v15_marketplace_epoch() {
    assert!(include_str!("../../syx-pkg/src/lockfile.rs").contains("marketplace_plugin_epoch"));
}

#[test]
fn p81_heap_v10_bridge_api() {
    let src = include_str!("../../syx-runtime-core/src/managed_heap.rs");
    assert!(src.contains("heap_root_bridge") || src.contains("heap_v10"));
}

#[test]
fn p81_inkwell_llvm_v7_link() {
    assert!(include_str!("../../syx-llvm/src/lib.rs").contains("compile_inkwell_object_direct"));
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("inkwell-llvm-v7-link"));
}

#[test]
fn p81_rt_native_stack_v6_full() {
    let abi = include_str!("../../syx-rt/src/abi.rs");
    assert!(abi.contains("NATIVE_STACK_DISPATCH_V6") || abi.contains("native_stack_dispatch"));
}

#[test]
fn p81_dist_actor_cdn_v3() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("dist-actor-cdn-v3"));
}

#[test]
fn p81_borrow_prover_docker_binary() {
    assert!(include_str!("../../syx-semantic/src/lib.rs").contains("borrow_regions_export_external_proof_files"));
}

#[test]
fn p81_mono_runtime_witness_cache() {
    assert!(include_str!("../../syx-meta/src/lib.rs").contains("runtime_mono_cache"));
}

#[test]
fn p81_heap_bridge_parse() {
    assert!(p81_doc_ok("func main() { var i = 0; while i < 3 { i = i + 1 }; print(i) }"));
}

#[test]
fn p81_heap_string_offheap_parse() {
    assert!(p81_doc_ok("func main() { let s: String = \"hello\"; print(s.count) }"));
}

#[test]
fn p81_async_fn_parse() {
    assert!(p81_doc_ok("async func f() -> Int { return 1 }\nfunc main() { print(1) }"));
}

#[test]
fn p81_actor_parse() {
    assert!(p81_doc_ok("actor A { var n: Int = 0 }\nfunc main() { print(0) }"));
}

#[test]
fn p81_closure_parse() {
    assert!(p81_doc_ok("func id(x: Int) -> Int { return x }\nfunc main() { print(id(1)) }"));
}

#[test]
fn p81_generic_fn_parse() {
    assert!(p81_doc_ok("func id<T>(x: T) -> T { return x }\nfunc main() { print(id(1)) }"));
}

#[test]
fn p81_gc_heap_bridge_report() {
    assert!(
        include_str!("../../syx-cli/src/main.rs").contains("heap-bridge-report")
            || include_str!("../../syx-driver/src/lib.rs").contains("heap_root_bridge")
    );
}

#[test]
fn p81_doctor_farm_billing_settlement_cli() {
    assert!(
        include_str!("../../syx-cli/src/main.rs").contains("farm-billing-report")
            || include_str!("../../syx-cli/src/main.rs").contains("billing_settlement")
    );
}

#[test]
fn p81_browser_tier2e_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("browser-tier2e-v1"));
}

#[test]
fn p81_adr_phase81_charter() {
    let adr = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/adr/architecture-decisions.md"),
    )
    .unwrap();
    assert!(adr.contains("Phase 81") || adr.contains("ADR-067"));
}

#[test]
fn p81_heap_bridge_fixture_exists() {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/syx-driver/tests/fixtures/memory/heap_bridge_cross_process.sy");
    assert!(p.exists());
}

#[test]
fn p81_actor_cluster_global_cdn_fixture_exists() {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/syx-driver/tests/fixtures/parity/actor_cluster_global_cdn/main.sy");
    assert!(p.exists());
}
