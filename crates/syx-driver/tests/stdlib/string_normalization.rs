//! 03 String NFC/NFD + caseFolded MVP.
//! native/wasm behavioral nails for text.normalization.
//! StrCaseFolded HostFn llvm=real (native AOT nail).
//! StrNormalizedNFC HostFn llvm=real (native AOT nail).
//! StrNormalizedNFD HostFn llvm=real (native AOT nail).
//! StrInstLowercased HostFn llvm=real (native AOT nail).
//! StrInstUppercased HostFn llvm=real (native AOT nail).
//! Str* norm/case HostFn wasm=real (HostFn wasm8 batch).
// was: m2_s2_03_string_normalization

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use syx_driver::{
    build_native_executable, compile_and_run_embedded, run_native_mir_file_printed,
    run_wasm_source_printed, DriverOptions,
};

fn product_block<'a>(caps: &'a str, id: &str) -> &'a str {
    let needle = format!("id = \"{id}\"");
    let rest = caps
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing capability {id}"));
    rest.split("[[capability]]").next().unwrap_or(rest)
}

fn host_block<'a>(toml: &'a str, name: &str) -> &'a str {
    let needle = format!("name = \"{name}\"");
    let rest = toml
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing host override {name}"));
    rest.split("[[host]]").next().unwrap_or(rest)
}

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &opts())
        .expect("run")
        .printed
        .join("\n")
}

fn write_src_tmp(label: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "syx_w212_{}_{}",
        label,
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("main.sy");
    fs::write(&path, src).unwrap();
    path
}

fn run_native_src(label: &str, src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = write_src_tmp(label, src);
    let (code, lines) =
        run_native_mir_file_printed(&path, &opts()).expect("native mir");
    assert_eq!(code, 0, "native exit {code}: {lines:?}");
    lines
}

fn run_wasm_src(label: &str, src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = write_src_tmp(label, src);
    let (code, lines) =
        run_wasm_source_printed(&path, "main", &opts()).expect("wasm");
    assert_eq!(code, 0, "wasm exit {code}: {lines:?}");
    lines
}

fn lines_joined(lines: &[String]) -> String {
    lines
        .iter()
        .map(|l| l.trim_end_matches('\r').trim_end_matches('\n'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn run_native_strict(src: &str) -> (i32, String) {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_w229_case_folded_llvm_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), src).unwrap();
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .unwrap();
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native exe");
    (
        output.status.code().unwrap_or(1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
    )
}

fn nfc_compose_src() -> String {
    let nfd = format!("e{}", '\u{0301}');
    format!(
        r#"
func main() {{
  print(__syx_str_normalized_nfc("{nfd}"))
  print(__syx_str_normalized_nfc("é"))
}}
"#
    )
}

const CASE_FOLDED: &str = r#"
func main() {
  print(__syx_str_case_folded("ß"))
  print(__syx_str_case_folded("AbC"))
}
"#;

const NFD_ROUNDTRIP: &str = r#"
func main() {
  let d = __syx_str_normalized_nfd("é")
  print(__syx_str_unicode_scalar_count(d))
  print(__syx_str_normalized_nfc(d))
}
"#;

const LOWERCASED_VS_FOLD: &str = r#"
func main() {
  print(__syx_str_lowercased("ß"))
  print(__syx_str_case_folded("ß"))
}
"#;

const UPPERCASED_ASCII: &str = r#"
func main() {
  print(__syx_str_uppercased("AbC"))
}
"#;

#[test]
fn m2_s2_03_nfc_composes_combining_acute() {
    let out = run(&nfc_compose_src());
    // Wave A W82 / ADR-426 / A-03: NFC compose combining acute product exact.
    assert_eq!(out, "é\né", "NFC compose must exact: {out}");
}

#[test]
fn m2_s2_03_nfc_compose_native() {
    let out = lines_joined(&run_native_src("nfc_native", &nfc_compose_src()));
    assert_eq!(out, "é\né", "native NFC compose must exact: {out}");
}

#[test]
fn m2_s2_03_nfc_compose_wasm() {
    let out = lines_joined(&run_wasm_src("nfc_wasm", &nfc_compose_src()));
    assert_eq!(out, "é\né", "wasm NFC compose must exact: {out}");
}

#[test]
fn m2_s2_03_nfc_compose_native_llvm() {
    let (code, stdout) = run_native_strict(&nfc_compose_src());
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(
        out, "é\né",
        "native llvm NFC compose must exact: {out}"
    );
}

#[test]
fn m2_s2_03_str_normalized_nfc_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "StrNormalizedNFC");
    assert!(
        block.contains("llvm = \"real\""),
        "StrNormalizedNFC must declare llvm=real (ADR-574): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "StrNormalizedNFC must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "StrNormalizedNFC must declare wasm=real (ADR-610): {block}"
    );
    assert!(
        block.contains("ADR-610") && block.contains("ADR-574"),
        "notes must cite ADR-610 + ADR-574: {block}"
    );
}

#[test]
fn m2_s2_03_nfd_roundtrip_via_nfc() {
    let out = run(NFD_ROUNDTRIP);
    // Wave A W83 / ADR-427 / A-03: NFD roundtrip via NFC product exact.
    assert_eq!(out, "2\né", "NFD roundtrip must exact: {out}");
}

#[test]
fn m2_s2_03_nfd_roundtrip_native_llvm() {
    let (code, stdout) = run_native_strict(NFD_ROUNDTRIP);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(
        out, "2\né",
        "native llvm NFD roundtrip must exact: {out}"
    );
}

#[test]
fn m2_s2_03_nfd_roundtrip_wasm() {
    let out = lines_joined(&run_wasm_src("nfd_wasm", NFD_ROUNDTRIP));
    assert_eq!(out, "2\né", "wasm NFD roundtrip must exact: {out}");
}

#[test]
fn m2_s2_03_str_normalized_nfd_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "StrNormalizedNFD");
    assert!(
        block.contains("llvm = \"real\""),
        "StrNormalizedNFD must declare llvm=real (ADR-575): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "StrNormalizedNFD must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "StrNormalizedNFD must declare wasm=real (ADR-610): {block}"
    );
    assert!(
        block.contains("ADR-610") && block.contains("ADR-575"),
        "notes must cite ADR-610 + ADR-575: {block}"
    );
}

#[test]
fn m2_s2_03_case_folded_full_and_ascii() {
    let out = run(CASE_FOLDED);
    // Wave A W82 / ADR-426 / A-03: caseFolded full + ASCII product exact.
    assert_eq!(out, "ss\nabc", "caseFolded must exact: {out}");
}

#[test]
fn m2_s2_03_case_folded_native() {
    let out = lines_joined(&run_native_src("case_native", CASE_FOLDED));
    assert_eq!(out, "ss\nabc", "native caseFolded must exact: {out}");
}

#[test]
fn m2_s2_03_case_folded_wasm() {
    let out = lines_joined(&run_wasm_src("case_wasm", CASE_FOLDED));
    assert_eq!(out, "ss\nabc", "wasm caseFolded must exact: {out}");
}

#[test]
fn m2_s2_03_case_folded_native_llvm() {
    let (code, stdout) = run_native_strict(CASE_FOLDED);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(
        out, "ss\nabc",
        "native llvm caseFolded must exact: {out}"
    );
}

#[test]
fn m2_s2_03_str_case_folded_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "StrCaseFolded");
    assert!(
        block.contains("llvm = \"real\""),
        "StrCaseFolded must declare llvm=real (ADR-573): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "StrCaseFolded must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "StrCaseFolded must declare wasm=real (ADR-610): {block}"
    );
    assert!(
        block.contains("ADR-610") && block.contains("ADR-573"),
        "notes must cite ADR-610 + ADR-573: {block}"
    );
}

#[test]
fn m2_s2_03_lowercased_not_case_folded() {
    let out = run(LOWERCASED_VS_FOLD);
    // Wave A W83 / ADR-427 / A-03: lowercased ≠ caseFolded for ß product exact.
    assert_eq!(out, "ß\nss", "lowercased vs caseFolded must exact: {out}");
}

#[test]
fn m2_s2_03_lowercased_native_llvm() {
    let (code, stdout) = run_native_strict(LOWERCASED_VS_FOLD);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(
        out, "ß\nss",
        "native llvm lowercased vs caseFolded must exact: {out}"
    );
}

#[test]
fn m2_s2_03_lowercased_wasm() {
    let out = lines_joined(&run_wasm_src("lower_wasm", LOWERCASED_VS_FOLD));
    assert_eq!(
        out, "ß\nss",
        "wasm lowercased vs caseFolded must exact: {out}"
    );
}

#[test]
fn m2_s2_03_str_inst_lowercased_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "StrInstLowercased");
    assert!(
        block.contains("llvm = \"real\""),
        "StrInstLowercased must declare llvm=real (ADR-576): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "StrInstLowercased must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "StrInstLowercased must declare wasm=real (ADR-610): {block}"
    );
    assert!(
        block.contains("ADR-610") && block.contains("ADR-576"),
        "notes must cite ADR-610 + ADR-576: {block}"
    );
}

#[test]
fn m2_s2_03_uppercased_ascii() {
    let out = run(UPPERCASED_ASCII);
    assert_eq!(out, "ABC", "uppercased ASCII must exact: {out}");
}

#[test]
fn m2_s2_03_uppercased_native_llvm() {
    let (code, stdout) = run_native_strict(UPPERCASED_ASCII);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(
        out, "ABC",
        "native llvm uppercased ASCII must exact: {out}"
    );
}

#[test]
fn m2_s2_03_uppercased_wasm() {
    let out = lines_joined(&run_wasm_src("upper_wasm", UPPERCASED_ASCII));
    assert_eq!(out, "ABC", "wasm uppercased ASCII must exact: {out}");
}

#[test]
fn m2_s2_03_str_inst_uppercased_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "StrInstUppercased");
    assert!(
        block.contains("llvm = \"real\""),
        "StrInstUppercased must declare llvm=real (ADR-577): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "StrInstUppercased must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "StrInstUppercased must declare wasm=real (ADR-610): {block}"
    );
    assert!(
        block.contains("ADR-610") && block.contains("ADR-577"),
        "notes must cite ADR-610 + ADR-577: {block}"
    );
}

#[test]
fn m2_s2_03_docs_honesty() {
    let usage = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        usage.contains("ADR-228")
            && usage.contains("normalizedNFC")
            && usage.contains("caseFolded")
            && (usage.contains("simple case") || usage.contains("≠ `caseFolded`")),
        "UsageGuide must document NFC/NFD/caseFolded + lower honesty"
    );
    let ch04 = include_str!("../../../../books/language-guide/04-strings-and-characters.md");
    assert!(
        ch04.contains("caseFolded")
            && ch04.contains("NFC")
            && (ch04.contains("simple case") || ch04.contains("≠")),
        "language-guide 04 must document normalization MVP"
    );
    assert!(
        !ch04.contains("ASCII 小写化"),
        "language-guide must not claim ASCII-only lowercased"
    );
}

#[test]
fn m2_s2_03_host_inventory_partial() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in [
        "StrNormalizedNFC",
        "StrNormalizedNFD",
        "StrCaseFolded",
        "StrInstLowercased",
        "StrInstUppercased",
    ] {
        assert!(
            toml.contains(&format!("name = \"{name}\"")),
            "host_fn_capabilities.toml must override {name}"
        );
        let block = toml
            .split(&format!("name = \"{name}\""))
            .nth(1)
            .unwrap_or("")
            .split("[[host]]")
            .next()
            .unwrap_or("");
        let adr_ok = match name {
            "StrCaseFolded" => block.contains("ADR-228") || block.contains("ADR-573"),
            "StrNormalizedNFC" => block.contains("ADR-228") || block.contains("ADR-574"),
            "StrNormalizedNFD" => block.contains("ADR-228") || block.contains("ADR-575"),
            "StrInstLowercased" => block.contains("ADR-228") || block.contains("ADR-576"),
            "StrInstUppercased" => block.contains("ADR-228") || block.contains("ADR-577"),
            _ => block.contains("ADR-228"),
        };
        assert!(
            block.contains("status = \"partial\"") && adr_ok,
            "{name} must be partial + ADR-228/ADR-573..ADR-577: {block}"
        );
    }
}

#[test]
fn m2_s2_03_product_normalization() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "text.normalization");
    assert!(
        block.contains("status = \"implemented\""),
        "text.normalization must be implemented: {block}"
    );
    assert!(
        block.contains("m2_s2_03_string_normalization")
            || block.contains("m2_s2_03_nfc_compose_native"),
        "tests must include nail: {block}"
    );
    assert!(
        block.contains("ADR-556") || block.contains("ADR-228") || block.contains("M2-S2-03"),
        "notes must cite ADR-556 or ADR-228: {block}"
    );
    assert!(
        block.contains("M2-S2-04")
            || block.contains("XID")
            || block.contains("NFKC")
            || block.contains("IANA"),
        "notes must keep honesty fence: {block}"
    );
}

#[test]
fn m2_s2_03_product_normalization_backends_real() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "text.normalization");
    assert!(
        block.contains("native = \"real\"") && block.contains("wasm = \"real\""),
        "text.normalization native/wasm must be real: {block}"
    );
    assert!(
        block.contains("mir = \"real\""),
        "text.normalization mir must stay real: {block}"
    );
    assert!(
        block.contains("status = \"implemented\""),
        "text.normalization status must stay implemented: {block}"
    );
    assert!(
        block.contains("ADR-556"),
        "notes must cite ADR-556: {block}"
    );
    assert!(
        block.contains("m2_s2_03_nfc_compose_native"),
        "tests must list native nail: {block}"
    );
}

#[test]
fn m2_s2_03_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m2_s2_03_string_normalization_tier"),
        "inventory must contain m2_s2_03_string_normalization_tier"
    );
    assert!(
        inv.contains("ADR-228") || inv.contains("signed-ADR-228"),
        "inventory must note ADR-228"
    );
}

#[test]
fn m2_s2_03_ci_wires() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test stdlib"),
        "ci.yml must wire m2_s2_03_string_normalization"
    );
}
