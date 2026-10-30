//! Long sidecar-omit C stub align wire first-slice (W430 / ADR-774).
//! C stub gains register_native_fn after sidecar install; sidecar still embedded;
//! ≠ emit omit / ≠ MIR-free / ≠ empty-only production cutover.

#[test]
fn long_mir_free_sidecar_omit_c_stub_align_wire_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-llvm/src/lib.rs",
        "crates/syx-rt/src/wave_b_horizon.rs",
        "docs/lang/long-mir-free-sidecar-omit-c-stub-align-wire.md",
        "docs/lang/long-mir-free-sidecar-omit-register-only-wire.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Long C stub align wire requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleLongHorizonSidecarOmitCStubAlignWireFirstSliceInventory")
            && mir.contains(
                "mir_module_long_horizon_sidecar_omit_c_stub_align_wire_first_slice_inventory"
            ),
        "syx-mir must expose C stub align wire inventory (W430)"
    );
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(
        !llvm.contains("@syx_metadata_sidecar =")
            && llvm.contains("syx_rt_install_empty_runtime")
            && llvm.contains("syx_rt_register_native_fn")
            && !llvm.contains("@syx_mir_blob ="),
        "llvm production emit: empty+register; no sidecar/blob (post-W431; W430 historical)"
    );
    let c_stub_start = llvm
        .find("fn native_c_stub_source")
        .or_else(|| llvm.find("fn compile_native_c_stub"))
        .expect("native_c_stub_source / compile_native_c_stub present");
    let c_stub = &llvm[c_stub_start..];
    let c_stub_end = c_stub.find("\nfn ").unwrap_or(c_stub.len().min(5000));
    let c_stub_body = &c_stub[..c_stub_end];
    assert!(
        c_stub_body.contains("syx_rt_install_empty_runtime")
            && c_stub_body.contains("register_native_fn")
            && !c_stub_body.contains("syx_metadata_sidecar"),
        "C stub source builder must empty-install + register; no sidecar (W431)"
    );
    let wave_b = include_str!("../../../syx-rt/src/wave_b_horizon.rs");
    assert!(
        wave_b.contains("sidecar_omit_c_stub_align_wire_first_slice_signed")
            && wave_b.contains("c_stub_register_loop_gap_open"),
        "wave_b_horizon must expose C stub align signed + gap (W430)"
    );
    assert!(
        wave_b.contains("sidecar_omit_c_stub_align_wire_first_slice_signed: true")
            || wave_b.contains("sidecar_omit_c_stub_align_wire_first_slice_signed:true"),
        "wave_b must set c-stub align wire signed=true (W430)"
    );
    assert!(
        wave_b.contains("c_stub_register_loop_gap_open: false")
            || wave_b.contains("c_stub_register_loop_gap_open:false"),
        "wave_b must set c_stub_register_loop_gap_open=false (W430)"
    );
}

#[test]
fn long_mir_free_sidecar_omit_c_stub_align_wire_spec_signed() {
    let spec =
        include_str!("../../../../docs/lang/long-mir-free-sidecar-omit-c-stub-align-wire.md");
    assert!(
        spec.contains("ADR-774")
            && (spec.contains("C stub align") || spec.contains("c_stub_align")),
        "spec must cite ADR-774 W430 C stub align wire"
    );
    assert!(
        spec.contains("MirModuleLongHorizonSidecarOmitCStubAlignWireFirstSliceInventory")
            && spec.contains(
                "mir_module_long_horizon_sidecar_omit_c_stub_align_wire_first_slice_inventory"
            ),
        "spec must name C stub align inventory API"
    );
    assert!(
        spec.contains("sidecar_omit_c_stub_align_wire_first_slice_signed=true")
            || spec.contains("`sidecar_omit_c_stub_align_wire_first_slice_signed=true`"),
        "spec must cite c-stub align signed=true"
    );
    assert!(
        spec.contains("c_stub_register_loop_gap_open=false")
            || spec.contains("`c_stub_register_loop_gap_open=false`"),
        "spec must cite c_stub_register_loop_gap_open=false"
    );
    assert!(
        spec.contains("syx_rt_register_native_fn")
            && (spec.contains("syx_rt_install_metadata_sidecar")
                || spec.contains("sidecar install")),
        "spec must cite sidecar install + register_native_fn"
    );
    assert!(
        spec.contains("sidecar_constant_still_embedded=true")
            || spec.contains("`sidecar_constant_still_embedded=true`")
            || (spec.contains("still embedded") && spec.contains("sidecar")),
        "spec must cite sidecar still embedded"
    );
    assert!(
        spec.contains("mir_free_enabled=false") || spec.contains("`mir_free_enabled=false`"),
        "spec must cite mir_free_enabled=false"
    );
    for line in spec.lines() {
        if line.contains("emit omit")
            && line.contains("met")
            && !line.contains("≠")
            && !line.contains("partial")
            && !line.contains("still")
            && !line.contains("unmet")
            && !line.contains("Frozen")
            && !line.contains("out of scope")
            && !line.contains("Claiming")
        {
            panic!("affirmative emit omit claim forbidden this window: {line}");
        }
        if (line.contains("mir_free_enabled=true")
            || (line.contains("MIR-free")
                && !line.contains("MIR-free thin")
                && !line.contains("MIR-free deepen")
                && !line.contains("MIR-free gap")
                && !line.contains("closure-async MIR-free")))
            && (line.contains("met") || line.contains("enabled=true"))
            && !line.contains("≠")
            && !line.contains("still unmet")
            && !line.contains("unmet")
            && !line.contains("false")
            && !line.contains("Frozen")
            && !line.contains("out of scope")
            && !line.contains("flipping")
            && !line.contains("Flipping")
        {
            panic!("affirmative MIR-free claim forbidden: {line}");
        }
    }
}


#[test]
fn long_mir_free_sidecar_omit_c_stub_align_wire_inventory_api_callable() {
    use syx_mir::{
        aot_codec_fixture_main_with_const_strs,
        mir_module_long_horizon_sidecar_omit_c_stub_align_wire_first_slice_inventory,
        mir_module_long_horizon_sidecar_omit_register_only_wire_first_slice_inventory,
    };
    let module = aot_codec_fixture_main_with_const_strs(4);
    let inv = mir_module_long_horizon_sidecar_omit_c_stub_align_wire_first_slice_inventory(&module);
    assert!(inv.sidecar_omit_c_stub_align_wire_first_slice_signed);
    assert!(!inv.c_stub_register_loop_gap_open);
    assert!(inv.sidecar_omit_register_only_wire_first_slice_signed);
    assert!(!inv.register_only_init_gap_open);
    assert!(inv.sidecar_constant_still_embedded);
    assert!(!inv.mir_free_enabled);
    assert!(!inv.blob_still_embedded);
    let reg =
        mir_module_long_horizon_sidecar_omit_register_only_wire_first_slice_inventory(&module);
    assert!(
        reg.c_stub_register_loop_gap_open,
        "W429 Register-only inventory must keep historical c_stub gap open"
    );
}
