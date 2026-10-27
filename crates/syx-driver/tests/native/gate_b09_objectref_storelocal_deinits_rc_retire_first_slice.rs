//! StoreLocal/`run_class_deinits` ObjectRef RC true-retire
//! migrate/cleanup false; Value API still present; RC retire ≠ Value full delete.

#[test]
fn b09_objectref_storelocal_deinits_rc_retire_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "crates/syx-vm/src/lib.rs",
        "crates/syx-mir-run/src/runtime.rs",
        "crates/syx-runtime-core/src/value.rs",
        "docs/gates/gate-b09-objectref-storelocal-deinits-rc-retire-first-slice.md",
        "docs/gates/gate-b09-gc-type-deinit-sweep-bridge-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 StoreLocal/deinits RC retire requires evidence file: {path}"
        );
    }
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09ObjectrefStorelocalDeinitsRcRetireFirstSliceInventory")
            && rt.contains("gate_b09_objectref_storelocal_deinits_rc_retire_first_slice_inventory")
            && rt.contains("ADR-872")
            && (rt.contains("b09_objectref_storelocal_deinits_rc_retire_first_slice_signed: true")
                || rt.contains("b09_objectref_storelocal_deinits_rc_retire_first_slice_signed:true")),
        "syx-rt must expose GateB09ObjectrefStorelocalDeinitsRcRetireFirstSliceInventory"
    );
    assert!(
        (rt.contains("vm_mir_run_storelocal_objectref_rc_retired: true")
            || rt.contains("vm_mir_run_storelocal_objectref_rc_retired:true"))
            && (rt.contains("vm_mir_run_deinits_objectref_rc_retired: true")
                || rt.contains("vm_mir_run_deinits_objectref_rc_retired:true"))
            && (rt.contains("vm_mir_run_storelocal_deinits_heap_unroot_kept: true")
                || rt.contains("vm_mir_run_storelocal_deinits_heap_unroot_kept:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false")),
        "inventory must document StoreLocal/deinits RC retire and migrate false"
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
        vm.contains("ADR-872") && vm.contains("maybe_untrack_objectref_value"),
        "VM must cite ADR-872 and keep untrack"
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
        "VM StoreLocal must unroot and must not release"
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
            && !deinit_body.contains("release_object_ref(")
            && !deinit_body.contains("object_ref_release("),
        "VM deinits must unroot and must not release"
    );
    assert!(
        !vm.contains("fn release_object_ref"),
        "VM release_object_ref helper must be retired with StoreLocal/deinits RC"
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
        gc_body.contains("collect_global") && gc_body.contains("run_deinit_for_value"),
        "VM run_gc_collect must still run Type_deinit on swept ObjectRef"
    );
    let mir = include_str!("../../../syx-mir-run/src/runtime.rs");
    let mir_store = mir
        .split("op::STORE_LOCAL =>")
        .nth(1)
        .and_then(|s| {
            let end = s.find("\n            op::").unwrap_or(s.len().min(1200));
            Some(&s[..end])
        })
        .expect("mir-run STORE_LOCAL body");
    assert!(
        mir_store.contains("maybe_untrack_objectref_value")
            && !mir_store.contains("object_ref_release(")
            && !mir_store.contains("run_deinit_for_value("),
        "mir-run STORE_LOCAL must unroot and must not release/deinit"
    );
    let mir_deinit = mir
        .split("pub fn run_class_deinits")
        .nth(1)
        .and_then(|s| {
            let end = s.find("\n    fn ").unwrap_or(s.len().min(900));
            Some(&s[..end])
        })
        .expect("mir-run run_class_deinits body");
    assert!(
        mir_deinit.contains("maybe_untrack_objectref_value")
            && !mir_deinit.contains("object_ref_release(")
            && !mir_deinit.contains("run_deinit_for_value("),
        "mir-run deinits must unroot and must not release/deinit"
    );
    let mir_collect = mir
        .split("pub fn heap_collect")
        .nth(1)
        .and_then(|s| {
            let end = s.find("\n    fn ").unwrap_or(s.len().min(700));
            Some(&s[..end])
        })
        .expect("mir-run heap_collect body");
    assert!(
        mir_collect.contains("collect_global")
            && (mir_collect.contains("run_sweep_objectref_type_deinits")
                || mir_collect.contains("run_deinit_for_value")),
        "mir-run heap_collect must still run Type_deinit on swept"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_objectref_storelocal_deinits_rc_retire_first_slice_inventory")
            && lib.contains("GateB09ObjectrefStorelocalDeinitsRcRetireFirstSliceInventory"),
        "syx-rt lib.rs must re-export StoreLocal/deinits RC retire inventory"
    );
}

#[test]
fn b09_objectref_storelocal_deinits_rc_retire_first_slice_spec_signed() {
    let spec = include_str!(
        "../../../../docs/gates/gate-b09-objectref-storelocal-deinits-rc-retire-first-slice.md"
    );
    assert!(
        spec.contains("ADR-872")
            && (spec.contains("true-retire")
                || spec.contains("RC")
                || spec.contains("StoreLocal")),
        "spec must cite ADR-872 StoreLocal/deinits RC true-retire"
    );
    assert!(
        spec.contains("three-hundred-and-seventy-eighth"),
        
        "spec must cite W528 fingerprint"
    );
    assert!(
        spec.contains("GateB09ObjectrefStorelocalDeinitsRcRetireFirstSliceInventory")
            && spec.contains("gate_b09_objectref_storelocal_deinits_rc_retire_first_slice_inventory"),
        "spec must name StoreLocal/deinits RC retire inventory API"
    );
    assert!(
        spec.contains("b09_objectref_storelocal_deinits_rc_retire_first_slice_signed=true")
            || spec.contains("`b09_objectref_storelocal_deinits_rc_retire_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("vm_mir_run_storelocal_objectref_rc_retired=true")
            || spec.contains("`vm_mir_run_storelocal_objectref_rc_retired=true`"))
            && (spec.contains("vm_mir_run_deinits_objectref_rc_retired=true")
                || spec.contains("`vm_mir_run_deinits_objectref_rc_retired=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`")),
        "spec must cite RC retire flags and migrate false"
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
            && (spec.contains("Value") || spec.contains("B-09") || spec.contains("ABI")),
        "spec Frozen must fence Value delete / ABI / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_objectref_storelocal_deinits_rc_retire_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_objectref_storelocal_deinits_rc_retire_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_gc_type_deinit_sweep_bridge_first_slice_floor_ok);
    assert!(inv.vm_mir_run_storelocal_objectref_rc_retired);
    assert!(inv.vm_mir_run_deinits_objectref_rc_retired);
    assert!(inv.vm_mir_run_storelocal_deinits_heap_unroot_kept);
    assert!(inv.managed_heap_collect_objectref_type_deinit_still_wired);
    assert!(inv.value_object_ref_retain_release_api_still_present);
    assert!(inv.rt_abi_object_ref_retain_release_wrap_still_present);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_objectref_storelocal_deinits_rc_retire_first_slice_signed);
    let floor = syx_rt::gate_b09_gc_type_deinit_sweep_bridge_first_slice_inventory();
    assert!(floor.b09_gc_type_deinit_sweep_bridge_first_slice_signed);
    assert!(floor.managed_heap_collect_objectref_type_deinit_wired);
    assert!(!floor.objectref_rc_migrate_enabled);
    assert!(!floor.b09_axis_exit_enabled);
}

#[test]
fn b09_objectref_storelocal_deinits_rc_retire_first_slice_fingerprint_and_adr() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W528")
            && (roadmap.contains("ADR-872")
                || roadmap.contains("true-retire")
                || roadmap.contains("StoreLocal")),
        "roadmap must cite W528 / ADR-872 StoreLocal/deinits RC retire"
    );
    assert!(
        roadmap.contains("three-hundred-and-seventy-eighth"),
        
        "roadmap must cite W528 fingerprint"
    );
    let adr = include_str!("../../../../docs/adr/architecture-decisions.md");
    assert!(
        adr.contains("ADR-872") && adr.contains("W528"),
        "ADR must record ADR-872 / W528"
    );
    crate::matrix_fingerprint::assert_live_matrix_fingerprint();
}

#[test]
fn b09_objectref_storelocal_deinits_rc_retire_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W528")
            && (roadmap.contains("ADR-872")
                || roadmap.contains("true-retire")
                || roadmap.contains("RC")),
        "roadmap must cite W528 / ADR-872 StoreLocal/deinits RC retire"
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
