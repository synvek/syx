//! S3-I96…I99 Stage-4 / S4-03 structured-concurrency census contracts.
//! Open + hinge + residual + claim · Stage-4 [x] 退出 · S4-03 [x] · 4/5.
//! See docs/adr/stage4-s4-03-open.md · docs/stage4-structured-concurrency-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s4_03_structured_concurrency_census_baseline() {
    let census = repo_file("docs/stage4-structured-concurrency-census.md");
    assert!(census.contains("S3-I96"), "census must name S3-I96");
    assert!(census.contains("S3-I97"), "census must name S3-I97");
    assert!(census.contains("S3-I98"), "census must name S3-I98");
    assert!(census.contains("S3-I99"), "census must name S3-I99");
    assert!(census.contains("S3-I100"), "census must name S3-I100 next");
    assert!(
        census.contains("Current:** S3-I99") || census.contains("Current: S3-I99"),
        "census Current must advance to S3-I99"
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
        census.contains("CancellationToken")
            || census.contains("cancel")
            || census.contains("TaskGroup")
            || census.contains("Sendable"),
        "census must cover cancel / TaskGroup / Sendable axes"
    );
    assert!(
        census.contains("cancel_taskgroup_hinge_helper")
            || census.contains("residual_scope_helper")
            || census.contains("cancel_token_helper"),
        "census must publish hinge / residual KPIs"
    );
    assert!(
        census.contains("s4_03_claim_helper") || census.contains("claim"),
        "census must publish claim hinge"
    );

    let adr = repo_file("docs/adr/stage4-s4-03-open.md");
    assert!(adr.contains("S3-I96"), "open ADR must name S3-I96");
    assert!(
        adr.contains("S3-I99") || adr.contains("stage4-s4-03-claim"),
        "open ADR Consequences must cite landed I99"
    );
    assert!(
        adr.contains("S3-I100") || adr.contains("S4-04"),
        "open ADR Consequences must lock next = I100"
    );
    assert!(
        adr.contains("S3-I95") || adr.contains("stage4-s4-02-claim"),
        "ADR must cite prior S4-02 claim"
    );
    assert!(
        adr.contains("Nursery") || adr.contains("N/A"),
        "ADR must mark GC Nursery N/A OUT"
    );

    let script = repo_file("scripts/s4_03_structured_concurrency_census.sh");
    assert!(
        script.contains("s4_03_open_helper")
            && script.contains("cancel_taskgroup_hinge_helper")
            && script.contains("residual_scope_helper")
            && script.contains("s4_03_claim_helper")
            && script.contains("cancel_token_helper")
            && script.contains("task_group_hostfn_helper")
            && script.contains("sendable_surface_helper")
            && script.contains("task_tree_cancel_test_count")
            && script.contains("current=S3-I99")
            && script.contains("next=S3-I100"),
        "census script must emit I99 ladder / KPIs"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let stage4 = plan02
        .lines()
        .find(|l| l.contains("| **4** |"))
        .expect("plan/02 stage-4 row");
    assert!(
        stage4.contains("[x]") && stage4.contains("退出"),
        "Stage-4 must be [x] 退出; got: {stage4}"
    );
    assert!(
        stage4.contains("5/5"),
        "Stage-4 checklist must be 5/5; got: {stage4}"
    );
    let s403 = plan02
        .lines()
        .find(|l| l.contains("| S4-03 |"))
        .expect("plan/02 S4-03 row");
    assert!(
        s403.contains("| [x] |") || s403.contains("| `[x]` |") || s403.contains("[x]"),
        "S4-03 must be [x] after I99; got: {s403}"
    );
    assert!(
        plan02.contains("S3-I96")
            && plan02.contains("S3-I97")
            && plan02.contains("S3-I98")
            && plan02.contains("S3-I99")
            && plan02.contains("S3-I100"),
        "plan/02 must cite I96…I100"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I99") && plan01.contains("5/5"),
        "plan/01 must cite I99 / 5/5"
    );
    assert!(
        plan01.contains("S4-03") && plan01.contains("[x]"),
        "plan/01 must reflect S4-03 [x]"
    );
    assert!(
        plan01.contains("S3-I100") || plan01.contains("S4-04"),
        "plan/01 must lock next = I100"
    );

    let prior = repo_file("docs/adr/stage4-s4-02-claim.md");
    assert!(
        prior.contains("S3-I96") || prior.contains("stage4-s4-03-open"),
        "I95 ADR must cite landed I96"
    );
}

#[test]
fn s4_03_structured_concurrency_census_script_runs() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = std::process::Command::new("bash")
        .arg("scripts/s4_03_structured_concurrency_census.sh")
        .current_dir(&root)
        .output()
        .expect("run s4_03 census");
    assert!(
        out.status.success(),
        "census script failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    for key in [
        "s4_03_open_helper=",
        "s4_03_census_helper=",
        "cancel_taskgroup_hinge_helper=",
        "residual_scope_helper=",
        "s4_03_claim_helper=",
        "cancel_token_helper=",
        "task_group_hostfn_helper=",
        "sendable_surface_helper=",
        "task_tree_cancel_test_count=",
        "stage4_status=",
        "s4_03_status=",
        "stage4_checklist=",
    ] {
        assert!(
            stdout.contains(key),
            "expected {key} in:\n{stdout}"
        );
    }
    assert!(
        stdout.contains("current=S3-I99") && stdout.contains("next=S3-I100"),
        "ladder I99→I100:\n{stdout}"
    );
    assert!(
        stdout.contains("stage4_status=x")
            && stdout.contains("s4_01_status=x")
            && stdout.contains("s4_02_status=x")
            && stdout.contains("s4_03_status=x")
            && stdout.contains("stage4_checklist=4/5"),
        "status / 4/5:\n{stdout}"
    );
    let parse = |key: &str| {
        stdout
            .lines()
            .find(|l| l.starts_with(key))
            .and_then(|l| l.split('=').nth(1))
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(999)
    };
    assert!(parse("s4_03_open_helper=") >= 1);
    assert!(parse("s4_03_census_helper=") >= 1);
    assert!(parse("cancel_taskgroup_hinge_helper=") >= 1);
    assert!(parse("residual_scope_helper=") >= 1);
    assert!(parse("s4_03_claim_helper=") >= 1);
    assert!(parse("cancel_token_helper=") >= 1);
    assert!(parse("task_group_hostfn_helper=") >= 1);
    assert!(parse("sendable_surface_helper=") >= 1);
    assert!(parse("task_tree_cancel_test_count=") >= 1);
}

#[test]
fn s4_03_open_does_not_exit_stage4() {
    let claim = repo_file("docs/adr/stage4-s4-03-claim.md");
    assert!(
        claim.contains("S4-03") && claim.contains("`[x]`"),
        "claim ADR Status must keep S4-03 [x]"
    );
    assert!(
        claim.contains("≠") && claim.contains("Stage-4"),
        "claim ADR must Non-claim Stage-4 exit"
    );
    let census = repo_file("docs/stage4-structured-concurrency-census.md");
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );
    assert!(
        census.contains("4/5"),
        "census must keep Stage-4 4/5"
    );
    let residual = repo_file("docs/adr/stage4-s4-03-residual-scope.md");
    assert!(
        residual.contains("S3-I99") || residual.contains("stage4-s4-03-claim"),
        "residual ADR must cite landed I99 claim"
    );
}
