//! VM GcCollect root-set first-slice
//! suspend_frame + locals_frames wired; ≠ enable / ≠ mailbox Value queue / ≠ dual-heap converge / ≠ B-09 [x].

#[test]
fn b09_vm_gccollect_root_set_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-vm/src/lib.rs",
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "docs/gates/gate-b09-vm-gccollect-root-set-first-slice.md",
        "docs/gates/gate-b09-mir-runtime-root-set-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 VM GcCollect root-set first-slice requires evidence file: {path}"
        );
    }
    let vm = include_str!("../../../syx-vm/src/lib.rs");
    assert!(
        vm.contains("collect_live_roots")
            && vm.contains("ADR-832")
            && vm.contains("suspend_frame")
            && vm.contains("locals_frames"),
        "VmInstance must expose collect_live_roots wiring suspend_frame + locals_frames (ADR-832)"
    );
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09VmGcCollectRootSetFirstSliceInventory")
            && rt.contains("gate_b09_vm_gccollect_root_set_first_slice_inventory")
            && rt.contains("ADR-832")
            && (rt.contains("b09_vm_gccollect_root_set_first_slice_signed: true")
                || rt.contains("b09_vm_gccollect_root_set_first_slice_signed:true")),
        "syx-rt must expose GateB09VmGcCollectRootSetFirstSliceInventory"
    );
    assert!(
        (rt.contains("vm_suspend_frame_roots_wired: true")
            || rt.contains("vm_suspend_frame_roots_wired:true"))
            && (rt.contains("vm_locals_frames_roots_wired: true")
                || rt.contains("vm_locals_frames_roots_wired:true"))
            && (rt.contains("actor_mailbox_value_queue_roots_wired: false")
                || rt.contains("actor_mailbox_value_queue_roots_wired:false"))
            && (rt.contains("vm_mir_runtime_heap_fully_converged: false")
                || rt.contains("vm_mir_runtime_heap_fully_converged:false"))
            && (rt.contains("unified_mir_runtime_heap_enabled: false")
                || rt.contains("unified_mir_runtime_heap_enabled:false")),
        "inventory must wire suspend/locals_frames and keep mailbox/converged/enable false"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_vm_gccollect_root_set_first_slice_inventory")
            && lib.contains("GateB09VmGcCollectRootSetFirstSliceInventory"),
        "syx-rt lib.rs must re-export VM GcCollect root-set first-slice inventory"
    );
}

#[test]
fn b09_vm_gccollect_root_set_first_slice_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b09-vm-gccollect-root-set-first-slice.md");
    assert!(
        spec.contains("ADR-832")
            && (spec.contains("root-set") || spec.contains("根集") || spec.contains("GcCollect")),
        "spec must cite ADR-832 VM GcCollect root-set first-slice"
    );
    assert!(
        spec.contains("three-hundred-and-thirty-eighth"),
        
        "spec must cite W488 fingerprint"
    );
    assert!(
        spec.contains("GateB09VmGcCollectRootSetFirstSliceInventory")
            && spec.contains("gate_b09_vm_gccollect_root_set_first_slice_inventory"),
        "spec must name root-set inventory API"
    );
    assert!(
        spec.contains("b09_vm_gccollect_root_set_first_slice_signed=true")
            || spec.contains("`b09_vm_gccollect_root_set_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("vm_suspend_frame_roots_wired=true")
            || spec.contains("`vm_suspend_frame_roots_wired=true`"))
            && (spec.contains("vm_locals_frames_roots_wired=true")
                || spec.contains("`vm_locals_frames_roots_wired=true`"))
            && (spec.contains("actor_mailbox_value_queue_roots_wired=false")
                || spec.contains("`actor_mailbox_value_queue_roots_wired=false`"))
            && (spec.contains("vm_mir_runtime_heap_fully_converged=false")
                || spec.contains("`vm_mir_runtime_heap_fully_converged=false`"))
            && (spec.contains("unified_mir_runtime_heap_enabled=false")
                || spec.contains("`unified_mir_runtime_heap_enabled=false`")),
        "spec must cite wiring true and mailbox/converged/enable false"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("unified") || spec.contains("B-09") || spec.contains("mailbox")),
        "spec Frozen must fence enable / B-09 [x] / mailbox Value"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_vm_gccollect_root_set_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_vm_gccollect_root_set_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_mir_runtime_root_set_first_slice_floor_ok);
    assert!(inv.vm_suspend_frame_roots_wired);
    assert!(inv.vm_locals_frames_roots_wired);
    assert!(!inv.actor_mailbox_value_queue_roots_wired);
    assert!(!inv.vm_mir_runtime_heap_fully_converged);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_vm_gccollect_root_set_first_slice_signed);
    let mir = syx_rt::gate_b09_mir_runtime_root_set_first_slice_inventory();
    assert!(!mir.unified_mir_runtime_heap_enabled);
    assert!(mir.b09_mir_runtime_root_set_first_slice_signed);
}

