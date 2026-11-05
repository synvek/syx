//! 04 retire M0 DateTime Unsupported/preview.
//! DateTimeNow HostFn wasm=real (epoch-scale wasm nail).
// was: m2_s5_04_retire_unsupported_preview

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use syx_driver::{
    build_native_executable, compile_and_run_embedded, run_wasm_source_printed, DriverOptions,
};

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

fn datetime_src(body: &str) -> String {
    let dt = include_str!("../../../../packages/datetime/mod.sy");
    format!("{dt}\n{body}")
}

fn write_src_tmp(label: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "syx_w260_{}_{}",
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

#[test]
fn m2_s5_04_now_returns_epoch_scale_ms() {
    let out = run(&datetime_src(
        r#"func main() {
    let o = DateTime.now()
    print(__syx_keypath_get(o, "DateTime", "ms", false))
}"#,
    ));
    let ms: i64 = out.trim().parse().expect("ms int");
    assert!(
        ms > 1_000_000_000_000,
        "DateTime.now ms must be UNIX epoch-scale; got {ms}"
    );
}

/// Wave A W255 / ADR-599 / A-05 — DateTimeNow llvm cell (native epoch-scale ms).
#[cfg(unix)]
#[test]
fn m2_s5_04_datetime_now_native_llvm() {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let src = datetime_src(
        r#"func main() {
  let o = __syx_datetime_now()
  print(__syx_keypath_get(o, "DateTime", "ms", false))
  return 0
}
"#,
    );
    let dir = std::env::temp_dir().join(format!(
        "syx_datetime_now_native_llvm_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    fs::write(dir.join("main.sy"), &src).expect("write main");
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .expect("write toml");
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let ms: i64 = stdout.trim().parse().expect("native ms int");
    let _ = fs::remove_dir_all(&dir);
    assert!(
        ms > 1_000_000_000_000,
        "DateTimeNow native llvm ms must be epoch-scale; got {ms}"
    );
}

#[test]
fn m2_s5_04_datetime_now_wasm() {
    let src = datetime_src(
        r#"func main() {
  let o = __syx_datetime_now()
  print(__syx_keypath_get(o, "DateTime", "ms", false))
}
"#,
    );
    let out = lines_joined(&run_wasm_src("datetime_now", &src));
    let ms: i64 = out.trim().parse().expect("wasm ms int");
    assert!(
        ms > 1_000_000_000_000,
        "DateTimeNow wasm ms must be epoch-scale; got {ms}"
    );
}

#[test]
fn m2_s5_04_unknown_iana_still_fail_closed() {
    // ADR-233: known IANA zones are implemented; unknown zone still U001.
    let tz = run_err(
        r#"func main() { print(__syx_datetime_timezone_offset(0, "Not/AZone")) }"#,
    );
    // Wave A W98 / ADR-442 / A-03: unknown IANA fail-closed Display product exact.
    assert_eq!(
        tz, "[U001] DateTime: unknown IANA timezone \"Not/AZone\" (no tzdb match)",
        "unknown tz must exact: {tz}"
    );
}

#[test]
fn m2_s5_04_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m2_s5_04_retire_unsupported_preview_tier"),
        "inventory must contain m2_s5_04_retire_unsupported_preview_tier"
    );
    assert!(
        inv.contains("ADR-222") || inv.contains("signed-ADR-222"),
        "inventory must note ADR-222"
    );
}

