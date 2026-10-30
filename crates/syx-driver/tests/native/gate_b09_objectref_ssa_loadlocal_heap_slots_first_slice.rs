//! SSA LoadLocal ManagedHeap + heap_slots first-slice
//! migrate/cleanup false; RC still live; no SSA retain.

#[test]
fn b09_objectref_ssa_loadlocal_heap_slots_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir-run/src/runtime.rs",
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "docs/gates/gate-b09-objectref-ssa-loadlocal-heap-slots-first-slice.md",
        "docs/gates/gate-b09-objectref-loadglobal-heap-slots-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 SSA LoadLocal heap_slots requires evidence file: {path}"
        );
    }
    let mir = include_str!("../../../syx-mir-run/src/runtime.rs");
    assert!(
        mir.contains("MirOp::LoadLocal")
            && !mir.contains("fn maybe_track_stack_top_objectref") /* W550 peel */
            && !mir.contains("maybe_track_stack_top_mutrefcell")
            && mir.contains("ADR-861"),
        "mir-run SSA LoadLocal must dual-track and cite ADR-861"
    );
    let ll = mir
        .split("MirOp::LoadLocal")
        .nth(1)
        .and_then(|s| s.split("MirOp::LoadGlobal").next())
        .unwrap_or("");
    assert!(
        !ll.contains("object_ref_retain"),
        "SSA LoadLocal must not add object_ref_retain"
    );
    assert!(
        !mir.contains("objectref_rc_migrate_enabled = true")
            && !mir.contains("objectref_rc_migrate_enabled=true"),
        "must not flip objectref_rc_migrate_enabled"
    );
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09ObjectRefSsaLoadlocalHeapSlotsFirstSliceInventory")
            && rt.contains("gate_b09_objectref_ssa_loadlocal_heap_slots_first_slice_inventory")
            && rt.contains("ADR-861")
            && (rt.contains("b09_objectref_ssa_loadlocal_heap_slots_first_slice_signed: true")
                || rt.contains("b09_objectref_ssa_loadlocal_heap_slots_first_slice_signed:true")),
        "syx-rt must expose GateB09ObjectRefSsaLoadlocalHeapSlotsFirstSliceInventory"
    );
    assert!(
        (rt.contains("mir_run_ssa_loadlocal_objectref_managed_heap_registered: true")
            || rt.contains("mir_run_ssa_loadlocal_objectref_managed_heap_registered:true"))
            && (rt.contains("mir_run_ssa_loadlocal_heap_slots_tracked: true")
                || rt.contains("mir_run_ssa_loadlocal_heap_slots_tracked:true"))
            && (rt.contains("llvm_wasm_objectref_rc_still_live: true")
                || rt.contains("llvm_wasm_objectref_rc_still_live:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false"))
            && (rt.contains("storage_cleanup_enabled: false")
                || rt.contains("storage_cleanup_enabled:false"))
            && (rt.contains("unified_mir_runtime_heap_enabled: false")
                || rt.contains("unified_mir_runtime_heap_enabled:false")),
        "inventory must register SSA LoadLocal and keep migrate/cleanup/enable false + RC live"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_objectref_ssa_loadlocal_heap_slots_first_slice_inventory")
            && lib.contains("GateB09ObjectRefSsaLoadlocalHeapSlotsFirstSliceInventory"),
        "syx-rt lib.rs must re-export SSA LoadLocal inventory"
    );
}

#[test]
fn b09_objectref_ssa_loadlocal_heap_slots_first_slice_spec_signed() {
    let spec =
        include_str!("../../../../docs/gates/gate-b09-objectref-ssa-loadlocal-heap-slots-first-slice.md");
    assert!(
        spec.contains("ADR-861")
            && (spec.contains("LoadLocal") || spec.contains("SSA"))
            && (spec.contains("MirOp") || spec.contains("mir-run")),
        "spec must cite ADR-861 SSA LoadLocal"
    );
    assert!(
        spec.contains("three-hundred-and-sixty-seventh"),
        
        "spec must cite W517 fingerprint"
    );
    assert!(
        spec.contains("GateB09ObjectRefSsaLoadlocalHeapSlotsFirstSliceInventory")
            && spec.contains("gate_b09_objectref_ssa_loadlocal_heap_slots_first_slice_inventory"),
        "spec must name SSA LoadLocal inventory API"
    );
    assert!(
        spec.contains("b09_objectref_ssa_loadlocal_heap_slots_first_slice_signed=true")
            || spec.contains("`b09_objectref_ssa_loadlocal_heap_slots_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("mir_run_ssa_loadlocal_heap_slots_tracked=true")
            || spec.contains("`mir_run_ssa_loadlocal_heap_slots_tracked=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`"))
            && (spec.contains("storage_cleanup_enabled=false")
                || spec.contains("`storage_cleanup_enabled=false`")),
        "spec must cite SSA LoadLocal flags and migrate/cleanup false"
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
fn b09_objectref_ssa_loadlocal_heap_slots_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_objectref_ssa_loadlocal_heap_slots_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_loadglobal_heap_slots_first_slice_floor_ok);
    assert!(inv.mir_run_ssa_loadlocal_objectref_managed_heap_registered);
    assert!(inv.mir_run_ssa_loadlocal_heap_slots_tracked);
    assert!(inv.objectref_rc_storage_still_present);
    assert!(inv.llvm_wasm_objectref_rc_still_live);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_objectref_ssa_loadlocal_heap_slots_first_slice_signed);
    let floor = syx_rt::gate_b09_objectref_loadglobal_heap_slots_first_slice_inventory();
    assert!(floor.vm_mir_run_loadglobal_heap_slots_tracked);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(floor.b09_objectref_loadglobal_heap_slots_first_slice_signed);
}

