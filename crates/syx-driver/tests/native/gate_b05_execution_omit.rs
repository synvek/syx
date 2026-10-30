//! Medium AOT execution-segment omit
//! Empty embed execution + RT sidecar-only; B-05 historical [~] at slice (live [x] via); ≠ MIR-free / blob delete.

#[test]
fn b05_execution_omit_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-rt/src/abi.rs",
        "docs/gates/gate-b05-execution-omit.md",
        "docs/gates/gate-b05-sidecar-primary.md",
        "docs/gates/gate-b05-dual-load-runtime.md",
        "docs/gates/gate-b05-aot-sidecar-embed.md",
        "docs/gates/gate-b05-production-cutover.md",
        "docs/gates/gate-b05-dual-blob-production-wire.md",
        "docs/gates/gate-b05-real-codec-first-slice.md",
        "docs/gates/gate-b05-production-cutover-deepen.md",
        "docs/gates/gate-b05-production-reopen.md",
        "docs/gates/gate-b05-mir-metadata-blob.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-05 Medium execution-omit requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleMediumHorizonExecutionOmitInventory")
            && mir.contains("mir_module_medium_horizon_execution_omit_inventory")
            && mir.contains("aot_embed_execution_omitted")
            && mir.contains("mir_module_from_sidecar_fn_stubs")
            && mir.contains("execution_omit_signed"),
        "syx-mir must expose Medium execution-omit inventory + empty embed helpers (W409)"
    );
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    assert!(
        abi.contains("install_runtime_from_sidecar_only")
            && abi.contains("aot_execution_segment_omitted"),
        "syx_rt_load_mir_blob must sidecar-only load when execution empty (W409)"
    );
}

#[test]
fn b05_execution_omit_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b05-execution-omit.md");
    assert!(
        spec.contains("ADR-753")
            && (spec.contains("execution-omit")
                || spec.contains("execution-segment omit")
                || spec.contains("aot_execution_segment_omitted")),
        "spec must cite ADR-753 W409 B-05 Medium execution-omit: missing markers"
    );
    assert!(
        spec.contains("MirModuleMediumHorizonExecutionOmitInventory")
            && (spec.contains("execution_omit_signed=true")
                || spec.contains("`execution_omit_signed=true`")),
        "spec must name execution-omit inventory / execution_omit_signed"
    );
    assert!(
        spec.contains("aot_execution_segment_omitted=true")
            || spec.contains("`aot_execution_segment_omitted=true`")
            || spec.contains("partial met"),
        "spec must cite aot_execution_segment_omitted=true / partial met"
    );
    for line in spec.lines() {
        if line.contains("MIR-free") || (line.contains("删除") && line.contains("blob")) {
            let denied = line.contains("out of scope")
                || line.contains("≠")
                || line.contains("禁止")
                || line.contains("Forbidden")
                || line.contains("forbidden")
                || line.contains("Frozen")
                || line.contains("非")
                || line.contains("not this window")
                || line.contains("Not this window")
                || line.contains("仍未")
                || line.contains("ahead");
            assert!(
                denied,
                "affirmative MIR-free / blob delete forbidden: {line}"
            );
        }
    }
}

