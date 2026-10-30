//! run_class_deinits ObjectRef ManagedHeap unroot first-slice
//! migrate/cleanup false; StoreLocal/deinit RC still live; unroot ≠ RC true-retire.

#[test]
fn b09_objectref_deinits_heap_unroot_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "crates/syx-vm/src/lib.rs",
        "crates/syx-mir-run/src/runtime.rs",
        "crates/syx-runtime-core/src/value.rs",
        "docs/gates/gate-b09-objectref-deinits-heap-unroot-first-slice.md",
        "docs/gates/gate-b09-objectref-storelocal-heap-unroot-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 deinits heap unroot requires evidence file: {path}"
        );
    }
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09ObjectrefDeinitsHeapUnrootFirstSliceInventory")
            && rt.contains("gate_b09_objectref_deinits_heap_unroot_first_slice_inventory")
            && rt.contains("ADR-870")
            && (rt.contains("b09_objectref_deinits_heap_unroot_first_slice_signed: true")
                || rt.contains("b09_objectref_deinits_heap_unroot_first_slice_signed:true")),
        "syx-rt must expose GateB09ObjectrefDeinitsHeapUnrootFirstSliceInventory"
    );
    assert!(
        (rt.contains("vm_mir_run_deinits_objectref_heap_unroot_wired: true")
            || rt.contains("vm_mir_run_deinits_objectref_heap_unroot_wired:true"))
            && (rt.contains("storelocal_objectref_rc_still_live: true")
                || rt.contains("storelocal_objectref_rc_still_live:true"))
            && (rt.contains("run_class_deinits_objectref_rc_still_live: true")
                || rt.contains("run_class_deinits_objectref_rc_still_live:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false")),
        "inventory must document deinits unroot and keep RC + migrate false"
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
        vm.contains("run_class_deinits_if_enabled")
            && vm.contains("ADR-870")
            && vm.contains("maybe_untrack_objectref_value"),
        "VM deinits must cite ADR-870 and call maybe_untrack_objectref_value"
    );
    let deinit_body = vm
        .split("fn run_class_deinits_if_enabled")
        .nth(1)
        .and_then(|s| {
            let end = s.find("\n    fn ").unwrap_or(s.len().min(800));
            Some(&s[..end])
        })
        .expect("VM run_class_deinits_if_enabled body");
    assert!(
        deinit_body.contains("maybe_untrack_objectref_value")
            && !deinit_body.contains("release_object_ref("),
        "VM deinits must unroot; RC release may be retired (W528+)"
    );
    let mir = include_str!("../../../syx-mir-run/src/runtime.rs");
    let mir_body = mir
        .split("pub fn run_class_deinits")
        .nth(1)
        .and_then(|s| {
            let end = s.find("\n    fn ").unwrap_or(s.len().min(900));
            Some(&s[..end])
        })
        .expect("mir-run run_class_deinits body");
    assert!(
        mir_body.contains("maybe_untrack_objectref_value")
            && !mir_body.contains("object_ref_release("),
        "mir-run deinits must unroot; RC release may be retired (W528+)"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_objectref_deinits_heap_unroot_first_slice_inventory")
            && lib.contains("GateB09ObjectrefDeinitsHeapUnrootFirstSliceInventory"),
        "syx-rt lib.rs must re-export deinits heap unroot inventory"
    );
}

#[test]
fn b09_objectref_deinits_heap_unroot_first_slice_spec_signed() {
    let spec =
        include_str!("../../../../docs/gates/gate-b09-objectref-deinits-heap-unroot-first-slice.md");
    assert!(
        spec.contains("ADR-870")
            && (spec.contains("unroot") || spec.contains("deinits") || spec.contains("run_class_deinits")),
        "spec must cite ADR-870 deinits heap unroot"
    );
    assert!(
        spec.contains("three-hundred-and-seventy-sixth"),
        
        "spec must cite W526 fingerprint"
    );
    assert!(
        spec.contains("GateB09ObjectrefDeinitsHeapUnrootFirstSliceInventory")
            && spec.contains("gate_b09_objectref_deinits_heap_unroot_first_slice_inventory"),
        "spec must name deinits heap unroot inventory API"
    );
    assert!(
        spec.contains("b09_objectref_deinits_heap_unroot_first_slice_signed=true")
            || spec.contains("`b09_objectref_deinits_heap_unroot_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("vm_mir_run_deinits_objectref_heap_unroot_wired=true")
            || spec.contains("`vm_mir_run_deinits_objectref_heap_unroot_wired=true`"))
            && (spec.contains("run_class_deinits_objectref_rc_still_live=true")
                || spec.contains("`run_class_deinits_objectref_rc_still_live=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`")),
        "spec must cite deinits unroot flags and migrate false"
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
        "spec Frozen must fence RC retire / Value delete / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_objectref_deinits_heap_unroot_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_objectref_deinits_heap_unroot_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_storelocal_heap_unroot_first_slice_floor_ok);
    assert!(inv.vm_mir_run_deinits_objectref_heap_unroot_wired);
    assert!(inv.storelocal_objectref_rc_still_live);
    assert!(inv.run_class_deinits_objectref_rc_still_live);
    assert!(inv.value_object_ref_retain_release_api_still_present);
    assert!(inv.rt_abi_object_ref_retain_release_wrap_still_present);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_objectref_deinits_heap_unroot_first_slice_signed);
    let floor = syx_rt::gate_b09_objectref_storelocal_heap_unroot_first_slice_inventory();
    assert!(floor.b09_objectref_storelocal_heap_unroot_first_slice_signed);
    assert!(floor.run_class_deinits_objectref_rc_still_live);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(!floor.b09_axis_exit_enabled);
}


