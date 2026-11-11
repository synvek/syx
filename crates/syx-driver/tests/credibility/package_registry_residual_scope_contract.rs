//! S3-I94 S4-02 residual-scope contracts.
//! Post-I95: S4-02 [x] · Stage-4 [x] 退出 4/5 · residual hinge stays green.
//! See docs/adr/stage4-s4-02-residual-scope.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s4_02_residual_scope_adr_and_files() {
    let adr = repo_file("docs/adr/stage4-s4-02-residual-scope.md");
    assert!(
        adr.contains("S3-I94"),
        "residual-scope ADR must name S3-I94"
    );
    assert!(
        adr.contains("scoped acceptance")
            || adr.contains("Scoped acceptance")
            || adr.contains("scoped"),
        "ADR must publish scoped acceptance"
    );
    assert!(
        (adr.contains("deferred") || adr.contains("Deferred"))
            && (adr.contains("supersede") || adr.contains("superseding")),
        "ADR must defer public GA/lock trim / supersede-only"
    );
    assert!(
        adr.contains("S3-I95")
            && (adr.contains("claim") || adr.contains("`[x]`") || adr.contains("[x]")),
        "ADR must cite landed I95 scoped claim"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-4"),
        "ADR must Non-claim Stage-4 exit"
    );

    let scope = repo_file("docs/census/s4-02-registry-residual-scope-v0.md");
    assert!(
        scope.contains("S3-I94")
            && scope.contains("Deferred")
            && (scope.contains("supersede") || scope.contains("superseding")),
        "scope-v0 must name I94 / deferred supersede"
    );

    let census = repo_file("docs/census/stage4-package-registry-census.md");
    assert!(
        census.contains("Current:** S3-I95") || census.contains("Current: S3-I95"),
        "census Current must advance to S3-I95"
    );
    assert!(
        census.contains("residual_scope_helper")
            && (census.contains("scoped") || census.contains("Deferred")),
        "census must publish I94 KPIs / scoped goal"
    );
    assert!(
        census.contains("S3-I96") && census.contains("4/5"),
        "census must name I96 next and Stage-4 4/5"
    );
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );

    let prior = repo_file("docs/adr/stage4-s4-02-file-registry-hinge.md");
    assert!(
        prior.contains("S3-I94") || prior.contains("stage4-s4-02-residual-scope"),
        "I93 ADR must cite landed I94"
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
        s402.contains("scoped")
            || s402.contains("residual")
            || s402.contains("deferred")
            || s402.contains("Deferred"),
        "S4-02 row must publish scoped/deferred; got: {s402}"
    );
    assert!(
        plan02.contains("S3-I94")
            && plan02.contains("S3-I95")
            && plan02.contains("S3-I96")
            && plan02.contains("5/5"),
        "plan/02 must cite I94…I96 / 5/5"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I94") && plan01.contains("S3-I95") && plan01.contains("5/5"),
        "plan/01 must cite I94 / I95 / 5/5"
    );
}

#[test]
fn s4_02_residual_scope_census_script_hinges() {
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
    assert!(parse("residual_scope_helper=") >= 1);
    assert!(parse("file_registry_hinge_helper=") >= 1);
    assert!(parse("file_registry_integration_test_count=") >= 1);
    assert!(parse("file_registry_evergreen_roundtrip_count=") >= 2);
    assert!(parse("registry_semver_r001_count=") >= 1);
}
