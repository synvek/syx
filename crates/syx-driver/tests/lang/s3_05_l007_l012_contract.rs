//! S3-I73 S3-05 L007/L012 false-positive knife contracts.
//! Hinge landed · post-I77: S3-05 [x] · Stage-3 still [~] 7/7.
//! See docs/adr/stage3-s3-05-l007-l012-false-positive.md · docs/stage3-lint-ast-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_05_l007_l012_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-05-l007-l012-false-positive.md");
    assert!(adr.contains("S3-I73"), "construct ADR must name S3-I73");
    assert!(
        adr.contains("in_type_member") || adr.contains("L007"),
        "ADR must name L007 type-member hinge"
    );
    assert!(
        adr.contains("starts_with(\"print(\"") || adr.contains("column-0") || adr.contains("L012"),
        "ADR must name L012 column-0 hinge"
    );
    assert!(
        adr.contains("S3-I74"),
        "ADR must cite S3-I74 ladder"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-3"),
        "ADR must Non-claim Stage-3 exit"
    );
    assert!(
        adr.contains("≠ S3-05 `[x]`") || adr.contains("≠ S3-05 [x]"),
        "ADR must Non-claim S3-05 [x] for that window"
    );

    let census = repo_file("docs/stage3-lint-ast-census.md");
    assert!(
        census.contains("Current:** S3-I77") || census.contains("Current: S3-I77"),
        "census Current must advance to S3-I77"
    );
    assert!(
        census.contains("S3-I73") && census.contains("class_demo_l007_count"),
        "census must retain I73 / class_demo KPIs"
    );
    assert!(census.contains("7/7"), "census must publish Stage-3 7/7");

    let lint = repo_file("crates/syx-lint/src/lib.rs");
    assert!(
        lint.contains("in_type_member") && lint.contains("S3-I73"),
        "syx-lint must mount in_type_member L007 guard"
    );
    assert!(
        lint.contains("line.starts_with(\"print(\")"),
        "syx-lint L012 must use column-0 starts_with"
    );
    assert!(
        !lint.contains("trim_start().starts_with(\"print(\")"),
        "syx-lint must not use trim_start for L012"
    );

    let prior = repo_file("docs/adr/stage3-s3-05-open.md");
    assert!(
        prior.contains("S3-I73")
            || prior.contains("stage3-s3-05-l007-l012-false-positive"),
        "open ADR must cite landed I73"
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
        plan02.contains("S3-I73") && plan02.contains("S3-I74"),
        "plan/02 must cite I73 and I74"
    );
    assert!(plan02.contains("7/7"), "plan/02 must publish Stage-3 7/7");
}

#[test]
fn s3_05_l007_l012_census_script_cleared() {
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
        stdout.contains("current=S3-I77") && stdout.contains("next=S3-I86"),
        "script ladder must be I77→I86:\n{stdout}"
    );
    assert!(
        stdout.contains("l007_l012_knife_helper="),
        "script must emit I73 hinge:\n{stdout}"
    );
    let l007 = stdout
        .lines()
        .find(|l| l.starts_with("class_demo_l007_count="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(999);
    let l012 = stdout
        .lines()
        .find(|l| l.starts_with("class_demo_l012_count="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(999);
    assert_eq!(l007, 0, "class_demo L007 must stay 0");
    assert_eq!(l012, 0, "class_demo L012 must stay 0");
}
