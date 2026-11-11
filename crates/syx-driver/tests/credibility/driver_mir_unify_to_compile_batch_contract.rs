//! S2-I218–I220 S2-05 syx-driver mir_unify→test→compile batch extract contracts.
//! Nails: 3 modules + wiring + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-driver-mir-unify-to-compile-batch.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_driver_mir_unify_to_compile_batch_baseline() {
    let mods = [
        ("crates/syx-driver/src/mir_unify.rs", "build_mir_from_graph"),
        (
            "crates/syx-driver/src/test_runner.rs",
            "test_project_with_format",
        ),
        (
            "crates/syx-driver/src/compile_linked.rs",
            "compile_linked_with_options",
        ),
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
    for mod_name in ["mir_unify", "test_runner", "compile_linked"] {
        assert!(
            lib.contains(&format!("mod {mod_name};")),
            "lib.rs must declare mod {mod_name}"
        );
    }
    assert!(
        lib.contains("pub use mir_unify::{")
            && lib.contains("pub use test_runner::{")
            && lib.contains("pub use compile_linked::{"),
        "lib.rs must wire mir_unify/test_runner/compile_linked pub use"
    );
    assert!(
        !lib.contains("\nfn build_mir_from_graph(")
            && !lib.contains("\npub fn test_project_with_format(")
            && !lib.contains("\npub fn compile_linked_with_options("),
        "extracted faces must not remain defined in lib.rs"
    );
    assert!(
        lib.contains("pub(crate) fn compile_file_with_context(")
            || lib.contains("pub fn compile_file_with_context(")
            || lib.contains("pub use compile_file_context::compile_file_with_context")
            || lib.contains("mod compile_file_context;"),
        "compile_file_with_context must remain wired (lib def or I221+ sidecar re-export)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-driver-mir-unify-to-compile-batch.md");
    assert!(
        adr.contains("S2-I218") && adr.contains("S2-I219") && adr.contains("S2-I220"),
        "batch ADR must name I218–I220"
    );
    assert!(
        adr.contains("S2-I221") && adr.contains("compile_file_with_context"),
        "ADR must lock next knife = S2-I221 compile_file_with_context"
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
