//! ObjectRef dual-track collapse true-delete first-slice
//! DictBuild ObjectRef Arc+maybe_track dual birth collapsed
//! to heap-primary; re-entry maybe_track residual noted; migrate still false
//! ≠ migrate ≠ enable ≠ [x].

#[test]
fn b09_objectref_dual_track_collapse_delete_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "crates/syx-vm/src/lib.rs",
        "crates/syx-mir-run/src/runtime.rs",
        "crates/syx-runtime-core/src/value.rs",
        "docs/gates/gate-b09-objectref-dual-track-collapse-delete-first-slice.md",
        "docs/gates/gate-b09-objectref-dual-track-collapse-readiness-thin.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 dual-track collapse true-delete requires evidence file: {path}"
        );
    }
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09ObjectrefDualTrackCollapseDeleteFirstSliceInventory")
            && rt.contains("gate_b09_objectref_dual_track_collapse_delete_first_slice_inventory")
            && rt.contains("ADR-891")
            && (rt.contains("b09_objectref_dual_track_collapse_delete_first_slice_signed: true")
                || rt.contains("b09_objectref_dual_track_collapse_delete_first_slice_signed:true")),
        "syx-rt must expose GateB09ObjectrefDualTrackCollapseDeleteFirstSliceInventory"
    );
    assert!(
        (rt.contains("dictbuild_objectref_dual_track_birth_deleted: true")
            || rt.contains("dictbuild_objectref_dual_track_birth_deleted:true"))
            && (rt.contains("dictbuild_objectref_heap_primary_birth_met: true")
                || rt.contains("dictbuild_objectref_heap_primary_birth_met:true"))
            && (rt.contains("dual_track_collapse_met: true")
                || rt.contains("dual_track_collapse_met:true"))
            && (rt.contains("objectref_reentry_maybe_track_residual_noted: true")
                || rt.contains("objectref_reentry_maybe_track_residual_noted:true"))
            && (rt.contains("mutrefcell_dual_track_out_of_scope: true")
                || rt.contains("mutrefcell_dual_track_out_of_scope:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false")),
        "inventory must document collapse met, re-entry residual, migrate false"
    );
    let vm = include_str!("../../../syx-vm/src/lib.rs");
    assert!(
        vm.contains("ADR-891")
            && vm.contains("alloc_heap_slot")
            && vm.contains("object_ref_from_map")
            && vm.contains("ManagedHeap::enabled()"),
        "VM DictBuild ObjectRef path must cite ADR-891 heap-primary birth"
    );
    // DictBuild ObjectRef site must not dual-push Arc + maybe_track (historical pattern).
    assert!(
        !vm.contains("maybe_track_objectref_value(&v);\n                    self.stack.push(v)")
            && !vm.contains("maybe_track_objectref_value(&v);\n                    self.stack.push(v);"),
        "VM must not keep DictBuild ObjectRef Arc+maybe_track dual birth pattern"
    );
    let mir = include_str!("../../../syx-mir-run/src/runtime.rs");
    assert!(
        mir.contains("ADR-891")
            && mir.contains("alloc_heap_slot")
            && mir.contains("object_ref_from_map"),
        "mir-run DictBuild ObjectRef path must cite ADR-891 heap-primary birth"
    );
    assert!(
        !mir.contains("maybe_track_objectref_value(&v);\n            self.stack.push(v)")
            && !mir.contains("self.maybe_track_objectref_value(&v);\n            self.stack.push(v);"),
        "mir-run must not keep DictBuild ObjectRef Arc+maybe_track dual birth pattern"
    );
    assert!(
        !mir.contains("fn maybe_track_objectref_value")
            && !mir.contains("fn maybe_track_stack_top_objectref"),
        "ObjectRef re-entry maybe_track_* peeled (W550); W547 inventory residual_noted historical"
    );
    assert!(
        !mir.contains("maybe_track_mutrefcell") && !mir.contains("maybe_track_stack_top_mutrefcell"),
        "MutRefCell maybe_track_* must be peeled (W553)"
    );
    let value = include_str!("../../../syx-runtime-core/src/value.rs");
    assert!(
        value.contains("ADR-891")
            && (value.contains("HeapSlot-wrapped ObjectRef")
                || value.contains("heap-primary")),
        "value.rs must document HeapSlot ObjectRef write-back for W547"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_objectref_dual_track_collapse_delete_first_slice_inventory")
            && lib.contains("GateB09ObjectrefDualTrackCollapseDeleteFirstSliceInventory"),
        "syx-rt lib.rs must re-export dual-track collapse delete inventory"
    );
}

#[test]
fn b09_objectref_dual_track_collapse_delete_first_slice_spec_signed() {
    let spec =
        include_str!("../../../../docs/gates/gate-b09-objectref-dual-track-collapse-delete-first-slice.md");
    assert!(
        spec.contains("ADR-891")
            && (spec.contains("dual-track collapse")
                || spec.contains("true-delete")
                || spec.contains("heap-primary")),
        "spec must cite ADR-891 dual-track collapse true-delete"
    );
    assert!(
        spec.contains("three-hundred-and-ninety-seventh"),
        
        "spec must cite W547 fingerprint"
    );
    assert!(
        spec.contains("GateB09ObjectrefDualTrackCollapseDeleteFirstSliceInventory")
            && spec.contains("gate_b09_objectref_dual_track_collapse_delete_first_slice_inventory"),
        "spec must name dual-track collapse delete inventory API"
    );
    assert!(
        spec.contains("b09_objectref_dual_track_collapse_delete_first_slice_signed=true")
            || spec.contains("`b09_objectref_dual_track_collapse_delete_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("dictbuild_objectref_dual_track_birth_deleted=true")
            || spec.contains("`dictbuild_objectref_dual_track_birth_deleted=true`"))
            && (spec.contains("dual_track_collapse_met=true")
                || spec.contains("`dual_track_collapse_met=true`"))
            && (spec.contains("dictbuild_objectref_heap_primary_birth_met=true")
                || spec.contains("`dictbuild_objectref_heap_primary_birth_met=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`")),
        "spec must cite collapse met, birth deleted, migrate false"
    );
    assert!(
        !spec.contains("objectref_rc_migrate_enabled=true")
            && !spec.contains("`objectref_rc_migrate_enabled=true`")
            && !spec.contains("unified_mir_runtime_heap_enabled=true")
            && !spec.contains("`unified_mir_runtime_heap_enabled=true`"),
        "spec must not claim full migrate or unified heap enabled"
    );
    assert!(
        spec.contains("## Frozen") && (spec.contains("migrate") || spec.contains("B-09")),
        "spec Frozen must fence migrate flip / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_objectref_dual_track_collapse_delete_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_objectref_dual_track_collapse_delete_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_dual_track_collapse_readiness_thin_floor_ok);
    assert!(inv.dictbuild_objectref_dual_track_birth_deleted);
    assert!(inv.dictbuild_objectref_heap_primary_birth_met);
    assert!(inv.dual_track_collapse_met);
    assert!(inv.mirop_vm_residual_peel_met);
    assert!(inv.objectref_reentry_maybe_track_residual_noted);
    assert!(inv.mutrefcell_dual_track_out_of_scope);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_objectref_dual_track_collapse_delete_first_slice_signed);
    let floor = syx_rt::gate_b09_objectref_dual_track_collapse_readiness_thin_inventory();
    assert!(floor.b09_objectref_dual_track_collapse_readiness_thin_signed);
    assert!(floor.dual_track_collapse_still_unmet);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(!floor.b09_axis_exit_enabled);
}


