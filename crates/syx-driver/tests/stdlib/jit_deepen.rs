//! Cranelift tier-2 JIT deepen — CallHost/ConstStr/ArrayBuild/DictBuild/SetBuild/LoadGlobal deopt nails.

#[test]
fn jit_deepen_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/ql5-jit-vectors.md",
        "crates/syx-driver/tests/lang/jit_suite1_differential.rs",
        "crates/syx-driver/tests/fixtures/jit_suite1/callhost_deopt.sy",
        "crates/syx-driver/tests/fixtures/jit_suite1/conststr_deopt.sy",
        "crates/syx-driver/tests/fixtures/jit_suite1/print_deopt.sy",
        "crates/syx-driver/tests/fixtures/jit_suite1/arraybuild_deopt.sy",
        "crates/syx-driver/tests/fixtures/jit_suite1/dictbuild_deopt.sy",
        "crates/syx-driver/tests/fixtures/jit_suite1/setbuild_deopt.sy",
        "crates/syx-driver/tests/fixtures/jit_suite1/loadglobal_deopt.sy",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Q-L5/B-08 deepen requires evidence file: {path}"
        );
    }
    let nails = include_str!("../lang/jit_suite1_differential.rs");
    assert!(
        nails.contains("fn jit_callhost_deopts")
            && nails.contains("fn jit_conststr_deopts")
            && nails.contains("fn jit_print_deopts")
            && nails.contains("fn jit_arraybuild_deopts")
            && nails.contains("fn jit_dictbuild_deopts")
            && nails.contains("fn jit_setbuild_deopts")
            && nails.contains("fn jit_loadglobal_deopts"),
        "lang must define Print + CallHost + ConstStr + ArrayBuild + DictBuild + SetBuild + LoadGlobal deopt nails"
    );
    assert!(
        nails.contains("UnsupportedMirOp(CallHost)")
            && nails.contains("UnsupportedMirOp(ConstStr)")
            && nails.contains("ArrayBuild")
            && nails.contains("DictBuild")
            && nails.contains("SetBuild")
            && nails.contains("LoadGlobal"),
        "lang nails must assert CallHost/ConstStr/ArrayBuild/DictBuild/SetBuild/LoadGlobal reasons"
    );
}

#[test]
fn jit_deepen_spec_partial_met() {
    let spec = include_str!("../../../../docs/lang/ql5-jit-vectors.md");
    assert!(
        spec.contains("ADR-710")
            && (spec.contains("floor signed") || spec.contains("floor")),
        "spec must cite ADR-710 floor signed"
    );
    assert!(
        spec.contains("ADR-709")
            && (spec.contains("deepen batch-3") || spec.contains("batch-3")),
        "spec must cite ADR-709 deepen batch-3"
    );
    assert!(
        spec.contains("ADR-708")
            && (spec.contains("deepen batch-2") || spec.contains("batch-2")),
        "spec must cite ADR-708 deepen batch-2"
    );
    assert!(
        spec.contains("ADR-707")
            && (spec.contains("deepen batch-1") || spec.contains("deepen")),
        "spec must cite ADR-707 deepen batch-1"
    );
    assert!(
        spec.contains("partial met")
            && spec.contains("jit_callhost_deopts")
            && spec.contains("jit_conststr_deopts")
            && spec.contains("jit_arraybuild_deopts")
            && spec.contains("jit_dictbuild_deopts")
            && spec.contains("jit_setbuild_deopts")
            && spec.contains("jit_loadglobal_deopts")
            && (spec.contains("jit_print_deopts") || spec.contains("Print")),
        "spec Deopt cluster must index seven deopt nails as partial met"
    );
    assert!(
        spec.contains("ADR-711")
            || spec.contains("exit signed")
            || spec.contains("ql5-jit-exit"),
        "spec must cross-index W367 exit"
    );
    assert!(
        spec.contains("production")
            && (spec.contains("default") || spec.contains("默认"))
            && (spec.contains("Not") || spec.contains("≠") || spec.contains("非")),
        "spec must deny production-default JIT"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("supported") || spec.contains("Supported") || spec.contains("MirOp")),
        "spec Frozen must keep supported-MirOp / production fences"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn jit_deepen_roadmap_checked() {
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
            || b08.contains("deepen"),
        "B-08 row must cite W367 exit or prior deepen: {b08}"
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
            "{id} must remain on the Wave table after Q-L5 deepen: {row}"
        );
    }
}
