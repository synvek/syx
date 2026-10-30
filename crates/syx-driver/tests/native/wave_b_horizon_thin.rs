//! Wave B tri-axis horizon structural readiness (W335 / ADR-679).
//! Public inventory API + policy/meta; B-04/B-05/B-06 remain [~]; ≠ Wave B exit.

#[test]
fn wave_b_horizon_thin_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/wave_b_horizon.rs",
        "docs/lang/wave-b-horizon-readiness.md",
        "docs/gates/gate-b04-mir-tree-shake.md",
        "docs/gates/gate-b05-mir-metadata-blob.md",
        "docs/gates/gate-b06-llvm-stub-audit.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Wave B tri-axis horizon requires evidence file: {path}"
        );
    }
    let rt = include_str!("../../../syx-rt/src/wave_b_horizon.rs");
    assert!(
        rt.contains("WaveBTriAxisHorizonReadinessInventory")
            && rt.contains("wave_b_tri_axis_horizon_readiness_inventory"),
        "syx-rt must expose WaveBTriAxisHorizonReadinessInventory / wave_b_tri_axis_horizon_readiness_inventory (W335)"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("wave_b_horizon")
            && lib.contains("WaveBTriAxisHorizonReadinessInventory")
            && lib.contains("wave_b_tri_axis_horizon_readiness_inventory"),
        "syx-rt lib.rs must pub use Wave B tri-axis horizon APIs"
    );
}

#[test]
fn wave_b_horizon_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/wave-b-horizon-readiness.md");
    assert!(
        spec.contains("ADR-679")
            && (spec.contains("tri-axis horizon")
                || spec.contains("三轴 horizon")
                || spec.contains("Tri-Axis Horizon")),
        "spec must cite ADR-679 W335 tri-axis horizon: missing markers"
    );
    assert!(
        spec.contains("WaveBTriAxisHorizonReadinessInventory")
            && spec.contains("wave_b_tri_axis_horizon_readiness_inventory"),
        "spec must name public Wave B tri-axis horizon inventory API"
    );
    assert!(
        spec.contains("Not** Wave B exit")
            || spec.contains("≠ Wave B exit")
            || spec.contains("**Not** Wave B exit"),
        "spec must deny Wave B exit claim"
    );
    for marker in [
        "production_cutover_enabled=true",
        "production_cutover_enabled=false",
        "real_codec_enabled=false",
        "missing_clear_enabled=false",
    ] {
        assert!(
            spec.contains(marker),
            "spec must cite honesty flag {marker}"
        );
    }
    assert!(
        spec.contains("W400") || spec.contains("ADR-744"),
        "spec must cross-index W400 cutover honesty"
    );
}

