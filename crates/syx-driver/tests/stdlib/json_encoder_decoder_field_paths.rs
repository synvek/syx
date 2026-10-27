//! 02 JSONEncoder/Decoder + field path diagnostics.
//! data.coding.json_encoder native/wasm product nails (Box roundtrip).
//! JsonEncoderEncode HostFn llvm=real (native AOT nail).
//! JsonDecoderDecode HostFn llvm=real (native AOT nail).
//! JsonEncode/JsonDecode HostFn llvm=real (native AOT nail).
// was: m2_s7_02_json_encoder_decoder_field_paths

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

fn write_src_tmp(label: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "syx_w218_{}_{}",
        label,
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("main.sy");
    fs::write(&path, src).unwrap();
    path
}

fn run_native_mir_src(label: &str, src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = write_src_tmp(label, src);
    let (code, lines) =
        run_native_mir_file_printed(&path, &opts()).expect("native mir");
    assert_eq!(code, 0, "native mir exit {code}: {lines:?}");
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

fn run_native_strict(src: &str) -> (i32, String) {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_w224_json_encode_llvm_{}",
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

const BOX_ROUNDTRIP_BODY: &str = r#"
data class Box(n: Int)

func main() {
  let b = Box(n: 42)
  let s = __syx_json_encoder_encode(b, "Box")
  let back = __syx_json_decoder_decode(s, "Box")
  print(__syx_value_description(back))
}
"#;

const BOX_ENCODE_ONLY_BODY: &str = r#"
data class Box(n: Int)

func main() {
  let b = Box(n: 42)
  print(__syx_json_encoder_encode(b, "Box"))
}
"#;

const BOX_DECODE_ONLY_BODY: &str = r#"
data class Box(n: Int)

func main() {
  let back = __syx_json_decoder_decode("{\"n\":42}", "Box")
  print(__syx_value_description(back))
}
"#;

const DYNAMIC_JSON_ROUNDTRIP: &str = r#"
func main() {
  let s = __syx_json_encode(42)
  print(s)
  let v = __syx_json_decode(s)
  print(__syx_value_description(v))
}
"#;

fn assert_box_roundtrip_line(out: &str) {
    assert_eq!(out, "{n: 42}", "Box roundtrip must print {{n: 42}}: {out:?}");
}

#[test]
fn m2_s7_02_nested_encode_decode_roundtrip() {
    let out = run(
        r#"
data class Address(city: String, zip: String)
data class User(name: String, address: Address)

func main() {
  let a = Address(city: "SF", zip: "94102")
  let u = User(name: "ada", address: a)
  let s = __syx_json_encoder_encode(u, "User")
  print(s)
  let back = __syx_json_decoder_decode(s, "User")
  print(__syx_value_description(back))
}
"#,
    );
    // Wave A W75 / ADR-419 / A-03: nested JSON encode/decode roundtrip product exact.
    assert_eq!(
        out,
        "{\"address\":{\"city\":\"SF\",\"zip\":\"94102\"},\"name\":\"ada\"}\n{address: {city: SF, zip: 94102}, name: ada}",
        "nested encode/decode roundtrip must exact: {out}"
    );
}

#[test]
fn m2_s7_02_missing_key_coding_path() {
    let out = run(
        r#"
data class Address(city: String, zip: String)
data class User(name: String, address: Address)

func main() {
  try {
    let s = "{\"name\":\"ada\"}"
    let back = __syx_json_decoder_decode(s, "User")
    print("UNEXPECTED_OK")
    print(__syx_value_description(back))
  } catch (err) {
    print("caught")
    print(err.message)
  }
}
"#,
    );
    // Wave A W73 / ADR-417 / A-03: JSONDecoder missing-key codingPath product exact.
    assert_eq!(
        out, "caught\nJSONDecoder missing required key 'address' codingPath=address",
        "JSONDecoder missing-key codingPath must exact: {out}"
    );
}

#[test]
fn m2_s7_02_optional_field_omit_and_present() {
    let out = run(
        r#"
data class Card(title: String, note: String?)

func main() {
  let omit = __syx_json_decoder_decode("{\"title\":\"t\"}", "Card")
  print(__syx_value_description(omit))
  let with = Card(title: "t2", note: "n2")
  let s = __syx_json_encoder_encode(with, "Card")
  print(s)
  let back = __syx_json_decoder_decode(s, "Card")
  print(__syx_value_description(back))
}
"#,
    );
    // Wave A W75 / ADR-419 / A-03: optional field omit/present product exact.
    assert_eq!(
        out,
        "{title: t}\n{\"note\":\"n2\",\"title\":\"t2\"}\n{note: n2, title: t2}",
        "optional field omit/present must exact: {out}"
    );
}

#[test]
fn m2_s7_02_dynamic_json_still_green() {
    let out = run(
        r#"
func main() {
  let s = __syx_json_encode(42)
  let v = __syx_json_decode(s)
  print(__syx_value_description(v))
}
"#,
    );
    let l = lines(&out);
    assert_eq!(l.get(0).copied(), Some("42"), "dynamic JSON: {out}");
}

#[test]
fn m2_s7_02_box_roundtrip_native_mir() {
    let out = lines_joined(&run_native_mir_src("box_rt_mir", BOX_ROUNDTRIP_BODY));
    assert_box_roundtrip_line(&out);
}

#[test]
fn m2_s7_02_box_roundtrip_wasm() {
    let out = lines_joined(&run_wasm_src("box_rt_wasm", BOX_ROUNDTRIP_BODY));
    assert_box_roundtrip_line(&out);
}

#[test]
fn m2_s7_02_box_encode_native_llvm() {
    let (code, stdout) = run_native_strict(BOX_ENCODE_ONLY_BODY);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(out, "{\"n\":42}", "native llvm Box encode must exact: {out}");
}

#[test]
fn m2_s7_02_box_decode_native_llvm() {
    let (code, stdout) = run_native_strict(BOX_DECODE_ONLY_BODY);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(out, "{n: 42}", "native llvm Box decode must exact: {out}");
}

#[test]
fn m2_s7_02_surface_files_static() {
    let enc = include_str!("../../../../packages/core/json_encoder.sy");
    assert!(
        enc.contains("JSONEncoder")
            && enc.contains("JSONDecoder")
            && enc.contains("__syx_json_encoder_encode")
            && enc.contains("__syx_json_decoder_decode")
            && enc.contains("throws"),
        "json_encoder.sy surface: {enc}"
    );
    let json = include_str!("../../../../packages/core/json.sy");
    assert!(
        json.contains("JSONEncoder") && json.contains("JSONDecoder"),
        "json.sy must expose JSONEncoder/JSONDecoder"
    );
}

#[test]
fn m2_s7_02_docs_claim_encoder_fence_streaming() {
    let usage = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        usage.contains("ADR-231")
            && usage.contains("JSONEncoder")
            && usage.contains("codingPath")
            && (usage.contains("M2-S7-03") || usage.contains("streaming")),
        "UsageGuide must document JSONEncoder + S7-03 fence"
    );
    let lr = include_str!("../../../../books/language-reference/09-protocols-and-conformances.md");
    assert!(
        lr.contains("ADR-231") && lr.contains("codingPath") && lr.contains("M2-S7-03"),
        "language-reference 09 must document S7-02 + streaming fence"
    );
}

#[test]
fn m2_s7_02_host_inventory() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in ["JsonEncoderEncode", "JsonDecoderDecode"] {
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
        let status_line = block
            .lines()
            .find(|l| l.trim_start().starts_with("status ="))
            .unwrap_or("");
        assert!(
            (status_line.contains("status = \"partial\"")
                || status_line.contains("status = \"implemented\""))
                && (block.contains("ADR-231")
                    || block.contains("ADR-568")
                    || block.contains("ADR-569")
                    || block.contains("ADR-606")
                    || block.contains("ADR-1346")),
            "{name} must keep status_line + ADR-231/568/569/606/1346: {block}"
        );
        assert!(
            block.contains("streaming")
                || block.contains("S7-03")
                || block.contains("CodingKeys")
                || block.contains("Decode llvm")
                || block.contains("wasm GA"),
            "{name} must fence streaming/CodingKeys/wasm: {block}"
        );
    }
}

#[test]
fn m2_s7_02_json_decoder_decode_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "JsonDecoderDecode");
    assert!(
        block.contains("llvm = \"real\""),
        "JsonDecoderDecode must declare llvm=real (ADR-569): {block}"
    );
    let status_line = block
        .lines()
        .find(|l| l.trim_start().starts_with("status ="))
        .unwrap_or("");
    assert!(
        status_line.contains("status = \"partial\"")
            || status_line.contains("status = \"implemented\""),
        "JsonDecoderDecode must keep status_line: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "JsonDecoderDecode must declare wasm=real (ADR-606): {block}"
    );
    assert!(
        block.contains("ADR-606") && block.contains("ADR-569"),
        "notes must cite ADR-606 + ADR-569: {block}"
    );
}

#[test]
fn m2_s7_02_json_encoder_encode_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "JsonEncoderEncode");
    assert!(
        block.contains("llvm = \"real\""),
        "JsonEncoderEncode must declare llvm=real (ADR-568): {block}"
    );
    let status_line = block
        .lines()
        .find(|l| l.trim_start().starts_with("status ="))
        .unwrap_or("");
    assert!(
        status_line.contains("status = \"partial\"")
            || status_line.contains("status = \"implemented\""),
        "JsonEncoderEncode must keep status_line: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "JsonEncoderEncode must declare wasm=real (ADR-606): {block}"
    );
    assert!(
        block.contains("ADR-606") && block.contains("ADR-568"),
        "notes must cite ADR-606 + ADR-568: {block}"
    );
}

#[test]
fn m2_s7_02_json_encode_decode_native_llvm() {
    let (code, stdout) = run_native_strict(DYNAMIC_JSON_ROUNDTRIP);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(
        out, "42\n42",
        "native llvm json_encode/decode 42 must exact: {out}"
    );
}

#[test]
fn m2_s7_02_json_encode_decode_wasm() {
    let out = lines_joined(&run_wasm_src("dyn_json_wasm", DYNAMIC_JSON_ROUNDTRIP));
    assert_eq!(
        out, "42\n42",
        "wasm json_encode/decode 42 must exact: {out}"
    );
}

#[test]
fn m2_s7_02_json_encode_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "JsonEncode");
    assert!(
        block.contains("llvm = \"real\""),
        "JsonEncode must declare llvm=real (ADR-583): {block}"
    );
    let status_line = block
        .lines()
        .find(|l| l.trim_start().starts_with("status ="))
        .unwrap_or("");
    assert!(
        status_line.contains("status = \"partial\"")
            || status_line.contains("status = \"implemented\""),
        "JsonEncode must keep status_line: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "JsonEncode must declare wasm=real (ADR-606): {block}"
    );
    assert!(
        block.contains("ADR-606") && block.contains("ADR-583"),
        "notes must cite ADR-606 + ADR-583: {block}"
    );
}

#[test]
fn m2_s7_02_json_decode_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "JsonDecode");
    assert!(
        block.contains("llvm = \"real\""),
        "JsonDecode must declare llvm=real (ADR-583): {block}"
    );
    let status_line = block
        .lines()
        .find(|l| l.trim_start().starts_with("status ="))
        .unwrap_or("");
    assert!(
        status_line.contains("status = \"partial\"")
            || status_line.contains("status = \"implemented\""),
        "JsonDecode must keep status_line: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "JsonDecode must declare wasm=real (ADR-606): {block}"
    );
    assert!(
        block.contains("ADR-606") && block.contains("ADR-583"),
        "notes must cite ADR-606 + ADR-583: {block}"
    );
}

#[test]
fn m2_s7_02_product_json_encoder() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "data.coding.json_encoder");
    assert!(
        block.contains("status = \"implemented\""),
        "data.coding.json_encoder must be implemented: {block}"
    );
    assert!(
        block.contains("m2_s7_02_json_encoder_decoder_field_paths"),
        "tests must include nail: {block}"
    );
    assert!(
        block.contains("ADR-231")
            || block.contains("M2-S7-02")
            || block.contains("ADR-562"),
        "notes must cite ADR-231 or ADR-562: {block}"
    );
    assert!(
        block.contains("streaming")
            || block.contains("S7-03")
            || block.contains("CodingKeys")
            || block.contains("IANA")
            || block.contains("JSONStream")
            || block.contains("nested/codingPath"),
        "notes must keep honesty fence: {block}"
    );
}

#[test]
fn m2_s7_02_json_encoder_product_backends_real() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "data.coding.json_encoder");
    assert!(
        block.contains("native = \"real\""),
        "data.coding.json_encoder must declare native=real (ADR-562): {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "data.coding.json_encoder must declare wasm=real (ADR-562): {block}"
    );
    assert!(
        block.contains("status = \"implemented\""),
        "data.coding.json_encoder must keep status=implemented: {block}"
    );
    assert!(
        block.contains("ADR-562"),
        "notes must cite ADR-562: {block}"
    );
    assert!(
        block.contains("m2_s7_02_box_roundtrip_native_mir"),
        "tests must include native mir nail: {block}"
    );
}

#[test]
fn m2_s7_02_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m2_s7_02_json_encoder_decoder_field_paths_tier"),
        "inventory must contain m2_s7_02_json_encoder_decoder_field_paths_tier"
    );
    assert!(
        inv.contains("signed-ADR-231") || inv.contains("ADR-231"),
        "inventory must note ADR-231"
    );
}

#[test]
fn m2_s7_02_ci_wires() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test stdlib"),
        "ci.yml must wire m2_s7_02_json_encoder_decoder_field_paths"
    );
}

#[test]
fn m2_s7_02_registry_hosts() {
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(
        reg.contains("JsonEncoderEncode")
            && reg.contains("__syx_json_encoder_encode")
            && reg.contains("JsonDecoderDecode")
            && reg.contains("__syx_json_decoder_decode"),
        "registry must list JSONEncoder/Decoder hosts"
    );
}
