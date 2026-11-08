//! S3-I79 S3-06 S045 LSP desugar parity knife contracts.
//! Hinge landed · post-I81: S3-06 [x] · Stage-3 still [~] 7/7.
//! See docs/adr/stage3-s3-06-s045-lsp-desugar.md · docs/stage3-lsp-book-census.md.

use std::path::Path;

use syx_driver::{analyze_document_for_lsp, diagnostic_codes, DriverOptions};

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn lsp_codes(path: Option<&Path>, source: &str) -> Vec<String> {
    let doc = analyze_document_for_lsp(path, source, &DriverOptions::default());
    diagnostic_codes(&doc.semantic.errors)
}

#[test]
fn s3_06_s045_lsp_desugar_adr_and_census() {
    let adr = repo_file("docs/adr/stage3-s3-06-s045-lsp-desugar.md");
    assert!(adr.contains("S3-I79"), "construct ADR must name S3-I79");
    assert!(
        adr.contains("should_desugar_top_level_stmts") || adr.contains("desugar"),
        "ADR must name desugar hinge"
    );
    assert!(
        adr.contains("S045"),
        "ADR must name S045 false-positive axis"
    );
    assert!(
        adr.contains("S3-I80")
            || adr.contains("stage3-s3-06-residual-scope"),
        "ADR must cite landed S3-I80 residual"
    );
    assert!(
        adr.contains("S3-I81") || adr.contains("stage3-s3-06-claim"),
        "ADR must cite landed S3-I81 claim"
    );
    assert!(
        adr.contains("≠ S3-06 `[x]`") || adr.contains("≠ S3-06 [x]"),
        "ADR must Non-claim S3-06 [x] for construct window"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-3"),
        "ADR must Non-claim Stage-3 exit"
    );

    let census = repo_file("docs/stage3-lsp-book-census.md");
    assert!(
        census.contains("Current:** S3-I81") || census.contains("Current: S3-I81"),
        "census Current must advance to S3-I81"
    );
    assert!(
        census.contains("Claimed") && (census.contains("`[x]`") || census.contains("[x]")),
        "census must mark Claimed scoped [x]"
    );
    assert!(
        census.contains("s045_lsp_desugar_nail_count")
            && census.contains("S3-I80")
            && census.contains("S3-I81")
            && census.contains("S3-I82")
            && census.contains("S3-I83")
            && census.contains("S3-I84")
            && census.contains("S3-I85")
            && census.contains("S3-I86")
            && census.contains("S3-I87")
            && (census.contains("S3-I88") || census.contains("Stage-4")),
        "census must publish I79 KPI and I80…I85 ladder"
    );
    assert!(census.contains("7/7"), "census must publish Stage-3 7/7");

    let lint = repo_file("crates/syx-driver/src/lsp_ide.rs");
    assert!(
        lint.contains("should_desugar_top_level_stmts") && lint.contains("S3-I79"),
        "lsp_ide must mount should_desugar hinge with S3-I79"
    );
    assert!(
        !lint.contains("desugar_top_level: false,\n            source: Some(source.clone())"),
        "analyze_document_for_lsp must not hard-code desugar_top_level: false"
    );

    let prior = repo_file("docs/adr/stage3-s3-06-open.md");
    assert!(
        prior.contains("S3-I79") || prior.contains("stage3-s3-06-s045-lsp-desugar"),
        "open ADR must cite landed I79"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s306 = plan02
        .lines()
        .find(|l| l.contains("| S3-06 |"))
        .expect("S3-06 row");
    assert!(
        s306.contains("| [x] |") || s306.contains("| `[x]` |"),
        "S3-06 must be [x] after I81; got: {s306}"
    );
    assert!(
        plan02.contains("S3-I79") && plan02.contains("S3-I80") && plan02.contains("S3-I81"),
        "plan/02 must cite I79…I81"
    );
    assert!(plan02.contains("7/7"), "plan/02 must publish Stage-3 7/7");

    let plan01 = repo_file("plan/01-project-status.md");
    assert!(
        plan01.contains("S3-I79") && plan01.contains("S3-I80") && plan01.contains("7/7"),
        "plan/01 must cite I79 / I80 / 7/7"
    );
}

#[test]
fn s3_06_s045_lsp_desugar_nail_cleared() {
    // Freestanding untitled buffer: check-green top-level print must not S045 under LSP.
    let untitled = lsp_codes(None, "print(1)\n");
    assert!(
        !untitled.iter().any(|c| c == "S045"),
        "untitled top-level print must not emit S045; got {untitled:?}"
    );

    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("orphan_print.sy");
    std::fs::write(&path, "print(1)\n").expect("write nail");
    let orphan = lsp_codes(Some(&path), "print(1)\n");
    assert!(
        !orphan.iter().any(|c| c == "S045"),
        "orphan .sy top-level print must not emit S045; got {orphan:?}"
    );

    // Book fixtures remain clean on LSP semantic path.
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    for rel in [
        "crates/syx-driver/tests/fixtures/lang/struct_book_point.sy",
        "crates/syx-driver/tests/fixtures/lang/struct_book_counter.sy",
        "crates/syx-driver/tests/fixtures/lang/enum_book_status.sy",
    ] {
        let p = root.join(rel);
        let src = std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("read {rel}: {e}"));
        let codes = lsp_codes(Some(&p), &src);
        assert!(
            codes.is_empty(),
            "book fixture {rel} must have empty LSP semantic errors; got {codes:?}"
        );
    }
}

#[test]
fn s3_06_s045_lsp_desugar_census_script_cleared() {
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
        stdout.contains("s045_lsp_desugar_helper=")
            && stdout.contains("s045_lsp_desugar_nail_count="),
        "script must emit I79 hinges:\n{stdout}"
    );
    let parse = |key: &str| {
        stdout
            .lines()
            .find(|l| l.starts_with(key))
            .and_then(|l| l.split('=').nth(1))
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(999)
    };
    assert!(
        parse("s045_lsp_desugar_helper=") >= 1,
        "s045_lsp_desugar_helper must be >=1\n{stdout}"
    );
    assert_eq!(parse("s045_lsp_desugar_nail_count="), 0);
    assert_eq!(parse("book_lsp_semantic_mismatch_count="), 0);
}
