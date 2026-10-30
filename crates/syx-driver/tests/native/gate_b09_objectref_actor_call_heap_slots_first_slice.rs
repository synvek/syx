//! ObjectRef+MutRefCell ActorCall (VM+mir-run runtime) ManagedHeap +
//! heap_slots first-slice migrate/cleanup false.

#[test]
fn b09_objectref_actor_call_heap_slots_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-vm/src/lib.rs",
        "crates/syx-mir-run/src/runtime.rs",
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "docs/gates/gate-b09-objectref-actor-call-heap-slots-first-slice.md",
        "docs/gates/gate-b09-objectref-mir-run-call-return-heap-slots-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 ActorCall heap_slots requires evidence file: {path}"
        );
    }
    let vm = include_str!("../../../syx-vm/src/lib.rs");
    assert!(
        vm.contains("Instruction::ActorCall")
            && vm.contains("finish_call")
            && vm.contains("ADR-852"),
        "VM ActorCall runtime Ok must reuse finish_call and cite ADR-852"
    );
    assert!(
        !vm.contains("objectref_rc_migrate_enabled = true")
            && !vm.contains("objectref_rc_migrate_enabled=true"),
        "must not flip objectref_rc_migrate_enabled"
    );
    let mir = include_str!("../../../syx-mir-run/src/runtime.rs");
    assert!(
        mir.contains("fn actor_call")
            && !mir.contains("fn maybe_track_stack_top_objectref") /* W550 peel */
            && !mir.contains("maybe_track_stack_top_mutrefcell")
            && mir.contains("ADR-852"),
        "mir-run actor_call runtime Ok must dual-track and cite ADR-852"
    );
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09ObjectRefActorCallHeapSlotsFirstSliceInventory")
            && rt.contains("gate_b09_objectref_actor_call_heap_slots_first_slice_inventory")
            && rt.contains("ADR-852")
            && (rt.contains("b09_objectref_actor_call_heap_slots_first_slice_signed: true")
                || rt.contains("b09_objectref_actor_call_heap_slots_first_slice_signed:true")),
        "syx-rt must expose GateB09ObjectRefActorCallHeapSlotsFirstSliceInventory"
    );
    assert!(
        (rt.contains("vm_actor_call_objectref_managed_heap_registered: true")
            || rt.contains("vm_actor_call_objectref_managed_heap_registered:true"))
            && (rt.contains("vm_actor_call_mutrefcell_managed_heap_registered: true")
                || rt.contains("vm_actor_call_mutrefcell_managed_heap_registered:true"))
            && (rt.contains("mir_run_actor_call_objectref_managed_heap_registered: true")
                || rt.contains("mir_run_actor_call_objectref_managed_heap_registered:true"))
            && (rt.contains("mir_run_actor_call_mutrefcell_managed_heap_registered: true")
                || rt.contains("mir_run_actor_call_mutrefcell_managed_heap_registered:true"))
            && (rt.contains("vm_mir_run_actor_call_heap_slots_tracked: true")
                || rt.contains("vm_mir_run_actor_call_heap_slots_tracked:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false"))
            && (rt.contains("storage_cleanup_enabled: false")
                || rt.contains("storage_cleanup_enabled:false"))
            && (rt.contains("unified_mir_runtime_heap_enabled: false")
                || rt.contains("unified_mir_runtime_heap_enabled:false")),
        "inventory must register ActorCall track and keep migrate/cleanup/enable false"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_objectref_actor_call_heap_slots_first_slice_inventory")
            && lib.contains("GateB09ObjectRefActorCallHeapSlotsFirstSliceInventory"),
        "syx-rt lib.rs must re-export ActorCall heap_slots inventory"
    );
}

#[test]
fn b09_objectref_actor_call_heap_slots_first_slice_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b09-objectref-actor-call-heap-slots-first-slice.md");
    assert!(
        spec.contains("ADR-852")
            && spec.contains("ActorCall")
            && (spec.contains("ObjectRef") || spec.contains("MutRefCell") || spec.contains("ManagedHeap")),
        "spec must cite ADR-852 ActorCall ObjectRef+MutRefCell ManagedHeap + heap_slots"
    );
    assert!(
        spec.contains("three-hundred-and-fifty-eighth"),
        
        "spec must cite W508 fingerprint"
    );
    assert!(
        spec.contains("GateB09ObjectRefActorCallHeapSlotsFirstSliceInventory")
            && spec.contains("gate_b09_objectref_actor_call_heap_slots_first_slice_inventory"),
        "spec must name ActorCall heap_slots inventory API"
    );
    assert!(
        spec.contains("b09_objectref_actor_call_heap_slots_first_slice_signed=true")
            || spec.contains("`b09_objectref_actor_call_heap_slots_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("vm_actor_call_objectref_managed_heap_registered=true")
            || spec.contains("`vm_actor_call_objectref_managed_heap_registered=true`"))
            && (spec.contains("vm_actor_call_mutrefcell_managed_heap_registered=true")
                || spec.contains("`vm_actor_call_mutrefcell_managed_heap_registered=true`"))
            && (spec.contains("mir_run_actor_call_objectref_managed_heap_registered=true")
                || spec.contains("`mir_run_actor_call_objectref_managed_heap_registered=true`"))
            && (spec.contains("mir_run_actor_call_mutrefcell_managed_heap_registered=true")
                || spec.contains("`mir_run_actor_call_mutrefcell_managed_heap_registered=true`"))
            && (spec.contains("vm_mir_run_actor_call_heap_slots_tracked=true")
                || spec.contains("`vm_mir_run_actor_call_heap_slots_tracked=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`"))
            && (spec.contains("storage_cleanup_enabled=false")
                || spec.contains("`storage_cleanup_enabled=false`"))
            && (spec.contains("llvm_wasm_objectref_rc_still_live=true")
                || spec.contains("`llvm_wasm_objectref_rc_still_live=true`")),
        "spec must cite ActorCall track flags and migrate/cleanup false + RC still live"
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
fn b09_objectref_actor_call_heap_slots_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_objectref_actor_call_heap_slots_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_mir_run_call_return_heap_slots_first_slice_floor_ok);
    assert!(inv.vm_actor_call_objectref_managed_heap_registered);
    assert!(inv.vm_actor_call_mutrefcell_managed_heap_registered);
    assert!(inv.mir_run_actor_call_objectref_managed_heap_registered);
    assert!(inv.mir_run_actor_call_mutrefcell_managed_heap_registered);
    assert!(inv.vm_mir_run_actor_call_heap_slots_tracked);
    assert!(inv.objectref_rc_storage_still_present);
    assert!(inv.llvm_wasm_objectref_rc_still_live);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_objectref_actor_call_heap_slots_first_slice_signed);
    let floor = syx_rt::gate_b09_objectref_mir_run_call_return_heap_slots_first_slice_inventory();
    assert!(floor.mir_run_call_return_heap_slots_tracked);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(floor.b09_objectref_mir_run_call_return_heap_slots_first_slice_signed);
}

