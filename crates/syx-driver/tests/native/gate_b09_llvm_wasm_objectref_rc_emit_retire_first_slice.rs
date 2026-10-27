//! LLVM·WASM ObjectRefRetain/Release emit-path retire first-slice
//! migrate/cleanup false; Value/VM/mir-run RC still live.

#[test]
fn b09_llvm_wasm_objectref_rc_emit_retire_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-llvm/src/lib.rs",
        "crates/syx-wasm-emit/src/mir_emit.rs",
        "crates/syx-rt/src/abi.rs",
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "docs/gates/gate-b09-llvm-wasm-objectref-rc-emit-retire-first-slice.md",
        "docs/gates/gate-b09-objectref-retain-catch-all-heap-slots-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 LLVM·WASM emit-path retire requires evidence file: {path}"
        );
    }
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    let retain_arm = format!("MirOp::{} =>", "ObjectRefRetain");
    let release_arm = format!("MirOp::{} =>", "ObjectRefRelease");
    assert!(
        llvm.contains("ADR-855")
            && !llvm.contains(&retain_arm)
            && !llvm.contains(&release_arm)
            && !llvm.contains("out.push_str(\"  call void @syx_rt_object_ref_retain()\\n\")")
            && !llvm.contains("out.push_str(\"  call void @syx_rt_object_ref_release()\\n\")")
            && !llvm.contains("out.push_str(\"declare void @syx_rt_object_ref_retain()\")")
            && !llvm.contains("out.push_str(\"declare void @syx_rt_object_ref_release()\")")
            && !llvm.contains("out.push_str(\"declare void @syx_rt_object_ref_wrap()\")")
            && !llvm.contains("out.push_str(\"  call void @syx_rt_object_ref_wrap()\\n\")"),
        "LLVM ObjectRefRetain/Release MirOp arms must be gone (W544); emit/declare remain retired; cite ADR-855"
    );
    assert!(
        !llvm.contains("objectref_rc_migrate_enabled = true")
            && !llvm.contains("objectref_rc_migrate_enabled=true"),
        "must not flip objectref_rc_migrate_enabled"
    );
    let wasm = include_str!("../../../syx-wasm-emit/src/mir_emit.rs");
    assert!(
        wasm.contains("ADR-855")
            && !wasm.contains("aot_import(aot::Import::ObjectRefRetain)")
            && !wasm.contains("aot_import(aot::Import::ObjectRefRelease)"),
        "WASM mir_emit must not Call ObjectRefRetain/Release AOT imports"
    );
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    let abi_deleted = !abi.contains("pub extern \"C\" fn syx_rt_object_ref_retain")
        && !abi.contains("pub extern \"C\" fn syx_rt_object_ref_release");
    let historical = rt.contains("rt_abi_object_ref_retain_release_still_present: true")
        || rt.contains("rt_abi_object_ref_retain_release_still_present:true");
    let opcodes_kept_or_peeled = abi.contains("OBJECT_REF_RETAIN")
        || abi.contains("OBJECT_REF_RELEASE")
        || abi.contains("ADR-885")
        || abi.contains("W541")
        || rt.contains("mir_run_object_ref_opcodes_deleted: true")
        || rt.contains("mir_run_object_ref_opcodes_deleted:true");
    assert!(
        (abi_deleted || historical) && opcodes_kept_or_peeled,
        "RT opcode retain/release ABI deleted (W530) or historical; mir-run opcodes kept or peeled (W541)"
    );
    assert!(
        rt.contains("GateB09LlvmWasmObjectrefRcEmitRetireFirstSliceInventory")
            && rt.contains("gate_b09_llvm_wasm_objectref_rc_emit_retire_first_slice_inventory")
            && rt.contains("ADR-855")
            && (rt.contains("b09_llvm_wasm_objectref_rc_emit_retire_first_slice_signed: true")
                || rt.contains("b09_llvm_wasm_objectref_rc_emit_retire_first_slice_signed:true")),
        "syx-rt must expose GateB09LlvmWasmObjectrefRcEmitRetireFirstSliceInventory"
    );
    assert!(
        (rt.contains("llvm_objectref_retain_release_call_emit_retired: true")
            || rt.contains("llvm_objectref_retain_release_call_emit_retired:true"))
            && (rt.contains("wasm_objectref_retain_release_aot_call_emit_retired: true")
                || rt.contains("wasm_objectref_retain_release_aot_call_emit_retired:true"))
            && (rt.contains("llvm_wasm_objectref_retain_release_emit_retired: true")
                || rt.contains("llvm_wasm_objectref_retain_release_emit_retired:true"))
            && (rt.contains("rt_abi_object_ref_retain_release_still_present: true")
                || rt.contains("rt_abi_object_ref_retain_release_still_present:true"))
            && (rt.contains("llvm_wasm_objectref_rc_still_live: true")
                || rt.contains("llvm_wasm_objectref_rc_still_live:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false"))
            && (rt.contains("storage_cleanup_enabled: false")
                || rt.contains("storage_cleanup_enabled:false"))
            && (rt.contains("unified_mir_runtime_heap_enabled: false")
                || rt.contains("unified_mir_runtime_heap_enabled:false")),
        "inventory must register emit retire and keep migrate/cleanup/enable false + RC live"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_llvm_wasm_objectref_rc_emit_retire_first_slice_inventory")
            && lib.contains("GateB09LlvmWasmObjectrefRcEmitRetireFirstSliceInventory"),
        "syx-rt lib.rs must re-export LLVM·WASM emit-retire inventory"
    );
}

#[test]
fn b09_llvm_wasm_objectref_rc_emit_retire_first_slice_spec_signed() {
    let spec =
        include_str!("../../../../docs/gates/gate-b09-llvm-wasm-objectref-rc-emit-retire-first-slice.md");
    assert!(
        spec.contains("ADR-855")
            && (spec.contains("emit-path") || spec.contains("emit"))
            && (spec.contains("ObjectRefRetain") || spec.contains("ObjectRef")),
        "spec must cite ADR-855 LLVM·WASM emit-path retire"
    );
    assert!(
        spec.contains("three-hundred-and-sixty-first"),
        
        "spec must cite W511 fingerprint"
    );
    assert!(
        spec.contains("GateB09LlvmWasmObjectrefRcEmitRetireFirstSliceInventory")
            && spec.contains("gate_b09_llvm_wasm_objectref_rc_emit_retire_first_slice_inventory"),
        "spec must name emit-retire inventory API"
    );
    assert!(
        spec.contains("b09_llvm_wasm_objectref_rc_emit_retire_first_slice_signed=true")
            || spec.contains("`b09_llvm_wasm_objectref_rc_emit_retire_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("llvm_objectref_retain_release_call_emit_retired=true")
            || spec.contains("`llvm_objectref_retain_release_call_emit_retired=true`"))
            && (spec.contains("wasm_objectref_retain_release_aot_call_emit_retired=true")
                || spec.contains("`wasm_objectref_retain_release_aot_call_emit_retired=true`"))
            && (spec.contains("llvm_wasm_objectref_retain_release_emit_retired=true")
                || spec.contains("`llvm_wasm_objectref_retain_release_emit_retired=true`"))
            && (spec.contains("rt_abi_object_ref_retain_release_still_present=true")
                || spec.contains("`rt_abi_object_ref_retain_release_still_present=true`"))
            && (spec.contains("llvm_wasm_objectref_rc_still_live=true")
                || spec.contains("`llvm_wasm_objectref_rc_still_live=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`"))
            && (spec.contains("storage_cleanup_enabled=false")
                || spec.contains("`storage_cleanup_enabled=false`")),
        "spec must cite emit-retire flags and migrate/cleanup false + RC still live"
    );
    assert!(
        !spec.contains("objectref_rc_migrate_enabled=true")
            && !spec.contains("`objectref_rc_migrate_enabled=true`")
            && !spec.contains("unified_mir_runtime_heap_enabled=true")
            && !spec.contains("`unified_mir_runtime_heap_enabled=true`"),
        "spec must not claim full migrate or unified heap enabled"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("unified") || spec.contains("cleanup") || spec.contains("B-09")),
        "spec Frozen must fence enable / cleanup / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_llvm_wasm_objectref_rc_emit_retire_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_llvm_wasm_objectref_rc_emit_retire_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_retain_catch_all_heap_slots_first_slice_floor_ok);
    assert!(inv.llvm_objectref_retain_release_call_emit_retired);
    assert!(inv.wasm_objectref_retain_release_aot_call_emit_retired);
    assert!(inv.llvm_wasm_objectref_retain_release_emit_retired);
    assert!(inv.rt_abi_object_ref_retain_release_still_present);
    assert!(inv.objectref_rc_storage_still_present);
    assert!(inv.llvm_wasm_objectref_rc_still_live);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_llvm_wasm_objectref_rc_emit_retire_first_slice_signed);
    let floor = syx_rt::gate_b09_objectref_retain_catch_all_heap_slots_first_slice_inventory();
    assert!(floor.vm_mir_run_objectref_retain_heap_slots_tracked);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(floor.b09_objectref_retain_catch_all_heap_slots_first_slice_signed);
}

#[test]
fn b09_llvm_wasm_objectref_rc_emit_retire_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W511")
            && (roadmap.contains("ADR-855")
                || roadmap.contains("emit-path")
                || roadmap.contains("emit")),
        "roadmap must cite W511 / ADR-855 LLVM·WASM emit-path retire"
    );
    let b02 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-02 |"))
        .expect("B-02 row");
    assert!(b02.contains("[x]"), "roadmap B-02 must remain [x]: {b02}");
    let b09 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-09 |"))
        .expect("B-09 row");
    assert!(
        b09.contains("[x]") || b09.contains("[~]"),
        "roadmap B-09 must be [x] (W558+) or historical [~]: {b09}"
    );
}
