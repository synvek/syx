//! ADR-214 / M2 W70: EC-M2-4 Path/File native/WASM GA third deepening (PathNormalize llvm=real).
//! Wave A W213 / ADR-557: io.path product PathJoin/PathExists mir-run native + wasm nails.
//! Wave A W214 / ADR-558: io.file.read_all_bytes product mir-run native + wasm nails.
//! Wave A W215 / ADR-559: io.path status=implemented via PathNormalize/PathExtension mir/wasm nails.
//! Wave A W268 / ADR-612: Path×4 + FileOpen/Close/Read/Write HostFn wasm=real.
//! Wave A W269 / ADR-613: FileSeek/FileMmapRead HostFn wasm=real (File 残余收官).
// was: ec_m2_4_path_file_native_wasm_ga3

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use syx_driver::{
    build_native_executable, compile_and_run_embedded, run_native_mir_file_printed,
    run_wasm_source_printed, DriverOptions,
};
use syx_vm::reset_file_handles_for_tests;

const OTHER_PATH_FILE_HOSTS: &[&str] = &[
    // PathExists / FileOpen / FileClose / FileRead / FileWrite / FileSeek / FileMmapRead
    // promoted llvm=real (W147 / W192 / W193 / W194). Path/File HostFn llvm ladder cleared.
];

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

fn run_native_strict(src: &str) -> (i32, String) {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_m2_w70_path_ga3_{}",
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

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn write_src_tmp(label: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "syx_w213_{}_{}",
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

const PATH_JOIN_SRC: &str = r#"func main() {
    print(__syx_path_join("a", "b/c"))
}"#;

const PATH_NORMALIZE_SRC: &str = r#"func main() {
    print(__syx_path_normalize("a/b/../c"))
}"#;

const PATH_EXTENSION_SRC: &str = r#"func main() {
    print(__syx_path_extension("a/b.txt"))
}"#;

fn path_exists_true_src() -> (String, std::path::PathBuf) {
    let dir = std::env::temp_dir().join(format!(
        "syx_w213_exists_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let file = dir.join("probe.txt");
    fs::write(&file, b"x").unwrap();
    let path_lit = syx_path_literal(&file);
    let src = format!(
        r#"func main() {{
    print(__syx_path_exists("{path_lit}"))
}}"#
    );
    (src, dir)
}

fn path_exists_false_src() -> (String, std::path::PathBuf) {
    let dir = std::env::temp_dir().join(format!(
        "syx_w213_missing_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let missing = dir.join("no_such_file.sy");
    let path_lit = syx_path_literal(&missing);
    let src = format!(
        r#"func main() {{
    print(__syx_path_exists("{path_lit}"))
}}"#
    );
    (src, dir)
}

fn assert_path_join_line(line: &str) {
    #[cfg(unix)]
    assert_eq!(line, "a/b/c", "Unix PathJoin: {line}");
    #[cfg(windows)]
    assert_eq!(line, r"a\b\c", "Windows PathJoin: {line}");
    #[cfg(not(any(unix, windows)))]
    panic!("PathJoin W213 nail requires unix or windows cfg");
}

fn assert_path_normalize_line(line: &str) {
    #[cfg(unix)]
    assert_eq!(line, "a/c", "Unix PathNormalize: {line}");
    #[cfg(windows)]
    assert_eq!(line, r"a\c", "Windows PathNormalize: {line}");
    #[cfg(not(any(unix, windows)))]
    panic!("PathNormalize W215 nail requires unix or windows cfg");
}

fn assert_path_extension_line(line: &str) {
    assert_eq!(line, "txt", "PathExtension must be txt: {line}");
}

fn read_all_bytes_src() -> (String, std::path::PathBuf) {
    let dir = std::env::temp_dir().join(format!(
        "syx_w214_rab_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let file = dir.join("mmap_probe.txt");
    fs::write(&file, b"ABCD").unwrap();
    let path_lit = syx_path_literal(&file);
    let src = format!(
        r#"func main() {{
    let h = __syx_file_open("{path_lit}", "r")
    let b = __syx_file_read_all_bytes(h)
    print(b.count)
    print(b[0])
}}"#
    );
    (src, dir)
}

fn assert_read_all_bytes_out(out: &str) {
    assert_eq!(
        out, "4\n65",
        "readAllBytes must yield length 4 and first byte 65: {out}"
    );
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga3_path_normalize_columns() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "PathNormalize");
    assert!(
        block.contains("vm = \"real\""),
        "PathNormalize must keep vm=real: {block}"
    );
    assert!(
        block.contains("llvm = \"real\""),
        "PathNormalize must declare llvm=real (ADR-214): {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "PathNormalize must declare wasm=real (ADR-612): {block}"
    );
    assert!(
        block.contains("ADR-612") && (block.contains("ADR-214") || block.contains("llvm")),
        "PathNormalize notes must cite ADR-612 + ADR-214: {block}"
    );
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga3_prior_ga_hosts_remain_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in ["PathJoin", "PathExtension"] {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must keep llvm=real: {block}"
        );
    }
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga3_other_hosts_remain_partial() {
    // Wave A W194 / ADR-538: Path/File HostFn llvm ladder cleared; fence must be empty.
    assert!(
        OTHER_PATH_FILE_HOSTS.is_empty(),
        "OTHER_PATH_FILE_HOSTS must be empty after FileSeek/FileMmapRead llvm=real: {OTHER_PATH_FILE_HOSTS:?}"
    );
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga3_product_columns() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "io.path");
    assert!(
        block.contains("ADR-214") || block.contains("llvm GA") || block.contains("ADR-557") || block.contains("ADR-559"),
        "io.path notes must cite ADR-214 or ADR-559: {block}"
    );
    assert!(
        block.contains("ec_m2_4_path_file_native_wasm_ga3"),
        "io.path tests must include ec_m2_4_path_file_native_wasm_ga3: {block}"
    );
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga3_native_path_normalize_exec() {
    let src = r#"func main() {
    let p = __syx_path_normalize("a/b/../c")
    print(p)
    return 0
}"#;
    let vm_out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("vm run")
        .printed
        .join("\n");
    let (code, native_out) = run_native_strict(src);
    assert_eq!(code, 0, "native exit nonzero; stdout={native_out}");
    let vm_line = vm_out.lines().next().unwrap_or("").trim();
    let native_line = native_out.lines().next().unwrap_or("").trim();
    assert_eq!(
        native_line, vm_line,
        "native PathNormalize must match VM; vm={vm_line:?} native={native_line:?}"
    );
    #[cfg(windows)]
    assert_eq!(
        native_line, r"a\c",
        "Windows normalize collapse: {native_line}"
    );
    #[cfg(unix)]
    assert_eq!(native_line, "a/c", "Unix normalize collapse: {native_line}");
    #[cfg(not(any(unix, windows)))]
    panic!("PathNormalize GA3 nail requires unix or windows cfg");
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga_native_path_join_exec() {
    // Wave A W146 / ADR-490 / A-03
    let src = r#"func main() {
    print(__syx_path_join("a", "b/c"))
    return 0
}"#;
    let vm_out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("vm run")
        .printed
        .join("\n");
    let (code, native_out) = run_native_strict(src);
    assert_eq!(code, 0, "native exit nonzero; stdout={native_out}");
    let vm_line = vm_out.lines().next().unwrap_or("").trim();
    let native_line = native_out.lines().next().unwrap_or("").trim();
    assert_eq!(
        native_line, vm_line,
        "native PathJoin must match VM; vm={vm_line:?} native={native_line:?}"
    );
    #[cfg(unix)]
    assert_eq!(native_line, "a/b/c", "Unix PathJoin: {native_line}");
    #[cfg(windows)]
    assert_eq!(
        native_line, r"a\b\c",
        "Windows PathJoin: {native_line}"
    );
    #[cfg(not(any(unix, windows)))]
    panic!("PathJoin W146 nail requires unix or windows cfg");
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga_native_path_extension_exec() {
    // Wave A W146 / ADR-490 / A-03
    let src = r#"func main() {
    print(__syx_path_extension("a/b.txt"))
    return 0
}"#;
    let vm_out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("vm run")
        .printed
        .join("\n");
    let (code, native_out) = run_native_strict(src);
    assert_eq!(code, 0, "native exit nonzero; stdout={native_out}");
    let vm_line = vm_out.lines().next().unwrap_or("").trim();
    let native_line = native_out.lines().next().unwrap_or("").trim();
    assert_eq!(
        native_line, vm_line,
        "native PathExtension must match VM; vm={vm_line:?} native={native_line:?}"
    );
    assert_eq!(native_line, "txt", "PathExtension must be txt: {native_line}");
}

fn syx_path_literal(path: &std::path::Path) -> String {
    path.to_string_lossy()
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
}

fn run_path_exists_exec(src: &str) -> (String, String) {
    let vm_out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("vm run")
        .printed
        .join("\n");
    let (code, native_out) = run_native_strict(src);
    assert_eq!(code, 0, "native exit nonzero; stdout={native_out}");
    let vm_line = vm_out.lines().next().unwrap_or("").trim().to_string();
    let native_line = native_out.lines().next().unwrap_or("").trim().to_string();
    (vm_line, native_line)
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga4_path_exists_columns() {
    // Wave A W147 / ADR-491 — PathExists llvm=real promotion
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    let block = host_block(toml, "PathExists");
    assert!(
        block.contains("vm = \"real\""),
        "PathExists must keep vm=real: {block}"
    );
    assert!(
        block.contains("llvm = \"real\""),
        "PathExists must declare llvm=real (ADR-491): {block}"
    );
    assert!(
        block.contains("wasm = \"real\""),
        "PathExists must declare wasm=real (ADR-612): {block}"
    );
    assert!(
        block.contains("ADR-612") && (block.contains("ADR-491") || block.contains("llvm")),
        "PathExists notes must cite ADR-612 + ADR-491: {block}"
    );
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga4_prior_path_hosts_remain_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in ["PathJoin", "PathExtension", "PathNormalize", "PathExists"] {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must keep llvm=real: {block}"
        );
        assert!(
            !block.contains("llvm = \"partial\""),
            "{name} must not remain llvm=partial: {block}"
        );
    }
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga4_product_columns() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "io.path");
    assert!(
        block.contains("ADR-491") || block.contains("llvm GA") || block.contains("ADR-557") || block.contains("ADR-559"),
        "io.path notes must cite ADR-491 or ADR-559: {block}"
    );
    assert!(
        block.contains("ec_m2_4_path_file_native_wasm_ga4"),
        "io.path tests must include ec_m2_4_path_file_native_wasm_ga4: {block}"
    );
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga4_native_path_exists_true_exec() {
    // Wave A W147 / ADR-491 / A-03
    let dir = std::env::temp_dir().join(format!(
        "syx_w147_exists_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let file = dir.join("probe.txt");
    fs::write(&file, b"x").unwrap();
    let path_lit = syx_path_literal(&file);
    let src = format!(
        r#"func main() {{
    print(__syx_path_exists("{path_lit}"))
    return 0
}}"#
    );
    let (vm_line, native_line) = run_path_exists_exec(&src);
    assert_eq!(
        native_line, vm_line,
        "native PathExists must match VM; vm={vm_line:?} native={native_line:?}"
    );
    assert_eq!(native_line, "true", "PathExists true probe: {native_line}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga4_native_path_exists_false_exec() {
    // Wave A W147 / ADR-491 / A-03
    let dir = std::env::temp_dir().join(format!(
        "syx_w147_missing_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let missing = dir.join("no_such_file.sy");
    let path_lit = syx_path_literal(&missing);
    let src = format!(
        r#"func main() {{
    print(__syx_path_exists("{path_lit}"))
    return 0
}}"#
    );
    let (vm_line, native_line) = run_path_exists_exec(&src);
    assert_eq!(
        native_line, vm_line,
        "native PathExists must match VM; vm={vm_line:?} native={native_line:?}"
    );
    assert_eq!(native_line, "false", "PathExists false probe: {native_line}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga5_file_open_close_columns() {
    // Wave A W192 / ADR-536 — FileOpen + FileClose llvm=real promotion
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in ["FileOpen", "FileClose"] {
        let block = host_block(toml, name);
        assert!(
            block.contains("vm = \"real\""),
            "{name} must keep vm=real: {block}"
        );
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must declare llvm=real (ADR-536): {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-612): {block}"
        );
        assert!(
            block.contains("ADR-612") && (block.contains("ADR-536") || block.contains("llvm")),
            "{name} notes must cite ADR-612 + ADR-536: {block}"
        );
    }
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga5_prior_path_file_hosts_remain_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in [
        "PathJoin",
        "PathExtension",
        "PathNormalize",
        "PathExists",
        "FileOpen",
        "FileClose",
    ] {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must keep llvm=real: {block}"
        );
        assert!(
            !block.contains("llvm = \"partial\""),
            "{name} must not remain llvm=partial: {block}"
        );
    }
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga5_product_columns() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "io.path");
    assert!(
        block.contains("ADR-536") || block.contains("llvm GA") || block.contains("ADR-557") || block.contains("ADR-559"),
        "io.path notes must cite ADR-536 or ADR-559: {block}"
    );
    assert!(
        block.contains("ec_m2_4_path_file_native_wasm_ga5"),
        "io.path tests must include ec_m2_4_path_file_native_wasm_ga5: {block}"
    );
}

fn run_file_open_close_exec(src: &str) -> (String, String) {
    reset_file_handles_for_tests();
    let vm_out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("vm run")
        .printed
        .join("\n");
    let (code, native_out) = run_native_strict(src);
    assert_eq!(code, 0, "native exit nonzero; stdout={native_out}");
    let vm_line = vm_out.lines().next().unwrap_or("").trim().to_string();
    let native_line = native_out.lines().next().unwrap_or("").trim().to_string();
    (vm_line, native_line)
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga_native_file_open_exec() {
    // Wave A W192 / ADR-536 / A-03
    let dir = std::env::temp_dir().join(format!(
        "syx_w192_fopen_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let file = dir.join("open_probe.bin");
    let path_lit = syx_path_literal(&file);
    let src = format!(
        r#"func main() {{
    let h = __syx_file_open("{path_lit}", "w")
    print(h)
    return 0
}}"#
    );
    let (vm_line, native_line) = run_file_open_close_exec(&src);
    assert_eq!(
        native_line, vm_line,
        "native FileOpen must match VM; vm={vm_line:?} native={native_line:?}"
    );
    assert_eq!(native_line, "1", "first file handle id must be 1: {native_line}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga_native_file_close_exec() {
    // Wave A W192 / ADR-536 / A-03
    let dir = std::env::temp_dir().join(format!(
        "syx_w192_fclose_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let file = dir.join("close_probe.bin");
    let path_lit = syx_path_literal(&file);
    let src = format!(
        r#"func main() {{
    let h = __syx_file_open("{path_lit}", "w")
    __syx_file_close(h)
    print("ok")
    return 0
}}"#
    );
    let (vm_line, native_line) = run_file_open_close_exec(&src);
    assert_eq!(
        native_line, vm_line,
        "native FileClose must match VM; vm={vm_line:?} native={native_line:?}"
    );
    assert_eq!(native_line, "ok", "close must succeed: {native_line}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga5_honesty_fences() {
    let stubs = include_str!("../../../../crates/syx-rt/src/llvm_stubs.rs");
    let idx = stubs
        .find("fn syx_rt_file_read_aot")
        .expect("syx_rt_file_read_aot must exist");
    let after = &stubs[idx..];
    let end = after.find("\n#[no_mangle]").unwrap_or(after.len().min(200));
    let body = &after[..end.min(200)];
    assert!(
        body.contains("HostFnId::FileRead") && body.contains("rt_call_host_native"),
        "syx_rt_file_read_aot must route HostFnId::FileRead (W452 wire): {body}"
    );
    assert!(
        !body.contains("rt_direct_exec"),
        "syx_rt_file_read_aot must leave rt_direct_exec after W452: {body}"
    );
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga5_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("ec_m2_4_path_file_native_wasm_ga5_tier"),
        "inventory must contain ec_m2_4_path_file_native_wasm_ga5_tier: {inv}"
    );
    assert!(
        inv.contains("ADR-536") || inv.contains("signed-ADR-536"),
        "inventory must note ADR-536: {inv}"
    );
}

fn run_file_read_write_exec(src: &str) -> (String, String) {
    let vm_out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("vm run")
        .printed
        .join("\n");
    let (code, native_out) = run_native_strict(src);
    assert_eq!(code, 0, "native exit nonzero; stdout={native_out}");
    let vm_line = vm_out.lines().next().unwrap_or("").trim().to_string();
    let native_line = native_out.lines().next().unwrap_or("").trim().to_string();
    (vm_line, native_line)
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga_native_file_write_exec() {
    // Wave A W193 / ADR-537 / A-03
    let dir = std::env::temp_dir().join(format!(
        "syx_w193_fwrite_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let file = dir.join("write_probe.txt");
    let path_lit = syx_path_literal(&file);
    let src = format!(
        r#"func main() {{
    __syx_file_write("{path_lit}", "w193")
    print("ok")
    return 0
}}"#
    );
    let (vm_line, native_line) = run_file_read_write_exec(&src);
    assert_eq!(
        native_line, vm_line,
        "native FileWrite must match VM; vm={vm_line:?} native={native_line:?}"
    );
    assert_eq!(native_line, "ok", "write must succeed: {native_line}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga_native_file_read_exec() {
    // Wave A W193 / ADR-537 / A-03
    let dir = std::env::temp_dir().join(format!(
        "syx_w193_fread_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let file = dir.join("read_probe.txt");
    let path_lit = syx_path_literal(&file);
    let src = format!(
        r#"func main() {{
    __syx_file_write("{path_lit}", "hello")
    print(__syx_file_read("{path_lit}"))
    return 0
}}"#
    );
    let (vm_line, native_line) = run_file_read_write_exec(&src);
    assert_eq!(
        native_line, vm_line,
        "native FileRead must match VM; vm={vm_line:?} native={native_line:?}"
    );
    assert_eq!(native_line, "hello", "read roundtrip must yield hello: {native_line}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga6_file_read_write_columns() {
    // Wave A W193 / ADR-537 — FileRead + FileWrite llvm=real promotion
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in ["FileRead", "FileWrite"] {
        let block = host_block(toml, name);
        assert!(
            block.contains("vm = \"real\""),
            "{name} must keep vm=real: {block}"
        );
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must declare llvm=real (ADR-537): {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-612): {block}"
        );
        assert!(
            block.contains("ADR-612") && (block.contains("ADR-537") || block.contains("llvm")),
            "{name} notes must cite ADR-612 + ADR-537: {block}"
        );
    }
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga6_prior_path_file_hosts_remain_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in [
        "PathJoin",
        "PathExtension",
        "PathNormalize",
        "PathExists",
        "FileOpen",
        "FileClose",
        "FileRead",
        "FileWrite",
    ] {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must keep llvm=real: {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-612): {block}"
        );
        assert!(
            block.contains("ADR-612"),
            "{name} notes must cite ADR-612: {block}"
        );
    }
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga6_product_columns() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "io.path");
    assert!(
        block.contains("ADR-537") || block.contains("llvm GA") || block.contains("ADR-557") || block.contains("ADR-559"),
        "io.path notes must cite ADR-537 or ADR-559: {block}"
    );
    assert!(
        block.contains("ec_m2_4_path_file_native_wasm_ga6"),
        "io.path tests must include ec_m2_4_path_file_native_wasm_ga6: {block}"
    );
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga6_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("ec_m2_4_path_file_native_wasm_ga6_tier"),
        "inventory must contain ec_m2_4_path_file_native_wasm_ga6_tier: {inv}"
    );
    assert!(
        inv.contains("ADR-537") || inv.contains("signed-ADR-537"),
        "inventory must note ADR-537: {inv}"
    );
}

fn run_file_handle_exec_stdout(src: &str) -> (String, String) {
    reset_file_handles_for_tests();
    let vm_out = compile_and_run_embedded(src, &DriverOptions::default())
        .expect("vm run")
        .printed
        .join("\n");
    let (code, native_out) = run_native_strict(src);
    assert_eq!(code, 0, "native exit nonzero; stdout={native_out}");
    let vm_text = vm_out.trim().to_string();
    let native_text = native_out.trim().to_string();
    (vm_text, native_text)
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga_native_file_seek_exec() {
    // Wave A W194 / ADR-538 / A-03
    let dir = std::env::temp_dir().join(format!(
        "syx_w194_fseek_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let file = dir.join("seek_probe.txt");
    fs::write(&file, b"ABCD").unwrap();
    let path_lit = syx_path_literal(&file);
    let src = format!(
        r#"func main() {{
    let h = __syx_file_open("{path_lit}", "r")
    __syx_file_seek(h, 0)
    print("ok")
    return 0
}}"#
    );
    let (vm_line, native_line) = run_file_handle_exec_stdout(&src);
    assert_eq!(
        native_line, vm_line,
        "native FileSeek must match VM; vm={vm_line:?} native={native_line:?}"
    );
    assert_eq!(native_line, "ok", "seek must succeed: {native_line}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga_file_seek_wasm() {
    // Wave A W269 / ADR-613 — FileSeek HostFn wasm=real
    let dir = std::env::temp_dir().join(format!(
        "syx_w269_fseek_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let file = dir.join("seek_probe.txt");
    fs::write(&file, b"ABCD").unwrap();
    let path_lit = syx_path_literal(&file);
    let src = format!(
        r#"func main() {{
    let h = __syx_file_open("{path_lit}", "r")
    __syx_file_seek(h, 0)
    print("ok")
    return 0
}}"#
    );
    let out = lines_joined(&run_wasm_src("file_seek_wasm", &src));
    assert_eq!(out, "ok", "wasm FileSeek must exact: {out}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga_native_file_mmap_read_exec() {
    // Wave A W194 / ADR-538 / A-03 + Wave A W195 / ADR-539 ArrayGet reclaim
    let dir = std::env::temp_dir().join(format!(
        "syx_w194_fmmap_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let file = dir.join("mmap_probe.txt");
    fs::write(&file, b"ABCD").unwrap();
    let path_lit = syx_path_literal(&file);
    let src = format!(
        r#"func main() {{
    let h = __syx_file_open("{path_lit}", "r")
    let b = __syx_file_read_all_bytes(h)
    print(b.count)
    print(b[0])
    return 0
}}"#
    );
    let (vm_text, native_text) = run_file_handle_exec_stdout(&src);
    assert_eq!(
        native_text, vm_text,
        "native FileMmapRead must match VM; vm={vm_text:?} native={native_text:?}"
    );
    assert_eq!(
        native_text, "4\n65",
        "mmap read must yield length 4 and first byte 65 (ArrayGet native ADR-539): {native_text}"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga7_file_seek_mmap_columns() {
    // Wave A W194 / ADR-538 llvm + Wave A W269 / ADR-613 wasm=real
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in ["FileSeek", "FileMmapRead"] {
        let block = host_block(toml, name);
        assert!(
            block.contains("vm = \"real\""),
            "{name} must keep vm=real: {block}"
        );
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must declare llvm=real (ADR-538): {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-613): {block}"
        );
        assert!(
            block.contains("ADR-613"),
            "{name} notes must cite ADR-613: {block}"
        );
        assert!(
            block.contains("ADR-538") || block.contains("llvm"),
            "{name} notes must cite ADR-538/llvm: {block}"
        );
    }
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga7_prior_path_file_hosts_remain_real() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in [
        "PathJoin",
        "PathExtension",
        "PathNormalize",
        "PathExists",
        "FileOpen",
        "FileClose",
        "FileRead",
        "FileWrite",
        "FileSeek",
        "FileMmapRead",
    ] {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must keep llvm=real: {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-612/613): {block}"
        );
    }
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga7_product_columns() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "io.path");
    assert!(
        block.contains("ADR-538") || block.contains("llvm GA") || block.contains("ADR-557") || block.contains("ADR-559"),
        "io.path notes must cite ADR-538 or ADR-559: {block}"
    );
    assert!(
        block.contains("ec_m2_4_path_file_native_wasm_ga7"),
        "io.path tests must include ec_m2_4_path_file_native_wasm_ga7: {block}"
    );
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga7_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("ec_m2_4_path_file_native_wasm_ga7_tier"),
        "inventory must contain ec_m2_4_path_file_native_wasm_ga7_tier: {inv}"
    );
    assert!(
        inv.contains("ADR-538") || inv.contains("signed-ADR-538"),
        "inventory must note ADR-538: {inv}"
    );
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga4_honesty_fences() {
    let stubs = include_str!("../../../../crates/syx-rt/src/llvm_stubs.rs");
    let idx = stubs
        .find("fn syx_rt_file_read_aot")
        .expect("syx_rt_file_read_aot must exist");
    let after = &stubs[idx..];
    let end = after.find("\n#[no_mangle]").unwrap_or(after.len().min(200));
    let body = &after[..end.min(200)];
    assert!(
        body.contains("HostFnId::FileRead") && body.contains("rt_call_host_native"),
        "syx_rt_file_read_aot must route HostFnId::FileRead (W452 wire): {body}"
    );
    assert!(
        !body.contains("rt_direct_exec"),
        "syx_rt_file_read_aot must leave rt_direct_exec after W452: {body}"
    );
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga4_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("ec_m2_4_path_file_native_wasm_ga4_tier"),
        "inventory must contain ec_m2_4_path_file_native_wasm_ga4_tier: {inv}"
    );
    assert!(
        inv.contains("ADR-491") || inv.contains("signed-ADR-491"),
        "inventory must note ADR-491: {inv}"
    );
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga3_honesty_fences() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in [
        "FileDelete",
        "FileExists",
        "FileReadChunk",
        "FileWriteChunk",
    ] {
        assert!(
            !toml.contains(&format!("name = \"{name}\"")),
            "{name} must remain uncovered (fail-closed default); found override"
        );
    }

    let stubs = include_str!("../../../../crates/syx-rt/src/llvm_stubs.rs");
    let idx = stubs
        .find("fn syx_rt_file_read_aot")
        .expect("syx_rt_file_read_aot must exist");
    let after = &stubs[idx..];
    let end = after.find("\n#[no_mangle]").unwrap_or(after.len().min(200));
    let body = &after[..end.min(200)];
    assert!(
        body.contains("HostFnId::FileRead") && body.contains("rt_call_host_native"),
        "syx_rt_file_read_aot must route HostFnId::FileRead (W452 wire): {body}"
    );
    assert!(
        !body.contains("rt_direct_exec"),
        "syx_rt_file_read_aot must leave rt_direct_exec after W452: {body}"
    );
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga3_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("ec_m2_4_path_file_native_wasm_ga3_tier"),
        "inventory must contain ec_m2_4_path_file_native_wasm_ga3_tier: {inv}"
    );
    assert!(
        inv.contains("ADR-214") || inv.contains("signed-ADR-214"),
        "inventory must note ADR-214: {inv}"
    );
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga3_ci_wires() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test evergreen"),
        "ci.yml must wire ec_m2_4_path_file_native_wasm_ga3"
    );
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga_path_join_native_mir() {
    let out = lines_joined(&run_native_mir_src("join_mir", PATH_JOIN_SRC));
    assert_path_join_line(&out);
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga_path_join_wasm() {
    let out = lines_joined(&run_wasm_src("join_wasm", PATH_JOIN_SRC));
    assert_path_join_line(&out);
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga_path_exists_true_native_mir() {
    let (src, dir) = path_exists_true_src();
    let out = lines_joined(&run_native_mir_src("exists_true_mir", &src));
    assert_eq!(out, "true", "PathExists true mir must exact: {out}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga_path_exists_true_wasm() {
    let (src, dir) = path_exists_true_src();
    let out = lines_joined(&run_wasm_src("exists_true_wasm", &src));
    assert_eq!(out, "true", "PathExists true wasm must exact: {out}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga_path_exists_false_native_mir() {
    let (src, dir) = path_exists_false_src();
    let out = lines_joined(&run_native_mir_src("exists_false_mir", &src));
    assert_eq!(out, "false", "PathExists false mir must exact: {out}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga_path_exists_false_wasm() {
    let (src, dir) = path_exists_false_src();
    let out = lines_joined(&run_wasm_src("exists_false_wasm", &src));
    assert_eq!(out, "false", "PathExists false wasm must exact: {out}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga_product_path_backends_real() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "io.path");
    assert!(
        block.contains("native = \"real\"") && block.contains("wasm = \"real\""),
        "io.path native/wasm must be real: {block}"
    );
    assert!(
        block.contains("mir = \"real\""),
        "io.path mir must stay real: {block}"
    );
    assert!(
        block.contains("status = \"implemented\""),
        "io.path status must be implemented: {block}"
    );
    assert!(
        block.contains("ADR-559"),
        "notes must cite ADR-559: {block}"
    );
    assert!(
        block.contains("ec_m2_4_path_file_native_wasm_ga_path_normalize_native_mir"),
        "tests must list normalize native mir nail: {block}"
    );
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga_path_normalize_native_mir() {
    let out = lines_joined(&run_native_mir_src("norm_mir", PATH_NORMALIZE_SRC));
    assert_path_normalize_line(&out);
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga_path_normalize_wasm() {
    let out = lines_joined(&run_wasm_src("norm_wasm", PATH_NORMALIZE_SRC));
    assert_path_normalize_line(&out);
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga_path_extension_native_mir() {
    let out = lines_joined(&run_native_mir_src("ext_mir", PATH_EXTENSION_SRC));
    assert_path_extension_line(&out);
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga_path_extension_wasm() {
    let out = lines_joined(&run_wasm_src("ext_wasm", PATH_EXTENSION_SRC));
    assert_path_extension_line(&out);
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga_read_all_bytes_native_mir() {
    let (src, dir) = read_all_bytes_src();
    let out = lines_joined(&run_native_mir_src("read_all_bytes_mir", &src));
    assert_read_all_bytes_out(&out);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga_read_all_bytes_wasm() {
    let (src, dir) = read_all_bytes_src();
    let out = lines_joined(&run_wasm_src("read_all_bytes_wasm", &src));
    assert_read_all_bytes_out(&out);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn ec_m2_4_path_file_native_wasm_ga_product_read_all_bytes_backends_real() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "io.file.read_all_bytes");
    assert!(
        block.contains("native = \"real\"") && block.contains("wasm = \"real\""),
        "io.file.read_all_bytes native/wasm must be real: {block}"
    );
    assert!(
        block.contains("mir = \"real\""),
        "io.file.read_all_bytes mir must stay real: {block}"
    );
    assert!(
        block.contains("status = \"implemented\""),
        "io.file.read_all_bytes status must stay implemented: {block}"
    );
    assert!(
        block.contains("ADR-558"),
        "notes must cite ADR-558: {block}"
    );
    assert!(
        block.contains("ec_m2_4_path_file_native_wasm_ga_read_all_bytes_native_mir"),
        "tests must list native mir nail: {block}"
    );
}

#[test]
fn ec_m2_4_path_file_hosts_wasm_real_w268() {
    // Wave A W268 / ADR-612 — Path×4 + FileOpen/Close/Read/Write wasm=real
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in [
        "PathJoin",
        "PathNormalize",
        "PathExists",
        "PathExtension",
        "FileOpen",
        "FileClose",
        "FileRead",
        "FileWrite",
    ] {
        let block = host_block(toml, name);
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-612): {block}"
        );
        assert!(
            block.contains("status = \"partial\""),
            "{name} must keep status=partial: {block}"
        );
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must keep llvm=real: {block}"
        );
        assert!(
            block.contains("ADR-612"),
            "{name} notes must cite ADR-612: {block}"
        );
    }
}

#[test]
fn ec_m2_4_path_file_hosts_wasm_real_w269() {
    // Wave A W269 / ADR-613 — File residual wasm8
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in [
        "FileSeek",
        "FileMmapRead",
        "FileWithOpen",
        "FileWithOpenFail",
        "FileCloseQuiet",
        "FileWatcherOpen",
        "FileWatcherPoll",
        "FileWatcherClose",
    ] {
        let block = host_block(toml, name);
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-613): {block}"
        );
        assert!(
            block.contains("status = \"partial\""),
            "{name} must keep status=partial: {block}"
        );
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must keep llvm=real: {block}"
        );
        assert!(
            block.contains("ADR-613"),
            "{name} notes must cite ADR-613: {block}"
        );
    }
}
