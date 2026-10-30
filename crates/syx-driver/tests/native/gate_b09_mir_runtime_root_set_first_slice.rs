//! MirRuntime root-set first-slice
//! suspend_frame + locals_frames wired; ≠ enable / ≠ mailbox Value queue / ≠ B-09 [x].

#[test]
fn b09_mir_runtime_root_set_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-mir-run/src/runtime.rs",
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "docs/gates/gate-b09-mir-runtime-root-set-first-slice.md",
        "docs/gates/gate-b09-gc-narrative-deepen.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 MirRuntime root-set first-slice requires evidence file: {path}"
        );
    }
    let runtime = include_str!("../../../syx-mir-run/src/runtime.rs");
    assert!(
        runtime.contains("collect_live_roots")
            && runtime.contains("ADR-831")
            && runtime.contains("suspend_frame")
            && runtime.contains("locals_frames"),
        "MirRuntime must expose collect_live_roots wiring suspend_frame + locals_frames (ADR-831)"
    );
    assert!(
        !runtime.contains("collect managed heap roots from stack/locals/globals/actor mailboxes"),
        "obsolete mailbox-scanned comment must be removed"
    );
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09MirRuntimeRootSetFirstSliceInventory")
            && rt.contains("gate_b09_mir_runtime_root_set_first_slice_inventory")
            && rt.contains("ADR-831")
            && (rt.contains("b09_mir_runtime_root_set_first_slice_signed: true")
                || rt.contains("b09_mir_runtime_root_set_first_slice_signed:true")),
        "syx-rt must expose GateB09MirRuntimeRootSetFirstSliceInventory"
    );
    assert!(
        (rt.contains("mir_runtime_suspend_frame_roots_wired: true")
            || rt.contains("mir_runtime_suspend_frame_roots_wired:true"))
            && (rt.contains("mir_runtime_locals_frames_roots_wired: true")
                || rt.contains("mir_runtime_locals_frames_roots_wired:true"))
            && (rt.contains("actor_mailbox_value_queue_roots_wired: false")
                || rt.contains("actor_mailbox_value_queue_roots_wired:false"))
            && (rt.contains("unified_mir_runtime_heap_enabled: false")
                || rt.contains("unified_mir_runtime_heap_enabled:false")),
        "inventory must wire suspend/locals_frames and keep mailbox Value / enable false"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_mir_runtime_root_set_first_slice_inventory")
            && lib.contains("GateB09MirRuntimeRootSetFirstSliceInventory"),
        "syx-rt lib.rs must re-export root-set first-slice inventory"
    );
}

#[test]
fn b09_mir_runtime_root_set_first_slice_spec_signed() {
    let spec = include_str!("../../../../docs/gates/gate-b09-mir-runtime-root-set-first-slice.md");
    assert!(
        spec.contains("ADR-831")
            && (spec.contains("root-set")
                || spec.contains("根集")
                || spec.contains("suspend_frame")),
        "spec must cite ADR-831 MirRuntime root-set first-slice"
    );
    assert!(
        spec.contains("three-hundred-and-thirty-seventh"),
        
        "spec must cite W487 fingerprint"
    );
    assert!(
        spec.contains("GateB09MirRuntimeRootSetFirstSliceInventory")
            && spec.contains("gate_b09_mir_runtime_root_set_first_slice_inventory"),
        "spec must name root-set inventory API"
    );
    assert!(
        spec.contains("b09_mir_runtime_root_set_first_slice_signed=true")
            || spec.contains("`b09_mir_runtime_root_set_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("mir_runtime_suspend_frame_roots_wired=true")
            || spec.contains("`mir_runtime_suspend_frame_roots_wired=true`"))
            && (spec.contains("mir_runtime_locals_frames_roots_wired=true")
                || spec.contains("`mir_runtime_locals_frames_roots_wired=true`"))
            && (spec.contains("actor_mailbox_value_queue_roots_wired=false")
                || spec.contains("`actor_mailbox_value_queue_roots_wired=false`"))
            && (spec.contains("unified_mir_runtime_heap_enabled=false")
                || spec.contains("`unified_mir_runtime_heap_enabled=false`")),
        "spec must cite wiring true and mailbox/enable false"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("unified") || spec.contains("B-09") || spec.contains("mailbox")),
        "spec Frozen must fence enable / B-09 [x] / mailbox Value"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_mir_runtime_root_set_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_mir_runtime_root_set_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_gc_narrative_deepen_floor_ok);
    assert!(inv.mir_runtime_suspend_frame_roots_wired);
    assert!(inv.mir_runtime_locals_frames_roots_wired);
    assert!(!inv.actor_mailbox_value_queue_roots_wired);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_mir_runtime_root_set_first_slice_signed);
    let deepen = syx_rt::gate_b09_gc_narrative_deepen_inventory();
    assert!(!deepen.unified_mir_runtime_heap_enabled);
    assert!(deepen.b09_gc_narrative_deepen_signed);
}

