//! S3-I88…I91 Stage-4 / S4-01 stdlib stability + doctest census contracts.
//! Open + SSOT + residual + claim · Stage-4 [x] 退出 · S4-01 [x] · 4/5.
//! See docs/adr/stage4-open.md · docs/census/stage4-stdlib-stability-doctest-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s4_01_stdlib_stability_doctest_census_baseline() {
    let census = repo_file("docs/census/stage4-stdlib-stability-doctest-census.md");
    assert!(census.contains("S3-I88"), "census must name S3-I88 open");
    assert!(census.contains("S3-I89"), "census must name S3-I89");
    assert!(census.contains("S3-I90"), "census must name S3-I90");
    assert!(census.contains("S3-I91"), "census must name S3-I91");
    assert!(census.contains("S3-I92"), "census must name S3-I92 next");
    assert!(
        census.contains("Current:** S3-I91") || census.contains("Current: S3-I91"),
        "census Current must advance to S3-I91"
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
        census.contains("stable")
            && census.contains("preview")
            && census.contains("experimental"),
        "census must name plan ternary"
    );
    assert!(
        census.contains("doctest") && census.contains("manifest"),
        "census must cover doctest / manifest gaps"
    );
    assert!(
        census.contains("stability_ssot_helper")
            || census.contains("stability.toml"),
        "census must publish SSOT hinge"
    );
    assert!(
        census.contains("residual_scope_helper")
            || census.contains("residual-scope"),
        "census must publish residual hinge"
    );
    assert!(
        census.contains("s4_01_claim_helper") || census.contains("claim"),
        "census must publish claim hinge"
    );

    let adr = repo_file("docs/adr/stage4-open.md");
    assert!(adr.contains("S3-I88"), "open ADR must name S3-I88");
    assert!(
        adr.contains("Stage-4 `[~]`"),
        "ADR Status must keep Stage-4 [x] 退出"
    );
    assert!(
        adr.contains("S3-I89") || adr.contains("stability-ssot"),
        "ADR Consequences must cite landed I89"
    );
    assert!(
        adr.contains("S3-I90") || adr.contains("residual"),
        "ADR Consequences must cite landed I90"
    );
    assert!(
        adr.contains("S3-I91") || adr.contains("claim"),
        "ADR Consequences must cite landed I91"
    );
    assert!(
        (adr.contains("S3-I92") || adr.contains("S4-02"))
            && (adr.contains("S3-I93") || adr.contains("construct")),
        "ADR Consequences must cite I92 landed / I93 next"
    );
    assert!(
        adr.contains("S3-I87") || adr.contains("stage3-exit"),
        "ADR must cite prior Stage-3 exit"
    );

    let script = repo_file("scripts/s4_01_stdlib_stability_doctest_census.sh");
    assert!(
        script.contains("stage4_open_helper")
            && script.contains("stability_ssot_helper")
            && script.contains("residual_scope_helper")
            && script.contains("s4_01_claim_helper")
            && script.contains("manifest_stability_field_count")
            && script.contains("syx_doctest_subcommand_count")
            && script.contains("current=S3-I91")
            && script.contains("next=S3-I93"),
        "census script must emit I91→I93 ladder / KPIs"
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
    let s401 = plan02
        .lines()
        .find(|l| l.contains("| S4-01 |"))
        .expect("plan/02 S4-01 row");
    assert!(
        s401.contains("| [x] |") || s401.contains("| `[x]` |"),
        "S4-01 must be [x]; got: {s401}"
    );
    assert!(
        plan02.contains("S3-I88") && plan02.contains("5/5"),
        "plan/02 must cite S3-I88 and Stage-4 4/5"
    );
    assert!(
        plan02.contains("S3-I89")
            && plan02.contains("S3-I90")
            && plan02.contains("S3-I91")
            && plan02.contains("S3-I92"),
        "plan/02 must cite I89…I92"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("5/5"),
        "plan/01 must cite Stage-4 5/5"
    );
    assert!(
        plan01.contains("S4-01") && plan01.contains("[x]"),
        "plan/01 must reflect S4-01 [x]"
    );
    assert!(
        plan01.contains("S3-I91")
            && (plan01.contains("S3-I92") || plan01.contains("S4-02")),
        "plan/01 must cite I91 / I92"
    );
    assert!(
        plan01.contains("S3-I88") || plan01.contains("stage4-open"),
        "plan/01 must cite Stage-4 open / I88"
    );

    let prior = repo_file("docs/adr/stage3-exit.md");
    assert!(
        prior.contains("S3-I88") || prior.contains("stage4-open"),
        "I87 ADR must cite landed I88"
    );
}

#[test]
fn s4_01_stdlib_stability_doctest_census_script_runs() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = std::process::Command::new("bash")
        .arg("scripts/s4_01_stdlib_stability_doctest_census.sh")
        .current_dir(&root)
        .output()
        .expect("run s4_01 census");
    assert!(
        out.status.success(),
        "census script failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    for key in [
        "stage4_open_helper=",
        "s4_01_census_helper=",
        "stability_ssot_helper=",
        "residual_scope_helper=",
        "s4_01_claim_helper=",
        "stability_ssot_package_rows=",
        "packages_readme_status_rows=",
        "manifest_stability_field_count=",
        "syx_doctest_subcommand_count=",
        "stage4_status=",
        "s4_01_status=",
        "stage4_checklist=",
    ] {
        assert!(
            stdout.contains(key),
            "expected {key} in:\n{stdout}"
        );
    }
    assert!(
        stdout.contains("current=S3-I91") && stdout.contains("next=S3-I93"),
        "ladder I91→I93:\n{stdout}"
    );
    assert!(
        stdout.contains("stage4_status=x")
            && stdout.contains("s4_01_status=x")
            && stdout.contains("stage4_checklist=4/5"),
        "claimed status / 4/5:\n{stdout}"
    );
    let parse = |key: &str| {
        stdout
            .lines()
            .find(|l| l.starts_with(key))
            .and_then(|l| l.split('=').nth(1))
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(999)
    };
    assert!(parse("stage4_open_helper=") >= 1);
    assert!(parse("s4_01_census_helper=") >= 1);
    assert!(parse("stability_ssot_helper=") >= 1);
    assert!(parse("residual_scope_helper=") >= 1);
    assert!(parse("s4_01_claim_helper=") >= 1);
    assert!(parse("stability_ssot_package_rows=") >= 19);
    assert!(parse("packages_readme_status_rows=") >= 1);
    assert_eq!(parse("manifest_stability_field_count="), 0);
    assert_eq!(parse("syx_doctest_subcommand_count="), 0);
    assert_eq!(parse("packages_readme_plan_ternary_preview="), 0);
    assert!(parse("doctest_annotations_doc_helper=") >= 1);
}

#[test]
fn s4_01_open_does_not_exit_stage4() {
    let adr = repo_file("docs/adr/stage4-open.md");
    assert!(
        adr.contains("Stage-4 `[~]`"),
        "open ADR Status must keep Stage-4 [x] 退出"
    );
    let census = repo_file("docs/census/stage4-stdlib-stability-doctest-census.md");
    assert!(
        census.contains("4/5"),
        "census must publish Stage-4 5/5 after S4-01 claim"
    );
    let claim = repo_file("docs/adr/stage4-s4-01-claim.md");
    assert!(
        claim.contains("S4-01 `[x]`")
            && claim.contains("Stage-4 `[~]`")
            && (claim.contains("≠ Stage-4") || claim.contains("≠") && claim.contains("Stage-4")),
        "claim ADR must keep Stage-4 [x] 退出 and Non-claim exit"
    );
}
