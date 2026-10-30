//! Medium production cutover honesty flip
//! Cutover inventory + honesty; B-05 historical [~] at slice (live [x] via); ≠ AOT sidecar embed / dual-load runtime / MIR-free.

#[test]
fn b05_production_cutover_flip_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
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
            "B-05 Medium cutover honesty requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleMediumHorizonProductionCutoverInventory")
            && mir.contains("mir_module_medium_horizon_production_cutover_inventory")
            && mir.contains("production_cutover_signed"),
        "syx-mir must expose Medium production cutover honesty inventory (W405)"
    );
}

#[test]
fn b05_production_cutover_flip_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b05-production-cutover.md");
    assert!(
        spec.contains("ADR-749")
            && (spec.contains("cutover honesty")
                || spec.contains("Cutover honesty")
                || spec.contains("production cutover")),
        "spec must cite ADR-749 W405 B-05 Medium cutover honesty: missing markers"
    );
    assert!(
        spec.contains("MirModuleMediumHorizonProductionCutoverInventory")
            && (spec.contains("production_cutover_signed=true")
                || spec.contains("`production_cutover_signed=true`")),
        "spec must name cutover inventory / production_cutover_signed"
    );
    assert!(
        spec.contains("cutover_enabled=true")
            || spec.contains("`cutover_enabled=true`")
            || spec.contains("partial met"),
        "spec must cite cutover_enabled=true / partial met"
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

