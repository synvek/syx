//! S2-I266 Stage-2 exit contracts.
//! Nails: exit ADR + plan/02 stage-2 [x] 退出; S2-01…07 [x]; next Stage-3; ≠ Stage-3 construction.
//! See docs/adr/stage2-exit.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_stage2_exit_locked() {
    let adr = repo_file("docs/adr/stage2-exit.md");
    assert!(adr.contains("S2-I266"), "Stage-2 exit ADR must name S2-I266");
    assert!(
        adr.contains("Stage-2 exits") || adr.contains("**Stage-2 exits**"),
        "ADR must decide Stage-2 exits"
    );
    assert!(
        adr.contains("Residual") || adr.contains("residual"),
        "ADR must list post-exit residuals"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-3"),
        "ADR must Non-claim Stage-3 construction this window"
    );
    assert!(
        adr.contains("S3-01") || adr.contains("Stage-3 open") || adr.contains("S3-I1"),
        "ADR must lock next = Stage-3 open"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let stage2_summary = plan02
        .lines()
        .find(|l| l.contains("| **2** |"))
        .expect("plan/02 must have stage-2 summary row");
    assert!(
        stage2_summary.contains("[x]") && stage2_summary.contains("退出"),
        "plan/02 stage-2 summary must be [x] 退出; got: {stage2_summary}"
    );
    assert!(
        plan02.contains("S2-I266") || plan02.contains("stage2-exit"),
        "plan/02 must cite S2-I266 / stage2-exit"
    );

    for id in ["S2-01", "S2-02", "S2-03", "S2-04", "S2-05", "S2-06", "S2-07"] {
        let row = plan02
            .lines()
            .find(|l| l.contains(&format!("| {id} |")))
            .unwrap_or_else(|| panic!("plan/02 must have {id} row"));
        assert!(
            row.contains("| [x] |") || row.contains("| `[x]` |"),
            "plan/02 {id} must remain [x]; got: {row}"
        );
    }

    // Stage-3 status is owned by s3_01_span_census_contract after S3-I1.
    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S2-I266") || plan01.contains("stage2-exit") || plan01.contains("阶段 0 / 1 / 2 已退出"),
        "plan/01 must keep Stage-2 exit honesty"
    );
}
