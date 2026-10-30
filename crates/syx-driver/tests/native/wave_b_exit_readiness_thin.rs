//! Wave B structural exit readiness (W336 / ADR-680).
//! Public inventory API + policy/meta; B-04/B-05/B-06 remain [~].
//! W340: charter signed via wave-b-exit.md — readiness inventory flags stay false.

#[test]
fn wave_b_exit_readiness_thin_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/wave_b_horizon.rs",
        "docs/lang/wave-b-exit-readiness.md",
        "docs/lang/wave-b-horizon-readiness.md",
        "docs/lang/wave-b-exit.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Wave B exit readiness requires evidence file: {path}"
        );
    }
    let rt = include_str!("../../../syx-rt/src/wave_b_horizon.rs");
    assert!(
        rt.contains("WaveBStructuralExitReadinessInventory")
            && rt.contains("wave_b_structural_exit_readiness_inventory"),
        "syx-rt must expose WaveBStructuralExitReadinessInventory / wave_b_structural_exit_readiness_inventory (W336)"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("WaveBStructuralExitReadinessInventory")
            && lib.contains("wave_b_structural_exit_readiness_inventory"),
        "syx-rt lib.rs must pub use Wave B structural exit readiness APIs"
    );
}

#[test]
fn wave_b_exit_readiness_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/wave-b-exit-readiness.md");
    assert!(
        spec.contains("ADR-680")
            && (spec.contains("exit readiness")
                || spec.contains("结构退出就绪")
                || spec.contains("Structural Exit Readiness")),
        "spec must cite ADR-680 W336 exit readiness: missing markers"
    );
    assert!(
        spec.contains("WaveBStructuralExitReadinessInventory")
            && spec.contains("wave_b_structural_exit_readiness_inventory"),
        "spec must name public Wave B structural exit readiness inventory API"
    );
    assert!(
        spec.contains("wave-b-exit.md")
            && (spec.contains("ADR-684")
                || spec.contains("W340")
                || spec.contains("Wave B exit signed")),
        "spec must cross-index W340 wave-b-exit charter"
    );
    for marker in [
        "wave_b_exit_enabled=false",
        "axes_complete_enabled=false",
        "production_delivery_enabled=false",
    ] {
        assert!(
            spec.contains(marker),
            "spec must cite honesty flag {marker}"
        );
    }
    assert!(
        spec.contains("诚实延后签收 met") && (spec.contains("3/3") || spec.contains("ADR-683")),
        "spec exit triad must remain 3/3 defer met"
    );
    assert!(
        !spec.contains("| **B-05 Medium 生产交付或诚实延后签收** | **未 met**"),
        "B-05 triad row must not remain unmet after W339"
    );
}

