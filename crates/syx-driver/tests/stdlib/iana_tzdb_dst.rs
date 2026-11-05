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

