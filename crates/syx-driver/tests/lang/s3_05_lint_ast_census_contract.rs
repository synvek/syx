//! S3-I72…I76 S3-05 lint-ast census contracts.
//! I72 open · I73–I74 construct · I75 claim-prep · I76 residual-scope · ≠ S3-05 [x].
//! See docs/adr/stage3-s3-05-open.md · docs/stage3-lint-ast-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_05_lint_ast_census_baseline() {
    let census = repo_file("docs/stage3-lint-ast-census.md");
    assert!(census.contains("S3-I72"), "census must name S3-I72 open");
    assert!(census.contains("S3-I73"), "census must name S3-I73");
    assert!(census.contains("S3-I74"), "census must name S3-I74");
    assert!(census.contains("S3-I75"), "census must name S3-I75");
    assert!(census.contains("S3-I76"), "census must name S3-I76");
    assert!(census.contains("S3-I77"), "census must name S3-I77 next");
    assert!(
        census.contains("Current:** S3-I76") || census.contains("Current: S3-I76"),
        "census Current must be S3-I76"
    );
    assert!(
        census.contains("Claimed:** no") || census.contains("Claimed: no"),
        "census must mark Claimed: no"
    );
    assert!(
        census.contains("residual_scope_helper") && census.contains("claim_prep_helper"),
        "census must publish I75/I76 KPIs"
    );
    assert!(
        census.contains("≠") && census.contains("Stage-3"),
        "census must Non-claim Stage-3 exit"
    );
    assert!(census.contains("4/7"), "census must keep Stage-3 4/7");

    let open_adr = repo_file("docs/adr/stage3-s3-05-open.md");
    assert!(open_adr.contains("S3-I72"), "open ADR must name S3-I72");
    assert!(
        open_adr.contains("S3-I76") || open_adr.contains("stage3-s3-05-residual-scope"),
        "open ADR must cite landed S3-I76"
    );

    let script = repo_file("scripts/s3_05_lint_ast_census.sh");
    assert!(
        script.contains("residual_scope_helper"),
        "census script must emit I76 KPI"
    );
    assert!(
        script.contains("current=S3-I76") && script.contains("next=S3-I77"),
        "census script must emit ladder current/next"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s305_line = plan02
        .lines()
        .find(|l| l.contains("| S3-05 |"))
        .expect("plan/02 must have S3-05 table row");
    assert!(
        s305_line.contains("[~]"),
        "plan/02 S3-05 must be [~], got: {s305_line}"
    );
    assert!(
        plan02.contains("S3-I76") && plan02.contains("4/7"),
        "plan/02 must cite S3-I76 and Stage-3 4/7"
    );
    assert!(
        plan02.contains("S3-I77"),
        "plan/02 must lock next default = S3-I77"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I76") && plan01.contains("4/7"),
        "plan/01 must cite S3-I76 and Stage-3 4/7"
    );
    assert!(
        plan01.contains("S3-I77"),
        "plan/01 must lock next = S3-I77"
    );
}

#[test]
fn s3_05_lint_ast_census_script_runs() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let script = root.join("scripts/s3_05_lint_ast_census.sh");
    let out = std::process::Command::new("bash")
        .arg(&script)
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
        stdout.contains("residual_scope_helper=")
            && stdout.contains("claim_prep_helper=")
            && stdout.contains("class_demo_l007_count="),
        "script stdout missing KPIs:\n{stdout}"
    );
    assert!(
        stdout.contains("current=S3-I76") && stdout.contains("next=S3-I77"),
        "script must emit ladder:\n{stdout}"
    );
    let parse = |key: &str| {
        stdout
            .lines()
            .find(|l| l.starts_with(key))
            .and_then(|l| l.split('=').nth(1))
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(999)
    };
    assert_eq!(parse("class_demo_l007_count="), 0);
    assert_eq!(parse("class_demo_l012_count="), 0);
    assert_eq!(parse("struct_book_point_l007_count="), 0);
}

#[test]
fn s3_05_open_does_not_claim_or_exit() {
    let open_adr = repo_file("docs/adr/stage3-s3-05-open.md");
    assert!(
        open_adr.contains("S3-05 `[~]`"),
        "open ADR Status must keep S3-05 [~]"
    );
    assert!(
        open_adr.contains("≠ S3-05 `[x]`"),
        "open ADR must Non-claim S3-05 [x]"
    );
    let census = repo_file("docs/stage3-lint-ast-census.md");
    assert!(
        census.contains("4/7"),
        "census must keep Stage-3 checklist 4/7"
    );
    let residual = repo_file("docs/adr/stage3-s3-05-residual-scope.md");
    assert!(
        residual.contains("S3-05 `[~]`") && residual.contains("≠ S3-05 `[x]`"),
        "residual-scope ADR must keep S3-05 [~] and Non-claim [x]"
    );
}
