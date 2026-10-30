//! MutRefCell mir-run DictBuild ManagedHeap + heap_slots align
//! DictBuild MutRefCell tracks ManagedHeap + heap_slots; migrate/cleanup false.

#[test]
fn b09_mutrefcell_mir_run_dictbuild_align_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir-run/src/runtime.rs",
        "crates/syx-vm/src/lib.rs",
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "docs/gates/gate-b09-mutrefcell-mir-run-dictbuild-align-first-slice.md",
        "docs/gates/gate-b09-mutrefcell-vm-dictbuild-heap-slots-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 MutRefCell mir-run DictBuild align requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir-run/src/runtime.rs");
    assert!(
        !mir.contains("maybe_track_mutrefcell_value")
            && mir.contains("MutRefCell")
            && mir.contains("mut_ref_cell")
            && mir.contains("heap_slots")
            && mir.contains("ManagedHeap")
            && mir.contains("ADR-847"),
        "mir-run DictBuild MutRefCell maybe_track_* peeled (W553); ADR-847 historical inventory remains"
    );
    assert!(
        !mir.contains("objectref_rc_migrate_enabled=true")
            && !mir.contains("objectref_rc_migrate_enabled = true"),
        "mir-run must not flip migrate_enabled=true"
    );
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09MutRefCellMirRunDictbuildAlignFirstSliceInventory")
            && rt.contains("gate_b09_mutrefcell_mir_run_dictbuild_align_first_slice_inventory")
            && rt.contains("ADR-847")
            && (rt.contains("b09_mutrefcell_mir_run_dictbuild_align_first_slice_signed: true")
                || rt.contains("b09_mutrefcell_mir_run_dictbuild_align_first_slice_signed:true")),
        "syx-rt must expose GateB09MutRefCellMirRunDictbuildAlignFirstSliceInventory"
    );
    assert!(
        (rt.contains("mir_run_dictbuild_mutrefcell_managed_heap_registered: true")
            || rt.contains("mir_run_dictbuild_mutrefcell_managed_heap_registered:true"))
            && (rt.contains("mir_run_dictbuild_mutrefcell_heap_slots_tracked: true")
                || rt.contains("mir_run_dictbuild_mutrefcell_heap_slots_tracked:true"))
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
        lib.contains("gate_b09_mutrefcell_mir_run_dictbuild_align_first_slice_inventory")
            && lib.contains("GateB09MutRefCellMirRunDictbuildAlignFirstSliceInventory"),
        "syx-rt lib.rs must re-export MutRefCell mir-run DictBuild align inventory"
    );
}

#[test]
fn b09_mutrefcell_mir_run_dictbuild_align_first_slice_spec_signed() {
    let spec =
        include_str!("../../../../docs/gates/gate-b09-mutrefcell-mir-run-dictbuild-align-first-slice.md");
    assert!(
        spec.contains("ADR-847") && (spec.contains("MutRefCell") || spec.contains("ManagedHeap")),
        "spec must cite ADR-847 MutRefCell ManagedHeap + heap_slots"
    );
    assert!(
        spec.contains("three-hundred-and-fifty-third"),
        
        "spec must cite W503 fingerprint"
    );
    assert!(
        spec.contains("GateB09MutRefCellMirRunDictbuildAlignFirstSliceInventory")
            && spec.contains("gate_b09_mutrefcell_mir_run_dictbuild_align_first_slice_inventory"),
        "spec must name MutRefCell mir-run DictBuild align inventory API"
    );
    assert!(
        spec.contains("b09_mutrefcell_mir_run_dictbuild_align_first_slice_signed=true")
            || spec.contains("`b09_mutrefcell_mir_run_dictbuild_align_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("mir_run_dictbuild_mutrefcell_managed_heap_registered=true")
            || spec.contains("`mir_run_dictbuild_mutrefcell_managed_heap_registered=true`"))
            && (spec.contains("mir_run_dictbuild_mutrefcell_heap_slots_tracked=true")
                || spec.contains("`mir_run_dictbuild_mutrefcell_heap_slots_tracked=true`"))
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
fn b09_mutrefcell_mir_run_dictbuild_align_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_mutrefcell_mir_run_dictbuild_align_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_floor_ok);
    assert!(inv.mir_run_dictbuild_mutrefcell_managed_heap_registered);
    assert!(inv.mir_run_dictbuild_mutrefcell_heap_slots_tracked);
    assert!(inv.objectref_rc_storage_still_present);
    assert!(inv.llvm_wasm_objectref_rc_still_live);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_mutrefcell_mir_run_dictbuild_align_first_slice_signed);
    let floor = syx_rt::gate_b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_inventory();
    assert!(floor.vm_dictbuild_mutrefcell_heap_slots_tracked);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(floor.b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_signed);
}

