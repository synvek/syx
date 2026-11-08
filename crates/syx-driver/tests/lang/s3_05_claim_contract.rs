//! S3-I77 S3-05 scoped plan claim contracts.
//! Nails: claim ADR + plan/02 S3-05 [x]; scoped Non-claims; Stage-3 still [~] 7/7.
//! See docs/adr/stage3-s3-05-claim.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_05_plan_claim_locked() {
    let adr = repo_file("docs/adr/stage3-s3-05-claim.md");
    assert!(adr.contains("S3-I77"), "S3-05 claim ADR must exist");
    assert!(
        adr.contains("S3-05") && adr.contains("`[x]`"),
        "ADR Status must claim S3-05 [x]"
    );
    assert!(
        adr.contains("Scoped") || adr.contains("scoped"),
        "ADR must define scoped [x] meaning"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-3"),
        "ADR must Non-claim Stage-3 exit"
    );
    assert!(
        adr.contains("L007") && adr.contains("L012"),
        "ADR Evidence must cite Primary L007/L012"
    );
    assert!(
        adr.contains("deferred") || adr.contains("supersede"),
        "ADR must keep hybrid deferred"
    );
    assert!(
        adr.contains("S3-I76") || adr.contains("residual-scope"),
        "ADR must cite I76 residual-scope prior"
    );
    assert!(
        adr.contains("S3-I78")
            && (adr.contains("S3-I79")
                || adr.contains("S3-06")
                || adr.contains("stage3-s3-06-open")),
        "ADR must cite landed I78 and I79 / S3-06 ladder"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s3_05 = plan02
        .lines()
        .find(|l| l.contains("| S3-05 |"))
        .expect("plan/02 must have S3-05 row");
    assert!(
        s3_05.contains("| [x] |") || s3_05.contains("| `[x]` |"),
        "plan/02 S3-05 status must be [x]; got: {s3_05}"
    );
    assert!(
        plan02.contains("S3-I77") || plan02.contains("stage3-s3-05-claim"),
        "plan/02 must cite S3-I77 claim"
    );
    let stage3 = plan02
        .lines()
        .find(|l| l.contains("| **3** |"))
        .expect("plan/02 must retain stage-3 summary row");
    assert!(
        stage3.contains("[x]") && stage3.contains("退出"),
        "Stage-3 must be [x] 退出; got: {stage3}"
    );
    assert!(
        plan02.contains("7/7"),
        "plan/02 must publish Stage-3 7/7"
    );
    assert!(
        plan02.contains("S3-I80")
            && (plan02.contains("S3-I81") || plan02.contains("S3-06")),
        "plan/02 must cite I80 / I81"
    );

    let census = repo_file("docs/stage3-lint-ast-census.md");
    assert!(
        census.contains("S3-I77"),
        "lint-ast census must name S3-I77"
    );
    assert!(
        census.contains("Current:** S3-I77") || census.contains("Current: S3-I77"),
        "census Current must be S3-I77"
    );
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );
    assert!(
        census.contains("7/7"),
        "census must publish Stage-3 7/7"
    );
    assert!(
        census.contains("S3-I80") && census.contains("S3-I81"),
        "census ladder must cite I80 / I81"
    );

    let prior = repo_file("docs/adr/stage3-s3-05-residual-scope.md");
    assert!(
        prior.contains("S3-I77") || prior.contains("stage3-s3-05-claim"),
        "I76 ADR must cite landed I77"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I77") && plan01.contains("[x]"),
        "plan/01 must reflect S3-I77 S3-05 [x]"
    );
    assert!(
        plan01.contains("7/7"),
        "plan/01 must publish Stage-3 7/7"
    );
    assert!(
        plan01.contains("S3-I80")
            && (plan01.contains("S3-I81") || plan01.contains("S3-06")),
        "plan/01 must cite I80 / I81"
    );
}

#[test]
fn s3_05_claim_census_script_hinges() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_05_lint_ast_census.sh")
        .current_dir(
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        )
        .output()
        .expect("run lint-ast census");
    assert!(
        output.status.success(),
        "census script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    for key in [
        "s3_05_claim_helper=",
        "residual_scope_helper=",
        "claim_prep_helper=",
        "class_demo_l007_count=",
        "s3_05_status=",
        "stage3_checklist=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    assert!(
        stdout.contains("current=S3-I77") && stdout.contains("next=S3-I88"),
        "script ladder must be I77→I88:\n{stdout}"
    );
    assert!(
        stdout.contains("s3_05_status=x") && stdout.contains("stage3_checklist=7/7"),
        "script must emit claimed status / 7/7:\n{stdout}"
    );
    let claim = stdout
        .lines()
        .find(|l| l.starts_with("s3_05_claim_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        claim >= 1,
        "expected s3_05_claim_helper>=1, got {claim}\n{stdout}"
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
