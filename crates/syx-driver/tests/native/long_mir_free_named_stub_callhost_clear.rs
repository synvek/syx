//! Long named HostFn stub CALL_HOST clear first-slice (W433 / ADR-777).
//! Four production stubs → rt_call_host_native; ≠ delete flag / ≠ TLS raise / ≠ B-05 [x].

#[test]
fn long_mir_free_named_stub_callhost_clear_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-rt/src/llvm_stubs.rs",
        "crates/syx-rt/src/wave_b_horizon.rs",
        "crates/syx-rt/src/abi.rs",
        "docs/lang/long-mir-free-named-stub-callhost-clear.md",
        "docs/lang/long-mir-free-enabled.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Long named stub CALL_HOST clear requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleLongHorizonNamedHostFnStubCallHostClearFirstSliceInventory")
            && mir.contains(
                "mir_module_long_horizon_named_hostfn_stub_call_host_clear_first_slice_inventory"
            ),
        "syx-mir must expose named stub CALL_HOST clear inventory (W433)"
    );
    let stubs = include_str!("../../../syx-rt/src/llvm_stubs.rs");
    for sym in [
        "syx_rt_json_decode_aot",
        "syx_rt_json_encode_aot",
        "syx_rt_tcp_connect",
        "syx_rt_tls_connect",
    ] {
        let start = stubs
            .find(&format!("pub extern \"C\" fn {sym}("))
            .unwrap_or_else(|| panic!("{sym} missing"));
        let body_start = stubs[start..]
            .find('{')
            .map(|i| start + i)
            .expect("body start");
        let body_end = stubs[body_start..]
            .find('}')
            .map(|i| body_start + i + 1)
            .expect("body end");
        let body = &stubs[body_start..body_end];
        assert!(
            body.contains("rt_call_host_native") && !body.contains("rt_direct_exec"),
            "{sym} must use rt_call_host_native without rt_direct_exec (W433): {body}"
        );
        assert!(
            !body.contains("CALL_HOST"),
            "{sym} body must not mention CALL_HOST (W433): {body}"
        );
    }
    let wave_b = include_str!("../../../syx-rt/src/wave_b_horizon.rs");
    assert!(
        wave_b.contains("named_hostfn_stub_call_host_clear_signed: true")
            || wave_b.contains("named_hostfn_stub_call_host_clear_signed:true"),
        "wave_b must set named_hostfn_stub_call_host_clear_signed=true (W433)"
    );
    assert!(
        wave_b.contains("named_hostfn_stub_call_host_gap_open: false")
            || wave_b.contains("named_hostfn_stub_call_host_gap_open:false"),
        "wave_b must set named_hostfn_stub_call_host_gap_open=false (W433)"
    );
}

#[test]
fn long_mir_free_named_stub_callhost_clear_spec_signed() {
    let spec = include_str!("../../../../docs/lang/long-mir-free-named-stub-callhost-clear.md");
    assert!(
        spec.contains("ADR-777")
            && (spec.contains("named HostFn stub CALL_HOST clear")
                || spec.contains("named_hostfn_stub_call_host_clear")),
        "spec must cite ADR-777 W433 named stub CALL_HOST clear"
    );
    assert!(
        spec.contains("MirModuleLongHorizonNamedHostFnStubCallHostClearFirstSliceInventory")
            && spec.contains(
                "mir_module_long_horizon_named_hostfn_stub_call_host_clear_first_slice_inventory"
            ),
        "spec must name named stub clear inventory API"
    );
    assert!(
        spec.contains("named_hostfn_stub_call_host_clear_signed=true")
            || spec.contains("`named_hostfn_stub_call_host_clear_signed=true`"),
        "spec must cite named_hostfn_stub_call_host_clear_signed=true"
    );
    assert!(
        spec.contains("named_hostfn_stub_call_host_gap_open=false")
            || spec.contains("`named_hostfn_stub_call_host_gap_open=false`"),
        "spec must cite named_hostfn_stub_call_host_gap_open=false"
    );
    assert!(
        spec.contains("delete_syx_mir_blob_enabled=false")
            || spec.contains("`delete_syx_mir_blob_enabled=false`"),
        "spec must cite delete_syx_mir_blob_enabled=false"
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
fn long_mir_free_named_stub_callhost_clear_inventory_api_callable() {
    use syx_mir::{
        aot_codec_fixture_main_with_const_strs,
        mir_module_long_horizon_mir_free_first_slice_inventory,
        mir_module_long_horizon_named_hostfn_stub_call_host_clear_first_slice_inventory,
    };
    let module = aot_codec_fixture_main_with_const_strs(4);
    let inv =
        mir_module_long_horizon_named_hostfn_stub_call_host_clear_first_slice_inventory(&module);
    assert!(inv.named_hostfn_stub_call_host_clear_signed);
    assert!(!inv.named_hostfn_stub_call_host_gap_open);
    assert!(inv.mir_free_enabled);
    assert!(!inv.delete_syx_mir_blob_enabled);
    assert!(!inv.blob_still_embedded);
    assert!(!inv.sidecar_constant_still_embedded);
    assert!(!inv.host_bridge_pure_native_gap_open);
    let mir_free = mir_module_long_horizon_mir_free_first_slice_inventory(&module);
    assert!(
        mir_free.mir_free_enabled,
        "W432 mir_free inventory must keep mir_free_enabled=true"
    );
}
