//! ObjectRef RC migrate product flip first-slice
//! Migrate product ON after refresh
//! peels/collapse/strip/walk met; cleanup/enable/[x] still false
//! ≠ cleanup ≠ enable ≠ [x].

#[test]
fn b09_objectref_rc_migrate_product_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "crates/syx-vm/src/lib.rs",
        "crates/syx-mir-run/src/runtime.rs",
        "docs/gates/gate-b09-objectref-rc-migrate-product-first-slice.md",
        "docs/gates/gate-b09-objectref-rc-migrate-readiness-post-mutrefcell-peel-refresh-thin.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 ObjectRef RC migrate product flip requires evidence file: {path}"
        );
    }
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09ObjectrefRcMigrateProductFirstSliceInventory")
            && rt.contains("gate_b09_objectref_rc_migrate_product_first_slice_inventory")
            && rt.contains("pub fn objectref_rc_migrate_enabled")
            && rt.contains("ADR-899")
            && (rt.contains("b09_objectref_rc_migrate_product_first_slice_signed: true")
                || rt.contains("b09_objectref_rc_migrate_product_first_slice_signed:true")),
        "syx-rt must expose GateB09ObjectrefRcMigrateProductFirstSliceInventory + API"
    );
    assert!(
        (rt.contains("objectref_rc_migrate_enabled: true")
            || rt.contains("objectref_rc_migrate_enabled:true"))
            && (rt.contains("migrate_product_flip_met: true")
                || rt.contains("migrate_product_flip_met:true"))
            && (rt.contains("sharedcell_arc_lifetime_still_present: true")
                || rt.contains("sharedcell_arc_lifetime_still_present:true"))
            && (rt.contains("storage_cleanup_enabled: false")
                || rt.contains("storage_cleanup_enabled:false")),
        "inventory must document migrate true, product flip met, cleanup false"
    );
    let vm = include_str!("../../../syx-vm/src/lib.rs");
    assert!(
        !vm.contains("fn maybe_track_objectref_value")
            && !vm.contains("fn maybe_track_stack_top_objectref")
            && !vm.contains("fn maybe_track_mutrefcell_value")
            && !vm.contains("fn maybe_track_stack_top_mutrefcell"),
        "VM ObjectRef+MutRefCell maybe_track_* must remain peeled"
    );
    let mir = include_str!("../../../syx-mir-run/src/runtime.rs");
    assert!(
        !mir.contains("fn maybe_track_objectref_value")
            && !mir.contains("fn maybe_track_stack_top_objectref")
            && !mir.contains("fn maybe_track_mutrefcell_value")
            && !mir.contains("fn maybe_track_stack_top_mutrefcell"),
        "mir-run ObjectRef+MutRefCell maybe_track_* must remain peeled"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_objectref_rc_migrate_product_first_slice_inventory")
            && lib.contains("GateB09ObjectrefRcMigrateProductFirstSliceInventory")
            && lib.contains("objectref_rc_migrate_enabled"),
        "syx-rt lib.rs must re-export migrate product inventory + API"
    );
}

#[test]
fn b09_objectref_rc_migrate_product_first_slice_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b09-objectref-rc-migrate-product-first-slice.md");
    assert!(
        spec.contains("ADR-899")
            && (spec.contains("migrate product")
                || spec.contains("product flip")
                || spec.contains("objectref_rc_migrate_enabled")),
        "spec must cite ADR-899 migrate product flip"
    );
    assert!(
        spec.contains("four-hundred-and-fifth"),
        
        "spec must cite W555 fingerprint"
    );
    assert!(
        spec.contains("GateB09ObjectrefRcMigrateProductFirstSliceInventory")
            && spec.contains("gate_b09_objectref_rc_migrate_product_first_slice_inventory")
            && spec.contains("objectref_rc_migrate_enabled()"),
        "spec must name migrate product inventory + API"
    );
    assert!(
        spec.contains("b09_objectref_rc_migrate_product_first_slice_signed=true")
            || spec.contains("`b09_objectref_rc_migrate_product_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("objectref_rc_migrate_enabled=true")
            || spec.contains("`objectref_rc_migrate_enabled=true`"))
            && (spec.contains("migrate_product_flip_met=true")
                || spec.contains("`migrate_product_flip_met=true`"))
            && (spec.contains("storage_cleanup_enabled=false")
                || spec.contains("`storage_cleanup_enabled=false`")),
        "spec must cite migrate true, product flip met, cleanup false"
    );
    assert!(
        !spec.contains("storage_cleanup_enabled=true")
            && !spec.contains("`storage_cleanup_enabled=true`")
            && !spec.contains("unified_mir_runtime_heap_enabled=true")
            && !spec.contains("`unified_mir_runtime_heap_enabled=true`"),
        "spec must not claim cleanup or unified heap enabled"
    );
    assert!(
        spec.contains("## Frozen") && (spec.contains("cleanup") || spec.contains("B-09")),
        "spec Frozen must fence cleanup / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_objectref_rc_migrate_product_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_objectref_rc_migrate_product_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_rc_migrate_readiness_post_mutrefcell_peel_refresh_thin_floor_ok);
    assert!(inv.migrate_flip_blockers_refreshed_after_mutrefcell_peel);
    assert!(inv.mutrefcell_peel_met);
    assert!(inv.reentry_maybe_track_peel_met);
    assert!(inv.dual_track_collapse_met);
    assert!(inv.dictbuild_objectref_heap_primary_birth_met);
    assert!(inv.objectref_rc_storage_stripped);
    assert!(inv.deep_field_graph_gc_walk_met);
    assert!(inv.sharedcell_arc_lifetime_still_present);
    assert!(inv.objectref_rc_migrate_enabled);
    assert!(inv.migrate_product_flip_met);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_objectref_rc_migrate_product_first_slice_signed);
    assert!(syx_rt::objectref_rc_migrate_enabled());
    assert_eq!(
        syx_rt::objectref_rc_migrate_enabled(),
        inv.objectref_rc_migrate_enabled
    );
    let floor = syx_rt::gate_b09_objectref_rc_migrate_readiness_post_mutrefcell_peel_refresh_thin_inventory();
    assert!(floor.b09_objectref_rc_migrate_readiness_post_mutrefcell_peel_refresh_thin_signed);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(!floor.b09_axis_exit_enabled);
}

#[test]
fn b09_objectref_rc_migrate_product_first_slice_fingerprint_and_adr() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W555")
            && (roadmap.contains("ADR-899")
                || roadmap.contains("migrate product")
                || roadmap.contains("product flip")),
        "roadmap must cite W555 / ADR-899 migrate product flip"
    );
    assert!(
        roadmap.contains("four-hundred-and-sixth")
            || roadmap.contains("four-hundred-and-fifth")
            || roadmap.contains("four-hundred-and-fourth"),
        
        "roadmap must cite W555 fingerprint"
    );
    let adr = include_str!("../../../../docs/adr/architecture-decisions.md");
    assert!(
        adr.contains("ADR-899") && adr.contains("W555"),
        "ADR must record ADR-899 / W555"
    );
    crate::matrix_fingerprint::assert_live_matrix_fingerprint();
}

#[test]
fn b09_objectref_rc_migrate_product_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W555")
            && (roadmap.contains("ADR-899")
                || roadmap.contains("migrate product")
                || roadmap.contains("product flip")),
        "roadmap must cite W555 / ADR-899 migrate product flip"
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
        "roadmap B-09 must be [x] (W558+) or historical [~] (migrate ≠ exit): {b09}"
    );
    let parity = include_str!("../../../../plan/topics/gc-parity.md");
    assert!(
        parity.contains("W555")
            && (parity.contains("migrate product")
                || parity.contains("product flip")
                || parity.contains("ADR-899")),
        "gc-parity must cite W555 migrate product flip"
    );
    assert!(
        parity.contains("still unmet")
            || parity.contains("仍 unmet")
            || parity.contains("[~]"),
        "gc-parity must keep cleanup/enable/[x] unmet honesty"
    );
}
