//! Long delete_syx_mir_blob_enabled first-slice (W434 / ADR-778).
//! Narrative delete flag on; emit already without blob/sidecar; ≠ TLS / ≠ Short / ≠ B-05 [x].

#[test]
fn long_mir_free_delete_blob_enabled_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-llvm/src/lib.rs",
        "crates/syx-rt/src/wave_b_horizon.rs",
        "docs/lang/long-mir-free-delete-blob-enabled.md",
        "docs/lang/long-mir-free-named-stub-callhost-clear.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Long delete blob enabled requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleLongHorizonDeleteSyxMirBlobEnabledFirstSliceInventory")
            && mir.contains(
                "mir_module_long_horizon_delete_syx_mir_blob_enabled_first_slice_inventory"
            ),
        "syx-mir must expose delete_syx_mir_blob_enabled first-slice inventory (W434)"
    );
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(
        !llvm.contains("@syx_metadata_sidecar =")
            && llvm.contains("syx_rt_install_empty_runtime")
            && llvm.contains("syx_rt_register_native_fn")
            && !llvm.contains("@syx_mir_blob ="),
        "llvm production emit must remain empty+register without sidecar/blob (W434)"
    );
    let wave_b = include_str!("../../../syx-rt/src/wave_b_horizon.rs");
    assert!(
        wave_b.contains("delete_syx_mir_blob_enabled_first_slice_signed: true")
            || wave_b.contains("delete_syx_mir_blob_enabled_first_slice_signed:true"),
        "wave_b must set delete_syx_mir_blob_enabled_first_slice_signed=true (W434)"
    );
    assert!(
        wave_b.contains("delete_syx_mir_blob_enabled: true")
            || wave_b.contains("delete_syx_mir_blob_enabled:true"),
        "wave_b must set delete_syx_mir_blob_enabled=true (W434)"
    );
}

#[test]
fn long_mir_free_delete_blob_enabled_spec_signed() {
    let spec = include_str!("../../../../docs/lang/long-mir-free-delete-blob-enabled.md");
    assert!(
        spec.contains("ADR-778")
            && (spec.contains("delete_syx_mir_blob_enabled")
                || spec.contains("delete blob enabled")),
        "spec must cite ADR-778 W434 delete_syx_mir_blob_enabled"
    );
    assert!(
        spec.contains("MirModuleLongHorizonDeleteSyxMirBlobEnabledFirstSliceInventory")
            && spec.contains(
                "mir_module_long_horizon_delete_syx_mir_blob_enabled_first_slice_inventory"
            ),
        "spec must name delete inventory API"
    );
    assert!(
        spec.contains("delete_syx_mir_blob_enabled_first_slice_signed=true")
            || spec.contains("`delete_syx_mir_blob_enabled_first_slice_signed=true`"),
        "spec must cite delete_syx_mir_blob_enabled_first_slice_signed=true"
    );
    assert!(
        spec.contains("delete_syx_mir_blob_enabled=true")
            || spec.contains("`delete_syx_mir_blob_enabled=true`"),
        "spec must cite delete_syx_mir_blob_enabled=true"
    );
    assert!(
        spec.contains("mir_free_enabled=true") || spec.contains("`mir_free_enabled=true`"),
        "spec must cite mir_free_enabled=true"
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
            && !line.contains("ADR-791")
            && !line.contains("W447")
            && !line.contains("exit")
            && !line.contains("medium-exit")
            && !line.contains("narrow")
        {
            panic!("affirmative B-05 [x] claim forbidden: {line}");
        }
    }
}


#[test]
fn long_mir_free_delete_blob_enabled_inventory_api_callable() {
    use syx_mir::{
        aot_codec_fixture_main_with_const_strs,
        mir_module_long_horizon_delete_syx_mir_blob_enabled_first_slice_inventory,
        mir_module_long_horizon_named_hostfn_stub_call_host_clear_first_slice_inventory,
    };
    let module = aot_codec_fixture_main_with_const_strs(4);
    let inv = mir_module_long_horizon_delete_syx_mir_blob_enabled_first_slice_inventory(&module);
    assert!(inv.delete_syx_mir_blob_enabled_first_slice_signed);
    assert!(inv.delete_syx_mir_blob_enabled);
    assert!(inv.mir_free_enabled);
    assert!(inv.named_hostfn_stub_call_host_clear_signed);
    assert!(!inv.blob_still_embedded);
    assert!(!inv.sidecar_constant_still_embedded);
    let clear =
        mir_module_long_horizon_named_hostfn_stub_call_host_clear_first_slice_inventory(&module);
    assert!(
        !clear.delete_syx_mir_blob_enabled,
        "W433 named-clear inventory must keep historical delete_syx_mir_blob_enabled=false"
    );
}
