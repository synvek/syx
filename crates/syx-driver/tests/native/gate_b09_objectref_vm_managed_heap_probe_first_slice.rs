//! ObjectRef VM ManagedHeap probe first-slice
//! DictBuild __type dual-registers ManagedHeap; ObjectRef RC still live; migrate/cleanup false.

#[test]
fn b09_objectref_vm_managed_heap_probe_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-vm/src/lib.rs",
        "crates/syx-mir-run/src/runtime.rs",
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "docs/gates/gate-b09-objectref-vm-managed-heap-probe-first-slice.md",
        "docs/gates/gate-b09-objectref-rc-migrate-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 ObjectRef VM ManagedHeap probe requires evidence file: {path}"
        );
    }
    let vm = include_str!("../../../syx-vm/src/lib.rs");
    assert!(
        vm.contains("DictBuild")
            && vm.contains("object_ref_from_map")
            && (vm.contains("ManagedHeap") || vm.contains("global_heap"))
            && (vm.contains("heap.alloc") || vm.contains("alloc_heap_slot"))
            && vm.contains("ADR-841"),
        "VmInstance DictBuild __type path must register ManagedHeap (alloc_heap_slot or heap.alloc) and cite ADR-841"
    );
    assert!(
        vm.contains("ObjectRefRetain")
            || vm.contains("ObjectRefRelease")
            || vm.contains("ObjectRef"),
        "ObjectRef RC handlers must remain live"
    );
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09ObjectRefVmManagedHeapProbeFirstSliceInventory")
            && rt.contains("gate_b09_objectref_vm_managed_heap_probe_first_slice_inventory")
            && rt.contains("ADR-841")
            && (rt.contains("b09_objectref_vm_managed_heap_probe_first_slice_signed: true")
                || rt.contains("b09_objectref_vm_managed_heap_probe_first_slice_signed:true")),
        "syx-rt must expose GateB09ObjectRefVmManagedHeapProbeFirstSliceInventory"
    );
    assert!(
        (rt.contains("vm_dictbuild_objectref_managed_heap_registered: true")
            || rt.contains("vm_dictbuild_objectref_managed_heap_registered:true"))
            && (rt.contains("mir_run_llvm_wasm_objectref_rc_still_live: true")
                || rt.contains("mir_run_llvm_wasm_objectref_rc_still_live:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false"))
            && (rt.contains("storage_cleanup_enabled: false")
                || rt.contains("storage_cleanup_enabled:false"))
            && (rt.contains("unified_mir_runtime_heap_enabled: false")
                || rt.contains("unified_mir_runtime_heap_enabled:false")),
        "inventory must register probe and keep migrate/cleanup/enable false"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_objectref_vm_managed_heap_probe_first_slice_inventory")
            && lib.contains("GateB09ObjectRefVmManagedHeapProbeFirstSliceInventory"),
        "syx-rt lib.rs must re-export VM ManagedHeap probe inventory"
    );
}

#[test]
fn b09_objectref_vm_managed_heap_probe_first_slice_spec_signed() {
    let spec =
        include_str!("../../../../docs/gates/gate-b09-objectref-vm-managed-heap-probe-first-slice.md");
    assert!(
        spec.contains("ADR-841")
            && (spec.contains("ManagedHeap")
                || spec.contains("DictBuild")
                || spec.contains("probe")),
        "spec must cite ADR-841 ObjectRef VM ManagedHeap probe"
    );
    assert!(
        spec.contains("three-hundred-and-forty-seventh"),
        
        "spec must cite W497 fingerprint"
    );
    assert!(
        spec.contains("GateB09ObjectRefVmManagedHeapProbeFirstSliceInventory")
            && spec.contains("gate_b09_objectref_vm_managed_heap_probe_first_slice_inventory"),
        "spec must name VM ManagedHeap probe inventory API"
    );
    assert!(
        spec.contains("b09_objectref_vm_managed_heap_probe_first_slice_signed=true")
            || spec.contains("`b09_objectref_vm_managed_heap_probe_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("vm_dictbuild_objectref_managed_heap_registered=true")
            || spec.contains("`vm_dictbuild_objectref_managed_heap_registered=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`"))
            && (spec.contains("storage_cleanup_enabled=false")
                || spec.contains("`storage_cleanup_enabled=false`"))
            && (spec.contains("mir_run_llvm_wasm_objectref_rc_still_live=true")
                || spec.contains("`mir_run_llvm_wasm_objectref_rc_still_live=true`")),
        "spec must cite probe registered and migrate/cleanup false + RC still live"
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
            && (spec.contains("unified") || spec.contains("cleanup") || spec.contains("B-09")),
        "spec Frozen must fence enable / cleanup / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_objectref_vm_managed_heap_probe_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_objectref_vm_managed_heap_probe_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_rc_migrate_first_slice_floor_ok);
    assert!(inv.vm_dictbuild_objectref_managed_heap_registered);
    assert!(inv.objectref_rc_storage_still_present);
    assert!(inv.mir_run_llvm_wasm_objectref_rc_still_live);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_objectref_vm_managed_heap_probe_first_slice_signed);
    let mig = syx_rt::gate_b09_objectref_rc_migrate_first_slice_inventory();
    assert!(mig.memory_pool_type_stub_retired);
    assert!(!mig.objectref_rc_migrate_enabled);
    assert!(mig.b09_objectref_rc_migrate_first_slice_signed);
}

#[test]
fn b09_objectref_vm_managed_heap_probe_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W497")
            && (roadmap.contains("ADR-841")
                || roadmap.contains("ManagedHeap")
                || roadmap.contains("probe")),
        "roadmap must cite W497 / ADR-841 ObjectRef VM ManagedHeap probe"
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
