//! Long closure-async wire first-slice — CallFn / syx_rt_call_fn (W416 / ADR-760).
//! Public inventory + RT fail-closed; gap #3 remains open; ≠ MIR-free / ≠ delete sidecar.

#[test]
fn long_mir_free_closure_async_wire_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-llvm/src/lib.rs",
        "crates/syx-rt/src/abi.rs",
        "crates/syx-rt/src/wave_b_horizon.rs",
        "docs/lang/long-mir-free-closure-async-wire.md",
        "docs/lang/long-mir-free-closure-async-deepen.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Long closure-async wire requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleLongHorizonClosureAsyncWireFirstSliceInventory")
            && mir.contains("mir_module_long_horizon_closure_async_wire_first_slice_inventory"),
        "syx-mir must expose MirModuleLongHorizonClosureAsyncWireFirstSliceInventory (W416)"
    );
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(
        !llvm.contains("@syx_metadata_sidecar =")
            && llvm.contains("syx_rt_install_empty_runtime")
            && llvm.contains("syx_rt_register_native_fn")
            && llvm.contains("mir_function_needs_mir_entry")
            && !llvm.contains("@syx_mir_blob ="),
        "llvm production emit: empty+register; no sidecar/blob (post-W431; W416 historical)"
    );
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    let start = abi
        .find("fn call_fn_or_fail_closed")
        .or_else(|| abi.find("pub extern \"C\" fn syx_rt_call_fn"))
        .expect("syx_rt_call_fn / call_fn_or_fail_closed");
    let body = &abi[start..];
    let end = body
        .find("pub extern \"C\" fn syx_rt_pop_bool")
        .unwrap_or(body.len());
    let call_fn = &body[..end];
    assert!(
        call_fn.contains("no-native") && call_fn.contains("panic!"),
        "syx_rt_call_fn must fail-closed (W416)"
    );
    assert!(
        !call_fn.contains("rt.mir.call_fn"),
        "syx_rt_call_fn must not MIR-interpret fallback (W416)"
    );
    let wave_b = include_str!("../../../syx-rt/src/wave_b_horizon.rs");
    assert!(
        wave_b.contains("closure_async_wire_first_slice_signed")
            && wave_b.contains("call_fn_mir_interpret_fallback_gap_open")
            && wave_b.contains("closure_async_mir_free_gap_open"),
        "wave_b_horizon must expose wire + CallFn + gap flags (W416)"
    );
}

#[test]
fn long_mir_free_closure_async_wire_spec_signed() {
    let spec = include_str!("../../../../docs/lang/long-mir-free-closure-async-wire.md");
    assert!(
        spec.contains("ADR-760")
            && (spec.contains("CallFn")
                || spec.contains("syx_rt_call_fn")
                || spec.contains("wire")),
        "spec must cite ADR-760 W416 Long closure-async wire: missing markers"
    );
    assert!(
        spec.contains("MirModuleLongHorizonClosureAsyncWireFirstSliceInventory")
            && spec.contains("mir_module_long_horizon_closure_async_wire_first_slice_inventory"),
        "spec must name public Long closure-async wire inventory API"
    );
    assert!(
        spec.contains("closure_async_wire_first_slice_signed=true")
            || spec.contains("`closure_async_wire_first_slice_signed=true`"),
        "spec must cite closure_async_wire_first_slice_signed=true"
    );
    assert!(
        spec.contains("call_fn_mir_interpret_fallback_gap_open=false")
            || spec.contains("`call_fn_mir_interpret_fallback_gap_open=false`")
            || (spec.contains("closed") && spec.contains("call_fn")),
        "spec must cite CallFn MIR fallback gap closed"
    );
    assert!(
        spec.contains("closure_async_mir_free_gap_open=true")
            || spec.contains("`closure_async_mir_free_gap_open=true`")
            || (spec.contains("still open") && spec.contains("closure")),
        "spec must cite closure_async_mir_free_gap_open still open"
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
    assert!(
        spec.contains("mir_function_needs_mir_entry")
            || spec.contains("unchanged")
            || spec.contains("W417"),
        "spec must note mir_function_needs_mir_entry (CallFn window / W417 Await drop)"
    );
    for line in spec.lines() {
        if line.contains("gap #3")
            && (line.contains("closed") || line.contains("close"))
            && !line.contains("≠")
            && !line.contains("not")
            && !line.contains("NOT")
            && !line.contains("仍")
            && !line.contains("still")
            && !line.contains("Frozen")
            && !line.contains("unmet")
            && !line.contains("open")
            && !line.contains("Closing")
            && !line.contains("关 gap")
        {
            panic!("affirmative gap #3 close forbidden: {line}");
        }
    }
}


#[test]
fn long_mir_free_closure_async_wire_inventory_api_callable() {
    use syx_mir::{
        aot_codec_fixture_main_with_const_strs,
        mir_module_long_horizon_closure_async_wire_first_slice_inventory,
    };
    let module = aot_codec_fixture_main_with_const_strs(4);
    let inv = mir_module_long_horizon_closure_async_wire_first_slice_inventory(&module);
    assert!(inv.closure_async_wire_first_slice_signed);
    assert!(!inv.call_fn_mir_interpret_fallback_gap_open);
    assert!(inv.closure_async_mir_free_deepen_thin_signed);
    assert!(inv.closure_async_mir_free_gap_open);
    assert!(!inv.mir_free_enabled);
    assert!(!inv.blob_still_embedded);
    assert!(inv.sidecar_constant_still_embedded);
    assert!(inv.host_bridge_pure_native_gap_open);
}
