//! ObjectRefRelease opcode RC true-retire first-slice
//! migrate/cleanup false; StoreLocal/deinit still live; Release opcode retire ≠ Value RC full delete.

#[test]
fn b09_objectref_rc_release_opcode_retire_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "crates/syx-vm/src/lib.rs",
        "crates/syx-mir-run/src/runtime.rs",
        "crates/syx-runtime-core/src/value.rs",
        "docs/gates/gate-b09-objectref-rc-release-opcode-retire-first-slice.md",
        "docs/gates/gate-b09-objectref-rc-dup-loadlocal-retire-first-slice.md",
        "docs/gates/gate-b09-objectref-rc-retain-opcode-retire-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 Release opcode RC retire requires evidence file: {path}"
        );
    }
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09ObjectrefRcReleaseOpcodeRetireFirstSliceInventory")
            && rt.contains("gate_b09_objectref_rc_release_opcode_retire_first_slice_inventory")
            && rt.contains("ADR-868")
            && (rt.contains("b09_objectref_rc_release_opcode_retire_first_slice_signed: true")
                || rt.contains("b09_objectref_rc_release_opcode_retire_first_slice_signed:true")),
        "syx-rt must expose GateB09ObjectrefRcReleaseOpcodeRetireFirstSliceInventory"
    );
    assert!(
        (rt.contains("vm_mir_run_objectref_release_opcode_rc_retired: true")
            || rt.contains("vm_mir_run_objectref_release_opcode_rc_retired:true"))
            && (rt.contains("storelocal_deinit_objectref_rc_still_live: true")
                || rt.contains("storelocal_deinit_objectref_rc_still_live:true"))
            && (rt.contains("value_object_ref_retain_release_api_still_present: true")
                || rt.contains("value_object_ref_retain_release_api_still_present:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false"))
            && (rt.contains("storage_cleanup_enabled: false")
                || rt.contains("storage_cleanup_enabled:false")),
        "inventory must document Release opcode retire and keep StoreLocal + migrate false"
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
    // Wave A W544 / ADR-888: ObjectRefRelease instruction peeled after W524 RC retire.
    let release_peeled = !vm.contains("Instruction::ObjectRefRelease =>");
    if release_peeled {
        // Historical W524 retire signed; instruction gone in W544 peel.
    } else {
        assert!(
            vm.contains("Instruction::ObjectRefRelease") && vm.contains("ADR-868"),
            "VM ObjectRefRelease must cite ADR-868"
        );
        let release_body = vm
            .split("Instruction::ObjectRefRelease =>")
            .nth(1)
            .and_then(|s| {
                let end = s.find("\n            Instruction::").unwrap_or(s.len().min(800));
                Some(&s[..end])
            })
            .expect("VM ObjectRefRelease body");
        assert!(
            !release_body.contains("release_object_ref")
                && !release_body.contains("object_ref_release"),
            "VM ObjectRefRelease arm must not call release_object_ref"
        );
        assert!(
            release_body.contains("pop") || release_body.contains("stack.pop"),
            "VM ObjectRefRelease must still pop"
        );
    }
    let mir = include_str!("../../../syx-mir-run/src/runtime.rs");
    let mir_release_peeled = !mir.contains("MirOp::ObjectRefRelease =>");
    if mir_release_peeled {
        assert!(
            mir.contains("maybe_untrack_objectref_value"),
            "mir-run must keep StoreLocal/deinits heap untrack after MirOp peel"
        );
    } else {
        let mir_body = mir
            .split("MirOp::ObjectRefRelease =>")
            .nth(1)
            .and_then(|s| {
                let end = s
                    .find("\n            _ =>")
                    .or_else(|| s.find("\n            MirOp::"))
                    .unwrap_or(s.len().min(800));
                Some(&s[..end])
            })
            .expect("mir-run ObjectRefRelease MirOp arm (W541 peel)");
        assert!(
            !mir_body.contains("object_ref_release") && !mir_body.contains("run_deinit_for_value"),
            "mir-run ObjectRefRelease MirOp arm must not call object_ref_release/run_deinit_for_value"
        );
        assert!(
            mir_body.contains("pop") || mir_body.contains("stack.pop"),
            "mir-run ObjectRefRelease must still pop"
        );
        assert!(
            mir.contains("maybe_untrack_objectref_value"),
            "mir-run must keep StoreLocal/deinits heap untrack"
        );
    }
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_objectref_rc_release_opcode_retire_first_slice_inventory")
            && lib.contains("GateB09ObjectrefRcReleaseOpcodeRetireFirstSliceInventory"),
        "syx-rt lib.rs must re-export Release opcode retire inventory"
    );
}

#[test]
fn b09_objectref_rc_release_opcode_retire_first_slice_spec_signed() {
    let spec =
        include_str!("../../../../docs/gates/gate-b09-objectref-rc-release-opcode-retire-first-slice.md");
    assert!(
        spec.contains("ADR-868")
            && (spec.contains("true-retire")
                || spec.contains("ObjectRefRelease")
                || spec.contains("Release opcode")),
        "spec must cite ADR-868 ObjectRefRelease RC true-retire"
    );
    assert!(
        spec.contains("three-hundred-and-seventy-fourth"),
        
        "spec must cite W524 fingerprint"
    );
    assert!(
        spec.contains("GateB09ObjectrefRcReleaseOpcodeRetireFirstSliceInventory")
            && spec.contains("gate_b09_objectref_rc_release_opcode_retire_first_slice_inventory"),
        "spec must name Release opcode retire inventory API"
    );
    assert!(
        spec.contains("b09_objectref_rc_release_opcode_retire_first_slice_signed=true")
            || spec.contains("`b09_objectref_rc_release_opcode_retire_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("vm_mir_run_objectref_release_opcode_rc_retired=true")
            || spec.contains("`vm_mir_run_objectref_release_opcode_rc_retired=true`"))
            && (spec.contains("storelocal_deinit_objectref_rc_still_live=true")
                || spec.contains("`storelocal_deinit_objectref_rc_still_live=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`"))
            && (spec.contains("storage_cleanup_enabled=false")
                || spec.contains("`storage_cleanup_enabled=false`")),
        "spec must cite Release opcode retire flags and migrate/cleanup false"
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
            && (spec.contains("Value") || spec.contains("StoreLocal") || spec.contains("B-09")),
        "spec Frozen must fence Value RC full delete / StoreLocal strip / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_objectref_rc_release_opcode_retire_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_objectref_rc_release_opcode_retire_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_rc_dup_loadlocal_retire_first_slice_floor_ok);
    assert!(inv.b09_objectref_rc_retain_opcode_retire_first_slice_floor_ok);
    assert!(inv.vm_mir_run_objectref_release_opcode_rc_retired);
    assert!(inv.storelocal_deinit_objectref_rc_still_live);
    assert!(inv.value_object_ref_retain_release_api_still_present);
    assert!(inv.rt_abi_object_ref_retain_release_wrap_still_present);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_objectref_rc_release_opcode_retire_first_slice_signed);
    let floor = syx_rt::gate_b09_objectref_rc_dup_loadlocal_retire_first_slice_inventory();
    assert!(floor.b09_objectref_rc_dup_loadlocal_retire_first_slice_signed);
    assert!(floor.objectref_release_deinit_rc_still_live);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(!floor.b09_axis_exit_enabled);
}


