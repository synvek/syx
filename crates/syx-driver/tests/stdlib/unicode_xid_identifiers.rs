//! 04 Unicode XID identifiers (lexer UAX #31).
// was: m2_s2_04_unicode_xid_identifiers

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn product_block<'a>(caps: &'a str, id: &str) -> &'a str {
    let needle = format!("id = \"{id}\"");
    let rest = caps
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing capability {id}"));
    rest.split("[[capability]]").next().unwrap_or(rest)
}

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

fn lines(out: &str) -> Vec<&str> {
    out.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect()
}

#[test]
fn m2_s2_04_ascii_regression() {
    let out = run(
        r#"
func main() {
  let x = 1
  let _foo = 2
  print(x)
  print(_foo)
}
"#,
    );
    let l = lines(&out);
    assert_eq!(l.get(0).copied(), Some("1"), "ascii x: {out}");
    assert_eq!(l.get(1).copied(), Some("2"), "ascii _foo: {out}");
}

#[test]
fn m2_s2_04_greek_ident() {
    let out = run(
        r#"
func main() {
  let α = 1
  print(α)
}
"#,
    );
    let l = lines(&out);
    assert_eq!(l.get(0).copied(), Some("1"), "greek α: {out}");
}

#[test]
fn m2_s2_04_cjk_func() {
    let out = run(
        r#"
func 计数() {
  print("ok")
}
func main() {
  计数()
}
"#,
    );
    let l = lines(&out);
    assert_eq!(l.get(0).copied(), Some("ok"), "CJK func: {out}");
}

#[test]
fn m2_s2_04_emoji_ident_start_fails() {
    syx_host_plugin::register_echo_plugin_in_process();
    let err = compile_and_run_embedded(
        r#"
func main() {
  let 😀 = 1
  print(😀)
}
"#,
        &DriverOptions::default(),
    );
    // Wave A W93 / ADR-437 / A-03: emoji ident start fail-closed product exact Display.
    let msg = match err {
        Ok(r) => panic!("emoji ident start must fail, got stdout={:?}", r.printed),
        Err(e) => format!("{e}"),
    };
    assert_eq!(
        msg, "embed.sy:3:7: [L001] Unexpected character '😀'",
        "emoji ident start must exact: {msg}"
    );
}

#[test]
fn m2_s2_04_docs_claim_xid() {
    let lex = include_str!("../../../../docs/1_LexicalAnalysis.md");
    assert!(
        lex.contains("XID_Start") && lex.contains("ADR-229"),
        "LexicalAnalysis must document XID + ADR-229"
    );
    assert!(
        !lex.contains("首字符：ASCII 字母或 `_`"),
        "LexicalAnalysis must not claim ASCII-only idents"
    );
    let lr = include_str!("../../../../books/language-reference/01-lexical-structure.md");
    assert!(
        lr.contains("ADR-229") && lr.contains("XID_Start"),
        "language-reference must note ADR-229 XID"
    );
    let usage = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        usage.contains("ADR-229")
            && (usage.contains("XID") || usage.contains("unicode_identifiers")),
        "UsageGuide must document XID identifiers"
    );
}

#[test]
fn m2_s2_04_lexer_static_xid() {
    let src = include_str!("../../../syx-lexer/src/lib.rs");
    assert!(
        src.contains("unicode_xid") && src.contains("is_xid_start") && src.contains("ADR-229"),
        "lexer must use unicode-xid for ident start/continue"
    );
}

#[test]
fn m2_s2_04_product_unicode_identifiers() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "language.unicode_identifiers");
    assert!(
        block.contains("status = \"implemented\""),
        "language.unicode_identifiers must be implemented: {block}"
    );
    assert!(
        block.contains("m2_s2_04_unicode_xid_identifiers"),
        "tests must include nail: {block}"
    );
    assert!(
        block.contains("ADR-229") || block.contains("M2-S2-04"),
        "notes must cite ADR-229: {block}"
    );
    assert!(
        block.contains("IANA")
            || block.contains("confusable")
            || block.contains("NFC")
            || block.contains("EC-M2-4-Z"),
        "notes must keep honesty fence: {block}"
    );
}

#[test]
fn m2_s2_04_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m2_s2_04_unicode_xid_identifiers_tier"),
        "inventory must contain m2_s2_04_unicode_xid_identifiers_tier"
    );
    assert!(
        inv.contains("ADR-229") || inv.contains("signed-ADR-229"),
        "inventory must note ADR-229"
    );
}

#[test]
fn m2_s2_04_ci_wires() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test stdlib"),
        "ci.yml must wire m2_s2_04_unicode_xid_identifiers"
    );
}
