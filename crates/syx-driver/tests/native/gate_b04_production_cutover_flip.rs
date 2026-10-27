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

#[test]
fn b04_production_cutover_flip_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W400")
            && (roadmap.contains("cutover")
                || roadmap.contains("ADR-744")
                || roadmap.contains("production_cutover")),
        "roadmap must cite W400 cutover honesty flip"
    );
    let b05 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-05 |"))
        .expect("B-05 row");
    assert!(
        b05.contains("| [x] |"),
        "B-05 must be [x] after W447 exit: {b05}"
    );
    let b06 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-06 |"))
        .expect("B-06 row");
    assert!(
        b06.contains("| [x] |"),
        "B-06 must be [x] after W448 exit: {b06}"
    );
    let b04 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-04 |"))
        .expect("B-04 row");
    assert!(
        b04.contains("| [x] |"),
        "B-04 must be [x] after W446 exit: {b04}"
    );
}
