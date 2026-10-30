//! Long closure-async MIR-free gap #3 close (W420 / ADR-764).
//! Public inventory + wave_b gap closed; host-bridge still open; sidecar still embedded;
//! ≠ MIR-free / ≠ delete sidecar / ≠ host-bridge closed.

#[test]
fn long_mir_free_closure_async_gap_close_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-llvm/src/lib.rs",
        "crates/syx-rt/src/wave_b_horizon.rs",
        "docs/lang/long-mir-free-closure-async-gap-close.md",
        "docs/lang/long-mir-free-closure-async-actor-wire.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Long gap #3 close requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleLongHorizonClosureAsyncMirFreeGapCloseInventory")
            && mir.contains("mir_module_long_horizon_closure_async_mir_free_gap_close_inventory"),
        "syx-mir must expose MirModuleLongHorizonClosureAsyncMirFreeGapCloseInventory (W420)"
    );
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(
        !llvm.contains("@syx_metadata_sidecar =")
            && llvm.contains("syx_rt_install_empty_runtime")
            && llvm.contains("syx_rt_register_native_fn")
            && !llvm.contains("@syx_mir_blob ="),
        "llvm production emit: empty install; no sidecar/blob (post-W431; W420 historical)"
    );
    let wave_b = include_str!("../../../syx-rt/src/wave_b_horizon.rs");
    assert!(
        wave_b.contains("closure_async_mir_free_gap_close_signed")
            && wave_b.contains("closure_async_mir_free_gap_open"),
        "wave_b_horizon must expose gap-close signed + gap flags (W420)"
    );
    assert!(
        wave_b.contains("closure_async_mir_free_gap_open: false")
            || wave_b.contains("closure_async_mir_free_gap_open:false"),
        "wave_b_horizon current horizon must set closure_async_mir_free_gap_open=false (W420)"
    );
}

#[test]
fn long_mir_free_closure_async_gap_close_spec_signed() {
    let spec = include_str!("../../../../docs/lang/long-mir-free-closure-async-gap-close.md");
    assert!(
        spec.contains("ADR-764")
            && (spec.contains("gap #3") || spec.contains("Gap #3") || spec.contains("gap close")),
        "spec must cite ADR-764 W420 Long gap #3 close: missing markers"
    );
    assert!(
        spec.contains("MirModuleLongHorizonClosureAsyncMirFreeGapCloseInventory")
            && spec.contains("mir_module_long_horizon_closure_async_mir_free_gap_close_inventory"),
        "spec must name public Long gap-close inventory API"
    );
    assert!(
        spec.contains("closure_async_mir_free_gap_close_signed=true")
            || spec.contains("`closure_async_mir_free_gap_close_signed=true`"),
        "spec must cite closure_async_mir_free_gap_close_signed=true"
    );
    assert!(
        spec.contains("closure_async_mir_free_gap_open=false")
            || spec.contains("`closure_async_mir_free_gap_open=false`")
            || (spec.contains("closed") && spec.contains("gap")),
        "spec must cite closure_async_mir_free_gap_open closed"
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
        spec.contains("host_bridge_pure_native_gap_open=true")
            || spec.contains("`host_bridge_pure_native_gap_open=true`")
            || (spec.contains("host-bridge") && spec.contains("open")),
        "spec must cite host-bridge still open"
    );
    for line in spec.lines() {
        // Product names like "Long closure-async MIR-free thin/deepen" may sit next to **met**;
        // only forbid claiming MIR-free itself / mir_free_enabled=true as delivered.
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
fn long_mir_free_closure_async_gap_close_inventory_api_callable() {
    use syx_mir::{
        aot_codec_fixture_main_with_const_strs,
        mir_module_long_horizon_closure_async_actor_wire_first_slice_inventory,
        mir_module_long_horizon_closure_async_mir_free_gap_close_inventory,
    };
    let module = aot_codec_fixture_main_with_const_strs(4);
    let inv = mir_module_long_horizon_closure_async_mir_free_gap_close_inventory(&module);
    assert!(inv.closure_async_mir_free_gap_close_signed);
    assert!(!inv.closure_async_mir_free_gap_open);
    assert!(inv.closure_async_actor_wire_first_slice_signed);
    assert!(!inv.actor_mir_interpret_fallback_gap_open);
    assert!(inv.closure_async_task_wire_first_slice_signed);
    assert!(!inv.task_mir_interpret_fallback_gap_open);
    assert!(inv.closure_async_await_wire_first_slice_signed);
    assert!(!inv.await_mir_interpret_fallback_gap_open);
    assert!(inv.closure_async_wire_first_slice_signed);
    assert!(!inv.call_fn_mir_interpret_fallback_gap_open);
    assert!(!inv.mir_free_enabled);
    assert!(!inv.blob_still_embedded);
    assert!(inv.sidecar_constant_still_embedded);
    assert!(inv.host_bridge_pure_native_gap_open);
    let actor = mir_module_long_horizon_closure_async_actor_wire_first_slice_inventory(&module);
    assert!(
        actor.closure_async_mir_free_gap_open,
        "W419 Actor inventory must keep historical gap_open=true"
    );
}
