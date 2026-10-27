//! ObjectRef deep field-graph GC walk true first-slice
//! Walk met + behavioral nail; ≠ peel ≠ migrate ≠ [x].

#[test]
fn b09_objectref_deep_field_graph_gc_walk_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "crates/syx-runtime-core/src/managed_heap.rs",
        "crates/syx-runtime-core/src/value.rs",
        "docs/gates/gate-b09-objectref-deep-field-graph-gc-walk-first-slice.md",
        "docs/gates/gate-b09-objectref-deep-field-graph-gc-walk-readiness-thin.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 deep-walk first-slice requires evidence file: {path}"
        );
    }
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09ObjectrefDeepFieldGraphGcWalkFirstSliceInventory")
            && rt.contains("gate_b09_objectref_deep_field_graph_gc_walk_first_slice_inventory")
            && rt.contains("ADR-883")
            && (rt.contains("b09_objectref_deep_field_graph_gc_walk_first_slice_signed: true")
                || rt.contains("b09_objectref_deep_field_graph_gc_walk_first_slice_signed:true")),
        "syx-rt must expose GateB09ObjectrefDeepFieldGraphGcWalkFirstSliceInventory"
    );
    assert!(
        (rt.contains("deep_field_graph_gc_walk_met: true")
            || rt.contains("deep_field_graph_gc_walk_met:true"))
            && (rt.contains("deep_field_graph_gc_walk_behavioral_nail_ok: true")
                || rt.contains("deep_field_graph_gc_walk_behavioral_nail_ok:true"))
            && (rt.contains("managed_heap_trace_object_ref_fields_wired: true")
                || rt.contains("managed_heap_trace_object_ref_fields_wired:true"))
            && (rt.contains("object_handle_table_api_still_present: true")
                || rt.contains("object_handle_table_api_still_present:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false")),
        "inventory must document walk met, nail ok, migrate false"
    );
    let heap = include_str!("../../../syx-runtime-core/src/managed_heap.rs");
    assert!(
        heap.contains("deep_field_graph_gc_walk_keeps_nested_sweeps_orphan")
            && heap.contains("object_ref_field_values")
            && heap.contains("fn trace"),
        "ManagedHeap must contain deep-walk behavioral nail + field trace"
    );
    let value = include_str!("../../../syx-runtime-core/src/value.rs");
    assert!(
        !value.contains("rc: u32"),
        "ObjectRef rc: u32 storage must remain deleted (W537 floor)"
    );
    assert!(
        !std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../crates/syx-runtime-core/src/object_handles.rs"
        ))
        .is_file(),
        "object_handles.rs must be deleted (W541 peel); inventory historical still_present remains"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_objectref_deep_field_graph_gc_walk_first_slice_inventory")
            && lib.contains("GateB09ObjectrefDeepFieldGraphGcWalkFirstSliceInventory"),
        "syx-rt lib.rs must re-export deep-walk first-slice inventory"
    );
}

#[test]
fn b09_objectref_deep_field_graph_gc_walk_first_slice_spec_signed() {
    let spec =
        include_str!("../../../../docs/gates/gate-b09-objectref-deep-field-graph-gc-walk-first-slice.md");
    assert!(
        spec.contains("ADR-883")
            && (spec.contains("deep field-graph")
                || spec.contains("deep-walk")
                || spec.contains("Deep-walk")),
        "spec must cite ADR-883 deep-walk first-slice"
    );
    assert!(
        spec.contains("three-hundred-and-eighty-ninth"),
        
        "spec must cite W539 fingerprint"
    );
    assert!(
        spec.contains("GateB09ObjectrefDeepFieldGraphGcWalkFirstSliceInventory")
            && spec.contains("gate_b09_objectref_deep_field_graph_gc_walk_first_slice_inventory"),
        "spec must name deep-walk first-slice inventory API"
    );
    assert!(
        spec.contains("b09_objectref_deep_field_graph_gc_walk_first_slice_signed=true")
            || spec.contains("`b09_objectref_deep_field_graph_gc_walk_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("deep_field_graph_gc_walk_met=true")
            || spec.contains("`deep_field_graph_gc_walk_met=true`"))
            && (spec.contains("deep_field_graph_gc_walk_behavioral_nail_ok=true")
                || spec.contains("`deep_field_graph_gc_walk_behavioral_nail_ok=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`")),
        "spec must cite walk met, nail ok, migrate false"
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
        "spec Frozen must fence peel / migrate flip / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_objectref_deep_field_graph_gc_walk_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_objectref_deep_field_graph_gc_walk_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_deep_field_graph_gc_walk_readiness_thin_floor_ok);
    assert!(inv.objectref_rc_storage_deleted);
    assert!(inv.object_handle_table_api_still_present);
    assert!(inv.mir_run_object_ref_opcodes_still_present);
    assert!(inv.deep_field_graph_gc_walk_met);
    assert!(inv.deep_field_graph_gc_walk_behavioral_nail_ok);
    assert!(inv.managed_heap_trace_object_ref_fields_wired);
    assert!(inv.dual_track_and_heap_coexistence_noted);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_objectref_deep_field_graph_gc_walk_first_slice_signed);
    let floor = syx_rt::gate_b09_objectref_deep_field_graph_gc_walk_readiness_thin_inventory();
    assert!(floor.b09_objectref_deep_field_graph_gc_walk_readiness_thin_signed);
    assert!(floor.deep_field_graph_gc_walk_still_unmet);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(!floor.b09_axis_exit_enabled);
}

#[test]
fn b09_objectref_deep_field_graph_gc_walk_first_slice_fingerprint_and_adr() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W539")
            && (roadmap.contains("ADR-883")
                || roadmap.contains("deep field-graph")
                || roadmap.contains("deep-walk")),
        "roadmap must cite W539 / ADR-883 deep-walk first-slice"
    );
    assert!(
        roadmap.contains("three-hundred-and-eighty-ninth")
            || roadmap.contains("three-hundred-and-eighty-eighth"),
        
        "roadmap must cite W539 fingerprint"
    );
    let adr = include_str!("../../../../docs/adr/architecture-decisions.md");
    assert!(
        adr.contains("ADR-883") && adr.contains("W539"),
        "ADR must record ADR-883 / W539"
    );
    crate::matrix_fingerprint::assert_live_matrix_fingerprint();
}

#[test]
fn b09_objectref_deep_field_graph_gc_walk_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W539")
            && (roadmap.contains("ADR-883")
                || roadmap.contains("deep field-graph")
                || roadmap.contains("deep-walk")),
        "roadmap must cite W539 / ADR-883 deep-walk first-slice"
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
        "roadmap B-09 must be [x] (W558+) or historical [~] (walk ≠ exit): {b09}"
    );
}
