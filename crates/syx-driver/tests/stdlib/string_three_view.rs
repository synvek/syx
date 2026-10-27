//! 01 String Character(grapheme) / scalar / UTF-8 three-view.
//! native/wasm behavioral nails for text.three_view.
//! StrUnicodeScalarCount HostFn llvm=real (native AOT nail).
//! StrUtf8Count HostFn llvm=real (native AOT nail).
//! StrUnicodeScalars HostFn llvm=real (native AOT nail).
//! StrUtf8Bytes HostFn llvm=real (native AOT nail).
//! StrUnicodeScalarCount + StrUtf8Count HostFn wasm=real (text HostFn wasm8).
//! StrUnicodeScalars + StrUtf8Bytes + CharacterIsSingleScalar HostFn wasm=real (text 收官).
// was: m2_s2_01_string_three_view

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
        "syx_w207_{}_{}",
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
        "syx_w234_scalar_count_llvm_{}",
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

const GRAPHEME_LENGTH_SUBSCRIPT: &str = r#"
func main() {
  let s = "👨‍👩‍👧"
  print(String.count(s))
  print(__syx_value_description(s[0]))
}
"#;

const SCALAR_VIEW_DIVERGES: &str = r#"
func main() {
  let s = "👨‍👩‍👧"
  print(String.count(s))
  print(__syx_str_unicode_scalar_count(s))
  print(__syx_str_unicode_scalar_count("ab"))
  print(__syx_str_unicode_scalars("ab").count)
}
"#;

const SCALAR_COUNT_AB: &str = r#"
func main() {
  print(__syx_str_unicode_scalar_count("ab"))
}
"#;

const UTF8_COUNT_HI: &str = r#"
func main() {
  print(__syx_str_utf8_count("hi"))
}
"#;

const SCALARS_LEN_AB: &str = r#"
func main() {
  print(__syx_str_unicode_scalars("ab").count)
}
"#;

const UTF8_BYTES_LEN_HI: &str = r#"
func main() {
  print(__syx_str_utf8_bytes("hi").count)
}
"#;

const IS_SINGLE_SCALAR_A: &str = r#"
func main() {
  print(__syx_character_is_single_scalar("a"))
}
"#;

const FOR_IN_IS_GRAPHEME: &str = r#"
func main() {
  let s = "👨‍👩‍👧"
  var n = 0
  for g in s { n = n + 1 }
  print(n)
  var m = 0
  for ch in "ab" { m = m + 1 }
  print(m)
}
"#;

#[test]
fn m2_s2_01_grapheme_length_and_subscript_native() {
    let out = lines_joined(&run_native_src(
        "grapheme_native",
        GRAPHEME_LENGTH_SUBSCRIPT,
    ));
    assert_eq!(out, "1\n👨‍👩‍👧", "native grapheme must exact: {out}");
}

#[test]
fn m2_s2_01_grapheme_length_and_subscript_wasm() {
    let out = lines_joined(&run_wasm_src("grapheme_wasm", GRAPHEME_LENGTH_SUBSCRIPT));
    assert_eq!(out, "1\n👨‍👩‍👧", "wasm grapheme must exact: {out}");
}

#[test]
fn m2_s2_01_scalar_view_diverges_native() {
    let out = lines_joined(&run_native_src("scalar_native", SCALAR_VIEW_DIVERGES));
    assert_eq!(out, "1\n5\n2\n2", "native scalar view must exact: {out}");
}

#[test]
fn m2_s2_01_scalar_view_diverges_wasm() {
    let out = lines_joined(&run_wasm_src("scalar_wasm", SCALAR_VIEW_DIVERGES));
    assert_eq!(out, "1\n5\n2\n2", "wasm scalar view must exact: {out}");
}

#[test]
fn m2_s2_01_scalar_count_native_llvm() {
    let (code, stdout) = run_native_strict(SCALAR_COUNT_AB);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(out, "2", "native llvm scalar_count ab must exact: {out}");
}

#[test]
fn m2_s2_01_scalar_count_wasm() {
    let out = lines_joined(&run_wasm_src("scalar_count_wasm", SCALAR_COUNT_AB));
    assert_eq!(out, "2", "wasm scalar_count ab must exact: {out}");
}

#[test]
fn m2_s2_01_str_unicode_scalar_count_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "StrUnicodeScalarCount");
    assert!(
        block.contains("llvm = \"real\""),
        "StrUnicodeScalarCount must declare llvm=real (ADR-578): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "StrUnicodeScalarCount must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "StrUnicodeScalarCount must declare wasm=real (ADR-610): {block}"
    );
    assert!(
        block.contains("ADR-610") && block.contains("ADR-578"),
        "notes must cite ADR-610 + ADR-578: {block}"
    );
}

#[test]
fn m2_s2_01_utf8_count_wasm() {
    let out = lines_joined(&run_wasm_src("utf8_count_wasm", UTF8_COUNT_HI));
    assert_eq!(out, "2", "wasm utf8_count hi must exact: {out}");
}

#[test]
fn m2_s2_01_utf8_count_native_llvm() {
    let (code, stdout) = run_native_strict(UTF8_COUNT_HI);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(out, "2", "native llvm utf8_count hi must exact: {out}");
}

#[test]
fn m2_s2_01_str_utf8_count_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "StrUtf8Count");
    assert!(
        block.contains("llvm = \"real\""),
        "StrUtf8Count must declare llvm=real (ADR-579): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "StrUtf8Count must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "StrUtf8Count must declare wasm=real (ADR-610): {block}"
    );
    assert!(
        block.contains("ADR-610") && block.contains("ADR-579"),
        "notes must cite ADR-610 + ADR-579: {block}"
    );
}

#[test]
fn m2_s2_01_unicode_scalars_length_native_llvm() {
    let (code, stdout) = run_native_strict(SCALARS_LEN_AB);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(
        out, "2",
        "native llvm scalars(ab).count must exact: {out}"
    );
}

#[test]
fn m2_s2_01_scalars_length_wasm() {
    let out = lines_joined(&run_wasm_src("scalars_len_wasm", SCALARS_LEN_AB));
    assert_eq!(out, "2", "wasm scalars(ab).count must exact: {out}");
}

#[test]
fn m2_s2_01_str_unicode_scalars_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "StrUnicodeScalars");
    assert!(
        block.contains("llvm = \"real\""),
        "StrUnicodeScalars must declare llvm=real (ADR-580): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "StrUnicodeScalars must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "StrUnicodeScalars must declare wasm=real (ADR-611): {block}"
    );
    assert!(
        block.contains("ADR-611") && block.contains("ADR-580"),
        "notes must cite ADR-611 + ADR-580: {block}"
    );
}

#[test]
fn m2_s2_01_utf8_bytes_length_wasm() {
    let out = lines_joined(&run_wasm_src("utf8_bytes_len_wasm", UTF8_BYTES_LEN_HI));
    assert_eq!(out, "2", "wasm utf8_bytes(hi).count must exact: {out}");
}

#[test]
fn m2_s2_01_utf8_bytes_length_native_llvm() {
    let (code, stdout) = run_native_strict(UTF8_BYTES_LEN_HI);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(
        out, "2",
        "native llvm utf8_bytes(hi).count must exact: {out}"
    );
}

#[test]
fn m2_s2_01_str_utf8_bytes_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "StrUtf8Bytes");
    assert!(
        block.contains("llvm = \"real\""),
        "StrUtf8Bytes must declare llvm=real (ADR-581): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "StrUtf8Bytes must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "StrUtf8Bytes must declare wasm=real (ADR-611): {block}"
    );
    assert!(
        block.contains("ADR-611") && block.contains("ADR-581"),
        "notes must cite ADR-611 + ADR-581: {block}"
    );
}

#[test]
fn m2_s2_01_is_single_scalar_wasm() {
    let out = lines_joined(&run_wasm_src("is_single_scalar_wasm", IS_SINGLE_SCALAR_A));
    assert_eq!(out, "true", "wasm is_single_scalar(a) must exact: {out}");
}

#[test]
fn m2_s2_01_is_single_scalar_native_llvm() {
    let (code, stdout) = run_native_strict(IS_SINGLE_SCALAR_A);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(
        out, "true",
        "native llvm is_single_scalar(a) must exact: {out}"
    );
}

#[test]
fn m2_s2_01_character_is_single_scalar_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "CharacterIsSingleScalar");
    assert!(
        block.contains("llvm = \"real\""),
        "CharacterIsSingleScalar must declare llvm=real (ADR-582): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "CharacterIsSingleScalar must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "CharacterIsSingleScalar must declare wasm=real (ADR-611): {block}"
    );
    assert!(
        block.contains("ADR-611") && block.contains("ADR-582"),
        "notes must cite ADR-611 + ADR-582: {block}"
    );
}

#[test]
fn m2_s2_01_for_in_is_grapheme_native() {
    let out = lines_joined(&run_native_src("for_in_native", FOR_IN_IS_GRAPHEME));
    assert_eq!(out, "1\n2", "native for-in grapheme must exact: {out}");
}

#[test]
fn m2_s2_01_for_in_is_grapheme_wasm() {
    let out = lines_joined(&run_wasm_src("for_in_wasm", FOR_IN_IS_GRAPHEME));
    assert_eq!(out, "1\n2", "wasm for-in grapheme must exact: {out}");
}

#[test]
fn m2_s2_01_grapheme_length_and_subscript() {
    let out = run(GRAPHEME_LENGTH_SUBSCRIPT);
    // Wave A W86 / ADR-430 / A-03: grapheme length + subscript product exact.
    assert_eq!(out, "1\n👨‍👩‍👧", "grapheme length/subscript must exact: {out}");
}

#[test]
fn m2_s2_01_scalar_view_diverges_from_grapheme() {
    let out = run(SCALAR_VIEW_DIVERGES);
    // Wave A W85 / ADR-429 / A-03: scalar view vs grapheme product exact.
    assert_eq!(out, "1\n5\n2\n2", "scalar view must exact: {out}");
}

#[test]
fn m2_s2_01_utf8_view() {
    let out = run(
        r#"
func main() {
  print(__syx_str_utf8_count("hi"))
  print(__syx_str_utf8_bytes("hi").count)
  print(__syx_str_utf8_bytes("hi")[0])
  let s = "👨‍👩‍👧"
  print(String.count(s))
  print(__syx_str_utf8_count(s))
}
"#,
    );
    // Wave A W85 / ADR-429 / A-03: utf8 view product exact.
    assert_eq!(out, "2\n2\n104\n1\n18", "utf8 view must exact: {out}");
}

#[test]
fn m2_s2_01_for_in_is_grapheme() {
    let out = run(FOR_IN_IS_GRAPHEME);
    // Wave A W84 / ADR-428 / A-03: for-in is grapheme product exact.
    assert_eq!(out, "1\n2", "for-in grapheme must exact: {out}");
}

#[test]
fn m2_s2_01_is_single_scalar_honest() {
    let out = run(
        r#"
func main() {
  print(__syx_character_is_single_scalar("a"))
  print(__syx_character_is_single_scalar("👨‍👩‍👧"))
  print(__syx_character_is_single_scalar("ab"))
}
"#,
    );
    // Wave A W84 / ADR-428 / A-03: isSingleScalar honesty product exact.
    assert_eq!(out, "true\nfalse\nfalse", "isSingleScalar must exact: {out}");
}

#[test]
fn m2_s2_01_hosts_no_empty_feature_off_static() {
    let hosts = include_str!("../../../syx-vm/src/hosts.rs");
    assert!(
        hosts.contains("host_str_unicode_scalar_count")
            && hosts.contains("host_str_utf8_bytes")
            && hosts.contains("host_character_is_single_scalar"),
        "hosts must define three-view hosts"
    );
    let lib = include_str!("../../../syx-vm/src/lib.rs");
    assert!(
        lib.contains("graphemes_vec") && lib.contains("ADR-226"),
        "MakeIterator String for-in must use graphemes: check lib.rs"
    );
}

#[test]
fn m2_s2_01_product_three_view() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "text.three_view");
    assert!(
        block.contains("status = \"implemented\""),
        "text.three_view must be implemented: {block}"
    );
    assert!(
        block.contains("m2_s2_01_string_three_view"),
        "tests must include nail: {block}"
    );
    assert!(
        block.contains("ADR-226") || block.contains("M2-S2-01"),
        "notes must cite ADR-226: {block}"
    );
    assert!(
        block.contains("M2-S2-02")
            || block.contains("Index")
            || block.contains("normalization")
            || block.contains("IANA"),
        "notes must keep honesty fence: {block}"
    );
}

#[test]
fn m2_s2_01_product_three_view_backends_real() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "text.three_view");
    assert!(
        block.contains("native = \"real\"") && block.contains("wasm = \"real\""),
        "text.three_view native/wasm must be real: {block}"
    );
    assert!(
        block.contains("mir = \"real\""),
        "text.three_view mir must stay real: {block}"
    );
    assert!(
        block.contains("status = \"implemented\""),
        "text.three_view status must stay implemented: {block}"
    );
    assert!(
        block.contains("ADR-551"),
        "notes must cite ADR-551: {block}"
    );
    assert!(
        block.contains("m2_s2_01_grapheme_length_and_subscript_native"),
        "tests must list native nail: {block}"
    );
}

#[test]
fn m2_s2_01_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m2_s2_01_string_three_view_tier"),
        "inventory must contain m2_s2_01_string_three_view_tier"
    );
    assert!(
        inv.contains("ADR-226") || inv.contains("signed-ADR-226"),
        "inventory must note ADR-226"
    );
}

#[test]
fn m2_s2_01_ci_wires() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test stdlib"),
        "ci.yml must wire m2_s2_01_string_three_view"
    );
}
