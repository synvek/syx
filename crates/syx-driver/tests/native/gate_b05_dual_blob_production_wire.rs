//! Medium dual-blob production wire
//! Package encode/decode + inventory; B-05 historical [~] at slice (live [x] via); ≠ AOT sidecar embed / cutover / MIR-free.

#[test]
fn b05_dual_blob_production_wire_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "docs/gates/gate-b05-dual-blob-production-wire.md",
        "docs/gates/gate-b05-real-codec-first-slice.md",
        "docs/gates/gate-b05-production-cutover-deepen.md",
        "docs/gates/gate-b05-production-reopen.md",
        "docs/gates/gate-b05-mir-metadata-blob.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-05 dual-blob production wire requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleMediumHorizonDualBlobProductionWireInventory")
            && mir.contains("mir_module_medium_horizon_dual_blob_production_wire_inventory")
            && mir.contains("encode_module_dual_blob_package")
            && mir.contains("decode_module_dual_blob_package"),
        "syx-mir must expose dual-blob production wire APIs (W404)"
    );
}

#[test]
fn b05_dual_blob_production_wire_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b05-dual-blob-production-wire.md");
    assert!(
        spec.contains("ADR-748")
            && (spec.contains("dual-blob")
                || spec.contains("Dual-blob")
                || spec.contains("dual blob")),
        "spec must cite ADR-748 W404 B-05 dual-blob production wire: missing markers"
    );
    assert!(
        spec.contains("MirModuleMediumHorizonDualBlobProductionWireInventory")
            && spec.contains("encode_module_dual_blob_package")
            && spec.contains("decode_module_dual_blob_package"),
        "spec must name dual-blob production wire inventory / package encode-decode APIs"
    );
    assert!(
        spec.contains("dual_blob_production_wire_signed=true")
            || spec.contains("`dual_blob_production_wire_signed=true`")
            || spec.contains("partial met"),
        "spec must cite dual-blob production wire signed / partial met"
    );
    assert!(
        spec.contains("production_dual_load_enabled=true")
            || spec.contains("`production_dual_load_enabled=true`"),
        "spec must cite production_dual_load_enabled=true"
    );
    assert!(
        spec.contains("aot_embeds_sidecar=true")
            || spec.contains("`aot_embeds_sidecar=true`")
            || spec.contains("aot_embeds_sidecar=false")
            || spec.contains("`aot_embeds_sidecar=false`")
            || (spec.contains("whole-module")
                && (spec.contains("single blob") || spec.contains("execution")))
            || spec.contains("b05-aot-sidecar-embed"),
        "spec must cite AOT embed honesty (sidecar flag / whole-module load / W406 cross)"
    );
    assert!(
        spec.contains("cutover_enabled=true")
            || spec.contains("`cutover_enabled=true`")
            || spec.contains("b05-production-cutover")
            || (spec.contains("cutover") && spec.contains("partial met")),
        "spec must cite Medium cutover honesty on/partial met (W405)"
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

