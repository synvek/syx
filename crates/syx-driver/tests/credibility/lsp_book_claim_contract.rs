//! S3-I81 S3-06 scoped plan claim contracts.
//! Nails: claim ADR + plan/02 S3-06 [x]; scoped Non-claims; Stage-3 still [~] 7/7.
//! See docs/adr/stage3-s3-06-claim.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_06_plan_claim_locked() {
    let adr = repo_file("docs/adr/stage3-s3-06-claim.md");
    assert!(adr.contains("S3-I81"), "S3-06 claim ADR must exist");
    assert!(
        adr.contains("S3-06") && adr.contains("`[x]`"),
        "ADR Status must claim S3-06 [x]"
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
        adr.contains("S045") || adr.contains("desugar"),
        "ADR Evidence must cite Primary S045 / desugar"
    );
    assert!(
        adr.contains("deferred") || adr.contains("supersede"),
        "ADR must keep rename/sig/fence deferred"
    );
    assert!(
        adr.contains("S3-I80") || adr.contains("residual-scope"),
        "ADR must cite I80 residual-scope prior"
    );
    assert!(
        adr.contains("S3-I82") || adr.contains("stage3-s3-07-open"),
        "ADR must cite landed I82"
    );
    assert!(
        adr.contains("S3-I83") || adr.contains("stage3-s3-07-default-vm"),
        "ADR must cite landed I83"
    );
    assert!(
        adr.contains("S3-I84") || adr.contains("stage3-s3-07-residual-scope"),
        "ADR must cite landed I84"
    );
    assert!(
        adr.contains("S3-I85") || (adr.contains("claim") && adr.contains("S3-07")),
        "ADR must cite S3-I85 claim ladder / I86 next"
    );
    assert!(
        adr.contains("S3-I86") || (adr.contains("exit") && adr.contains("open")),
        "ADR Consequences must lock next = Stage-3 exit open / I86"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s3_06 = plan02
        .lines()
        .find(|l| l.contains("| S3-06 |"))
        .expect("plan/02 must have S3-06 row");
    assert!(
        s3_06.contains("| [x] |") || s3_06.contains("| `[x]` |"),
        "plan/02 S3-06 status must be [x]; got: {s3_06}"
    );
    assert!(
        plan02.contains("S3-I81") || plan02.contains("stage3-s3-06-claim"),
        "plan/02 must cite S3-I81 claim"
    );
    let stage3 = plan02
        .lines()
        .find(|l| l.contains("| **3** |"))
        .expect("plan/02 must retain stage-3 summary row");
    assert!(
        stage3.contains("[x]") && stage3.contains("退出"),
        "Stage-3 must be [x] 退出; got: {stage3}"
    );
    assert!(plan02.contains("7/7"), "plan/02 must publish Stage-3 7/7");
    assert!(
        plan02.contains("S3-I82")
            && plan02.contains("S3-I83")
            && plan02.contains("S3-I84")
            && plan02.contains("S3-I85")
            && plan02.contains("S3-I86")
            && plan02.contains("S3-I87")
            && (plan02.contains("S3-I88") || plan02.contains("Stage-4")),
        "plan/02 must cite I82…I85"
    );

    let census = repo_file("docs/census/stage3-lsp-book-census.md");
    assert!(
        census.contains("S3-I81"),
        "lsp-book census must name S3-I81"
    );
    assert!(
        census.contains("Current:** S3-I81") || census.contains("Current: S3-I81"),
        "census Current must be S3-I81"
    );
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );
    assert!(census.contains("7/7"), "census must publish Stage-3 7/7");
    assert!(
        census.contains("S3-I82")
            && census.contains("S3-I83")
            && census.contains("S3-I84")
            && census.contains("S3-I85")
            && census.contains("S3-I86")
            && census.contains("S3-I87")
            && (census.contains("S3-I88") || census.contains("Stage-4")),
        "census ladder must cite I82…I85"
    );

    let prior = repo_file("docs/adr/stage3-s3-06-residual-scope.md");
    assert!(
        prior.contains("S3-I81") || prior.contains("stage3-s3-06-claim"),
        "I80 ADR must cite landed I81"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I81") && plan01.contains("[x]"),
        "plan/01 must reflect S3-I81 S3-06 [x]"
    );
    assert!(plan01.contains("7/7"), "plan/01 must publish Stage-3 7/7");
    assert!(
        plan01.contains("S3-I82")
            && plan01.contains("S3-I83")
            && plan01.contains("S3-I84")
            && plan01.contains("S3-I85")
            && plan01.contains("S3-I86")
            && plan01.contains("S3-I87")
            && (plan01.contains("S3-I88") || plan01.contains("Stage-4")),
        "plan/01 must cite I82…I85"
    );
}

#[test]
fn s3_06_claim_census_script_hinges() {
    let output = std::process::Command::new("bash")
        .arg("scripts/s3_06_lsp_book_census.sh")
        .current_dir(std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."))
        .output()
        .expect("run lsp-book census");
    assert!(
        output.status.success(),
        "census script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    for key in [
        "s3_06_claim_helper=",
        "residual_scope_helper=",
        "s045_lsp_desugar_nail_count=",
        "book_lsp_semantic_mismatch_count=",
        "s3_06_status=",
        "stage3_checklist=",
    ] {
        assert!(
            stdout.contains(key),
            "expected script to emit {key}, got:\n{stdout}"
        );
    }
    assert!(
        stdout.contains("current=S3-I81") && stdout.contains("next=S3-I89"),
        "script ladder must be I81→I89:\n{stdout}"
    );
    assert!(
        stdout.contains("s3_06_status=x") && stdout.contains("stage3_checklist=7/7"),
        "script must emit claimed status / 7/7:\n{stdout}"
    );
    let claim = stdout
        .lines()
        .find(|l| l.starts_with("s3_06_claim_helper="))
        .and_then(|l| l.split('=').nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    assert!(
        claim >= 1,
        "expected s3_06_claim_helper>=1, got {claim}\n{stdout}"
    );
    let parse = |key: &str| {
        stdout
            .lines()
            .find(|l| l.starts_with(key))
            .and_then(|l| l.split('=').nth(1))
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(999)
    };
    assert_eq!(parse("s045_lsp_desugar_nail_count="), 0);
    assert_eq!(parse("book_lsp_semantic_mismatch_count="), 0);
}
