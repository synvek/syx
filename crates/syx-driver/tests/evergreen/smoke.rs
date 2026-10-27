//! SYX architecture smoke — manifest, ADR, invariants.
// was: smoke

use syx_driver::{compile_file_with_options, emit_mir_text, DriverOptions, NativeBackend};

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

fn adr_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/adr/architecture-decisions.md")
}

#[test]
fn manifest_current() {
    let m: serde_json::Value =
        serde_json::from_str(syx_driver::embedded_std::internal_manifest_json()).unwrap();
    assert_eq!(m["phase"], 139);
    assert_eq!(m["version"], "117.0");
}

#[test]
fn architecture_decisions_exists() {
    let adr = std::fs::read_to_string(adr_path()).expect("architecture-decisions.md");
    assert!(
        adr.contains("SYX 117.0")
            || adr.contains("Phase 139")
            || adr.contains("SYX 96.0")
            || adr.contains("Phase 118")
    );
    assert!(
        adr.contains("LLVM is the sole native backend")
            || adr.contains("fileprivate")
            || adr.contains("Decision")
    );
}

#[test]
fn cache_v91() {
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs"),
    )
    .unwrap();
    assert!(src.contains("syx-cache-v91"));
    assert!(src.contains("heap_v91_phase119_track_b_wasm_fingerprint"));
}

#[test]
fn cranelift_enum_removed() {
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs"),
    )
    .unwrap();
    assert!(!src.contains("NativeBackend::Cranelift"));
    assert!(NativeBackend::parse("cranelift").is_err());
}

#[test]
fn add_assign_host_registered() {
    assert!(syx_stdlib::lookup_numeric_host("__syx_int_add_assign").is_some());
    assert!(syx_stdlib::lookup_numeric_host("__syx_int_sub_assign").is_some());
}

#[test]
fn seccomp_no_fallback() {
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../syx-macro/src/external_sandbox.rs"),
    )
    .unwrap();
    assert!(!src.contains("or_else(|_| InProcessMacroExpander"));
    assert!(src.contains("E004"));
}

#[test]
fn object_drop_llvm_abi() {
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../syx-llvm/src/lib.rs"),
    )
    .unwrap();
    // Wave A W534 / ADR-878: handle C ABI declares deleted.
    assert!(!src.contains("declare void @syx_rt_object_retain"));
    assert!(!src.contains("declare void @syx_rt_object_release"));
    assert!(!src.contains("declare i32 @syx_rt_object_drop"));
}

#[test]
fn wasm_object_imports() {
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../syx-wasm-run/src/lib.rs"),
    )
    .unwrap();
    // Wave A W534 / ADR-878: link_syx_rt_object_abi retired.
    assert!(!src.contains("fn link_syx_rt_object_abi"));
}

#[test]
fn mir_object_handle_drop_peeled() {
    // Wave A W541 / ADR-885: object_handle_drop drained with handle table peel.
    // Wave A W544 / ADR-888: MirOp::ObjectRefWrap also peeled (DictBuild dual-track birth kept).
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../syx-mir-run/src/runtime.rs"),
    )
    .unwrap();
    assert!(
        !src.contains("register_object_handle")
            && !src.contains("MirOp::ObjectRefWrap =>")
            && !src.contains("fn object_ref_wrap")
            && (src.contains("ADR-885") || src.contains("peel object_handle") || src.contains("ADR-888")),
        "mir-run must not call peeled register_object_handle; MirOp ObjectRef path must be absent"
    );
}

#[test]
fn semantic_e005() {
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../syx-semantic/src/lib.rs"),
    )
    .unwrap();
    assert!(src.contains("check_dataclass_codable_witness"));
    assert!(src.contains("E005"));
}

#[test]
fn llvm_sole_native_documented() {
    let adr = std::fs::read_to_string(adr_path()).unwrap();
    assert!(adr.contains("LLVM is the sole native backend"));
}

#[test]
fn lsp_macro_expanded_field() {
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs"),
    )
    .unwrap();
    assert!(src.contains("macro_expanded"));
}

#[test]
fn witness_matrix_attached() {
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../syx-macro/src/attached.rs"),
    )
    .unwrap();
    assert!(src.contains("witness_constraint_matrix"));
}

#[test]
fn data_class_where_parser() {
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../syx-parser/src/lib.rs"),
    )
    .unwrap();
    assert!(src.contains("parse_where_clause"));
}

#[test]
fn engine_fixed_point() {
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../syx-macro/src/engine.rs"),
    )
    .unwrap();
    assert!(src.contains("expand_to_fixed_point"));
}

#[test]
fn assign_mir_hosts() {
    let mir = emit_mir_text(&fixture("operators/assign_host.sy"), &DriverOptions::default())
        .unwrap();
    assert!(mir.contains("__syx_int_add_assign") || mir.contains("CallHost"));
}

#[test]
fn generic_witness_missing_fail() {
    assert!(compile_file_with_options(
        &fixture("macros/generic_witness_missing.sy"),
        &DriverOptions::default()
    )
    .is_err());
}

#[test]
fn int_compare_hosts() {
    for slug in [
        "__syx_int_eq",
        "__syx_int_ne",
        "__syx_int_lt",
        "__syx_int_le",
        "__syx_int_compare",
    ] {
        assert!(syx_stdlib::lookup_numeric_host(slug).is_some(), "{slug}");
    }
}

#[test]
fn llvm_only_backend() {
    assert_eq!(DriverOptions::default().native_backend, NativeBackend::Llvm);
}

#[test]
fn object_handles_module() {
    // Wave A W541 / ADR-885: object_handles.rs peeled (table API deleted).
    assert!(
        !std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../syx-runtime-core/src/object_handles.rs")
            .exists(),
        "object_handles.rs must remain deleted after W541 peel"
    );
}

#[test]
fn cranelift_compat_deleted() {
    assert!(
        !std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/cranelift_compat.rs")
            .exists()
    );
}
