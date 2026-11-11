//! S2-I227–I229 S2-05 syx-driver format→cache→wasm batch extract contracts.
//! Nails: 3 modules + wiring + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit / ≠ driver peel exhausted.
//! See docs/adr/stage2-s2-05-driver-format-to-wasm-batch.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_driver_format_to_wasm_batch_baseline() {
    let mods = [
        ("crates/syx-driver/src/format_lint.rs", "format_project"),
        ("crates/syx-driver/src/compile_cache.rs", "CachedModuleUnit"),
        ("crates/syx-driver/src/run_wasm.rs", "run_wasm_bytes"),
    ];
    for (path, needle) in mods {
        let src = repo_file(path);
        assert!(
            src.contains(needle)
                || src.contains(&format!("fn {needle}"))
                || src.contains(&format!("pub fn {needle}"))
                || src.contains(&format!("struct {needle}")),
            "{path} must hold {needle}"
        );
    }

    let lib = repo_file("crates/syx-driver/src/lib.rs");
    for mod_name in ["format_lint", "compile_cache", "run_wasm"] {
        assert!(
            lib.contains(&format!("mod {mod_name};")),
            "lib.rs must declare mod {mod_name}"
        );
    }
    assert!(
        lib.contains("pub use format_lint::{")
            && lib.contains("pub use compile_cache::")
            && lib.contains("pub use run_wasm::{"),
        "lib.rs must wire format_lint/compile_cache/run_wasm pub use"
    );
    assert!(
        !lib.contains("\npub fn format_project(")
            && !lib.contains("\npub(crate) struct CachedModuleUnit {")
            && !lib.contains("\npub fn run_wasm_bytes("),
        "extracted faces must not remain defined in lib.rs"
    );
    assert!(
        lib.contains("pub(crate) use compile_to_program::{")
            || lib.contains("compile_to_program::"),
        "compile_to_program must be wired (I230–I232 landed)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-driver-format-to-wasm-batch.md");
    assert!(
        adr.contains("S2-I227") && adr.contains("S2-I228") && adr.contains("S2-I229"),
        "batch ADR must name I227–I229"
    );
    assert!(
        adr.contains("S2-I230") && adr.contains("compile_to_program"),
        "ADR must name S2-I230 compile_to_program ladder"
    );
    assert!(
        adr.contains("≠") && adr.contains("S2-05"),
        "ADR must Non-claim S2-05 [x]"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-2"),
        "ADR must Non-claim Stage-2 exit"
    );
}
