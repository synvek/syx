//! S3-I76 S3-05 residual-scope (scoped acceptance + deferred hybrid) contracts.
//! Hinge landed · post-I77: S3-05 [x] · Stage-3 still [~] 7/7 · ≠ Stage-3 exit.
//! See docs/adr/stage3-s3-05-residual-scope.md · docs/census/stage3-lint-ast-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_05_residual_scope_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-05-residual-scope.md");
    assert!(adr.contains("S3-I76"), "residual-scope ADR must name S3-I76");
    assert!(
        adr.contains("scoped acceptance")
            || adr.contains("Scoped acceptance")
            || adr.contains("scoped"),
        "ADR must publish scoped acceptance"
    );
    assert!(
        (adr.contains("deferred") || adr.contains("Deferred"))
            && (adr.contains("supersede") || adr.contains("superseding")),
        "ADR must defer hybrid / supersede-only"
    );
    assert!(
        adr.contains("L010") || adr.contains("L011") || adr.contains("hybrid"),
        "ADR must name deferred hybrid axes"
    );
    assert!(
        adr.contains("S3-I77")
            && (adr.contains("claim") || adr.contains("stage3-s3-05-claim") || adr.contains("[x]")),
        "ADR must cite landed S3-I77 claim"
    );
    assert!(
        adr.contains("≠ S3-05 `[x]`") || adr.contains("≠ S3-05 [x]"),
        "ADR must Non-claim S3-05 [x] this window"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-3"),
        "ADR must Non-claim Stage-3 exit"
    );

    let census = repo_file("docs/census/stage3-lint-ast-census.md");
    assert!(
        census.contains("Current:** S3-I77") || census.contains("Current: S3-I77"),
        "census Current must be S3-I77"
    );
    assert!(
        census.contains("residual_scope_helper")
            && (census.contains("scoped") || census.contains("Deferred")),
        "census must publish I76 KPIs / scoped goal"
    );
    assert!(
        census.contains("S3-I80") && census.contains("S3-I81"),
        "census ladder must name S3-I80 / S3-I81"
    );
    assert!(census.contains("7/7"), "census must publish Stage-3 7/7");
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );

    let prior = repo_file("docs/adr/stage3-s3-05-claim-prep.md");
    assert!(
        prior.contains("S3-I76") || prior.contains("stage3-s3-05-residual-scope"),
        "I75 ADR must cite landed I76"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s305 = plan02
        .lines()
        .find(|l| l.contains("| S3-05 |"))
        .expect("S3-05 row");
    assert!(
        s305.contains("| [x] |") || s305.contains("| `[x]` |"),
        "S3-05 must be [x] after I77 claim; got: {s305}"
    );
    assert!(
        s305.contains("scoped")
            || s305.contains("residual-scope")
            || s305.contains("deferred")
            || s305.contains("Deferred"),
        "S3-05 row must publish scoped acceptance; got: {s305}"
    );
    assert!(
        plan02.contains("S3-I76") || plan02.contains("residual-scope"),
        "plan/02 must cite I76"
    );
    assert!(
        plan02.contains("S3-I80") && plan02.contains("S3-I81"),
        "plan/02 must cite I80 / I81"
    );
    assert!(plan02.contains("7/7"), "plan/02 must publish Stage-3 7/7");

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I77") && plan01.contains("7/7"),
        "plan/01 must cite I77 / 7/7"
    );
}

#[test]
fn s3_05_residual_scope_census_script_emits_hinge() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = std::process::Command::new("bash")
        .arg("scripts/s3_05_lint_ast_census.sh")
        .current_dir(&root)
        .output()
        .expect("run s3_05_lint_ast_census.sh");
    assert!(
        out.status.success(),
        "census script failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("current=S3-I77") && stdout.contains("next=S3-I89"),
        "script ladder must be I77→I89:\n{stdout}"
    );
    assert!(
        stdout.contains("residual_scope_helper="),
        "script must emit I76 hinge:\n{stdout}"
    );
    let parse = |key: &str| {
        stdout
            .lines()
            .find(|l| l.starts_with(key))
            .and_then(|l| l.split('=').nth(1))
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(0)
    };
    assert!(
        parse("residual_scope_helper=") >= 1,
        "residual_scope_helper must be >=1\n{stdout}"
    );
    assert_eq!(parse("class_demo_l007_count="), 0);
    assert_eq!(parse("class_demo_l012_count="), 0);
    assert_eq!(parse("struct_book_point_l007_count="), 0);
}
