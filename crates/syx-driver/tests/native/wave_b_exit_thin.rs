//! Wave B structural exit charter evidence nails (Wave A W340 / ADR-684).
//! Policy + meta flip only — top-level sign-off of plan/02 Wave B exit triad.

#[test]
fn wave_b_exit_thin_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/wave-b-exit.md",
        "docs/lang/wave-b-exit-readiness.md",
        "docs/lang/wave-b-exit-triad-b04-defer.md",
        "docs/lang/wave-b-exit-triad-b05-defer.md",
        "docs/lang/wave-b-exit-triad-b06-defer.md",
        "crates/syx-mir/src/lib.rs",  // MirModuleMediumHorizon*Inventory (F-10 W587)
        "crates/syx-rt/src/llvm_stub_audit.rs",
        "crates/syx-driver/tests/native/wave_b_exit_readiness_thin.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Wave B exit requires evidence file: {path}"
        );
    }
}

#[test]
fn wave_b_exit_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/wave-b-exit.md");
    assert!(
        spec.contains("ADR-684") && spec.contains("Wave B exit signed"),
        "spec must cite ADR-684 Wave B exit signed: missing markers"
    );
    assert!(
        spec.contains("B-04") && spec.contains("B-05") && spec.contains("B-06"),
        "spec must cite plan/02 Wave B exit triad"
    );
    assert!(
        spec.contains("诚实延后签收 met") || spec.contains("honest defer"),
        "spec triad rows must be honest-defer met"
    );
    assert!(
        spec.contains("## Frozen")
            && spec.contains("B-04")
            && (spec.contains("B-05") || spec.contains("B-06"))
            && (spec.contains("native=partial") || spec.contains("timeout_tls")),
        "spec Frozen must keep axis [x] and native=partial fences"
    );
    assert!(
        !spec.contains("B-04 `[x]` signed")
            && !spec.contains("B-05 `[x]` signed")
            && !spec.contains("B-06 `[x]` signed"),
        "spec must not claim B-04/B-05/B-06 axis exit"
    );
    assert!(
        spec.contains("Progress may claim **Wave B 退出条件已满足**")
            || spec.contains("Wave B 退出条件已满足"),
        "spec must allow Wave B 退出条件已满足 claim"
    );
}

