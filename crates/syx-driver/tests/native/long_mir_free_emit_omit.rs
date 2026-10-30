//! Long emit-omit first-slice (W413 / ADR-757).
//! llvm/C stub stop `@syx_mir_blob`; init → install_metadata_sidecar; ≠ MIR-free.

#[test]
fn long_mir_free_emit_omit_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-llvm/src/lib.rs",
        "crates/syx-rt/src/wave_b_horizon.rs",
        "docs/lang/long-mir-free-emit-omit.md",
        "docs/lang/long-mir-free-native-metadata-wire.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Long emit-omit requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleLongHorizonEmitOmitFirstSliceInventory")
            && mir.contains("mir_module_long_horizon_emit_omit_first_slice_inventory"),
        "syx-mir must expose MirModuleLongHorizonEmitOmitFirstSliceInventory (W413)"
    );
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(
        !llvm.contains("@syx_metadata_sidecar =")
            && llvm.contains("syx_rt_install_empty_runtime")
            && llvm.contains("syx_rt_register_native_fn")
            && !llvm.contains("@syx_mir_blob ="),
        "llvm production emit: empty install; no sidecar/blob (post-W431; W413 historical blob omit)"
    );
    let shim_start = llvm
        .find("fn append_aot_runtime_shim")
        .expect("append_aot_runtime_shim present");
    let shim = &llvm[shim_start..];
    let shim_end = shim.find("\nfn ").unwrap_or(shim.len().min(2500));
    let shim_body = &shim[..shim_end];
    assert!(
        !shim_body.contains("call void @syx_rt_load_mir_blob")
            && !shim_body.contains("syx_rt_load_mir_blob("),
        "llvm syx_user_init shim must not call syx_rt_load_mir_blob (W413)"
    );
    let wave_b = include_str!("../../../syx-rt/src/wave_b_horizon.rs");
    assert!(
        wave_b.contains("emit_omit_first_slice_signed")
            && wave_b.contains("sidecar_constant_still_embedded"),
        "wave_b_horizon must expose emit-omit flags (W413)"
    );
}

#[test]
fn long_mir_free_emit_omit_spec_signed() {
    let spec = include_str!("../../../../docs/lang/long-mir-free-emit-omit.md");
    assert!(
        spec.contains("ADR-757") && (spec.contains("emit-omit") || spec.contains("emit omit")),
        "spec must cite ADR-757 W413 Long emit-omit: missing markers"
    );
    assert!(
        spec.contains("MirModuleLongHorizonEmitOmitFirstSliceInventory")
            && spec.contains("mir_module_long_horizon_emit_omit_first_slice_inventory"),
        "spec must name public Long emit-omit inventory API"
    );
    assert!(
        spec.contains("syx_rt_install_metadata_sidecar")
            && (spec.contains("@syx_metadata_sidecar") || spec.contains("syx_metadata_sidecar")),
        "spec must cite sidecar install ABI / symbol"
    );
    assert!(
        spec.contains("emit_omit_first_slice_signed=true")
            || spec.contains("`emit_omit_first_slice_signed=true`"),
        "spec must cite emit_omit_first_slice_signed=true"
    );
    assert!(
        (spec.contains("llvm_emit_omit_blob_gap_open=false")
            || spec.contains("`llvm_emit_omit_blob_gap_open=false`"))
            && (spec.contains("user_init_omit_load_gap_open=false")
                || spec.contains("`user_init_omit_load_gap_open=false`")),
        "spec must cite both omit gaps closed"
    );
    assert!(
        spec.contains("mir_free_enabled=false") || spec.contains("`mir_free_enabled=false`"),
        "spec must cite mir_free_enabled=false"
    );
    assert!(
        spec.contains("sidecar_constant_still_embedded=true")
            || spec.contains("`sidecar_constant_still_embedded=true`"),
        "spec must cite sidecar constant still embedded"
    );
    for line in spec.lines() {
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

