//! StoreLocal ObjectRef ManagedHeap unroot first-slice
//! migrate/cleanup false; StoreLocal/deinit RC still live; unroot ≠ StoreLocal RC true-retire.

#[test]
fn b09_objectref_storelocal_heap_unroot_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "crates/syx-vm/src/lib.rs",
        "crates/syx-mir-run/src/runtime.rs",
        "crates/syx-runtime-core/src/value.rs",
        "crates/syx-runtime-core/src/managed_heap.rs",
        "docs/gates/gate-b09-objectref-storelocal-heap-unroot-first-slice.md",
        "docs/gates/gate-b09-objectref-rc-release-opcode-retire-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 StoreLocal heap unroot requires evidence file: {path}"
        );
    }
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09ObjectrefStorelocalHeapUnrootFirstSliceInventory")
            && rt.contains("gate_b09_objectref_storelocal_heap_unroot_first_slice_inventory")
            && rt.contains("ADR-869")
            && (rt.contains("b09_objectref_storelocal_heap_unroot_first_slice_signed: true")
                || rt.contains("b09_objectref_storelocal_heap_unroot_first_slice_signed:true")),
        "syx-rt must expose GateB09ObjectrefStorelocalHeapUnrootFirstSliceInventory"
    );
    assert!(
        (rt.contains("vm_mir_run_storelocal_objectref_heap_unroot_wired: true")
            || rt.contains("vm_mir_run_storelocal_objectref_heap_unroot_wired:true"))
            && (rt.contains("storelocal_objectref_rc_still_live: true")
                || rt.contains("storelocal_objectref_rc_still_live:true"))
            && (rt.contains("run_class_deinits_objectref_rc_still_live: true")
                || rt.contains("run_class_deinits_objectref_rc_still_live:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false"))
            && (rt.contains("storage_cleanup_enabled: false")
                || rt.contains("storage_cleanup_enabled:false")),
        "inventory must document StoreLocal unroot and keep RC + migrate false"
    );
    let value = include_str!("../../../syx-runtime-core/src/value.rs");
    let value_api_deleted = !value.contains("fn object_ref_retain") && !value.contains("fn object_ref_release");
    let historical_api_claim = rt.contains("value_object_ref_retain_release_api_still_present: true")
        || rt.contains("value_object_ref_retain_release_api_still_present:true");
    assert!(
        value_api_deleted || historical_api_claim,
        "Value object_ref_retain/release must be deleted (W529+) or historical inventory may still claim present"
    );
    let heap = include_str!("../../../syx-runtime-core/src/managed_heap.rs");
    assert!(
        heap.contains("fn free_slot") && heap.contains("ADR-869"),
        "ManagedHeap must expose free_slot for StoreLocal death-path"
    );
    let vm = include_str!("../../../syx-vm/src/lib.rs");
    assert!(
        vm.contains("Instruction::StoreLocal")
            && vm.contains("ADR-869")
            && vm.contains("maybe_untrack_objectref_value"),
        "VM StoreLocal must cite ADR-869 and call maybe_untrack_objectref_value"
    );
    let store_body = vm
        .split("Instruction::StoreLocal")
        .nth(1)
        .and_then(|s| {
            let end = s.find("\n            Instruction::").unwrap_or(s.len().min(1200));
            Some(&s[..end])
        })
        .expect("VM StoreLocal body");
    assert!(
        store_body.contains("maybe_untrack_objectref_value")
            && !store_body.contains("release_object_ref(")
            && !store_body.contains("object_ref_release("),
        "VM StoreLocal must unroot; RC release may be retired (W528+)"
    );
    assert!(
        vm.contains("run_class_deinits") && vm.contains("maybe_untrack_objectref_value"),
        "VM deinits path must keep untrack"
    );
    let mir = include_str!("../../../syx-mir-run/src/runtime.rs");
    let mir_body = mir
        .split("op::STORE_LOCAL =>")
        .nth(1)
        .and_then(|s| {
            let end = s.find("\n            op::").unwrap_or(s.len().min(1200));
            Some(&s[..end])
        })
        .expect("mir-run STORE_LOCAL body");
    assert!(
        mir_body.contains("maybe_untrack_objectref_value")
            && !mir_body.contains("object_ref_release("),
        "mir-run STORE_LOCAL must unroot; RC release may be retired (W528+)"
    );
    assert!(
        mir.contains("run_class_deinits") && mir.contains("maybe_untrack_objectref_value"),
        "mir-run deinits must keep untrack"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_objectref_storelocal_heap_unroot_first_slice_inventory")
            && lib.contains("GateB09ObjectrefStorelocalHeapUnrootFirstSliceInventory"),
        "syx-rt lib.rs must re-export StoreLocal heap unroot inventory"
    );
}

#[test]
fn b09_objectref_storelocal_heap_unroot_first_slice_spec_signed() {
    let spec =
        include_str!("../../../../docs/gates/gate-b09-objectref-storelocal-heap-unroot-first-slice.md");
    assert!(
        spec.contains("ADR-869")
            && (spec.contains("unroot") || spec.contains("StoreLocal") || spec.contains("heap_slots")),
        "spec must cite ADR-869 StoreLocal heap unroot"
    );
    assert!(
        spec.contains("three-hundred-and-seventy-fifth"),
        
        "spec must cite W525 fingerprint"
    );
    assert!(
        spec.contains("GateB09ObjectrefStorelocalHeapUnrootFirstSliceInventory")
            && spec.contains("gate_b09_objectref_storelocal_heap_unroot_first_slice_inventory"),
        "spec must name StoreLocal heap unroot inventory API"
    );
    assert!(
        spec.contains("b09_objectref_storelocal_heap_unroot_first_slice_signed=true")
            || spec.contains("`b09_objectref_storelocal_heap_unroot_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("vm_mir_run_storelocal_objectref_heap_unroot_wired=true")
            || spec.contains("`vm_mir_run_storelocal_objectref_heap_unroot_wired=true`"))
            && (spec.contains("storelocal_objectref_rc_still_live=true")
                || spec.contains("`storelocal_objectref_rc_still_live=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`"))
            && (spec.contains("storage_cleanup_enabled=false")
                || spec.contains("`storage_cleanup_enabled=false`")),
        "spec must cite StoreLocal unroot flags and migrate/cleanup false"
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
            && (spec.contains("StoreLocal") || spec.contains("Value") || spec.contains("B-09")),
        "spec Frozen must fence StoreLocal RC retire / Value delete / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_objectref_storelocal_heap_unroot_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_objectref_storelocal_heap_unroot_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_rc_release_opcode_retire_first_slice_floor_ok);
    assert!(inv.vm_mir_run_storelocal_objectref_heap_unroot_wired);
    assert!(inv.storelocal_objectref_rc_still_live);
    assert!(inv.run_class_deinits_objectref_rc_still_live);
    assert!(inv.value_object_ref_retain_release_api_still_present);
    assert!(inv.rt_abi_object_ref_retain_release_wrap_still_present);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_objectref_storelocal_heap_unroot_first_slice_signed);
    let floor = syx_rt::gate_b09_objectref_rc_release_opcode_retire_first_slice_inventory();
    assert!(floor.b09_objectref_rc_release_opcode_retire_first_slice_signed);
    assert!(floor.storelocal_deinit_objectref_rc_still_live);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(!floor.b09_axis_exit_enabled);
}

#[test]
fn b09_objectref_storelocal_heap_unroot_first_slice_fingerprint_and_adr() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W525")
            && (roadmap.contains("ADR-869")
                || roadmap.contains("storelocal")
                || roadmap.contains("StoreLocal")
                || roadmap.contains("heap_unroot")
                || roadmap.contains("unroot")),
        "roadmap must cite W525 / ADR-869 StoreLocal heap unroot"
    );
    assert!(
        roadmap.contains("three-hundred-and-seventy-fifth"),
        
        "roadmap must cite W525 fingerprint"
    );
    let adr = include_str!("../../../../docs/adr/architecture-decisions.md");
    assert!(
        adr.contains("ADR-869") && adr.contains("W525"),
        "ADR must record ADR-869 / W525"
    );
    crate::matrix_fingerprint::assert_live_matrix_fingerprint();
}

#[test]
fn b09_objectref_storelocal_heap_unroot_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W525")
            && (roadmap.contains("ADR-869")
                || roadmap.contains("unroot")
                || roadmap.contains("StoreLocal")),
        "roadmap must cite W525 / ADR-869 StoreLocal heap unroot"
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
