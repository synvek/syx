//! S3-I91 S4-01 scoped plan claim contracts.
//! Nails: claim ADR + plan/02 S4-01 [x]; scoped Non-claims; Stage-4 still [~] 1/5.
//! See docs/adr/stage4-s4-01-claim.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s4_01_plan_claim_locked() {
    let adr = repo_file("docs/adr/stage4-s4-01-claim.md");
    assert!(adr.contains("S3-I91"), "S4-01 claim ADR must exist");
    assert!(
        adr.contains("S4-01") && adr.contains("`[x]`"),
        "ADR Status must claim S4-01 [x]"
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
        adr.contains("stability.toml") || adr.contains("stability_ssot"),
        "ADR Evidence must cite Primary SSOT"
    );
    assert!(
        adr.contains("deferred") || adr.contains("supersede"),
        "ADR must keep manifest/doctest deferred"
    );
    assert!(
        adr.contains("S3-I90") || adr.contains("residual-scope"),
        "ADR must cite I90 residual-scope prior"
    );
    assert!(
        adr.contains("S3-I92")
            || (adr.contains("S4-02") && adr.contains("open")),
        "ADR must cite S3-I92 S4-02 open ladder"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s401 = plan02
        .lines()
        .find(|l| l.contains("| S4-01 |"))
        .expect("plan/02 must have S4-01 row");
    assert!(
        s401.contains("| [x] |") || s401.contains("| `[x]` |"),
        "plan/02 S4-01 status must be [x]; got: {s401}"
    );
    assert!(
        plan02.contains("S3-I91") || plan02.contains("stage4-s4-01-claim"),
        "plan/02 must cite S3-I91 claim"
    );
    let stage4 = plan02
        .lines()
        .find(|l| l.contains("| **4** |"))
        .expect("plan/02 must retain stage-4 summary row");
    assert!(
        stage4.contains("[~]") || stage4.contains("`[~]`"),
        "Stage-4 must stay [~]; got: {stage4}"
    );
    assert!(
        plan02.contains("1/5"),
        "plan/02 must publish Stage-4 1/5"
    );
    assert!(
        plan02.contains("S3-I92")
            || (plan02.contains("S4-02") && plan02.contains("open")),
        "plan/02 must cite I92 S4-02 open"
    );

    let census = repo_file("docs/stage4-stdlib-stability-doctest-census.md");
    assert!(
        census.contains("S3-I91"),
        "census must name S3-I91"
    );
    assert!(
        census.contains("Current:** S3-I91") || census.contains("Current: S3-I91"),
        "census Current must be S3-I91"
    );
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );
    assert!(
        census.contains("1/5"),
        "census must publish Stage-4 1/5"
    );
    assert!(
        census.contains("S3-I92")
            || (census.contains("S4-02") && census.contains("open")),
        "census ladder must cite I92 S4-02 open"
    );

    let prior = repo_file("docs/adr/stage4-s4-01-residual-scope.md");
    assert!(
        prior.contains("S3-I91") || prior.contains("stage4-s4-01-claim"),
        "I90 ADR must cite landed I91"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I91") && plan01.contains("[x]"),
        "plan/01 must reflect S3-I91 S4-01 [x]"
    );
    assert!(
        plan01.contains("1/5"),
        "plan/01 must publish Stage-4 1/5"
    );
    assert!(
        plan01.contains("S3-I92")
            || (plan01.contains("S4-02") && plan01.contains("open")),
        "plan/01 must lock next = S4-02 open / I92"
    );
}

#[test]
fn s4_01_claim_census_script_hinges() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s4_01_stdlib_stability_doctest_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run s4_01 census");
    assert!(
        output.status.success(),
        "census script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    for key in [
        "s4_01_claim_helper=",
        "residual_scope_helper=",
        "stability_ssot_helper=",
        "stability_ssot_package_rows=",
        "s4_01_status=",
        "stage4_checklist=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    assert!(
        stdout.contains("current=S3-I91") && stdout.contains("next=S3-I92"),
        "script ladder must be I91→I92:\n{stdout}"
    );
    assert!(
        stdout.contains("s4_01_status=x")
            && stdout.contains("stage4_status=~")
            && stdout.contains("stage4_checklist=1/5"),
        "script must emit claimed status / 1/5:\n{stdout}"
    );
    let claim = stdout
        .lines()
        .find(|l| l.starts_with("s4_01_claim_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        claim >= 1,
        "expected s4_01_claim_helper>=1, got {claim}\n{stdout}"
    );
    let parse = |key: &str| {
        stdout
            .lines()
            .find(|l| l.starts_with(key))
            .and_then(|l| l.split('=').nth(1))
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(999)
    };
    assert!(parse("stability_ssot_package_rows=") >= 19);
    assert!(parse("stability_ssot_helper=") >= 1);
    assert!(parse("residual_scope_helper=") >= 1);
    assert_eq!(parse("manifest_stability_field_count="), 0);
    assert_eq!(parse("syx_doctest_subcommand_count="), 0);
}
