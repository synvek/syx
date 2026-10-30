//! Cranelift tier-2 JIT thin-open evidence (Suite-1 index; opt-in, not production default).


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
