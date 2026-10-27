//! MutRefCell maybe_track residual peel true-delete first-slice
//! MutRefCell maybe_track_* deleted; ObjectRef already peeled
//! migrate still false; ≠ migrate ≠ enable ≠ [x].

#[test]
fn b09_mutrefcell_maybe_track_peel_delete_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "crates/syx-vm/src/lib.rs",
        "crates/syx-mir-run/src/runtime.rs",
        "docs/gates/gate-b09-mutrefcell-maybe-track-peel-delete-first-slice.md",
        "docs/gates/gate-b09-mutrefcell-maybe-track-peel-readiness-thin.md",
        "docs/gates/gate-b09-objectref-rc-migrate-readiness-post-reentry-peel-refresh-thin.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 MutRefCell maybe_track peel true-delete requires evidence file: {path}"
        );
    }
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09MutrefcellMaybeTrackPeelDeleteFirstSliceInventory")
            && rt.contains("gate_b09_mutrefcell_maybe_track_peel_delete_first_slice_inventory")
            && rt.contains("ADR-897")
            && (rt.contains("b09_mutrefcell_maybe_track_peel_delete_first_slice_signed: true")
                || rt.contains("b09_mutrefcell_maybe_track_peel_delete_first_slice_signed:true")),
        "syx-rt must expose GateB09MutrefcellMaybeTrackPeelDeleteFirstSliceInventory"
    );
    assert!(
        (rt.contains("mutrefcell_maybe_track_deleted: true")
            || rt.contains("mutrefcell_maybe_track_deleted:true"))
            && (rt.contains("mutrefcell_peel_met: true") || rt.contains("mutrefcell_peel_met:true"))
            && (rt.contains("objectref_reentry_maybe_track_deleted: true")
                || rt.contains("objectref_reentry_maybe_track_deleted:true"))
            && (rt.contains("reentry_maybe_track_peel_met: true")
                || rt.contains("reentry_maybe_track_peel_met:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false")),
        "inventory must document MutRefCell peel met/deleted, migrate false"
    );
    let vm = include_str!("../../../syx-vm/src/lib.rs");
    assert!(
        !vm.contains("fn maybe_track_objectref_value")
            && !vm.contains("fn maybe_track_stack_top_objectref")
            && !vm.contains("self.maybe_track_stack_top_objectref()")
            && !vm.contains("self.maybe_track_objectref_value("),
        "ObjectRef re-entry maybe_track_* must remain peeled (W550)"
    );
    assert!(
        !vm.contains("fn maybe_track_mutrefcell_value")
            && !vm.contains("fn maybe_track_stack_top_mutrefcell")
            && !vm.contains("self.maybe_track_stack_top_mutrefcell()")
            && !vm.contains("self.maybe_track_mutrefcell_value("),
        "MutRefCell maybe_track_* must be peeled (W553)"
    );
    assert!(
        vm.contains("ADR-891")
            && vm.contains("alloc_heap_slot")
            && vm.contains("object_ref_from_map"),
        "DictBuild ObjectRef heap-primary birth must remain (W547 floor)"
    );
    let mir = include_str!("../../../syx-mir-run/src/runtime.rs");
    assert!(
        !mir.contains("fn maybe_track_objectref_value")
            && !mir.contains("fn maybe_track_stack_top_objectref")
            && !mir.contains("self.maybe_track_stack_top_objectref()")
            && !mir.contains("self.maybe_track_objectref_value("),
        "mir-run ObjectRef re-entry maybe_track_* must remain peeled (W550)"
    );
    assert!(
        !mir.contains("fn maybe_track_mutrefcell_value")
            && !mir.contains("fn maybe_track_stack_top_mutrefcell")
            && !mir.contains("self.maybe_track_stack_top_mutrefcell()")
            && !mir.contains("self.maybe_track_mutrefcell_value("),
        "mir-run MutRefCell maybe_track_* must be peeled (W553)"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_mutrefcell_maybe_track_peel_delete_first_slice_inventory")
            && lib.contains("GateB09MutrefcellMaybeTrackPeelDeleteFirstSliceInventory"),
        "syx-rt lib.rs must re-export MutRefCell maybe_track peel true-delete inventory"
    );
}

#[test]
fn b09_mutrefcell_maybe_track_peel_delete_first_slice_spec_signed() {
    let spec =
        include_str!("../../../../docs/gates/gate-b09-mutrefcell-maybe-track-peel-delete-first-slice.md");
    assert!(
        spec.contains("ADR-897")
            && (spec.contains("MutRefCell")
                || spec.contains("maybe_track")
                || spec.contains("true-delete")),
        "spec must cite ADR-897 MutRefCell maybe_track peel true-delete"
    );
    assert!(
        spec.contains("four-hundred-and-third"),
        
        "spec must cite W553 fingerprint"
    );
    assert!(
        spec.contains("GateB09MutrefcellMaybeTrackPeelDeleteFirstSliceInventory")
            && spec.contains("gate_b09_mutrefcell_maybe_track_peel_delete_first_slice_inventory"),
        "spec must name MutRefCell maybe_track peel true-delete inventory API"
    );
    assert!(
        spec.contains("b09_mutrefcell_maybe_track_peel_delete_first_slice_signed=true")
            || spec.contains("`b09_mutrefcell_maybe_track_peel_delete_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("mutrefcell_maybe_track_deleted=true")
            || spec.contains("`mutrefcell_maybe_track_deleted=true`"))
            && (spec.contains("mutrefcell_peel_met=true")
                || spec.contains("`mutrefcell_peel_met=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`")),
        "spec must cite MutRefCell deleted/peel met, migrate false"
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
fn b09_mutrefcell_maybe_track_peel_delete_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_mutrefcell_maybe_track_peel_delete_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_mutrefcell_maybe_track_peel_readiness_thin_floor_ok);
    assert!(inv.objectref_reentry_maybe_track_deleted);
    assert!(inv.reentry_maybe_track_peel_met);
    assert!(inv.dual_track_collapse_met);
    assert!(inv.dictbuild_objectref_heap_primary_birth_met);
    assert!(inv.migrate_flip_blockers_refreshed_after_reentry_peel);
    assert!(inv.mutrefcell_maybe_track_deleted);
    assert!(inv.mutrefcell_peel_met);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_mutrefcell_maybe_track_peel_delete_first_slice_signed);
    let floor = syx_rt::gate_b09_mutrefcell_maybe_track_peel_readiness_thin_inventory();
    assert!(floor.b09_mutrefcell_maybe_track_peel_readiness_thin_signed);
    assert!(floor.mutrefcell_maybe_track_still_present);
    assert!(floor.mutrefcell_peel_still_unmet);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(!floor.b09_axis_exit_enabled);
}

#[test]
fn b09_mutrefcell_maybe_track_peel_delete_first_slice_fingerprint_and_adr() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W553")
            && (roadmap.contains("ADR-897")
                || roadmap.contains("MutRefCell")
                || roadmap.contains("maybe_track")),
        "roadmap must cite W553 / ADR-897 MutRefCell maybe_track peel true-delete"
    );
    assert!(
        roadmap.contains("four-hundred-and-fourth")
            || roadmap.contains("four-hundred-and-third")
            || roadmap.contains("four-hundred-and-second"),
        
        "roadmap must cite W553 fingerprint"
    );
    let adr = include_str!("../../../../docs/adr/architecture-decisions.md");
    assert!(
        adr.contains("ADR-897") && adr.contains("W553"),
        "ADR must record ADR-897 / W553"
    );
    crate::matrix_fingerprint::assert_live_matrix_fingerprint();
}

#[test]
fn b09_mutrefcell_maybe_track_peel_delete_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W553")
            && (roadmap.contains("ADR-897")
                || roadmap.contains("MutRefCell")
                || roadmap.contains("maybe_track")),
        "roadmap must cite W553 / ADR-897 MutRefCell maybe_track peel true-delete"
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
        "roadmap B-09 must be [x] (W558+) or historical [~] (peel ≠ exit): {b09}"
    );
    let parity = include_str!("../../../../plan/topics/gc-parity.md");
    assert!(
        parity.contains("W553")
            && (parity.contains("MutRefCell")
                || parity.contains("maybe_track")
                || parity.contains("ADR-897")),
        "gc-parity must cite W553 MutRefCell maybe_track peel true-delete"
    );
    assert!(
        parity.contains("still unmet")
            || parity.contains("仍 unmet")
            || parity.contains("[~]"),
        "gc-parity must keep migrate/enable/[x] unmet honesty"
    );
}
