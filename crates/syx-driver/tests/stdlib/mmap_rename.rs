//! 04 mmap rename closeout nail.
// was: m2_s3_04_mmap_rename

use std::fs;
use std::path::PathBuf;

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn product_block<'a>(caps: &'a str, id: &str) -> &'a str {
    let needle = format!("id = \"{id}\"");
    let rest = caps
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing capability {id}"));
    rest.split("[[capability]]").next().unwrap_or(rest)
}

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

fn facade_src(body: &str) -> String {
    let fh = include_str!("../../../../packages/io/file_handle.sy");
    format!("{fh}\n{body}")
}

fn tmp_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("syx_m2_s3_04_{name}"))
}

const PAYLOAD: &[u8] = b"SYX-M2-S3-04-READALLBYTES-PAYLOAD-v1";

#[test]
fn m2_s3_04_read_all_bytes_roundtrip() {
    // Wave A W64 / ADR-408 / A-03: FileHandle readAllBytes product exact (drop print(err)).
    let path = tmp_path("roundtrip.bin");
    fs::write(&path, PAYLOAD).expect("write");
    let path_s = path.to_string_lossy().replace('\\', "\\\\");
    let out = run(&facade_src(&format!(
        r#"
func main() {{
  try {{
    let h = FileHandle.open(path: "{path_s}", mode: "r")
    let b = FileHandle.readAllBytes(handle: h)
    FileHandle.close(handle: h)
    print("OK")
    print(b.count)
    print(b[0])
    print(b[b.count - 1])
  }} catch (err) {{
    print("ERR")
  }}
}}
"#
    )));
    let _ = fs::remove_file(&path);
    assert_eq!(
        out, "OK\n36\n83\n49",
        "readAllBytes roundtrip must exact: {out}"
    );
}

#[test]
fn m2_s3_04_no_public_read_mmap() {
    let fh = include_str!("../../../../packages/io/file_handle.sy");
    assert!(
        !fh.contains("func readMmap") && !fh.contains("readMmap("),
        "public readMmap must be gone: facade still exposes it"
    );
    assert!(
        fh.contains("readAllBytes") && fh.contains("__syx_file_read_all_bytes"),
        "canonical host __syx_file_read_all_bytes required"
    );
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(
        reg.contains("\"__syx_file_read_all_bytes\"")
            || reg.contains("__syx_file_read_all_bytes"),
        "registry must expose canonical read_all_bytes name"
    );
    // Historical id kept for EC-M2-4 matrix.
    assert!(
        reg.contains("FileMmapRead"),
        "HostFnId::FileMmapRead must remain (historical)"
    );
}

#[test]
fn m2_s3_04_docs_and_capability() {

    let guide = include_str!("../../../../docs/9_UsageGuide.md");
    assert!(
        guide.contains("ADR-241")
            && guide.contains("readAllBytes")
            && guide.contains("read_to_end")
            && guide.contains("≠ OS mmap"),
        "UsageGuide must document readAllBytes honesty"
    );

    let hosts = include_str!("../../../syx-vm/src/hosts.rs");
    assert!(
        hosts.contains("ADR-241")
            && hosts.contains("read_to_end")
            && (hosts.contains("rename closeout") || hosts.contains("S3-04 rename")),
        "host comment must cite ADR-241 rename closeout / read_to_end"
    );

    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "io.file.read_all_bytes");
    assert!(
        block.contains("status = \"implemented\""),
        "io.file.read_all_bytes must be implemented: {block}"
    );
    assert!(
        block.contains("m2_s3_04_mmap_rename"),
        "tests must list nail: {block}"
    );
    assert!(
        (block.contains("ADR-558") || block.contains("ADR-241"))
            && (block.contains("≠ OS mmap") || block.contains("OS mmap") || block.contains("read_to_end")),
        "notes must cite ADR-558/241 honesty: {block}"
    );

    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m2_s3_04_mmap_rename_tier = \"signed-ADR-241\""),
        "inventory must note signed-ADR-241"
    );
}

#[test]
fn m2_s3_04_fence_no_ec_m2_4_z_or_m2_exit() {
    let roadmap = include_str!("../../../../plan/archive/roadmap-m0-m4-task-list.md");
    assert!(
        roadmap.contains("M2-S3-04")
            && roadmap.contains("ADR-241")
            && roadmap.contains("S3 叶子收口"),
        "roadmap must list M2-S3-04 / ADR-241 S3 closeout"
    );
    assert!(
        roadmap.contains("M2-S6-01") && roadmap.contains("W98"),
        "roadmap must lock next W98 = M2-S6-01"
    );
    assert!(
        roadmap.contains("**M2 已退出**")
            || roadmap.contains("ADR-259")
            || roadmap.contains("| **exited** |"),
        "W115 may claim M2 exited"
    );
    // M2-S3-04 leaf row checked; EC-M2-4-Z still open.
    let s3_04 = roadmap
        .lines()
        .find(|l| l.starts_with("| M2-S3-04 |"))
        .expect("M2-S3-04 leaf row");
    assert!(
        s3_04.contains("| [x] |"),
        "M2-S3-04 must be checked: {s3_04}"
    );
    // W214: product native/wasm=real deepens honesty without claiming EC-M2-4-Z / M2 exit.
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let block = product_block(caps, "io.file.read_all_bytes");
    assert!(
        !block.contains("EC-M2-4-Z") && !block.contains("EC-M2-4 total"),
        "W214 must not claim EC-M2-4-Z / EC-M2-4 total: {block}"
    );
    assert!(
        !block.contains("M2 已退出") && !block.contains("M2 exit"),
        "W214 must not claim M2 exit: {block}"
    );
}
