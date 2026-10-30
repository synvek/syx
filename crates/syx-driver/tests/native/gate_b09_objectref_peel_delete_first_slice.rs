//! ObjectRef peel 表·opcodes true-delete first-slice
//! Table + mir-run opcodes deleted; MirOp/VM still present; ≠ migrate ≠ [x].

#[test]
fn b09_objectref_peel_delete_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "crates/syx-runtime-core/src/value.rs",
        "crates/syx-mir-run/src/op.rs",
        "crates/syx-mir-run/src/runtime.rs",
        "crates/syx-mir/src/lib.rs",
        "crates/syx-vm/src/lib.rs",
        "docs/gates/gate-b09-objectref-peel-delete-first-slice.md",
        "docs/gates/gate-b09-objectref-peel-readiness-thin.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 peel delete requires evidence file: {path}"
        );
    }
    assert!(
        !std::path::Path::new(&format!("{root}crates/syx-runtime-core/src/object_handles.rs"))
            .is_file(),
        "object_handles.rs must be deleted"
    );
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09ObjectrefPeelDeleteFirstSliceInventory")
            && rt.contains("gate_b09_objectref_peel_delete_first_slice_inventory")
            && rt.contains("ADR-885")
            && (rt.contains("b09_objectref_peel_delete_first_slice_signed: true")
                || rt.contains("b09_objectref_peel_delete_first_slice_signed:true")),
        "syx-rt must expose GateB09ObjectrefPeelDeleteFirstSliceInventory"
    );
    assert!(
        (rt.contains("object_handle_table_api_deleted: true")
            || rt.contains("object_handle_table_api_deleted:true"))
            && (rt.contains("mir_run_object_ref_opcodes_deleted: true")
                || rt.contains("mir_run_object_ref_opcodes_deleted:true"))
            && (rt.contains("handle_table_mir_run_opcode_peel_met: true")
                || rt.contains("handle_table_mir_run_opcode_peel_met:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false")),
        "inventory must document peel met, migrate false"
    );
    let op = include_str!("../../../syx-mir-run/src/op.rs");
    assert!(
        !op.contains("pub const OBJECT_REF_WRAP")
            && !op.contains("pub const OBJECT_REF_RETAIN")
            && !op.contains("pub const OBJECT_REF_RELEASE"),
        "mir-run OBJECT_REF_* opcode constants must be deleted"
    );
    let mir = include_str!("../../../syx-mir-run/src/runtime.rs");
    assert!(
        !mir.contains("register_object_handle")
            && !mir.contains("MirOp::ObjectRefWrap =>")
            && !mir.contains("fn object_ref_wrap")
            && (mir.contains("ADR-885") || mir.contains("peel object_handle") || mir.contains("ADR-888")),
        "mir-run must peel handle registration and MirOp ObjectRef path (W544)"
    );
    let mir_ops = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        !mir_ops.contains("\n    ObjectRefWrap,")
            && !mir_ops.contains("\n    ObjectRefRetain,")
            && !mir_ops.contains("\n    ObjectRefRelease,"),
        "MirOp ObjectRef variants must be deleted (W544 peel); inventory historical still_present remains"
    );
    let vm = include_str!("../../../syx-vm/src/lib.rs");
    assert!(
        !vm.contains("Instruction::ObjectRefRetain =>")
            && !vm.contains("Instruction::ObjectRefRelease =>")
            && !vm.contains("ObjectRefRetain,\n")
            && !vm.contains("ObjectRefRelease,\n"),
        "VM ObjectRefRetain/Release must be deleted (W544 peel); inventory historical still_present remains"
    );
    let value = include_str!("../../../syx-runtime-core/src/value.rs");
    assert!(
        !value.contains("rc: u32") && !value.contains("release_object_ref_for_handle_drop"),
        "ObjectRef rc storage and handle-drop helper must remain deleted"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_objectref_peel_delete_first_slice_inventory")
            && lib.contains("GateB09ObjectrefPeelDeleteFirstSliceInventory"),
        "syx-rt lib.rs must re-export peel delete inventory"
    );
}

#[test]
fn b09_objectref_peel_delete_first_slice_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b09-objectref-peel-delete-first-slice.md");
    assert!(
        spec.contains("ADR-885") && (spec.contains("peel") || spec.contains("Peel")),
        "spec must cite ADR-885 peel true-delete"
    );
    assert!(
        spec.contains("three-hundred-and-ninety-first"),
        
        "spec must cite W541 fingerprint"
    );
    assert!(
        spec.contains("GateB09ObjectrefPeelDeleteFirstSliceInventory")
            && spec.contains("gate_b09_objectref_peel_delete_first_slice_inventory"),
        "spec must name peel delete inventory API"
    );
    assert!(
        spec.contains("b09_objectref_peel_delete_first_slice_signed=true")
            || spec.contains("`b09_objectref_peel_delete_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("object_handle_table_api_deleted=true")
            || spec.contains("`object_handle_table_api_deleted=true`"))
            && (spec.contains("handle_table_mir_run_opcode_peel_met=true")
                || spec.contains("`handle_table_mir_run_opcode_peel_met=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`")),
        "spec must cite peel met, migrate false"
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
            && (spec.contains("migrate") || spec.contains("B-09") || spec.contains("MirOp")),
        "spec Frozen must fence migrate flip / MirOp peel / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_objectref_peel_delete_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_objectref_peel_delete_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_peel_readiness_thin_floor_ok);
    assert!(inv.deep_field_graph_gc_walk_met);
    assert!(inv.objectref_rc_storage_deleted);
    assert!(inv.object_handle_table_api_deleted);
    assert!(inv.mir_run_object_ref_opcodes_deleted);
    assert!(inv.handle_table_mir_run_opcode_peel_met);
    assert!(inv.object_ref_wrap_without_handle_registration_wired);
    assert!(inv.run_class_deinits_no_object_handle_drop_wired);
    assert!(inv.release_object_ref_for_handle_drop_deleted);
    assert!(inv.mir_op_objectref_wrap_retain_release_still_present);
    assert!(inv.vm_instruction_objectref_retain_release_still_present);
    assert!(inv.dual_track_and_heap_coexistence_noted);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_objectref_peel_delete_first_slice_signed);
    let floor = syx_rt::gate_b09_objectref_peel_readiness_thin_inventory();
    assert!(floor.b09_objectref_peel_readiness_thin_signed);
    assert!(floor.handle_table_mir_run_opcode_peel_still_unmet);
    assert!(floor.object_handle_table_api_still_present);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(!floor.b09_axis_exit_enabled);
}


