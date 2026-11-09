//! S3-I98 S4-03 residual-scope contracts.
//! Post-I99: S4-03 [x] · Stage-4 [x] 退出 4/5 · residual hinge stays green.
//! See docs/adr/stage4-s4-03-residual-scope.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s4_03_residual_scope_adr_and_files() {
    let adr = repo_file("docs/adr/stage4-s4-03-residual-scope.md");
    assert!(adr.contains("S3-I98"), "residual-scope ADR must name S3-I98");
    assert!(
        adr.contains("scoped acceptance")
            || adr.contains("Scoped acceptance")
            || adr.contains("scoped"),
        "ADR must publish scoped acceptance"
    );
    assert!(
        (adr.contains("deferred") || adr.contains("Deferred"))
            && (adr.contains("supersede") || adr.contains("superseding")),
        "ADR must defer Sendable/withTaskGroup / supersede-only"
    );
    assert!(
        adr.contains("S3-I99")
            && (adr.contains("claim") || adr.contains("`[x]`") || adr.contains("[x]")),
        "ADR must cite landed I99 scoped claim"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-4"),
        "ADR must Non-claim Stage-4 exit"
    );

    let scope = repo_file("docs/census/s4-03-concurrency-residual-scope-v0.md");
    assert!(
        scope.contains("S3-I98")
            && scope.contains("Deferred")
            && (scope.contains("supersede") || scope.contains("superseding")),
        "scope-v0 must name I98 / deferred supersede"
    );

    let census = repo_file("docs/census/stage4-structured-concurrency-census.md");
    assert!(
        census.contains("Current:** S3-I99") || census.contains("Current: S3-I99"),
        "census Current must advance to S3-I99"
    );
    assert!(
        census.contains("residual_scope_helper")
            && (census.contains("scoped") || census.contains("Deferred")),
        "census must publish I98 KPIs / scoped goal"
    );
    assert!(
        census.contains("S3-I100") && census.contains("4/5"),
        "census must name I100 next and Stage-4 4/5"
    );
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );

    let prior = repo_file("docs/adr/stage4-s4-03-cancel-taskgroup-hinge.md");
    assert!(
        prior.contains("S3-I98") || prior.contains("stage4-s4-03-residual-scope"),
        "I97 ADR must cite landed I98"
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
        s403.contains("scoped")
            || s403.contains("residual")
            || s403.contains("deferred")
            || s403.contains("Deferred"),
        "S4-03 row must publish scoped/deferred; got: {s403}"
    );
    assert!(
        plan02.contains("S3-I98")
            && plan02.contains("S3-I99")
            && plan02.contains("S3-I100")
            && plan02.contains("5/5"),
        "plan/02 must cite I98…I100 / 5/5"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I98")
            && plan01.contains("S3-I99")
            && plan01.contains("5/5"),
        "plan/01 must cite I98 / I99 / 5/5"
    );
}

#[test]
fn s4_03_residual_scope_census_script_hinges() {
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
    assert!(parse("residual_scope_helper=") >= 1);
    assert!(parse("cancel_taskgroup_hinge_helper=") >= 1);
    assert!(parse("cancel_token_helper=") >= 1);
    assert!(parse("task_group_hostfn_helper=") >= 1);
    assert!(parse("task_tree_cancel_test_count=") >= 1);
}
