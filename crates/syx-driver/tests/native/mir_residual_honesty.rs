//! MIR residual honesty first-slice (W459 / ADR-803).
//! Production omit + residual declare/ABI/dead encode honesty; ≠ delete ABI / ≠ exports.
//! W484 live llvm omit declare is signed via mir_residual_abi_narrow_cleanup; this meta keeps
//! historical inventory floors + docs (declare_present=true / cleanup=false).

#[test]
fn mir_residual_honesty_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/wave_b_horizon.rs",
        "crates/syx-rt/src/abi.rs",
        "crates/syx-llvm/src/lib.rs",
        "docs/lang/mir-residual-honesty.md",
        "docs/lang/long-mir-free-readiness.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "MIR residual honesty requires evidence file: {path}"
        );
    }
    let wave_b = include_str!("../../../syx-rt/src/wave_b_horizon.rs");
    assert!(
        wave_b.contains("MirResidualHonestyInventory")
            && wave_b.contains("mir_residual_honesty_inventory")
            && wave_b.contains("ADR-803")
            && (wave_b.contains("residual_honesty_signed: true")
                || wave_b.contains("residual_honesty_signed:true")),
        "syx-rt must expose MirResidualHonestyInventory (W459 / ADR-803)"
    );
    // Historical floor: honesty inventory still claims declare_present / dead encode / cleanup off.
    assert!(
        (wave_b.contains("residual_ir_declare_present: true")
            || wave_b.contains("residual_ir_declare_present:true"))
            && (wave_b.contains("residual_aot_compact_view_dead: true")
                || wave_b.contains("residual_aot_compact_view_dead:true"))
            && (wave_b.contains("residual_cleanup_enabled: false")
                || wave_b.contains("residual_cleanup_enabled:false")),
        "W459 honesty inventory historical floor flags must remain"
    );
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(
        llvm.contains("fn aot_compact_view")
            && !llvm.contains("@syx_mir_blob =")
            && llvm.contains("syx_rt_install_empty_runtime"),
        "llvm must retain aot_compact_view probe helper while production omits blob"
    );
    let shim_start = llvm
        .find("fn append_aot_runtime_shim")
        .expect("append_aot_runtime_shim present");
    let shim = &llvm[shim_start..];
    let shim_end = shim.find("\nfn ").unwrap_or(shim.len().min(3500));
    let shim_body = &shim[..shim_end];
    assert!(
        !shim_body.contains("call void @syx_rt_load_mir_blob")
            && !shim_body.contains("syx_rt_load_mir_blob("),
        "production shim path must not call syx_rt_load_mir_blob"
    );
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    assert!(
        abi.contains("syx_rt_load_mir_blob") || abi.contains("fn syx_rt_load_mir_blob"),
        "RT ABI must still export syx_rt_load_mir_blob (residual)"
    );
}

#[test]
fn mir_residual_honesty_spec_signed() {
    let spec = include_str!("../../../../docs/lang/mir-residual-honesty.md");
    assert!(
        spec.contains("ADR-803")
            && (spec.contains("residual honesty") || spec.contains("Residual")),
        "spec must cite ADR-803 residual honesty"
    );
    assert!(
        spec.contains("three-hundred-and-ninth"),
        
        "spec must cite W459 fingerprint"
    );
    assert!(
        spec.contains("MirResidualHonestyInventory")
            && spec.contains("mir_residual_honesty_inventory"),
        "spec must name residual honesty inventory API"
    );
    assert!(
        spec.contains("residual_honesty_signed=true")
            || spec.contains("`residual_honesty_signed=true`"),
        "spec must cite residual_honesty_signed=true"
    );
    assert!(
        spec.contains("residual_cleanup_enabled=false")
            || spec.contains("`residual_cleanup_enabled=false`"),
        "spec must cite residual_cleanup_enabled=false (historical honesty floor)"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("ABI") || spec.contains("declare") || spec.contains("exports")),
        "spec Frozen must fence ABI delete / declare / exports"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn mir_residual_honesty_inventory_nails() {
    let inv = syx_rt::mir_residual_honesty_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(!inv.production_blob_embedded);
    assert!(!inv.production_user_init_calls_load);
    assert!(inv.residual_ir_declare_present);
    assert!(inv.residual_rt_abi_exported);
    assert!(inv.residual_aot_compact_view_dead);
    assert!(!inv.residual_cleanup_enabled);
    assert!(inv.residual_honesty_signed);
}

