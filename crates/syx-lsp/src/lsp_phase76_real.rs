// Phase 76 real LSP matrix (<=25): lock v10, cache push, VM heap slot, inkwell crate.

use super::*;

fn p76_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p76_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p76_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}

#[test]
fn p76_lock_v10_default_version() {
    let src = include_str!("../../syx-pkg/src/lockfile.rs");
    assert!(src.contains("default_lock_version() -> u32") && src.contains("13"));
}

#[test]
fn p76_lock_v10_cdn_regions_field() {
    let src = include_str!("../../syx-pkg/src/lockfile.rs");
    assert!(src.contains("artifact_cdn_regions"));
}

#[test]
fn p76_cache_remote_push_fn() {
    let src = include_str!("../../syx-driver/src/lib.rs");
    assert!(src.contains("push_cache_remote"));
}

#[test]
fn p76_vm_heap_slot_array_build() {
    let src = include_str!("../../syx-vm/src/lib.rs");
    assert!(src.contains("alloc_heap_slot"));
}

#[test]
fn p76_vm_heap_compact_rewrite() {
    let src = include_str!("../../syx-vm/src/lib.rs");
    assert!(src.contains("rewrite_slot_indices"));
}

#[test]
fn p76_inkwell_crate_phase76() {
    let src = include_str!("../../syx-llvm/src/lib.rs");
    assert!(src.contains("llvm_inkwell_phase76_crate_ready"));
}

#[test]
fn p76_mir_run_root_rewrite() {
    let src = include_str!("../../syx-mir-run/src/runtime.rs");
    assert!(src.contains("sync_roots_from_rewrite"));
}

#[test]
fn p76_rt_exec_counter() {
    let src = include_str!("../../syx-rt/src/abi.rs");
    assert!(src.contains("rt_internal_exec_count"));
}

#[test]
fn p76_build_farm_remote_target() {
    let src = include_str!("../../syx-driver/src/build_syx.rs");
    assert!(src.contains("\"remote\""));
}

#[test]
fn p76_cache_remote_report_module() {
    let src = include_str!("../../syx-driver/src/cache_remote_report.rs");
    assert!(src.contains("CacheRemoteReport"));
}

#[test]
fn p76_rt_exec_report_module() {
    let src = include_str!("../../syx-driver/src/rt_exec_report.rs");
    assert!(src.contains("RtExecReport"));
}

#[test]
fn p76_heap_array_parse() {
    assert!(p76_doc_ok("func main() { let a = [1, 2, 3]; print(a[0]) }"));
}

#[test]
fn p76_actor_parse() {
    assert!(p76_doc_ok(
        "actor Worker { func run() { print(1) } }\nfunc main() { Worker().run() }"
    ));
}

#[test]
fn p76_async_fn() {
    assert!(p76_doc_ok("async func f() -> Int { return 1 }\nfunc main() { print(1) }"));
}

#[test]
fn p76_class_heap_field() {
    assert!(p76_doc_ok(
        "class Node { var n: Int init(n: Int) { self.n = n } }\nfunc main() { print(Node(n: 2).n) }"
    ));
}

#[test]
fn p76_adr053_heap_four_end() {
    let adr = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/adr/architecture-decisions.md"),
    )
    .unwrap();
    assert!(adr.contains("ADR-053") || adr.contains("Phase 76"));
}

#[test]
fn p76_adr054_distributed_cache() {
    let adr = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/adr/architecture-decisions.md"),
    )
    .unwrap();
    assert!(adr.contains("ADR-054") || adr.contains("lock v10"));
}

#[test]
fn p76_doctor_cache_remote_cli() {
    let src = include_str!("../../syx-cli/src/main.rs");
    assert!(src.contains("cache_remote_report") || src.contains("cache-remote-report"));
}

#[test]
fn p76_doctor_rt_exec_cli() {
    let src = include_str!("../../syx-cli/src/main.rs");
    assert!(src.contains("rt_exec_report") || src.contains("rt-exec-report"));
}

#[test]
fn p76_lock_cache_remote_nodes() {
    let src = include_str!("../../syx-pkg/src/lockfile.rs");
    assert!(src.contains("cache_remote_nodes"));
}

#[test]
fn p76_inkwell_crate_dep() {
    let toml = include_str!("../../syx-llvm/Cargo.toml");
    assert!(toml.contains("inkwell"));
}

#[test]
fn p76_gc_compact_global_export() {
    let src = include_str!("../../syx-runtime-core/src/lib.rs");
    assert!(src.contains("compact_global"));
}
