//! dual-heap converge first-slice
//! Gap documented + ManagedHeap primary aligned; MemoryPool residual; ≠ fully_converged / ≠ enable / ≠ B-09 [x] / ≠ GC GA.

#[test]
fn b09_dual_heap_converge_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-vm/src/lib.rs",
        "crates/syx-mir-run/src/runtime.rs",
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "docs/gates/gate-b09-dual-heap-converge-first-slice.md",
        "docs/gates/gate-b09-actor-mailbox-value-queue-root-set-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 dual-heap converge first-slice requires evidence file: {path}"
        );
    }
    let vm = include_str!("../../../syx-vm/src/lib.rs");
    assert!(
        vm.contains("run_gc_collect")
            && vm.contains("ADR-834")
            && vm.contains("collect_global")
            && (vm.contains("MemoryPool") || vm.contains("mark-sweep") || vm.contains("residual")),
        "VmInstance must extract run_gc_collect with ManagedHeap primary + MemoryPool storage (ADR-834)"
    );
    assert!(
        !vm.contains("fn collect_mark_sweep"),
        "Gc::collect_mark_sweep dead API retired (ADR-838); dual-heap path residual already retired at W491"
    );
    let mir = include_str!("../../../syx-mir-run/src/runtime.rs");
    assert!(
        mir.contains("heap_collect")
            && mir.contains("ADR-834")
            && (mir.contains("ManagedHeap") || mir.contains("collect_global"))
            && (mir.contains("fully_converged") || mir.contains("dual-heap")),
        "MirRuntime heap_collect must cite ADR-834 ManagedHeap-only / not fully converged"
    );
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09DualHeapConvergeFirstSliceInventory")
            && rt.contains("gate_b09_dual_heap_converge_first_slice_inventory")
            && rt.contains("ADR-834")
            && (rt.contains("b09_dual_heap_converge_first_slice_signed: true")
                || rt.contains("b09_dual_heap_converge_first_slice_signed:true")),
        "syx-rt must expose GateB09DualHeapConvergeFirstSliceInventory"
    );
    assert!(
        (rt.contains("dual_heap_gap_documented: true")
            || rt.contains("dual_heap_gap_documented:true"))
            && (rt.contains("shared_managed_heap_collect_path_aligned: true")
                || rt.contains("shared_managed_heap_collect_path_aligned:true"))
            && (rt.contains("vm_memory_pool_mark_sweep_residual: true")
                || rt.contains("vm_memory_pool_mark_sweep_residual:true"))
            && (rt.contains("vm_mir_runtime_heap_fully_converged: false")
                || rt.contains("vm_mir_runtime_heap_fully_converged:false"))
            && (rt.contains("unified_mir_runtime_heap_enabled: false")
                || rt.contains("unified_mir_runtime_heap_enabled:false")),
        "inventory must document gap/aligned/residual true and keep converged/enable false"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_dual_heap_converge_first_slice_inventory")
            && lib.contains("GateB09DualHeapConvergeFirstSliceInventory"),
        "syx-rt lib.rs must re-export dual-heap converge inventory"
    );
}

#[test]
fn b09_dual_heap_converge_first_slice_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b09-dual-heap-converge-first-slice.md");
    assert!(
        spec.contains("ADR-834")
            && (spec.contains("dual-heap")
                || spec.contains("ManagedHeap")
                || spec.contains("MemoryPool")),
        "spec must cite ADR-834 dual-heap converge first-slice"
    );
    assert!(
        spec.contains("three-hundred-and-fortieth"),
        
        "spec must cite W490 fingerprint"
    );
    assert!(
        spec.contains("GateB09DualHeapConvergeFirstSliceInventory")
            && spec.contains("gate_b09_dual_heap_converge_first_slice_inventory"),
        "spec must name dual-heap inventory API"
    );
    assert!(
        spec.contains("b09_dual_heap_converge_first_slice_signed=true")
            || spec.contains("`b09_dual_heap_converge_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("dual_heap_gap_documented=true")
            || spec.contains("`dual_heap_gap_documented=true`"))
            && (spec.contains("shared_managed_heap_collect_path_aligned=true")
                || spec.contains("`shared_managed_heap_collect_path_aligned=true`"))
            && (spec.contains("vm_memory_pool_mark_sweep_residual=true")
                || spec.contains("`vm_memory_pool_mark_sweep_residual=true`"))
            && (spec.contains("vm_mir_runtime_heap_fully_converged=false")
                || spec.contains("`vm_mir_runtime_heap_fully_converged=false`"))
            && (spec.contains("unified_mir_runtime_heap_enabled=false")
                || spec.contains("`unified_mir_runtime_heap_enabled=false`")),
        "spec must cite gap/aligned/residual true and converged/enable false"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("fully_converged")
                || spec.contains("unified")
                || spec.contains("B-09")),
        "spec Frozen must fence fully_converged / enable / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_dual_heap_converge_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_dual_heap_converge_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_actor_mailbox_value_queue_root_set_first_slice_floor_ok);
    assert!(inv.dual_heap_gap_documented);
    assert!(inv.shared_managed_heap_collect_path_aligned);
    assert!(inv.vm_memory_pool_mark_sweep_residual);
    assert!(!inv.vm_mir_runtime_heap_fully_converged);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_dual_heap_converge_first_slice_signed);
    let mb = syx_rt::gate_b09_actor_mailbox_value_queue_root_set_first_slice_inventory();
    assert!(!mb.vm_mir_runtime_heap_fully_converged);
    assert!(mb.b09_actor_mailbox_value_queue_root_set_first_slice_signed);
}

#[test]
fn b09_dual_heap_converge_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W490")
            && (roadmap.contains("ADR-834")
                || roadmap.contains("dual-heap")
                || roadmap.contains("双堆")
                || roadmap.contains("converge")),
        "roadmap must cite W490 / ADR-834 dual-heap converge first-slice"
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
