//! Long closure-async Task wire first-slice (W418 / ADR-762).
//! Public inventory + RT fail-closed + llvm entry drop Task*; gap #3 remains open;
//! ≠ MIR-free / ≠ delete sidecar / ≠ true native Task.

#[test]
fn long_mir_free_closure_async_task_wire_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir/src/lib.rs",
        "crates/syx-llvm/src/lib.rs",
        "crates/syx-rt/src/llvm_stubs.rs",
        "crates/syx-rt/src/wave_b_horizon.rs",
        "docs/lang/long-mir-free-closure-async-task-wire.md",
        "docs/lang/long-mir-free-closure-async-await-wire.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "Long Task wire requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        mir.contains("MirModuleLongHorizonClosureAsyncTaskWireFirstSliceInventory")
            && mir
                .contains("mir_module_long_horizon_closure_async_task_wire_first_slice_inventory"),
        "syx-mir must expose MirModuleLongHorizonClosureAsyncTaskWireFirstSliceInventory (W418)"
    );
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(
        !llvm.contains("@syx_metadata_sidecar =")
            && llvm.contains("syx_rt_install_empty_runtime")
            && llvm.contains("syx_rt_register_native_fn")
            && llvm.contains("mir_function_needs_mir_entry")
            && !llvm.contains("@syx_mir_blob ="),
        "llvm production emit: empty+register; no sidecar/blob (post-W431; W418 historical)"
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
        .find("fn task_cancel_or_fail_closed")
        .or_else(|| stubs.find("pub extern \"C\" fn syx_rt_task_spawn"))
        .expect("syx_rt_task_* / task_*_or_fail_closed");
    let region = &stubs[start..];
    let end = region
        .find("pub extern \"C\" fn syx_rt_tcp_connect")
        .unwrap_or(region.len());
    let task_region = &region[..end];
    assert!(
        task_region.contains("no-native") && task_region.contains("panic!"),
        "Task stubs must fail-closed (W418)"
    );
    assert!(
        !task_region.contains("rt_direct_exec(") && !task_region.contains("TASK_SPAWN"),
        "Task stubs must not rt_direct_exec(TASK_*) (W418)"
    );
    let wave_b = include_str!("../../../syx-rt/src/wave_b_horizon.rs");
    assert!(
        wave_b.contains("closure_async_task_wire_first_slice_signed")
            && wave_b.contains("task_mir_interpret_fallback_gap_open")
            && wave_b.contains("closure_async_mir_free_gap_open"),
        "wave_b_horizon must expose Task wire + Task sub-gap + gap flags (W418)"
    );
}

#[test]
fn long_mir_free_closure_async_task_wire_spec_signed() {
    let spec = include_str!("../../../../docs/lang/long-mir-free-closure-async-task-wire.md");
    assert!(
        spec.contains("ADR-762")
            && (spec.contains("Task") || spec.contains("task") || spec.contains("wire")),
        "spec must cite ADR-762 W418 Long Task wire: missing markers"
    );
    assert!(
        spec.contains("MirModuleLongHorizonClosureAsyncTaskWireFirstSliceInventory")
            && spec
                .contains("mir_module_long_horizon_closure_async_task_wire_first_slice_inventory"),
        "spec must name public Long Task wire inventory API"
    );
    assert!(
        spec.contains("closure_async_task_wire_first_slice_signed=true")
            || spec.contains("`closure_async_task_wire_first_slice_signed=true`"),
        "spec must cite closure_async_task_wire_first_slice_signed=true"
    );
    assert!(
        spec.contains("task_mir_interpret_fallback_gap_open=false")
            || spec.contains("`task_mir_interpret_fallback_gap_open=false`")
            || (spec.contains("closed") && spec.contains("task")),
        "spec must cite Task MIR fallback gap closed"
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
            && (spec.contains("Task") || spec.contains("drops") || spec.contains("去掉")),
        "spec must note mir_function_needs_mir_entry drops Task*"
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
fn long_mir_free_closure_async_task_wire_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W418")
            && (roadmap.contains("Task")
                || roadmap.contains("task")
                || roadmap.contains("ADR-762")
                || roadmap.contains("wire first-slice")),
        "roadmap must cite W418 Long Task wire first-slice"
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
        "Wave A exit met claim must remain after W418"
    );
    assert!(
        roadmap.contains("Wave B 退出条件已满足")
            || roadmap.contains("Wave B exit met")
            || roadmap.contains("Wave B 退出已满足"),
        "roadmap must claim Wave B exit met after W340"
    );
}

#[test]
fn long_mir_free_closure_async_task_wire_inventory_api_callable() {
    use syx_mir::{
        aot_codec_fixture_main_with_const_strs,
        mir_module_long_horizon_closure_async_task_wire_first_slice_inventory,
    };
    let module = aot_codec_fixture_main_with_const_strs(4);
    let inv = mir_module_long_horizon_closure_async_task_wire_first_slice_inventory(&module);
    assert!(inv.closure_async_task_wire_first_slice_signed);
    assert!(!inv.task_mir_interpret_fallback_gap_open);
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
