//! Cranelift tier-2 JIT floor — indexes thin + deepen×3 Suite-1 evidence.

#[test]
fn jit_floor_thin_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/ql5-jit-floor.md",
        "docs/lang/ql5-jit-vectors.md",
        "plan/topics/jit.md",
        "packages/internal/capabilities.toml",
        "crates/syx-cranelift-jit/src/lib.rs",
        "crates/syx-driver/tests/lang/jit_suite1_differential.rs",
        "crates/syx-driver/tests/stdlib/jit_thin.rs",
        "crates/syx-driver/tests/stdlib/jit_deepen.rs",
        "crates/syx-driver/tests/fixtures/jit_suite1/print_deopt.sy",
        "crates/syx-driver/tests/fixtures/jit_suite1/callhost_deopt.sy",
        "crates/syx-driver/tests/fixtures/jit_suite1/conststr_deopt.sy",
        "crates/syx-driver/tests/fixtures/jit_suite1/arraybuild_deopt.sy",
        "crates/syx-driver/tests/fixtures/jit_suite1/dictbuild_deopt.sy",
        "crates/syx-driver/tests/fixtures/jit_suite1/setbuild_deopt.sy",
        "crates/syx-driver/tests/fixtures/jit_suite1/loadglobal_deopt.sy",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Q-L5/B-08 floor requires evidence file: {path}"
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
        "lang must retain seven floor-evidence deopt nails"
    );
}

#[test]
fn jit_floor_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/ql5-jit-floor.md");
    assert!(
        spec.contains("ADR-710")
            && (spec.contains("Q-L5 / B-08 floor signed")
                || spec.contains("floor signed")
                || spec.contains("B-08 floor signed")),
        "spec must cite ADR-710 Q-L5/B-08 floor signed"
    );
    assert!(
        spec.contains("Floor ≠ exit")
            || spec.contains("floor ≠ exit")
            || (spec.contains("Exit") && spec.contains("separately")),
        "spec must keep floor ≠ exit distinction"
    );
    for cluster in [
        "Cranelift tier-2 Suite-1 pilot",
        "Opt-in 默认关",
        "Deopt UnsupportedMirOp",
        "更多 MirOp",
    ] {
        assert!(spec.contains(cluster), "spec must cover cluster {cluster}");
    }
    assert!(
        spec.contains("**met**")
            && spec.contains("**partial met**")
            && spec.contains("**unmet**"),
        "spec must index met / partial met / unmet statuses"
    );
    for nail in [
        "jit_print_deopts",
        "jit_callhost_deopts",
        "jit_conststr_deopts",
        "jit_arraybuild_deopts",
        "jit_dictbuild_deopts",
        "jit_setbuild_deopts",
        "jit_loadglobal_deopts",
    ] {
        assert!(spec.contains(nail), "spec must index evidence {nail}");
    }
    assert!(
        spec.contains("ADR-711")
            || spec.contains("ql5-jit-exit")
            || spec.contains("exit signed"),
        "spec must cross-index W367 exit"
    );
    assert!(
        spec.contains("Floor ≠ exit")
            || spec.contains("floor ≠ exit")
            || (spec.contains("Exit") && spec.contains("separately")),
        "spec must keep floor ≠ exit distinction"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("production") || spec.contains("residual") || spec.contains("MirOp")),
        "spec Frozen must keep production / residual fences"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn jit_floor_thin_roadmap_checked() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W366")
            && (roadmap.contains("floor")
                || roadmap.contains("ADR-710")
                || roadmap.contains("ql5-jit-floor")
                || roadmap.contains("JIT floor")),
        "roadmap must cite W366 Q-L5/B-08 floor"
    );
    let b08 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-08 |"))
        .expect("B-08 row");
    assert!(
        b08.contains("| [x] |"),
        "B-08 must be [x] after W367 exit (floor + exit): {b08}"
    );
    assert!(
        b08.contains("W367")
            || b08.contains("ADR-711")
            || b08.contains("exit")
            || b08.contains("W366")
            || b08.contains("floor"),
        "B-08 row must cite W366 floor and/or W367 exit: {b08}"
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
            "{id} must remain on the Wave table after Q-L5 floor: {row}"
        );
    }
}
