//! Language edge: typed catch / continue-after-catch (native + wasm).

use std::fs;
use std::process::Command;

use syx_driver::{
    build_native_executable, run_native_mir_file_printed, run_wasm_source_printed, DriverOptions,
};

const TYPED_CATCH: &str = include_str!("../fixtures/errors/typed_catch.sy");

const TYPED_CATCH_EXPECTED: &str = "boom\n42";

const CATCH_CONTINUES: &str = r#"
data class AppError(code: Int, message: String)

func fail() throws {
    throw AppError(code: 7, message: "x")
}

func main() {
    try {
        fail()
    } catch (err) {
        print(err.code)
    }
    print("after")
}
"#;

const CATCH_CONTINUES_EXPECTED: &str = "7\nafter";

fn file_open_missing_src() -> String {
    let fh = include_str!("../../../../packages/io/file_handle.sy");
    format!(
        r#"{fh}
func main() {{
  try {{
    let h = FileHandle.open(path: "/tmp/syx_w281_a04_no_such_file.bin", mode: "r")
    print("UNEXPECTED_OK")
    print(h)
  }} catch (err) {{
    print("caught")
    print(err.code)
  }}
  return 0
}}
"#
    )
}

const FILE_OPEN_MISSING_EXPECTED: &str = "caught\n1";

fn lines_joined(lines: &[String]) -> String {
    lines
        .iter()
        .map(|l| l.trim_end_matches('\r').trim_end_matches('\n'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn write_src(label: &str, src: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "syx_w281_a04_{}_{}",
        label,
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let path = dir.join("main.sy");
    fs::write(&path, src).expect("write main");
    path
}

fn run_native_mir(label: &str, src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = write_src(label, src);
    let (code, lines) =
        run_native_mir_file_printed(&path, &DriverOptions::default()).expect("native mir");
    assert_eq!(code, 0, "native mir exit {code}: {lines:?}");
    lines_joined(&lines)
}

fn run_native_strict(label: &str, src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_w281_a04_strict_{}_{}",
        label,
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    fs::write(dir.join("main.sy"), src).expect("write main");
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .expect("write manifest");
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native exe");
    assert_eq!(
        output.status.code().unwrap_or(1),
        0,
        "native strict exit: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim_end().to_string()
}

fn run_wasm(label: &str, src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let path = write_src(label, src);
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (code, lines) =
        run_wasm_source_printed(&path, "main", &opts).expect("wasm");
    assert_eq!(code, 0, "wasm exit {code}: {lines:?}");
    lines_joined(&lines)
}

#[test]
fn edge_typed_catch_message_code_native_strict() {
    let out = run_native_strict("typed_catch", TYPED_CATCH);
    assert_eq!(
        out, TYPED_CATCH_EXPECTED,
        "native strict typed catch must exact: {out}"
    );
}

#[test]
fn edge_typed_catch_message_code_wasm() {
    let out = run_wasm("typed_catch", TYPED_CATCH);
    assert_eq!(
        out, TYPED_CATCH_EXPECTED,
        "wasm typed catch must exact: {out}"
    );
}

#[test]
fn edge_catch_continues_native() {
    let out = run_native_mir("catch_continues", CATCH_CONTINUES);
    assert_eq!(
        out, CATCH_CONTINUES_EXPECTED,
        "native catch-continues must exact: {out}"
    );
}

#[test]
fn edge_catch_continues_wasm() {
    let out = run_wasm("catch_continues", CATCH_CONTINUES);
    assert_eq!(
        out, CATCH_CONTINUES_EXPECTED,
        "wasm catch-continues must exact: {out}"
    );
}

#[test]
fn edge_file_open_missing_catch_code_native() {
    let out = run_native_strict("file_open_missing", &file_open_missing_src());
    assert_eq!(
        out, FILE_OPEN_MISSING_EXPECTED,
        "native File open-missing catch code must exact: {out}"
    );
}

#[test]
fn edge_file_open_missing_catch_code_wasm() {
    let out = run_wasm("file_open_missing", &file_open_missing_src());
    assert_eq!(
        out, FILE_OPEN_MISSING_EXPECTED,
        "wasm File open-missing catch code must exact: {out}"
    );
}

#[test]
fn edge_spec_doc_cites_adr625() {
    let spec = include_str!("../../../../docs/gates/gate-a04-edge-semantics.md");
    assert!(
        spec.contains("ADR-625") && spec.contains("exception"),
        "a04-edge-semantics.md must cite ADR-625 exception: {spec}"
    );
    assert!(!spec.contains("| actor ordering | deferred") && !spec.contains("actor ordering | deferred"),
        "spec must not leave actor deferred"
    );
    assert!(
        spec.contains("ADR-632") && spec.contains("Q-L4 floor"),
        "spec must cite ADR-632 Q-L4 floor signed"
    );
    assert!(
        spec.contains("Not** Wave A")
            || spec.contains("**Not** Wave A")
            || spec.contains("Not Wave A"),
        "spec must remain Not Wave A exit"
    );
}

