//! LoadGlobal (VM+mir-run/SSA) ManagedHeap + heap_slots
//! first-slice migrate/cleanup false; RC still live; no retain.

#[test]
fn b09_objectref_loadglobal_heap_slots_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-vm/src/lib.rs",
        "crates/syx-mir-run/src/runtime.rs",
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "docs/gates/gate-b09-objectref-loadglobal-heap-slots-first-slice.md",
        "docs/gates/gate-b09-objectref-objectset-heap-slots-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 LoadGlobal heap_slots requires evidence file: {path}"
        );
    }
    let vm = include_str!("../../../syx-vm/src/lib.rs");
    assert!(
        vm.contains("Instruction::LoadGlobal")
            && !vm.contains("fn maybe_track_stack_top_objectref") /* W550 peel */
            && !vm.contains("maybe_track_stack_top_mutrefcell")
            && vm.contains("ADR-860"),
        "VM LoadGlobal must dual-track and cite ADR-860"
    );
    // LoadGlobal arm must not introduce object_ref_retain (W516 NO-GO).
    let lg = vm
        .split("Instruction::LoadGlobal")
        .nth(1)
        .and_then(|s| s.split("Instruction::StoreGlobal").next())
        .unwrap_or("");
    assert!(
        !lg.contains("object_ref_retain"),
        "LoadGlobal must not add object_ref_retain"
    );
    assert!(
        !vm.contains("objectref_rc_migrate_enabled = true")
            && !vm.contains("objectref_rc_migrate_enabled=true"),
        "must not flip objectref_rc_migrate_enabled"
    );
    let mir = include_str!("../../../syx-mir-run/src/runtime.rs");
    assert!(
        mir.contains("op::LOAD_GLOBAL")
            && mir.contains("MirOp::LoadGlobal")
            && !mir.contains("fn maybe_track_stack_top_objectref") /* W550 peel */
            && !mir.contains("maybe_track_stack_top_mutrefcell")
            && mir.contains("ADR-860"),
        "mir-run LOAD_GLOBAL + SSA LoadGlobal must dual-track and cite ADR-860"
    );
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09ObjectRefLoadGlobalHeapSlotsFirstSliceInventory")
            && rt.contains("gate_b09_objectref_loadglobal_heap_slots_first_slice_inventory")
            && rt.contains("ADR-860")
            && (rt.contains("b09_objectref_loadglobal_heap_slots_first_slice_signed: true")
                || rt.contains("b09_objectref_loadglobal_heap_slots_first_slice_signed:true")),
        "syx-rt must expose GateB09ObjectRefLoadGlobalHeapSlotsFirstSliceInventory"
    );
    assert!(
        (rt.contains("vm_loadglobal_objectref_managed_heap_registered: true")
            || rt.contains("vm_loadglobal_objectref_managed_heap_registered:true"))
            && (rt.contains("mir_run_loadglobal_objectref_managed_heap_registered: true")
                || rt.contains("mir_run_loadglobal_objectref_managed_heap_registered:true"))
            && (rt.contains("mir_run_ssa_loadglobal_objectref_managed_heap_registered: true")
                || rt.contains("mir_run_ssa_loadglobal_objectref_managed_heap_registered:true"))
            && (rt.contains("vm_mir_run_loadglobal_heap_slots_tracked: true")
                || rt.contains("vm_mir_run_loadglobal_heap_slots_tracked:true"))
            && (rt.contains("llvm_wasm_objectref_rc_still_live: true")
                || rt.contains("llvm_wasm_objectref_rc_still_live:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false"))
            && (rt.contains("storage_cleanup_enabled: false")
                || rt.contains("storage_cleanup_enabled:false"))
            && (rt.contains("unified_mir_runtime_heap_enabled: false")
                || rt.contains("unified_mir_runtime_heap_enabled:false")),
        "inventory must register LoadGlobal paths and keep migrate/cleanup/enable false + RC live"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_objectref_loadglobal_heap_slots_first_slice_inventory")
            && lib.contains("GateB09ObjectRefLoadGlobalHeapSlotsFirstSliceInventory"),
        "syx-rt lib.rs must re-export LoadGlobal inventory"
    );
}

#[test]
fn b09_objectref_loadglobal_heap_slots_first_slice_spec_signed() {
    let spec =
        include_str!("../../../../docs/gates/gate-b09-objectref-loadglobal-heap-slots-first-slice.md");
    assert!(
        spec.contains("ADR-860")
            && (spec.contains("LoadGlobal") || spec.contains("LOAD_GLOBAL"))
            && (spec.contains("SSA") || spec.contains("mir-run")),
        "spec must cite ADR-860 LoadGlobal VM+mir-run/SSA"
    );
    assert!(
        spec.contains("three-hundred-and-sixty-sixth"),
        
        "spec must cite W516 fingerprint"
    );
    assert!(
        spec.contains("GateB09ObjectRefLoadGlobalHeapSlotsFirstSliceInventory")
            && spec.contains("gate_b09_objectref_loadglobal_heap_slots_first_slice_inventory"),
        "spec must name LoadGlobal inventory API"
    );
    assert!(
        spec.contains("b09_objectref_loadglobal_heap_slots_first_slice_signed=true")
            || spec.contains("`b09_objectref_loadglobal_heap_slots_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("vm_mir_run_loadglobal_heap_slots_tracked=true")
            || spec.contains("`vm_mir_run_loadglobal_heap_slots_tracked=true`"))
            && (spec.contains("mir_run_ssa_loadglobal_objectref_managed_heap_registered=true")
                || spec.contains("`mir_run_ssa_loadglobal_objectref_managed_heap_registered=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`"))
            && (spec.contains("storage_cleanup_enabled=false")
                || spec.contains("`storage_cleanup_enabled=false`")),
        "spec must cite LoadGlobal flags and migrate/cleanup false"
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
fn b09_objectref_loadglobal_heap_slots_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_objectref_loadglobal_heap_slots_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_objectset_heap_slots_first_slice_floor_ok);
    assert!(inv.vm_loadglobal_objectref_managed_heap_registered);
    assert!(inv.mir_run_loadglobal_objectref_managed_heap_registered);
    assert!(inv.mir_run_ssa_loadglobal_objectref_managed_heap_registered);
    assert!(inv.vm_mir_run_loadglobal_heap_slots_tracked);
    assert!(inv.objectref_rc_storage_still_present);
    assert!(inv.llvm_wasm_objectref_rc_still_live);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_objectref_loadglobal_heap_slots_first_slice_signed);
    let floor = syx_rt::gate_b09_objectref_objectset_heap_slots_first_slice_inventory();
    assert!(floor.vm_mir_run_objectset_heap_slots_tracked);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(floor.b09_objectref_objectset_heap_slots_first_slice_signed);
}

#[test]
fn b09_objectref_loadglobal_heap_slots_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W516")
            && (roadmap.contains("ADR-860")
                || roadmap.contains("LoadGlobal")
                || roadmap.contains("LOAD_GLOBAL")),
        "roadmap must cite W516 / ADR-860 LoadGlobal heap_slots"
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
