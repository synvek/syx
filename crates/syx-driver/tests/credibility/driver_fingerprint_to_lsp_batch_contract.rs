//! S2-I215–I217 S2-05 syx-driver fingerprint→lsp batch extract contracts.
//! Nails: 3 modules + wiring + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-driver-fingerprint-to-lsp-batch.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_driver_fingerprint_to_lsp_batch_baseline() {
    let mods = [
        (
            "crates/syx-driver/src/cache_fingerprint.rs",
            "heap_v100_phase128_track_c_wasm_digest",
        ),
        (
            "crates/syx-driver/src/farm_tcp.rs",
            "farm_tcp_handle_request",
        ),
        (
            "crates/syx-driver/src/lsp_ide.rs",
            "analyze_document_for_lsp",
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
    for mod_name in ["cache_fingerprint", "farm_tcp", "lsp_ide"] {
        assert!(
            lib.contains(&format!("mod {mod_name};")),
            "lib.rs must declare mod {mod_name}"
        );
    }
    assert!(
        lib.contains("use cache_fingerprint::{")
            && lib.contains("pub use farm_tcp::{")
            && lib.contains("pub use lsp_ide::{"),
        "lib.rs must wire fingerprint use + farm/lsp pub use"
    );
    assert!(
        !lib.contains("\nfn heap_v100_phase128_track_c_wasm_digest(")
            && !lib.contains("\npub fn farm_tcp_handle_request(")
            && !lib.contains("\npub fn analyze_document_for_lsp("),
        "extracted faces must not remain defined in lib.rs"
    );
    assert!(
        lib.contains("mod lsp_completion;"),
        "lsp_completion sidecar must remain (not absorbed)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-driver-fingerprint-to-lsp-batch.md");
    assert!(
        adr.contains("S2-I215") && adr.contains("S2-I216") && adr.contains("S2-I217"),
        "batch ADR must name I215–I217"
    );
    assert!(
        adr.contains("S2-I218") && adr.contains("mir_unify"),
        "ADR must lock next knife = S2-I218 mir_unify"
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
