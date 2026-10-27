//! 03 ISO8601/RFC3339 standard-set round-trip.
//! DateTimeFormat HostFn llvm=real (UTC native AOT nail).
//! DateTimeParseIso8601 HostFn llvm=real (UTC native AOT nail).
//! DateTimeParse HostFn llvm=real (date-only native AOT nail).
//! DateTimeFormat HostFn wasm=real (UTC wasm nail).
//! DateTimeParse + DateTimeParseIso8601 HostFn wasm=real.
// was: m2_s5_03_iso_rfc3339_roundtrip

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use syx_driver::{
    build_native_executable, compile_and_run_embedded, run_wasm_source_printed, DriverOptions,
};

fn host_block<'a>(toml: &'a str, name: &str) -> &'a str {
    let needle = format!("name = \"{name}\"");
    let rest = toml
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing host override {name}"));
    rest.split("[[host]]").next().unwrap_or(rest)
}

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

fn run_err(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    match compile_and_run_embedded(src, &DriverOptions::default()) {
        Ok(r) => panic!("expected failure, got stdout={:?}", r.printed),
        Err(e) => format!("{e}"),
    }
}

fn run_native_strict(src: &str) -> (i32, String) {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_w226_utc_format_llvm_{}",
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

fn write_src_tmp(label: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "syx_w259_{}_{}",
        label,
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("main.sy");
    fs::write(&path, src).unwrap();
    path
}

fn run_wasm_src(label: &str, src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = write_src_tmp(label, src);
    let (code, lines) =
        run_wasm_source_printed(&path, "main", &DriverOptions::default()).expect("wasm");
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

const UTC_FORMAT_ONLY_SRC: &str = r#"
func main() {
  print(__syx_datetime_format(86400000, "iso"))
}
"#;

const UTC_PARSE_ISO8601_ONLY_SRC: &str = r#"
func main() {
  let o = __syx_datetime_parse_iso8601("1970-01-02T00:00:00.000Z")
  print(__syx_keypath_get(o, "DateTime", "ms", false))
}
"#;

const DATE_ONLY_PARSE_SRC: &str = r#"
func main() {
  let o = __syx_datetime_parse("2020-02-29")
  print(__syx_datetime_format(__syx_keypath_get(o, "DateTime", "ms", false), "%Y-%m-%d"))
}
"#;

#[test]
fn m2_s5_03_host_fn_columns() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    {
        let block = host_block(toml, "DateTimeParse");
        assert!(
            block.contains("vm = \"real\""),
            "DateTimeParse must declare vm=real: {block}"
        );
        assert!(
            block.contains("mir = \"real\""),
            "DateTimeParse must declare mir=real: {block}"
        );
        assert!(
            block.contains("llvm = \"real\""),
            "DateTimeParse must declare llvm=real (ADR-572): {block}"
        );
        assert!(
            block.contains("ADR-572")
                || block.contains("ADR-221")
                || block.contains("M2-S5-03"),
            "DateTimeParse notes must cite ADR-572/ADR-221: {block}"
        );
    }
    {
        let block = host_block(toml, "DateTimeParseIso8601");
        assert!(
            block.contains("vm = \"real\""),
            "DateTimeParseIso8601 must declare vm=real: {block}"
        );
        assert!(
            block.contains("mir = \"real\""),
            "DateTimeParseIso8601 must declare mir=real: {block}"
        );
        assert!(
            block.contains("llvm = \"real\""),
            "DateTimeParseIso8601 must declare llvm=real (ADR-571): {block}"
        );
        assert!(
            block.contains("ADR-571")
                || block.contains("ADR-221")
                || block.contains("M2-S5-03"),
            "DateTimeParseIso8601 notes must cite ADR-571/ADR-221: {block}"
        );
    }
    {
        let block = host_block(toml, "DateTimeFormat");
        assert!(
            block.contains("vm = \"real\""),
            "DateTimeFormat must declare vm=real: {block}"
        );
        assert!(
            block.contains("mir = \"real\""),
            "DateTimeFormat must declare mir=real: {block}"
        );
        assert!(
            block.contains("llvm = \"real\""),
            "DateTimeFormat must declare llvm=real (ADR-570): {block}"
        );
        assert!(
            block.contains("ADR-570")
                || block.contains("ADR-221")
                || block.contains("M2-S5-03"),
            "DateTimeFormat notes must cite ADR-570/ADR-221: {block}"
        );
    }
    // ADR-233 / M2-S5-02 unlocked IANA hosts; W222/W223 deepened llvm nails.
    for name in ["DateTimeFormatWithTimezone", "DateTimeTimezoneOffset"] {
        assert!(
            toml.contains(&format!("name = \"{name}\"")),
            "{name} must be overridden by ADR-233 IANA window"
        );
        let block = host_block(toml, name);
        assert!(
            block.contains("ADR-233")
                || block.contains("M2-S5-02")
                || block.contains("ADR-566")
                || block.contains("ADR-567"),
            "{name} notes must cite ADR-233/ADR-566/ADR-567: {block}"
        );
    }
}

#[test]
fn m2_s5_03_utc_format_native_llvm() {
    let (code, stdout) = run_native_strict(UTC_FORMAT_ONLY_SRC);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(
        out, "1970-01-02T00:00:00.000Z",
        "native llvm UTC format must exact: {out}"
    );
}

#[test]
fn m2_s5_03_utc_format_wasm() {
    let out = lines_joined(&run_wasm_src("utc_fmt", UTC_FORMAT_ONLY_SRC));
    assert_eq!(
        out, "1970-01-02T00:00:00.000Z",
        "wasm UTC format must exact: {out}"
    );
}

#[test]
fn m2_s5_03_utc_parse_iso8601_native_llvm() {
    let (code, stdout) = run_native_strict(UTC_PARSE_ISO8601_ONLY_SRC);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(
        out, "86400000",
        "native llvm UTC parse_iso8601 must exact: {out}"
    );
}

#[test]
fn m2_s5_03_utc_parse_iso8601_wasm() {
    let out = lines_joined(&run_wasm_src("utc_parse_iso", UTC_PARSE_ISO8601_ONLY_SRC));
    assert_eq!(
        out, "86400000",
        "wasm UTC parse_iso8601 must exact: {out}"
    );
}

#[test]
fn m2_s5_03_datetime_format_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "DateTimeFormat");
    assert!(
        block.contains("llvm = \"real\""),
        "DateTimeFormat must declare llvm=real (ADR-570/ADR-603): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "DateTimeFormat must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "DateTimeFormat must declare wasm=real (ADR-603): {block}"
    );
    assert!(
        block.contains("ADR-603") || block.contains("ADR-570"),
        "notes must cite ADR-603/ADR-570: {block}"
    );
}

#[test]
fn m2_s5_03_datetime_parse_iso8601_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "DateTimeParseIso8601");
    assert!(
        block.contains("llvm = \"real\""),
        "DateTimeParseIso8601 must declare llvm=real (ADR-571/ADR-604): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "DateTimeParseIso8601 must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "DateTimeParseIso8601 must declare wasm=real (ADR-604): {block}"
    );
    assert!(
        block.contains("ADR-604") || block.contains("ADR-571"),
        "notes must cite ADR-604/ADR-571: {block}"
    );
}

#[test]
fn m2_s5_03_date_only_parse_native_llvm() {
    let (code, stdout) = run_native_strict(DATE_ONLY_PARSE_SRC);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(
        out, "2020-02-29",
        "native llvm date-only parse must exact: {out}"
    );
}

#[test]
fn m2_s5_03_date_only_parse_wasm() {
    let out = lines_joined(&run_wasm_src("date_only_parse", DATE_ONLY_PARSE_SRC));
    assert_eq!(
        out, "2020-02-29",
        "wasm date-only parse must exact: {out}"
    );
}

#[test]
fn m2_s5_03_datetime_parse_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "DateTimeParse");
    assert!(
        block.contains("llvm = \"real\""),
        "DateTimeParse must declare llvm=real (ADR-572/ADR-604): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "DateTimeParse must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "DateTimeParse must declare wasm=real (ADR-604): {block}"
    );
    assert!(
        block.contains("ADR-604") || block.contains("ADR-572"),
        "notes must cite ADR-604/ADR-572: {block}"
    );
}

#[test]
fn m2_s5_03_product_columns() {
    // Wave A W284 / ADR-628: datetime.calendar dual-backend real + status=implemented.
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "datetime.calendar");
    assert!(
        block.contains("vm = \"real\""),
        "datetime.calendar vm must be real: {block}"
    );
    assert!(
        block.contains("mir = \"real\""),
        "datetime.calendar mir must be real: {block}"
    );
    assert!(
        block.contains("native = \"real\""),
        "datetime.calendar native must be real (ADR-628): {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "datetime.calendar wasm must be real (ADR-628): {block}"
    );
    assert!(
        block.contains("status = \"implemented\""),
        "datetime.calendar must be implemented (ADR-628): {block}"
    );
    assert!(
        block.contains("ADR-628") && block.contains("W284"),
        "notes must cite ADR-628+W284: {block}"
    );
    assert!(
        block.contains("m2_s5_03_iso_rfc3339_roundtrip"),
        "tests must include nail: {block}"
    );
    assert!(
        block.contains("ADR-221")
            || block.contains("M2-S5-03")
            || block.contains("ADR-222")
            || block.contains("M2-S5-04")
            || block.contains("RFC3339"),
        "notes must cite ISO/RFC3339 honesty lineage: {block}"
    );
}

#[test]
fn m2_s5_03_utc_millis_roundtrip() {
    // Wave A W68 / ADR-412 / A-03: UTC ISO millis + format product exact.
    let out = run(
        r#"func main() {
    let o = __syx_datetime_parse_iso8601("1970-01-02T00:00:00.000Z")
    print(__syx_keypath_get(o, "DateTime", "ms", false))
    print(__syx_datetime_format(86400000, "iso"))
}"#,
    );
    assert_eq!(
        out, "86400000\n1970-01-02T00:00:00.000Z",
        "UTC millis roundtrip must exact: {out}"
    );
}

#[test]
fn m2_s5_03_fixed_offset_to_utc_ms() {
    // Wave A W70 / ADR-414 / A-03: fixed-offset ISO → UTC ms product exact.
    let out = run(
        r#"func main() {
    let o = __syx_datetime_parse_iso8601("1970-01-02T01:00:00.000+01:00")
    print(__syx_keypath_get(o, "DateTime", "ms", false))
}"#,
    );
    assert_eq!(out, "86400000", "fixed offset must map to UTC ms: {out}");
}

#[test]
fn m2_s5_03_negative_epoch_roundtrip() {
    // Wave A W69 / ADR-413 / A-03: negative-epoch ISO roundtrip product exact.
    let out = run(
        r#"func main() {
    let o = __syx_datetime_parse_iso8601("1969-12-31T23:59:59.000Z")
    let ms = __syx_keypath_get(o, "DateTime", "ms", false)
    print(ms)
    let s = __syx_datetime_format(ms, "iso")
    let o2 = __syx_datetime_parse_iso8601(s)
    print(__syx_keypath_get(o2, "DateTime", "ms", false))
}"#,
    );
    assert_eq!(
        out, "-1000\n-1000",
        "negative-epoch roundtrip must exact: {out}"
    );
}

#[test]
fn m2_s5_03_leap_day_roundtrip() {
    // Wave A W68 / ADR-412 / A-03: leap-day ISO roundtrip product exact.
    let out = run(
        r#"func main() {
    let o = __syx_datetime_parse_iso8601("2020-02-29T12:00:00.000Z")
    let ms = __syx_keypath_get(o, "DateTime", "ms", false)
    let s = __syx_datetime_format(ms, "iso")
    let o2 = __syx_datetime_parse_iso8601(s)
    print(__syx_keypath_get(o2, "DateTime", "ms", false) == ms)
    print(s)
}"#,
    );
    assert_eq!(
        out, "true\n2020-02-29T12:00:00.000Z",
        "leap-day roundtrip must exact: {out}"
    );
}

#[test]
fn m2_s5_03_post_2038_roundtrip() {
    // Wave A W69 / ADR-413 / A-03: post-2038 ISO roundtrip product exact.
    let out = run(
        r#"func main() {
    let o = __syx_datetime_parse_iso8601("2100-01-01T00:00:00.000Z")
    let ms = __syx_keypath_get(o, "DateTime", "ms", false)
    let s = __syx_datetime_format(ms, "iso")
    let o2 = __syx_datetime_parse_iso8601(s)
    print(__syx_keypath_get(o2, "DateTime", "ms", false) == ms)
    print(s)
}"#,
    );
    assert_eq!(
        out, "true\n2100-01-01T00:00:00.000Z",
        "post-2038 roundtrip must exact: {out}"
    );
}

#[test]
fn m2_s5_03_date_only_yyyy_mm_dd() {
    // Wave A W70 / ADR-414 / A-03: date-only parse + format product exact.
    let out = run(
        r#"func main() {
    let o = __syx_datetime_parse("2020-02-29")
    print(__syx_datetime_format(__syx_keypath_get(o, "DateTime", "ms", false), "%Y-%m-%d"))
}"#,
    );
    assert_eq!(out, "2020-02-29", "date-only format must exact: {out}");
}

#[test]
fn m2_s5_03_fail_closed_invalid_and_iana() {
    // Wave A W91 / ADR-435 / A-03: ISO invalid + IANA unknown fail-closed product exact.
    let bad = run_err(r#"func main() { print(__syx_datetime_parse_iso8601("not-a-date")) }"#);
    assert_eq!(
        bad,
        "[U002] DateTime.parse: expected RFC3339 or YYYY-MM-DD, got \"not-a-date\"",
        "invalid ISO must exact: {bad}"
    );
    // ADR-233: known IANA zones are implemented; unknown zone still fail-closed U001.
    let tz = run_err(
        r#"func main() { print(__syx_datetime_timezone_offset(0, "Not/AZone")) }"#,
    );
    assert_eq!(
        tz,
        "[U001] DateTime: unknown IANA timezone \"Not/AZone\" (no tzdb match)",
        "unknown tz must exact: {tz}"
    );
}

#[test]
fn m2_s5_03_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m2_s5_03_iso_rfc3339_roundtrip_tier"),
        "inventory must contain m2_s5_03_iso_rfc3339_roundtrip_tier"
    );
    assert!(
        inv.contains("ADR-221") || inv.contains("signed-ADR-221"),
        "inventory must note ADR-221"
    );
}

#[test]
fn m2_s5_03_ci_wires() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test stdlib"),
        "ci.yml must wire m2_s5_03_iso_rfc3339_roundtrip"
    );
}
