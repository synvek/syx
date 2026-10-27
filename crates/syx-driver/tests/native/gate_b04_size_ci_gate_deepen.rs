//! Short size golden CI gate deepen
//! Relational gate + honesty migration; B-04 remains [~]; ≠ cutover / absolute thresholds / MIR-free.

#[test]
fn b04_size_ci_gate_deepen_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "docs/gates/gate-b04-size-ci-gate.md",
        "docs/gates/gate-b04-multi-root-wire.md",
        "docs/gates/gate-b04-production-cutover-deepen.md",
        "docs/gates/gate-b04-mir-tree-shake.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-04 size-CI gate requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("AotShortHorizonSizeCiGateInventory")
            && mir.contains("aot_short_horizon_size_ci_gate_inventory")
            && mir.contains("size_ci_gate_signed"),
        "syx-mir must expose AotShortHorizonSizeCiGateInventory (W399)"
    );
}

#[test]
fn b04_size_ci_gate_deepen_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b04-size-ci-gate.md");
    assert!(
        spec.contains("ADR-743")
            && (spec.contains("size-CI")
                || spec.contains("size golden")
                || spec.contains("Size golden")),
        "spec must cite ADR-743 W399 size-CI gate"
    );
    assert!(
        spec.contains("AotShortHorizonSizeCiGateInventory")
            && (spec.contains("size_ci_gate_signed=true")
                || spec.contains("`size_ci_gate_signed=true`")),
        "spec must name size-CI gate inventory / signed flag"
    );
    assert!(
        spec.contains("size_ci_gate_enabled=true")
            || spec.contains("`size_ci_gate_enabled=true`")
            || spec.contains("partial met"),
        "spec must cite size-CI gate enabled / partial met"
    );
    assert!(
        spec.contains("cutover_enabled=false")
            || spec.contains("`cutover_enabled=false`")
            || spec.contains("cutover_enabled=true")
            || (spec.contains("cutover")
                && (spec.contains("unmet")
                    || spec.contains("partial met")
                    || spec.contains("W400"))),
        "spec must cite cutover honesty status (W399 false / W400 true)"
    );
    assert!(
        spec.contains("absolute_threshold_count")
            || (spec.contains("absolute")
                && (spec.contains("0") || spec.contains("3") || spec.contains("met"))),
        "spec must cite absolute thresholds (W399 absent / W439 met)"
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
fn b04_size_ci_gate_deepen_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W399")
            && (roadmap.contains("size-CI")
                || roadmap.contains("size_ci")
                || roadmap.contains("size golden")
                || roadmap.contains("ADR-743")),
        "roadmap must cite W399 size-CI gate"
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
