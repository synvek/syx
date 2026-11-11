//! S2-I233–I235 S2-05 syx-driver production close-out batch extract contracts.
//! Nails: 3 modules + wiring + ADR + host_rt stubs remain; ≠ S2-05 [x] / ≠ Stage-2 exit /
//! ≠ lib peel fully exhausted.
//! See docs/adr/stage2-s2-05-driver-production-closeout-batch.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_driver_production_closeout_batch_baseline() {
    let mods = [
        ("crates/syx-driver/src/doc_project.rs", "doc_project"),
        (
            "crates/syx-driver/src/cranelift_stubs.rs",
            "cranelift_backend_retired",
        ),
        ("crates/syx-driver/src/thin_host.rs", "publish_package"),
    ];
    for (path, needle) in mods {
        let src = repo_file(path);
        assert!(
            src.contains(needle)
                || src.contains(&format!("fn {needle}"))
                || src.contains(&format!("pub fn {needle}")),
            "{path} must hold {needle}"
        );
    }

    let lib = repo_file("crates/syx-driver/src/lib.rs");
    for mod_name in ["doc_project", "cranelift_stubs", "thin_host"] {
        assert!(
            lib.contains(&format!("mod {mod_name};")),
            "lib.rs must declare mod {mod_name}"
        );
    }
    assert!(
        lib.contains("pub use doc_project::doc_project")
            && lib.contains("pub use cranelift_stubs::{")
            && lib.contains("pub use thin_host::{"),
        "lib.rs must wire doc_project/cranelift_stubs/thin_host pub use"
    );
    assert!(
        !lib.contains("\npub fn doc_project(")
            && !lib.contains("\npub fn cranelift_backend_retired(")
            && !lib.contains("\npub fn publish_package(")
            && !lib.contains("\npub fn backfill_mono_witness_cache("),
        "extracted faces must not remain defined in lib.rs"
    );
    assert!(
        lib.contains("\npub fn ensure_host_rt_link_stubs("),
        "ensure_host_rt_link_stubs must remain at crate root"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-driver-production-closeout-batch.md");
    assert!(
        adr.contains("S2-I233") && adr.contains("S2-I234") && adr.contains("S2-I235"),
        "batch ADR must name I233–I235"
    );
    assert!(
        adr.contains("S2-I236") && adr.contains("syx-semantic"),
        "ADR must lock next knife = S2-I236 syx-semantic first extract"
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
