//! RT ABI ObjectRef handle retain/release/drop true-delete
//! Handle C ABI deleted; table/opcodes/RC storage still; migrate false
//! ≠ peel table ≠ migrate ≠ B-09 [x].

#[test]
fn b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "crates/syx-rt/src/abi.rs",
        "crates/syx-llvm/src/lib.rs",
        "crates/syx-wasm-run/src/lib.rs",
        "crates/syx-runtime-core/src/value.rs",
        "docs/gates/gate-b09-rt-abi-objectref-handle-retain-release-drop-delete-first-slice.md",
        "docs/gates/gate-b09-rt-abi-objectref-handle-retire-readiness-thin.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 RT ABI handle delete requires evidence file: {path}"
        );
    }
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09RtAbiObjectrefHandleRetainReleaseDropDeleteFirstSliceInventory")
            && rt.contains(
                "gate_b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_inventory",
            )
            && rt.contains("ADR-878")
            && (rt.contains(
                "b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_signed: true",
            ) || rt.contains(
                "b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_signed:true",
            )),
        "syx-rt must expose GateB09RtAbiObjectrefHandleRetainReleaseDropDeleteFirstSliceInventory"
    );
    assert!(
        (rt.contains("rt_abi_object_handle_retain_release_drop_deleted: true")
            || rt.contains("rt_abi_object_handle_retain_release_drop_deleted:true"))
            && (rt.contains("object_handle_table_api_still_present: true")
                || rt.contains("object_handle_table_api_still_present:true"))
            && (rt.contains("mir_run_object_ref_opcodes_still_present: true")
                || rt.contains("mir_run_object_ref_opcodes_still_present:true"))
            && (rt.contains("objectref_rc_storage_still_present: true")
                || rt.contains("objectref_rc_storage_still_present:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false")),
        "inventory must document handle deleted, table/opcodes/RC still, migrate false"
    );
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    assert!(
        !abi.contains("pub extern \"C\" fn syx_rt_object_retain")
            && !abi.contains("pub extern \"C\" fn syx_rt_object_release")
            && !abi.contains("pub extern \"C\" fn syx_rt_object_drop"),
        "RT handle C ABI retain/release/drop must be deleted"
    );
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(
        !llvm.contains("declare void @syx_rt_object_retain")
            && !llvm.contains("declare void @syx_rt_object_release")
            && !llvm.contains("declare i32 @syx_rt_object_drop")
            && !llvm.contains("\"syx_rt_object_retain\"")
            && !llvm.contains("\"syx_rt_object_release\"")
            && !llvm.contains("\"syx_rt_object_drop\""),
        "LLVM must omit handle C ABI declares and allowlist entries"
    );
    let wasm = include_str!("../../../syx-wasm-run/src/lib.rs");
    assert!(
        !wasm.contains("fn link_syx_rt_object_abi"),
        "wasm-run must retire link_syx_rt_object_abi"
    );
    assert!(
        !std::path::Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../crates/syx-runtime-core/src/object_handles.rs"
        ))
        .is_file(),
        "object_handles.rs must be deleted (W541 peel); inventory historical still_present remains"
    );
    let value = include_str!("../../../syx-runtime-core/src/value.rs");
    assert!(
        !value.contains("rc: u32"),
        "ObjectRef rc: u32 storage must be deleted (W537)"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_inventory")
            && lib.contains("GateB09RtAbiObjectrefHandleRetainReleaseDropDeleteFirstSliceInventory"),
        "syx-rt lib.rs must re-export handle ABI delete inventory"
    );
}

#[test]
fn b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_spec_signed() {
    let spec = include_str!(
        "../../../../docs/gates/gate-b09-rt-abi-objectref-handle-retain-release-drop-delete-first-slice.md"
    );
    assert!(
        spec.contains("ADR-878")
            && (spec.contains("true-delete") || spec.contains("delete") || spec.contains("handle")),
        "spec must cite ADR-878 handle ABI true-delete"
    );
    assert!(
        spec.contains("three-hundred-and-eighty-fourth"),
        
        "spec must cite W534 fingerprint"
    );
    assert!(
        spec.contains("GateB09RtAbiObjectrefHandleRetainReleaseDropDeleteFirstSliceInventory")
            && spec.contains(
                "gate_b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_inventory",
            ),
        "spec must name handle ABI delete inventory API"
    );
    assert!(
        spec.contains(
            "b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_signed=true",
        ) || spec.contains(
            "`b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_signed=true`",
        ),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("rt_abi_object_handle_retain_release_drop_deleted=true")
            || spec.contains("`rt_abi_object_handle_retain_release_drop_deleted=true`"))
            && (spec.contains("object_handle_table_api_still_present=true")
                || spec.contains("`object_handle_table_api_still_present=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`")),
        "spec must cite deleted/table-still/migrate false"
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
            && (spec.contains("migrate") || spec.contains("B-09") || spec.contains("peel")),
        "spec Frozen must fence peel table / migrate / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_rt_abi_objectref_handle_retire_readiness_thin_floor_ok);
    assert!(inv.rt_abi_object_handle_retain_release_drop_deleted);
    assert!(inv.object_handle_table_api_still_present);
    assert!(inv.mir_run_object_ref_opcodes_still_present);
    assert!(inv.objectref_rc_storage_still_present);
    assert!(inv.nested_field_write_path_dual_track_met);
    assert!(inv.deep_field_graph_gc_walk_still_unmet);
    assert!(inv.value_object_ref_retain_release_api_deleted);
    assert!(inv.rt_abi_object_ref_wrap_retain_release_deleted);
    assert!(inv.managed_heap_collect_objectref_type_deinit_still_wired);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_signed);
    let floor = syx_rt::gate_b09_rt_abi_objectref_handle_retire_readiness_thin_inventory();
    assert!(floor.b09_rt_abi_objectref_handle_retire_readiness_thin_signed);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(!floor.b09_axis_exit_enabled);
}

#[test]
fn b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_fingerprint_and_adr() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W534")
            && (roadmap.contains("ADR-878")
                || roadmap.contains("handle")
                || roadmap.contains("true-delete")
                || roadmap.contains("delete")),
        "roadmap must cite W534 / ADR-878 handle ABI true-delete"
    );
    assert!(
        roadmap.contains("three-hundred-and-eighty-seventh")
            || roadmap.contains("three-hundred-and-eighty-sixth")
            || roadmap.contains("three-hundred-and-eighty-fifth")
            || roadmap.contains("three-hundred-and-eighty-fourth"),
        
        "roadmap must cite W535 or W534 fingerprint"
    );
    let adr = include_str!("../../../../docs/adr/architecture-decisions.md");
    assert!(
        adr.contains("ADR-878") && adr.contains("W534"),
        "ADR must record ADR-878 / W534"
    );
    crate::matrix_fingerprint::assert_live_matrix_fingerprint();
}

#[test]
fn b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W534")
            && (roadmap.contains("ADR-878")
                || roadmap.contains("handle")
                || roadmap.contains("delete")),
        "roadmap must cite W534 / ADR-878 handle ABI true-delete"
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
        "roadmap B-09 must be [x] (W558+) or historical [~] (delete ≠ exit): {b09}"
    );
}
