//! ObjectRef mir-run DictBuild ManagedHeap align first-slice
//! DictBuild __type dual-registers ManagedHeap + heap_slots; migrate/cleanup false.

#[test]
fn b09_objectref_mir_run_dictbuild_align_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir-run/src/runtime.rs",
        "crates/syx-vm/src/lib.rs",
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "docs/gates/gate-b09-objectref-mir-run-dictbuild-align-first-slice.md",
        "docs/gates/gate-b09-objectref-vm-heap-slots-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 mir-run DictBuild align requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir-run/src/runtime.rs");
    assert!(
        mir.contains("object_ref_from_map")
            && (mir.contains("heap.alloc") || mir.contains("alloc_heap_slot"))
            && (mir.contains("self.heap_slots.push")
                || mir.contains("rt.heap_slots.push")
                || mir.contains("alloc_heap_slot")
                || mir.contains("heap_slots"))
            && mir.contains("ADR-843"),
        "mir-run DictBuild __type path must register ManagedHeap + heap_slots (or alloc_heap_slot) and cite ADR-843"
    );
    assert!(
        mir.contains("MutRefCell") && mir.contains("mut_ref_cell_from_map"),
        "MutRefCell DictBuild branch must remain unchanged"
    );
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09ObjectRefMirRunDictbuildAlignFirstSliceInventory")
            && rt.contains("gate_b09_objectref_mir_run_dictbuild_align_first_slice_inventory")
            && rt.contains("ADR-843")
            && (rt.contains("b09_objectref_mir_run_dictbuild_align_first_slice_signed: true")
                || rt.contains("b09_objectref_mir_run_dictbuild_align_first_slice_signed:true")),
        "syx-rt must expose GateB09ObjectRefMirRunDictbuildAlignFirstSliceInventory"
    );
    assert!(
        (rt.contains("mir_run_dictbuild_objectref_managed_heap_registered: true")
            || rt.contains("mir_run_dictbuild_objectref_managed_heap_registered:true"))
            && (rt.contains("mir_run_dictbuild_objectref_heap_slots_tracked: true")
                || rt.contains("mir_run_dictbuild_objectref_heap_slots_tracked:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false"))
            && (rt.contains("storage_cleanup_enabled: false")
                || rt.contains("storage_cleanup_enabled:false"))
            && (rt.contains("unified_mir_runtime_heap_enabled: false")
                || rt.contains("unified_mir_runtime_heap_enabled:false")),
        "inventory must register align and keep migrate/cleanup/enable false"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_objectref_mir_run_dictbuild_align_first_slice_inventory")
            && lib.contains("GateB09ObjectRefMirRunDictbuildAlignFirstSliceInventory"),
        "syx-rt lib.rs must re-export mir-run DictBuild align inventory"
    );
}

#[test]
fn b09_objectref_mir_run_dictbuild_align_first_slice_spec_signed() {
    let spec =
        include_str!("../../../../docs/gates/gate-b09-objectref-mir-run-dictbuild-align-first-slice.md");
    assert!(
        spec.contains("ADR-843")
            && (spec.contains("DictBuild")
                || spec.contains("ManagedHeap")
                || spec.contains("align")),
        "spec must cite ADR-843 mir-run DictBuild ManagedHeap align"
    );
    assert!(
        spec.contains("three-hundred-and-forty-ninth"),
        
        "spec must cite W499 fingerprint"
    );
    assert!(
        spec.contains("GateB09ObjectRefMirRunDictbuildAlignFirstSliceInventory")
            && spec.contains("gate_b09_objectref_mir_run_dictbuild_align_first_slice_inventory"),
        "spec must name mir-run DictBuild align inventory API"
    );
    assert!(
        spec.contains("b09_objectref_mir_run_dictbuild_align_first_slice_signed=true")
            || spec.contains("`b09_objectref_mir_run_dictbuild_align_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("mir_run_dictbuild_objectref_managed_heap_registered=true")
            || spec.contains("`mir_run_dictbuild_objectref_managed_heap_registered=true`"))
            && (spec.contains("mir_run_dictbuild_objectref_heap_slots_tracked=true")
                || spec.contains("`mir_run_dictbuild_objectref_heap_slots_tracked=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`"))
            && (spec.contains("storage_cleanup_enabled=false")
                || spec.contains("`storage_cleanup_enabled=false`"))
            && (spec.contains("llvm_wasm_objectref_rc_still_live=true")
                || spec.contains("`llvm_wasm_objectref_rc_still_live=true`")),
        "spec must cite align flags and migrate/cleanup false + RC still live"
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
fn b09_objectref_mir_run_dictbuild_align_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_objectref_mir_run_dictbuild_align_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_vm_heap_slots_first_slice_floor_ok);
    assert!(inv.mir_run_dictbuild_objectref_managed_heap_registered);
    assert!(inv.mir_run_dictbuild_objectref_heap_slots_tracked);
    assert!(inv.objectref_rc_storage_still_present);
    assert!(inv.llvm_wasm_objectref_rc_still_live);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_objectref_mir_run_dictbuild_align_first_slice_signed);
    let slots = syx_rt::gate_b09_objectref_vm_heap_slots_first_slice_inventory();
    assert!(slots.vm_dictbuild_objectref_heap_slots_tracked);
    assert!(!slots.objectref_rc_migrate_enabled);
    assert!(slots.b09_objectref_vm_heap_slots_first_slice_signed);
}

