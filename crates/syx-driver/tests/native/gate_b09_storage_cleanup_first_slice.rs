//! storage-cleanup first-slice
//! Dead Gc::collect_mark_sweep retired; cleanup/enable false; storage still present.

#[test]
fn b09_storage_cleanup_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-vm/src/lib.rs",
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "docs/gates/gate-b09-storage-cleanup-first-slice.md",
        "docs/gates/gate-b09-enable-readiness-thin.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 storage-cleanup first-slice requires evidence file: {path}"
        );
    }
    let vm = include_str!("../../../syx-vm/src/lib.rs");
    assert!(
        vm.contains("MemoryPool")
            && (vm.contains("ObjectRef") || vm.contains("ObjectRefRetain"))
            && vm.contains("run_gc_collect")
            && vm.contains("ADR-838"),
        "VmInstance must keep MemoryPool / ObjectRef storage and cite ADR-838"
    );
    assert!(
        !vm.contains("fn collect_mark_sweep"),
        "dead Gc::collect_mark_sweep must be deleted (ADR-838)"
    );
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09StorageCleanupFirstSliceInventory")
            && rt.contains("gate_b09_storage_cleanup_first_slice_inventory")
            && rt.contains("ADR-838")
            && (rt.contains("b09_storage_cleanup_first_slice_signed: true")
                || rt.contains("b09_storage_cleanup_first_slice_signed:true")),
        "syx-rt must expose GateB09StorageCleanupFirstSliceInventory"
    );
    assert!(
        (rt.contains("gc_collect_mark_sweep_dead_api_retired: true")
            || rt.contains("gc_collect_mark_sweep_dead_api_retired:true"))
            && (rt.contains("storage_cleanup_checklist_progress_documented: true")
                || rt.contains("storage_cleanup_checklist_progress_documented:true"))
            && (rt.contains("vm_memory_pool_objectref_storage_still_present: true")
                || rt.contains("vm_memory_pool_objectref_storage_still_present:true"))
            && (rt.contains("storage_cleanup_enabled: false")
                || rt.contains("storage_cleanup_enabled:false"))
            && (rt.contains("unified_mir_runtime_heap_enabled: false")
                || rt.contains("unified_mir_runtime_heap_enabled:false")),
        "inventory must retire dead API and keep cleanup/enable false + storage present"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_storage_cleanup_first_slice_inventory")
            && lib.contains("GateB09StorageCleanupFirstSliceInventory"),
        "syx-rt lib.rs must re-export storage-cleanup first-slice inventory"
    );
}

#[test]
fn b09_storage_cleanup_first_slice_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b09-storage-cleanup-first-slice.md");
    assert!(
        spec.contains("ADR-838")
            && (spec.contains("storage-cleanup")
                || spec.contains("dead API")
                || spec.contains("collect_mark_sweep")),
        "spec must cite ADR-838 storage-cleanup first-slice"
    );
    assert!(
        spec.contains("three-hundred-and-forty-fourth"),
        
        "spec must cite W494 fingerprint"
    );
    assert!(
        spec.contains("GateB09StorageCleanupFirstSliceInventory")
            && spec.contains("gate_b09_storage_cleanup_first_slice_inventory"),
        "spec must name storage-cleanup first-slice inventory API"
    );
    assert!(
        spec.contains("b09_storage_cleanup_first_slice_signed=true")
            || spec.contains("`b09_storage_cleanup_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("gc_collect_mark_sweep_dead_api_retired=true")
            || spec.contains("`gc_collect_mark_sweep_dead_api_retired=true`"))
            && (spec.contains("storage_cleanup_enabled=false")
                || spec.contains("`storage_cleanup_enabled=false`"))
            && (spec.contains("unified_mir_runtime_heap_enabled=false")
                || spec.contains("`unified_mir_runtime_heap_enabled=false`"))
            && (spec.contains("vm_memory_pool_objectref_storage_still_present=true")
                || spec.contains("`vm_memory_pool_objectref_storage_still_present=true`")),
        "spec must cite retired true and cleanup/enable false + storage present"
    );
    assert!(
        !spec.contains("storage_cleanup_enabled=true")
            && !spec.contains("`storage_cleanup_enabled=true`"),
        "spec must not claim storage_cleanup_enabled=true"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("unified") || spec.contains("cleanup") || spec.contains("B-09")),
        "spec Frozen must fence enable / cleanup / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_storage_cleanup_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_storage_cleanup_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_enable_readiness_thin_floor_ok);
    assert!(inv.gc_collect_mark_sweep_dead_api_retired);
    assert!(inv.storage_cleanup_checklist_progress_documented);
    assert!(inv.vm_memory_pool_objectref_storage_still_present);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_storage_cleanup_first_slice_signed);
    let er = syx_rt::gate_b09_enable_readiness_thin_inventory();
    assert!(er.gc_collect_mark_sweep_dead_api_documented);
    assert!(!er.storage_cleanup_enabled);
    assert!(er.b09_enable_readiness_signed);
}

#[test]
fn b09_storage_cleanup_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W494")
            && (roadmap.contains("ADR-838")
                || roadmap.contains("storage-cleanup")
                || roadmap.contains("dead API")
                || roadmap.contains("collect_mark_sweep")),
        "roadmap must cite W494 / ADR-838 storage-cleanup first-slice"
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
