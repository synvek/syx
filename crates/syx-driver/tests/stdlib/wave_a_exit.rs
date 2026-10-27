//! Wave A structural exit charter evidence nails
//! Policy + meta flip only — top-level sign-off of plan/02 exit triad.

#[test]
fn wave_a_exit_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        // F-10 / W575: a02 pure-meta gate retired; Q-L2 behavior floor = differential Suite-1/2.
        "crates/syx-driver/tests/differential/suite1.rs",
        "crates/syx-driver/tests/differential/suite2.rs",
        "plan/archive/gates/gate-a02-ql2-floor.md",
        // F-10 / W576: a05 pure-meta gate retired; Q-S1 behavior floor = qs1 five-domain + matrix.
        "crates/syx-driver/tests/stdlib/hostfn_file.rs",
        "crates/syx-driver/tests/stdlib/hostfn_http.rs",
        "crates/syx-driver/tests/stdlib/hostfn_tcp.rs",
        "crates/syx-driver/tests/stdlib/hostfn_process.rs",
        "crates/syx-driver/tests/stdlib/hostfn_sql.rs",
        "packages/internal/hostfn_qs1_matrix.toml",
        "plan/archive/gates/gate-a05-qs1-floor.md",
        // F-10 / W577: a04 exit meta retired; Q-L4 behavior floor = 6 modules + edge-semantics (stays live).
        "docs/gates/gate-a04-edge-semantics.md",
        "crates/syx-driver/tests/stdlib/edge_deinit_nil.rs",
        "crates/syx-driver/tests/stdlib/edge_exception.rs",
        "crates/syx-driver/tests/stdlib/edge_overflow.rs",
        "crates/syx-driver/tests/stdlib/edge_nan.rs",
        "crates/syx-driver/tests/stdlib/edge_unicode.rs",
        "crates/syx-driver/tests/stdlib/edge_actor.rs",
        "docs/lang/wave-a-exit.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Wave A exit requires evidence file: {path}"
        );
    }
}

#[test]
fn wave_a_exit_spec_signed() {
    let spec = include_str!("../../../../docs/lang/wave-a-exit.md");
    assert!(
        spec.contains("ADR-642") && spec.contains("Wave A exit signed"),
        "spec must cite ADR-642 Wave A exit signed: missing markers"
    );
    assert!(
        spec.contains("Q-L2") && spec.contains("Q-S1") && spec.contains("differential"),
        "spec must cite plan/02 exit triad"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("native=partial")
                || spec.contains("Q-L5")
                || spec.contains("B-04")
                || spec.contains("wasm skip")),
        "spec Frozen must keep residual / Q-L5 / B / wasm-skip fences"
    );
    assert!(
        !spec.contains("all A-* `[x]` signed") && !spec.contains("全体轴 `[x]` signed"),
        "spec must not claim all A-* exit"
    );
}

#[test]
fn wave_a_exit_roadmap_exit_met() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    // F-02 slim SSOT dropped `- [x] Q-L*` checklist lines; Q-L2/Q-S1 remain as A-02/A-05 source tags.
    assert!(
        roadmap.lines().any(|l| l.starts_with("- [x] Q-L2"))
            || roadmap.lines().any(|l| l.starts_with("| A-02 |") && l.contains("Q-L2") && l.contains("| [x] |")),
        "Q-L2 must remain checked (checklist or A-02 [x] source tag)"
    );
    assert!(
        roadmap.lines().any(|l| l.starts_with("- [x] Q-S1"))
            || roadmap.lines().any(|l| l.starts_with("| A-05 |") && l.contains("Q-S1") && l.contains("| [x] |")),
        "Q-S1 must remain checked (checklist or A-05 [x] source tag)"
    );
    let a09 = roadmap
        .lines()
        .find(|l| l.starts_with("| A-09 |"))
        .expect("A-09 row");
    assert!(
        a09.contains("| [x] |"),
        "A-09 must remain [x] after Wave A exit: {a09}"
    );
    assert!(
        roadmap.contains("Wave A 退出条件已满足")
            || roadmap.contains("Wave A exit met")
            || roadmap.contains("Wave A 退出已满足"),
        "roadmap progress must claim Wave A exit conditions met"
    );
    assert!(
        roadmap.contains("W298") || roadmap.contains("ADR-642"),
        "roadmap must cite W298/ADR-642 exit charter"
    );
    // Honesty: structural exit does not require A-06/A-07/A-08 open; after W345/W351/W357 they may be [x].
    // After W358 A-10 may be thin [~]; must not be [x] (LSP/web unmet).
    let a08 = roadmap
        .lines()
        .find(|l| l.starts_with("| A-08 |"))
        .expect("A-08 row");
    let a10 = roadmap
        .lines()
        .find(|l| l.starts_with("| A-10 |"))
        .expect("A-10 row");
    assert!(
        a08.contains("| [ ] |") || a08.contains("| [~] |") || a08.contains("| [x] |"),
        "A-08 may be [ ]/[~]/[x] (post-W357): {a08}"
    );
    assert!(
        a10.contains("| [ ] |") || a10.contains("| [~] |") || a10.contains("| [x] |"),
        "A-10 may be [ ]/[~]/[x] (post-W361 exit): {a10}"
    );
}
