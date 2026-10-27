//! MIR residual ABI narrow cleanup (W484 / ADR-828).
//! Omit IR declare + dead sidecar encode; retain RT ABI; ≠ blob distance change / ≠ B-09 [x].

#[test]
fn mir_residual_abi_narrow_cleanup_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/wave_b_horizon.rs",
        "crates/syx-rt/src/abi.rs",
        "crates/syx-llvm/src/lib.rs",
        "docs/lang/mir-residual-abi-narrow-cleanup.md",
        "docs/lang/mir-residual-honesty.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "residual ABI narrow cleanup requires evidence file: {path}"
        );
    }
    let wave_b = include_str!("../../../syx-rt/src/wave_b_horizon.rs");
    assert!(
        wave_b.contains("MirResidualAbiNarrowCleanupInventory")
            && wave_b.contains("mir_residual_abi_narrow_cleanup_inventory")
            && wave_b.contains("ADR-828")
            && (wave_b.contains("mir_residual_abi_narrow_cleanup_signed: true")
                || wave_b.contains("mir_residual_abi_narrow_cleanup_signed:true")),
        "syx-rt must expose MirResidualAbiNarrowCleanupInventory (W484 / ADR-828)"
    );
    assert!(
        (wave_b.contains("residual_cleanup_enabled: true")
            || wave_b.contains("residual_cleanup_enabled:true"))
            && (wave_b.contains("residual_ir_declare_omitted: true")
                || wave_b.contains("residual_ir_declare_omitted:true"))
            && (wave_b.contains("rt_abi_delete_enabled: false")
                || wave_b.contains("rt_abi_delete_enabled:false")),
        "narrow cleanup inventory must enable cleanup, omit declare, keep RT ABI delete false"
    );
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(
        !llvm.contains("out.push_str(\"declare void @syx_rt_load_mir_blob")
            && llvm.contains("fn aot_compact_view")
            && llvm.contains("compact_module_for_aot_blob")
            && llvm.contains("syx_rt_install_empty_runtime")
            && llvm.contains("ADR-828"),
        "llvm must omit residual declare push while retaining compact helpers (W484)"
    );
    let shim_start = llvm
        .find("fn append_aot_runtime_shim")
        .expect("append_aot_runtime_shim present");
    let shim = &llvm[shim_start..];
    let shim_end = shim.find("\nfn ").unwrap_or(shim.len().min(3500));
    let shim_body = &shim[..shim_end];
    assert!(
        !shim_body.contains("call void @syx_rt_load_mir_blob")
            && !shim_body.contains("aot_compact_view")
            && !shim_body.contains("encode_module_metadata_sidecar")
            && shim_body.contains("compact_module_for_aot_blob"),
        "production shim must compact-only without load call / dead encode"
    );
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    assert!(
        abi.contains("syx_rt_load_mir_blob") || abi.contains("fn syx_rt_load_mir_blob"),
        "RT ABI must still export syx_rt_load_mir_blob (narrow cleanup retains ABI)"
    );
}

#[test]
fn mir_residual_abi_narrow_cleanup_spec_signed() {
    let spec = include_str!("../../../../docs/lang/mir-residual-abi-narrow-cleanup.md");
    assert!(
        spec.contains("ADR-828")
            && (spec.contains("narrow cleanup") || spec.contains("residual ABI")),
        "spec must cite ADR-828 residual ABI narrow cleanup"
    );
    assert!(
        spec.contains("three-hundred-and-thirty-fourth"),
        
        "spec must cite W484 fingerprint"
    );
    assert!(
        spec.contains("MirResidualAbiNarrowCleanupInventory")
            && spec.contains("mir_residual_abi_narrow_cleanup_inventory"),
        "spec must name residual ABI narrow cleanup inventory API"
    );
    assert!(
        spec.contains("mir_residual_abi_narrow_cleanup_signed=true")
            || spec.contains("`mir_residual_abi_narrow_cleanup_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("residual_cleanup_enabled=true")
            || spec.contains("`residual_cleanup_enabled=true`"))
            && (spec.contains("residual_ir_declare_omitted=true")
                || spec.contains("`residual_ir_declare_omitted=true`"))
            && (spec.contains("rt_abi_delete_enabled=false")
                || spec.contains("`rt_abi_delete_enabled=false`")),
        "spec must cite cleanup true, declare omitted, rt_abi_delete false"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("RT ABI") || spec.contains("rt_abi") || spec.contains("B-09")),
        "spec Frozen must fence RT ABI delete / B-09"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn mir_residual_abi_narrow_cleanup_inventory_nails() {
    let inv = syx_rt::mir_residual_abi_narrow_cleanup_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(!inv.production_blob_embedded);
    assert!(inv.residual_honesty_floor_ok);
    assert!(inv.residual_cleanup_enabled);
    assert!(inv.residual_ir_declare_omitted);
    assert!(inv.residual_dead_sidecar_encode_omitted);
    assert!(inv.residual_rt_abi_exported);
    assert!(!inv.rt_abi_delete_enabled);
    assert!(inv.mir_residual_abi_narrow_cleanup_signed);
    let floor = syx_rt::mir_residual_honesty_inventory();
    assert!(floor.residual_ir_declare_present);
    assert!(floor.residual_aot_compact_view_dead);
    assert!(!floor.residual_cleanup_enabled);
    assert!(floor.residual_honesty_signed);
}

#[test]
fn mir_residual_abi_narrow_cleanup_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W484")
            && (roadmap.contains("ADR-828")
                || roadmap.contains("residual")
                || roadmap.contains("窄清理")
                || roadmap.contains("declare")),
        "roadmap must cite W484 / ADR-828 residual ABI narrow cleanup"
    );
    let b02 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-02 |"))
        .expect("B-02 row");
    assert!(b02.contains("[x]"), "roadmap B-02 must remain [x]: {b02}");
    let b09 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-09 |"))
        .expect("B-09 row");
    assert!(
        b09.contains("[x]") || b09.contains("[~]"),
        "roadmap B-09 must be [x] (W558+) or historical [~]: {b09}"
    );
}
