//! ADR-199 / M2 W55: EC-M2-4 Path/File native/WASM availability matrix first deepening.
// was: ec_m2_4_path_file_native_wasm_matrix

const PATH_FILE_HOSTS: &[&str] = &[
    "PathJoin",
    "PathNormalize",
    "PathExists",
    "PathExtension",
    "FileOpen",
    "FileClose",
    "FileRead",
    "FileWrite",
    "FileSeek",
    "FileMmapRead",
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

#[test]
fn ec_m2_4_path_file_native_wasm_matrix_host_fn_columns() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in PATH_FILE_HOSTS {
        let block = host_block(toml, name);
        assert!(
            block.contains("vm = \"real\""),
            "{name} must keep vm=real: {block}"
        );
        assert!(
            block.contains("cranelift = \"unsupported\""),
            "{name} must declare cranelift=unsupported: {block}"
        );
        // Wave A W268 / ADR-612: Path×4 + FileOpen/Close/Read/Write wasm=real;
        // Wave A W269 / ADR-613: FileSeek/FileMmapRead wasm=real (File* HostFn wasm 收官).
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-612/613): {block}"
        );
        assert!(
            block.contains("ADR-612") || block.contains("ADR-613"),
            "{name} notes must cite ADR-612 or ADR-613: {block}"
        );
        // ADR-204/209/214/491/536/537/538: PathJoin / PathExtension / PathNormalize / PathExists / FileOpen / FileClose / FileRead / FileWrite / FileSeek / FileMmapRead may declare llvm=real.
        if *name == "PathJoin"
            || *name == "PathExtension"
            || *name == "PathNormalize"
            || *name == "PathExists"
            || *name == "FileOpen"
            || *name == "FileClose"
            || *name == "FileRead"
            || *name == "FileWrite"
            || *name == "FileSeek"
            || *name == "FileMmapRead"
        {
            assert!(
                block.contains("llvm = \"real\"") || block.contains("llvm = \"partial\""),
                "{name} must declare llvm=real (GA) or legacy partial: {block}"
            );
        } else {
            assert!(
                block.contains("llvm = \"partial\""),
                "{name} must declare llvm=partial: {block}"
            );
            assert!(
                !block.contains("llvm = \"real\""),
                "{name} must not claim llvm=real (≠ GA): {block}"
            );
        }
    }
}

#[test]
fn ec_m2_4_path_file_native_wasm_matrix_generated_md_columns() {
    let md = include_str!("../../../../books/shared/host-fn-capabilities.md");
    // Wave A W268–W269: Path/File HostFn wasm 收官（含 Seek/Mmap）.
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
        let row = md
            .lines()
            .find(|l| l.starts_with(&format!("| {name} |")))
            .unwrap_or_else(|| panic!("{name} row missing"));
        assert!(
            row.contains("| unsupported |"),
            "{name} row must list unsupported: {row}"
        );
        assert!(
            row.contains("| real | real | real | real |"),
            "{name} row must list wasm=real (ADR-612/613): {row}"
        );
        assert!(
            !row.contains("| host-assisted |"),
            "{name} must not list host-assisted after ADR-613: {row}"
        );
    }

    let file_seek = md
        .lines()
        .find(|l| l.starts_with("| FileSeek |"))
        .expect("FileSeek row missing");
    assert!(
        file_seek.contains("| real | real | real | real |"),
        "FileSeek row must list wasm=real (ADR-613): {file_seek}"
    );
}

#[test]
fn ec_m2_4_path_file_native_wasm_matrix_product_columns() {
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    let io_path = product_block(caps, "io.path");
    assert!(
        io_path.contains("native = \"real\"") && io_path.contains("wasm = \"real\""),
        "io.path must declare native+wasm=real: {io_path}"
    );
    assert!(
        io_path.contains("status = \"implemented\""),
        "io.path status must be implemented: {io_path}"
    );
    assert!(
        io_path.contains("ADR-559") || io_path.contains("ADR-557") || io_path.contains("ADR-199") || io_path.contains("native/WASM matrix"),
        "io.path notes must cite ADR-559 or ADR-557: {io_path}"
    );
    assert!(
        io_path.contains("ec_m2_4_path_file_native_wasm_matrix"),
        "io.path tests must include ec_m2_4_path_file_native_wasm_matrix: {io_path}"
    );
    let file_read = product_block(caps, "io.file.read_all_bytes");
    assert!(
        file_read.contains("native = \"real\"") && file_read.contains("wasm = \"real\""),
        "io.file.read_all_bytes native/wasm must be real: {file_read}"
    );
    assert!(
        file_read.contains("mir = \"real\""),
        "io.file.read_all_bytes mir must stay real: {file_read}"
    );
    assert!(
        file_read.contains("status = \"implemented\""),
        "io.file.read_all_bytes status must stay implemented: {file_read}"
    );
    assert!(
        file_read.contains("ADR-558") || file_read.contains("ADR-241") || file_read.contains("read_to_end"),
        "io.file.read_all_bytes notes must cite ADR-558 or ADR-241: {file_read}"
    );
    assert!(
        file_read.contains("ec_m2_4_path_file_native_wasm_ga_read_all_bytes_native_mir")
            || file_read.contains("m2_s3_04_mmap_rename"),
        "io.file.read_all_bytes tests must include read_all_bytes nail: {file_read}"
    );
}

#[test]
fn ec_m2_4_path_file_native_wasm_matrix_honesty_fences() {
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

    let oracle = include_str!(
        "../fixtures/differential/suite2/file_text_roundtrip.oracle.json"
    );
    assert!(
        oracle.contains("\"wasm\": true") || oracle.contains("\"wasm\":true"),
        "Suite-2 file_text_roundtrip must still skip wasm: {oracle}"
    );
}

#[test]
fn ec_m2_4_path_file_native_wasm_matrix_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("ec_m2_4_path_file_native_wasm_matrix_tier"),
        "inventory must contain ec_m2_4_path_file_native_wasm_matrix_tier: {inv}"
    );
    assert!(
        inv.contains("ADR-199") || inv.contains("signed-ADR-199"),
        "inventory must note ADR-199: {inv}"
    );
}

#[test]
fn ec_m2_4_path_file_native_wasm_matrix_ci_wires() {
    let ci = include_str!("../../../../.github/workflows/ci.yml");
    assert!(
        ci.contains("--test backends"),
        "ci.yml must wire ec_m2_4_path_file_native_wasm_matrix"
    );
}
