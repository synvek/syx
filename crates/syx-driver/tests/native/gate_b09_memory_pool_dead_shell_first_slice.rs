//! MemoryPool dead-shell first-slice
//! alloc + pool field retired; ObjectRef still present; cleanup/enable false.

#[test]
fn b09_memory_pool_dead_shell_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-vm/src/lib.rs",
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "docs/gates/gate-b09-memory-pool-dead-shell-first-slice.md",
        "docs/gates/gate-b09-storage-cleanup-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 MemoryPool dead-shell first-slice requires evidence file: {path}"
        );
    }
    let vm = include_str!("../../../syx-vm/src/lib.rs");
    assert!(
        vm.contains("MemoryPool")
            && (vm.contains("ObjectRef") || vm.contains("ObjectRefRetain"))
            && vm.contains("run_gc_collect")
            && vm.contains("ADR-839"),
        "VmInstance must keep MemoryPool honesty string + ObjectRef and cite ADR-839"
    );
    assert!(
        !vm.contains("fn alloc") && !vm.contains("pub pool:"),
        "MemoryPool::alloc and VmInstance.pool must be retired (ADR-839)"
    );
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09MemoryPoolDeadShellFirstSliceInventory")
            && rt.contains("gate_b09_memory_pool_dead_shell_first_slice_inventory")
            && rt.contains("ADR-839")
            && (rt.contains("b09_memory_pool_dead_shell_first_slice_signed: true")
                || rt.contains("b09_memory_pool_dead_shell_first_slice_signed:true")),
        "syx-rt must expose GateB09MemoryPoolDeadShellFirstSliceInventory"
    );
    assert!(
        (rt.contains("memory_pool_alloc_retired: true")
            || rt.contains("memory_pool_alloc_retired:true"))
            && (rt.contains("vm_instance_pool_field_retired: true")
                || rt.contains("vm_instance_pool_field_retired:true"))
            && (rt.contains("objectref_rc_storage_still_present: true")
                || rt.contains("objectref_rc_storage_still_present:true"))
            && (rt.contains("storage_cleanup_enabled: false")
                || rt.contains("storage_cleanup_enabled:false"))
            && (rt.contains("unified_mir_runtime_heap_enabled: false")
                || rt.contains("unified_mir_runtime_heap_enabled:false")),
        "inventory must retire alloc/pool and keep cleanup/enable false + ObjectRef present"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_memory_pool_dead_shell_first_slice_inventory")
            && lib.contains("GateB09MemoryPoolDeadShellFirstSliceInventory"),
        "syx-rt lib.rs must re-export MemoryPool dead-shell inventory"
    );
}

#[test]
fn b09_memory_pool_dead_shell_first_slice_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b09-memory-pool-dead-shell-first-slice.md");
    assert!(
        spec.contains("ADR-839")
            && (spec.contains("dead-shell")
                || spec.contains("MemoryPool")
                || spec.contains("alloc")),
        "spec must cite ADR-839 MemoryPool dead-shell first-slice"
    );
    assert!(
        spec.contains("three-hundred-and-forty-fifth"),
        
        "spec must cite W495 fingerprint"
    );
    assert!(
        spec.contains("GateB09MemoryPoolDeadShellFirstSliceInventory")
            && spec.contains("gate_b09_memory_pool_dead_shell_first_slice_inventory"),
        "spec must name MemoryPool dead-shell inventory API"
    );
    assert!(
        spec.contains("b09_memory_pool_dead_shell_first_slice_signed=true")
            || spec.contains("`b09_memory_pool_dead_shell_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("memory_pool_alloc_retired=true")
            || spec.contains("`memory_pool_alloc_retired=true`"))
            && (spec.contains("vm_instance_pool_field_retired=true")
                || spec.contains("`vm_instance_pool_field_retired=true`"))
            && (spec.contains("storage_cleanup_enabled=false")
                || spec.contains("`storage_cleanup_enabled=false`"))
            && (spec.contains("unified_mir_runtime_heap_enabled=false")
                || spec.contains("`unified_mir_runtime_heap_enabled=false`"))
            && (spec.contains("objectref_rc_storage_still_present=true")
                || spec.contains("`objectref_rc_storage_still_present=true`")),
        "spec must cite alloc/pool retired and cleanup/enable false + ObjectRef present"
    );
    assert!(
        !spec.contains("unified_mir_runtime_heap_enabled=true")
            && !spec.contains("`unified_mir_runtime_heap_enabled=true`"),
        "spec must not claim unified heap enabled"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("unified") || spec.contains("cleanup") || spec.contains("B-09")),
        "spec Frozen must fence enable / cleanup / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_memory_pool_dead_shell_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_memory_pool_dead_shell_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_storage_cleanup_first_slice_floor_ok);
    assert!(inv.memory_pool_alloc_retired);
    assert!(inv.vm_instance_pool_field_retired);
    assert!(inv.objectref_rc_storage_still_present);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_memory_pool_dead_shell_first_slice_signed);
    let sc = syx_rt::gate_b09_storage_cleanup_first_slice_inventory();
    assert!(sc.gc_collect_mark_sweep_dead_api_retired);
    assert!(!sc.storage_cleanup_enabled);
    assert!(sc.b09_storage_cleanup_first_slice_signed);
}

#[test]
fn b09_memory_pool_dead_shell_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W495")
            && (roadmap.contains("ADR-839")
                || roadmap.contains("dead-shell")
                || roadmap.contains("MemoryPool")),
        "roadmap must cite W495 / ADR-839 MemoryPool dead-shell first-slice"
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
