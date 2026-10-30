//! unified MirRuntime heap enable product flip first-slice
//! Enable product ON after cleanup
//! migrate+cleanup+readiness floors met; axis/GA still false
//! ≠ [x] ≠ GC GA.

#[test]
fn b09_unified_mir_runtime_heap_enable_product_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "docs/gates/gate-b09-unified-mir-runtime-heap-enable-product-first-slice.md",
        "docs/gates/gate-b09-storage-cleanup-product-first-slice.md",
        "docs/gates/gate-b09-enable-readiness-thin.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 unified enable product flip requires evidence file: {path}"
        );
    }
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09UnifiedMirRuntimeHeapEnableProductFirstSliceInventory")
            && rt.contains("gate_b09_unified_mir_runtime_heap_enable_product_first_slice_inventory")
            && rt.contains("pub fn unified_mir_runtime_heap_enabled")
            && rt.contains("ADR-901")
            && (rt.contains(
                "b09_unified_mir_runtime_heap_enable_product_first_slice_signed: true",
            ) || rt.contains(
                "b09_unified_mir_runtime_heap_enable_product_first_slice_signed:true",
            )),
        "syx-rt must expose GateB09UnifiedMirRuntimeHeapEnableProductFirstSliceInventory + API"
    );
    assert!(
        (rt.contains("unified_mir_runtime_heap_enabled: true")
            || rt.contains("unified_mir_runtime_heap_enabled:true"))
            && (rt.contains("unified_enable_product_flip_met: true")
                || rt.contains("unified_enable_product_flip_met:true"))
            && (rt.contains("sharedcell_arc_lifetime_still_present: true")
                || rt.contains("sharedcell_arc_lifetime_still_present:true"))
            && (rt.contains("b09_axis_exit_enabled: false")
                || rt.contains("b09_axis_exit_enabled:false")),
        "inventory must document enable true, product flip met, axis exit false"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_unified_mir_runtime_heap_enable_product_first_slice_inventory")
            && lib.contains("GateB09UnifiedMirRuntimeHeapEnableProductFirstSliceInventory")
            && lib.contains("unified_mir_runtime_heap_enabled"),
        "syx-rt lib.rs must re-export unified enable product inventory + API"
    );
}

#[test]
fn b09_unified_mir_runtime_heap_enable_product_first_slice_spec_signed() {
    let spec = include_str!(
        "../../../../docs/gates/gate-b09-unified-mir-runtime-heap-enable-product-first-slice.md"
    );
    assert!(
        spec.contains("ADR-901")
            && (spec.contains("unified")
                || spec.contains("enable product")
                || spec.contains("unified_mir_runtime_heap_enabled")),
        "spec must cite ADR-901 unified MirRuntime heap enable product flip"
    );
    assert!(
        spec.contains("four-hundred-and-seventh"),
        
        "spec must cite W557 fingerprint"
    );
    assert!(
        spec.contains("GateB09UnifiedMirRuntimeHeapEnableProductFirstSliceInventory")
            && spec.contains("gate_b09_unified_mir_runtime_heap_enable_product_first_slice_inventory")
            && spec.contains("unified_mir_runtime_heap_enabled()"),
        "spec must name unified enable product inventory + API"
    );
    assert!(
        spec.contains("b09_unified_mir_runtime_heap_enable_product_first_slice_signed=true")
            || spec.contains(
                "`b09_unified_mir_runtime_heap_enable_product_first_slice_signed=true`",
            ),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("unified_mir_runtime_heap_enabled=true")
            || spec.contains("`unified_mir_runtime_heap_enabled=true`"))
            && (spec.contains("unified_enable_product_flip_met=true")
                || spec.contains("`unified_enable_product_flip_met=true`"))
            && (spec.contains("b09_axis_exit_enabled=false")
                || spec.contains("`b09_axis_exit_enabled=false`")),
        "spec must cite enable true, product flip met, axis exit false"
    );
    assert!(
        !spec.contains("b09_axis_exit_enabled=true")
            && !spec.contains("`b09_axis_exit_enabled=true`")
            && !spec.contains("gc_ga_claimed=true")
            && !spec.contains("`gc_ga_claimed=true`"),
        "spec must not claim axis exit or GC GA"
    );
    assert!(
        spec.contains("## Frozen") && (spec.contains("[x]") || spec.contains("B-09")),
        "spec Frozen must fence B-09 [x] / GA"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_unified_mir_runtime_heap_enable_product_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_unified_mir_runtime_heap_enable_product_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_storage_cleanup_product_first_slice_floor_ok);
    assert!(inv.storage_cleanup_enabled);
    assert!(inv.storage_cleanup_product_flip_met);
    assert!(inv.objectref_rc_migrate_enabled);
    assert!(inv.migrate_product_flip_met);
    assert!(inv.b09_enable_readiness_thin_floor_ok);
    assert!(inv.sharedcell_arc_lifetime_still_present);
    assert!(inv.unified_mir_runtime_heap_enabled);
    assert!(inv.unified_enable_product_flip_met);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_unified_mir_runtime_heap_enable_product_first_slice_signed);
    assert!(syx_rt::unified_mir_runtime_heap_enabled());
    assert_eq!(
        syx_rt::unified_mir_runtime_heap_enabled(),
        inv.unified_mir_runtime_heap_enabled
    );
    let floor = syx_rt::gate_b09_storage_cleanup_product_first_slice_inventory();
    assert!(floor.b09_storage_cleanup_product_first_slice_signed);
    assert!(floor.storage_cleanup_enabled);
    assert!(!floor.unified_mir_runtime_heap_enabled);
    assert!(!floor.b09_axis_exit_enabled);
}


