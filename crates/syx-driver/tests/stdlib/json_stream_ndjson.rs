//! 03 JSONStream NDJSON session + chunk feed.
//! data.coding.json_stream native/wasm product nails (Box stream roundtrip).
//! JsonStream writer HostFn llvm=real (native AOT nail).
//! JsonStream reader HostFn llvm=real (native AOT nail).
//! JsonStream reader HostFn wasm=real (reuse box_stream_roundtrip_wasm).
//! JsonStream writer HostFn wasm=real (writer finish wasm + JsonStream* wasm 收官).
// was: m2_s7_03_json_stream_ndjson

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
        "syx_w219_{}_{}",
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

const BOX_STREAM_ROUNDTRIP_BODY: &str = r#"
data class Box(n: Int)

func main() {
  let b = Box(n: 42)
  let w = __syx_json_stream_open_writer()
  __syx_json_stream_write(w, b, "Box")
  let text = __syx_json_stream_finish_writer(w)
  let r = __syx_json_stream_open_reader()
  __syx_json_stream_feed(r, text)
  let back = __syx_json_stream_decode_next(r, "Box")
  print(__syx_value_description(back))
  __syx_json_stream_close_reader(r)
}
"#;

const BOX_STREAM_WRITER_FINISH: &str = r#"
data class Box(n: Int)

func main() {
  let b = Box(n: 42)
  let w = __syx_json_stream_open_writer()
  __syx_json_stream_write(w, b, "Box")
  print(__syx_json_stream_finish_writer(w))
}
"#;

fn run_native_strict(src: &str) -> (i32, String) {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_w239_json_stream_writer_llvm_{}",
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

fn assert_box_stream_roundtrip_line(out: &str) {
    assert_eq!(
        out, "{n: 42}",
        "Box stream roundtrip must print {{n: 42}}: {out:?}"
    );
}

#[test]
fn m2_s7_03_writer_two_users_ndjson() {
    let out = run(
        r#"
data class Address(city: String, zip: String)
data class User(name: String, address: Address)

func main() {
  try {
    let a1 = Address(city: "SF", zip: "94102")
    let u1 = User(name: "ada", address: a1)
    let a2 = Address(city: "NY", zip: "10001")
    let u2 = User(name: "bob", address: a2)
    let w = __syx_json_stream_open_writer()
    __syx_json_stream_write(w, u1, "User")
    __syx_json_stream_write(w, u2, "User")
    let text = __syx_json_stream_finish_writer(w)
    print(text)
    let e1 = __syx_json_encoder_encode(u1, "User")
    print(e1)
  } catch (err) {
    print("UNEXPECTED")
    print(err)
  }
}
"#,
    );
    // Wave A W76 / ADR-420 / A-03: JSONStream writer NDJSON + buffered encode product exact.
    // finish_writer text ends with trailing newline → blank line before encode print.
    assert_eq!(
        out,
        "{\"address\":{\"city\":\"SF\",\"zip\":\"94102\"},\"name\":\"ada\"}\n{\"address\":{\"city\":\"NY\",\"zip\":\"10001\"},\"name\":\"bob\"}\n\n{\"address\":{\"city\":\"SF\",\"zip\":\"94102\"},\"name\":\"ada\"}",
        "JSONStream writer NDJSON roundtrip must exact: {out}"
    );
}

#[test]
fn m2_s7_03_reader_chunk_feed_decode_next() {
    let out = run(
        r#"
data class Address(city: String, zip: String)
data class User(name: String, address: Address)

func main() {
  try {
    let r = __syx_json_stream_open_reader()
    __syx_json_stream_feed(r, "{\"name\":\"ada\",\"address\":")
    __syx_json_stream_feed(r, "{\"city\":\"SF\",\"zip\":\"94102\"}}\n")
    __syx_json_stream_feed(r, "{\"name\":\"bob\",\"address\":{\"city\":\"NY\",\"zip\":\"10001\"}}\n")
    let u1 = __syx_json_stream_decode_next(r, "User")
    let u2 = __syx_json_stream_decode_next(r, "User")
    print(__syx_value_description(u1))
    print(__syx_value_description(u2))
    __syx_json_stream_close_reader(r)
  } catch (err) {
    print("UNEXPECTED")
    print(err)
  }
}
"#,
    );
    // Wave A W76 / ADR-420 / A-03: JSONStream reader chunk-feed decodeNext product exact.
    assert_eq!(
        out,
        "{address: {city: SF, zip: 94102}, name: ada}\n{address: {city: NY, zip: 10001}, name: bob}",
        "JSONStream reader chunk-feed decodeNext must exact: {out}"
    );
}

#[test]
fn m2_s7_03_missing_key_line_coding_path() {
    let out = run(
        r#"
data class Address(city: String, zip: String)
data class User(name: String, address: Address)

func main() {
  try {
    let r = __syx_json_stream_open_reader()
    __syx_json_stream_feed(r, "{\"name\":\"ada\"}\n")
    let back = __syx_json_stream_decode_next(r, "User")
    print("UNEXPECTED_OK")
    print(__syx_value_description(back))
  } catch (err) {
    print("caught")
    print(err.message)
  }
}
"#,
    );
    // Wave A W73 / ADR-417 / A-03: JSONStream missing-key line codingPath product exact.
    assert_eq!(
        out,
        "caught\nJSONDecoder missing required key 'address' codingPath=line[0].address",
        "JSONStream missing-key line codingPath must exact: {out}"
    );
}

#[test]
fn m2_s7_03_dynamic_json_still_green() {
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
fn m2_s7_03_box_stream_roundtrip_native_mir() {
    let out = lines_joined(&run_native_mir_src(
        "box_stream_mir",
        BOX_STREAM_ROUNDTRIP_BODY,
    ));
    assert_box_stream_roundtrip_line(&out);
}

#[test]
fn m2_s7_03_box_stream_roundtrip_wasm() {
    let out = lines_joined(&run_wasm_src(
        "box_stream_wasm",
        BOX_STREAM_ROUNDTRIP_BODY,
    ));
    assert_box_stream_roundtrip_line(&out);
}

#[test]
fn m2_s7_03_surface_files_static() {
    let src = include_str!("../../../../packages/core/json_stream.sy");
    assert!(
        src.contains("JSONStream")
            && src.contains("__syx_json_stream_open_writer")
            && src.contains("__syx_json_stream_decode_next")
            && src.contains("throws"),
        "json_stream.sy surface: {src}"
    );
    let json = include_str!("../../../../packages/core/json.sy");
    assert!(
        json.contains("JSONStream"),
        "json.sy must expose JSONStream"
    );
}

#[test]
fn m2_s7_03_docs_claim_stream_fence() {
    let usage = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        usage.contains("ADR-232")
            && usage.contains("JSONStream")
            && usage.contains("line[")
            && (usage.contains("M2-S5-02") || usage.contains("IANA"))
            && (usage.contains("CodingKeys") || usage.contains("S3-03")),
        "UsageGuide must document JSONStream + fences"
    );
    let lr = include_str!("../../../../books/language-reference/09-protocols-and-conformances.md");
    assert!(
        lr.contains("ADR-232") && lr.contains("JSONStream") && lr.contains("S7"),
        "language-reference 09 must document S7-03"
    );
}

#[test]
fn m2_s7_03_host_inventory() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in [
        "JsonStreamOpenWriter",
        "JsonStreamWrite",
        "JsonStreamFinishWriter",
        "JsonStreamOpenReader",
        "JsonStreamFeed",
        "JsonStreamDecodeNext",
        "JsonStreamCloseReader",
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
        assert!(
            block.contains("status = \"partial\"") && block.contains("ADR-232"),
            "{name} must be partial + ADR-232: {block}"
        );
    }
    for name in [
        "JsonStreamOpenWriter",
        "JsonStreamWrite",
        "JsonStreamFinishWriter",
    ] {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\"") && block.contains("ADR-583"),
            "{name} must declare llvm=real + ADR-583: {block}"
        );
        assert!(
            block.contains("wasm = \"real\"") && block.contains("ADR-609"),
            "{name} must declare wasm=real + ADR-609: {block}"
        );
    }
    for name in [
        "JsonStreamOpenReader",
        "JsonStreamFeed",
        "JsonStreamDecodeNext",
        "JsonStreamCloseReader",
    ] {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\"") && block.contains("ADR-584"),
            "{name} must declare llvm=real + ADR-584: {block}"
        );
        assert!(
            block.contains("wasm = \"real\"") && block.contains("ADR-608"),
            "{name} must declare wasm=real + ADR-608: {block}"
        );
    }
}

#[test]
fn m2_s7_03_json_stream_writer_native_llvm() {
    let (code, stdout) = run_native_strict(BOX_STREAM_WRITER_FINISH);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert!(
        out.contains("{\"n\":42}"),
        "native llvm finish_writer must contain {{\"n\":42}}: {out}"
    );
}

#[test]
fn m2_s7_03_json_stream_writer_finish_wasm() {
    let out = lines_joined(&run_wasm_src(
        "box_stream_writer_finish_wasm",
        BOX_STREAM_WRITER_FINISH,
    ));
    assert!(
        out.contains("{\"n\":42}"),
        "wasm finish_writer must contain {{\"n\":42}}: {out}"
    );
}

#[test]
fn m2_s7_03_json_stream_writer_hosts_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in [
        "JsonStreamOpenWriter",
        "JsonStreamWrite",
        "JsonStreamFinishWriter",
    ] {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must declare llvm=real (ADR-583): {block}"
        );
        assert!(
            block.contains("status = \"partial\""),
            "{name} must keep status=partial: {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-609): {block}"
        );
        assert!(
            block.contains("ADR-609"),
            "{name} notes must cite ADR-609: {block}"
        );
        assert!(
            block.contains("ADR-583"),
            "{name} notes must cite ADR-583: {block}"
        );
    }
}

#[test]
fn m2_s7_03_json_stream_writer_hosts_wasm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in [
        "JsonStreamOpenWriter",
        "JsonStreamWrite",
        "JsonStreamFinishWriter",
    ] {
        let block = host_block(toml, name);
        assert!(
            block.contains("wasm = \"real\"") && block.contains("ADR-609"),
            "{name} must declare wasm=real + ADR-609: {block}"
        );
        assert!(
            !block.contains("wasm = \"host-assisted\""),
            "{name} must not remain host-assisted: {block}"
        );
    }
}

#[test]
fn m2_s7_03_json_stream_reader_native_llvm() {
    let (code, stdout) = run_native_strict(BOX_STREAM_ROUNDTRIP_BODY);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(
        out, "{n: 42}",
        "native llvm JsonStream reader roundtrip must exact: {out}"
    );
}

#[test]
fn m2_s7_03_json_stream_reader_hosts_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in [
        "JsonStreamOpenReader",
        "JsonStreamFeed",
        "JsonStreamDecodeNext",
        "JsonStreamCloseReader",
    ] {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must declare llvm=real (ADR-584): {block}"
        );
        assert!(
            block.contains("status = \"partial\""),
            "{name} must keep status=partial: {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-608): {block}"
        );
        assert!(
            block.contains("ADR-608"),
            "{name} notes must cite ADR-608: {block}"
        );
        assert!(
            block.contains("ADR-584"),
            "{name} notes must cite ADR-584: {block}"
        );
        assert!(
            block.contains("ADR-232"),
            "{name} notes must retain ADR-232: {block}"
        );
    }
}

#[test]
fn m2_s7_03_json_stream_reader_hosts_wasm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in [
        "JsonStreamOpenReader",
        "JsonStreamFeed",
        "JsonStreamDecodeNext",
        "JsonStreamCloseReader",
    ] {
        let block = host_block(toml, name);
        assert!(
            block.contains("wasm = \"real\"") && block.contains("ADR-608"),
            "{name} must declare wasm=real + ADR-608: {block}"
        );
        assert!(
            !block.contains("wasm = \"host-assisted\""),
            "{name} must not remain host-assisted: {block}"
        );
    }
}

#[test]
fn m2_s7_03_product_json_stream() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "data.coding.json_stream");
    assert!(
        block.contains("status = \"implemented\""),
        "data.coding.json_stream must be implemented: {block}"
    );
    assert!(
        block.contains("m2_s7_03_json_stream_ndjson"),
        "tests must include nail: {block}"
    );
    assert!(
        block.contains("ADR-232")
            || block.contains("M2-S7-03")
            || block.contains("ADR-563"),
        "notes must cite ADR-232 or ADR-563: {block}"
    );
    assert!(
        block.contains("CodingKeys")
            || block.contains("IANA")
            || block.contains("S3-03")
            || block.contains("S4-04")
            || block.contains("File/HTTP")
            || block.contains("nested/codingPath"),
        "notes must keep honesty fence: {block}"
    );
}

#[test]
fn m2_s7_03_json_stream_product_backends_real() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "data.coding.json_stream");
    assert!(
        block.contains("native = \"real\""),
        "data.coding.json_stream must declare native=real (ADR-563): {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "data.coding.json_stream must declare wasm=real (ADR-563): {block}"
    );
    assert!(
        block.contains("status = \"implemented\""),
        "data.coding.json_stream must keep status=implemented: {block}"
    );
    assert!(
        block.contains("ADR-563"),
        "notes must cite ADR-563: {block}"
    );
    assert!(
        block.contains("m2_s7_03_box_stream_roundtrip_native_mir"),
        "tests must include native mir nail: {block}"
    );
}

#[test]
fn m2_s7_03_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m2_s7_03_json_stream_ndjson_tier"),
        "inventory must contain m2_s7_03_json_stream_ndjson_tier"
    );
    assert!(
        inv.contains("signed-ADR-232") || inv.contains("ADR-232"),
        "inventory must note ADR-232"
    );
}

#[test]
fn m2_s7_03_ci_wires() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test stdlib"),
        "ci.yml must wire m2_s7_03_json_stream_ndjson"
    );
}

#[test]
fn m2_s7_03_registry_hosts() {
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(
        reg.contains("JsonStreamOpenWriter")
            && reg.contains("__syx_json_stream_decode_next")
            && reg.contains("JsonStreamFeed"),
        "registry must list JSONStream hosts"
    );
}
