//! S3-I89 S4-01 stability.toml SSOT contracts.
//! Hinge: plan ternary SSOT + v0 mapping · ≠ S4-01 [x] · ≠ syx doctest.
//! See docs/adr/stage4-s4-01-stability-ssot.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s4_01_stability_ssot_adr_and_files() {
    let adr = repo_file("docs/adr/stage4-s4-01-stability-ssot.md");
    assert!(adr.contains("S3-I89"), "construct ADR must name S3-I89");
    assert!(
        adr.contains("stability.toml") || adr.contains("SSOT"),
        "ADR must name stability SSOT"
    );
    assert!(
        adr.contains("stable") && adr.contains("preview") && adr.contains("experimental"),
        "ADR must name plan ternary"
    );
    assert!(
        adr.contains("≠ S4-01 `[x]`") || adr.contains("≠ S4-01 [x]"),
        "ADR must Non-claim S4-01 [x]"
    );
    assert!(
        adr.contains("S3-I90") || adr.contains("residual"),
        "ADR must lock next = S3-I90 residual-scope"
    );
    assert!(
        adr.contains("S3-I88") || adr.contains("stage4-open"),
        "ADR must cite prior open"
    );

    let ssot = repo_file("packages/internal/stability.toml");
    assert!(
        ssot.contains("syx-stdlib-stability-v0") || ssot.contains("stability"),
        "stability.toml must exist with schema"
    );
    let rows = ssot.matches("[[package]]").count();
    assert!(
        rows >= 19,
        "expected ≥19 [[package]] rows, got {rows}"
    );
    for tier in ["stability = \"stable\"", "stability = \"preview\"", "stability = \"experimental\""] {
        assert!(
            ssot.contains(tier),
            "SSOT must use ternary {tier}"
        );
    }
    assert!(
        ssot.contains("name = \"Testing\"") && ssot.contains("name = \"internal\""),
        "SSOT must include Testing + internal rows"
    );

    let mapping = repo_file("docs/stdlib-stability-v0.md");
    assert!(
        mapping.contains("Stable") && mapping.contains("preview") && mapping.contains("MVP"),
        "v0 mapping must document README→ternary"
    );

    let readme = repo_file("packages/README.md");
    assert!(
        readme.contains("stability.toml") || readme.contains("internal/stability.toml"),
        "README must point at SSOT"
    );

    let prior = repo_file("docs/adr/stage4-open.md");
    assert!(
        prior.contains("S3-I89") || prior.contains("stage4-s4-01-stability-ssot"),
        "I88 ADR must cite landed I89"
    );

    let census = repo_file("docs/stage4-stdlib-stability-doctest-census.md");
    assert!(
        census.contains("Current:** S3-I91") || census.contains("Current: S3-I91"),
        "census Current must advance to S3-I91 after claim"
    );
    assert!(
        census.contains("stability_ssot_helper") && census.contains("S3-I92"),
        "census must publish I89 KPIs / I92 next"
    );
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s401 = plan02
        .lines()
        .find(|l| l.contains("| S4-01 |"))
        .expect("S4-01 row");
    assert!(
        s401.contains("| [x] |") || s401.contains("| `[x]` |"),
        "S4-01 must be [x] after claim; got: {s401}"
    );
    assert!(
        plan02.contains("S3-I89")
            && plan02.contains("S3-I90")
            && plan02.contains("S3-I91")
            && plan02.contains("S3-I92"),
        "plan/02 must cite I89…I92"
    );
    assert!(
        plan02.contains("5/5"),
        "plan/02 must publish Stage-4 5/5"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I89") && plan01.contains("5/5"),
        "plan/01 must cite I89 / 5/5"
    );
}

#[test]
fn s4_01_stability_ssot_census_script_hinges() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = std::process::Command::new("bash")
        .arg("scripts/s4_01_stdlib_stability_doctest_census.sh")
        .current_dir(&root)
        .output()
        .expect("run s4_01 census");
    assert!(
        out.status.success(),
        "census failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("current=S3-I91") && stdout.contains("next=S3-I93"),
        "ladder I91→I93:\n{stdout}"
    );
    assert!(
        stdout.contains("stage4_status=x")
            && stdout.contains("s4_01_status=x")
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
    assert!(parse("stability_ssot_helper=") >= 1);
    assert!(parse("stability_ssot_package_rows=") >= 19);
    assert_eq!(parse("manifest_stability_field_count="), 0);
    assert_eq!(parse("syx_doctest_subcommand_count="), 0);
}
