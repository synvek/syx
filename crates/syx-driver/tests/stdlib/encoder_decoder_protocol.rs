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
fn m2_s7_01_host_inventory_partial() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let matrix = include_str!("../../../../packages/internal/hostfn_qs1_matrix.toml");
    let tip_w1024 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-seventy-fourth");
    let tip_w1025 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-seventy-fifth");
    let tip_w1026 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-seventy-sixth");
    let tip_w1027 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-seventy-seventh");
    let tip_w1028 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-seventy-eighth");
let tip_w1029 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-seventy-ninth");
let tip_w1030 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-eightieth");
let tip_w1031 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-eighty-first");
let tip_w1032 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-eighty-second");
let tip_w1033 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-eighty-third");
let tip_w1034 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-eighty-fourth");
let tip_w1035 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-eighty-fifth");
let tip_w1036 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-eighty-sixth");
let tip_w1037 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-eighty-seventh");
let tip_w1038 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-eighty-eighth");
let tip_w1039 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-eighty-ninth");
let tip_w1040 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-ninetieth");
let tip_w1041 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-ninety-first");
let tip_w1042 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-ninety-second");
let tip_w1043 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-ninety-third");
let tip_w1044 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-ninety-fourth");
let tip_w1045 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-ninety-fifth");
let tip_w1046 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-ninety-sixth");
let tip_w1047 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-ninety-seventh");
let tip_w1048 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-ninety-eighth");
let tip_w1049 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("eight-hundred-and-ninety-ninth");
let tip_w1050 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("nine-hundredth");
let tip_w1051 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("nine-hundred-and-first");
let tip_w1052 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("nine-hundred-and-second");
let tip_w1053 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("nine-hundred-and-third");
let tip_w1054 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("nine-hundred-and-fourth");
let tip_w1055 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("nine-hundred-and-fifth");
let tip_w1056 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("nine-hundred-and-sixth");
let tip_w1057 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("nine-hundred-and-seventh");
let tip_w1058 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("nine-hundred-and-eighth");
let tip_w1059 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("nine-hundred-and-ninth");
let tip_w1060 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("nine-hundred-and-tenth");
let tip_w1061 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("nine-hundred-and-eleventh");
let tip_w1062 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("nine-hundred-and-twelfth");
let tip_w1063 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("nine-hundred-and-thirteenth");
let tip_w1064 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("nine-hundred-and-fourteenth");
let tip_w1065 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("nine-hundred-and-fifteenth");
let tip_w1066 = matrix.contains("wave = \"PUBLIC_WAVE\"")
        && matrix.contains("nine-hundred-and-sixteenth");
    for name in ["JsonEncode", "JsonDecode", "CodableEncode", "CodableDecode"] {
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
        let status_ok = if name.starts_with("Json") {
            status_line.contains("status = \"implemented\"")
        } else {
            status_line.contains("status = \"partial\"")
                || ((tip_w1024 || tip_w1025 || tip_w1026 || tip_w1027 || tip_w1028 || tip_w1029 || tip_w1030 || tip_w1031 || tip_w1032 || tip_w1033 || tip_w1034 || tip_w1035 || tip_w1036 || tip_w1037 || tip_w1038 || tip_w1039 || tip_w1040 || tip_w1041 || tip_w1042 || tip_w1043 || tip_w1044 || tip_w1045 || tip_w1046 || tip_w1047 || tip_w1048 || tip_w1049 || tip_w1050 || tip_w1051 || tip_w1052 || tip_w1053 || tip_w1054 || tip_w1055 || tip_w1056 || tip_w1057 || tip_w1058 || tip_w1059 || tip_w1060 || tip_w1061 || tip_w1062 || tip_w1063 || tip_w1064 || tip_w1065 || tip_w1066) && status_line.contains("status = \"implemented\""))
        };
        let adr_ok = if name.starts_with("Json") {
            block.contains("ADR-230")
                || block.contains("ADR-1346")
                || block.contains("ADR-606")
        } else {
            block.contains("ADR-230")
        };
        assert!(
            status_ok && adr_ok,
            "{name} must keep status_line + ADR notes: {block}"
        );
    }
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
fn m2_s7_01_ci_wires() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test stdlib"),
        "ci.yml must wire m2_s7_01_encoder_decoder_protocol"
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
