//! Cranelift tier-2 JIT exit charter (scope-narrow; ≠ production-default JIT).

#[test]
fn jit_exit_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/ql5-jit-exit.md",
        "docs/lang/ql5-jit-floor.md",
        "docs/lang/ql5-jit-vectors.md",
        "crates/syx-driver/tests/stdlib/jit_thin.rs",
        "crates/syx-driver/tests/stdlib/jit_deepen.rs",
        "crates/syx-driver/tests/stdlib/jit_floor_thin.rs",
        "crates/syx-driver/tests/lang/jit_suite1_differential.rs",
        "plan/topics/jit.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Q-L5/B-08 exit requires evidence file: {path}"
        );
    }
    let nails = include_str!("../lang/jit_suite1_differential.rs");
    assert!(
        nails.contains("fn jit_print_deopts")
            && nails.contains("fn jit_callhost_deopts")
            && nails.contains("fn jit_conststr_deopts")
            && nails.contains("fn jit_arraybuild_deopts")
            && nails.contains("fn jit_dictbuild_deopts")
            && nails.contains("fn jit_setbuild_deopts")
            && nails.contains("fn jit_loadglobal_deopts"),
        "exit evidence must retain seven deopt nails"
    );
}

#[test]
fn jit_exit_spec_signed() {
    let spec = include_str!("../../../../docs/lang/ql5-jit-exit.md");
    assert!(
        spec.contains("ADR-711")
            && (spec.contains("exit signed") || spec.contains("Exit signed")),
        "spec must cite ADR-711 exit signed"
    );
    assert!(
        spec.contains("scope narrow") || spec.contains("Scope narrow"),
        "spec must cite scope narrow"
    );
    assert!(
        (spec.contains("production") || spec.contains("默认"))
            && (spec.contains("Not") || spec.contains("≠") || spec.contains("非") || spec.contains("unmet") || spec.contains("defer")),
        "spec must deny production-default JIT as exit claim"
    );
    for marker in ["W362", "W363", "W366", "ADR-710"] {
        assert!(
            spec.contains(marker),
            "exit acceptance must cite prerequisite {marker}"
        );
    }
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("production")
                || spec.contains("residual")
                || spec.contains("B-04")
                || spec.contains("MirOp")),
        "spec Frozen must keep production / residual / B fences"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

