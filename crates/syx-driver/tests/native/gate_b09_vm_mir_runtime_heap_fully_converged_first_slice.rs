//! VM↔MirRuntime heap fully_converged first-slice
//! MemoryPool mark-sweep retired from GcCollect; converged true; ≠ enable / ≠ B-09 [x] / ≠ GA.

#[test]
fn b09_vm_mir_runtime_heap_fully_converged_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-vm/src/lib.rs",
        "crates/syx-mir-run/src/runtime.rs",
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "docs/gates/gate-b09-vm-mir-runtime-heap-fully-converged-first-slice.md",
        "docs/gates/gate-b09-dual-heap-converge-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 fully_converged first-slice requires evidence file: {path}"
        );
    }
    let vm = include_str!("../../../syx-vm/src/lib.rs");
    assert!(
        vm.contains("run_gc_collect")
            && vm.contains("ADR-835")
            && vm.contains("collect_global")
            && vm.contains("MemoryPool"),
        "VmInstance must keep run_gc_collect ManagedHeap-only + MemoryPool storage (ADR-835)"
    );
    assert!(
        !vm.contains("fn collect_mark_sweep"),
        "Gc::collect_mark_sweep dead API must be retired (ADR-838); historical path retire still holds"
    );
    // run_gc_collect body must not call collect_mark_sweep (narrow retire).
    let Some(start) = vm.find("pub fn run_gc_collect") else {
        panic!("run_gc_collect missing");
    };
    let rest = &vm[start..];
    let end = rest.find("\n    pub fn ").unwrap_or(rest.len().min(800));
    let body = &rest[..end];
    assert!(
        !body.contains("collect_mark_sweep"),
        "run_gc_collect must not call collect_mark_sweep after ADR-835"
    );
    let mir = include_str!("../../../syx-mir-run/src/runtime.rs");
    assert!(
        mir.contains("heap_collect")
            && mir.contains("ADR-835")
            && (mir.contains("fully_converged") || mir.contains("ManagedHeap")),
        "MirRuntime heap_collect must cite ADR-835 fully_converged narrow floor"
    );
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09VmMirRuntimeHeapFullyConvergedFirstSliceInventory")
            && rt.contains("gate_b09_vm_mir_runtime_heap_fully_converged_first_slice_inventory")
            && rt.contains("ADR-835")
            && (rt.contains("b09_vm_mir_runtime_heap_fully_converged_first_slice_signed: true")
                || rt.contains("b09_vm_mir_runtime_heap_fully_converged_first_slice_signed:true")),
        "syx-rt must expose GateB09VmMirRuntimeHeapFullyConvergedFirstSliceInventory"
    );
    assert!(
        (rt.contains("vm_memory_pool_mark_sweep_gc_collect_residual_retired: true")
            || rt.contains("vm_memory_pool_mark_sweep_gc_collect_residual_retired:true"))
            && (rt.contains("vm_mir_runtime_heap_fully_converged: true")
                || rt.contains("vm_mir_runtime_heap_fully_converged:true"))
            && (rt.contains("vm_memory_pool_objectref_storage_still_present: true")
                || rt.contains("vm_memory_pool_objectref_storage_still_present:true"))
            && (rt.contains("unified_mir_runtime_heap_enabled: false")
                || rt.contains("unified_mir_runtime_heap_enabled:false")),
        "inventory must retire residual / converge true / storage present / enable false"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_vm_mir_runtime_heap_fully_converged_first_slice_inventory")
            && lib.contains("GateB09VmMirRuntimeHeapFullyConvergedFirstSliceInventory"),
        "syx-rt lib.rs must re-export fully_converged inventory"
    );
}

#[test]
fn b09_vm_mir_runtime_heap_fully_converged_first_slice_spec_signed() {
    let spec = include_str!(
        "../../../../docs/gates/gate-b09-vm-mir-runtime-heap-fully-converged-first-slice.md"
    );
    assert!(
        spec.contains("ADR-835")
            && (spec.contains("fully_converged") || spec.contains("ManagedHeap")),
        "spec must cite ADR-835 fully_converged first-slice"
    );
    assert!(
        spec.contains("three-hundred-and-forty-first"),
        
        "spec must cite W491 fingerprint"
    );
    assert!(
        spec.contains("GateB09VmMirRuntimeHeapFullyConvergedFirstSliceInventory")
            && spec.contains("gate_b09_vm_mir_runtime_heap_fully_converged_first_slice_inventory"),
        "spec must name fully_converged inventory API"
    );
    assert!(
        spec.contains("b09_vm_mir_runtime_heap_fully_converged_first_slice_signed=true")
            || spec.contains("`b09_vm_mir_runtime_heap_fully_converged_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("vm_memory_pool_mark_sweep_gc_collect_residual_retired=true")
            || spec.contains("`vm_memory_pool_mark_sweep_gc_collect_residual_retired=true`"))
            && (spec.contains("vm_mir_runtime_heap_fully_converged=true")
                || spec.contains("`vm_mir_runtime_heap_fully_converged=true`"))
            && (spec.contains("vm_memory_pool_objectref_storage_still_present=true")
                || spec.contains("`vm_memory_pool_objectref_storage_still_present=true`"))
            && (spec.contains("unified_mir_runtime_heap_enabled=false")
                || spec.contains("`unified_mir_runtime_heap_enabled=false`")),
        "spec must cite retired/converged/storage true and enable false"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("unified") || spec.contains("B-09") || spec.contains("enable")),
        "spec Frozen must fence enable / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_vm_mir_runtime_heap_fully_converged_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_vm_mir_runtime_heap_fully_converged_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_dual_heap_converge_first_slice_floor_ok);
    assert!(inv.vm_memory_pool_mark_sweep_gc_collect_residual_retired);
    assert!(inv.vm_mir_runtime_heap_fully_converged);
    assert!(inv.vm_memory_pool_objectref_storage_still_present);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_vm_mir_runtime_heap_fully_converged_first_slice_signed);
    let dh = syx_rt::gate_b09_dual_heap_converge_first_slice_inventory();
    assert!(!dh.vm_mir_runtime_heap_fully_converged);
    assert!(dh.vm_memory_pool_mark_sweep_residual);
    assert!(dh.b09_dual_heap_converge_first_slice_signed);
}

