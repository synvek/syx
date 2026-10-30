//! Long native-metadata wire first-slice (W412 / ADR-756).
//! Public C ABI + inventory; gap #1 closed; ≠ MIR-free / ≠ delete blob / ≠ emit cutover.

#[test]
fn long_mir_free_native_metadata_wire_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-rt/src/abi.rs",
        "crates/syx-rt/src/wave_b_horizon.rs",
        "docs/lang/long-mir-free-native-metadata-wire.md",
        "docs/lang/long-mir-free-native-metadata-deepen.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Long native-metadata wire requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleLongHorizonNativeMetadataWireFirstSliceInventory")
            && mir.contains("mir_module_long_horizon_native_metadata_wire_first_slice_inventory"),
        "syx-mir must expose MirModuleLongHorizonNativeMetadataWireFirstSliceInventory (W412)"
    );
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    assert!(
        abi.contains("syx_rt_install_metadata_sidecar")
            && abi.contains("NATIVE_METADATA_ALTERNATE_ENTRY"),
        "syx-rt must expose syx_rt_install_metadata_sidecar alternate entry (W412)"
    );
    let wave_b = include_str!("../../../syx-rt/src/wave_b_horizon.rs");
    assert!(
        wave_b.contains("native_metadata_wire_first_slice_signed")
            && wave_b.contains("native_metadata_without_blob_gap_open"),
        "wave_b_horizon must expose native-metadata wire flags (W412)"
    );
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(
        !llvm.contains("@syx_metadata_sidecar =")
            && llvm.contains("syx_rt_install_empty_runtime")
            && llvm.contains("syx_rt_register_native_fn")
            && !llvm.contains("@syx_mir_blob ="),
        "llvm production emit omit (W413): must use sidecar install; no @syx_mir_blob"
    );
}

#[test]
fn long_mir_free_native_metadata_wire_spec_signed() {
    let spec = include_str!("../../../../docs/lang/long-mir-free-native-metadata-wire.md");
    assert!(
        spec.contains("ADR-756")
            && (spec.contains("native-metadata")
                || spec.contains("native metadata")
                || spec.contains("wire first-slice")),
        "spec must cite ADR-756 W412 Long native-metadata wire: missing markers"
    );
    assert!(
        spec.contains("MirModuleLongHorizonNativeMetadataWireFirstSliceInventory")
            && spec.contains("mir_module_long_horizon_native_metadata_wire_first_slice_inventory"),
        "spec must name public Long native-metadata wire inventory API"
    );
    assert!(
        spec.contains("syx_rt_install_metadata_sidecar"),
        "spec must cite syx_rt_install_metadata_sidecar C ABI"
    );
    assert!(
        spec.contains("native_metadata_wire_first_slice_signed=true")
            || spec.contains("`native_metadata_wire_first_slice_signed=true`"),
        "spec must cite native_metadata_wire_first_slice_signed=true"
    );
    assert!(
        spec.contains("native_metadata_without_blob_gap_open=false")
            || spec.contains("`native_metadata_without_blob_gap_open=false`")
            || (spec.contains("closed") && spec.contains("gap")),
        "spec must cite native_metadata_without_blob_gap_open closed"
    );
    assert!(
        spec.contains("mir_free_enabled=false") || spec.contains("`mir_free_enabled=false`"),
        "spec must cite mir_free_enabled=false"
    );
    assert!(
        spec.contains("blob_still_embedded=true")
            || spec.contains("`blob_still_embedded=true`")
            || spec.contains("@syx_mir_blob"),
        "spec must cite blob still embedded"
    );
    for line in spec.lines() {
        if line.contains("删除") && line.contains("blob") {
            let denied = line.contains("out of scope")
                || line.contains("≠")
                || line.contains("禁止")
                || line.contains("Forbidden")
                || line.contains("forbidden")
                || line.contains("Frozen")
                || line.contains("非")
                || line.contains("not this window")
                || line.contains("unmet")
                || line.contains("仍");
            assert!(
                denied,
                "affirmative MIR-free / blob delete forbidden: {line}"
            );
        }
        if (line.contains("MIR-free") || line.contains("mir_free_enabled=true"))
            && line.contains("enabled")
            && !line.contains("false")
            && !line.contains("≠")
            && !line.contains("unmet")
            && !line.contains("off")
            && !line.contains("Frozen")
            && !line.contains("out of scope")
        {
            panic!("affirmative MIR-free enable forbidden: {line}");
        }
    }
}

