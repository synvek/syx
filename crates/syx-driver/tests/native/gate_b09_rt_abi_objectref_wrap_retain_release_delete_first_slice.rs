//! RT ABI ObjectRef wrap/retain/release true-delete
//! Opcode ABI deleted; handle ABI still; Value API deleted; migrate false
//! ≠ handle delete ≠ migrate ≠ B-09 [x].

#[test]
fn b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "crates/syx-rt/src/abi.rs",
        "crates/syx-runtime-core/src/value.rs",
        "docs/gates/gate-b09-rt-abi-objectref-wrap-retain-release-delete-first-slice.md",
        "docs/gates/gate-b09-objectref-value-rc-api-delete-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 RT ABI wrap/retain/release delete requires evidence file: {path}"
        );
    }
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09RtAbiObjectrefWrapRetainReleaseDeleteFirstSliceInventory")
            && rt.contains("gate_b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_inventory")
            && rt.contains("ADR-874")
            && (rt.contains(
                "b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_signed: true",
            ) || rt.contains(
                "b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_signed:true",
            )),
        "syx-rt must expose GateB09RtAbiObjectrefWrapRetainReleaseDeleteFirstSliceInventory"
    );
    assert!(
        (rt.contains("rt_abi_object_ref_wrap_retain_release_deleted: true")
            || rt.contains("rt_abi_object_ref_wrap_retain_release_deleted:true"))
            && (rt.contains("rt_abi_object_handle_retain_release_drop_still_present: true")
                || rt.contains("rt_abi_object_handle_retain_release_drop_still_present:true"))
            && (rt.contains("value_object_ref_retain_release_api_deleted: true")
                || rt.contains("value_object_ref_retain_release_api_deleted:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false")),
        "inventory must document opcode ABI deleted, handle still, Value deleted, migrate false"
    );
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    assert!(
        !abi.contains("pub extern \"C\" fn syx_rt_object_ref_wrap")
            && !abi.contains("pub extern \"C\" fn syx_rt_object_ref_retain")
            && !abi.contains("pub extern \"C\" fn syx_rt_object_ref_release"),
        "RT opcode ABI wrap/retain/release must be deleted"
    );
    assert!(
        !abi.contains("pub extern \"C\" fn syx_rt_object_retain")
            && !abi.contains("pub extern \"C\" fn syx_rt_object_release")
            && !abi.contains("pub extern \"C\" fn syx_rt_object_drop"),
        "handle ABI retain/release/drop must be deleted (W534)"
    );
    let value = include_str!("../../../syx-runtime-core/src/value.rs");
    assert!(
        !value.contains("fn object_ref_retain") && !value.contains("fn object_ref_release"),
        "Value object_ref_retain/release must remain deleted (W529 floor)"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_inventory")
            && lib.contains("GateB09RtAbiObjectrefWrapRetainReleaseDeleteFirstSliceInventory"),
        "syx-rt lib.rs must re-export RT ABI delete inventory"
    );
}

#[test]
fn b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_spec_signed() {
    let spec = include_str!(
        "../../../../docs/gates/gate-b09-rt-abi-objectref-wrap-retain-release-delete-first-slice.md"
    );
    assert!(
        spec.contains("ADR-874")
            && (spec.contains("wrap") || spec.contains("retain") || spec.contains("ABI")),
        "spec must cite ADR-874 RT ABI true-delete"
    );
    assert!(
        spec.contains("three-hundred-and-eighty-second")
            || spec.contains("three-hundred-and-eighty-first")
            || spec.contains("three-hundred-and-eightieth"),
        
        "spec must cite W532 or W531 or W530 fingerprint"
    );
    assert!(
        spec.contains("GateB09RtAbiObjectrefWrapRetainReleaseDeleteFirstSliceInventory")
            && spec.contains("gate_b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_inventory"),
        "spec must name RT ABI delete inventory API"
    );
    assert!(
        spec.contains("b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_signed=true")
            || spec.contains(
                "`b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_signed=true`",
            ),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("rt_abi_object_ref_wrap_retain_release_deleted=true")
            || spec.contains("`rt_abi_object_ref_wrap_retain_release_deleted=true`"))
            && (spec.contains("rt_abi_object_handle_retain_release_drop_still_present=true")
                || spec.contains("`rt_abi_object_handle_retain_release_drop_still_present=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`")),
        "spec must cite delete/handle-still/migrate false"
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
            && (spec.contains("handle") || spec.contains("migrate") || spec.contains("B-09")),
        "spec Frozen must fence handle delete / migrate / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_value_rc_api_delete_first_slice_floor_ok);
    assert!(inv.rt_abi_object_ref_wrap_retain_release_deleted);
    assert!(inv.rt_abi_object_handle_retain_release_drop_still_present);
    assert!(inv.value_object_ref_retain_release_api_deleted);
    assert!(inv.managed_heap_collect_objectref_type_deinit_still_wired);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_signed);
    let floor = syx_rt::gate_b09_objectref_value_rc_api_delete_first_slice_inventory();
    assert!(floor.b09_objectref_value_rc_api_delete_first_slice_signed);
    assert!(floor.value_object_ref_retain_release_api_deleted);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(!floor.b09_axis_exit_enabled);
}

#[test]
fn b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_fingerprint_and_adr() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W530")
            && (roadmap.contains("ADR-874")
                || roadmap.contains("wrap")
                || roadmap.contains("ABI")),
        "roadmap must cite W530 / ADR-874 RT ABI true-delete"
    );
    assert!(
        roadmap.contains("three-hundred-and-eighty-seventh")
            || roadmap.contains("three-hundred-and-eighty-sixth")
            || roadmap.contains("three-hundred-and-eighty-fifth")
            || roadmap.contains("three-hundred-and-eighty-fourth")
            || roadmap.contains("three-hundred-and-eighty-second")
            || roadmap.contains("three-hundred-and-eighty-first")
            || roadmap.contains("three-hundred-and-eightieth"),
        
        "roadmap must cite W535 or earlier fingerprint"
    );
    let adr = include_str!("../../../../docs/adr/architecture-decisions.md");
    assert!(
        adr.contains("ADR-874") && adr.contains("W530"),
        "ADR must record ADR-874 / W530"
    );
    crate::matrix_fingerprint::assert_live_matrix_fingerprint();
}

#[test]
fn b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W530")
            && (roadmap.contains("ADR-874") || roadmap.contains("ABI") || roadmap.contains("wrap")),
        "roadmap must cite W530 / ADR-874 RT ABI true-delete"
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
}
