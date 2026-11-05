//! 04 Unicode XID identifiers (lexer UAX #31).
// was: m2_s2_04_unicode_xid_identifiers

use syx_driver::{compile_and_run_embedded, DriverOptions};

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

