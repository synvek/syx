//! Cranelift tier-2 JIT thin-open evidence (Suite-1 index; opt-in, not production default).

#[test]
fn jit_thin_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/ql5-jit-vectors.md",
        "docs/lang/ql5-jit-floor.md",
        "plan/topics/jit.md",
        "packages/internal/capabilities.toml",
        "crates/syx-cranelift-jit/src/lib.rs",
        "crates/syx-driver/tests/lang/jit_suite1_differential.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Q-L5/B-08 thin requires evidence file: {path}"
        );
    }
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    assert!(
        caps.contains("id = \"runtime.jit.tier2\"")
            && caps.contains("experimental-real-subset")
            && caps.contains("jit_suite1_differential"),
        "capabilities.toml must retain runtime.jit.tier2 Suite-1 evidence"
    );
    assert!(
        caps.contains("ADR-706")
            && (caps.contains("W362") || caps.contains("thin-open") || caps.contains("B-08")),
        "runtime.jit.tier2 notes must cross-index ADR-706 / W362 thin-open"
    );
    let jit_topic = include_str!("../../../../plan/topics/jit.md");
    assert!(
        jit_topic.contains("Default: off")
            || jit_topic.contains("default: off")
            || jit_topic.contains("opt-in"),
        "jit.md must keep default-off honesty"
    );
}

#[test]
fn jit_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/ql5-jit-vectors.md");
    assert!(
        spec.contains("ADR-706")
            && (spec.contains("thin floor signed") || spec.contains("thin-open")),
        "spec must cite ADR-706 Q-L5/B-08 thin floor signed"
    );
    assert!(
        spec.contains("ADR-711")
            || spec.contains("exit signed")
            || spec.contains("ql5-jit-exit"),
        "spec must cross-index W367 exit"
    );
    assert!(
        (spec.contains("production") || spec.contains("默认"))
            && (spec.contains("Not") || spec.contains("≠") || spec.contains("非")),
        "spec must deny production-default JIT"
    );
    assert!(
        spec.contains("production")
            && (spec.contains("default") || spec.contains("默认"))
            && (spec.contains("Not") || spec.contains("≠") || spec.contains("非")),
        "spec must deny production-default JIT"
    );
    for cluster in [
        "Cranelift tier-2 Suite-1 pilot",
        "Opt-in 默认关",
        "Deopt 原因",
        "更多 MirOp",
    ] {
        assert!(spec.contains(cluster), "spec must cover cluster {cluster}");
    }
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("Q-L5") || spec.contains("B-08") || spec.contains("production")),
        "spec Frozen must keep Q-L5 / B-08 / production fences"
    );
    assert!(
        spec.contains("W361") || spec.contains("ADR-705") || spec.contains("a10-capability-exit"),
        "spec must cross-index W361 A-10 exit"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn jit_thin_roadmap_checked() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    let b08 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-08 |"))
        .expect("B-08 row");
    assert!(
        b08.contains("| [x] |"),
        "B-08 must be [x] after W367 exit: {b08}"
    );
    assert!(
        b08.contains("W367")
            || b08.contains("ADR-711")
            || b08.contains("exit")
            || b08.contains("W362")
            || b08.contains("thin"),
        "B-08 row must cite W362 thin and/or W367 exit: {b08}"
    );
    // F-02 slim SSOT dropped `- [x] Q-L5` checklist lines; Q-L5 remains as B-08 source tag.
    assert!(
        roadmap.lines().any(|l| l.starts_with("- [x] Q-L5"))
            || (b08.contains("Q-L5") && b08.contains("| [x] |")),
        "Q-L5 must remain checked (checklist or B-08 [x] source tag)"
    );
    assert!(
        !roadmap.lines().any(|l| l.starts_with("- [ ] Q-L5")),
        "unchecked Q-L5 must not remain"
    );
    // Honesty: Wave B axes remain on the live table (may all be [x] after exit charters).
    for id in ["B-04", "B-05", "B-06"] {
        let row = roadmap
            .lines()
            .find(|l| l.starts_with(&format!("| {id} |")))
            .unwrap_or_else(|| panic!("{id} row"));
        assert!(
            row.contains("| [ ] |") || row.contains("| [~] |") || row.contains("| [x] |"),
            "{id} must remain on the Wave table after Q-L5 thin: {row}"
        );
    }
    for id in ["A-01", "A-10"] {
        let row = roadmap
            .lines()
            .find(|l| l.starts_with(&format!("| {id} |")))
            .unwrap_or_else(|| panic!("{id} row"));
        assert!(
            row.contains("| [x] |"),
            "{id} must remain [x] after W362: {row}"
        );
    }
}

#[test]
fn jit_thin_wave_a_exit_hygiene() {
    let exit = include_str!("../../../../docs/lang/wave-a-exit.md");
    assert!(
        !exit.contains("A-08 / A-10 remain open") && !exit.contains("A-10 remain open"),
        "wave-a-exit.md must not claim A-08/A-10 remain open after W361"
    );
    assert!(
        exit.contains("A-01") && (exit.contains("all `[x]`") || exit.contains("全 `[x]`") || exit.contains("A-10")),
        "wave-a-exit.md must acknowledge A axes complete or A-10 closed"
    );
    assert!(
        exit.contains("Q-L5")
            && (exit.contains("checked")
                || exit.contains("[x]")
                || exit.contains("W367")
                || exit.contains("ADR-711")
                || exit.contains("complete")),
        "wave-a-exit.md must acknowledge Q-L5 checked / W367 exit"
    );
    assert!(
        exit.contains("W362")
            || exit.contains("ADR-706")
            || exit.contains("W367")
            || exit.contains("W368"),
        "wave-a-exit.md Next must cite W362–W368"
    );
}
