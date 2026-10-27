//! Dup+LoadLocal ObjectRef RC true-retire first-slice
//! migrate/cleanup false; Release still live; Dup/LoadLocal retire ≠ Value RC full delete.

#[test]
fn b09_objectref_rc_dup_loadlocal_retire_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "crates/syx-vm/src/lib.rs",
        "crates/syx-mir-run/src/runtime.rs",
        "crates/syx-runtime-core/src/value.rs",
        "docs/gates/gate-b09-objectref-rc-dup-loadlocal-retire-first-slice.md",
        "docs/gates/gate-b09-objectref-rc-retain-opcode-retire-first-slice.md",
        "docs/gates/gate-b09-objectref-retain-catch-all-heap-slots-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 Dup/LoadLocal RC retire requires evidence file: {path}"
        );
    }
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09ObjectrefRcDupLoadlocalRetireFirstSliceInventory")
            && rt.contains("gate_b09_objectref_rc_dup_loadlocal_retire_first_slice_inventory")
            && rt.contains("ADR-867")
            && (rt.contains("b09_objectref_rc_dup_loadlocal_retire_first_slice_signed: true")
                || rt.contains("b09_objectref_rc_dup_loadlocal_retire_first_slice_signed:true")),
        "syx-rt must expose GateB09ObjectrefRcDupLoadlocalRetireFirstSliceInventory"
    );
    assert!(
        (rt.contains("vm_mir_run_dup_loadlocal_objectref_rc_retired: true")
            || rt.contains("vm_mir_run_dup_loadlocal_objectref_rc_retired:true"))
            && (rt.contains("vm_mir_run_dup_loadlocal_dual_track_kept: true")
                || rt.contains("vm_mir_run_dup_loadlocal_dual_track_kept:true"))
            && (rt.contains("objectref_release_deinit_rc_still_live: true")
                || rt.contains("objectref_release_deinit_rc_still_live:true"))
            && (rt.contains("value_object_ref_retain_release_api_still_present: true")
                || rt.contains("value_object_ref_retain_release_api_still_present:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false"))
            && (rt.contains("storage_cleanup_enabled: false")
                || rt.contains("storage_cleanup_enabled:false")),
        "inventory must document Dup/LoadLocal retire and keep Release + migrate false"
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
    assert!(
        vm.contains("Instruction::Dup")
            && vm.contains("Instruction::LoadLocal")
            && vm.contains("ADR-867")
            && !vm.contains("fn maybe_track_stack_top_objectref") /* W550 peel */,
        "VM Dup/LoadLocal must cite ADR-867 and keep dual-track"
    );
    let dup_arm = vm
        .split("Instruction::Dup =>")
        .nth(1)
        .and_then(|s| s.split("Instruction::LoadLocal").next())
        .expect("VM Dup arm");
    assert!(
        !dup_arm.contains("object_ref_retain"),
        "VM Dup arm must not call object_ref_retain"
    );
    let load_arm = vm
        .split("Instruction::LoadLocal")
        .nth(1)
        .and_then(|s| s.split("Instruction::StoreLocal").next())
        .expect("VM LoadLocal arm");
    assert!(
        !load_arm.contains("object_ref_retain"),
        "VM LoadLocal arm must not call object_ref_retain"
    );
    let mir = include_str!("../../../syx-mir-run/src/runtime.rs");
    let mir_load = mir
        .split("op::LOAD_LOCAL =>")
        .nth(1)
        .and_then(|s| s.split("op::STORE_LOCAL =>").next())
        .expect("mir-run LOAD_LOCAL arm");
    assert!(
        !mir_load.contains("object_ref_retain"),
        "mir-run LOAD_LOCAL arm must not call object_ref_retain"
    );
    let mir_dup = mir
        .split("op::DUP =>")
        .nth(1)
        .and_then(|s| s.split("op::CALL_HOST =>").next())
        .expect("mir-run DUP arm");
    assert!(
        !mir_dup.contains("object_ref_retain"),
        "mir-run DUP arm must not call object_ref_retain"
    );
    assert!(
        mir.contains("maybe_untrack_objectref_value"),
        "mir-run must keep StoreLocal/deinits heap untrack"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_objectref_rc_dup_loadlocal_retire_first_slice_inventory")
            && lib.contains("GateB09ObjectrefRcDupLoadlocalRetireFirstSliceInventory"),
        "syx-rt lib.rs must re-export Dup/LoadLocal retire inventory"
    );
}

#[test]
fn b09_objectref_rc_dup_loadlocal_retire_first_slice_spec_signed() {
    let spec =
        include_str!("../../../../docs/gates/gate-b09-objectref-rc-dup-loadlocal-retire-first-slice.md");
    assert!(
        spec.contains("ADR-867")
            && (spec.contains("true-retire")
                || spec.contains("Dup+LoadLocal")
                || spec.contains("Dup/LoadLocal")),
        "spec must cite ADR-867 Dup+LoadLocal RC true-retire"
    );
    assert!(
        spec.contains("three-hundred-and-seventy-third"),
        
        "spec must cite W523 fingerprint"
    );
    assert!(
        spec.contains("GateB09ObjectrefRcDupLoadlocalRetireFirstSliceInventory")
            && spec.contains("gate_b09_objectref_rc_dup_loadlocal_retire_first_slice_inventory"),
        "spec must name Dup/LoadLocal retire inventory API"
    );
    assert!(
        spec.contains("b09_objectref_rc_dup_loadlocal_retire_first_slice_signed=true")
            || spec.contains("`b09_objectref_rc_dup_loadlocal_retire_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("vm_mir_run_dup_loadlocal_objectref_rc_retired=true")
            || spec.contains("`vm_mir_run_dup_loadlocal_objectref_rc_retired=true`"))
            && (spec.contains("objectref_release_deinit_rc_still_live=true")
                || spec.contains("`objectref_release_deinit_rc_still_live=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`"))
            && (spec.contains("storage_cleanup_enabled=false")
                || spec.contains("`storage_cleanup_enabled=false`")),
        "spec must cite Dup/LoadLocal retire flags and migrate/cleanup false"
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
            && (spec.contains("Value") || spec.contains("Release") || spec.contains("B-09")),
        "spec Frozen must fence Value RC full delete / Release strip / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_objectref_rc_dup_loadlocal_retire_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_objectref_rc_dup_loadlocal_retire_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_rc_retain_opcode_retire_first_slice_floor_ok);
    assert!(inv.b09_objectref_ssa_loadlocal_heap_slots_first_slice_floor_ok);
    assert!(inv.vm_mir_run_dup_loadlocal_objectref_rc_retired);
    assert!(inv.vm_mir_run_dup_loadlocal_dual_track_kept);
    assert!(inv.objectref_release_deinit_rc_still_live);
    assert!(inv.value_object_ref_retain_release_api_still_present);
    assert!(inv.rt_abi_object_ref_retain_release_wrap_still_present);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_objectref_rc_dup_loadlocal_retire_first_slice_signed);
    let floor = syx_rt::gate_b09_objectref_rc_retain_opcode_retire_first_slice_inventory();
    assert!(floor.b09_objectref_rc_retain_opcode_retire_first_slice_signed);
    assert!(floor.objectref_release_deinit_rc_still_live);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(!floor.b09_axis_exit_enabled);
}

#[test]
fn b09_objectref_rc_dup_loadlocal_retire_first_slice_fingerprint_and_adr() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W523")
            && (roadmap.contains("ADR-867")
                || roadmap.contains("dup-loadlocal")
                || roadmap.contains("Dup+LoadLocal")
                || roadmap.contains("objectref_rc_dup_loadlocal")),
        "roadmap must cite W523 / ADR-867 Dup+LoadLocal RC true-retire"
    );
    assert!(
        roadmap.contains("three-hundred-and-seventy-third"),
        
        "roadmap must cite W523 fingerprint"
    );
    let adr = include_str!("../../../../docs/adr/architecture-decisions.md");
    assert!(
        adr.contains("ADR-867") && adr.contains("W523"),
        "ADR must record ADR-867 / W523"
    );
    crate::matrix_fingerprint::assert_live_matrix_fingerprint();
}

#[test]
fn b09_objectref_rc_dup_loadlocal_retire_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W523")
            && (roadmap.contains("ADR-867")
                || roadmap.contains("true-retire")
                || roadmap.contains("Dup+LoadLocal")
                || roadmap.contains("Dup/LoadLocal")),
        "roadmap must cite W523 / ADR-867 Dup+LoadLocal RC true-retire"
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
