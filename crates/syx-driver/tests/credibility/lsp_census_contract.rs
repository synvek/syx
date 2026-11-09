//! S3-I78…I81 S3-06 LSP-book census contracts.
//! I78 open · I79 S045 desugar · I80 residual · I81 scoped claim · Stage-3 still [~] 7/7.
//! See docs/adr/stage3-s3-06-open.md · docs/census/stage3-lsp-book-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_06_lsp_census_baseline() {
    let census = repo_file("docs/census/stage3-lsp-book-census.md");
    assert!(census.contains("S3-I78"), "census must name S3-I78 open");
    assert!(census.contains("S3-I79"), "census must name S3-I79");
    assert!(census.contains("S3-I80"), "census must name S3-I80");
    assert!(census.contains("S3-I81"), "census must name S3-I81");
    assert!(census.contains("S3-I82"), "census must name S3-I82");
    assert!(census.contains("S3-I83"), "census must name S3-I83");
    assert!(census.contains("S3-I84"), "census must name S3-I84");
    assert!(
        census.contains("S3-I85")
            && census.contains("S3-I86")
            && census.contains("S3-I87")
            && (census.contains("S3-I88") || census.contains("Stage-4")),
        "census must name I85…I86 landed / I87 next"
    );
    assert!(
        census.contains("Current:** S3-I81") || census.contains("Current: S3-I81"),
        "census Current must be S3-I81"
    );
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );
    assert!(
        census.contains("stage3_lsp_census_helper")
            && census.contains("s045_lsp_desugar_nail_count")
            && census.contains("residual_scope_helper")
            && census.contains("s3_06_claim_helper")
            && census.contains("rename_provider_helper"),
        "census must publish I79–I81 KPIs"
    );
    assert!(
        census.contains("≠") && census.contains("Stage-3"),
        "census must Non-claim Stage-3 exit"
    );
    assert!(census.contains("7/7"), "census must publish Stage-3 7/7");

    let open_adr = repo_file("docs/adr/stage3-s3-06-open.md");
    assert!(open_adr.contains("S3-I78"), "open ADR must name S3-I78");
    assert!(
        open_adr.contains("S3-06 `[~]`"),
        "open ADR Status must keep historical S3-06 [~]"
    );
    assert!(
        open_adr.contains("≠ S3-06 `[x]`") || open_adr.contains("≠ S3-06 [x]"),
        "open ADR must Non-claim S3-06 [x] for that window"
    );
    assert!(
        open_adr.contains("S3-I77") || open_adr.contains("stage3-s3-05-claim"),
        "open ADR must cite prior S3-I77 claim"
    );
    assert!(
        open_adr.contains("S3-I81") || open_adr.contains("stage3-s3-06-claim"),
        "open ADR must cite landed S3-I81 claim"
    );

    let script = repo_file("scripts/s3_06_lsp_book_census.sh");
    assert!(
        script.contains("stage3_lsp_census_helper")
            && script.contains("s045_lsp_desugar_helper")
            && script.contains("residual_scope_helper")
            && script.contains("s3_06_claim_helper")
            && script.contains("rename_provider_helper"),
        "census script must emit I79–I81 KPIs"
    );
    assert!(
        script.contains("current=S3-I81") && script.contains("next=S3-I89"),
        "census script must emit ladder current/next"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s306_line = plan02
        .lines()
        .find(|l| l.contains("| S3-06 |"))
        .expect("plan/02 must have S3-06 table row");
    assert!(
        s306_line.contains("| [x] |") || s306_line.contains("| `[x]` |"),
        "plan/02 S3-06 must be [x], got: {s306_line}"
    );
    assert!(
        plan02.contains("S3-I79") && plan02.contains("7/7"),
        "plan/02 must cite S3-I79 and Stage-3 7/7"
    );
    assert!(
        plan02.contains("S3-I80")
            && plan02.contains("S3-I81")
            && plan02.contains("S3-I82")
            && plan02.contains("S3-I83")
            && plan02.contains("S3-I84")
            && plan02.contains("S3-I85")
            && plan02.contains("S3-I86")
            && plan02.contains("S3-I87")
            && (plan02.contains("S3-I88") || plan02.contains("Stage-4")),
        "plan/02 must cite I80…I85"
    );

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I79") && plan01.contains("7/7"),
        "plan/01 must cite S3-I79 and Stage-3 7/7"
    );
    assert!(
        plan01.contains("S3-I80")
            && plan01.contains("S3-I81")
            && plan01.contains("S3-I82")
            && plan01.contains("S3-I83")
            && plan01.contains("S3-I84")
            && plan01.contains("S3-I85")
            && plan01.contains("S3-I86")
            && plan01.contains("S3-I87")
            && (plan01.contains("S3-I88") || plan01.contains("Stage-4")),
        "plan/01 must cite I80…I85"
    );

    let prior = repo_file("docs/adr/stage3-s3-05-claim.md");
    assert!(
        prior.contains("S3-I78") || prior.contains("stage3-s3-06-open"),
        "I77 claim ADR must cite landed I78"
    );
}

#[test]
fn s3_06_lsp_census_script_runs() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let script = root.join("scripts/s3_06_lsp_book_census.sh");
    let out = std::process::Command::new("bash")
        .arg(&script)
        .current_dir(&root)
        .output()
        .expect("run s3_06_lsp_book_census.sh");
    assert!(
        out.status.success(),
        "census script failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("stage3_lsp_census_helper=")
            && stdout.contains("s045_lsp_desugar_helper=")
            && stdout.contains("residual_scope_helper=")
            && stdout.contains("s3_06_claim_helper=")
            && stdout.contains("rename_provider_helper=")
            && stdout.contains("signature_help_helper="),
        "script stdout missing KPIs:\n{stdout}"
    );
    assert!(
        stdout.contains("current=S3-I81") && stdout.contains("next=S3-I89"),
        "script must emit ladder:\n{stdout}"
    );
    assert!(
        stdout.contains("s3_06_status=x") && stdout.contains("stage3_checklist=7/7"),
        "script must emit claimed status / 7/7:\n{stdout}"
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
        parse("stage3_lsp_census_helper=") >= 1,
        "stage3_lsp_census_helper must be >=1\n{stdout}"
    );
    assert!(
        parse("s045_lsp_desugar_helper=") >= 1,
        "s045_lsp_desugar_helper must be >=1\n{stdout}"
    );
    assert!(
        parse("s3_06_claim_helper=") >= 1,
        "s3_06_claim_helper must be >=1\n{stdout}"
    );
    assert_eq!(parse("s045_lsp_desugar_nail_count="), 0);
    assert_eq!(parse("book_lsp_semantic_mismatch_count="), 0);
    assert!(
        parse("book_fixture_sample_n=") >= 3,
        "book_fixture_sample_n must be >=3\n{stdout}"
    );
}

#[test]
fn s3_06_open_does_not_claim_or_exit() {
    let open_adr = repo_file("docs/adr/stage3-s3-06-open.md");
    assert!(
        open_adr.contains("S3-06 `[~]`"),
        "open ADR Status must keep historical S3-06 [~]"
    );
    assert!(
        open_adr.contains("≠ S3-06 `[x]`"),
        "open ADR must Non-claim S3-06 [x] for that window"
    );
    let census = repo_file("docs/census/stage3-lsp-book-census.md");
    assert!(
        census.contains("7/7"),
        "census must publish Stage-3 checklist 7/7"
    );
    let residual = repo_file("docs/adr/stage3-s3-06-residual-scope.md");
    assert!(
        residual.contains("S3-06 `[~]`") && residual.contains("≠ S3-06 `[x]`"),
        "residual-scope ADR must keep historical S3-06 [~] and Non-claim [x]"
    );
}
