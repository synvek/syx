//! storage-cleanup product flip first-slice
//! Cleanup product ON after migrate
//! dead-API/MemoryPool floors met; enable/[x] still false
//! ≠ enable ≠ [x].

#[test]
fn b09_storage_cleanup_product_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "docs/gates/gate-b09-storage-cleanup-product-first-slice.md",
        "docs/gates/gate-b09-objectref-rc-migrate-product-first-slice.md",
        "docs/gates/gate-b09-storage-cleanup-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 storage-cleanup product flip requires evidence file: {path}"
        );
    }
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09StorageCleanupProductFirstSliceInventory")
            && rt.contains("gate_b09_storage_cleanup_product_first_slice_inventory")
            && rt.contains("pub fn storage_cleanup_enabled")
            && rt.contains("ADR-900")
            && (rt.contains("b09_storage_cleanup_product_first_slice_signed: true")
                || rt.contains("b09_storage_cleanup_product_first_slice_signed:true")),
        "syx-rt must expose GateB09StorageCleanupProductFirstSliceInventory + API"
    );
    assert!(
        (rt.contains("storage_cleanup_enabled: true")
            || rt.contains("storage_cleanup_enabled:true"))
            && (rt.contains("storage_cleanup_product_flip_met: true")
                || rt.contains("storage_cleanup_product_flip_met:true"))
            && (rt.contains("sharedcell_arc_lifetime_still_present: true")
                || rt.contains("sharedcell_arc_lifetime_still_present:true"))
            && (rt.contains("unified_mir_runtime_heap_enabled: false")
                || rt.contains("unified_mir_runtime_heap_enabled:false")),
        "inventory must document cleanup true, product flip met, enable false"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_storage_cleanup_product_first_slice_inventory")
            && lib.contains("GateB09StorageCleanupProductFirstSliceInventory")
            && lib.contains("storage_cleanup_enabled"),
        "syx-rt lib.rs must re-export storage-cleanup product inventory + API"
    );
}

#[test]
fn b09_storage_cleanup_product_first_slice_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b09-storage-cleanup-product-first-slice.md");
    assert!(
        spec.contains("ADR-900")
            && (spec.contains("storage-cleanup")
                || spec.contains("product flip")
                || spec.contains("storage_cleanup_enabled")),
        "spec must cite ADR-900 storage-cleanup product flip"
    );
    assert!(
        spec.contains("four-hundred-and-sixth"),
        
        "spec must cite W556 fingerprint"
    );
    assert!(
        spec.contains("GateB09StorageCleanupProductFirstSliceInventory")
            && spec.contains("gate_b09_storage_cleanup_product_first_slice_inventory")
            && spec.contains("storage_cleanup_enabled()"),
        "spec must name storage-cleanup product inventory + API"
    );
    assert!(
        spec.contains("b09_storage_cleanup_product_first_slice_signed=true")
            || spec.contains("`b09_storage_cleanup_product_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("storage_cleanup_enabled=true")
            || spec.contains("`storage_cleanup_enabled=true`"))
            && (spec.contains("storage_cleanup_product_flip_met=true")
                || spec.contains("`storage_cleanup_product_flip_met=true`"))
            && (spec.contains("unified_mir_runtime_heap_enabled=false")
                || spec.contains("`unified_mir_runtime_heap_enabled=false`")),
        "spec must cite cleanup true, product flip met, enable false"
    );
    assert!(
        !spec.contains("unified_mir_runtime_heap_enabled=true")
            && !spec.contains("`unified_mir_runtime_heap_enabled=true`")
            && !spec.contains("b09_axis_exit_enabled=true")
            && !spec.contains("`b09_axis_exit_enabled=true`"),
        "spec must not claim unified heap or axis exit enabled"
    );
    assert!(
        spec.contains("## Frozen") && (spec.contains("enable") || spec.contains("B-09")),
        "spec Frozen must fence enable / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_storage_cleanup_product_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_storage_cleanup_product_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_objectref_rc_migrate_product_first_slice_floor_ok);
    assert!(inv.objectref_rc_migrate_enabled);
    assert!(inv.migrate_product_flip_met);
    assert!(inv.b09_storage_cleanup_first_slice_floor_ok);
    assert!(inv.storage_cleanup_dead_api_retired);
    assert!(inv.memory_pool_type_stub_retired);
    assert!(inv.sharedcell_arc_lifetime_still_present);
    assert!(inv.storage_cleanup_enabled);
    assert!(inv.storage_cleanup_product_flip_met);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_storage_cleanup_product_first_slice_signed);
    assert!(syx_rt::storage_cleanup_enabled());
    assert_eq!(
        syx_rt::storage_cleanup_enabled(),
        inv.storage_cleanup_enabled
    );
    let floor = syx_rt::gate_b09_objectref_rc_migrate_product_first_slice_inventory();
    assert!(floor.b09_objectref_rc_migrate_product_first_slice_signed);
    assert!(floor.objectref_rc_migrate_enabled);
    assert!(!floor.storage_cleanup_enabled);
    assert!(!floor.b09_axis_exit_enabled);
}

#[test]
fn b09_storage_cleanup_product_first_slice_fingerprint_and_adr() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W556")
            && (roadmap.contains("ADR-900")
                || roadmap.contains("storage-cleanup")
                || roadmap.contains("product flip")),
        "roadmap must cite W556 / ADR-900 storage-cleanup product flip"
    );
    assert!(
        roadmap.contains("four-hundred-and-seventh")
            || roadmap.contains("four-hundred-and-sixth")
            || roadmap.contains("four-hundred-and-fifth"),
        
        "roadmap must cite W556 fingerprint"
    );
    let adr = include_str!("../../../../docs/adr/architecture-decisions.md");
    assert!(
        adr.contains("ADR-900") && adr.contains("W556"),
        "ADR must record ADR-900 / W556"
    );
    crate::matrix_fingerprint::assert_live_matrix_fingerprint();
}

#[test]
fn b09_storage_cleanup_product_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W556")
            && (roadmap.contains("ADR-900")
                || roadmap.contains("storage-cleanup")
                || roadmap.contains("product flip")),
        "roadmap must cite W556 / ADR-900 storage-cleanup product flip"
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
        "roadmap B-09 must be [x] (W558+) or historical [~] (cleanup ≠ exit): {b09}"
    );
    let parity = include_str!("../../../../plan/topics/gc-parity.md");
    assert!(
        parity.contains("W556")
            && (parity.contains("storage-cleanup")
                || parity.contains("product flip")
                || parity.contains("ADR-900")),
        "gc-parity must cite W556 storage-cleanup product flip"
    );
    assert!(
        parity.contains("still unmet")
            || parity.contains("仍 unmet")
            || parity.contains("[~]"),
        "gc-parity must keep enable/[x] unmet honesty"
    );
}
