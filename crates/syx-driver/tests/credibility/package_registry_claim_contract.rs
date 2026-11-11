//! S3-I95 S4-02 scoped plan claim contracts.
//! Nails: claim ADR + plan/02 S4-02 [x]; scoped Non-claims; Stage-4 [x] 退出 4/5.
//! See docs/adr/stage4-s4-02-claim.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s4_02_plan_claim_locked() {
    let adr = repo_file("docs/adr/stage4-s4-02-claim.md");
    assert!(adr.contains("S3-I95"), "S4-02 claim ADR must exist");
    assert!(
        adr.contains("S4-02") && adr.contains("`[x]`"),
        "ADR Status must claim S4-02 [x]"
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
        adr.contains("file-registry") || adr.contains("file_registry") || adr.contains("file://"),
        "ADR Evidence must cite Primary file-registry hinge"
    );
    assert!(
        adr.contains("deferred") || adr.contains("supersede"),
        "ADR must keep public GA/lock trim deferred"
    );
    assert!(
        adr.contains("S3-I94") || adr.contains("residual-scope"),
        "ADR must cite I94 residual-scope prior"
    );
    assert!(
        adr.contains("S3-I96") || (adr.contains("S4-03") && adr.contains("open")),
        "ADR must cite S3-I96 S4-03 open"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s402 = plan02
        .lines()
        .find(|l| l.contains("| S4-02 |"))
        .expect("plan/02 must have S4-02 row");
    assert!(
        s402.contains("| [x] |") || s402.contains("| `[x]` |"),
        "plan/02 S4-02 status must be [x]; got: {s402}"
    );
    assert!(
        plan02.contains("S3-I95") || plan02.contains("stage4-s4-02-claim"),
        "plan/02 must cite S3-I95 claim"
    );
    let stage4 = plan02
        .lines()
        .find(|l| l.contains("| **4** |"))
        .expect("plan/02 must retain stage-4 summary row");
    assert!(
        stage4.contains("[x]") && stage4.contains("退出"),
        "Stage-4 must be [x] 退出; got: {stage4}"
    );
    assert!(plan02.contains("5/5"), "plan/02 must publish Stage-4 5/5");
    assert!(
        plan02.contains("S3-I96") || plan02.contains("S4-03"),
        "plan/02 must cite I96 / S4-03 open"
    );

    let census = repo_file("docs/census/stage4-package-registry-census.md");
    assert!(census.contains("S3-I95"), "census must name S3-I95");
    assert!(
        census.contains("Current:** S3-I95") || census.contains("Current: S3-I95"),
        "census Current must be S3-I95"
    );
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );
    assert!(census.contains("4/5"), "census must publish Stage-4 5/5");
    assert!(
        census.contains("S3-I96"),
        "census ladder must cite I96 next"
    );

    let prior = repo_file("docs/adr/stage4-s4-02-residual-scope.md");
    assert!(
        prior.contains("S3-I95") || prior.contains("stage4-s4-02-claim"),
        "I94 ADR must cite landed I95"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I95") && plan01.contains("[x]"),
        "plan/01 must reflect S3-I95 S4-02 [x]"
    );
    assert!(plan01.contains("5/5"), "plan/01 must publish Stage-4 5/5");
    assert!(
        plan01.contains("S3-I96") || plan01.contains("S4-03"),
        "plan/01 must cite I96 / S4-03"
    );
}

#[test]
fn s4_02_claim_census_script_hinges() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s4_02_package_registry_census.sh")
        .current_dir(std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .output()
        .expect("run s4_02 census");
    assert!(
        output.status.success(),
        "census script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    for key in [
        "s4_02_claim_helper=",
        "residual_scope_helper=",
        "file_registry_hinge_helper=",
        "file_registry_integration_test_count=",
        "file_registry_evergreen_roundtrip_count=",
        "registry_semver_r001_count=",
        "s4_02_status=",
        "stage4_checklist=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    assert!(
        stdout.contains("current=S3-I95") && stdout.contains("next=S3-I96"),
        "script ladder must be I95→I96:\n{stdout}"
    );
    assert!(
        stdout.contains("s4_02_status=x")
            && stdout.contains("stage4_status=x")
            && stdout.contains("stage4_checklist=4/5"),
        "script must emit claimed status / 4/5:\n{stdout}"
    );
    let claim = stdout
        .lines()
        .find(|l| l.starts_with("s4_02_claim_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        claim >= 1,
        "expected s4_02_claim_helper>=1, got {claim}\n{stdout}"
    );
    let parse = |key: &str| {
        stdout
            .lines()
            .find(|l| l.starts_with(key))
            .and_then(|l| l.split('=').nth(1))
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(999)
    };
    assert!(parse("file_registry_hinge_helper=") >= 1);
    assert!(parse("file_registry_integration_test_count=") >= 1);
    assert!(parse("file_registry_evergreen_roundtrip_count=") >= 2);
    assert!(parse("registry_semver_r001_count=") >= 1);
    assert!(parse("residual_scope_helper=") >= 1);
}
