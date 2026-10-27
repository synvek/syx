//! 02 DateTime + IANA tzdb DST.
//! datetime.timezone native/wasm product nails (NY DST offset).
//! DateTimeTimezoneOffset HostFn llvm=real (native AOT nail).
//! DateTimeFormatWithTimezone HostFn llvm=real (native AOT nail).
//! Offset + FormatWithTimezone HostFn wasm=real.
// was: m2_s5_02_iana_tzdb_dst

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

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn write_src_tmp(label: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "syx_w221_{}_{}",
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

fn run_native_strict(src: &str) -> (i32, String) {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_w222_tz_offset_llvm_{}",
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

const NY_DST_OFFSETS_SRC: &str = r#"
func main() {
  try {
    let winter = __syx_datetime_parse_iso8601("2024-01-15T12:00:00.000Z")
    let summer = __syx_datetime_parse_iso8601("2024-07-15T12:00:00.000Z")
    let wMs = __syx_keypath_get(winter, "DateTime", "ms", false)
    let sMs = __syx_keypath_get(summer, "DateTime", "ms", false)
    print(__syx_datetime_timezone_offset(wMs, "America/New_York"))
    print(__syx_datetime_timezone_offset(sMs, "America/New_York"))
  } catch (err) {
    print("UNEXPECTED")
    print(err)
  }
}
"#;

const NY_WINTER_OFFSET_SRC: &str = r#"
func main() {
  try {
    let winter = __syx_datetime_parse_iso8601("2024-01-15T12:00:00.000Z")
    let wMs = __syx_keypath_get(winter, "DateTime", "ms", false)
    print(__syx_datetime_timezone_offset(wMs, "America/New_York"))
  } catch (err) {
    print("UNEXPECTED")
    print(err)
  }
}
"#;

const NY_WINTER_FORMAT_SRC: &str = r#"
func main() {
  try {
    let winter = __syx_datetime_parse_iso8601("2024-01-15T12:00:00.000Z")
    let wMs = __syx_keypath_get(winter, "DateTime", "ms", false)
    print(__syx_datetime_format_with_timezone(wMs, "iso", "America/New_York"))
  } catch (err) {
    print("UNEXPECTED")
    print(err)
  }
}
"#;

#[test]
fn m2_s5_02_ny_dst_offsets() {
    // 2024-01-15T12:00:00.000Z and 2024-07-15T12:00:00.000Z
    let out = run(NY_DST_OFFSETS_SRC);
    // Wave A W78 / ADR-422 / A-03: IANA NY DST offsets product exact.
    assert_eq!(out, "-300\n-240", "NY DST offsets must exact: {out}");
}

#[test]
fn m2_s5_02_ny_dst_offsets_native_mir() {
    let out = lines_joined(&run_native_mir_src("ny_dst", NY_DST_OFFSETS_SRC));
    assert_eq!(
        out, "-300\n-240",
        "native mir NY DST offsets must exact: {out}"
    );
}

#[test]
fn m2_s5_02_ny_dst_offsets_wasm() {
    let out = lines_joined(&run_wasm_src("ny_dst", NY_DST_OFFSETS_SRC));
    assert_eq!(out, "-300\n-240", "wasm NY DST offsets must exact: {out}");
}

#[test]
fn m2_s5_02_format_with_timezone_ny_wasm() {
    let out = lines_joined(&run_wasm_src("ny_fmt", NY_WINTER_FORMAT_SRC));
    assert_eq!(
        out, "2024-01-15T07:00:00.000-05:00",
        "wasm NY winter formatWithTimezone must exact: {out}"
    );
}

#[test]
fn m2_s5_02_ny_dst_offset_native_llvm() {
    let (code, stdout) = run_native_strict(NY_WINTER_OFFSET_SRC);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(out, "-300", "native llvm NY winter offset must exact: {out}");
}

#[test]
fn m2_s5_02_format_with_timezone_ny() {
    let out = run(
        r#"
func main() {
  try {
    let winter = __syx_datetime_parse_iso8601("2024-01-15T12:00:00.000Z")
    let summer = __syx_datetime_parse_iso8601("2024-07-15T12:00:00.000Z")
    let wMs = __syx_keypath_get(winter, "DateTime", "ms", false)
    let sMs = __syx_keypath_get(summer, "DateTime", "ms", false)
    print(__syx_datetime_format_with_timezone(wMs, "iso", "America/New_York"))
    print(__syx_datetime_format_with_timezone(sMs, "iso", "America/New_York"))
  } catch (err) {
    print("UNEXPECTED")
    print(err)
  }
}
"#,
    );
    // Wave A W78 / ADR-422 / A-03: IANA formatWithTimezone NY product exact.
    assert_eq!(
        out,
        "2024-01-15T07:00:00.000-05:00\n2024-07-15T08:00:00.000-04:00",
        "formatWithTimezone NY must exact: {out}"
    );
}

#[test]
fn m2_s5_02_format_with_timezone_ny_native_llvm() {
    let (code, stdout) = run_native_strict(NY_WINTER_FORMAT_SRC);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(
        out, "2024-01-15T07:00:00.000-05:00",
        "native llvm NY winter format must exact: {out}"
    );
}

#[test]
fn m2_s5_02_leap_day_still_green() {
    let out = run(
        r#"
func main() {
  let o = __syx_datetime_parse_iso8601("2020-02-29")
  let ms = __syx_keypath_get(o, "DateTime", "ms", false)
  print(__syx_datetime_format(ms, "iso"))
}
"#,
    );
    // Wave A W74 / ADR-418 / A-03: IANA leap-day ISO product exact.
    assert_eq!(
        out, "2020-02-29T00:00:00.000Z",
        "leap day ISO must exact: {out}"
    );
}

#[test]
fn m2_s5_02_unknown_zone_u001() {
    let out = run(
        r#"
func main() {
  try {
    print(__syx_datetime_timezone_offset(0, "Not/AZone"))
    print("UNEXPECTED_OK")
  } catch (err) {
    print("caught")
    print(err.message)
  }
}
"#,
    );
    // Wave A W74 / ADR-418 / A-03: IANA unknown zone U001 product exact.
    assert_eq!(
        out,
        "caught\nDateTime: unknown IANA timezone \"Not/AZone\" (no tzdb match)",
        "unknown zone U001 must exact: {out}"
    );
}

#[test]
fn m2_s5_02_docs_claim_iana() {
    let usage = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        usage.contains("ADR-233")
            && usage.contains("timezoneOffset")
            && usage.contains("America/New_York")
            && (usage.contains("leap second") || usage.contains("EC-M2-4-Z")),
        "UsageGuide must document IANA MVP + fences"
    );
}

#[test]
fn m2_s5_02_host_inventory() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in ["DateTimeFormatWithTimezone", "DateTimeTimezoneOffset"] {
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
            block.contains("status = \"partial\"")
                && (block.contains("ADR-233")
                    || block.contains("ADR-566")
                    || block.contains("ADR-567")
                    || block.contains("ADR-603")),
            "{name} must be partial + ADR-233/566/567/603: {block}"
        );
    }
}

#[test]
fn m2_s5_02_format_with_timezone_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "DateTimeFormatWithTimezone");
    assert!(
        block.contains("llvm = \"real\""),
        "DateTimeFormatWithTimezone must declare llvm=real (ADR-567/ADR-603): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "DateTimeFormatWithTimezone must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "DateTimeFormatWithTimezone must declare wasm=real (ADR-603): {block}"
    );
    assert!(
        block.contains("ADR-603") || block.contains("ADR-567"),
        "notes must cite ADR-603/ADR-567: {block}"
    );
}

#[test]
fn m2_s5_02_timezone_offset_host_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "DateTimeTimezoneOffset");
    assert!(
        block.contains("llvm = \"real\""),
        "DateTimeTimezoneOffset must declare llvm=real (ADR-566/ADR-603): {block}"
    );
    assert!(
        block.contains("status = \"partial\""),
        "DateTimeTimezoneOffset must keep status=partial: {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "DateTimeTimezoneOffset must declare wasm=real (ADR-603): {block}"
    );
    assert!(
        block.contains("ADR-603") || block.contains("ADR-566"),
        "notes must cite ADR-603/ADR-566: {block}"
    );
}

#[test]
fn m2_s5_02_product_timezone() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "datetime.timezone");
    assert!(
        block.contains("status = \"implemented\""),
        "datetime.timezone must be implemented: {block}"
    );
    assert!(
        block.contains("m2_s5_02_iana_tzdb_dst"),
        "tests must include nail: {block}"
    );
    assert!(
        block.contains("ADR-233")
            || block.contains("M2-S5-02")
            || block.contains("ADR-565"),
        "notes must cite ADR-233 or ADR-565: {block}"
    );
    assert!(
        block.contains("leap second")
            || block.contains("EC-M2-4-Z")
            || block.contains("full tz")
            || block.contains("llvm DateTime"),
        "notes must keep honesty fence: {block}"
    );
}

#[test]
fn m2_s5_02_timezone_product_backends_real() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "datetime.timezone");
    assert!(
        block.contains("native = \"real\""),
        "datetime.timezone must declare native=real (ADR-565): {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "datetime.timezone must declare wasm=real (ADR-565): {block}"
    );
    assert!(
        block.contains("status = \"implemented\""),
        "datetime.timezone must keep status=implemented: {block}"
    );
    assert!(
        block.contains("ADR-565"),
        "notes must cite ADR-565: {block}"
    );
    assert!(
        block.contains("m2_s5_02_ny_dst_offsets_native_mir"),
        "tests must include native mir nail: {block}"
    );
}

#[test]
fn m2_s5_02_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m2_s5_02_iana_tzdb_dst_tier"),
        "inventory must contain m2_s5_02_iana_tzdb_dst_tier"
    );
    assert!(
        inv.contains("signed-ADR-233") || inv.contains("ADR-233"),
        "inventory must note ADR-233"
    );
}

#[test]
fn m2_s5_02_ci_wires() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test stdlib"),
        "ci.yml must wire m2_s5_02_iana_tzdb_dst"
    );
}

#[test]
fn m2_s5_02_surface_signature() {
    let src = include_str!("../../../../packages/datetime/mod.sy");
    assert!(
        src.contains("timezoneOffset(ms: Int, timezone: String)")
            && src.contains("formatWithTimezone")
            && src.contains("throws"),
        "datetime.sy must expose ms+timezone signatures: {src}"
    );
}
