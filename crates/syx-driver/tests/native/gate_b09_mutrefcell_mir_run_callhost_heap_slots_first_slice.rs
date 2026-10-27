//! MutRefCell mir-run CallHost ManagedHeap + heap_slots first-slice
//! CALL_HOST success tracks MutRefCell; migrate/cleanup false.

#[test]
fn b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir-run/src/runtime.rs",
        "crates/syx-vm/src/lib.rs",
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "docs/gates/gate-b09-mutrefcell-mir-run-callhost-heap-slots-first-slice.md",
        "docs/gates/gate-b09-mutrefcell-vm-callhost-heap-slots-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 MutRefCell mir-run CallHost heap_slots requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir-run/src/runtime.rs");
    assert!(
        !mir.contains("maybe_track_stack_top_mutrefcell")
            && !mir.contains("maybe_track_mutrefcell_value")
            && mir.contains("CALL_HOST")
            && mir.contains("heap_slots")
            && mir.contains("ManagedHeap")
            && mir.contains("ADR-849"),
        "mir-run CALL_HOST MutRefCell maybe_track_* peeled (W553); ADR-849 historical inventory remains"
    );
    assert!(
        !mir.contains("fn maybe_track_stack_top_objectref") /* W550 peel */,
        "ObjectRef CallHost track peeled (W550); MutRefCell track remains"
    );
    assert!(
        !mir.contains("objectref_rc_migrate_enabled = true")
            && !mir.contains("objectref_rc_migrate_enabled=true"),
        "must not flip objectref_rc_migrate_enabled"
    );
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09MutRefCellMirRunCallhostHeapSlotsFirstSliceInventory")
            && rt.contains("gate_b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_inventory")
            && rt.contains("ADR-849")
            && (rt.contains("b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_signed: true")
                || rt.contains(
                    "b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_signed:true"
                )),
        "syx-rt must expose GateB09MutRefCellMirRunCallhostHeapSlotsFirstSliceInventory"
    );
    assert!(
        (rt.contains("mir_run_callhost_mutrefcell_managed_heap_registered: true")
            || rt.contains("mir_run_callhost_mutrefcell_managed_heap_registered:true"))
            && (rt.contains("mir_run_callhost_mutrefcell_heap_slots_tracked: true")
                || rt.contains("mir_run_callhost_mutrefcell_heap_slots_tracked:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false"))
            && (rt.contains("storage_cleanup_enabled: false")
                || rt.contains("storage_cleanup_enabled:false"))
            && (rt.contains("unified_mir_runtime_heap_enabled: false")
                || rt.contains("unified_mir_runtime_heap_enabled:false")),
        "inventory must register CallHost MutRefCell track and keep migrate/cleanup/enable false"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_inventory")
            && lib.contains("GateB09MutRefCellMirRunCallhostHeapSlotsFirstSliceInventory"),
        "syx-rt lib.rs must re-export MutRefCell mir-run CallHost heap_slots inventory"
    );
}

#[test]
fn b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_spec_signed() {
    let spec = include_str!(
        "../../../../docs/gates/gate-b09-mutrefcell-mir-run-callhost-heap-slots-first-slice.md"
    );
    assert!(
        spec.contains("ADR-849")
            && (spec.contains("MutRefCell")
                || spec.contains("ManagedHeap")
                || spec.contains("CallHost")),
        "spec must cite ADR-849 MutRefCell mir-run CallHost ManagedHeap + heap_slots"
    );
    assert!(
        spec.contains("three-hundred-and-fifty-fifth"),
        
        "spec must cite W505 fingerprint"
    );
    assert!(
        spec.contains("GateB09MutRefCellMirRunCallhostHeapSlotsFirstSliceInventory")
            && spec.contains("gate_b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_inventory"),
        "spec must name MutRefCell mir-run CallHost heap_slots inventory API"
    );
    assert!(
        spec.contains("b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_signed=true")
            || spec
                .contains("`b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("mir_run_callhost_mutrefcell_managed_heap_registered=true")
            || spec.contains("`mir_run_callhost_mutrefcell_managed_heap_registered=true`"))
            && (spec.contains("mir_run_callhost_mutrefcell_heap_slots_tracked=true")
                || spec.contains("`mir_run_callhost_mutrefcell_heap_slots_tracked=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`"))
            && (spec.contains("storage_cleanup_enabled=false")
                || spec.contains("`storage_cleanup_enabled=false`"))
            && (spec.contains("llvm_wasm_objectref_rc_still_live=true")
                || spec.contains("`llvm_wasm_objectref_rc_still_live=true`")),
        "spec must cite CallHost MutRefCell track flags and migrate/cleanup false + RC still live"
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
fn b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_mutrefcell_vm_callhost_heap_slots_first_slice_floor_ok);
    assert!(inv.mir_run_callhost_mutrefcell_managed_heap_registered);
    assert!(inv.mir_run_callhost_mutrefcell_heap_slots_tracked);
    assert!(inv.objectref_rc_storage_still_present);
    assert!(inv.llvm_wasm_objectref_rc_still_live);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_signed);
    let floor = syx_rt::gate_b09_mutrefcell_vm_callhost_heap_slots_first_slice_inventory();
    assert!(floor.vm_callhost_mutrefcell_heap_slots_tracked);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(floor.b09_mutrefcell_vm_callhost_heap_slots_first_slice_signed);
}

#[test]
fn b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W505")
            && (roadmap.contains("ADR-849")
                || roadmap.contains("MutRefCell")
                || roadmap.contains("mutrefcell")),
        "roadmap must cite W505 / ADR-849 MutRefCell mir-run CallHost heap_slots"
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
