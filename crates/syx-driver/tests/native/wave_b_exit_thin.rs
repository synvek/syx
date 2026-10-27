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

#[test]
fn wave_b_exit_thin_roadmap_exit_met() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("Wave B 退出条件已满足")
            || roadmap.contains("Wave B exit met")
            || roadmap.contains("Wave B 退出已满足"),
        "roadmap progress must claim Wave B exit conditions met"
    );
    assert!(
        roadmap.contains("W340") || roadmap.contains("ADR-684"),
        "roadmap must cite W340/ADR-684 exit charter"
    );
    assert!(
        roadmap.contains("Wave A 退出条件已满足")
            || roadmap.contains("Wave A exit met")
            || roadmap.contains("Wave A 退出已满足"),
        "Wave A exit met claim must remain after Wave B exit"
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
        "B-04 [x] allowed only via W446 exit charter, not structural exit alone: {b04}"
    );
    let a06 = roadmap
        .lines()
        .find(|l| l.starts_with("| A-06 |"))
        .expect("A-06 row");
    assert!(
        a06.contains("| [x] |"),
        "A-06 must be [x] after ADR-689 exit (live SSOT): {a06}"
    );
    assert!(
        roadmap.lines().any(|l| l.contains("Q-S2") && (l.contains("[x]") || l.contains("ADR-689"))),
        "Q-S2 must be checked after A-06 exit (ADR-689)"
    );
}
