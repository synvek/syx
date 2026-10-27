//! Long sidecar-omit wire first-slice (W428 / ADR-772).
//! Public Empty-pool install ABI + inventory + wave_b wire signed; sidecar still
//! embedded; ≠ emit omit / ≠ MIR-free / ≠ Register-only full / ≠ delete sidecar.

#[test]
fn long_mir_free_sidecar_omit_wire_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-llvm/src/lib.rs",
        "crates/syx-rt/src/wave_b_horizon.rs",
        "crates/syx-rt/src/abi.rs",
        "docs/lang/long-mir-free-sidecar-omit-wire.md",
        "docs/lang/long-mir-free-sidecar-omit-deepen.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Long sidecar-omit wire requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleLongHorizonSidecarOmitWireFirstSliceInventory")
            && mir.contains("mir_module_long_horizon_sidecar_omit_wire_first_slice_inventory"),
        "syx-mir must expose MirModuleLongHorizonSidecarOmitWireFirstSliceInventory (W428)"
    );
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(
        !llvm.contains("@syx_metadata_sidecar =")
            && llvm.contains("syx_rt_install_empty_runtime")
            && llvm.contains("syx_rt_register_native_fn")
            && !llvm.contains("@syx_mir_blob ="),
        "llvm production emit: empty install; no sidecar/blob (post-W431; W428 historical)"
    );
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    assert!(
        abi.contains("syx_rt_install_empty_runtime")
            && abi.contains("install_runtime_empty")
            && abi.contains("syx_rt_install_metadata_sidecar"),
        "abi must expose empty-pool install and keep sidecar install (W428)"
    );
    let wave_b = include_str!("../../../syx-rt/src/wave_b_horizon.rs");
    assert!(
        wave_b.contains("sidecar_omit_wire_first_slice_signed")
            && wave_b.contains("empty_pool_init_gap_open")
            && wave_b.contains("sidecar_constant_still_embedded"),
        "wave_b_horizon must expose wire signed + empty_pool gap + sidecar embedded (W428)"
    );
    assert!(
        wave_b.contains("sidecar_omit_wire_first_slice_signed: true")
            || wave_b.contains("sidecar_omit_wire_first_slice_signed:true"),
        "wave_b_horizon must set sidecar_omit_wire_first_slice_signed=true (W428)"
    );
    assert!(
        wave_b.contains("empty_pool_init_gap_open: false")
            || wave_b.contains("empty_pool_init_gap_open:false"),
        "wave_b_horizon must set empty_pool_init_gap_open=false (W428)"
    );
    assert!(
        wave_b.contains("sidecar_constant_still_embedded: false")
            || wave_b.contains("sidecar_constant_still_embedded:false"),
        "wave_b live after W431: sidecar_constant_still_embedded=false (W428 historical inventory still true)"
    );
}

#[test]
fn long_mir_free_sidecar_omit_wire_spec_signed() {
    let spec = include_str!("../../../../docs/lang/long-mir-free-sidecar-omit-wire.md");
    assert!(
        spec.contains("ADR-772")
            && (spec.contains("sidecar-omit") || spec.contains("sidecar_omit")),
        "spec must cite ADR-772 W428 Long sidecar-omit wire: missing markers"
    );
    assert!(
        spec.contains("MirModuleLongHorizonSidecarOmitWireFirstSliceInventory")
            && spec.contains("mir_module_long_horizon_sidecar_omit_wire_first_slice_inventory"),
        "spec must name public Long sidecar-omit wire inventory API"
    );
    assert!(
        spec.contains("sidecar_omit_wire_first_slice_signed=true")
            || spec.contains("`sidecar_omit_wire_first_slice_signed=true`"),
        "spec must cite sidecar_omit_wire_first_slice_signed=true"
    );
    assert!(
        spec.contains("empty_pool_init_gap_open=false")
            || spec.contains("`empty_pool_init_gap_open=false`"),
        "spec must cite empty_pool_init_gap_open=false"
    );
    assert!(
        spec.contains("syx_rt_install_empty_runtime")
            || spec.contains("`syx_rt_install_empty_runtime`"),
        "spec must cite syx_rt_install_empty_runtime"
    );
    assert!(
        spec.contains("sidecar_constant_still_embedded=true")
            || spec.contains("`sidecar_constant_still_embedded=true`")
            || (spec.contains("still embedded") && spec.contains("sidecar")),
        "spec must cite sidecar_constant_still_embedded still embedded"
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
            && !line.contains("Flipping")
            && !line.contains("Deleting")
        {
            panic!("affirmative MIR-free claim forbidden: {line}");
        }
        if (line.contains("sidecar") && line.contains("deleted") && line.contains("met"))
            || (line.contains("sidecar_constant_still_embedded=false")
                && !line.contains("≠")
                && !line.contains("Frozen")
                && !line.contains("not"))
        {
            panic!("sidecar delete claim forbidden this window: {line}");
        }
        if line.contains("emit omit")
            && line.contains("met")
            && !line.contains("≠")
            && !line.contains("partial")
            && !line.contains("out of scope")
            && !line.contains("W413")
        {
            panic!("affirmative emit omit claim forbidden this window: {line}");
        }
    }
}

#[test]
fn long_mir_free_sidecar_omit_wire_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W428")
            && (roadmap.contains("sidecar-omit")
                || roadmap.contains("sidecar omit")
                || roadmap.contains("ADR-772")
                || roadmap.contains("sidecar_omit")
                || roadmap.contains("Empty-pool")
                || roadmap.contains("empty-pool")),
        "roadmap must cite W428 Long sidecar-omit wire"
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
        "Wave A exit met claim must remain after W428"
    );
    assert!(
        roadmap.contains("Wave B 退出条件已满足")
            || roadmap.contains("Wave B exit met")
            || roadmap.contains("Wave B 退出已满足"),
        "roadmap must claim Wave B exit met after W340"
    );
}

#[test]
fn long_mir_free_sidecar_omit_wire_inventory_api_callable() {
    use syx_mir::{
        aot_codec_fixture_main_with_const_strs,
        mir_module_long_horizon_sidecar_omit_deepen_thin_inventory,
        mir_module_long_horizon_sidecar_omit_wire_first_slice_inventory,
    };
    let module = aot_codec_fixture_main_with_const_strs(4);
    let inv = mir_module_long_horizon_sidecar_omit_wire_first_slice_inventory(&module);
    assert!(inv.sidecar_omit_wire_first_slice_signed);
    assert!(inv.sidecar_omit_deepen_thin_signed);
    assert!(inv.sidecar_omit_thin_signed);
    assert!(inv.sidecar_constant_still_embedded);
    assert!(!inv.empty_pool_init_gap_open);
    assert!(!inv.mir_free_enabled);
    assert!(!inv.blob_still_embedded);
    let deepen = mir_module_long_horizon_sidecar_omit_deepen_thin_inventory(&module);
    assert!(
        deepen.sidecar_constant_still_embedded,
        "W427 deepen inventory must keep historical sidecar embedded"
    );
}
