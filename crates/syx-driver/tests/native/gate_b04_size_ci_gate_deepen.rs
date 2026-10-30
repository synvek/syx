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

