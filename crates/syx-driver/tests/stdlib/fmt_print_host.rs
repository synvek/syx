//! FmtPrint + FmtPrintLine HostFn llvm=real (native AOT nails).
//! FmtPrint + FmtPrintLine HostFn wasm=real (text 收官 + Fmt/print wasm8).

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use syx_driver::{
    build_native_executable, run_wasm_source_printed, DriverOptions,
};

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
        "syx_w267_fmt_{}_{}",
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
        "syx_w241_fmt_print_llvm_{}",
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

const FMT_PRINT_HI: &str = r#"
func main() {
  __syx_fmt_print("hi")
}
"#;

const FMT_PRINT_LINE_HI: &str = r#"
func main() {
  __syx_fmt_print_line("hi")
}
"#;

#[test]
fn m2_fmt_print_native_llvm() {
    let (code, stdout) = run_native_strict(FMT_PRINT_HI);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    // FmtPrint has no trailing newline; trim empty lines still yields "hi".
    assert_eq!(out, "hi", "native llvm fmt_print(hi) must exact: {out:?} stdout={stdout:?}");
}

#[test]
fn m2_fmt_print_line_native_llvm() {
    let (code, stdout) = run_native_strict(FMT_PRINT_LINE_HI);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(out, "hi", "native llvm fmt_print_line(hi) must exact: {out}");
}

#[test]
fn m2_fmt_print_wasm() {
    let out = lines_joined(&run_wasm_src("fmt_print_wasm", FMT_PRINT_HI));
    assert_eq!(out, "hi", "wasm fmt_print(hi) must exact: {out}");
}

#[test]
fn m2_fmt_print_line_wasm() {
    let out = lines_joined(&run_wasm_src("fmt_print_line_wasm", FMT_PRINT_LINE_HI));
    assert_eq!(out, "hi", "wasm fmt_print_line(hi) must exact: {out}");
}

#[test]
fn m2_fmt_print_hosts_llvm_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in ["FmtPrint", "FmtPrintLine"] {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must declare llvm=real (ADR-585): {block}"
        );
        assert!(
            block.contains("status = \"partial\""),
            "{name} must keep status=partial: {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-611): {block}"
        );
        assert!(
            block.contains("ADR-611") && block.contains("ADR-585"),
            "{name} notes must cite ADR-611 + ADR-585: {block}"
        );
    }
}
