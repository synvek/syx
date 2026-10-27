//! Long sidecar-omit Register-only wire first-slice (W429 / ADR-773).
//! Empty install + register_native_fn proven; sidecar still embedded;
//! C stub register-loop gap still open; ≠ emit omit / ≠ MIR-free / ≠ C stub align.

#[test]
fn long_mir_free_sidecar_omit_register_only_wire_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-llvm/src/lib.rs",
        "crates/syx-rt/src/wave_b_horizon.rs",
        "crates/syx-rt/src/abi.rs",
        "docs/lang/long-mir-free-sidecar-omit-register-only-wire.md",
        "docs/lang/long-mir-free-sidecar-omit-wire.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Long Register-only wire requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleLongHorizonSidecarOmitRegisterOnlyWireFirstSliceInventory")
            && mir.contains(
                "mir_module_long_horizon_sidecar_omit_register_only_wire_first_slice_inventory"
            ),
        "syx-mir must expose Register-only wire inventory (W429)"
    );
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(
        !llvm.contains("@syx_metadata_sidecar =")
            && llvm.contains("syx_rt_install_empty_runtime")
            && llvm.contains("syx_rt_register_native_fn")
            && !llvm.contains("@syx_mir_blob ="),
        "llvm production emit: empty+register; no sidecar/blob (post-W431; W429 historical)"
    );
    let c_stub_start = llvm
        .find("fn native_c_stub_source")
        .or_else(|| llvm.find("fn compile_native_c_stub"))
        .expect("native_c_stub_source / compile_native_c_stub present");
    let c_stub = &llvm[c_stub_start..];
    let c_stub_end = c_stub.find("\nfn ").unwrap_or(c_stub.len().min(4000));
    let c_stub_body = &c_stub[..c_stub_end];
    assert!(
        c_stub_body.contains("syx_rt_install_empty_runtime")
            && c_stub_body.contains("register_native_fn"),
        "C stub production emit: empty+register (post-W431; W429 historical inventory keeps gap flags)"
    );
    // Live emit after W430 includes register; historical inventory still reports gap open.
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    assert!(
        abi.contains("syx_rt_install_empty_runtime")
            && abi.contains("syx_rt_register_native_fn")
            && abi.contains("register_only_runtime_install")
            && abi.contains("syx_rt_register_only_after_empty_install_invokes_call_fn"),
        "abi must expose empty→register→call_fn Register-only path (W429)"
    );
    let wave_b = include_str!("../../../syx-rt/src/wave_b_horizon.rs");
    assert!(
        wave_b.contains("sidecar_omit_register_only_wire_first_slice_signed")
            && wave_b.contains("register_only_init_gap_open")
            && wave_b.contains("c_stub_register_loop_gap_open")
            && wave_b.contains("sidecar_constant_still_embedded"),
        "wave_b_horizon must expose Register-only signed + gaps + sidecar embedded (W429)"
    );
    assert!(
        wave_b.contains("sidecar_omit_register_only_wire_first_slice_signed: true")
            || wave_b.contains("sidecar_omit_register_only_wire_first_slice_signed:true"),
        "wave_b must set register-only wire signed=true (W429)"
    );
    assert!(
        wave_b.contains("register_only_init_gap_open: false")
            || wave_b.contains("register_only_init_gap_open:false"),
        "wave_b must set register_only_init_gap_open=false (W429)"
    );
    // Live wave_b after W430 closes c_stub gap; historical Register-only inventory keeps gap open.
}

#[test]
fn long_mir_free_sidecar_omit_register_only_wire_spec_signed() {
    let spec =
        include_str!("../../../../docs/lang/long-mir-free-sidecar-omit-register-only-wire.md");
    assert!(
        spec.contains("ADR-773")
            && (spec.contains("Register-only") || spec.contains("register_only")),
        "spec must cite ADR-773 W429 Register-only wire"
    );
    assert!(
        spec.contains("MirModuleLongHorizonSidecarOmitRegisterOnlyWireFirstSliceInventory")
            && spec.contains(
                "mir_module_long_horizon_sidecar_omit_register_only_wire_first_slice_inventory"
            ),
        "spec must name Register-only inventory API"
    );
    assert!(
        spec.contains("sidecar_omit_register_only_wire_first_slice_signed=true")
            || spec.contains("`sidecar_omit_register_only_wire_first_slice_signed=true`"),
        "spec must cite register-only signed=true"
    );
    assert!(
        spec.contains("register_only_init_gap_open=false")
            || spec.contains("`register_only_init_gap_open=false`"),
        "spec must cite register_only_init_gap_open=false"
    );
    assert!(
        spec.contains("c_stub_register_loop_gap_open=true")
            || spec.contains("`c_stub_register_loop_gap_open=true`"),
        "spec must cite c_stub_register_loop_gap_open=true"
    );
    assert!(
        spec.contains("syx_rt_install_empty_runtime") && spec.contains("syx_rt_register_native_fn"),
        "spec must cite empty install + register_native_fn sequence"
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
        if line.contains("C stub")
            && line.contains("met")
            && line.contains("align")
            && !line.contains("≠")
            && !line.contains("still")
            && !line.contains("out of scope")
            && !line.contains("Frozen")
            && !line.contains("unmet")
            && !line.contains("open")
            && !line.contains("after this window")
            && !line.contains("W430")
            && !line.contains("historical")
        {
            panic!("affirmative C stub align claim forbidden this window: {line}");
        }
    }
}

#[test]
fn long_mir_free_sidecar_omit_register_only_wire_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W429")
            && (roadmap.contains("Register-only")
                || roadmap.contains("register-only")
                || roadmap.contains("ADR-773")
                || roadmap.contains("register_only")),
        "roadmap must cite W429 Register-only wire"
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
        "Wave A exit met claim must remain after W429"
    );
    assert!(
        roadmap.contains("Wave B 退出条件已满足")
            || roadmap.contains("Wave B exit met")
            || roadmap.contains("Wave B 退出已满足"),
        "roadmap must claim Wave B exit met after W340"
    );
}

#[test]
fn long_mir_free_sidecar_omit_register_only_wire_inventory_api_callable() {
    use syx_mir::{
        aot_codec_fixture_main_with_const_strs,
        mir_module_long_horizon_sidecar_omit_register_only_wire_first_slice_inventory,
        mir_module_long_horizon_sidecar_omit_wire_first_slice_inventory,
    };
    let module = aot_codec_fixture_main_with_const_strs(4);
    let inv =
        mir_module_long_horizon_sidecar_omit_register_only_wire_first_slice_inventory(&module);
    assert!(inv.sidecar_omit_register_only_wire_first_slice_signed);
    assert!(!inv.register_only_init_gap_open);
    assert!(inv.c_stub_register_loop_gap_open);
    assert!(inv.sidecar_omit_wire_first_slice_signed);
    assert!(!inv.empty_pool_init_gap_open);
    assert!(inv.sidecar_constant_still_embedded);
    assert!(!inv.mir_free_enabled);
    assert!(!inv.blob_still_embedded);
    let wire = mir_module_long_horizon_sidecar_omit_wire_first_slice_inventory(&module);
    assert!(
        wire.sidecar_constant_still_embedded,
        "W428 Empty-pool wire inventory must keep historical sidecar embedded"
    );
}
