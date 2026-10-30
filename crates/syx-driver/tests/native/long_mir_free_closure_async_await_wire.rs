//! Long closure-async Await wire first-slice (W417 / ADR-761).
//! Public inventory + RT pure-native Await (no MIR opcode fallback) + llvm entry drop Await*;
//! gap #3 remains open; ≠ MIR-free / ≠ delete sidecar / ≠ full state-machine resume.

#[test]
fn long_mir_free_closure_async_await_wire_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-llvm/src/lib.rs",
        "crates/syx-rt/src/llvm_stubs.rs",
        "crates/syx-rt/src/wave_b_horizon.rs",
        "docs/lang/long-mir-free-closure-async-await-wire.md",
        "docs/lang/long-mir-free-closure-async-wire.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Long Await wire requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleLongHorizonClosureAsyncAwaitWireFirstSliceInventory")
            && mir
                .contains("mir_module_long_horizon_closure_async_await_wire_first_slice_inventory"),
        "syx-mir must expose MirModuleLongHorizonClosureAsyncAwaitWireFirstSliceInventory (W417)"
    );
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(
        !llvm.contains("@syx_metadata_sidecar =")
            && llvm.contains("syx_rt_install_empty_runtime")
            && llvm.contains("syx_rt_register_native_fn")
            && llvm.contains("mir_function_needs_mir_entry")
            && !llvm.contains("@syx_mir_blob ="),
        "llvm production emit: empty+register; no sidecar/blob (post-W431; W417 historical)"
    );
    let entry_start = llvm
        .find("pub fn mir_function_needs_mir_entry")
        .expect("mir_function_needs_mir_entry");
    let entry_body = &llvm[entry_start..];
    let entry_end = entry_body
        .find("pub fn llvm_ir_uses_mir_main_entry")
        .unwrap_or(entry_body.len());
    let entry_fn = &entry_body[..entry_end];
    assert!(
        !entry_fn.contains("AwaitPoll") && !entry_fn.contains("AwaitSuspend"),
        "mir_function_needs_mir_entry must not match Await* after W417"
    );
    assert!(
        !entry_fn.contains("TaskSpawn") && !entry_fn.contains("TaskJoin"),
        "mir_function_needs_mir_entry must not match Task* after W418"
    );
    assert!(
        !entry_fn.contains("ActorCall")
            && !entry_fn.contains("ActorSend")
            && !entry_fn.contains("ActorRegister"),
        "mir_function_needs_mir_entry must not match Actor* after W419"
    );
    let stubs = include_str!("../../../syx-rt/src/llvm_stubs.rs");
    let start = stubs
        .find("fn await_poll_native_wire")
        .or_else(|| stubs.find("pub extern \"C\" fn syx_rt_await_poll"))
        .expect("syx_rt_await_poll / await_poll_native_wire");
    let region = &stubs[start..];
    let end = region
        .find("pub extern \"C\" fn syx_rt_begin_try")
        .unwrap_or(region.len());
    let await_region = &region[..end];
    assert!(
        await_region.contains("rt_await_poll_native")
            && await_region.contains("rt_await_suspend_native"),
        "Await stubs must call pure-native wire (W417+)"
    );
    assert!(
        !await_region.contains("rt_direct_exec(") && !await_region.contains("AWAIT_POLL"),
        "Await stubs must not rt_direct_exec(AWAIT_*) (W417)"
    );
    assert!(
        !await_region.contains("panic!"),
        "Await ready-path must not fail-closed abort"
    );
    let wave_b = include_str!("../../../syx-rt/src/wave_b_horizon.rs");
    assert!(
        wave_b.contains("closure_async_await_wire_first_slice_signed")
            && wave_b.contains("await_mir_interpret_fallback_gap_open")
            && wave_b.contains("closure_async_mir_free_gap_open"),
        "wave_b_horizon must expose Await wire + Await sub-gap + gap flags (W417)"
    );
}

#[test]
fn long_mir_free_closure_async_await_wire_spec_signed() {
    let spec = include_str!("../../../../docs/lang/long-mir-free-closure-async-await-wire.md");
    assert!(
        spec.contains("ADR-761")
            && (spec.contains("Await") || spec.contains("await") || spec.contains("wire")),
        "spec must cite ADR-761 W417 Long Await wire: missing markers"
    );
    assert!(
        spec.contains("MirModuleLongHorizonClosureAsyncAwaitWireFirstSliceInventory")
            && spec
                .contains("mir_module_long_horizon_closure_async_await_wire_first_slice_inventory"),
        "spec must name public Long Await wire inventory API"
    );
    assert!(
        spec.contains("closure_async_await_wire_first_slice_signed=true")
            || spec.contains("`closure_async_await_wire_first_slice_signed=true`"),
        "spec must cite closure_async_await_wire_first_slice_signed=true"
    );
    assert!(
        spec.contains("await_mir_interpret_fallback_gap_open=false")
            || spec.contains("`await_mir_interpret_fallback_gap_open=false`")
            || (spec.contains("closed") && spec.contains("await")),
        "spec must cite Await MIR fallback gap closed"
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
            && (spec.contains("Await") || spec.contains("drops") || spec.contains("去掉")),
        "spec must note mir_function_needs_mir_entry drops Await*"
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
fn long_mir_free_closure_async_await_wire_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W417")
            && (roadmap.contains("Await")
                || roadmap.contains("await")
                || roadmap.contains("ADR-761")
                || roadmap.contains("wire first-slice")),
        "roadmap must cite W417 Long Await wire first-slice"
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
        "Wave A exit met claim must remain after W417"
    );
    assert!(
        roadmap.contains("Wave B 退出条件已满足")
            || roadmap.contains("Wave B exit met")
            || roadmap.contains("Wave B 退出已满足"),
        "roadmap must claim Wave B exit met after W340"
    );
}

#[test]
fn long_mir_free_closure_async_await_wire_inventory_api_callable() {
    use syx_mir::{
        aot_codec_fixture_main_with_const_strs,
        mir_module_long_horizon_closure_async_await_wire_first_slice_inventory,
    };
    let module = aot_codec_fixture_main_with_const_strs(4);
    let inv = mir_module_long_horizon_closure_async_await_wire_first_slice_inventory(&module);
    assert!(inv.closure_async_await_wire_first_slice_signed);
    assert!(!inv.await_mir_interpret_fallback_gap_open);
    assert!(inv.closure_async_wire_first_slice_signed);
    assert!(!inv.call_fn_mir_interpret_fallback_gap_open);
    assert!(inv.closure_async_mir_free_deepen_thin_signed);
    assert!(inv.closure_async_mir_free_gap_open);
    assert!(!inv.mir_free_enabled);
    assert!(!inv.blob_still_embedded);
    assert!(inv.sidecar_constant_still_embedded);
    assert!(inv.host_bridge_pure_native_gap_open);
}
