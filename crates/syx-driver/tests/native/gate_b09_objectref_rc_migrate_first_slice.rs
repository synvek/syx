//! ObjectRef RC migrate first-slice
//! MemoryPool type stub retired; migrate surface documented; ObjectRef still present; cleanup/enable false.

#[test]
fn b09_objectref_rc_migrate_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-vm/src/lib.rs",
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "docs/gates/gate-b09-objectref-rc-migrate-first-slice.md",
        "docs/gates/gate-b09-memory-pool-dead-shell-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 ObjectRef RC migrate first-slice requires evidence file: {path}"
        );
    }
    let vm = include_str!("../../../syx-vm/src/lib.rs");
    assert!(
        (vm.contains("ObjectRef") || vm.contains("ObjectRefRetain"))
            && vm.contains("run_gc_collect")
            && vm.contains("ADR-840"),
        "VmInstance must keep ObjectRef and cite ADR-840"
    );
    assert!(
        !vm.contains("pub struct MemoryPool"),
        "MemoryPool type stub must be retired (ADR-840)"
    );
    assert!(
        vm.contains("MemoryPool") || vm.contains("ObjectRef"),
        "historical MemoryPool / ObjectRef honesty strings must remain"
    );
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09ObjectRefRcMigrateFirstSliceInventory")
            && rt.contains("gate_b09_objectref_rc_migrate_first_slice_inventory")
            && rt.contains("ADR-840")
            && (rt.contains("b09_objectref_rc_migrate_first_slice_signed: true")
                || rt.contains("b09_objectref_rc_migrate_first_slice_signed:true")),
        "syx-rt must expose GateB09ObjectRefRcMigrateFirstSliceInventory"
    );
    assert!(
        (rt.contains("memory_pool_type_stub_retired: true")
            || rt.contains("memory_pool_type_stub_retired:true"))
            && (rt.contains("objectref_migrate_surface_documented: true")
                || rt.contains("objectref_migrate_surface_documented:true"))
            && (rt.contains("objectref_rc_storage_still_present: true")
                || rt.contains("objectref_rc_storage_still_present:true"))
            && (rt.contains("objectref_rc_migrate_enabled: false")
                || rt.contains("objectref_rc_migrate_enabled:false"))
            && (rt.contains("storage_cleanup_enabled: false")
                || rt.contains("storage_cleanup_enabled:false"))
            && (rt.contains("unified_mir_runtime_heap_enabled: false")
                || rt.contains("unified_mir_runtime_heap_enabled:false")),
        "inventory must retire MemoryPool stub, document surface, keep migrate/cleanup/enable false"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_objectref_rc_migrate_first_slice_inventory")
            && lib.contains("GateB09ObjectRefRcMigrateFirstSliceInventory"),
        "syx-rt lib.rs must re-export ObjectRef migrate inventory"
    );
}

#[test]
fn b09_objectref_rc_migrate_first_slice_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b09-objectref-rc-migrate-first-slice.md");
    assert!(
        spec.contains("ADR-840")
            && (spec.contains("migrate")
                || spec.contains("ObjectRef")
                || spec.contains("MemoryPool")),
        "spec must cite ADR-840 ObjectRef RC migrate first-slice"
    );
    assert!(
        spec.contains("three-hundred-and-forty-sixth"),
        
        "spec must cite W496 fingerprint"
    );
    assert!(
        spec.contains("GateB09ObjectRefRcMigrateFirstSliceInventory")
            && spec.contains("gate_b09_objectref_rc_migrate_first_slice_inventory"),
        "spec must name ObjectRef migrate inventory API"
    );
    assert!(
        spec.contains("b09_objectref_rc_migrate_first_slice_signed=true")
            || spec.contains("`b09_objectref_rc_migrate_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("memory_pool_type_stub_retired=true")
            || spec.contains("`memory_pool_type_stub_retired=true`"))
            && (spec.contains("objectref_migrate_surface_documented=true")
                || spec.contains("`objectref_migrate_surface_documented=true`"))
            && (spec.contains("objectref_rc_migrate_enabled=false")
                || spec.contains("`objectref_rc_migrate_enabled=false`"))
            && (spec.contains("storage_cleanup_enabled=false")
                || spec.contains("`storage_cleanup_enabled=false`"))
            && (spec.contains("objectref_rc_storage_still_present=true")
                || spec.contains("`objectref_rc_storage_still_present=true`")),
        "spec must cite stub retired, surface documented, migrate/cleanup false, ObjectRef present"
    );
    assert!(
        !spec.contains("unified_mir_runtime_heap_enabled=true")
            && !spec.contains("`unified_mir_runtime_heap_enabled=true`")
            && !spec.contains("objectref_rc_migrate_enabled=true")
            && !spec.contains("`objectref_rc_migrate_enabled=true`"),
        "spec must not claim unified heap or full migrate enabled"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("unified") || spec.contains("cleanup") || spec.contains("B-09")),
        "spec Frozen must fence enable / cleanup / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_objectref_rc_migrate_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_objectref_rc_migrate_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_memory_pool_dead_shell_first_slice_floor_ok);
    assert!(inv.memory_pool_type_stub_retired);
    assert!(inv.objectref_migrate_surface_documented);
    assert!(inv.objectref_rc_storage_still_present);
    assert!(!inv.objectref_rc_migrate_enabled);
    assert!(!inv.storage_cleanup_enabled);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_objectref_rc_migrate_first_slice_signed);
    let ds = syx_rt::gate_b09_memory_pool_dead_shell_first_slice_inventory();
    assert!(ds.memory_pool_alloc_retired);
    assert!(!ds.storage_cleanup_enabled);
    assert!(ds.b09_memory_pool_dead_shell_first_slice_signed);
}

