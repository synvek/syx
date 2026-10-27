//! Value ObjectRef RC API delete first-slice
//! Value retain/release deleted; handle drop self-contained; RT ABI still
//! migrate false; ≠ ABI true delete ≠ B-09 [x].

#[test]
fn b09_objectref_value_rc_api_delete_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "crates/syx-rt/src/abi.rs",
        "crates/syx-runtime-core/src/value.rs",
        "docs/gates/gate-b09-objectref-value-rc-api-delete-first-slice.md",
        "docs/gates/gate-b09-objectref-storelocal-deinits-rc-retire-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 Value RC API delete requires evidence file: {path}"
        );
    }
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09ObjectrefValueRcApiDeleteFirstSliceInventory")
            && rt.contains("gate_b09_objectref_value_rc_api_delete_first_slice_inventory")
            && rt.contains("ADR-873")
            && (rt.contains("b09_objectref_value_rc_api_delete_first_slice_signed: true")
                || rt.contains("b09_objectref_value_rc_api_delete_first_slice_signed:true")),
        "syx-rt must expose GateB09ObjectrefValueRcApiDeleteFirstSliceInventory"
    );
    assert!(
        (rt.contains("value_object_ref_retain_release_api_deleted: true")
            || rt.contains("value_object_ref_retain_release_api_deleted:true"))
            && (rt.contains("object_handle_drop_self_contained_rc: true")
                || rt.contains("object_handle_drop_self_contained_rc:true"))
            && (rt.contains("rt_abi_object_ref_retain_release_wrap_still_present: true")
                || rt.contains("rt_abi_object_ref_retain_release_wrap_still_present:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false")),
        "inventory must document Value API deleted, handle drop self-contained, RT ABI still, migrate false"
    );
    let value = include_str!("../../../syx-runtime-core/src/value.rs");
    assert!(
        !value.contains("fn object_ref_retain") && !value.contains("fn object_ref_release"),
        "Value object_ref_retain/release must be deleted"
    );
    // Wave A W541 / ADR-885: object_handles.rs peeled; W529 inventory historical
    // self-contained drop claim remains in narrative inventory nails.
    assert!(
        !std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../crates/syx-runtime-core/src/object_handles.rs"
        ))
        .is_file(),
        "object_handles.rs must be deleted (W541 peel)"
    );
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    let abi_deleted = !abi.contains("pub extern \"C\" fn syx_rt_object_ref_wrap")
        && !abi.contains("pub extern \"C\" fn syx_rt_object_ref_retain")
        && !abi.contains("pub extern \"C\" fn syx_rt_object_ref_release");
    let historical = rt.contains("rt_abi_object_ref_retain_release_wrap_still_present: true")
        || rt.contains("rt_abi_object_ref_retain_release_wrap_still_present:true");
    assert!(
        abi_deleted || historical,
        "RT opcode ABI wrap/retain/release must be deleted (W530) or historical inventory still claims present"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_objectref_value_rc_api_delete_first_slice_inventory")
            && lib.contains("GateB09ObjectrefValueRcApiDeleteFirstSliceInventory"),
        "syx-rt lib.rs must re-export Value RC API delete inventory"
    );
}

#[test]
fn b09_objectref_value_rc_api_delete_first_slice_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b09-objectref-value-rc-api-delete-first-slice.md");
    assert!(
        spec.contains("ADR-873")
            && (spec.contains("Value") || spec.contains("object_ref_retain") || spec.contains("API")),
        "spec must cite ADR-873 Value RC API delete"
    );
    assert!(
        spec.contains("three-hundred-and-eightieth")
            || spec.contains("three-hundred-and-seventy-ninth"),
        
        "spec must cite W530 or W529 fingerprint"
    );
    assert!(
        spec.contains("GateB09ObjectrefValueRcApiDeleteFirstSliceInventory")
            && spec.contains("gate_b09_objectref_value_rc_api_delete_first_slice_inventory"),
        "spec must name Value RC API delete inventory API"
    );
    assert!(
        spec.contains("b09_objectref_value_rc_api_delete_first_slice_signed=true")
            || spec.contains("`b09_objectref_value_rc_api_delete_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("value_object_ref_retain_release_api_deleted=true")
            || spec.contains("`value_object_ref_retain_release_api_deleted=true`"))
            && (spec.contains("object_handle_drop_self_contained_rc=true")
                || spec.contains("`object_handle_drop_self_contained_rc=true`"))
            && (spec.contains("rt_abi_object_ref_retain_release_wrap_still_present=true")
                || spec.contains("`rt_abi_object_ref_retain_release_wrap_still_present=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`")),
        "spec must cite delete/self-contained/ABI still/migrate false"
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
            && (spec.contains("ABI") || spec.contains("B-09") || spec.contains("migrate")),
        "spec Frozen must fence ABI delete / migrate / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_objectref_value_rc_api_delete_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_objectref_value_rc_api_delete_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_storelocal_deinits_rc_retire_first_slice_floor_ok);
    assert!(inv.value_object_ref_retain_release_api_deleted);
    assert!(inv.object_handle_drop_self_contained_rc);
    assert!(inv.rt_abi_object_ref_retain_release_wrap_still_present);
    assert!(inv.managed_heap_collect_objectref_type_deinit_still_wired);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_objectref_value_rc_api_delete_first_slice_signed);
    let floor = syx_rt::gate_b09_objectref_storelocal_deinits_rc_retire_first_slice_inventory();
    assert!(floor.b09_objectref_storelocal_deinits_rc_retire_first_slice_signed);
    assert!(floor.vm_mir_run_storelocal_objectref_rc_retired);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(!floor.b09_axis_exit_enabled);
}

#[test]
fn b09_objectref_value_rc_api_delete_first_slice_fingerprint_and_adr() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W529")
            && (roadmap.contains("ADR-873")
                || roadmap.contains("Value")
                || roadmap.contains("object_ref")),
        "roadmap must cite W529 / ADR-873 Value RC API delete"
    );
    assert!(
        roadmap.contains("three-hundred-and-eightieth")
            || roadmap.contains("three-hundred-and-seventy-ninth"),
        
        "roadmap must cite W530 or W529 fingerprint"
    );
    let adr = include_str!("../../../../docs/adr/architecture-decisions.md");
    assert!(
        adr.contains("ADR-873") && adr.contains("W529"),
        "ADR must record ADR-873 / W529"
    );
    crate::matrix_fingerprint::assert_live_matrix_fingerprint();
}

#[test]
fn b09_objectref_value_rc_api_delete_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W529")
            && (roadmap.contains("ADR-873") || roadmap.contains("Value") || roadmap.contains("API")),
        "roadmap must cite W529 / ADR-873 Value RC API delete"
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
        "roadmap B-09 must be [x] (W558+) or historical [~]: {b09}"
    );
    assert!(
        !roadmap.contains("objectref_rc_migrate_enabled=true")
            || roadmap.contains("objectref_rc_migrate_enabled=false"),
        "roadmap must not claim migrate flipped true without honesty false"
    );
}
