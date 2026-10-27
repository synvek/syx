//! ObjectRef MirOp/VM residual peel true-delete first-slice
//! MirOp/VM ObjectRef residual deleted; DictBuild dual-track kept; ≠ migrate ≠ [x].

#[test]
fn b09_objectref_mirop_vm_residual_peel_delete_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "crates/syx-mir/src/lib.rs",
        "crates/syx-vm/src/lib.rs",
        "crates/syx-mir-run/src/runtime.rs",
        "docs/gates/gate-b09-objectref-mirop-vm-residual-peel-delete-first-slice.md",
        "docs/gates/gate-b09-objectref-mirop-vm-residual-peel-readiness-thin.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 MirOp/VM residual peel true-delete requires evidence file: {path}"
        );
    }
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09ObjectrefMiropVmResidualPeelDeleteFirstSliceInventory")
            && rt.contains("gate_b09_objectref_mirop_vm_residual_peel_delete_first_slice_inventory")
            && rt.contains("ADR-888")
            && (rt.contains("b09_objectref_mirop_vm_residual_peel_delete_first_slice_signed: true")
                || rt.contains("b09_objectref_mirop_vm_residual_peel_delete_first_slice_signed:true")),
        "syx-rt must expose GateB09ObjectrefMiropVmResidualPeelDeleteFirstSliceInventory"
    );
    assert!(
        (rt.contains("mirop_vm_residual_peel_met: true")
            || rt.contains("mirop_vm_residual_peel_met:true"))
            && (rt.contains("mir_op_objectref_wrap_retain_release_deleted: true")
                || rt.contains("mir_op_objectref_wrap_retain_release_deleted:true"))
            && (rt.contains("vm_instruction_objectref_retain_release_deleted: true")
                || rt.contains("vm_instruction_objectref_retain_release_deleted:true"))
            && (rt.contains("dictbuild_objectref_dual_track_birth_kept: true")
                || rt.contains("dictbuild_objectref_dual_track_birth_kept:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false")),
        "inventory must document peel met, MirOp/VM deleted, DictBuild kept, migrate false"
    );
    let mir_ops = include_str!("../../../syx-mir/src/lib.rs");
    assert!(
        !mir_ops.contains("\n    ObjectRefWrap,")
            && !mir_ops.contains("\n    ObjectRefRetain,")
            && !mir_ops.contains("\n    ObjectRefRelease,"),
        "MirOp ObjectRef variants must be deleted (W544 peel)"
    );
    let vm = include_str!("../../../syx-vm/src/lib.rs");
    assert!(
        !vm.contains("Instruction::ObjectRefRetain =>")
            && !vm.contains("Instruction::ObjectRefRelease =>"),
        "VM ObjectRefRetain/Release must be deleted (W544 peel)"
    );
    let mir = include_str!("../../../syx-mir-run/src/runtime.rs");
    assert!(
        !mir.contains("MirOp::ObjectRefWrap =>")
            && !mir.contains("MirOp::ObjectRefRetain =>")
            && !mir.contains("fn object_ref_wrap"),
        "mir-run MirOp ObjectRef path + object_ref_wrap must be peeled (W544)"
    );
    assert!(
        mir.contains("object_ref_from_map") && mir.contains("maybe_track_"),
        "DictBuild ObjectRef dual-track birth must remain"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_objectref_mirop_vm_residual_peel_delete_first_slice_inventory")
            && lib.contains("GateB09ObjectrefMiropVmResidualPeelDeleteFirstSliceInventory"),
        "syx-rt lib.rs must re-export MirOp/VM residual peel delete inventory"
    );
}

#[test]
fn b09_objectref_mirop_vm_residual_peel_delete_first_slice_spec_signed() {
    let spec =
        include_str!("../../../../docs/gates/gate-b09-objectref-mirop-vm-residual-peel-delete-first-slice.md");
    assert!(
        spec.contains("ADR-888")
            && (spec.contains("MirOp") || spec.contains("residual") || spec.contains("peel")),
        "spec must cite ADR-888 MirOp/VM residual peel true-delete"
    );
    assert!(
        spec.contains("three-hundred-and-ninety-fifth")
            || spec.contains("three-hundred-and-ninety-fourth"),
        
        "spec must cite W544 fingerprint"
    );
    assert!(
        spec.contains("GateB09ObjectrefMiropVmResidualPeelDeleteFirstSliceInventory")
            && spec.contains("gate_b09_objectref_mirop_vm_residual_peel_delete_first_slice_inventory"),
        "spec must name MirOp/VM residual peel delete inventory API"
    );
    assert!(
        spec.contains("b09_objectref_mirop_vm_residual_peel_delete_first_slice_signed=true")
            || spec.contains("`b09_objectref_mirop_vm_residual_peel_delete_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("mirop_vm_residual_peel_met=true")
            || spec.contains("`mirop_vm_residual_peel_met=true`"))
            && (spec.contains("mir_op_objectref_wrap_retain_release_deleted=true")
                || spec.contains("`mir_op_objectref_wrap_retain_release_deleted=true`"))
            && (spec.contains("dictbuild_objectref_dual_track_birth_kept=true")
                || spec.contains("`dictbuild_objectref_dual_track_birth_kept=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`")),
        "spec must cite peel met, deleted, DictBuild kept, migrate false"
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
            && (spec.contains("migrate") || spec.contains("B-09")),
        "spec Frozen must fence migrate flip / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_objectref_mirop_vm_residual_peel_delete_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_objectref_mirop_vm_residual_peel_delete_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_mirop_vm_residual_peel_readiness_thin_floor_ok);
    assert!(inv.handle_table_mir_run_opcode_peel_met);
    assert!(inv.migrate_flip_blockers_refreshed_after_peel);
    assert!(inv.mir_op_objectref_wrap_retain_release_deleted);
    assert!(inv.vm_instruction_objectref_retain_release_deleted);
    assert!(inv.mirop_vm_residual_peel_met);
    assert!(inv.dictbuild_objectref_dual_track_birth_kept);
    assert!(inv.object_ref_wrap_helper_deleted);
    assert!(inv.dual_track_and_heap_coexistence_noted);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_objectref_mirop_vm_residual_peel_delete_first_slice_signed);
    let floor = syx_rt::gate_b09_objectref_mirop_vm_residual_peel_readiness_thin_inventory();
    assert!(floor.b09_objectref_mirop_vm_residual_peel_readiness_thin_signed);
    assert!(floor.mirop_vm_residual_peel_still_unmet);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(!floor.b09_axis_exit_enabled);
}

#[test]
fn b09_objectref_mirop_vm_residual_peel_delete_first_slice_fingerprint_and_adr() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W544")
            && (roadmap.contains("ADR-888")
                || roadmap.contains("MirOp")
                || roadmap.contains("residual")),
        "roadmap must cite W544 / ADR-888 MirOp/VM residual peel true-delete"
    );
    assert!(
        roadmap.contains("three-hundred-and-ninety-fifth")
            || roadmap.contains("three-hundred-and-ninety-fourth")
            || roadmap.contains("three-hundred-and-ninety-third"),
        
        "roadmap must cite W544 fingerprint"
    );
    let adr = include_str!("../../../../docs/adr/architecture-decisions.md");
    assert!(
        adr.contains("ADR-888") && adr.contains("W544"),
        "ADR must record ADR-888 / W544"
    );
    crate::matrix_fingerprint::assert_live_matrix_fingerprint();
}

#[test]
fn b09_objectref_mirop_vm_residual_peel_delete_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W544")
            && (roadmap.contains("ADR-888")
                || roadmap.contains("MirOp")
                || roadmap.contains("residual")),
        "roadmap must cite W544 / ADR-888 MirOp/VM residual peel true-delete"
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
}
