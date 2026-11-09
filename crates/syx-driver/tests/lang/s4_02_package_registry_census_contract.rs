//! S3-I92…I95 Stage-4 / S4-02 package-registry census contracts.
//! Open + hinge + residual + claim · Stage-4 [x] 退出 · S4-02 [x] · 4/5.
//! See docs/adr/stage4-s4-02-open.md · docs/stage4-package-registry-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s4_02_package_registry_census_baseline() {
    let census = repo_file("docs/stage4-package-registry-census.md");
    assert!(census.contains("S3-I92"), "census must name S3-I92");
    assert!(census.contains("S3-I93"), "census must name S3-I93");
    assert!(census.contains("S3-I94"), "census must name S3-I94");
    assert!(census.contains("S3-I95"), "census must name S3-I95");
    assert!(census.contains("S3-I96"), "census must name S3-I96 next");
    assert!(
        census.contains("Current:** S3-I95") || census.contains("Current: S3-I95"),
        "census Current must advance to S3-I95"
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
        census.contains("add")
            && census.contains("publish")
            && (census.contains("lock") || census.contains("semver")),
        "census must cover plan five-verb axes"
    );
    assert!(
        census.contains("file_registry_hinge_helper") || census.contains("file://"),
        "census must publish file-registry hinge"
    );
    assert!(
        census.contains("residual_scope_helper") || census.contains("residual-scope"),
        "census must publish residual hinge"
    );
    assert!(
        census.contains("s4_02_claim_helper") || census.contains("claim"),
        "census must publish claim hinge"
    );

    let adr = repo_file("docs/adr/stage4-s4-02-open.md");
    assert!(adr.contains("S3-I92"), "open ADR must name S3-I92");
    assert!(
        adr.contains("S3-I95") || adr.contains("stage4-s4-02-claim"),
        "open ADR Consequences must cite landed I95"
    );
    assert!(
        adr.contains("S3-I96") || adr.contains("S4-03"),
        "open ADR Consequences must lock next = I96"
    );
    assert!(
        adr.contains("S3-I91") || adr.contains("stage4-s4-01-claim"),
        "ADR must cite prior S4-01 claim"
    );

    let script = repo_file("scripts/s4_02_package_registry_census.sh");
    assert!(
        script.contains("s4_02_open_helper")
            && script.contains("file_registry_hinge_helper")
            && script.contains("residual_scope_helper")
            && script.contains("s4_02_claim_helper")
            && script.contains("syx_add_helper")
            && script.contains("syx_lock_helper")
            && script.contains("registry_source_helper")
            && script.contains("semver_dep_helper")
            && script.contains("current=S3-I95")
            && script.contains("next=S3-I96"),
        "census script must emit I95 ladder / KPIs"
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
    let s402 = plan02
        .lines()
        .find(|l| l.contains("| S4-02 |"))
        .expect("plan/02 S4-02 row");
    assert!(
        s402.contains("| [x] |") || s402.contains("| `[x]` |") || s402.contains("[x]"),
        "S4-02 must be [x] after I95; got: {s402}"
    );
    assert!(
        plan02.contains("S3-I92")
            && plan02.contains("S3-I93")
            && plan02.contains("S3-I94")
            && plan02.contains("S3-I95")
            && plan02.contains("S3-I96"),
        "plan/02 must cite I92…I96"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I95") && plan01.contains("5/5"),
        "plan/01 must cite I95 / 5/5"
    );
    assert!(
        plan01.contains("S4-02") && plan01.contains("[x]"),
        "plan/01 must reflect S4-02 [x]"
    );
    assert!(
        plan01.contains("S3-I96") || plan01.contains("S4-03"),
        "plan/01 must lock next = I96"
    );

    let prior = repo_file("docs/adr/stage4-s4-01-claim.md");
    assert!(
        prior.contains("S3-I92") || prior.contains("stage4-s4-02-open"),
        "I91 ADR must cite landed I92"
    );
}

#[test]
fn s4_02_package_registry_census_script_runs() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = std::process::Command::new("bash")
        .arg("scripts/s4_02_package_registry_census.sh")
        .current_dir(&root)
        .output()
        .expect("run s4_02 census");
    assert!(
        out.status.success(),
        "census script failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    for key in [
        "s4_02_open_helper=",
        "s4_02_census_helper=",
        "file_registry_hinge_helper=",
        "residual_scope_helper=",
        "s4_02_claim_helper=",
        "file_registry_integration_test_count=",
        "file_registry_evergreen_roundtrip_count=",
        "registry_semver_r001_count=",
        "syx_add_helper=",
        "syx_update_helper=",
        "syx_publish_helper=",
        "syx_lock_helper=",
        "registry_source_helper=",
        "semver_dep_helper=",
        "stage4_status=",
        "s4_02_status=",
        "stage4_checklist=",
    ] {
        assert!(
            stdout.contains(key),
            "expected {key} in:\n{stdout}"
        );
    }
    assert!(
        stdout.contains("current=S3-I95") && stdout.contains("next=S3-I96"),
        "ladder I95→I96:\n{stdout}"
    );
    assert!(
        stdout.contains("stage4_status=x")
            && stdout.contains("s4_01_status=x")
            && stdout.contains("s4_02_status=x")
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
    assert!(parse("s4_02_open_helper=") >= 1);
    assert!(parse("s4_02_census_helper=") >= 1);
    assert!(parse("file_registry_hinge_helper=") >= 1);
    assert!(parse("residual_scope_helper=") >= 1);
    assert!(parse("s4_02_claim_helper=") >= 1);
    assert!(parse("file_registry_integration_test_count=") >= 1);
    assert!(parse("file_registry_evergreen_roundtrip_count=") >= 2);
    assert!(parse("registry_semver_r001_count=") >= 1);
    assert!(parse("syx_add_helper=") >= 1);
    assert!(parse("syx_update_helper=") >= 1);
    assert!(parse("syx_publish_helper=") >= 1);
    assert!(parse("syx_lock_helper=") >= 1);
    assert!(parse("registry_source_helper=") >= 1);
    assert!(parse("semver_dep_helper=") >= 1);
}

#[test]
fn s4_02_open_does_not_exit_stage4() {
    let claim = repo_file("docs/adr/stage4-s4-02-claim.md");
    assert!(
        claim.contains("S4-02") && claim.contains("`[x]`"),
        "claim ADR Status must keep S4-02 [x]"
    );
    assert!(
        claim.contains("≠") && claim.contains("Stage-4"),
        "claim ADR must Non-claim Stage-4 exit"
    );
    let census = repo_file("docs/stage4-package-registry-census.md");
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );
    assert!(
        census.contains("4/5"),
        "census must keep Stage-4 4/5"
    );
    let residual = repo_file("docs/adr/stage4-s4-02-residual-scope.md");
    assert!(
        residual.contains("S3-I95") || residual.contains("stage4-s4-02-claim"),
        "residual ADR must cite landed I95 claim"
    );
}
