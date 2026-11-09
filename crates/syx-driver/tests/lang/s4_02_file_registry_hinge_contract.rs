//! S3-I93 S4-02 file-registry hinge contracts.
//! Post-I95: S4-02 [x] · Stage-4 [x] 退出 4/5 · Primary KPIs stay green.
//! See docs/adr/stage4-s4-02-file-registry-hinge.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s4_02_file_registry_hinge_adr_and_files() {
    let adr = repo_file("docs/adr/stage4-s4-02-file-registry-hinge.md");
    assert!(adr.contains("S3-I93"), "hinge ADR must name S3-I93");
    assert!(
        adr.contains("file://") || adr.contains("file-registry") || adr.contains("file_registry"),
        "ADR must publish file-registry Primary"
    );
    assert!(
        adr.contains("S3-I95") || adr.contains("stage4-s4-02-claim"),
        "ADR Consequences must cite landed I95"
    );

    let charter = repo_file("docs/s4-02-file-registry-v0.md");
    assert!(
        charter.contains("file://") || charter.contains("file-registry"),
        "file-registry-v0 must exist"
    );

    let census = repo_file("docs/stage4-package-registry-census.md");
    assert!(
        census.contains("Current:** S3-I95") || census.contains("Current: S3-I95"),
        "census Current must advance to S3-I95 after claim"
    );
    assert!(
        census.contains("file_registry_hinge_helper") && census.contains("S3-I96"),
        "census must publish I93 KPIs / I96 next"
    );
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );

    let prior = repo_file("docs/adr/stage4-s4-02-open.md");
    assert!(
        prior.contains("S3-I93") || prior.contains("stage4-s4-02-file-registry-hinge"),
        "I92 ADR must cite landed I93"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s402 = plan02
        .lines()
        .find(|l| l.contains("| S4-02 |"))
        .expect("S4-02 row");
    assert!(
        s402.contains("| [x] |") || s402.contains("| `[x]` |") || s402.contains("[x]"),
        "S4-02 must be [x] after I95; got: {s402}"
    );
    assert!(
        plan02.contains("S3-I93")
            && plan02.contains("S3-I94")
            && plan02.contains("S3-I95")
            && plan02.contains("5/5"),
        "plan/02 must cite I93…I95 / 5/5"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I93") && plan01.contains("5/5"),
        "plan/01 must cite I93 / 5/5"
    );
}

#[test]
fn s4_02_file_registry_hinge_census_script_hinges() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = std::process::Command::new("bash")
        .arg("scripts/s4_02_package_registry_census.sh")
        .current_dir(&root)
        .output()
        .expect("run s4_02 census");
    assert!(
        out.status.success(),
        "census failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("current=S3-I95") && stdout.contains("next=S3-I96"),
        "ladder I95→I96:\n{stdout}"
    );
    assert!(
        stdout.contains("stage4_status=x")
            && stdout.contains("s4_02_status=x")
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
    assert!(parse("file_registry_hinge_helper=") >= 1);
    assert!(parse("file_registry_integration_test_count=") >= 1);
    assert!(parse("file_registry_evergreen_roundtrip_count=") >= 2);
    assert!(parse("registry_semver_r001_count=") >= 1);
    assert!(parse("syx_lock_helper=") >= 1);
}
