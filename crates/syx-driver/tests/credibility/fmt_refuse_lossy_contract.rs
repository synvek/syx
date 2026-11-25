//! C-8b: format refuse-lossy contracts.
//! Default CST001 → F003 (no silent AST write); --allow-lossy restores auto;
//! LSP hard-refuses lossy. ≠ C-8 [x] / ≠ lossless GA.

use syx_diagnostics::SourceFile;
use syx_fmt::{format_dry_run, format_dry_run_with, FormatPolicy};

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn c8b_fmt_refuse_lossy_policy_and_cli() {
    let fmt = repo_file("crates/syx-fmt/src/lib.rs");
    assert!(
        fmt.contains("struct FormatPolicy") && fmt.contains("allow_lossy"),
        "syx-fmt must define FormatPolicy.allow_lossy"
    );
    assert!(
        fmt.contains("\"F003\""),
        "syx-fmt must emit F003 on refuse-lossy CST001"
    );

    let cli = repo_file("crates/syx-cli/src/main.rs");
    assert!(
        cli.contains("allow_lossy: bool") && cli.contains("#[arg(long)]"),
        "CLI Format must expose --allow-lossy (clap long on allow_lossy)"
    );
    assert!(
        cli.contains("format_project_with") && cli.contains("format_check_path_with_policy"),
        "CLI must wire allow_lossy into project/check"
    );

    let driver = repo_file("crates/syx-driver/src/format_lint.rs");
    let lsp_idx = driver
        .find("pub fn format_document_for_lsp(")
        .expect("format_document_for_lsp");
    let lsp_chunk = &driver[lsp_idx..lsp_idx + 800.min(driver.len() - lsp_idx)];
    assert!(
        lsp_chunk.contains("allow_lossy: false"),
        "LSP must hardcode allow_lossy=false"
    );
    assert!(
        !lsp_chunk.contains("allow_lossy: true"),
        "LSP must never allow_lossy=true"
    );

    let en = repo_file("docs/9_UsageGuide.md");
    let zh = repo_file("docs/zh/9_UsageGuide.zh-CN.md");
    assert!(
        en.contains("--allow-lossy") && en.contains("F003"),
        "en UsageGuide must document --allow-lossy / F003"
    );
    assert!(
        zh.contains("--allow-lossy") && zh.contains("F003"),
        "zh UsageGuide must document --allow-lossy / F003"
    );

    // C-8f: refuse-lossy ≠ format lossless GA (UsageGuide Honesty).
    assert!(
        en.contains("Honesty (C-8f)")
            && en.contains("lossless GA")
            && en.contains("≠"),
        "en UsageGuide must Honesty (C-8f) ≠ lossless GA"
    );
    assert!(
        zh.contains("诚实（C-8f）") && zh.contains("lossless GA") && zh.contains("≠"),
        "zh UsageGuide must 诚实（C-8f）≠ lossless GA"
    );
    assert!(
        fmt.contains("≠ format lossless GA (C-8f)") || fmt.contains("lossless GA (C-8f)"),
        "FormatPolicy docs must note ≠ format lossless GA (C-8f)"
    );
}

#[test]
fn c8b_fmt_refuse_lossy_behavior() {
    // Out-of-subset paren nest + leading comment: AST would drop comment.
    let src = "// keep?\nfunc main() { while ((((a)))) { print(1) } }\n";
    let file = SourceFile::new("lossy.sy", src);

    let refuse = format_dry_run(&file).expect_err("default must F003");
    assert_eq!(refuse.code.as_deref(), Some("F003"));
    assert!(
        refuse.message.contains("lossy") && refuse.message.contains("--allow-lossy"),
        "F003 message must guide --allow-lossy; got {refuse:?}"
    );

    let lossy = format_dry_run_with(&file, FormatPolicy { allow_lossy: true })
        .expect("allow_lossy must Ok via AST");
    assert!(
        lossy.formatted.contains("while"),
        "allow_lossy AST path must format"
    );
    assert!(
        !lossy.formatted.contains("// keep?"),
        "lossy AST may drop comments (honesty: lossy)"
    );

    // LSP surface: hard refuse.
    let lsp_err = syx_driver::format_document_for_lsp("lossy.sy", src)
        .expect_err("LSP must refuse lossy");
    assert_eq!(lsp_err.code.as_deref(), Some("F003"));
}
