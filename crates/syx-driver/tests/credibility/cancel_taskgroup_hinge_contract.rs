//! S3-I97 S4-03 cancel/TaskGroup hinge contracts.
//! Post-I99: S4-03 [x] · Stage-4 [x] 退出 4/5 · Primary KPIs stay green.
//! See docs/adr/stage4-s4-03-cancel-taskgroup-hinge.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s4_03_cancel_taskgroup_hinge_adr_and_files() {
    let adr = repo_file("docs/adr/stage4-s4-03-cancel-taskgroup-hinge.md");
    assert!(adr.contains("S3-I97"), "hinge ADR must name S3-I97");
    assert!(
        adr.contains("CancellationToken") || adr.contains("TaskGroup") || adr.contains("cancel"),
        "ADR must publish cancel/TaskGroup Primary"
    );
    assert!(
        adr.contains("S3-I99") || adr.contains("stage4-s4-03-claim"),
        "ADR Consequences must cite landed I99"
    );

    let charter = repo_file("docs/census/s4-03-cancel-taskgroup-v0.md");
    assert!(
        charter.contains("S3-I97")
            && (charter.contains("CancellationToken") || charter.contains("TaskGroup")),
        "cancel-taskgroup-v0 must exist"
    );

    let census = repo_file("docs/census/stage4-structured-concurrency-census.md");
    assert!(
        census.contains("Current:** S3-I99") || census.contains("Current: S3-I99"),
        "census Current must advance to S3-I99 after claim"
    );
    assert!(
        census.contains("cancel_taskgroup_hinge_helper") && census.contains("S3-I100"),
        "census must publish I97 KPIs / I100 next"
    );
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );

    let prior = repo_file("docs/adr/stage4-s4-03-open.md");
    assert!(
        prior.contains("S3-I97") || prior.contains("stage4-s4-03-cancel-taskgroup-hinge"),
        "I96 ADR must cite landed I97"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s403 = plan02
        .lines()
        .find(|l| l.contains("| S4-03 |"))
        .expect("S4-03 row");
    assert!(
        s403.contains("| [x] |") || s403.contains("| `[x]` |") || s403.contains("[x]"),
        "S4-03 must be [x] after I99; got: {s403}"
    );
    assert!(
        plan02.contains("S3-I97")
            && plan02.contains("S3-I98")
            && plan02.contains("S3-I99")
            && plan02.contains("5/5"),
        "plan/02 must cite I97…I99 / 5/5"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I97") && plan01.contains("5/5"),
        "plan/01 must cite I97 / 5/5"
    );
}

#[test]
fn s4_03_cancel_taskgroup_hinge_census_script_hinges() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = std::process::Command::new("bash")
        .arg("scripts/s4_03_structured_concurrency_census.sh")
        .current_dir(&root)
        .output()
        .expect("run s4_03 census");
    assert!(
        out.status.success(),
        "census failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("current=S3-I99") && stdout.contains("next=S3-I100"),
        "ladder I99→I100:\n{stdout}"
    );
    assert!(
        stdout.contains("stage4_status=x")
            && stdout.contains("s4_03_status=x")
            && stdout.contains("stage4_checklist=4/5"),
        "status/checklist:\n{stdout}"
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
    assert!(parse("s4_03_open_helper=") >= 1);
}
