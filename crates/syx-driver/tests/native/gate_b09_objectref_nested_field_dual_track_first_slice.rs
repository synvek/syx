//! ObjectRef nested field dual-track first-slice
//! ObjectSet nested write-path val dual-track; ≠ deep walk ≠ migrate ≠ [x].

#[test]
fn b09_objectref_nested_field_dual_track_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "crates/syx-vm/src/lib.rs",
        "crates/syx-mir-run/src/runtime.rs",
        "crates/syx-rt/src/abi.rs",
        "crates/syx-runtime-core/src/value.rs",
        "docs/gates/gate-b09-objectref-nested-field-dual-track-first-slice.md",
        "docs/gates/gate-b09-objectref-rc-migrate-readiness-refresh-thin.md",
        "docs/gates/gate-b09-objectref-residual-birth-heap-slots-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 nested field dual-track requires evidence file: {path}"
        );
    }
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09ObjectrefNestedFieldDualTrackFirstSliceInventory")
            && rt.contains("gate_b09_objectref_nested_field_dual_track_first_slice_inventory")
            && rt.contains("ADR-876")
            && (rt.contains("b09_objectref_nested_field_dual_track_first_slice_signed: true")
                || rt.contains("b09_objectref_nested_field_dual_track_first_slice_signed:true")),
        "syx-rt must expose GateB09ObjectrefNestedFieldDualTrackFirstSliceInventory"
    );
    assert!(
        (rt.contains("nested_field_write_path_dual_track_met: true")
            || rt.contains("nested_field_write_path_dual_track_met:true"))
            && (rt.contains("deep_field_graph_gc_walk_still_unmet: true")
                || rt.contains("deep_field_graph_gc_walk_still_unmet:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false")),
        "inventory must document write-path met, deep walk unmet, migrate false"
    );
    let vm = include_str!("../../../syx-vm/src/lib.rs");
    assert!(
        vm.contains("ADR-876")
            && !vm.contains("fn maybe_track_objectref_value") /* W550 peel */
            && !vm.contains("maybe_track_mutrefcell_value(&val)"),
        "VM ObjectSet must dual-track nested val before set (ADR-876)"
    );
    let mir = include_str!("../../../syx-mir-run/src/runtime.rs");
    assert!(
        mir.contains("ADR-876")
            && !mir.contains("fn maybe_track_objectref_value") /* W550 peel */
            && !mir.contains("maybe_track_mutrefcell_value(&value)"),
        "mir-run object_set must dual-track nested value before set (ADR-876)"
    );
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    assert!(
        !abi.contains("pub extern \"C\" fn syx_rt_object_retain")
            && !abi.contains("pub extern \"C\" fn syx_rt_object_release")
            && !abi.contains("pub extern \"C\" fn syx_rt_object_drop"),
        "handle ABI retain/release/drop must be deleted (W534)"
    );
    let value = include_str!("../../../syx-runtime-core/src/value.rs");
    assert!(
        !value.contains("rc: u32"),
        "ObjectRef rc: u32 storage must be deleted (W537)"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_objectref_nested_field_dual_track_first_slice_inventory")
            && lib.contains("GateB09ObjectrefNestedFieldDualTrackFirstSliceInventory"),
        "syx-rt lib.rs must re-export nested field dual-track inventory"
    );
}

#[test]
fn b09_objectref_nested_field_dual_track_first_slice_spec_signed() {
    let spec =
        include_str!("../../../../docs/gates/gate-b09-objectref-nested-field-dual-track-first-slice.md");
    assert!(
        spec.contains("ADR-876")
            && (spec.contains("nested") || spec.contains("write-path") || spec.contains("dual-track")),
        "spec must cite ADR-876 nested field dual-track"
    );
    assert!(
        spec.contains("three-hundred-and-eighty-second"),
        
        "spec must cite W532 fingerprint"
    );
    assert!(
        spec.contains("GateB09ObjectrefNestedFieldDualTrackFirstSliceInventory")
            && spec.contains("gate_b09_objectref_nested_field_dual_track_first_slice_inventory"),
        "spec must name nested field dual-track inventory API"
    );
    assert!(
        spec.contains("b09_objectref_nested_field_dual_track_first_slice_signed=true")
            || spec.contains("`b09_objectref_nested_field_dual_track_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("nested_field_write_path_dual_track_met=true")
            || spec.contains("`nested_field_write_path_dual_track_met=true`"))
            && (spec.contains("deep_field_graph_gc_walk_still_unmet=true")
                || spec.contains("`deep_field_graph_gc_walk_still_unmet=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`")),
        "spec must cite write-path met, deep walk unmet, migrate false"
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
            && (spec.contains("migrate") || spec.contains("B-09") || spec.contains("deep")),
        "spec Frozen must fence migrate flip / deep walk / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_objectref_nested_field_dual_track_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_objectref_nested_field_dual_track_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_rc_migrate_readiness_refresh_thin_floor_ok);
    assert!(inv.vm_objectset_nested_value_dual_track_wired);
    assert!(inv.mir_run_objectset_nested_value_dual_track_wired);
    assert!(inv.nested_field_write_path_dual_track_met);
    assert!(inv.objectref_rc_storage_still_present);
    assert!(inv.rt_abi_object_handle_retain_release_drop_still_present);
    assert!(inv.dual_track_and_rc_storage_coexistence_noted);
    assert!(inv.deep_field_graph_gc_walk_still_unmet);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_objectref_nested_field_dual_track_first_slice_signed);
    let floor = syx_rt::gate_b09_objectref_rc_migrate_readiness_refresh_thin_inventory();
    assert!(floor.b09_objectref_rc_migrate_readiness_refresh_thin_signed);
    assert!(floor.nested_field_dual_track_still_unmet);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(!floor.b09_axis_exit_enabled);
}


