//! Long closure-async MIR-free deepen thin-open inventory (W415 / ADR-759).
//! Public inventory API + policy/meta; gap remains open; ≠ MIR-free / ≠ delete sidecar.

#[test]
fn long_mir_free_closure_async_deepen_thin_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-llvm/src/lib.rs",
        "crates/syx-rt/src/wave_b_horizon.rs",
        "docs/lang/long-mir-free-closure-async-deepen.md",
        "docs/lang/long-mir-free-closure-async.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Long closure-async deepen thin requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleLongHorizonClosureAsyncMirFreeDeepenThinInventory")
            && mir.contains("mir_module_long_horizon_closure_async_mir_free_deepen_thin_inventory"),
        "syx-mir must expose MirModuleLongHorizonClosureAsyncMirFreeDeepenThinInventory (W415)"
    );
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(
        !llvm.contains("@syx_metadata_sidecar =")
            && llvm.contains("syx_rt_install_empty_runtime")
            && llvm.contains("syx_rt_register_native_fn")
            && llvm.contains("mir_function_needs_mir_entry")
            && !llvm.contains("@syx_mir_blob ="),
        "llvm production emit: empty+register; no sidecar/blob (post-W431; W415 historical)"
    );
    let wave_b = include_str!("../../../syx-rt/src/wave_b_horizon.rs");
    assert!(
        wave_b.contains("closure_async_mir_free_deepen_thin_signed")
            && wave_b.contains("closure_async_mir_free_gap_open"),
        "wave_b_horizon must expose closure-async deepen flags (W415)"
    );
}

#[test]
fn long_mir_free_closure_async_deepen_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/long-mir-free-closure-async-deepen.md");
    assert!(
        spec.contains("ADR-759")
            && (spec.contains("closure-async") || spec.contains("closure async"))
            && spec.contains("deepen"),
        "spec must cite ADR-759 W415 Long closure-async deepen thin: missing markers"
    );
    assert!(
        spec.contains("MirModuleLongHorizonClosureAsyncMirFreeDeepenThinInventory")
            && spec
                .contains("mir_module_long_horizon_closure_async_mir_free_deepen_thin_inventory"),
        "spec must name public Long closure-async deepen thin inventory API"
    );
    assert!(
        spec.contains("Await")
            && spec.contains("Task")
            && spec.contains("Actor")
            && (spec.contains("CallClosure") || spec.contains("syx_rt_call_fn")),
        "spec must cite Await/Task/Actor/CallClosure prerequisites table"
    );
    assert!(
        spec.contains("closure_async_mir_free_deepen_thin_signed=true")
            || spec.contains("`closure_async_mir_free_deepen_thin_signed=true`"),
        "spec must cite closure_async_mir_free_deepen_thin_signed=true"
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
        spec.contains("mir_function_needs_mir_entry") || spec.contains("unchanged"),
        "spec must note mir_function_needs_mir_entry unchanged or cite it"
    );
    for line in spec.lines() {
        if line.contains("gap")
            && (line.contains("closed") || line.contains("close"))
            && !line.contains("≠")
            && !line.contains("not")
            && !line.contains("NOT")
            && !line.contains("仍")
            && !line.contains("still")
            && !line.contains("forbidden")
            && !line.contains("Frozen")
            && !line.contains("unmet")
            && !line.contains("open")
            && !line.contains("Closing")
            && !line.contains("closing")
            && !line.contains("Prerequisite")
            && !line.contains("prerequisite")
            && !line.contains("关 gap")
        {
            panic!("affirmative gap close forbidden: {line}");
        }
    }
}

#[test]
fn long_mir_free_closure_async_deepen_thin_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W415")
            && (roadmap.contains("closure-async")
                || roadmap.contains("closure async")
                || roadmap.contains("ADR-759")
                || roadmap.contains("Long MIR-free")),
        "roadmap must cite W415 Long closure-async deepen thin"
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
        "Wave A exit met claim must remain after W415"
    );
    assert!(
        roadmap.contains("Wave B 退出条件已满足")
            || roadmap.contains("Wave B exit met")
            || roadmap.contains("Wave B 退出已满足"),
        "roadmap must claim Wave B exit met after W340"
    );
}

#[test]
fn long_mir_free_closure_async_deepen_thin_inventory_api_callable() {
    use syx_mir::{
        aot_codec_fixture_main_with_const_strs,
        mir_module_long_horizon_closure_async_mir_free_deepen_thin_inventory,
    };
    let module = aot_codec_fixture_main_with_const_strs(4);
    let inv = mir_module_long_horizon_closure_async_mir_free_deepen_thin_inventory(&module);
    assert!(inv.closure_async_mir_free_deepen_thin_signed);
    assert!(inv.closure_async_mir_free_thin_signed);
    assert!(inv.closure_async_mir_free_gap_open);
    assert!(!inv.mir_free_enabled);
    assert!(!inv.blob_still_embedded);
    assert!(inv.sidecar_constant_still_embedded);
    assert!(inv.host_bridge_pure_native_gap_open);
}
