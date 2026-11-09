//! S3-I90 S4-01 residual-scope contracts (post-I91 claim).
//! Hinge landed · post-I91: S4-01 [x] · Stage-4 [x] 退出 4/5 · ≠ product knife.
//! See docs/adr/stage4-s4-01-residual-scope.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s4_01_residual_scope_adr_and_files() {
    let adr = repo_file("docs/adr/stage4-s4-01-residual-scope.md");
    assert!(adr.contains("S3-I90"), "residual-scope ADR must name S3-I90");
    assert!(
        adr.contains("scoped acceptance")
            || adr.contains("Scoped acceptance")
            || adr.contains("scoped"),
        "ADR must publish scoped acceptance"
    );
    assert!(
        (adr.contains("deferred") || adr.contains("Deferred"))
            && (adr.contains("supersede") || adr.contains("superseding")),
        "ADR must defer manifest/doctest / supersede-only"
    );
    assert!(
        adr.contains("manifest") && (adr.contains("doctest") || adr.contains("syx doctest")),
        "ADR must name deferred axes"
    );
    assert!(
        adr.contains("S3-I91")
            && (adr.contains("claim") || adr.contains("`[x]`") || adr.contains("[x]")),
        "ADR must cite I91 scoped claim"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-4"),
        "ADR must Non-claim Stage-4 exit"
    );

    let scope = repo_file("docs/s4-01-manifest-doctest-scope-v0.md");
    assert!(
        scope.contains("S3-I90")
            && scope.contains("Deferred")
            && (scope.contains("supersede") || scope.contains("superseding")),
        "scope-v0 must name I90 / deferred supersede"
    );

    let census = repo_file("docs/stage4-stdlib-stability-doctest-census.md");
    assert!(
        census.contains("Current:** S3-I91") || census.contains("Current: S3-I91"),
        "census Current must advance to S3-I91"
    );
    assert!(
        census.contains("residual_scope_helper")
            && (census.contains("scoped") || census.contains("Deferred")),
        "census must publish I90 KPIs / scoped goal"
    );
    assert!(
        census.contains("S3-I92") && census.contains("4/5"),
        "census must name I92 next and Stage-4 4/5"
    );
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );

    let prior = repo_file("docs/adr/stage4-s4-01-stability-ssot.md");
    assert!(
        prior.contains("S3-I90") || prior.contains("stage4-s4-01-residual-scope"),
        "I89 ADR must cite landed I90"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s401 = plan02
        .lines()
        .find(|l| l.contains("| S4-01 |"))
        .expect("S4-01 row");
    assert!(
        s401.contains("| [x] |") || s401.contains("| `[x]` |") || s401.contains("[x]"),
        "S4-01 must be [x] after I91 claim; got: {s401}"
    );
    assert!(
        s401.contains("scoped")
            || s401.contains("residual")
            || s401.contains("deferred")
            || s401.contains("Deferred"),
        "S4-01 row must publish scoped/deferred; got: {s401}"
    );
    assert!(
        plan02.contains("S3-I90")
            && plan02.contains("S3-I91")
            && plan02.contains("S3-I92")
            && plan02.contains("5/5"),
        "plan/02 must cite I90…I92 / 5/5"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I90")
            && plan01.contains("S3-I91")
            && plan01.contains("S3-I92")
            && plan01.contains("5/5"),
        "plan/01 must cite I90…I92 / 5/5"
    );
}

#[test]
fn s4_01_residual_scope_census_script_hinges() {
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
    assert!(
        stdout.contains("residual_scope_helper="),
        "script must emit I90 hinge:\n{stdout}"
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
    assert!(parse("stability_ssot_helper=") >= 1);
    assert!(parse("stability_ssot_package_rows=") >= 19);
    assert_eq!(parse("manifest_stability_field_count="), 0);
    assert_eq!(parse("syx_doctest_subcommand_count="), 0);
}
