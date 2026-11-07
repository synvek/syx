//! S2-I221–I223 S2-05 syx-driver compile_ctx→prelude→native batch extract contracts.
//! Nails: 3 modules + wiring + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-driver-compile-ctx-to-native-batch.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_driver_compile_ctx_to_native_batch_baseline() {
    let mods = [
        (
            "crates/syx-driver/src/compile_file_context.rs",
            "compile_file_with_context",
        ),
        (
            "crates/syx-driver/src/prelude_glue.rs",
            "core_prelude_import_symbols",
        ),
        ("crates/syx-driver/src/native_build.rs", "build_native"),
    ];
    for (path, fn_name) in mods {
        let src = repo_file(path);
        assert!(
            src.contains(&format!("fn {fn_name}"))
                || src.contains(&format!("pub(crate) fn {fn_name}"))
                || src.contains(&format!("pub fn {fn_name}")),
            "{path} must hold {fn_name}"
        );
    }

    let lib = repo_file("crates/syx-driver/src/lib.rs");
    for mod_name in ["compile_file_context", "prelude_glue", "native_build"] {
        assert!(
            lib.contains(&format!("mod {mod_name};")),
            "lib.rs must declare mod {mod_name}"
        );
    }
    assert!(
        lib.contains("pub use compile_file_context::")
            && lib.contains("pub use prelude_glue::{")
            && lib.contains("pub(crate) use prelude_glue::{")
            && lib.contains("pub use native_build::{"),
        "lib.rs must wire compile_file_context/prelude_glue/native_build uses"
    );
    assert!(
        !lib.contains("\npub fn compile_file_with_context(")
            && !lib.contains("\npub(crate) fn core_prelude_import_symbols(")
            && !lib.contains("\npub fn build_native(root:"),
        "extracted faces must not remain defined in lib.rs"
    );
    assert!(
        lib.contains("pub struct DriverOptions")
            || lib.contains("pub struct ModuleUnit")
            || lib.contains("pub use driver_options::{")
            || lib.contains("mod driver_options;"),
        "DriverOptions/ModuleUnit hub must remain wired (lib def or I224+ sidecar re-export)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-driver-compile-ctx-to-native-batch.md");
    assert!(
        adr.contains("S2-I221") && adr.contains("S2-I222") && adr.contains("S2-I223"),
        "batch ADR must name I221–I223"
    );
    assert!(
        adr.contains("S2-I224") && adr.contains("driver_options"),
        "ADR must lock next knife = S2-I224 driver_options hub"
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
