//! Long host-bridge deepen thin-open inventory (W422 / ADR-766).
//! Public inventory + wave_b deepen signed; host-bridge gap still open; sidecar still
//! embedded; ≠ wire / ≠ host gap close / ≠ MIR-free / ≠ delete sidecar.

#[test]
fn long_mir_free_host_bridge_deepen_thin_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-llvm/src/lib.rs",
        "crates/syx-rt/src/wave_b_horizon.rs",
        "crates/syx-rt/src/abi.rs",
        "crates/syx-rt/src/llvm_stubs.rs",
        "docs/lang/long-mir-free-host-bridge-deepen.md",
        "docs/lang/long-mir-free-host-bridge.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Long host-bridge deepen requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleLongHorizonHostBridgeDeepenThinInventory")
            && mir.contains("mir_module_long_horizon_host_bridge_deepen_thin_inventory"),
        "syx-mir must expose MirModuleLongHorizonHostBridgeDeepenThinInventory (W422)"
    );
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(
        !llvm.contains("@syx_metadata_sidecar =")
            && llvm.contains("syx_rt_install_empty_runtime")
            && llvm.contains("syx_rt_register_native_fn")
            && !llvm.contains("@syx_mir_blob ="),
        "llvm production emit: empty install; no sidecar/blob (post-W431; W422 historical)"
    );
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    assert!(
        abi.contains("syx_rt_call_host") && abi.contains("CALL_HOST"),
        "abi must still expose CallHost entry (W422 deepen charter)"
    );
    let stubs = include_str!("../../../syx-rt/src/llvm_stubs.rs");
    assert!(
        stubs.contains("syx_rt_print") && stubs.contains("rt_print_native"),
        "llvm_stubs must expose Print via rt_print_native after W423 (deepen charter ≠ CallHost wire)"
    );
    let wave_b = include_str!("../../../syx-rt/src/wave_b_horizon.rs");
    assert!(
        wave_b.contains("host_bridge_deepen_thin_signed")
            && wave_b.contains("host_bridge_pure_native_gap_open"),
        "wave_b_horizon must expose host-bridge deepen signed + gap flags (W422)"
    );
    assert!(
        wave_b.contains("host_bridge_deepen_thin_signed: true")
            || wave_b.contains("host_bridge_deepen_thin_signed:true"),
        "wave_b_horizon current horizon must set host_bridge_deepen_thin_signed=true (W422)"
    );
    assert!(
        wave_b.contains("host_bridge_pure_native_gap_open: false")
            || wave_b.contains("host_bridge_pure_native_gap_open:false"),
        "wave_b_horizon current horizon must set host_bridge_pure_native_gap_open=false (current horizon W425) (W422)"
    );
}

#[test]
fn long_mir_free_host_bridge_deepen_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/long-mir-free-host-bridge-deepen.md");
    assert!(
        spec.contains("ADR-766") && (spec.contains("deepen") || spec.contains("Prerequisites")),
        "spec must cite ADR-766 W422 Long host-bridge deepen: missing markers"
    );
    assert!(
        spec.contains("MirModuleLongHorizonHostBridgeDeepenThinInventory")
            && spec.contains("mir_module_long_horizon_host_bridge_deepen_thin_inventory"),
        "spec must name public Long host-bridge deepen inventory API"
    );
    assert!(
        spec.contains("host_bridge_deepen_thin_signed=true")
            || spec.contains("`host_bridge_deepen_thin_signed=true`"),
        "spec must cite host_bridge_deepen_thin_signed=true"
    );
    assert!(
        spec.contains("host_bridge_thin_signed=true")
            || spec.contains("`host_bridge_thin_signed=true`"),
        "spec must cite host_bridge_thin_signed=true"
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
        spec.contains("rt_direct_exec")
            && (spec.contains("PRINT") || spec.contains("CALL_HOST") || spec.contains("Print")),
        "spec must cite Print/CallHost prerequisites vs rt_direct_exec"
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
fn long_mir_free_host_bridge_deepen_thin_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W422")
            && (roadmap.contains("host-bridge deepen")
                || roadmap.contains("host bridge deepen")
                || roadmap.contains("ADR-766")
                || roadmap.contains("deepen thin")),
        "roadmap must cite W422 Long host-bridge deepen"
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
        "Wave A exit met claim must remain after W422"
    );
    assert!(
        roadmap.contains("Wave B 退出条件已满足")
            || roadmap.contains("Wave B exit met")
            || roadmap.contains("Wave B 退出已满足"),
        "roadmap must claim Wave B exit met after W340"
    );
}

#[test]
fn long_mir_free_host_bridge_deepen_thin_inventory_api_callable() {
    use syx_mir::{
        aot_codec_fixture_main_with_const_strs,
        mir_module_long_horizon_host_bridge_deepen_thin_inventory,
        mir_module_long_horizon_host_bridge_thin_inventory,
    };
    let module = aot_codec_fixture_main_with_const_strs(4);
    let inv = mir_module_long_horizon_host_bridge_deepen_thin_inventory(&module);
    assert!(inv.host_bridge_deepen_thin_signed);
    assert!(inv.host_bridge_thin_signed);
    assert!(inv.host_bridge_pure_native_gap_open);
    assert!(inv.closure_async_mir_free_gap_close_signed);
    assert!(!inv.closure_async_mir_free_gap_open);
    assert!(!inv.mir_free_enabled);
    assert!(!inv.blob_still_embedded);
    assert!(inv.sidecar_constant_still_embedded);
    let thin = mir_module_long_horizon_host_bridge_thin_inventory(&module);
    assert!(
        thin.host_bridge_pure_native_gap_open,
        "W421 thin inventory must keep historical host_bridge gap open"
    );
}
