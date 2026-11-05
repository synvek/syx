//! Wave B structural exit charter evidence nails (Wave A W340 / ADR-684).
//! Policy + meta flip only — top-level sign-off of plan/02 Wave B exit triad.

#[test]
fn wave_b_exit_thin_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/wave-b-exit.md",
        "docs/lang/wave-b-exit-readiness.md",
        "docs/lang/wave-b-exit-triad-b04-defer.md",
        "docs/lang/wave-b-exit-triad-b05-defer.md",
        "docs/lang/wave-b-exit-triad-b06-defer.md",
        "crates/syx-mir/src/lib.rs",  // MirModuleMediumHorizon*Inventory (F-10 W587)
        "crates/syx-rt/src/llvm_stub_audit.rs",
        "docs/adr/stage2-wave-b-inventory-retire.md", // S2-I16 readiness inventory retired
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Wave B exit requires evidence file: {path}"
        );
    }
}

