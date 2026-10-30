//! Long sidecar-omit emit first-slice (W431 / ADR-775).
//! Production llvm/C stub stop sidecar embed; empty install + register;
//! ≠ MIR-free / ≠ B-05 [x].

#[test]
fn long_mir_free_sidecar_omit_emit_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-llvm/src/lib.rs",
        "crates/syx-rt/src/wave_b_horizon.rs",
        "docs/lang/long-mir-free-sidecar-omit-emit.md",
        "docs/lang/long-mir-free-sidecar-omit-c-stub-align-wire.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Long sidecar emit requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleLongHorizonSidecarOmitEmitFirstSliceInventory")
            && mir.contains("mir_module_long_horizon_sidecar_omit_emit_first_slice_inventory"),
        "syx-mir must expose sidecar-omit emit inventory (W431)"
    );
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(
        !llvm.contains("@syx_metadata_sidecar =")
            && llvm.contains("syx_rt_install_empty_runtime")
            && llvm.contains("syx_rt_register_native_fn")
            && !llvm.contains("@syx_mir_blob ="),
        "llvm production emit must empty+register without sidecar/blob (W431)"
    );
    let c_stub_start = llvm
        .find("fn native_c_stub_source")
        .expect("native_c_stub_source present");
    let c_stub = &llvm[c_stub_start..];
    let c_stub_end = c_stub.find("\nfn ").unwrap_or(c_stub.len().min(5000));
    let c_stub_body = &c_stub[..c_stub_end];
    assert!(
        c_stub_body.contains("syx_rt_install_empty_runtime")
            && c_stub_body.contains("register_native_fn")
            && !c_stub_body.contains("syx_metadata_sidecar"),
        "C stub must empty+register without sidecar array (W431)"
    );
    let wave_b = include_str!("../../../syx-rt/src/wave_b_horizon.rs");
    assert!(
        wave_b.contains("sidecar_omit_emit_first_slice_signed: true")
            || wave_b.contains("sidecar_omit_emit_first_slice_signed:true"),
        "wave_b must set sidecar emit signed=true (W431)"
    );
    assert!(
        wave_b.contains("sidecar_constant_still_embedded: false")
            || wave_b.contains("sidecar_constant_still_embedded:false"),
        "wave_b must set sidecar_constant_still_embedded=false (W431)"
    );
}

#[test]
fn long_mir_free_sidecar_omit_emit_spec_signed() {
    let spec = include_str!("../../../../docs/lang/long-mir-free-sidecar-omit-emit.md");
    assert!(
        spec.contains("ADR-775") && spec.contains("sidecar-omit emit"),
        "spec must cite ADR-775 W431 sidecar emit"
    );
    assert!(
        spec.contains("MirModuleLongHorizonSidecarOmitEmitFirstSliceInventory")
            && spec.contains("mir_module_long_horizon_sidecar_omit_emit_first_slice_inventory"),
        "spec must name emit inventory API"
    );
    assert!(
        spec.contains("sidecar_omit_emit_first_slice_signed=true")
            || spec.contains("`sidecar_omit_emit_first_slice_signed=true`"),
        "spec must cite emit signed=true"
    );
    assert!(
        spec.contains("sidecar_constant_still_embedded=false")
            || spec.contains("`sidecar_constant_still_embedded=false`"),
        "spec must cite sidecar_constant_still_embedded=false"
    );
    assert!(
        spec.contains("syx_rt_install_empty_runtime") && spec.contains("syx_rt_register_native_fn"),
        "spec must cite empty install + register"
    );
    assert!(
        spec.contains("mir_free_enabled=false") || spec.contains("`mir_free_enabled=false`"),
        "spec must cite mir_free_enabled=false"
    );
    for line in spec.lines() {
        if (line.contains("mir_free_enabled=true")
            || (line.contains("MIR-free")
                && !line.contains("MIR-free thin")
                && !line.contains("MIR-free deepen")
                && !line.contains("MIR-free gap")
                && !line.contains("closure-async MIR-free")
                && !line.contains("Long MIR-free readiness")))
            && (line.contains("met") || line.contains("enabled=true"))
            && !line.contains("≠")
            && !line.contains("still unmet")
            && !line.contains("unmet")
            && !line.contains("false")
            && !line.contains("Frozen")
            && !line.contains("out of scope")
            && !line.contains("flipping")
            && !line.contains("Claiming")
            && !line.contains("unless")
        {
            panic!("affirmative MIR-free claim forbidden: {line}");
        }
    }
}


#[test]
fn long_mir_free_sidecar_omit_emit_inventory_api_callable() {
    use syx_mir::{
        aot_codec_fixture_main_with_const_strs,
        mir_module_long_horizon_sidecar_omit_c_stub_align_wire_first_slice_inventory,
        mir_module_long_horizon_sidecar_omit_emit_first_slice_inventory,
    };
    let module = aot_codec_fixture_main_with_const_strs(4);
    let inv = mir_module_long_horizon_sidecar_omit_emit_first_slice_inventory(&module);
    assert!(inv.sidecar_omit_emit_first_slice_signed);
    assert!(!inv.sidecar_constant_still_embedded);
    assert!(!inv.c_stub_register_loop_gap_open);
    assert!(!inv.mir_free_enabled);
    assert!(!inv.blob_still_embedded);
    let align =
        mir_module_long_horizon_sidecar_omit_c_stub_align_wire_first_slice_inventory(&module);
    assert!(
        align.sidecar_constant_still_embedded,
        "W430 C stub align inventory must keep historical sidecar embedded"
    );
}
