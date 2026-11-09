//! S3-I75 S3-05 claim-prep contracts.
//! Scaffolding hinge · post-I77: S3-05 [x] · Stage-3 still [~] 7/7.
//! See docs/adr/stage3-s3-05-claim-prep.md · docs/census/stage3-lint-ast-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn repo_path(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

#[test]
fn s3_05_claim_prep_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-05-claim-prep.md");
    assert!(adr.contains("S3-I75"), "claim-prep ADR must name S3-I75");
    assert!(
        adr.contains("Primary") && (adr.contains("判据") || adr.contains("criteria") || adr.contains("draft")),
        "ADR must publish claim criteria draft"
    );
    assert!(
        adr.contains("Deferred")
            && (adr.contains("L010") || adr.contains("L011") || adr.contains("hybrid")),
        "ADR must cite Deferred hybrid residual"
    );
    assert!(
        adr.contains("S3-I76") && adr.contains("residual"),
        "ADR must cite landed S3-I76 residual-scope"
    );
    assert!(
        adr.contains("S3-I77") && (adr.contains("claim") || adr.contains("`[x]`")),
        "ADR must cite landed S3-I77 claim"
    );
    assert!(
        adr.contains("≠ S3-05 `[x]`") || adr.contains("≠ S3-05 [x]"),
        "ADR must Non-claim S3-05 [x] for that window"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-3"),
        "ADR must Non-claim Stage-3 exit"
    );

    let census = repo_file("docs/census/stage3-lint-ast-census.md");
    assert!(
        census.contains("Current:** S3-I77") || census.contains("Current: S3-I77"),
        "census Current must advance to S3-I77"
    );
    assert!(
        census.contains("claim_prep_helper") && census.contains("scoped_fixture_helper"),
        "census must publish I75 KPIs"
    );
    assert!(
        census.contains("S3-I76") && census.contains("S3-I77") && census.contains("S3-I78"),
        "census ladder must name I76 / I77 / I78"
    );
    assert!(census.contains("7/7"), "census must publish Stage-3 7/7");
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );

    assert!(
        repo_path("examples/class_demo.sy").is_file(),
        "examples/class_demo.sy must exist"
    );
    assert!(
        repo_path("examples/struct_demo.sy").is_file(),
        "examples/struct_demo.sy must exist"
    );
    assert!(
        repo_path("crates/syx-driver/tests/fixtures/lang/struct_book_point.sy").is_file(),
        "struct_book_point.sy must exist"
    );

    let prior = repo_file("docs/adr/stage3-s3-05-struct-extension-type-member.md");
    assert!(
        prior.contains("S3-I75") || prior.contains("stage3-s3-05-claim-prep"),
        "I74 ADR must cite landed I75"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s305 = plan02
        .lines()
        .find(|l| l.contains("| S3-05 |"))
        .expect("S3-05 row");
    assert!(
        s305.contains("| [x] |") || s305.contains("| `[x]` |"),
        "S3-05 must be [x] after claim; got: {s305}"
    );
    assert!(
        plan02.contains("S3-I75") && plan02.contains("S3-I76") && plan02.contains("S3-I77"),
        "plan/02 must cite I75 / I76 / I77"
    );
    assert!(plan02.contains("7/7"), "plan/02 must publish Stage-3 7/7");

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I77") && plan01.contains("7/7"),
        "plan/01 must cite I77 / 7/7"
    );
}

#[test]
fn s3_05_claim_prep_census_script_emits_hinge() {
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
        stdout.contains("claim_prep_helper=") && stdout.contains("scoped_fixture_helper="),
        "script must emit I75 hinges:\n{stdout}"
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
        parse("claim_prep_helper=") >= 1,
        "claim_prep_helper must be >=1\n{stdout}"
    );
    assert!(
        parse("scoped_fixture_helper=") >= 3,
        "scoped_fixture_helper must be >=3\n{stdout}"
    );
    assert_eq!(parse("class_demo_l007_count="), 0);
    assert_eq!(parse("class_demo_l012_count="), 0);
    assert_eq!(parse("struct_book_point_l007_count="), 0);
}
