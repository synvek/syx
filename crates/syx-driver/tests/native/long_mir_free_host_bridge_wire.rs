//! Long host-bridge Print wire first-slice (W423 / ADR-767).
//! Public inventory + RT Print pure native; host-bridge gap still open; CallHost
//! still bridged; sidecar still embedded; ≠ gap close / ≠ CallHost wire / ≠ MIR-free.

#[test]
fn long_mir_free_host_bridge_wire_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-llvm/src/lib.rs",
        "crates/syx-rt/src/wave_b_horizon.rs",
        "crates/syx-rt/src/abi.rs",
        "crates/syx-rt/src/llvm_stubs.rs",
        "docs/lang/long-mir-free-host-bridge-wire.md",
        "docs/lang/long-mir-free-host-bridge-deepen.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Long host-bridge Print wire requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleLongHorizonHostBridgePrintWireFirstSliceInventory")
            && mir.contains("mir_module_long_horizon_host_bridge_print_wire_first_slice_inventory"),
        "syx-mir must expose MirModuleLongHorizonHostBridgePrintWireFirstSliceInventory (W423)"
    );
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(
        !llvm.contains("@syx_metadata_sidecar =")
            && llvm.contains("syx_rt_install_empty_runtime")
            && llvm.contains("syx_rt_register_native_fn")
            && !llvm.contains("@syx_mir_blob ="),
        "llvm production emit: empty install; no sidecar/blob (post-W431; W423 historical)"
    );
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    assert!(
        abi.contains("rt_print_native") && abi.contains("mir_print_top"),
        "abi must expose rt_print_native / mir_print_top (W423 Print wire)"
    );
    assert!(
        abi.contains("syx_rt_call_host") && abi.contains("CALL_HOST"),
        "abi must still expose CallHost entry after Print wire (W423)"
    );
    let stubs = include_str!("../../../syx-rt/src/llvm_stubs.rs");
    let print_start = stubs
        .find("pub extern \"C\" fn syx_rt_print()")
        .expect("syx_rt_print");
    let print_aot = stubs
        .find("pub extern \"C\" fn syx_rt_print_aot()")
        .expect("syx_rt_print_aot");
    let print_body_start = stubs[print_start..]
        .find('{')
        .map(|i| print_start + i)
        .expect("syx_rt_print body");
    let print_body_end = stubs[print_body_start..]
        .find('}')
        .map(|i| print_body_start + i + 1)
        .expect("syx_rt_print body end");
    let print_fn = &stubs[print_body_start..print_body_end];
    let print_aot_body_start = stubs[print_aot..]
        .find('{')
        .map(|i| print_aot + i)
        .expect("syx_rt_print_aot body");
    let print_aot_body_end = stubs[print_aot_body_start..]
        .find('}')
        .map(|i| print_aot_body_start + i + 1)
        .expect("syx_rt_print_aot body end");
    let print_aot_fn = &stubs[print_aot_body_start..print_aot_body_end];
    assert!(
        print_fn.contains("rt_print_native") && !print_fn.contains("rt_direct_exec"),
        "syx_rt_print body must call rt_print_native without rt_direct_exec (W423)"
    );
    assert!(
        print_aot_fn.contains("rt_print_native") && !print_aot_fn.contains("rt_direct_exec"),
        "syx_rt_print_aot body must call rt_print_native without rt_direct_exec (W423)"
    );
    let wave_b = include_str!("../../../syx-rt/src/wave_b_horizon.rs");
    assert!(
        wave_b.contains("host_bridge_print_wire_first_slice_signed")
            && wave_b.contains("print_mir_opcode_bridge_gap_open")
            && wave_b.contains("host_bridge_pure_native_gap_open"),
        "wave_b_horizon must expose Print wire + Print sub-gap + host gap flags (W423)"
    );
    assert!(
        wave_b.contains("host_bridge_print_wire_first_slice_signed: true")
            || wave_b.contains("host_bridge_print_wire_first_slice_signed:true"),
        "wave_b_horizon current horizon must set host_bridge_print_wire_first_slice_signed=true (W423)"
    );
    assert!(
        wave_b.contains("print_mir_opcode_bridge_gap_open: false")
            || wave_b.contains("print_mir_opcode_bridge_gap_open:false"),
        "wave_b_horizon current horizon must set print_mir_opcode_bridge_gap_open=false (W423)"
    );
    assert!(
        wave_b.contains("host_bridge_pure_native_gap_open: false")
            || wave_b.contains("host_bridge_pure_native_gap_open:false"),
        "wave_b_horizon current horizon must set host_bridge_pure_native_gap_open=false (current horizon W425) (W423)"
    );
}

#[test]
fn long_mir_free_host_bridge_wire_spec_signed() {
    let spec = include_str!("../../../../docs/lang/long-mir-free-host-bridge-wire.md");
    assert!(
        spec.contains("ADR-767")
            && (spec.contains("Print wire") || spec.contains("rt_print_native")),
        "spec must cite ADR-767 W423 Long host-bridge Print wire: missing markers"
    );
    assert!(
        spec.contains("MirModuleLongHorizonHostBridgePrintWireFirstSliceInventory")
            && spec
                .contains("mir_module_long_horizon_host_bridge_print_wire_first_slice_inventory"),
        "spec must name public Long host-bridge Print wire inventory API"
    );
    assert!(
        spec.contains("host_bridge_print_wire_first_slice_signed=true")
            || spec.contains("`host_bridge_print_wire_first_slice_signed=true`"),
        "spec must cite host_bridge_print_wire_first_slice_signed=true"
    );
    assert!(
        spec.contains("print_mir_opcode_bridge_gap_open=false")
            || spec.contains("`print_mir_opcode_bridge_gap_open=false`"),
        "spec must cite print_mir_opcode_bridge_gap_open=false"
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
    assert!(
        spec.contains("CallHost")
            && (spec.contains("still") || spec.contains("Unchanged") || spec.contains("unmet")),
        "spec must note CallHost still bridged"
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
        if line.contains("CallHost wire")
            && line.contains("met")
            && !line.contains("≠")
            && !line.contains("unmet")
            && !line.contains("still")
            && !line.contains("Frozen")
        {
            panic!("CallHost wire met claim forbidden this window: {line}");
        }
    }
}


#[test]
fn long_mir_free_host_bridge_wire_inventory_api_callable() {
    use syx_mir::{
        aot_codec_fixture_main_with_const_strs,
        mir_module_long_horizon_host_bridge_deepen_thin_inventory,
        mir_module_long_horizon_host_bridge_print_wire_first_slice_inventory,
    };
    let module = aot_codec_fixture_main_with_const_strs(4);
    let inv = mir_module_long_horizon_host_bridge_print_wire_first_slice_inventory(&module);
    assert!(inv.host_bridge_print_wire_first_slice_signed);
    assert!(!inv.print_mir_opcode_bridge_gap_open);
    assert!(inv.host_bridge_deepen_thin_signed);
    assert!(inv.host_bridge_thin_signed);
    assert!(inv.host_bridge_pure_native_gap_open);
    assert!(inv.closure_async_mir_free_gap_close_signed);
    assert!(!inv.closure_async_mir_free_gap_open);
    assert!(!inv.mir_free_enabled);
    assert!(!inv.blob_still_embedded);
    assert!(inv.sidecar_constant_still_embedded);
    let deepen = mir_module_long_horizon_host_bridge_deepen_thin_inventory(&module);
    assert!(
        deepen.host_bridge_pure_native_gap_open,
        "W422 deepen inventory must keep historical host_bridge gap open"
    );
}
