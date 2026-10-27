//! ObjectRef VM heap_slots first-slice
//! VmInstance.heap_slots + DictBuild push + collect/compact; migrate/cleanup false.

#[test]
fn b09_objectref_vm_heap_slots_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-vm/src/lib.rs",
        "crates/syx-mir-run/src/runtime.rs",
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "docs/gates/gate-b09-objectref-vm-heap-slots-first-slice.md",
        "docs/gates/gate-b09-objectref-vm-managed-heap-probe-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 ObjectRef VM heap_slots requires evidence file: {path}"
        );
    }
    let vm = include_str!("../../../syx-vm/src/lib.rs");
    assert!(
        vm.contains("heap_slots")
            && (vm.contains("self.heap_slots.push")
                || vm.contains("alloc_heap_slot")
                || vm.contains("ManagedHeap"))
            && vm.contains("ADR-842")
            && (vm.contains("for slot in &mut self.heap_slots")
                || vm.contains("for slot in &self.heap_slots")),
        "VmInstance must expose heap_slots / ManagedHeap path, collect/compact rewrite, ADR-842"
    );
    assert!(
        vm.contains("ObjectRefRetain")
            || vm.contains("ObjectRefRelease")
            || vm.contains("ObjectRef"),
        "ObjectRef RC handlers must remain live"
    );
    assert!(
        !vm.contains("objectref_rc_migrate_enabled = true")
            && !vm.contains("objectref_rc_migrate_enabled=true"),
        "must not flip objectref_rc_migrate_enabled"
    );
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09ObjectRefVmHeapSlotsFirstSliceInventory")
            && rt.contains("gate_b09_objectref_vm_heap_slots_first_slice_inventory")
            && rt.contains("ADR-842")
            && (rt.contains("b09_objectref_vm_heap_slots_first_slice_signed: true")
                || rt.contains("b09_objectref_vm_heap_slots_first_slice_signed:true")),
        "syx-rt must expose GateB09ObjectRefVmHeapSlotsFirstSliceInventory"
    );
    assert!(
        (rt.contains("vm_instance_heap_slots_present: true")
            || rt.contains("vm_instance_heap_slots_present:true"))
            && (rt.contains("vm_dictbuild_objectref_heap_slots_tracked: true")
                || rt.contains("vm_dictbuild_objectref_heap_slots_tracked:true"))
            && (rt.contains("vm_heap_slots_compact_rewrite_wired: true")
                || rt.contains("vm_heap_slots_compact_rewrite_wired:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false"))
            && (rt.contains("storage_cleanup_enabled: false")
                || rt.contains("storage_cleanup_enabled:false"))
            && (rt.contains("unified_mir_runtime_heap_enabled: false")
                || rt.contains("unified_mir_runtime_heap_enabled:false")),
        "inventory must register heap_slots wire and keep migrate/cleanup/enable false"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_objectref_vm_heap_slots_first_slice_inventory")
            && lib.contains("GateB09ObjectRefVmHeapSlotsFirstSliceInventory"),
        "syx-rt lib.rs must re-export VM heap_slots inventory"
    );
}

#[test]
fn b09_objectref_vm_heap_slots_first_slice_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b09-objectref-vm-heap-slots-first-slice.md");
    assert!(
        spec.contains("ADR-842") && (spec.contains("heap_slots") || spec.contains("HeapCompact")),
        "spec must cite ADR-842 ObjectRef VM heap_slots"
    );
    assert!(
        spec.contains("three-hundred-and-forty-eighth"),
        
        "spec must cite W498 fingerprint"
    );
    assert!(
        spec.contains("GateB09ObjectRefVmHeapSlotsFirstSliceInventory")
            && spec.contains("gate_b09_objectref_vm_heap_slots_first_slice_inventory"),
        "spec must name VM heap_slots inventory API"
    );
    assert!(
        spec.contains("b09_objectref_vm_heap_slots_first_slice_signed=true")
            || spec.contains("`b09_objectref_vm_heap_slots_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("vm_instance_heap_slots_present=true")
            || spec.contains("`vm_instance_heap_slots_present=true`"))
            && (spec.contains("vm_dictbuild_objectref_heap_slots_tracked=true")
                || spec.contains("`vm_dictbuild_objectref_heap_slots_tracked=true`"))
            && (spec.contains("vm_heap_slots_compact_rewrite_wired=true")
                || spec.contains("`vm_heap_slots_compact_rewrite_wired=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`"))
            && (spec.contains("storage_cleanup_enabled=false")
                || spec.contains("`storage_cleanup_enabled=false`"))
            && (spec.contains("mir_run_llvm_wasm_objectref_rc_still_live=true")
                || spec.contains("`mir_run_llvm_wasm_objectref_rc_still_live=true`")),
        "spec must cite heap_slots flags and migrate/cleanup false + RC still live"
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
fn b09_objectref_vm_heap_slots_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_objectref_vm_heap_slots_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_vm_managed_heap_probe_first_slice_floor_ok);
    assert!(inv.vm_instance_heap_slots_present);
    assert!(inv.vm_dictbuild_objectref_heap_slots_tracked);
    assert!(inv.vm_heap_slots_compact_rewrite_wired);
    assert!(inv.objectref_rc_storage_still_present);
    assert!(inv.mir_run_llvm_wasm_objectref_rc_still_live);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_objectref_vm_heap_slots_first_slice_signed);
    let probe = syx_rt::gate_b09_objectref_vm_managed_heap_probe_first_slice_inventory();
    assert!(probe.vm_dictbuild_objectref_managed_heap_registered);
    assert!(!probe.objectref_rc_migrate_enabled);
    assert!(probe.b09_objectref_vm_managed_heap_probe_first_slice_signed);
}

#[test]
fn b09_objectref_vm_heap_slots_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W498")
            && (roadmap.contains("ADR-842")
                || roadmap.contains("heap_slots")
                || roadmap.contains("HeapCompact")),
        "roadmap must cite W498 / ADR-842 ObjectRef VM heap_slots"
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
