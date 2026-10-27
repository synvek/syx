//! Long host-bridge pure-native gap close (W425 / ADR-769).
//! Public inventory + wave_b gap closed; sidecar still embedded;
//! ≠ MIR-free / ≠ delete sidecar / ≠ named stub clear.

#[test]
fn long_mir_free_host_bridge_gap_close_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-llvm/src/lib.rs",
        "crates/syx-rt/src/wave_b_horizon.rs",
        "crates/syx-rt/src/abi.rs",
        "crates/syx-rt/src/llvm_stubs.rs",
        "docs/lang/long-mir-free-host-bridge-gap-close.md",
        "docs/lang/long-mir-free-host-bridge-callhost-wire.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Long host-bridge gap close requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleLongHorizonHostBridgePureNativeGapCloseInventory")
            && mir.contains("mir_module_long_horizon_host_bridge_pure_native_gap_close_inventory"),
        "syx-mir must expose MirModuleLongHorizonHostBridgePureNativeGapCloseInventory (W425)"
    );
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(
        !llvm.contains("@syx_metadata_sidecar =")
            && llvm.contains("syx_rt_install_empty_runtime")
            && llvm.contains("syx_rt_register_native_fn")
            && !llvm.contains("@syx_mir_blob ="),
        "llvm production emit: empty install; no sidecar/blob (post-W431; W425 historical)"
    );
    // Evidence nail: primary Print/CallHost stubs still leave rt_direct_exec (W423/W424).
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    let stubs = include_str!("../../../syx-rt/src/llvm_stubs.rs");
    for (src, name) in [
        (stubs, "pub extern \"C\" fn syx_rt_print("),
        (stubs, "pub extern \"C\" fn syx_rt_print_aot("),
        (abi, "pub extern \"C\" fn syx_rt_call_host("),
        (stubs, "pub extern \"C\" fn syx_rt_call_host_aot("),
    ] {
        let start = src.find(name).unwrap_or_else(|| panic!("{name}"));
        let body_start = src[start..].find('{').map(|i| start + i).expect("fn body");
        let body_end = src[body_start..]
            .find('}')
            .map(|i| body_start + i + 1)
            .expect("fn body end");
        let body = &src[body_start..body_end];
        assert!(
            !body.contains("rt_direct_exec"),
            "{name} body must remain free of rt_direct_exec (W425 evidence nail)"
        );
    }
    let wave_b = include_str!("../../../syx-rt/src/wave_b_horizon.rs");
    assert!(
        wave_b.contains("host_bridge_pure_native_gap_close_signed")
            && wave_b.contains("host_bridge_pure_native_gap_open"),
        "wave_b_horizon must expose gap-close signed + host gap flags (W425)"
    );
    assert!(
        wave_b.contains("host_bridge_pure_native_gap_close_signed: true")
            || wave_b.contains("host_bridge_pure_native_gap_close_signed:true"),
        "wave_b_horizon must set host_bridge_pure_native_gap_close_signed=true (W425)"
    );
    assert!(
        wave_b.contains("host_bridge_pure_native_gap_open: false")
            || wave_b.contains("host_bridge_pure_native_gap_open:false"),
        "wave_b_horizon current horizon must set host_bridge_pure_native_gap_open=false (W425)"
    );
}

#[test]
fn long_mir_free_host_bridge_gap_close_spec_signed() {
    let spec = include_str!("../../../../docs/lang/long-mir-free-host-bridge-gap-close.md");
    assert!(
        spec.contains("ADR-769")
            && (spec.contains("gap close")
                || spec.contains("Gap close")
                || spec.contains("pure-native")),
        "spec must cite ADR-769 W425 Long host-bridge gap close: missing markers"
    );
    assert!(
        spec.contains("MirModuleLongHorizonHostBridgePureNativeGapCloseInventory")
            && spec.contains("mir_module_long_horizon_host_bridge_pure_native_gap_close_inventory"),
        "spec must name public Long host-bridge gap-close inventory API"
    );
    assert!(
        spec.contains("host_bridge_pure_native_gap_close_signed=true")
            || spec.contains("`host_bridge_pure_native_gap_close_signed=true`"),
        "spec must cite host_bridge_pure_native_gap_close_signed=true"
    );
    assert!(
        spec.contains("host_bridge_pure_native_gap_open=false")
            || spec.contains("`host_bridge_pure_native_gap_open=false`")
            || (spec.contains("closed") && spec.contains("host")),
        "spec must cite host_bridge_pure_native_gap_open closed"
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
    }
}

#[test]
fn long_mir_free_host_bridge_gap_close_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W425")
            && (roadmap.contains("gap close")
                || roadmap.contains("host-bridge gap")
                || roadmap.contains("ADR-769")
                || roadmap.contains("pure-native gap")),
        "roadmap must cite W425 Long host-bridge gap close"
    );
    for id in ["B-04", "B-05"] {
        let row = roadmap
            .lines()
            .find(|l| l.starts_with(&format!("| {id} |")))
            .unwrap_or_else(|| panic!("{id} row"));
        assert!(
            row.contains("| [x] |"),
            "{id} must be [x] after exit charters (ADR-790/ADR-791): {row}"
        );
    }
    let b06 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-06 |"))
        .expect("B-06 row");
    assert!(
        b06.contains("| [x] |"),
        "B-06 must be [x] after W448 exit: {b06}"
    );
    assert!(
        roadmap.contains("Wave A 退出条件已满足")
            || roadmap.contains("Wave A exit met")
            || roadmap.contains("Wave A 退出已满足"),
        "Wave A exit met claim must remain after W425"
    );
    assert!(
        roadmap.contains("Wave B 退出条件已满足")
            || roadmap.contains("Wave B exit met")
            || roadmap.contains("Wave B 退出已满足"),
        "roadmap must claim Wave B exit met after W340"
    );
}

#[test]
fn long_mir_free_host_bridge_gap_close_inventory_api_callable() {
    use syx_mir::{
        aot_codec_fixture_main_with_const_strs,
        mir_module_long_horizon_host_bridge_call_host_wire_first_slice_inventory,
        mir_module_long_horizon_host_bridge_print_wire_first_slice_inventory,
        mir_module_long_horizon_host_bridge_pure_native_gap_close_inventory,
    };
    let module = aot_codec_fixture_main_with_const_strs(4);
    let inv = mir_module_long_horizon_host_bridge_pure_native_gap_close_inventory(&module);
    assert!(inv.host_bridge_pure_native_gap_close_signed);
    assert!(!inv.host_bridge_pure_native_gap_open);
    assert!(inv.host_bridge_call_host_wire_first_slice_signed);
    assert!(!inv.call_host_mir_opcode_bridge_gap_open);
    assert!(inv.host_bridge_print_wire_first_slice_signed);
    assert!(!inv.print_mir_opcode_bridge_gap_open);
    assert!(!inv.mir_free_enabled);
    assert!(!inv.blob_still_embedded);
    assert!(inv.sidecar_constant_still_embedded);
    let callhost =
        mir_module_long_horizon_host_bridge_call_host_wire_first_slice_inventory(&module);
    let print = mir_module_long_horizon_host_bridge_print_wire_first_slice_inventory(&module);
    assert!(
        callhost.host_bridge_pure_native_gap_open,
        "W424 CallHost inventory must keep historical host_bridge gap open"
    );
    assert!(
        print.host_bridge_pure_native_gap_open,
        "W423 Print inventory must keep historical host_bridge gap open"
    );
}
