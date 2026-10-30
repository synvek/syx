//! Long host-bridge CallHost wire first-slice (W424 / ADR-768).
//! Public inventory + RT CallHost via rt_call_host_native; host-bridge gap still
//! open; sidecar still embedded; ≠ gap close / ≠ MIR-free / ≠ delete sidecar.

#[test]
fn long_mir_free_host_bridge_callhost_wire_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-llvm/src/lib.rs",
        "crates/syx-rt/src/wave_b_horizon.rs",
        "crates/syx-rt/src/abi.rs",
        "crates/syx-rt/src/llvm_stubs.rs",
        "docs/lang/long-mir-free-host-bridge-callhost-wire.md",
        "docs/lang/long-mir-free-host-bridge-wire.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Long host-bridge CallHost wire requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleLongHorizonHostBridgeCallHostWireFirstSliceInventory")
            && mir.contains(
                "mir_module_long_horizon_host_bridge_call_host_wire_first_slice_inventory"
            ),
        "syx-mir must expose MirModuleLongHorizonHostBridgeCallHostWireFirstSliceInventory (W424)"
    );
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(
        !llvm.contains("@syx_metadata_sidecar =")
            && llvm.contains("syx_rt_install_empty_runtime")
            && llvm.contains("syx_rt_register_native_fn")
            && !llvm.contains("@syx_mir_blob ="),
        "llvm production emit: empty install; no sidecar/blob (post-W431; W424 historical)"
    );
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    assert!(
        abi.contains("rt_call_host_native") && abi.contains("mir_call_host"),
        "abi must expose rt_call_host_native / mir_call_host (W424 CallHost wire)"
    );
    let call_host_start = abi
        .find("pub extern \"C\" fn syx_rt_call_host(")
        .expect("syx_rt_call_host");
    let call_host_body_start = abi[call_host_start..]
        .find('{')
        .map(|i| call_host_start + i)
        .expect("syx_rt_call_host body");
    let call_host_body_end = abi[call_host_body_start..]
        .find('}')
        .map(|i| call_host_body_start + i + 1)
        .expect("syx_rt_call_host body end");
    let call_host_fn = &abi[call_host_body_start..call_host_body_end];
    assert!(
        call_host_fn.contains("rt_call_host_native") && !call_host_fn.contains("rt_direct_exec"),
        "syx_rt_call_host body must call rt_call_host_native without rt_direct_exec (W424)"
    );
    let stubs = include_str!("../../../syx-rt/src/llvm_stubs.rs");
    let aot_start = stubs
        .find("pub extern \"C\" fn syx_rt_call_host_aot(")
        .expect("syx_rt_call_host_aot");
    let aot_body_start = stubs[aot_start..]
        .find('{')
        .map(|i| aot_start + i)
        .expect("syx_rt_call_host_aot body");
    let aot_body_end = stubs[aot_body_start..]
        .find('}')
        .map(|i| aot_body_start + i + 1)
        .expect("syx_rt_call_host_aot body end");
    let aot_fn = &stubs[aot_body_start..aot_body_end];
    assert!(
        aot_fn.contains("rt_call_host_native") && !aot_fn.contains("rt_direct_exec"),
        "syx_rt_call_host_aot body must call rt_call_host_native without rt_direct_exec (W424)"
    );
    let wave_b = include_str!("../../../syx-rt/src/wave_b_horizon.rs");
    assert!(
        wave_b.contains("host_bridge_call_host_wire_first_slice_signed")
            && wave_b.contains("call_host_mir_opcode_bridge_gap_open")
            && wave_b.contains("host_bridge_pure_native_gap_open"),
        "wave_b_horizon must expose CallHost wire + CallHost sub-gap + host gap flags (W424)"
    );
    assert!(
        wave_b.contains("host_bridge_call_host_wire_first_slice_signed: true")
            || wave_b.contains("host_bridge_call_host_wire_first_slice_signed:true"),
        "wave_b_horizon must set host_bridge_call_host_wire_first_slice_signed=true (W424)"
    );
    assert!(
        wave_b.contains("call_host_mir_opcode_bridge_gap_open: false")
            || wave_b.contains("call_host_mir_opcode_bridge_gap_open:false"),
        "wave_b_horizon must set call_host_mir_opcode_bridge_gap_open=false (W424)"
    );
    assert!(
        wave_b.contains("host_bridge_pure_native_gap_open: false")
            || wave_b.contains("host_bridge_pure_native_gap_open:false"),
        "wave_b_horizon must set host_bridge_pure_native_gap_open=false (current horizon W425) (W424)"
    );
}

#[test]
fn long_mir_free_host_bridge_callhost_wire_spec_signed() {
    let spec = include_str!("../../../../docs/lang/long-mir-free-host-bridge-callhost-wire.md");
    assert!(
        spec.contains("ADR-768")
            && (spec.contains("CallHost wire") || spec.contains("rt_call_host_native")),
        "spec must cite ADR-768 W424 Long host-bridge CallHost wire: missing markers"
    );
    assert!(
        spec.contains("MirModuleLongHorizonHostBridgeCallHostWireFirstSliceInventory")
            && spec.contains(
                "mir_module_long_horizon_host_bridge_call_host_wire_first_slice_inventory"
            ),
        "spec must name public Long host-bridge CallHost wire inventory API"
    );
    assert!(
        spec.contains("host_bridge_call_host_wire_first_slice_signed=true")
            || spec.contains("`host_bridge_call_host_wire_first_slice_signed=true`"),
        "spec must cite host_bridge_call_host_wire_first_slice_signed=true"
    );
    assert!(
        spec.contains("call_host_mir_opcode_bridge_gap_open=false")
            || spec.contains("`call_host_mir_opcode_bridge_gap_open=false`"),
        "spec must cite call_host_mir_opcode_bridge_gap_open=false"
    );
    assert!(
        spec.contains("host_bridge_pure_native_gap_open=true")
            || spec.contains("`host_bridge_pure_native_gap_open=true`")
            || (spec.contains("still open") && spec.contains("host")),
        "spec must cite host_bridge_pure_native_gap_open still open"
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
            && !line.contains("not enabled")
            && !line.contains("still")
            && !line.contains("flipping")
            && !line.contains("Deleting")
            && !line.contains("Hard-closing")
        {
            panic!("affirmative MIR-free claim forbidden: {line}");
        }
        if line.contains("host_bridge_pure_native_gap_open=false")
            && !line.contains("≠")
            && !line.contains("Frozen")
            && !line.contains("not")
        {
            panic!("host-bridge gap close claim forbidden this window: {line}");
        }
    }
}


#[test]
fn long_mir_free_host_bridge_callhost_wire_inventory_api_callable() {
    use syx_mir::{
        aot_codec_fixture_main_with_const_strs,
        mir_module_long_horizon_host_bridge_call_host_wire_first_slice_inventory,
        mir_module_long_horizon_host_bridge_print_wire_first_slice_inventory,
    };
    let module = aot_codec_fixture_main_with_const_strs(4);
    let inv = mir_module_long_horizon_host_bridge_call_host_wire_first_slice_inventory(&module);
    assert!(inv.host_bridge_call_host_wire_first_slice_signed);
    assert!(!inv.call_host_mir_opcode_bridge_gap_open);
    assert!(inv.host_bridge_print_wire_first_slice_signed);
    assert!(!inv.print_mir_opcode_bridge_gap_open);
    assert!(inv.host_bridge_pure_native_gap_open);
    assert!(!inv.mir_free_enabled);
    assert!(!inv.blob_still_embedded);
    assert!(inv.sidecar_constant_still_embedded);
    let print = mir_module_long_horizon_host_bridge_print_wire_first_slice_inventory(&module);
    assert!(
        print.host_bridge_pure_native_gap_open,
        "W423 Print inventory must keep historical host_bridge gap open"
    );
}
