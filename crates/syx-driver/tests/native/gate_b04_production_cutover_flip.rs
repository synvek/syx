//! Short production cutover honesty flip
//! Honesty flag + main-only behavior nail; B-04 remains [~]; ≠ default multi-root emit / absolute thresholds / MIR-free.

#[test]
fn b04_production_cutover_flip_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "docs/gates/gate-b04-production-cutover.md",
        "docs/gates/gate-b04-size-ci-gate.md",
        "docs/gates/gate-b04-multi-root-wire.md",
        "docs/gates/gate-b04-production-cutover-deepen.md",
        "docs/gates/gate-b04-mir-tree-shake.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-04 production cutover flip requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("AotShortHorizonProductionCutoverInventory")
            && mir.contains("aot_short_horizon_production_cutover_inventory")
            && mir.contains("production_cutover_signed"),
        "syx-mir must expose AotShortHorizonProductionCutoverInventory (W400)"
    );
}

#[test]
fn b04_production_cutover_flip_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b04-production-cutover.md");
    assert!(
        spec.contains("ADR-744")
            && (spec.contains("cutover honesty")
                || spec.contains("production cutover")
                || spec.contains("cutover_enabled")),
        "spec must cite ADR-744 W400 cutover honesty flip"
    );
    assert!(
        spec.contains("AotShortHorizonProductionCutoverInventory")
            && (spec.contains("production_cutover_signed=true")
                || spec.contains("`production_cutover_signed=true`")),
        "spec must name production cutover inventory / signed flag"
    );
    assert!(
        spec.contains("cutover_enabled=true")
            || spec.contains("`cutover_enabled=true`")
            || (spec.contains("cutover") && spec.contains("partial met")),
        "spec must cite cutover honesty on / partial met"
    );
    assert!(
        spec.contains("main-only")
            || spec.contains("仍 main-only")
            || spec.contains("still main-only"),
        "spec must nail default emit/compact main-only"
    );
    assert!(
        spec.contains("absolute_threshold_count")
            || (spec.contains("absolute")
                && (spec.contains("0") || spec.contains("3") || spec.contains("met"))),
        "spec must cite absolute thresholds (W400 absent / W439 met)"
    );
    for line in spec.lines() {
        if line.contains("MIR-free") || (line.contains("删除") && line.contains("blob")) {
            let denied = line.contains("out of scope")
                || line.contains("≠")
                || line.contains("禁止")
                || line.contains("Frozen")
                || line.contains("非")
                || line.contains("not this window");
            assert!(denied, "affirmative MIR-free forbidden: {line}");
        }
    }
}

