//! ObjectRef re-entry maybe_track residual peel true-delete first-slice
//! ObjectRef maybe_track_* deleted; MutRefCell kept
//! migrate still false; ≠ migrate ≠ enable ≠ [x].

#[test]
fn b09_objectref_reentry_maybe_track_peel_delete_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "crates/syx-vm/src/lib.rs",
        "crates/syx-mir-run/src/runtime.rs",
        "docs/gates/gate-b09-objectref-reentry-maybe-track-peel-delete-first-slice.md",
        "docs/gates/gate-b09-objectref-reentry-maybe-track-peel-readiness-thin.md",
        "docs/gates/gate-b09-objectref-dual-track-collapse-delete-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 re-entry maybe_track peel true-delete requires evidence file: {path}"
        );
    }
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09ObjectrefReentryMaybeTrackPeelDeleteFirstSliceInventory")
            && rt.contains("gate_b09_objectref_reentry_maybe_track_peel_delete_first_slice_inventory")
            && rt.contains("ADR-894")
            && (rt.contains("b09_objectref_reentry_maybe_track_peel_delete_first_slice_signed: true")
                || rt.contains("b09_objectref_reentry_maybe_track_peel_delete_first_slice_signed:true")),
        "syx-rt must expose GateB09ObjectrefReentryMaybeTrackPeelDeleteFirstSliceInventory"
    );
    assert!(
        (rt.contains("objectref_reentry_maybe_track_deleted: true")
            || rt.contains("objectref_reentry_maybe_track_deleted:true"))
            && (rt.contains("reentry_maybe_track_peel_met: true")
                || rt.contains("reentry_maybe_track_peel_met:true"))
            && (rt.contains("mutrefcell_maybe_track_kept: true")
                || rt.contains("mutrefcell_maybe_track_kept:true"))
            && (rt.contains("dual_track_collapse_met: true")
                || rt.contains("dual_track_collapse_met:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false")),
        "inventory must document peel met, MutRefCell kept, migrate false"
    );
    let vm = include_str!("../../../syx-vm/src/lib.rs");
    assert!(
        !vm.contains("fn maybe_track_objectref_value")
            && !vm.contains("fn maybe_track_stack_top_objectref")
            && !vm.contains("self.maybe_track_stack_top_objectref()")
            && !vm.contains("self.maybe_track_objectref_value("),
        "VM ObjectRef re-entry maybe_track_* must be deleted"
    );
    assert!(
        !vm.contains("maybe_track_mutrefcell") && !vm.contains("maybe_track_stack_top_mutrefcell"),
        "VM MutRefCell maybe_track_* must be peeled (W553)"
    );
    assert!(
        vm.contains("ADR-891")
            && vm.contains("alloc_heap_slot")
            && vm.contains("object_ref_from_map"),
        "DictBuild ObjectRef heap-primary birth must remain (W547 floor)"
    );
    assert!(
        vm.contains("ADR-894") || vm.contains("W550"),
        "VM must cite ADR-894 / W550 peel"
    );
    let mir = include_str!("../../../syx-mir-run/src/runtime.rs");
    assert!(
        !mir.contains("fn maybe_track_objectref_value")
            && !mir.contains("fn maybe_track_stack_top_objectref")
            && !mir.contains("self.maybe_track_stack_top_objectref()")
            && !mir.contains("self.maybe_track_objectref_value("),
        "mir-run ObjectRef re-entry maybe_track_* must be deleted"
    );
    assert!(
        !mir.contains("maybe_track_mutrefcell") && !mir.contains("maybe_track_stack_top_mutrefcell"),
        "mir-run MutRefCell maybe_track_* must be peeled (W553)"
    );
    assert!(
        mir.contains("ADR-891") && mir.contains("alloc_heap_slot"),
        "mir-run DictBuild ObjectRef heap-primary birth must remain (W547 floor)"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_objectref_reentry_maybe_track_peel_delete_first_slice_inventory")
            && lib.contains("GateB09ObjectrefReentryMaybeTrackPeelDeleteFirstSliceInventory"),
        "syx-rt lib.rs must re-export re-entry maybe_track peel delete inventory"
    );
}

#[test]
fn b09_objectref_reentry_maybe_track_peel_delete_first_slice_spec_signed() {
    let spec =
        include_str!("../../../../docs/gates/gate-b09-objectref-reentry-maybe-track-peel-delete-first-slice.md");
    assert!(
        spec.contains("ADR-894")
            && (spec.contains("re-entry")
                || spec.contains("maybe_track")
                || spec.contains("true-delete")
                || spec.contains("peel")),
        "spec must cite ADR-894 re-entry maybe_track peel true-delete"
    );
    assert!(
        spec.contains("four-hundredth"),
        
        "spec must cite W550 fingerprint"
    );
    assert!(
        spec.contains("GateB09ObjectrefReentryMaybeTrackPeelDeleteFirstSliceInventory")
            && spec.contains("gate_b09_objectref_reentry_maybe_track_peel_delete_first_slice_inventory"),
        "spec must name re-entry maybe_track peel delete inventory API"
    );
    assert!(
        spec.contains("b09_objectref_reentry_maybe_track_peel_delete_first_slice_signed=true")
            || spec.contains("`b09_objectref_reentry_maybe_track_peel_delete_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("objectref_reentry_maybe_track_deleted=true")
            || spec.contains("`objectref_reentry_maybe_track_deleted=true`"))
            && (spec.contains("reentry_maybe_track_peel_met=true")
                || spec.contains("`reentry_maybe_track_peel_met=true`"))
            && (spec.contains("mutrefcell_maybe_track_kept=true")
                || spec.contains("`mutrefcell_maybe_track_kept=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`")),
        "spec must cite peel met, MutRefCell kept, migrate false"
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
fn b09_objectref_reentry_maybe_track_peel_delete_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_objectref_reentry_maybe_track_peel_delete_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_reentry_maybe_track_peel_readiness_thin_floor_ok);
    assert!(inv.dual_track_collapse_met);
    assert!(inv.dictbuild_objectref_heap_primary_birth_met);
    assert!(inv.objectref_reentry_maybe_track_deleted);
    assert!(inv.reentry_maybe_track_peel_met);
    assert!(inv.mutrefcell_maybe_track_kept);
    assert!(inv.mutrefcell_dual_track_out_of_scope);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_objectref_reentry_maybe_track_peel_delete_first_slice_signed);
    let floor = syx_rt::gate_b09_objectref_reentry_maybe_track_peel_readiness_thin_inventory();
    assert!(floor.b09_objectref_reentry_maybe_track_peel_readiness_thin_signed);
    assert!(floor.objectref_reentry_maybe_track_still_present);
    assert!(floor.reentry_maybe_track_peel_still_unmet);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(!floor.b09_axis_exit_enabled);
}


