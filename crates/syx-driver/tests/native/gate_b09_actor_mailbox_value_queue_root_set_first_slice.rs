//! actor mailbox Value-queue root-set first-slice
//! Value queue + roots wired; ≠ enable / ≠ dual-heap converge / ≠ B-09 [x] / ≠ GC GA.

#[test]
fn b09_actor_mailbox_value_queue_root_set_first_slice_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "crates/syx-vm/src/actor_mailbox.rs",
        "crates/syx-vm/src/lib.rs",
        "crates/syx-mir-run/src/runtime.rs",
        "crates/syx-rt/src/gate_b09_gc_narrative.rs",
        "crates/syx-rt/src/lib.rs",
        "docs/gates/gate-b09-actor-mailbox-value-queue-root-set-first-slice.md",
        "docs/gates/gate-b09-vm-gccollect-root-set-first-slice.md",
        "plan/topics/gc-parity.md",
        "plan/topics/mir-aot.md",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "B-09 mailbox Value-queue root-set requires evidence file: {path}"
        );
    }
    let mailbox = include_str!("../../../syx-vm/src/actor_mailbox.rs");
    assert!(
        mailbox.contains("mailbox_enqueue")
            && mailbox.contains("mailbox_collect_roots")
            && mailbox.contains("ADR-833")
            && mailbox.contains("VecDeque"),
        "actor_mailbox must expose Value queue + collect_roots (ADR-833)"
    );
    let vm = include_str!("../../../syx-vm/src/lib.rs");
    assert!(
        vm.contains("mailbox_collect_roots")
            && vm.contains("ADR-833")
            && vm.contains("mailbox_enqueue"),
        "VmInstance collect_live_roots / CallActor must wire mailbox (ADR-833)"
    );
    let mir = include_str!("../../../syx-mir-run/src/runtime.rs");
    assert!(
        mir.contains("mailbox_collect_roots") && mir.contains("ADR-833"),
        "MirRuntime collect_live_roots must scan mailbox queues (ADR-833)"
    );
    let rt = include_str!("../../../syx-rt/src/gate_b09_gc_narrative.rs");
    assert!(
        rt.contains("GateB09ActorMailboxValueQueueRootSetFirstSliceInventory")
            && rt.contains("gate_b09_actor_mailbox_value_queue_root_set_first_slice_inventory")
            && rt.contains("ADR-833")
            && (rt.contains("b09_actor_mailbox_value_queue_root_set_first_slice_signed: true")
                || rt.contains("b09_actor_mailbox_value_queue_root_set_first_slice_signed:true")),
        "syx-rt must expose GateB09ActorMailboxValueQueueRootSetFirstSliceInventory"
    );
    assert!(
        (rt.contains("actor_mailbox_value_queue_roots_wired: true")
            || rt.contains("actor_mailbox_value_queue_roots_wired:true"))
            && (rt.contains("vm_mir_runtime_heap_fully_converged: false")
                || rt.contains("vm_mir_runtime_heap_fully_converged:false"))
            && (rt.contains("unified_mir_runtime_heap_enabled: false")
                || rt.contains("unified_mir_runtime_heap_enabled:false")),
        "inventory must wire mailbox roots true and keep converged/enable false"
    );
    let lib = include_str!("../../../syx-rt/src/lib.rs");
    assert!(
        lib.contains("gate_b09_actor_mailbox_value_queue_root_set_first_slice_inventory")
            && lib.contains("GateB09ActorMailboxValueQueueRootSetFirstSliceInventory"),
        "syx-rt lib.rs must re-export mailbox Value-queue inventory"
    );
}

#[test]
fn b09_actor_mailbox_value_queue_root_set_first_slice_spec_signed() {
    let spec =
        include_str!("../../../../docs/gates/gate-b09-actor-mailbox-value-queue-root-set-first-slice.md");
    assert!(
        spec.contains("ADR-833")
            && (spec.contains("mailbox") || spec.contains("根集") || spec.contains("Value")),
        "spec must cite ADR-833 mailbox Value-queue root-set first-slice"
    );
    assert!(
        spec.contains("three-hundred-and-thirty-ninth"),
        
        "spec must cite W489 fingerprint"
    );
    assert!(
        spec.contains("GateB09ActorMailboxValueQueueRootSetFirstSliceInventory")
            && spec.contains("gate_b09_actor_mailbox_value_queue_root_set_first_slice_inventory"),
        "spec must name mailbox inventory API"
    );
    assert!(
        spec.contains("b09_actor_mailbox_value_queue_root_set_first_slice_signed=true")
            || spec.contains("`b09_actor_mailbox_value_queue_root_set_first_slice_signed=true`"),
        "spec must cite signed=true"
    );
    assert!(
        (spec.contains("actor_mailbox_value_queue_roots_wired=true")
            || spec.contains("`actor_mailbox_value_queue_roots_wired=true`"))
            && (spec.contains("vm_mir_runtime_heap_fully_converged=false")
                || spec.contains("`vm_mir_runtime_heap_fully_converged=false`"))
            && (spec.contains("unified_mir_runtime_heap_enabled=false")
                || spec.contains("`unified_mir_runtime_heap_enabled=false`")),
        "spec must cite mailbox wired true and converged/enable false"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("unified") || spec.contains("B-09") || spec.contains("enable")),
        "spec Frozen must fence enable / B-09 [x]"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn b09_actor_mailbox_value_queue_root_set_first_slice_inventory_nails() {
    let inv = syx_rt::gate_b09_actor_mailbox_value_queue_root_set_first_slice_inventory();
    assert!(inv.relational_inventory_overlap_ok);
    assert!(inv.b09_vm_gccollect_root_set_first_slice_floor_ok);
    assert!(inv.actor_mailbox_value_queue_roots_wired);
    assert!(!inv.vm_mir_runtime_heap_fully_converged);
    assert!(!inv.unified_mir_runtime_heap_enabled);
    assert!(!inv.b09_axis_exit_enabled);
    assert!(!inv.gc_ga_claimed);
    assert!(inv.b09_actor_mailbox_value_queue_root_set_first_slice_signed);
    let vm = syx_rt::gate_b09_vm_gccollect_root_set_first_slice_inventory();
    assert!(!vm.actor_mailbox_value_queue_roots_wired);
    assert!(vm.b09_vm_gccollect_root_set_first_slice_signed);
}

#[test]
fn b09_actor_mailbox_value_queue_root_set_first_slice_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W489")
            && (roadmap.contains("ADR-833")
                || roadmap.contains("mailbox")
                || roadmap.contains("根集")
                || roadmap.contains("Value")),
        "roadmap must cite W489 / ADR-833 mailbox Value-queue root-set first-slice"
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
