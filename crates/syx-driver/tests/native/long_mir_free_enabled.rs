//! Long MIR-free first-slice (W432 / ADR-776).
//! Flip mir_free_enabled=true after sidecar emit; ≠ stub clear / ≠ B-05 [x].

#[test]
fn long_mir_free_enabled_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-llvm/src/lib.rs",
        "crates/syx-rt/src/wave_b_horizon.rs",
        "docs/lang/long-mir-free-enabled.md",
        "docs/lang/long-mir-free-sidecar-omit-emit.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Long MIR-free enabled requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleLongHorizonMirFreeFirstSliceInventory")
            && mir.contains("mir_module_long_horizon_mir_free_first_slice_inventory"),
        "syx-mir must expose MIR-free first-slice inventory (W432)"
    );
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(
        !llvm.contains("@syx_metadata_sidecar =")
            && llvm.contains("syx_rt_install_empty_runtime")
            && llvm.contains("syx_rt_register_native_fn")
            && !llvm.contains("@syx_mir_blob ="),
        "llvm production emit must remain empty+register without sidecar/blob (W432)"
    );
    let wave_b = include_str!("../../../syx-rt/src/wave_b_horizon.rs");
    assert!(
        wave_b.contains("mir_free_first_slice_signed: true")
            || wave_b.contains("mir_free_first_slice_signed:true"),
        "wave_b must set mir_free_first_slice_signed=true (W432)"
    );
    assert!(
        wave_b.contains("mir_free_enabled: true") || wave_b.contains("mir_free_enabled:true"),
        "wave_b must set mir_free_enabled=true (W432)"
    );
}

#[test]
fn long_mir_free_enabled_spec_signed() {
    let spec = include_str!("../../../../docs/lang/long-mir-free-enabled.md");
    assert!(
        spec.contains("ADR-776")
            && (spec.contains("MIR-free first-slice") || spec.contains("mir_free_enabled=true")),
        "spec must cite ADR-776 W432 MIR-free first-slice"
    );
    assert!(
        spec.contains("MirModuleLongHorizonMirFreeFirstSliceInventory")
            && spec.contains("mir_module_long_horizon_mir_free_first_slice_inventory"),
        "spec must name MIR-free inventory API"
    );
    assert!(
        spec.contains("mir_free_first_slice_signed=true")
            || spec.contains("`mir_free_first_slice_signed=true`"),
        "spec must cite mir_free_first_slice_signed=true"
    );
    assert!(
        spec.contains("mir_free_enabled=true") || spec.contains("`mir_free_enabled=true`"),
        "spec must cite mir_free_enabled=true"
    );
    assert!(
        spec.contains("delete_syx_mir_blob_enabled=false")
            || spec.contains("`delete_syx_mir_blob_enabled=false`"),
        "spec must cite delete_syx_mir_blob_enabled=false"
    );
    for line in spec.lines() {
        if line.contains("B-05")
            && line.contains("[x]")
            && !line.contains("≠")
            && !line.contains("仍")
            && !line.contains("Frozen")
            && !line.contains("still")
            && !line.contains("Forbidden")
            && !line.contains("Claiming")
            && !line.contains("≠")
        {
            panic!("affirmative B-05 [x] claim forbidden: {line}");
        }
    }
}


#[test]
fn long_mir_free_enabled_inventory_api_callable() {
    use syx_mir::{
        aot_codec_fixture_main_with_const_strs,
        mir_module_long_horizon_mir_free_first_slice_inventory,
        mir_module_long_horizon_sidecar_omit_emit_first_slice_inventory,
    };
    let module = aot_codec_fixture_main_with_const_strs(4);
    let inv = mir_module_long_horizon_mir_free_first_slice_inventory(&module);
    assert!(inv.mir_free_first_slice_signed);
    assert!(inv.mir_free_enabled);
    assert!(!inv.delete_syx_mir_blob_enabled);
    assert!(!inv.sidecar_constant_still_embedded);
    assert!(!inv.blob_still_embedded);
    let emit = mir_module_long_horizon_sidecar_omit_emit_first_slice_inventory(&module);
    assert!(
        !emit.mir_free_enabled,
        "W431 emit inventory must keep historical mir_free_enabled=false"
    );
}
