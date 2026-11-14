//! S2-I152 S2-04 LSP DocumentFormatting contracts.
//! Nails: ADR + documentFormattingProvider + format_document_for_lsp → format_dry_run.
//! ≠ lint share / ≠ S2-04 [x].
//! See docs/adr/stage2-s2-04-lsp-document-formatting.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_04_lsp_document_formatting() {
    let adr = repo_file("docs/adr/stage2-s2-04-lsp-document-formatting.md");
    assert!(
        adr.contains("S2-I152"),
        "lsp-document-formatting ADR must exist"
    );
    assert!(
        adr.contains("documentFormattingProvider") || adr.contains("textDocument/formatting"),
        "ADR must lock LSP formatting surface"
    );
    assert!(
        adr.contains("format_dry_run") || adr.contains("format_document_for_lsp"),
        "ADR must lock dry-run/auto SSOT"
    );
    assert!(
        adr.contains("≠") && (adr.contains("S2-04") || adr.contains("[x]")),
        "ADR must Non-claim S2-04 [x]"
    );
    assert!(
        adr.contains("≠") && adr.contains("lint"),
        "ADR must Non-claim lint tree share"
    );

    let driver = repo_file("crates/syx-driver/src/format_lint.rs");
    let idx = driver
        .find("pub fn format_document_for_lsp(")
        .expect("format_document_for_lsp must exist");
    let chunk = &driver[idx..idx + 500.min(driver.len() - idx)];
    assert!(
        chunk.contains("format_dry_run("),
        "format_document_for_lsp must call format_dry_run"
    );

    let lsp = repo_file("crates/syx-lsp/src/lib.rs");
    assert!(
        lsp.contains("documentFormattingProvider"),
        "LSP must advertise documentFormattingProvider"
    );
    assert!(
        lsp.contains("textDocument/formatting"),
        "LSP must handle textDocument/formatting"
    );
    assert!(
        lsp.contains("formatting_for_document("),
        "LSP must route formatting through formatting_for_document"
    );
    assert!(
        lsp.contains("format_document_for_lsp"),
        "LSP formatting must call driver format_document_for_lsp"
    );

    // Behavioral nail: CST subset preserves comments via driver API.
    let src = "// ahead\nfunc main() {\n    // inside\n    print(1)\n}\n/* trail */\n";
    let out = syx_driver::format_document_for_lsp("t.sy", src).expect("format");
    assert!(
        out.contains("// ahead") && out.contains("// inside") && out.contains("/* trail */"),
        "driver LSP format must preserve comments; got {out:?}"
    );
    let again = syx_driver::format_document_for_lsp("t.sy", &out).expect("idempotent");
    assert_eq!(again, out);
}
