//! S3-I99 S4-03 scoped plan claim contracts.
//! Nails: claim ADR + plan/02 S4-03 [x]; scoped Non-claims; Stage-4 [x] 退出 4/5.
//! See docs/adr/stage4-s4-03-claim.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s4_03_plan_claim_locked() {
    let adr = repo_file("docs/adr/stage4-s4-03-claim.md");
    assert!(adr.contains("S3-I99"), "S4-03 claim ADR must exist");
    assert!(
        adr.contains("S4-03") && adr.contains("`[x]`"),
        "ADR Status must claim S4-03 [x]"
    );
    assert!(
        adr.contains("Scoped") || adr.contains("scoped"),
        "ADR must define scoped [x] meaning"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-4"),
        "ADR must Non-claim Stage-4 exit"
    );
    assert!(
        adr.contains("CancellationToken")
            || adr.contains("TaskGroup")
            || adr.contains("cancel"),
        "ADR Evidence must cite Primary cancel/TaskGroup hinge"
    );
    assert!(
        adr.contains("deferred") || adr.contains("supersede"),
        "ADR must keep Sendable/withTaskGroup deferred"
    );
    assert!(
        adr.contains("S3-I98") || adr.contains("residual-scope"),
        "ADR must cite I98 residual-scope prior"
    );
    assert!(
        adr.contains("S3-I100")
            || (adr.contains("S4-04") && adr.contains("open")),
        "ADR must cite S3-I100 S4-04 open"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s403 = plan02
        .lines()
        .find(|l| l.contains("| S4-03 |"))
        .expect("plan/02 must have S4-03 row");
    assert!(
        s403.contains("| [x] |") || s403.contains("| `[x]` |"),
        "plan/02 S4-03 status must be [x]; got: {s403}"
    );
    assert!(
        plan02.contains("S3-I99") || plan02.contains("stage4-s4-03-claim"),
        "plan/02 must cite S3-I99 claim"
    );
    let stage4 = plan02
        .lines()
        .find(|l| l.contains("| **4** |"))
        .expect("plan/02 must retain stage-4 summary row");
    assert!(
        stage4.contains("[x]") && stage4.contains("退出"),
        "Stage-4 must be [x] 退出; got: {stage4}"
    );
    assert!(
        plan02.contains("5/5"),
        "plan/02 must publish Stage-4 5/5"
    );
    assert!(
        plan02.contains("S3-I100") || plan02.contains("S4-04"),
        "plan/02 must cite I100 / S4-04 open"
    );

    let census = repo_file("docs/census/stage4-structured-concurrency-census.md");
    assert!(
        census.contains("S3-I99"),
        "census must name S3-I99"
    );
    assert!(
        census.contains("Current:** S3-I99") || census.contains("Current: S3-I99"),
        "census Current must be S3-I99"
    );
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );
    assert!(
        census.contains("4/5"),
        "census must publish Stage-4 5/5"
    );
    assert!(
        census.contains("S3-I100"),
        "census ladder must cite I100 next"
    );

    let prior = repo_file("docs/adr/stage4-s4-03-residual-scope.md");
    assert!(
        prior.contains("S3-I99") || prior.contains("stage4-s4-03-claim"),
        "I98 ADR must cite landed I99"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I99") && plan01.contains("[x]"),
        "plan/01 must reflect S3-I99 S4-03 [x]"
    );
    assert!(
        plan01.contains("5/5"),
        "plan/01 must publish Stage-4 5/5"
    );
    assert!(
        plan01.contains("S3-I100") || plan01.contains("S4-04"),
        "plan/01 must cite I100 / S4-04"
    );
}

#[test]
fn s4_03_claim_census_script_hinges() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s4_03_structured_concurrency_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run s4_03 census");
    assert!(
        output.status.success(),
        "census script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    for key in [
        "s4_03_claim_helper=",
        "residual_scope_helper=",
        "cancel_taskgroup_hinge_helper=",
        "cancel_token_helper=",
        "task_group_hostfn_helper=",
        "task_tree_cancel_test_count=",
        "s4_03_status=",
        "stage4_checklist=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    assert!(
        stdout.contains("current=S3-I99") && stdout.contains("next=S3-I100"),
        "script ladder must be I99→I100:\n{stdout}"
    );
    assert!(
        stdout.contains("s4_03_status=x")
            && stdout.contains("stage4_status=x")
            && stdout.contains("stage4_checklist=4/5"),
        "script must emit claimed status / 4/5:\n{stdout}"
    );
    let claim = stdout
        .lines()
        .find(|l| l.starts_with("s4_03_claim_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        claim >= 1,
        "expected s4_03_claim_helper>=1, got {claim}\n{stdout}"
    );
    let parse = |key: &str| {
        stdout
            .lines()
            .find(|l| l.starts_with(key))
            .and_then(|l| l.split('=').nth(1))
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(999)
    };
    assert!(parse("cancel_taskgroup_hinge_helper=") >= 1);
    assert!(parse("cancel_token_helper=") >= 1);
    assert!(parse("task_group_hostfn_helper=") >= 1);
    assert!(parse("task_tree_cancel_test_count=") >= 1);
    assert!(parse("residual_scope_helper=") >= 1);
}
