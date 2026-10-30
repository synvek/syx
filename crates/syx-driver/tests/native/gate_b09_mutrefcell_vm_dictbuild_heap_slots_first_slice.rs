//! MutRefCell VM DictBuild ManagedHeap + heap_slots first-slice
//! DictBuild MutRefCell tracks ManagedHeap + heap_slots; migrate/cleanup false.

#[test]
fn b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-vm/src/lib.rs",
        "crates/syx-runtime-core/src/value.rs",
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "docs/gates/gate-b09-mutrefcell-vm-dictbuild-heap-slots-first-slice.md",
        "docs/gates/gate-b09-objectref-mir-run-callhost-heap-slots-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 MutRefCell VM DictBuild heap_slots requires evidence file: {path}"
        );
    }
    let vm = include_str!("../../../syx-vm/src/lib.rs");
    assert!(
        !vm.contains("maybe_track_mutrefcell_value")
            && vm.contains("MutRefCell")
            && vm.contains("mut_ref_cell")
            && vm.contains("heap_slots")
            && vm.contains("ManagedHeap")
            && vm.contains("ADR-846"),
        "VM DictBuild MutRefCell maybe_track_* peeled (W553); ADR-846 historical inventory remains"
    );
    assert!(
        !vm.contains("objectref_rc_migrate_enabled=true")
            && !vm.contains("objectref_rc_migrate_enabled = true"),
        "VM must not flip migrate_enabled=true"
    );
    let value = include_str!("../../../syx-runtime-core/src/value.rs");
    assert!(
        value.contains("mut_ref_cell_identity") && value.contains("ADR-846"),
        "runtime-core must expose mut_ref_cell_identity for dedup"
    );
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09MutRefCellVmDictbuildHeapSlotsFirstSliceInventory")
            && rt.contains("gate_b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_inventory")
            && rt.contains("ADR-846")
            && (rt.contains("b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_signed: true")
                || rt.contains("b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_signed:true")),
        "syx-rt must expose GateB09MutRefCellVmDictbuildHeapSlotsFirstSliceInventory"
    );
    assert!(
        (rt.contains("vm_dictbuild_mutrefcell_managed_heap_registered: true")
            || rt.contains("vm_dictbuild_mutrefcell_managed_heap_registered:true"))
            && (rt.contains("vm_dictbuild_mutrefcell_heap_slots_tracked: true")
                || rt.contains("vm_dictbuild_mutrefcell_heap_slots_tracked:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false"))
            && (rt.contains("storage_cleanup_enabled: false")
                || rt.contains("storage_cleanup_enabled:false"))
            && (rt.contains("unified_mir_runtime_heap_enabled: false")
                || rt.contains("unified_mir_runtime_heap_enabled:false")),
        "inventory must register MutRefCell DictBuild track and keep migrate/cleanup/enable false"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_inventory")
            && lib.contains("GateB09MutRefCellVmDictbuildHeapSlotsFirstSliceInventory"),
        "syx-rt lib.rs must re-export MutRefCell VM DictBuild heap_slots inventory"
    );
}

#[test]
fn b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_spec_signed() {
    let spec =
        include_str!("../../../../docs/gates/gate-b09-mutrefcell-vm-dictbuild-heap-slots-first-slice.md");
    assert!(
        spec.contains("ADR-846") && (spec.contains("MutRefCell") || spec.contains("ManagedHeap")),
        "spec must cite ADR-846 MutRefCell ManagedHeap + heap_slots"
    );
    assert!(
        spec.contains("three-hundred-and-fifty-second"),
        
        "spec must cite W502 fingerprint"
    );
    assert!(
        spec.contains("GateB09MutRefCellVmDictbuildHeapSlotsFirstSliceInventory")
            && spec.contains("gate_b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_inventory"),
        "spec must name MutRefCell VM DictBuild heap_slots inventory API"
    );
    assert!(
        spec.contains("b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_signed=true")
            || spec.contains("`b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("vm_dictbuild_mutrefcell_managed_heap_registered=true")
            || spec.contains("`vm_dictbuild_mutrefcell_managed_heap_registered=true`"))
            && (spec.contains("vm_dictbuild_mutrefcell_heap_slots_tracked=true")
                || spec.contains("`vm_dictbuild_mutrefcell_heap_slots_tracked=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`"))
            && (spec.contains("storage_cleanup_enabled=false")
                || spec.contains("`storage_cleanup_enabled=false`"))
            && (spec.contains("llvm_wasm_objectref_rc_still_live=true")
                || spec.contains("`llvm_wasm_objectref_rc_still_live=true`")),
        "spec must cite MutRefCell track flags and migrate/cleanup false + RC still live"
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
fn b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_mir_run_callhost_heap_slots_first_slice_floor_ok);
    assert!(inv.vm_dictbuild_mutrefcell_managed_heap_registered);
    assert!(inv.vm_dictbuild_mutrefcell_heap_slots_tracked);
    assert!(inv.objectref_rc_storage_still_present);
    assert!(inv.llvm_wasm_objectref_rc_still_live);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_signed);
    let callhost = syx_rt::gate_b09_objectref_mir_run_callhost_heap_slots_first_slice_inventory();
    assert!(callhost.mir_run_callhost_objectref_heap_slots_tracked);
    assert!(!callhost.objectref_rc_migrate_enabled);
    assert!(callhost.b09_objectref_mir_run_callhost_heap_slots_first_slice_signed);
}

