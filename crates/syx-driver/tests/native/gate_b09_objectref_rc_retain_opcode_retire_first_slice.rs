//! ObjectRefRetain opcode RC true-retire first-slice
//! migrate/cleanup false; Dup/LoadLocal RC still live; opcode retire ≠ Value RC full delete.

#[test]
fn b09_objectref_rc_retain_opcode_retire_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "crates/syx-vm/src/lib.rs",
        "crates/syx-mir-run/src/runtime.rs",
        "crates/syx-runtime-core/src/value.rs",
        "docs/gates/gate-b09-objectref-rc-retain-opcode-retire-first-slice.md",
        "docs/gates/gate-b09-objectref-rc-retire-readiness-thin.md",
        "docs/gates/gate-b09-objectref-retain-catch-all-heap-slots-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 retain opcode RC retire requires evidence file: {path}"
        );
    }
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09ObjectrefRcRetainOpcodeRetireFirstSliceInventory")
            && rt.contains("gate_b09_objectref_rc_retain_opcode_retire_first_slice_inventory")
            && rt.contains("ADR-866")
            && (rt.contains("b09_objectref_rc_retain_opcode_retire_first_slice_signed: true")
                || rt.contains("b09_objectref_rc_retain_opcode_retire_first_slice_signed:true")),
        "syx-rt must expose GateB09ObjectrefRcRetainOpcodeRetireFirstSliceInventory"
    );
    assert!(
        (rt.contains("vm_mir_run_objectref_retain_opcode_rc_retired: true")
            || rt.contains("vm_mir_run_objectref_retain_opcode_rc_retired:true"))
            && (rt.contains("vm_mir_run_objectref_retain_opcode_dual_track_kept: true")
                || rt.contains("vm_mir_run_objectref_retain_opcode_dual_track_kept:true"))
            && (rt.contains("dup_loadlocal_objectref_rc_still_live: true")
                || rt.contains("dup_loadlocal_objectref_rc_still_live:true"))
            && (rt.contains("objectref_release_deinit_rc_still_live: true")
                || rt.contains("objectref_release_deinit_rc_still_live:true"))
            && (rt.contains("value_object_ref_retain_release_api_still_present: true")
                || rt.contains("value_object_ref_retain_release_api_still_present:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false"))
            && (rt.contains("storage_cleanup_enabled: false")
                || rt.contains("storage_cleanup_enabled:false")),
        "inventory must document opcode retire and keep Dup/LoadLocal + migrate false"
    );
    let value = include_str!("../../../syx-runtime-core/src/value.rs");
    let value_api_deleted = !value.contains("fn object_ref_retain") && !value.contains("fn object_ref_release");
    let historical_api_claim = rt.contains("value_object_ref_retain_release_api_still_present: true")
        || rt.contains("value_object_ref_retain_release_api_still_present:true");
    assert!(
        value_api_deleted || historical_api_claim,
        "Value object_ref_retain/release must be deleted (W529+) or historical inventory may still claim present"
    );
    let vm = include_str!("../../../syx-vm/src/lib.rs");
    // Wave A W544 / ADR-888: ObjectRefRetain instruction peeled after W522 RC retire.
    let retain_peeled = !vm.contains("Instruction::ObjectRefRetain =>");
    if retain_peeled {
        assert!(
            !vm.contains("fn maybe_track_stack_top_objectref") /* W550 peel */,
            "VM dual-track helpers must remain after ObjectRefRetain peel"
        );
    } else {
        assert!(
            vm.contains("Instruction::ObjectRefRetain")
                && vm.contains("ADR-866")
                && !vm.contains("fn maybe_track_stack_top_objectref") /* W550 peel */,
            "VM ObjectRefRetain must cite ADR-866 and keep dual-track"
        );
        let retain_arm = vm
            .split("Instruction::ObjectRefRetain =>")
            .nth(1)
            .and_then(|s| s.split("Instruction::ObjectRefRelease =>").next())
            .expect("VM ObjectRefRetain arm");
        assert!(
            !retain_arm.contains("object_ref_retain"),
            "VM ObjectRefRetain arm must not call object_ref_retain"
        );
    }
    let mir = include_str!("../../../syx-mir-run/src/runtime.rs");
    let mir_retain_peeled = !mir.contains("MirOp::ObjectRefRetain =>");
    if mir_retain_peeled {
        assert!(
            mir.contains("maybe_untrack_objectref_value"),
            "mir-run must keep StoreLocal/deinits heap untrack after MirOp peel"
        );
    } else {
        let mir_arm = mir
            .split("MirOp::ObjectRefRetain =>")
            .nth(1)
            .and_then(|s| s.split("MirOp::ObjectRefRelease =>").next())
            .expect("mir-run ObjectRefRetain MirOp arm (W541 peel)");
        assert!(
            !mir_arm.contains("object_ref_retain"),
            "mir-run ObjectRefRetain MirOp arm must not call object_ref_retain"
        );
        assert!(
            mir.contains("maybe_untrack_objectref_value"),
            "mir-run must keep StoreLocal/deinits heap untrack"
        );
    }
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_objectref_rc_retain_opcode_retire_first_slice_inventory")
            && lib.contains("GateB09ObjectrefRcRetainOpcodeRetireFirstSliceInventory"),
        "syx-rt lib.rs must re-export retain opcode retire inventory"
    );
}

#[test]
fn b09_objectref_rc_retain_opcode_retire_first_slice_spec_signed() {
    let spec =
        include_str!("../../../../docs/gates/gate-b09-objectref-rc-retain-opcode-retire-first-slice.md");
    assert!(
        spec.contains("ADR-866")
            && (spec.contains("true-retire")
                || spec.contains("opcode RC")
                || spec.contains("ObjectRefRetain opcode")),
        "spec must cite ADR-866 ObjectRefRetain opcode RC true-retire"
    );
    assert!(
        spec.contains("three-hundred-and-seventy-second"),
        
        "spec must cite W522 fingerprint"
    );
    assert!(
        spec.contains("GateB09ObjectrefRcRetainOpcodeRetireFirstSliceInventory")
            && spec.contains("gate_b09_objectref_rc_retain_opcode_retire_first_slice_inventory"),
        "spec must name retain opcode retire inventory API"
    );
    assert!(
        spec.contains("b09_objectref_rc_retain_opcode_retire_first_slice_signed=true")
            || spec.contains("`b09_objectref_rc_retain_opcode_retire_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("vm_mir_run_objectref_retain_opcode_rc_retired=true")
            || spec.contains("`vm_mir_run_objectref_retain_opcode_rc_retired=true`"))
            && (spec.contains("dup_loadlocal_objectref_rc_still_live=true")
                || spec.contains("`dup_loadlocal_objectref_rc_still_live=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`"))
            && (spec.contains("storage_cleanup_enabled=false")
                || spec.contains("`storage_cleanup_enabled=false`")),
        "spec must cite opcode retire flags and migrate/cleanup false"
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
            && (spec.contains("Value") || spec.contains("Dup") || spec.contains("B-09")),
        "spec Frozen must fence Value RC full delete / Dup strip / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_objectref_rc_retain_opcode_retire_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_objectref_rc_retain_opcode_retire_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_rc_retire_readiness_thin_floor_ok);
    assert!(inv.vm_mir_run_objectref_retain_opcode_rc_retired);
    assert!(inv.vm_mir_run_objectref_retain_opcode_dual_track_kept);
    assert!(inv.dup_loadlocal_objectref_rc_still_live);
    assert!(inv.objectref_release_deinit_rc_still_live);
    assert!(inv.value_object_ref_retain_release_api_still_present);
    assert!(inv.rt_abi_object_ref_retain_release_wrap_still_present);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_objectref_rc_retain_opcode_retire_first_slice_signed);
    let floor = syx_rt::gate_b09_objectref_rc_retire_readiness_thin_inventory();
    assert!(floor.b09_objectref_rc_retire_readiness_thin_signed);
    assert!(floor.value_vm_mir_run_objectref_rc_still_live);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(!floor.b09_axis_exit_enabled);
}

#[test]
fn b09_objectref_rc_retain_opcode_retire_first_slice_fingerprint_and_adr() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W522")
            && (roadmap.contains("ADR-866")
                || roadmap.contains("retain-opcode")
                || roadmap.contains("ObjectRefRetain opcode")
                || roadmap.contains("objectref_rc_retain_opcode")),
        "roadmap must cite W522 / ADR-866 ObjectRefRetain opcode RC true-retire"
    );
    assert!(
        roadmap.contains("three-hundred-and-seventy-second"),
        
        "roadmap must cite W522 fingerprint"
    );
    let adr = include_str!("../../../../docs/adr/architecture-decisions.md");
    assert!(
        adr.contains("ADR-866") && adr.contains("W522"),
        "ADR must record ADR-866 / W522"
    );
    crate::matrix_fingerprint::assert_live_matrix_fingerprint();
}

#[test]
fn b09_objectref_rc_retain_opcode_retire_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W522")
            && (roadmap.contains("ADR-866")
                || roadmap.contains("true-retire")
                || roadmap.contains("ObjectRefRetain")),
        "roadmap must cite W522 / ADR-866 ObjectRefRetain opcode RC true-retire"
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
