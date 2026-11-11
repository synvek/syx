//! S2-I230–I232 S2-05 syx-driver compile→gates→debug batch extract contracts.
//! Nails: 3 modules + wiring + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit / ≠ driver peel exhausted.
//! See docs/adr/stage2-s2-05-driver-compile-to-debug-batch.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_driver_compile_to_debug_batch_baseline() {
    let mods = [
        (
            "crates/syx-driver/src/compile_to_program.rs",
            "compile_to_program",
        ),
        (
            "crates/syx-driver/src/feature_gates.rs",
            "result_legacy_object_enabled",
        ),
        ("crates/syx-driver/src/debug_repl.rs", "debug_project"),
    ];
    for (path, needle) in mods {
        let src = repo_file(path);
        assert!(
            src.contains(needle)
                || src.contains(&format!("fn {needle}"))
                || src.contains(&format!("pub fn {needle}"))
                || src.contains(&format!("pub(crate) fn {needle}")),
            "{path} must hold {needle}"
        );
    }

    let lib = repo_file("crates/syx-driver/src/lib.rs");
    for mod_name in ["compile_to_program", "feature_gates", "debug_repl"] {
        assert!(
            lib.contains(&format!("mod {mod_name};")),
            "lib.rs must declare mod {mod_name}"
        );
    }
    assert!(
        lib.contains("pub(crate) use compile_to_program::{")
            && lib.contains("pub use feature_gates::{")
            && lib.contains("pub use debug_repl::{"),
        "lib.rs must wire compile_to_program/feature_gates/debug_repl pub use"
    );
    assert!(
        !lib.contains("\npub(crate) fn compile_to_program(")
            && !lib.contains("\npub fn result_legacy_object_enabled(")
            && !lib.contains("\npub fn debug_project("),
        "extracted faces must not remain defined in lib.rs"
    );
    assert!(
        lib.contains("pub use doc_project::doc_project") || lib.contains("doc_project::"),
        "doc_project must be wired (I233–I235 landed)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-driver-compile-to-debug-batch.md");
    assert!(
        adr.contains("S2-I230") && adr.contains("S2-I231") && adr.contains("S2-I232"),
        "batch ADR must name I230–I232"
    );
    assert!(
        adr.contains("S2-I233") && adr.contains("production close-out"),
        "ADR must name S2-I233 production close-out ladder"
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
