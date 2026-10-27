//! GC↔Type_deinit sweep-finalizer first-slice
//! migrate/cleanup false; StoreLocal/deinit RC still live; sweep ≠ RC true-retire.

#[test]
fn b09_gc_type_deinit_sweep_bridge_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "crates/syx-vm/src/lib.rs",
        "crates/syx-mir-run/src/runtime.rs",
        "crates/syx-runtime-core/src/managed_heap.rs",
        "crates/syx-runtime-core/src/value.rs",
        "docs/gates/gate-b09-gc-type-deinit-sweep-bridge-first-slice.md",
        "docs/gates/gate-b09-objectref-deinits-heap-unroot-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 sweep Type_deinit bridge requires evidence file: {path}"
        );
    }
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09GcTypeDeinitSweepBridgeFirstSliceInventory")
            && rt.contains("gate_b09_gc_type_deinit_sweep_bridge_first_slice_inventory")
            && rt.contains("ADR-871")
            && (rt.contains("b09_gc_type_deinit_sweep_bridge_first_slice_signed: true")
                || rt.contains("b09_gc_type_deinit_sweep_bridge_first_slice_signed:true")),
        "syx-rt must expose GateB09GcTypeDeinitSweepBridgeFirstSliceInventory"
    );
    assert!(
        (rt.contains("managed_heap_collect_objectref_type_deinit_wired: true")
            || rt.contains("managed_heap_collect_objectref_type_deinit_wired:true"))
            && (rt.contains("free_slot_still_skips_type_deinit: true")
                || rt.contains("free_slot_still_skips_type_deinit:true"))
            && (rt.contains("storelocal_objectref_rc_still_live: true")
                || rt.contains("storelocal_objectref_rc_still_live:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false")),
        "inventory must document sweep Type_deinit and keep RC + migrate false"
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
        heap.contains("ADR-871")
            && heap.contains("Does **not** run Type_deinit")
            && heap.contains("fn collect_mark_sweep"),
        "ManagedHeap must cite ADR-871 and keep free_slot skip Type_deinit"
    );
    let vm = include_str!("../../../syx-vm/src/lib.rs");
    assert!(
        vm.contains("run_gc_collect")
            && vm.contains("ADR-871")
            && vm.contains("run_deinit_for_value")
            && vm.contains("object_ref_is_weak_dead"),
        "VM GcCollect must cite ADR-871 and run Type_deinit on swept ObjectRef"
    );
    let gc_body = vm
        .split("pub fn run_gc_collect")
        .nth(1)
        .and_then(|s| {
            let end = s.find("\n    pub fn ").unwrap_or(s.len().min(900));
            Some(&s[..end])
        })
        .expect("VM run_gc_collect body");
    assert!(
        gc_body.contains("collect_global")
            && gc_body.contains("run_deinit_for_value")
            && !gc_body.contains("collect_mark_sweep"),
        "VM run_gc_collect must use collect_global swept list, not call collect_mark_sweep"
    );
    assert!(
        vm.contains("maybe_untrack_objectref_value"),
        "VM must keep maybe_untrack_objectref_value (heap unroot floor)"
    );
    let mir = include_str!("../../../syx-mir-run/src/runtime.rs");
    let mir_body = mir
        .split("pub fn heap_collect")
        .nth(1)
        .and_then(|s| {
            let end = s.find("\n    fn ").unwrap_or(s.len().min(700));
            Some(&s[..end])
        })
        .expect("mir-run heap_collect body");
    assert!(
        mir_body.contains("collect_global")
            && (mir_body.contains("run_sweep_objectref_type_deinits")
                || mir_body.contains("run_deinit_for_value")),
        "mir-run heap_collect must collect_global and run Type_deinit on swept"
    );
    assert!(
        mir.contains("maybe_untrack_objectref_value"),
        "mir-run must keep maybe_untrack_objectref_value (heap unroot floor)"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_gc_type_deinit_sweep_bridge_first_slice_inventory")
            && lib.contains("GateB09GcTypeDeinitSweepBridgeFirstSliceInventory"),
        "syx-rt lib.rs must re-export sweep Type_deinit inventory"
    );
}

#[test]
fn b09_gc_type_deinit_sweep_bridge_first_slice_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b09-gc-type-deinit-sweep-bridge-first-slice.md");
    assert!(
        spec.contains("ADR-871")
            && (spec.contains("sweep") || spec.contains("Type_deinit") || spec.contains("collect")),
        "spec must cite ADR-871 sweep Type_deinit"
    );
    assert!(
        spec.contains("three-hundred-and-seventy-seventh"),
        
        "spec must cite W527 fingerprint"
    );
    assert!(
        spec.contains("GateB09GcTypeDeinitSweepBridgeFirstSliceInventory")
            && spec.contains("gate_b09_gc_type_deinit_sweep_bridge_first_slice_inventory"),
        "spec must name sweep Type_deinit inventory API"
    );
    assert!(
        spec.contains("b09_gc_type_deinit_sweep_bridge_first_slice_signed=true")
            || spec.contains("`b09_gc_type_deinit_sweep_bridge_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("managed_heap_collect_objectref_type_deinit_wired=true")
            || spec.contains("`managed_heap_collect_objectref_type_deinit_wired=true`"))
            && (spec.contains("free_slot_still_skips_type_deinit=true")
                || spec.contains("`free_slot_still_skips_type_deinit=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`")),
        "spec must cite sweep flags and migrate false"
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
fn b09_gc_type_deinit_sweep_bridge_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_gc_type_deinit_sweep_bridge_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_deinits_heap_unroot_first_slice_floor_ok);
    assert!(inv.managed_heap_collect_objectref_type_deinit_wired);
    assert!(inv.free_slot_still_skips_type_deinit);
    assert!(inv.storelocal_objectref_rc_still_live);
    assert!(inv.run_class_deinits_objectref_rc_still_live);
    assert!(inv.value_object_ref_retain_release_api_still_present);
    assert!(inv.rt_abi_object_ref_retain_release_wrap_still_present);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_gc_type_deinit_sweep_bridge_first_slice_signed);
    let floor = syx_rt::gate_b09_objectref_deinits_heap_unroot_first_slice_inventory();
    assert!(floor.b09_objectref_deinits_heap_unroot_first_slice_signed);
    assert!(floor.run_class_deinits_objectref_rc_still_live);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(!floor.b09_axis_exit_enabled);
}

#[test]
fn b09_gc_type_deinit_sweep_bridge_first_slice_fingerprint_and_adr() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W527")
            && (roadmap.contains("ADR-871")
                || roadmap.contains("sweep")
                || roadmap.contains("Type_deinit")),
        "roadmap must cite W527 / ADR-871 sweep Type_deinit"
    );
    assert!(
        roadmap.contains("three-hundred-and-seventy-seventh"),
        
        "roadmap must cite W527 fingerprint"
    );
    let adr = include_str!("../../../../docs/adr/architecture-decisions.md");
    assert!(
        adr.contains("ADR-871") && adr.contains("W527"),
        "ADR must record ADR-871 / W527"
    );
    crate::matrix_fingerprint::assert_live_matrix_fingerprint();
}

#[test]
fn b09_gc_type_deinit_sweep_bridge_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W527")
            && (roadmap.contains("ADR-871")
                || roadmap.contains("sweep")
                || roadmap.contains("Type_deinit")),
        "roadmap must cite W527 / ADR-871 sweep Type_deinit"
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
