//! S3-I80 S3-06 residual-scope (scoped acceptance + deferred rename/sig/fence) contracts.
//! Hinge landed · post-I81: S3-06 [x] · Stage-3 still [~] 7/7 · ≠ Stage-3 exit.
//! See docs/adr/stage3-s3-06-residual-scope.md · docs/stage3-lsp-book-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s3_06_residual_scope_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-06-residual-scope.md");
    assert!(adr.contains("S3-I80"), "residual-scope ADR must name S3-I80");
    assert!(
        adr.contains("scoped acceptance")
            || adr.contains("Scoped acceptance")
            || adr.contains("scoped"),
        "ADR must publish scoped acceptance"
    );
    assert!(
        (adr.contains("deferred") || adr.contains("Deferred"))
            && (adr.contains("supersede") || adr.contains("superseding")),
        "ADR must defer rename/sig/fence / supersede-only"
    );
    assert!(
        adr.contains("rename") || adr.contains("signature") || adr.contains("fence"),
        "ADR must name deferred axes"
    );
    assert!(
        adr.contains("S3-I81")
            && (adr.contains("claim") || adr.contains("`[x]`") || adr.contains("[x]")),
        "ADR must cite landed S3-I81 claim"
    );
    assert!(
        adr.contains("≠ S3-06 `[x]`") || adr.contains("≠ S3-06 [x]"),
        "ADR must Non-claim S3-06 [x] for residual window"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-3"),
        "ADR must Non-claim Stage-3 exit"
    );

    let census = repo_file("docs/stage3-lsp-book-census.md");
    assert!(
        census.contains("Current:** S3-I81") || census.contains("Current: S3-I81"),
        "census Current must be S3-I81"
    );
    assert!(
        census.contains("residual_scope_helper")
            && (census.contains("scoped") || census.contains("Deferred")),
        "census must publish I80 KPIs / scoped goal"
    );
    assert!(
        census.contains("S3-I81")
            && census.contains("S3-I82")
            && census.contains("S3-I83")
            && census.contains("S3-I84")
            && census.contains("S3-I85")
            && census.contains("S3-I86")
            && census.contains("S3-I87")
            && (census.contains("S3-I88") || census.contains("Stage-4")),
        "census ladder must name S3-I81…I85"
    );
    assert!(census.contains("7/7"), "census must publish Stage-3 7/7");
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );

    let prior = repo_file("docs/adr/stage3-s3-06-s045-lsp-desugar.md");
    assert!(
        prior.contains("S3-I80") || prior.contains("stage3-s3-06-residual-scope"),
        "I79 ADR must cite landed I80"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s306 = plan02
        .lines()
        .find(|l| l.contains("| S3-06 |"))
        .expect("S3-06 row");
    assert!(
        s306.contains("| [x] |") || s306.contains("| `[x]` |"),
        "S3-06 must be [x] after I81 claim; got: {s306}"
    );
    assert!(
        s306.contains("scoped")
            || s306.contains("residual-scope")
            || s306.contains("deferred")
            || s306.contains("Deferred"),
        "S3-06 row must publish scoped acceptance; got: {s306}"
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
    assert!(plan02.contains("7/7"), "plan/02 must publish Stage-3 7/7");

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I81") && plan01.contains("S3-I82") && plan01.contains("7/7"),
        "plan/01 must cite I81 / I82 / 7/7"
    );
}

#[test]
fn s3_06_residual_scope_census_script_emits_hinge() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = std::process::Command::new("bash")
        .arg("scripts/s3_06_lsp_book_census.sh")
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
        stdout.contains("current=S3-I81") && stdout.contains("next=S3-I88"),
        "script ladder must be I81→I88:\n{stdout}"
    );
    assert!(
        stdout.contains("residual_scope_helper="),
        "script must emit I80 hinge:\n{stdout}"
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
    assert_eq!(parse("s045_lsp_desugar_nail_count="), 0);
    assert_eq!(parse("book_lsp_semantic_mismatch_count="), 0);
}
