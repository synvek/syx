//! 01 Encoder/Decoder protocol surface.
//! data.coding.protocol native/wasm product nails (JsonEncode/JsonDecode roundtrip).
// was: m2_s7_01_encoder_decoder_protocol

use std::fs;
use std::path::PathBuf;

use syx_driver::{
    compile_and_run_embedded, run_native_mir_file_printed, run_wasm_source_printed, DriverOptions,
};

fn product_block<'a>(caps: &'a str, id: &str) -> &'a str {
    let needle = format!("id = \"{id}\"");
    let rest = caps
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing capability {id}"));
    rest.split("[[capability]]").next().unwrap_or(rest)
}

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn write_src_tmp(label: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "syx_w217_{}_{}",
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

const JSON_ROUNDTRIP_BODY: &str = r#"
func main() {
  let s = __syx_json_encode(42)
  let v = __syx_json_decode(s)
  print(__syx_value_description(v))
}
"#;

fn assert_json_roundtrip_line(out: &str) {
    assert_eq!(out, "42", "JSON roundtrip must print 42: {out:?}");
}

#[test]
fn m2_s7_01_protocol_files_static() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../packages/core/protocols/");
    for name in [
        "encoder.sy",
        "decoder.sy",
        "encodable.sy",
        "decodable.sy",
        "codable.sy",
    ] {
        let path = format!("{root}{name}");
        let src = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));
        assert!(
            src.contains("public protocol"),
            "{name} must declare a protocol"
        );
    }
    let codable = std::fs::read_to_string(format!("{root}codable.sy")).unwrap();
    assert!(
        codable.contains("Codable: Encodable, Decodable"),
        "Codable must inherit Encodable, Decodable: {codable}"
    );
}

#[test]
fn m2_s7_01_json_roundtrip_still_green() {
    let out = run(JSON_ROUNDTRIP_BODY);
    let l = lines(&out);
    assert_eq!(l.get(0).copied(), Some("42"), "JSON roundtrip: {out}");
}

#[test]
fn m2_s7_01_json_roundtrip_native_mir() {
    let out = lines_joined(&run_native_mir_src("json_rt_mir", JSON_ROUNDTRIP_BODY));
    assert_json_roundtrip_line(&out);
}

#[test]
fn m2_s7_01_json_roundtrip_wasm() {
    let out = lines_joined(&run_wasm_src("json_rt_wasm", JSON_ROUNDTRIP_BODY));
    assert_json_roundtrip_line(&out);
}

#[test]
fn m2_s7_01_codable_conformance_decl() {
    let out = run(
        r#"
@Codable
data class Point(x: Int, y: Int)

func main() {
  let p = Point(x: 1, y: 2)
  let s = p.encode()
  print(s)
}
"#,
    );
    let l = lines(&out);
    let encoded = l.get(0).copied().unwrap_or("");
    assert!(
        !encoded.is_empty() && (encoded.contains('1') || encoded.contains('{')),
        "@Codable encode non-empty JSON-ish: {out}"
    );
}

#[test]
fn m2_s7_01_docs_claim_protocol() {
    let usage = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        usage.contains("ADR-230")
            && usage.contains("Encoder")
            && (usage.contains("M2-S7-02") || usage.contains("JSONEncoder")),
        "UsageGuide must document coding protocol + S7-02 fence"
    );
    let lr = include_str!("../../../../books/language-reference/09-protocols-and-conformances.md");
    assert!(
        lr.contains("ADR-230")
            && lr.contains("Codable: Encodable, Decodable")
            && lr.contains("Encoder"),
        "language-reference 09 must document protocol surface"
    );
}

#[test]
fn m2_s7_01_product_coding_protocol() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "data.coding.protocol");
    assert!(
        block.contains("status = \"implemented\""),
        "data.coding.protocol must be implemented: {block}"
    );
    assert!(
        block.contains("m2_s7_01_encoder_decoder_protocol"),
        "tests must include nail: {block}"
    );
    assert!(
        block.contains("ADR-230")
            || block.contains("M2-S7-01")
            || block.contains("ADR-561"),
        "notes must cite ADR-230 or ADR-561: {block}"
    );
    assert!(
        block.contains("S7-02")
            || block.contains("JSONEncoder")
            || block.contains("streaming")
            || block.contains("IANA")
            || block.contains("JSONStream")
            || block.contains("@Codable"),
        "notes must keep honesty fence: {block}"
    );
}

#[test]
fn m2_s7_01_coding_protocol_product_backends_real() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "data.coding.protocol");
    assert!(
        block.contains("native = \"real\""),
        "data.coding.protocol must declare native=real (ADR-561): {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "data.coding.protocol must declare wasm=real (ADR-561): {block}"
    );
    assert!(
        block.contains("status = \"implemented\""),
        "data.coding.protocol must keep status=implemented: {block}"
    );
    assert!(
        block.contains("ADR-561"),
        "notes must cite ADR-561: {block}"
    );
    assert!(
        block.contains("m2_s7_01_json_roundtrip_native_mir"),
        "tests must include native mir nail: {block}"
    );
}

#[test]
fn m2_s7_01_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m2_s7_01_encoder_decoder_protocol_tier"),
        "inventory must contain m2_s7_01_encoder_decoder_protocol_tier"
    );
    assert!(
        inv.contains("ADR-230") || inv.contains("signed-ADR-230"),
        "inventory must note ADR-230"
    );
}

#[test]
fn m2_s7_01_known_protocol_static() {
    let src = include_str!("../../../syx-stdlib/src/protocols.rs");
    assert!(
        src.contains("\"Encoder\"") && src.contains("\"Decoder\""),
        "known_protocol must list Encoder/Decoder"
    );
}
