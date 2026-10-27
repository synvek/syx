//! ObjectRef strip-RC true-delete first-slice
//! `rc` field deleted; table/opcodes kept; ≠ peel ≠ migrate ≠ [x].

#[test]
fn b09_objectref_strip_rc_delete_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "crates/syx-runtime-core/src/value.rs",
        "docs/gates/gate-b09-objectref-strip-rc-delete-first-slice.md",
        "docs/gates/gate-b09-objectref-strip-rc-readiness-thin.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 strip-RC true-delete requires evidence file: {path}"
        );
    }
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09ObjectrefStripRcDeleteFirstSliceInventory")
            && rt.contains("gate_b09_objectref_strip_rc_delete_first_slice_inventory")
            && rt.contains("ADR-881")
            && (rt.contains("b09_objectref_strip_rc_delete_first_slice_signed: true")
                || rt.contains("b09_objectref_strip_rc_delete_first_slice_signed:true")),
        "syx-rt must expose GateB09ObjectrefStripRcDeleteFirstSliceInventory"
    );
    assert!(
        (rt.contains("objectref_rc_storage_deleted: true")
            || rt.contains("objectref_rc_storage_deleted:true"))
            && (rt.contains("object_handle_retain_rc_retired: true")
                || rt.contains("object_handle_retain_rc_retired:true"))
            && (rt.contains("object_handle_drop_arc_death_wired: true")
                || rt.contains("object_handle_drop_arc_death_wired:true"))
            && (rt.contains("object_handle_table_api_still_present: true")
                || rt.contains("object_handle_table_api_still_present:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false")),
        "inventory must document rc deleted, retain retired, table present, migrate false"
    );
    let value = include_str!("../../../syx-runtime-core/src/value.rs");
    assert!(
        !value.contains("rc: u32"),
        "ObjectRef rc: u32 field must be deleted"
    );
    assert!(
        value.contains("ObjectRef {") && value.contains("deiniting"),
        "ObjectRef variant must remain without rc"
    );
    assert!(
        !std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../crates/syx-runtime-core/src/object_handles.rs"
        ))
        .is_file(),
        "object_handles.rs must be deleted (W541 peel); inventory historical still_present remains"
    );
    // W541: table deleted; W537 inventory nails still claim historical retain no-op / drop wiring.
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_objectref_strip_rc_delete_first_slice_inventory")
            && lib.contains("GateB09ObjectrefStripRcDeleteFirstSliceInventory"),
        "syx-rt lib.rs must re-export strip-RC delete inventory"
    );
}

#[test]
fn b09_objectref_strip_rc_delete_first_slice_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b09-objectref-strip-rc-delete-first-slice.md");
    assert!(
        spec.contains("ADR-881")
            && (spec.contains("strip-RC") || spec.contains("true-delete") || spec.contains("deleted")),
        "spec must cite ADR-881 strip-RC true-delete"
    );
    assert!(
        spec.contains("three-hundred-and-eighty-seventh"),
        
        "spec must cite W537 fingerprint"
    );
    assert!(
        spec.contains("GateB09ObjectrefStripRcDeleteFirstSliceInventory")
            && spec.contains("gate_b09_objectref_strip_rc_delete_first_slice_inventory"),
        "spec must name strip-RC delete inventory API"
    );
    assert!(
        spec.contains("b09_objectref_strip_rc_delete_first_slice_signed=true")
            || spec.contains("`b09_objectref_strip_rc_delete_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("objectref_rc_storage_deleted=true")
            || spec.contains("`objectref_rc_storage_deleted=true`"))
            && (spec.contains("object_handle_table_api_still_present=true")
                || spec.contains("`object_handle_table_api_still_present=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`")),
        "spec must cite rc deleted, table present, migrate false"
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
            && (spec.contains("peel") || spec.contains("migrate") || spec.contains("B-09")),
        "spec Frozen must fence peel / migrate / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_objectref_strip_rc_delete_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_objectref_strip_rc_delete_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_strip_rc_readiness_thin_floor_ok);
    assert!(inv.objectref_rc_storage_deleted);
    assert!(inv.object_handle_table_api_still_present);
    assert!(inv.mir_run_object_ref_opcodes_still_present);
    assert!(inv.deep_field_graph_gc_walk_still_unmet);
    assert!(inv.dual_track_and_heap_coexistence_noted);
    assert!(inv.object_handle_retain_rc_retired);
    assert!(inv.object_handle_drop_arc_death_wired);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_objectref_strip_rc_delete_first_slice_signed);
    let floor = syx_rt::gate_b09_objectref_strip_rc_readiness_thin_inventory();
    assert!(floor.b09_objectref_strip_rc_readiness_thin_signed);
    assert!(floor.objectref_rc_storage_still_present);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(!floor.b09_axis_exit_enabled);
}

#[test]
fn b09_objectref_strip_rc_delete_first_slice_fingerprint_and_adr() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W537")
            && (roadmap.contains("ADR-881")
                || roadmap.contains("strip-RC")
                || roadmap.contains("true-delete")),
        "roadmap must cite W537 / ADR-881 strip-RC true-delete"
    );
    assert!(
        roadmap.contains("three-hundred-and-eighty-seventh"),
        
        "roadmap must cite W537 fingerprint"
    );
    let adr = include_str!("../../../../docs/adr/architecture-decisions.md");
    assert!(
        adr.contains("ADR-881") && adr.contains("W537"),
        "ADR must record ADR-881 / W537"
    );
    crate::matrix_fingerprint::assert_live_matrix_fingerprint();
}

#[test]
fn b09_objectref_strip_rc_delete_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W537")
            && (roadmap.contains("ADR-881")
                || roadmap.contains("strip-RC")
                || roadmap.contains("true-delete")),
        "roadmap must cite W537 / ADR-881 strip-RC true-delete"
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
        "roadmap B-09 must be [x] (W558+) or historical [~] (strip ≠ exit): {b09}"
    );
}
