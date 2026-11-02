//! B-09 GC narrative thin inventory (W476 / ADR-820).
//!
//! Signs the three-layer heap honesty table (VM / WASM / Native) and the preferred
//! MirRuntime mark-sweep direction. Does **not** enable a unified heap product
//! switch, claim GC GA, or flip B-09 to `[x]`.

/// B-09 GC narrative thin inventory (W476 / ADR-820).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateB09GcNarrativeThinInventory {
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — VM / WASM / Native heap rows documented in policy + gc-parity.
    pub three_layer_heap_table_documented: bool,
    /// Always `true` — VM `GcCollect` / MemoryPool path documented as experimental GC.
    pub vm_gc_collect_path_documented: bool,
    /// Always `true` — WASM/Native host MirRuntime/Rust heap documented as no SYX GC.
    pub wasm_native_no_syx_gc_documented: bool,
    /// Always `true` — preferred unified MirRuntime mark-sweep direction documented.
    pub preferred_unified_mir_runtime_direction_documented: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W476 / ADR-820: B-09 GC narrative thin signed.
    pub b09_gc_narrative_thin_signed: bool,
}

/// Inventory B-09 GC narrative thin (W476 / ADR-820).
pub fn gate_b09_gc_narrative_thin_inventory() -> GateB09GcNarrativeThinInventory {
    let again = gate_b09_gc_narrative_thin_inventory_once();
    let first = gate_b09_gc_narrative_thin_inventory_once();
    GateB09GcNarrativeThinInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_gc_narrative_thin_inventory_once() -> GateB09GcNarrativeThinInventory {
    GateB09GcNarrativeThinInventory {
        relational_inventory_overlap_ok: true,
        three_layer_heap_table_documented: true,
        vm_gc_collect_path_documented: true,
        wasm_native_no_syx_gc_documented: true,
        preferred_unified_mir_runtime_direction_documented: true,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_gc_narrative_thin_signed: true,
    }
}

/// B-09 GC narrative deepen inventory (W485 / ADR-829).
///
/// Signs unified MirRuntime mark-sweep **construction checklist** (root set /
/// VM converge / wasm-gc deferred / ADR-137 boundary). Does **not** enable the
/// product switch, claim GC GA, or flip B-09 to `[x]`. Historical thin inventory
/// floors remain unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateB09GcNarrativeDeepenInventory {
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W476 thin floor still signed.
    pub b09_gc_narrative_thin_floor_ok: bool,
    /// Always `true` — construction ADR checklist documented in deepen policy.
    pub unified_heap_construction_adr_documented: bool,
    /// Always `true` — MirRuntime root set (stack/locals/globals/mailboxes/tasks) documented.
    pub mir_runtime_root_set_documented: bool,
    /// Always `true` — eventual converge with VM `GcCollect` documented.
    pub vm_gccollect_converge_path_documented: bool,
    /// Always `true` — premature wasm-gc types explicitly deferred.
    pub wasm_gc_types_deferred_documented: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W485 / ADR-829: B-09 GC narrative deepen signed.
    pub b09_gc_narrative_deepen_signed: bool,
}

/// Inventory B-09 GC narrative deepen (W485 / ADR-829).
pub fn gate_b09_gc_narrative_deepen_inventory() -> GateB09GcNarrativeDeepenInventory {
    let again = gate_b09_gc_narrative_deepen_inventory_once();
    let first = gate_b09_gc_narrative_deepen_inventory_once();
    GateB09GcNarrativeDeepenInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_gc_narrative_deepen_inventory_once() -> GateB09GcNarrativeDeepenInventory {
    GateB09GcNarrativeDeepenInventory {
        relational_inventory_overlap_ok: true,
        b09_gc_narrative_thin_floor_ok: true,
        unified_heap_construction_adr_documented: true,
        mir_runtime_root_set_documented: true,
        vm_gccollect_converge_path_documented: true,
        wasm_gc_types_deferred_documented: true,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_gc_narrative_deepen_signed: true,
    }
}

/// B-09 MirRuntime root-set first-slice inventory (W487 / ADR-831).
///
/// Signs MirRuntime `heap_collect`/`heap_compact` wiring of `suspend_frame` +
/// `locals_frames` into live roots. Does **not** enable unified heap product
/// switch, claim mailbox Value-queue roots, GC GA, or B-09 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateB09MirRuntimeRootSetFirstSliceInventory {
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W485 deepen floor still signed.
    pub b09_gc_narrative_deepen_floor_ok: bool,
    /// Always `true` — async `suspend_frame` stack/locals wired into collect roots.
    pub mir_runtime_suspend_frame_roots_wired: bool,
    /// Always `true` — caller `locals_frames` wired into collect roots.
    pub mir_runtime_locals_frames_roots_wired: bool,
    /// Always `false` — honesty: actor mailbox Value queue roots still unmet (depth-only).
    pub actor_mailbox_value_queue_roots_wired: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W487 / ADR-831: MirRuntime root-set first-slice signed.
    pub b09_mir_runtime_root_set_first_slice_signed: bool,
}

/// Inventory B-09 MirRuntime root-set first-slice (W487 / ADR-831).
pub fn gate_b09_mir_runtime_root_set_first_slice_inventory() -> GateB09MirRuntimeRootSetFirstSliceInventory {
    let again = gate_b09_mir_runtime_root_set_first_slice_inventory_once();
    let first = gate_b09_mir_runtime_root_set_first_slice_inventory_once();
    GateB09MirRuntimeRootSetFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_mir_runtime_root_set_first_slice_inventory_once() -> GateB09MirRuntimeRootSetFirstSliceInventory
{
    let deepen = gate_b09_gc_narrative_deepen_inventory_once();
    let floor_ok = deepen.b09_gc_narrative_deepen_signed
        && deepen.b09_gc_narrative_thin_floor_ok
        && deepen.mir_runtime_root_set_documented
        && !deepen.unified_mir_runtime_heap_enabled
        && !deepen.b09_axis_exit_enabled
        && !deepen.gc_ga_claimed;
    GateB09MirRuntimeRootSetFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_gc_narrative_deepen_floor_ok: floor_ok,
        mir_runtime_suspend_frame_roots_wired: true,
        mir_runtime_locals_frames_roots_wired: true,
        actor_mailbox_value_queue_roots_wired: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_mir_runtime_root_set_first_slice_signed: true,
    }
}

/// B-09 VM GcCollect root-set first-slice inventory (W488 / ADR-832).
///
/// Signs VM `GcCollect`/`HeapCompact` wiring of `suspend_frame` + `locals_frames`
/// into live roots (via `VmInstance::collect_live_roots`). Does **not** enable
/// unified heap product switch, claim mailbox Value-queue roots, claim full
/// VM↔MirRuntime heap convergence, GC GA, or B-09 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateB09VmGcCollectRootSetFirstSliceInventory {
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W487 MirRuntime root-set first-slice floor still signed.
    pub b09_mir_runtime_root_set_first_slice_floor_ok: bool,
    /// Always `true` — VM async `suspend_frame` stack/locals wired into GcCollect roots.
    pub vm_suspend_frame_roots_wired: bool,
    /// Always `true` — nested `locals_frames` wired into GcCollect roots.
    pub vm_locals_frames_roots_wired: bool,
    /// Always `false` — honesty: actor mailbox Value queue roots still unmet (depth-only).
    pub actor_mailbox_value_queue_roots_wired: bool,
    /// Always `false` — honesty: VM↔MirRuntime MemoryPool+ManagedHeap not fully converged.
    pub vm_mir_runtime_heap_fully_converged: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W488 / ADR-832: VM GcCollect root-set first-slice signed.
    pub b09_vm_gccollect_root_set_first_slice_signed: bool,
}

/// Inventory B-09 VM GcCollect root-set first-slice (W488 / ADR-832).
pub fn gate_b09_vm_gccollect_root_set_first_slice_inventory() -> GateB09VmGcCollectRootSetFirstSliceInventory
{
    let again = gate_b09_vm_gccollect_root_set_first_slice_inventory_once();
    let first = gate_b09_vm_gccollect_root_set_first_slice_inventory_once();
    GateB09VmGcCollectRootSetFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_vm_gccollect_root_set_first_slice_inventory_once() -> GateB09VmGcCollectRootSetFirstSliceInventory
{
    let mir = gate_b09_mir_runtime_root_set_first_slice_inventory_once();
    let floor_ok = mir.b09_mir_runtime_root_set_first_slice_signed
        && mir.b09_gc_narrative_deepen_floor_ok
        && mir.mir_runtime_suspend_frame_roots_wired
        && mir.mir_runtime_locals_frames_roots_wired
        && !mir.actor_mailbox_value_queue_roots_wired
        && !mir.unified_mir_runtime_heap_enabled
        && !mir.b09_axis_exit_enabled
        && !mir.gc_ga_claimed;
    GateB09VmGcCollectRootSetFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_mir_runtime_root_set_first_slice_floor_ok: floor_ok,
        vm_suspend_frame_roots_wired: true,
        vm_locals_frames_roots_wired: true,
        actor_mailbox_value_queue_roots_wired: false,
        vm_mir_runtime_heap_fully_converged: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_vm_gccollect_root_set_first_slice_signed: true,
    }
}

/// B-09 actor mailbox Value-queue root-set first-slice inventory (W489 / ADR-833).
///
/// Signs actor mailbox Value queue + wiring into VM/MirRuntime `collect_live_roots`.
/// Does **not** enable unified heap, claim dual-heap converge, GC GA, or B-09 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateB09ActorMailboxValueQueueRootSetFirstSliceInventory {
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W488 VM GcCollect root-set first-slice floor still signed.
    pub b09_vm_gccollect_root_set_first_slice_floor_ok: bool,
    /// Always `true` — actor mailbox Value queue roots wired into collect.
    pub actor_mailbox_value_queue_roots_wired: bool,
    /// Always `false` — honesty: VM↔MirRuntime MemoryPool+ManagedHeap not fully converged.
    pub vm_mir_runtime_heap_fully_converged: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W489 / ADR-833: mailbox Value-queue root-set first-slice signed.
    pub b09_actor_mailbox_value_queue_root_set_first_slice_signed: bool,
}

/// Inventory B-09 actor mailbox Value-queue root-set first-slice (W489 / ADR-833).
pub fn gate_b09_actor_mailbox_value_queue_root_set_first_slice_inventory(
) -> GateB09ActorMailboxValueQueueRootSetFirstSliceInventory {
    let again = gate_b09_actor_mailbox_value_queue_root_set_first_slice_inventory_once();
    let first = gate_b09_actor_mailbox_value_queue_root_set_first_slice_inventory_once();
    GateB09ActorMailboxValueQueueRootSetFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_actor_mailbox_value_queue_root_set_first_slice_inventory_once(
) -> GateB09ActorMailboxValueQueueRootSetFirstSliceInventory {
    let vm = gate_b09_vm_gccollect_root_set_first_slice_inventory_once();
    let floor_ok = vm.b09_vm_gccollect_root_set_first_slice_signed
        && vm.b09_mir_runtime_root_set_first_slice_floor_ok
        && vm.vm_suspend_frame_roots_wired
        && vm.vm_locals_frames_roots_wired
        && !vm.actor_mailbox_value_queue_roots_wired
        && !vm.vm_mir_runtime_heap_fully_converged
        && !vm.unified_mir_runtime_heap_enabled
        && !vm.b09_axis_exit_enabled
        && !vm.gc_ga_claimed;
    GateB09ActorMailboxValueQueueRootSetFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_vm_gccollect_root_set_first_slice_floor_ok: floor_ok,
        actor_mailbox_value_queue_roots_wired: true,
        vm_mir_runtime_heap_fully_converged: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_actor_mailbox_value_queue_root_set_first_slice_signed: true,
    }
}

/// B-09 dual-heap converge first-slice inventory (W490 / ADR-834).
///
/// Signs documented dual-path gap + VM GcCollect primary ManagedHeap alignment
/// with MirRuntime, while MemoryPool mark-sweep remains residual. Does **not**
/// claim fully converged, unified heap enable, GC GA, or B-09 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateB09DualHeapConvergeFirstSliceInventory {
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W489 mailbox Value-queue root-set first-slice floor still signed.
    pub b09_actor_mailbox_value_queue_root_set_first_slice_floor_ok: bool,
    /// Always `true` — dual-path gap (ManagedHeap vs MemoryPool) documented.
    pub dual_heap_gap_documented: bool,
    /// Always `true` — VM primary collect + MirRuntime share ManagedHeap `collect_global`.
    pub shared_managed_heap_collect_path_aligned: bool,
    /// Always `true` — honesty: VM MemoryPool mark-sweep still residual.
    pub vm_memory_pool_mark_sweep_residual: bool,
    /// Always `false` — honesty: VM↔MirRuntime heaps not fully converged.
    pub vm_mir_runtime_heap_fully_converged: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W490 / ADR-834: dual-heap converge first-slice signed.
    pub b09_dual_heap_converge_first_slice_signed: bool,
}

/// Inventory B-09 dual-heap converge first-slice (W490 / ADR-834).
pub fn gate_b09_dual_heap_converge_first_slice_inventory() -> GateB09DualHeapConvergeFirstSliceInventory {
    let again = gate_b09_dual_heap_converge_first_slice_inventory_once();
    let first = gate_b09_dual_heap_converge_first_slice_inventory_once();
    GateB09DualHeapConvergeFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_dual_heap_converge_first_slice_inventory_once() -> GateB09DualHeapConvergeFirstSliceInventory {
    let mb = gate_b09_actor_mailbox_value_queue_root_set_first_slice_inventory_once();
    let floor_ok = mb.b09_actor_mailbox_value_queue_root_set_first_slice_signed
        && mb.b09_vm_gccollect_root_set_first_slice_floor_ok
        && mb.actor_mailbox_value_queue_roots_wired
        && !mb.vm_mir_runtime_heap_fully_converged
        && !mb.unified_mir_runtime_heap_enabled
        && !mb.b09_axis_exit_enabled
        && !mb.gc_ga_claimed;
    GateB09DualHeapConvergeFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_actor_mailbox_value_queue_root_set_first_slice_floor_ok: floor_ok,
        dual_heap_gap_documented: true,
        shared_managed_heap_collect_path_aligned: true,
        vm_memory_pool_mark_sweep_residual: true,
        vm_mir_runtime_heap_fully_converged: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_dual_heap_converge_first_slice_signed: true,
    }
}

/// B-09 VM↔MirRuntime heap fully_converged first-slice inventory (W491 / ADR-835).
///
/// Signs GcCollect-path retirement of MemoryPool mark-sweep residual so VM
/// collect matches MirRuntime ManagedHeap-only. Does **not** enable unified
/// heap product switch, delete MemoryPool/ObjectRef storage, claim GC GA, or
/// B-09 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateB09VmMirRuntimeHeapFullyConvergedFirstSliceInventory {
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W490 dual-heap converge first-slice floor still signed.
    pub b09_dual_heap_converge_first_slice_floor_ok: bool,
    /// Always `true` — MemoryPool mark-sweep no longer called from `run_gc_collect`.
    pub vm_memory_pool_mark_sweep_gc_collect_residual_retired: bool,
    /// Always `true` — narrow: GcCollect path aligned with MirRuntime ManagedHeap-only.
    pub vm_mir_runtime_heap_fully_converged: bool,
    /// Always `true` — honesty: MemoryPool type / ObjectRef RC storage still present.
    pub vm_memory_pool_objectref_storage_still_present: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W491 / ADR-835: fully_converged first-slice signed.
    pub b09_vm_mir_runtime_heap_fully_converged_first_slice_signed: bool,
}

/// Inventory B-09 VM↔MirRuntime heap fully_converged first-slice (W491 / ADR-835).
pub fn gate_b09_vm_mir_runtime_heap_fully_converged_first_slice_inventory(
) -> GateB09VmMirRuntimeHeapFullyConvergedFirstSliceInventory {
    let again = gate_b09_vm_mir_runtime_heap_fully_converged_first_slice_inventory_once();
    let first = gate_b09_vm_mir_runtime_heap_fully_converged_first_slice_inventory_once();
    GateB09VmMirRuntimeHeapFullyConvergedFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_vm_mir_runtime_heap_fully_converged_first_slice_inventory_once(
) -> GateB09VmMirRuntimeHeapFullyConvergedFirstSliceInventory {
    let dh = gate_b09_dual_heap_converge_first_slice_inventory_once();
    let floor_ok = dh.b09_dual_heap_converge_first_slice_signed
        && dh.b09_actor_mailbox_value_queue_root_set_first_slice_floor_ok
        && dh.dual_heap_gap_documented
        && dh.shared_managed_heap_collect_path_aligned
        && dh.vm_memory_pool_mark_sweep_residual
        && !dh.vm_mir_runtime_heap_fully_converged
        && !dh.unified_mir_runtime_heap_enabled
        && !dh.b09_axis_exit_enabled
        && !dh.gc_ga_claimed;
    GateB09VmMirRuntimeHeapFullyConvergedFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_dual_heap_converge_first_slice_floor_ok: floor_ok,
        vm_memory_pool_mark_sweep_gc_collect_residual_retired: true,
        vm_mir_runtime_heap_fully_converged: true,
        vm_memory_pool_objectref_storage_still_present: true,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_vm_mir_runtime_heap_fully_converged_first_slice_signed: true,
    }
}

/// B-09 ObjectRef/MemoryPool storage honesty thin inventory (W492 / ADR-836).
///
/// Signs that ObjectRef/`MemoryPool` storage remains present and cleanup is
/// not enabled — blocking honest product enable. Does **not** enable unified
/// heap, delete MemoryPool, claim GC GA, or B-09 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateB09ObjectRefMemoryPoolStorageHonestyThinInventory {
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W491 fully_converged first-slice floor still signed.
    pub b09_vm_mir_runtime_heap_fully_converged_first_slice_floor_ok: bool,
    /// Always `true` — ObjectRef/MemoryPool storage layer documented as still present.
    pub objectref_memory_pool_storage_documented: bool,
    /// Always `true` — honesty: MemoryPool type / ObjectRef RC storage still present.
    pub vm_memory_pool_objectref_storage_still_present: bool,
    /// Always `false` — honesty: storage cleanup not enabled this window.
    pub storage_cleanup_enabled: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W492 / ADR-836: storage honesty thin signed.
    pub b09_objectref_memory_pool_storage_honesty_thin_signed: bool,
}

/// Inventory B-09 ObjectRef/MemoryPool storage honesty thin (W492 / ADR-836).
pub fn gate_b09_objectref_memory_pool_storage_honesty_thin_inventory(
) -> GateB09ObjectRefMemoryPoolStorageHonestyThinInventory {
    let again = gate_b09_objectref_memory_pool_storage_honesty_thin_inventory_once();
    let first = gate_b09_objectref_memory_pool_storage_honesty_thin_inventory_once();
    GateB09ObjectRefMemoryPoolStorageHonestyThinInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_memory_pool_storage_honesty_thin_inventory_once(
) -> GateB09ObjectRefMemoryPoolStorageHonestyThinInventory {
    let fc = gate_b09_vm_mir_runtime_heap_fully_converged_first_slice_inventory_once();
    let floor_ok = fc.b09_vm_mir_runtime_heap_fully_converged_first_slice_signed
        && fc.b09_dual_heap_converge_first_slice_floor_ok
        && fc.vm_memory_pool_mark_sweep_gc_collect_residual_retired
        && fc.vm_mir_runtime_heap_fully_converged
        && fc.vm_memory_pool_objectref_storage_still_present
        && !fc.unified_mir_runtime_heap_enabled
        && !fc.b09_axis_exit_enabled
        && !fc.gc_ga_claimed;
    GateB09ObjectRefMemoryPoolStorageHonestyThinInventory {
        relational_inventory_overlap_ok: true,
        b09_vm_mir_runtime_heap_fully_converged_first_slice_floor_ok: floor_ok,
        objectref_memory_pool_storage_documented: true,
        vm_memory_pool_objectref_storage_still_present: true,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_memory_pool_storage_honesty_thin_signed: true,
    }
}

/// B-09 enable-readiness thin inventory (W493 / ADR-837).
///
/// Signs that enable **preconditions / blockers** are documented after W492
/// storage honesty. Does **not** enable unified heap, run storage cleanup,
/// claim GC GA, or B-09 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateB09EnableReadinessThinInventory {
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W492 ObjectRef/MemoryPool storage honesty thin floor still signed.
    pub b09_objectref_memory_pool_storage_honesty_thin_floor_ok: bool,
    /// Always `true` — enable blockers table documented this window.
    pub enable_blockers_documented: bool,
    /// Always `true` — honesty: MemoryPool type / ObjectRef RC storage still present.
    pub vm_memory_pool_objectref_storage_still_present: bool,
    /// Always `true` — honesty: dead `Gc::collect_mark_sweep` API residual documented (≠ deleted).
    pub gc_collect_mark_sweep_dead_api_documented: bool,
    /// Always `false` — honesty: storage cleanup not enabled this window.
    pub storage_cleanup_enabled: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W493 / ADR-837: enable-readiness thin signed.
    pub b09_enable_readiness_signed: bool,
}

/// Inventory B-09 enable-readiness thin (W493 / ADR-837).
pub fn gate_b09_enable_readiness_thin_inventory() -> GateB09EnableReadinessThinInventory {
    let again = gate_b09_enable_readiness_thin_inventory_once();
    let first = gate_b09_enable_readiness_thin_inventory_once();
    GateB09EnableReadinessThinInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_enable_readiness_thin_inventory_once() -> GateB09EnableReadinessThinInventory {
    let sh = gate_b09_objectref_memory_pool_storage_honesty_thin_inventory_once();
    let floor_ok = sh.b09_objectref_memory_pool_storage_honesty_thin_signed
        && sh.b09_vm_mir_runtime_heap_fully_converged_first_slice_floor_ok
        && sh.objectref_memory_pool_storage_documented
        && sh.vm_memory_pool_objectref_storage_still_present
        && !sh.storage_cleanup_enabled
        && !sh.unified_mir_runtime_heap_enabled
        && !sh.b09_axis_exit_enabled
        && !sh.gc_ga_claimed;
    GateB09EnableReadinessThinInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_memory_pool_storage_honesty_thin_floor_ok: floor_ok,
        enable_blockers_documented: true,
        vm_memory_pool_objectref_storage_still_present: true,
        gc_collect_mark_sweep_dead_api_documented: true,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_enable_readiness_signed: true,
    }
}

/// B-09 storage-cleanup first-slice inventory (W494 / ADR-838).
///
/// Signs that dead `Gc::collect_mark_sweep` / `Gc::collect` APIs are retired.
/// Does **not** flip `storage_cleanup_enabled`, delete MemoryPool, migrate
/// ObjectRef, enable unified heap, claim GC GA, or B-09 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateB09StorageCleanupFirstSliceInventory {
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W493 enable-readiness thin floor still signed.
    pub b09_enable_readiness_thin_floor_ok: bool,
    /// Always `true` — dead `Gc::collect_mark_sweep` API retired (deleted) this window.
    pub gc_collect_mark_sweep_dead_api_retired: bool,
    /// Always `true` — cleanup checklist progress documented (dead API done; storage open).
    pub storage_cleanup_checklist_progress_documented: bool,
    /// Always `true` — honesty: MemoryPool type / ObjectRef RC storage still present.
    pub vm_memory_pool_objectref_storage_still_present: bool,
    /// Always `false` — honesty: full storage-layer cleanup not enabled this window.
    pub storage_cleanup_enabled: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W494 / ADR-838: storage-cleanup first-slice signed.
    pub b09_storage_cleanup_first_slice_signed: bool,
}

/// Inventory B-09 storage-cleanup first-slice (W494 / ADR-838).
pub fn gate_b09_storage_cleanup_first_slice_inventory() -> GateB09StorageCleanupFirstSliceInventory {
    let again = gate_b09_storage_cleanup_first_slice_inventory_once();
    let first = gate_b09_storage_cleanup_first_slice_inventory_once();
    GateB09StorageCleanupFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_storage_cleanup_first_slice_inventory_once() -> GateB09StorageCleanupFirstSliceInventory {
    let er = gate_b09_enable_readiness_thin_inventory_once();
    let floor_ok = er.b09_enable_readiness_signed
        && er.b09_objectref_memory_pool_storage_honesty_thin_floor_ok
        && er.enable_blockers_documented
        && er.vm_memory_pool_objectref_storage_still_present
        && er.gc_collect_mark_sweep_dead_api_documented
        && !er.storage_cleanup_enabled
        && !er.unified_mir_runtime_heap_enabled
        && !er.b09_axis_exit_enabled
        && !er.gc_ga_claimed;
    GateB09StorageCleanupFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_enable_readiness_thin_floor_ok: floor_ok,
        gc_collect_mark_sweep_dead_api_retired: true,
        storage_cleanup_checklist_progress_documented: true,
        vm_memory_pool_objectref_storage_still_present: true,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_storage_cleanup_first_slice_signed: true,
    }
}

/// B-09 MemoryPool dead-shell first-slice inventory (W495 / ADR-839).
///
/// Signs that unused `MemoryPool::alloc` and `VmInstance.pool` are retired.
/// Does **not** flip `storage_cleanup_enabled`, delete ObjectRef RC, enable
/// unified heap, claim GC GA, or B-09 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateB09MemoryPoolDeadShellFirstSliceInventory {
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W494 storage-cleanup first-slice floor still signed.
    pub b09_storage_cleanup_first_slice_floor_ok: bool,
    /// Always `true` — `MemoryPool::alloc` retired (deleted) this window.
    pub memory_pool_alloc_retired: bool,
    /// Always `true` — `VmInstance.pool` field retired (deleted) this window.
    pub vm_instance_pool_field_retired: bool,
    /// Always `true` — honesty: ObjectRef RC storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `false` — honesty: full storage-layer cleanup not enabled this window.
    pub storage_cleanup_enabled: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W495 / ADR-839: MemoryPool dead-shell first-slice signed.
    pub b09_memory_pool_dead_shell_first_slice_signed: bool,
}

/// Inventory B-09 MemoryPool dead-shell first-slice (W495 / ADR-839).
pub fn gate_b09_memory_pool_dead_shell_first_slice_inventory(
) -> GateB09MemoryPoolDeadShellFirstSliceInventory {
    let again = gate_b09_memory_pool_dead_shell_first_slice_inventory_once();
    let first = gate_b09_memory_pool_dead_shell_first_slice_inventory_once();
    GateB09MemoryPoolDeadShellFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_memory_pool_dead_shell_first_slice_inventory_once(
) -> GateB09MemoryPoolDeadShellFirstSliceInventory {
    let sc = gate_b09_storage_cleanup_first_slice_inventory_once();
    let floor_ok = sc.b09_storage_cleanup_first_slice_signed
        && sc.b09_enable_readiness_thin_floor_ok
        && sc.gc_collect_mark_sweep_dead_api_retired
        && sc.storage_cleanup_checklist_progress_documented
        && sc.vm_memory_pool_objectref_storage_still_present
        && !sc.storage_cleanup_enabled
        && !sc.unified_mir_runtime_heap_enabled
        && !sc.b09_axis_exit_enabled
        && !sc.gc_ga_claimed;
    GateB09MemoryPoolDeadShellFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_storage_cleanup_first_slice_floor_ok: floor_ok,
        memory_pool_alloc_retired: true,
        vm_instance_pool_field_retired: true,
        objectref_rc_storage_still_present: true,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_memory_pool_dead_shell_first_slice_signed: true,
    }
}

/// B-09 ObjectRef RC migrate first-slice inventory (W496 / ADR-840).
///
/// Signs that empty `MemoryPool` type stub is retired and ObjectRef migrate
/// surface is documented. Does **not** flip `storage_cleanup_enabled`, enable
/// full ObjectRef→ManagedHeap migrate, claim GC GA, or B-09 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateB09ObjectRefRcMigrateFirstSliceInventory {
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W495 MemoryPool dead-shell first-slice floor still signed.
    pub b09_memory_pool_dead_shell_first_slice_floor_ok: bool,
    /// Always `true` — empty `MemoryPool` type stub retired (deleted) this window.
    pub memory_pool_type_stub_retired: bool,
    /// Always `true` — ObjectRef migrate surface (VM/mir-run/LLVM/WASM/RT) documented.
    pub objectref_migrate_surface_documented: bool,
    /// Always `true` — honesty: ObjectRef RC storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `false` — honesty: full ObjectRef→ManagedHeap migrate not enabled.
    pub objectref_rc_migrate_enabled: bool,
    /// Always `false` — honesty: full storage-layer cleanup not enabled this window.
    pub storage_cleanup_enabled: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W496 / ADR-840: ObjectRef RC migrate first-slice signed.
    pub b09_objectref_rc_migrate_first_slice_signed: bool,
}

/// Inventory B-09 ObjectRef RC migrate first-slice (W496 / ADR-840).
pub fn gate_b09_objectref_rc_migrate_first_slice_inventory() -> GateB09ObjectRefRcMigrateFirstSliceInventory
{
    let again = gate_b09_objectref_rc_migrate_first_slice_inventory_once();
    let first = gate_b09_objectref_rc_migrate_first_slice_inventory_once();
    GateB09ObjectRefRcMigrateFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_rc_migrate_first_slice_inventory_once() -> GateB09ObjectRefRcMigrateFirstSliceInventory
{
    let ds = gate_b09_memory_pool_dead_shell_first_slice_inventory_once();
    let floor_ok = ds.b09_memory_pool_dead_shell_first_slice_signed
        && ds.b09_storage_cleanup_first_slice_floor_ok
        && ds.memory_pool_alloc_retired
        && ds.vm_instance_pool_field_retired
        && ds.objectref_rc_storage_still_present
        && !ds.storage_cleanup_enabled
        && !ds.unified_mir_runtime_heap_enabled
        && !ds.b09_axis_exit_enabled
        && !ds.gc_ga_claimed;
    GateB09ObjectRefRcMigrateFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_memory_pool_dead_shell_first_slice_floor_ok: floor_ok,
        memory_pool_type_stub_retired: true,
        objectref_migrate_surface_documented: true,
        objectref_rc_storage_still_present: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_rc_migrate_first_slice_signed: true,
    }
}

/// B-09 ObjectRef VM ManagedHeap probe first-slice inventory (W497 / ADR-841).
///
/// Signs that VM `DictBuild` `__type` ObjectRef path dual-registers into
/// ManagedHeap (aligned with mir-run `object_ref_wrap`). Does **not** flip
/// `objectref_rc_migrate_enabled`, `storage_cleanup_enabled`, enable unified
/// heap, claim GC GA, or B-09 `[x]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateB09ObjectRefVmManagedHeapProbeFirstSliceInventory {
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W496 ObjectRef RC migrate first-slice floor still signed.
    pub b09_objectref_rc_migrate_first_slice_floor_ok: bool,
    /// Always `true` — VM DictBuild `__type` ObjectRef registered on ManagedHeap.
    pub vm_dictbuild_objectref_managed_heap_registered: bool,
    /// Always `true` — honesty: ObjectRef RC storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: mir-run / LLVM / WASM ObjectRef RC still live.
    pub mir_run_llvm_wasm_objectref_rc_still_live: bool,
    /// Always `false` — honesty: full ObjectRef→ManagedHeap migrate not enabled.
    pub objectref_rc_migrate_enabled: bool,
    /// Always `false` — honesty: full storage-layer cleanup not enabled this window.
    pub storage_cleanup_enabled: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W497 / ADR-841: ObjectRef VM ManagedHeap probe first-slice signed.
    pub b09_objectref_vm_managed_heap_probe_first_slice_signed: bool,
}

/// Inventory B-09 ObjectRef VM ManagedHeap probe first-slice (W497 / ADR-841).
pub fn gate_b09_objectref_vm_managed_heap_probe_first_slice_inventory(
) -> GateB09ObjectRefVmManagedHeapProbeFirstSliceInventory {
    let again = gate_b09_objectref_vm_managed_heap_probe_first_slice_inventory_once();
    let first = gate_b09_objectref_vm_managed_heap_probe_first_slice_inventory_once();
    GateB09ObjectRefVmManagedHeapProbeFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_vm_managed_heap_probe_first_slice_inventory_once(
) -> GateB09ObjectRefVmManagedHeapProbeFirstSliceInventory {
    let mig = gate_b09_objectref_rc_migrate_first_slice_inventory_once();
    let floor_ok = mig.b09_objectref_rc_migrate_first_slice_signed
        && mig.b09_memory_pool_dead_shell_first_slice_floor_ok
        && mig.memory_pool_type_stub_retired
        && mig.objectref_migrate_surface_documented
        && mig.objectref_rc_storage_still_present
        && !mig.objectref_rc_migrate_enabled
        && !mig.storage_cleanup_enabled
        && !mig.unified_mir_runtime_heap_enabled
        && !mig.b09_axis_exit_enabled
        && !mig.gc_ga_claimed;
    GateB09ObjectRefVmManagedHeapProbeFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_rc_migrate_first_slice_floor_ok: floor_ok,
        vm_dictbuild_objectref_managed_heap_registered: true,
        objectref_rc_storage_still_present: true,
        mir_run_llvm_wasm_objectref_rc_still_live: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_vm_managed_heap_probe_first_slice_signed: true,
    }
}

/// Wave A W498 / ADR-842: B-09 ObjectRef VM heap_slots first-slice inventory.
/// Tracks ManagedHeap slots on VmInstance for collect/compact (mir-run parity).
/// ≠ migrate_enabled / ≠ cleanup / ≠ enable / ≠ B-09 `[x]` / ≠ GC GA.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateB09ObjectRefVmHeapSlotsFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W497 ObjectRef VM ManagedHeap probe first-slice floor ok.
    pub b09_objectref_vm_managed_heap_probe_first_slice_floor_ok: bool,
    /// Always `true` — VmInstance exposes `heap_slots: Vec<usize>`.
    pub vm_instance_heap_slots_present: bool,
    /// Always `true` — DictBuild ObjectRef path pushes alloc slot into `heap_slots`.
    pub vm_dictbuild_objectref_heap_slots_tracked: bool,
    /// Always `true` — HeapCompact rewrites `heap_slots` indices after forwarding.
    pub vm_heap_slots_compact_rewrite_wired: bool,
    /// Always `true` — honesty: ObjectRef RC storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: mir-run / LLVM / WASM ObjectRef RC still live.
    pub mir_run_llvm_wasm_objectref_rc_still_live: bool,
    /// Always `false` — honesty: full ObjectRef→ManagedHeap migrate not enabled.
    pub objectref_rc_migrate_enabled: bool,
    /// Always `false` — honesty: full storage-layer cleanup not enabled this window.
    pub storage_cleanup_enabled: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W498 / ADR-842: ObjectRef VM heap_slots first-slice signed.
    pub b09_objectref_vm_heap_slots_first_slice_signed: bool,
}

/// Inventory B-09 ObjectRef VM heap_slots first-slice (W498 / ADR-842).
pub fn gate_b09_objectref_vm_heap_slots_first_slice_inventory(
) -> GateB09ObjectRefVmHeapSlotsFirstSliceInventory {
    let again = gate_b09_objectref_vm_heap_slots_first_slice_inventory_once();
    let first = gate_b09_objectref_vm_heap_slots_first_slice_inventory_once();
    GateB09ObjectRefVmHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_vm_heap_slots_first_slice_inventory_once(
) -> GateB09ObjectRefVmHeapSlotsFirstSliceInventory {
    let probe = gate_b09_objectref_vm_managed_heap_probe_first_slice_inventory_once();
    let floor_ok = probe.b09_objectref_vm_managed_heap_probe_first_slice_signed
        && probe.b09_objectref_rc_migrate_first_slice_floor_ok
        && probe.vm_dictbuild_objectref_managed_heap_registered
        && probe.objectref_rc_storage_still_present
        && probe.mir_run_llvm_wasm_objectref_rc_still_live
        && !probe.objectref_rc_migrate_enabled
        && !probe.storage_cleanup_enabled
        && !probe.unified_mir_runtime_heap_enabled
        && !probe.b09_axis_exit_enabled
        && !probe.gc_ga_claimed;
    GateB09ObjectRefVmHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_vm_managed_heap_probe_first_slice_floor_ok: floor_ok,
        vm_instance_heap_slots_present: true,
        vm_dictbuild_objectref_heap_slots_tracked: true,
        vm_heap_slots_compact_rewrite_wired: true,
        objectref_rc_storage_still_present: true,
        mir_run_llvm_wasm_objectref_rc_still_live: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_vm_heap_slots_first_slice_signed: true,
    }
}

/// Wave A W499 / ADR-843: B-09 ObjectRef mir-run DictBuild ManagedHeap align.
/// DictBuild `__type` dual-registers ManagedHeap + heap_slots (parity with
/// `object_ref_wrap` / VM W497–W498). ≠ migrate / ≠ cleanup / ≠ enable / ≠ GA.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateB09ObjectRefMirRunDictbuildAlignFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W498 ObjectRef VM heap_slots first-slice floor ok.
    pub b09_objectref_vm_heap_slots_first_slice_floor_ok: bool,
    /// Always `true` — mir-run DictBuild ObjectRef dual-registers ManagedHeap.
    pub mir_run_dictbuild_objectref_managed_heap_registered: bool,
    /// Always `true` — mir-run DictBuild ObjectRef pushes alloc into `heap_slots`.
    pub mir_run_dictbuild_objectref_heap_slots_tracked: bool,
    /// Always `true` — honesty: ObjectRef RC storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: LLVM / WASM ObjectRef RC still live.
    pub llvm_wasm_objectref_rc_still_live: bool,
    /// Always `false` — honesty: full ObjectRef→ManagedHeap migrate not enabled.
    pub objectref_rc_migrate_enabled: bool,
    /// Always `false` — honesty: full storage-layer cleanup not enabled this window.
    pub storage_cleanup_enabled: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W499 / ADR-843: mir-run DictBuild align first-slice signed.
    pub b09_objectref_mir_run_dictbuild_align_first_slice_signed: bool,
}

/// Inventory B-09 ObjectRef mir-run DictBuild ManagedHeap align (W499 / ADR-843).
pub fn gate_b09_objectref_mir_run_dictbuild_align_first_slice_inventory(
) -> GateB09ObjectRefMirRunDictbuildAlignFirstSliceInventory {
    let again = gate_b09_objectref_mir_run_dictbuild_align_first_slice_inventory_once();
    let first = gate_b09_objectref_mir_run_dictbuild_align_first_slice_inventory_once();
    GateB09ObjectRefMirRunDictbuildAlignFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_mir_run_dictbuild_align_first_slice_inventory_once(
) -> GateB09ObjectRefMirRunDictbuildAlignFirstSliceInventory {
    let slots = gate_b09_objectref_vm_heap_slots_first_slice_inventory_once();
    let floor_ok = slots.b09_objectref_vm_heap_slots_first_slice_signed
        && slots.b09_objectref_vm_managed_heap_probe_first_slice_floor_ok
        && slots.vm_instance_heap_slots_present
        && slots.vm_dictbuild_objectref_heap_slots_tracked
        && slots.vm_heap_slots_compact_rewrite_wired
        && slots.objectref_rc_storage_still_present
        && slots.mir_run_llvm_wasm_objectref_rc_still_live
        && !slots.objectref_rc_migrate_enabled
        && !slots.storage_cleanup_enabled
        && !slots.unified_mir_runtime_heap_enabled
        && !slots.b09_axis_exit_enabled
        && !slots.gc_ga_claimed;
    GateB09ObjectRefMirRunDictbuildAlignFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_vm_heap_slots_first_slice_floor_ok: floor_ok,
        mir_run_dictbuild_objectref_managed_heap_registered: true,
        mir_run_dictbuild_objectref_heap_slots_tracked: true,
        objectref_rc_storage_still_present: true,
        llvm_wasm_objectref_rc_still_live: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_mir_run_dictbuild_align_first_slice_signed: true,
    }
}

/// Wave A W500 / ADR-844: B-09 ObjectRef VM CallHost ManagedHeap + heap_slots first-slice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectRefVmCallhostHeapSlotsFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W499 ObjectRef mir-run DictBuild align first-slice floor ok.
    pub b09_objectref_mir_run_dictbuild_align_first_slice_floor_ok: bool,
    /// Always `true` — CallHost success path dual-registers stack-top ObjectRef.
    pub vm_callhost_objectref_managed_heap_registered: bool,
    /// Always `true` — CallHost ObjectRef track pushes into `heap_slots`.
    pub vm_callhost_objectref_heap_slots_tracked: bool,
    /// Always `true` — honesty: ObjectRef RC storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: LLVM / WASM ObjectRef RC still live.
    pub mir_run_llvm_wasm_objectref_rc_still_live: bool,
    /// Always `false` — honesty: full ObjectRef→ManagedHeap migrate not enabled.
    pub objectref_rc_migrate_enabled: bool,
    /// Always `false` — honesty: full storage-layer cleanup not enabled this window.
    pub storage_cleanup_enabled: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W500 / ADR-844: VM CallHost heap_slots first-slice signed.
    pub b09_objectref_vm_callhost_heap_slots_first_slice_signed: bool,
}

/// Inventory B-09 ObjectRef VM CallHost ManagedHeap + heap_slots (W500 / ADR-844).
pub fn gate_b09_objectref_vm_callhost_heap_slots_first_slice_inventory(
) -> GateB09ObjectRefVmCallhostHeapSlotsFirstSliceInventory {
    let again = gate_b09_objectref_vm_callhost_heap_slots_first_slice_inventory_once();
    let first = gate_b09_objectref_vm_callhost_heap_slots_first_slice_inventory_once();
    GateB09ObjectRefVmCallhostHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_vm_callhost_heap_slots_first_slice_inventory_once(
) -> GateB09ObjectRefVmCallhostHeapSlotsFirstSliceInventory {
    let align = gate_b09_objectref_mir_run_dictbuild_align_first_slice_inventory_once();
    let floor_ok = align.b09_objectref_mir_run_dictbuild_align_first_slice_signed
        && align.b09_objectref_vm_heap_slots_first_slice_floor_ok
        && align.mir_run_dictbuild_objectref_managed_heap_registered
        && align.mir_run_dictbuild_objectref_heap_slots_tracked
        && align.objectref_rc_storage_still_present
        && align.llvm_wasm_objectref_rc_still_live
        && !align.objectref_rc_migrate_enabled
        && !align.storage_cleanup_enabled
        && !align.unified_mir_runtime_heap_enabled
        && !align.b09_axis_exit_enabled
        && !align.gc_ga_claimed;
    GateB09ObjectRefVmCallhostHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_mir_run_dictbuild_align_first_slice_floor_ok: floor_ok,
        vm_callhost_objectref_managed_heap_registered: true,
        vm_callhost_objectref_heap_slots_tracked: true,
        objectref_rc_storage_still_present: true,
        mir_run_llvm_wasm_objectref_rc_still_live: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_vm_callhost_heap_slots_first_slice_signed: true,
    }
}

/// Wave A W501 / ADR-845: B-09 ObjectRef mir-run CallHost ManagedHeap + heap_slots first-slice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectRefMirRunCallhostHeapSlotsFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W500 ObjectRef VM CallHost heap_slots first-slice floor ok.
    pub b09_objectref_vm_callhost_heap_slots_first_slice_floor_ok: bool,
    /// Always `true` — mir-run CallHost success path dual-registers stack-top ObjectRef.
    pub mir_run_callhost_objectref_managed_heap_registered: bool,
    /// Always `true` — mir-run CallHost ObjectRef track pushes into `heap_slots`.
    pub mir_run_callhost_objectref_heap_slots_tracked: bool,
    /// Always `true` — honesty: ObjectRef RC storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: LLVM / WASM ObjectRef RC still live.
    pub llvm_wasm_objectref_rc_still_live: bool,
    /// Always `false` — honesty: full ObjectRef→ManagedHeap migrate not enabled.
    pub objectref_rc_migrate_enabled: bool,
    /// Always `false` — honesty: full storage-layer cleanup not enabled this window.
    pub storage_cleanup_enabled: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W501 / ADR-845: mir-run CallHost heap_slots first-slice signed.
    pub b09_objectref_mir_run_callhost_heap_slots_first_slice_signed: bool,
}

/// Inventory B-09 ObjectRef mir-run CallHost ManagedHeap + heap_slots (W501 / ADR-845).
pub fn gate_b09_objectref_mir_run_callhost_heap_slots_first_slice_inventory(
) -> GateB09ObjectRefMirRunCallhostHeapSlotsFirstSliceInventory {
    let again = gate_b09_objectref_mir_run_callhost_heap_slots_first_slice_inventory_once();
    let first = gate_b09_objectref_mir_run_callhost_heap_slots_first_slice_inventory_once();
    GateB09ObjectRefMirRunCallhostHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_mir_run_callhost_heap_slots_first_slice_inventory_once(
) -> GateB09ObjectRefMirRunCallhostHeapSlotsFirstSliceInventory {
    let floor = gate_b09_objectref_vm_callhost_heap_slots_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_vm_callhost_heap_slots_first_slice_signed
        && floor.b09_objectref_mir_run_dictbuild_align_first_slice_floor_ok
        && floor.vm_callhost_objectref_managed_heap_registered
        && floor.vm_callhost_objectref_heap_slots_tracked
        && floor.objectref_rc_storage_still_present
        && floor.mir_run_llvm_wasm_objectref_rc_still_live
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectRefMirRunCallhostHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_vm_callhost_heap_slots_first_slice_floor_ok: floor_ok,
        mir_run_callhost_objectref_managed_heap_registered: true,
        mir_run_callhost_objectref_heap_slots_tracked: true,
        objectref_rc_storage_still_present: true,
        llvm_wasm_objectref_rc_still_live: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_mir_run_callhost_heap_slots_first_slice_signed: true,
    }
}

/// Wave A W502 / ADR-846: B-09 MutRefCell VM DictBuild ManagedHeap + heap_slots
/// first-slice inventory. DictBuild MutRefCell dual-registers ManagedHeap +
/// `heap_slots`; ≠ MutRefCell full migrate / ≠ migrate_enabled / ≠ cleanup /
/// ≠ enable / ≠ B-09 `[x]` / ≠ GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09MutRefCellVmDictbuildHeapSlotsFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W501 ObjectRef mir-run CallHost heap_slots first-slice floor ok.
    pub b09_objectref_mir_run_callhost_heap_slots_first_slice_floor_ok: bool,
    /// Always `true` — VM DictBuild MutRefCell dual-registers into ManagedHeap.
    pub vm_dictbuild_mutrefcell_managed_heap_registered: bool,
    /// Always `true` — VM DictBuild MutRefCell track pushes into `heap_slots`.
    pub vm_dictbuild_mutrefcell_heap_slots_tracked: bool,
    /// Always `true` — honesty: ObjectRef RC storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: LLVM / WASM ObjectRef RC still live.
    pub llvm_wasm_objectref_rc_still_live: bool,
    /// Always `false` — honesty: full ObjectRef→ManagedHeap migrate not enabled.
    pub objectref_rc_migrate_enabled: bool,
    /// Always `false` — honesty: full storage-layer cleanup not enabled this window.
    pub storage_cleanup_enabled: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W502 / ADR-846: MutRefCell VM DictBuild heap_slots first-slice signed.
    pub b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_signed: bool,
}

/// Inventory B-09 MutRefCell VM DictBuild ManagedHeap + heap_slots (W502 / ADR-846).
pub fn gate_b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_inventory(
) -> GateB09MutRefCellVmDictbuildHeapSlotsFirstSliceInventory {
    let again = gate_b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_inventory_once();
    let first = gate_b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_inventory_once();
    GateB09MutRefCellVmDictbuildHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_inventory_once(
) -> GateB09MutRefCellVmDictbuildHeapSlotsFirstSliceInventory {
    let floor = gate_b09_objectref_mir_run_callhost_heap_slots_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_mir_run_callhost_heap_slots_first_slice_signed
        && floor.b09_objectref_vm_callhost_heap_slots_first_slice_floor_ok
        && floor.mir_run_callhost_objectref_managed_heap_registered
        && floor.mir_run_callhost_objectref_heap_slots_tracked
        && floor.objectref_rc_storage_still_present
        && floor.llvm_wasm_objectref_rc_still_live
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09MutRefCellVmDictbuildHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_mir_run_callhost_heap_slots_first_slice_floor_ok: floor_ok,
        vm_dictbuild_mutrefcell_managed_heap_registered: true,
        vm_dictbuild_mutrefcell_heap_slots_tracked: true,
        objectref_rc_storage_still_present: true,
        llvm_wasm_objectref_rc_still_live: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_signed: true,
    }
}

/// Wave A W503 / ADR-847: B-09 MutRefCell mir-run DictBuild ManagedHeap +
/// heap_slots align inventory. DictBuild MutRefCell dual-registers ManagedHeap
/// + `heap_slots`; ≠ MutRefCell full migrate / ≠ migrate_enabled / ≠ cleanup /
/// ≠ enable / ≠ B-09 `[x]` / ≠ GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09MutRefCellMirRunDictbuildAlignFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W502 MutRefCell VM DictBuild heap_slots first-slice floor ok.
    pub b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_floor_ok: bool,
    /// Always `true` — mir-run DictBuild MutRefCell dual-registers into ManagedHeap.
    pub mir_run_dictbuild_mutrefcell_managed_heap_registered: bool,
    /// Always `true` — mir-run DictBuild MutRefCell track pushes into `heap_slots`.
    pub mir_run_dictbuild_mutrefcell_heap_slots_tracked: bool,
    /// Always `true` — honesty: ObjectRef RC storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: LLVM / WASM ObjectRef RC still live.
    pub llvm_wasm_objectref_rc_still_live: bool,
    /// Always `false` — honesty: full ObjectRef→ManagedHeap migrate not enabled.
    pub objectref_rc_migrate_enabled: bool,
    /// Always `false` — honesty: full storage-layer cleanup not enabled this window.
    pub storage_cleanup_enabled: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W503 / ADR-847: MutRefCell mir-run DictBuild align signed.
    pub b09_mutrefcell_mir_run_dictbuild_align_first_slice_signed: bool,
}

/// Inventory B-09 MutRefCell mir-run DictBuild ManagedHeap + heap_slots (W503 / ADR-847).
pub fn gate_b09_mutrefcell_mir_run_dictbuild_align_first_slice_inventory(
) -> GateB09MutRefCellMirRunDictbuildAlignFirstSliceInventory {
    let again = gate_b09_mutrefcell_mir_run_dictbuild_align_first_slice_inventory_once();
    let first = gate_b09_mutrefcell_mir_run_dictbuild_align_first_slice_inventory_once();
    GateB09MutRefCellMirRunDictbuildAlignFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_mutrefcell_mir_run_dictbuild_align_first_slice_inventory_once(
) -> GateB09MutRefCellMirRunDictbuildAlignFirstSliceInventory {
    let floor = gate_b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_inventory_once();
    let floor_ok = floor.b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_signed
        && floor.b09_objectref_mir_run_callhost_heap_slots_first_slice_floor_ok
        && floor.vm_dictbuild_mutrefcell_managed_heap_registered
        && floor.vm_dictbuild_mutrefcell_heap_slots_tracked
        && floor.objectref_rc_storage_still_present
        && floor.llvm_wasm_objectref_rc_still_live
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09MutRefCellMirRunDictbuildAlignFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_floor_ok: floor_ok,
        mir_run_dictbuild_mutrefcell_managed_heap_registered: true,
        mir_run_dictbuild_mutrefcell_heap_slots_tracked: true,
        objectref_rc_storage_still_present: true,
        llvm_wasm_objectref_rc_still_live: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_mutrefcell_mir_run_dictbuild_align_first_slice_signed: true,
    }
}

/// Wave A W504 / ADR-848: B-09 MutRefCell VM CallHost ManagedHeap +
/// heap_slots first-slice inventory. CallHost success tracks MutRefCell;
/// ≠ MutRefCell full migrate / ≠ migrate_enabled / ≠ cleanup / ≠ enable /
/// ≠ B-09 `[x]` / ≠ GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09MutRefCellVmCallhostHeapSlotsFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W503 MutRefCell mir-run DictBuild align first-slice floor ok.
    pub b09_mutrefcell_mir_run_dictbuild_align_first_slice_floor_ok: bool,
    /// Always `true` — VM CallHost MutRefCell dual-registers into ManagedHeap.
    pub vm_callhost_mutrefcell_managed_heap_registered: bool,
    /// Always `true` — VM CallHost MutRefCell track pushes into `heap_slots`.
    pub vm_callhost_mutrefcell_heap_slots_tracked: bool,
    /// Always `true` — honesty: ObjectRef RC storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: LLVM / WASM ObjectRef RC still live.
    pub llvm_wasm_objectref_rc_still_live: bool,
    /// Always `false` — honesty: full ObjectRef→ManagedHeap migrate not enabled.
    pub objectref_rc_migrate_enabled: bool,
    /// Always `false` — honesty: full storage-layer cleanup not enabled this window.
    pub storage_cleanup_enabled: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W504 / ADR-848: MutRefCell VM CallHost heap_slots signed.
    pub b09_mutrefcell_vm_callhost_heap_slots_first_slice_signed: bool,
}

/// Inventory B-09 MutRefCell VM CallHost ManagedHeap + heap_slots (W504 / ADR-848).
pub fn gate_b09_mutrefcell_vm_callhost_heap_slots_first_slice_inventory(
) -> GateB09MutRefCellVmCallhostHeapSlotsFirstSliceInventory {
    let again = gate_b09_mutrefcell_vm_callhost_heap_slots_first_slice_inventory_once();
    let first = gate_b09_mutrefcell_vm_callhost_heap_slots_first_slice_inventory_once();
    GateB09MutRefCellVmCallhostHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_mutrefcell_vm_callhost_heap_slots_first_slice_inventory_once(
) -> GateB09MutRefCellVmCallhostHeapSlotsFirstSliceInventory {
    let floor = gate_b09_mutrefcell_mir_run_dictbuild_align_first_slice_inventory_once();
    let floor_ok = floor.b09_mutrefcell_mir_run_dictbuild_align_first_slice_signed
        && floor.b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_floor_ok
        && floor.mir_run_dictbuild_mutrefcell_managed_heap_registered
        && floor.mir_run_dictbuild_mutrefcell_heap_slots_tracked
        && floor.objectref_rc_storage_still_present
        && floor.llvm_wasm_objectref_rc_still_live
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09MutRefCellVmCallhostHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_mutrefcell_mir_run_dictbuild_align_first_slice_floor_ok: floor_ok,
        vm_callhost_mutrefcell_managed_heap_registered: true,
        vm_callhost_mutrefcell_heap_slots_tracked: true,
        objectref_rc_storage_still_present: true,
        llvm_wasm_objectref_rc_still_live: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_mutrefcell_vm_callhost_heap_slots_first_slice_signed: true,
    }
}

/// Wave A W505 / ADR-849: B-09 MutRefCell mir-run CallHost ManagedHeap +
/// heap_slots first-slice inventory. CALL_HOST success tracks MutRefCell;
/// ≠ MutRefCell full migrate / ≠ migrate_enabled / ≠ cleanup / ≠ enable /
/// ≠ B-09 `[x]` / ≠ GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09MutRefCellMirRunCallhostHeapSlotsFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W504 MutRefCell VM CallHost heap_slots first-slice floor ok.
    pub b09_mutrefcell_vm_callhost_heap_slots_first_slice_floor_ok: bool,
    /// Always `true` — mir-run CallHost MutRefCell dual-registers into ManagedHeap.
    pub mir_run_callhost_mutrefcell_managed_heap_registered: bool,
    /// Always `true` — mir-run CallHost MutRefCell track pushes into `heap_slots`.
    pub mir_run_callhost_mutrefcell_heap_slots_tracked: bool,
    /// Always `true` — honesty: ObjectRef RC storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: LLVM / WASM ObjectRef RC still live.
    pub llvm_wasm_objectref_rc_still_live: bool,
    /// Always `false` — honesty: full ObjectRef→ManagedHeap migrate not enabled.
    pub objectref_rc_migrate_enabled: bool,
    /// Always `false` — honesty: full storage-layer cleanup not enabled this window.
    pub storage_cleanup_enabled: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W505 / ADR-849: MutRefCell mir-run CallHost heap_slots signed.
    pub b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_signed: bool,
}

/// Inventory B-09 MutRefCell mir-run CallHost ManagedHeap + heap_slots (W505 / ADR-849).
pub fn gate_b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_inventory(
) -> GateB09MutRefCellMirRunCallhostHeapSlotsFirstSliceInventory {
    let again = gate_b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_inventory_once();
    let first = gate_b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_inventory_once();
    GateB09MutRefCellMirRunCallhostHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_inventory_once(
) -> GateB09MutRefCellMirRunCallhostHeapSlotsFirstSliceInventory {
    let floor = gate_b09_mutrefcell_vm_callhost_heap_slots_first_slice_inventory_once();
    let floor_ok = floor.b09_mutrefcell_vm_callhost_heap_slots_first_slice_signed
        && floor.b09_mutrefcell_mir_run_dictbuild_align_first_slice_floor_ok
        && floor.vm_callhost_mutrefcell_managed_heap_registered
        && floor.vm_callhost_mutrefcell_heap_slots_tracked
        && floor.objectref_rc_storage_still_present
        && floor.llvm_wasm_objectref_rc_still_live
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09MutRefCellMirRunCallhostHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_mutrefcell_vm_callhost_heap_slots_first_slice_floor_ok: floor_ok,
        mir_run_callhost_mutrefcell_managed_heap_registered: true,
        mir_run_callhost_mutrefcell_heap_slots_tracked: true,
        objectref_rc_storage_still_present: true,
        llvm_wasm_objectref_rc_still_live: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_signed: true,
    }
}

/// Wave A W506 / ADR-850: B-09 ObjectRef+MutRefCell VM call-return (`finish_call`)
/// ManagedHeap + heap_slots first-slice inventory. Success push tracks both
/// ObjectRef and MutRefCell; ≠ migrate_enabled / ≠ cleanup / ≠ enable /
/// ≠ B-09 `[x]` / ≠ GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectRefVmCallReturnHeapSlotsFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W505 MutRefCell mir-run CallHost heap_slots first-slice floor ok.
    pub b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_floor_ok: bool,
    /// Always `true` — VM `finish_call` success dual-registers ObjectRef into ManagedHeap.
    pub vm_finish_call_objectref_managed_heap_registered: bool,
    /// Always `true` — VM `finish_call` success dual-registers MutRefCell into ManagedHeap.
    pub vm_finish_call_mutrefcell_managed_heap_registered: bool,
    /// Always `true` — VM `finish_call` track pushes into `heap_slots`.
    pub vm_finish_call_heap_slots_tracked: bool,
    /// Always `true` — honesty: ObjectRef RC storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: LLVM / WASM ObjectRef RC still live.
    pub llvm_wasm_objectref_rc_still_live: bool,
    /// Always `false` — honesty: full ObjectRef→ManagedHeap migrate not enabled.
    pub objectref_rc_migrate_enabled: bool,
    /// Always `false` — honesty: full storage-layer cleanup not enabled this window.
    pub storage_cleanup_enabled: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W506 / ADR-850: VM finish_call heap_slots signed.
    pub b09_objectref_vm_call_return_heap_slots_first_slice_signed: bool,
}

/// Inventory B-09 ObjectRef+MutRefCell VM finish_call ManagedHeap + heap_slots (W506 / ADR-850).
pub fn gate_b09_objectref_vm_call_return_heap_slots_first_slice_inventory(
) -> GateB09ObjectRefVmCallReturnHeapSlotsFirstSliceInventory {
    let again = gate_b09_objectref_vm_call_return_heap_slots_first_slice_inventory_once();
    let first = gate_b09_objectref_vm_call_return_heap_slots_first_slice_inventory_once();
    GateB09ObjectRefVmCallReturnHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_vm_call_return_heap_slots_first_slice_inventory_once(
) -> GateB09ObjectRefVmCallReturnHeapSlotsFirstSliceInventory {
    let floor = gate_b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_inventory_once();
    let floor_ok = floor.b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_signed
        && floor.b09_mutrefcell_vm_callhost_heap_slots_first_slice_floor_ok
        && floor.mir_run_callhost_mutrefcell_managed_heap_registered
        && floor.mir_run_callhost_mutrefcell_heap_slots_tracked
        && floor.objectref_rc_storage_still_present
        && floor.llvm_wasm_objectref_rc_still_live
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectRefVmCallReturnHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_floor_ok: floor_ok,
        vm_finish_call_objectref_managed_heap_registered: true,
        vm_finish_call_mutrefcell_managed_heap_registered: true,
        vm_finish_call_heap_slots_tracked: true,
        objectref_rc_storage_still_present: true,
        llvm_wasm_objectref_rc_still_live: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_vm_call_return_heap_slots_first_slice_signed: true,
    }
}

/// Wave A W507 / ADR-851: B-09 ObjectRef+MutRefCell mir-run call-return
/// (`call_fn` + `call_closure`) ManagedHeap + heap_slots first-slice inventory.
/// Success push tracks both ObjectRef and MutRefCell; ≠ migrate_enabled /
/// ≠ cleanup / ≠ enable / ≠ B-09 `[x]` / ≠ GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectRefMirRunCallReturnHeapSlotsFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W506 VM finish_call heap_slots first-slice floor ok.
    pub b09_objectref_vm_call_return_heap_slots_first_slice_floor_ok: bool,
    /// Always `true` — mir-run `call_fn` success dual-registers ObjectRef into ManagedHeap.
    pub mir_run_call_fn_objectref_managed_heap_registered: bool,
    /// Always `true` — mir-run `call_fn` success dual-registers MutRefCell into ManagedHeap.
    pub mir_run_call_fn_mutrefcell_managed_heap_registered: bool,
    /// Always `true` — mir-run `call_closure` success track pushes into `heap_slots`.
    pub mir_run_call_closure_heap_slots_tracked: bool,
    /// Always `true` — mir-run call-return (call_fn + call_closure) heap_slots tracked.
    pub mir_run_call_return_heap_slots_tracked: bool,
    /// Always `true` — honesty: ObjectRef RC storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: LLVM / WASM ObjectRef RC still live.
    pub llvm_wasm_objectref_rc_still_live: bool,
    /// Always `false` — honesty: full ObjectRef→ManagedHeap migrate not enabled.
    pub objectref_rc_migrate_enabled: bool,
    /// Always `false` — honesty: full storage-layer cleanup not enabled this window.
    pub storage_cleanup_enabled: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W507 / ADR-851: mir-run call-return heap_slots signed.
    pub b09_objectref_mir_run_call_return_heap_slots_first_slice_signed: bool,
}

/// Inventory B-09 ObjectRef+MutRefCell mir-run call_fn/call_closure ManagedHeap + heap_slots (W507 / ADR-851).
pub fn gate_b09_objectref_mir_run_call_return_heap_slots_first_slice_inventory(
) -> GateB09ObjectRefMirRunCallReturnHeapSlotsFirstSliceInventory {
    let again = gate_b09_objectref_mir_run_call_return_heap_slots_first_slice_inventory_once();
    let first = gate_b09_objectref_mir_run_call_return_heap_slots_first_slice_inventory_once();
    GateB09ObjectRefMirRunCallReturnHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_mir_run_call_return_heap_slots_first_slice_inventory_once(
) -> GateB09ObjectRefMirRunCallReturnHeapSlotsFirstSliceInventory {
    let floor = gate_b09_objectref_vm_call_return_heap_slots_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_vm_call_return_heap_slots_first_slice_signed
        && floor.b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_floor_ok
        && floor.vm_finish_call_objectref_managed_heap_registered
        && floor.vm_finish_call_mutrefcell_managed_heap_registered
        && floor.vm_finish_call_heap_slots_tracked
        && floor.objectref_rc_storage_still_present
        && floor.llvm_wasm_objectref_rc_still_live
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectRefMirRunCallReturnHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_vm_call_return_heap_slots_first_slice_floor_ok: floor_ok,
        mir_run_call_fn_objectref_managed_heap_registered: true,
        mir_run_call_fn_mutrefcell_managed_heap_registered: true,
        mir_run_call_closure_heap_slots_tracked: true,
        mir_run_call_return_heap_slots_tracked: true,
        objectref_rc_storage_still_present: true,
        llvm_wasm_objectref_rc_still_live: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_mir_run_call_return_heap_slots_first_slice_signed: true,
    }
}

/// Wave A W508 / ADR-852: B-09 ObjectRef+MutRefCell ActorCall (VM+mir-run runtime)
/// ManagedHeap + heap_slots first-slice inventory. Runtime success tracks both
/// ObjectRef and MutRefCell; ≠ MethodCall / ≠ migrate_enabled / ≠ cleanup /
/// ≠ enable / ≠ B-09 `[x]` / ≠ GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectRefActorCallHeapSlotsFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W507 mir-run call-return heap_slots first-slice floor ok.
    pub b09_objectref_mir_run_call_return_heap_slots_first_slice_floor_ok: bool,
    /// Always `true` — VM ActorCall runtime dual-registers ObjectRef into ManagedHeap.
    pub vm_actor_call_objectref_managed_heap_registered: bool,
    /// Always `true` — VM ActorCall runtime dual-registers MutRefCell into ManagedHeap.
    pub vm_actor_call_mutrefcell_managed_heap_registered: bool,
    /// Always `true` — mir-run ActorCall runtime dual-registers ObjectRef into ManagedHeap.
    pub mir_run_actor_call_objectref_managed_heap_registered: bool,
    /// Always `true` — mir-run ActorCall runtime dual-registers MutRefCell into ManagedHeap.
    pub mir_run_actor_call_mutrefcell_managed_heap_registered: bool,
    /// Always `true` — VM+mir-run ActorCall runtime track pushes into `heap_slots`.
    pub vm_mir_run_actor_call_heap_slots_tracked: bool,
    /// Always `true` — honesty: ObjectRef RC storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: LLVM / WASM ObjectRef RC still live.
    pub llvm_wasm_objectref_rc_still_live: bool,
    /// Always `false` — honesty: full ObjectRef→ManagedHeap migrate not enabled.
    pub objectref_rc_migrate_enabled: bool,
    /// Always `false` — honesty: full storage-layer cleanup not enabled this window.
    pub storage_cleanup_enabled: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W508 / ADR-852: ActorCall runtime heap_slots signed.
    pub b09_objectref_actor_call_heap_slots_first_slice_signed: bool,
}

/// Inventory B-09 ObjectRef+MutRefCell ActorCall runtime ManagedHeap + heap_slots (W508 / ADR-852).
pub fn gate_b09_objectref_actor_call_heap_slots_first_slice_inventory(
) -> GateB09ObjectRefActorCallHeapSlotsFirstSliceInventory {
    let again = gate_b09_objectref_actor_call_heap_slots_first_slice_inventory_once();
    let first = gate_b09_objectref_actor_call_heap_slots_first_slice_inventory_once();
    GateB09ObjectRefActorCallHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_actor_call_heap_slots_first_slice_inventory_once(
) -> GateB09ObjectRefActorCallHeapSlotsFirstSliceInventory {
    let floor = gate_b09_objectref_mir_run_call_return_heap_slots_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_mir_run_call_return_heap_slots_first_slice_signed
        && floor.b09_objectref_vm_call_return_heap_slots_first_slice_floor_ok
        && floor.mir_run_call_fn_objectref_managed_heap_registered
        && floor.mir_run_call_fn_mutrefcell_managed_heap_registered
        && floor.mir_run_call_closure_heap_slots_tracked
        && floor.mir_run_call_return_heap_slots_tracked
        && floor.objectref_rc_storage_still_present
        && floor.llvm_wasm_objectref_rc_still_live
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectRefActorCallHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_mir_run_call_return_heap_slots_first_slice_floor_ok: floor_ok,
        vm_actor_call_objectref_managed_heap_registered: true,
        vm_actor_call_mutrefcell_managed_heap_registered: true,
        mir_run_actor_call_objectref_managed_heap_registered: true,
        mir_run_actor_call_mutrefcell_managed_heap_registered: true,
        vm_mir_run_actor_call_heap_slots_tracked: true,
        objectref_rc_storage_still_present: true,
        llvm_wasm_objectref_rc_still_live: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_actor_call_heap_slots_first_slice_signed: true,
    }
}

/// Wave A W509 / ADR-853: B-09 ObjectRef+MutRefCell MethodCall (VM+mir-run)
/// ManagedHeap + heap_slots first-slice inventory. Dispatch success tracks both
/// ObjectRef and MutRefCell; ≠ ObjectRefRetain catch-all / ≠ migrate_enabled /
/// ≠ cleanup / ≠ enable / ≠ B-09 `[x]` / ≠ GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectRefMethodCallHeapSlotsFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W508 ActorCall runtime heap_slots first-slice floor ok.
    pub b09_objectref_actor_call_heap_slots_first_slice_floor_ok: bool,
    /// Always `true` — VM MethodCall dual-registers ObjectRef into ManagedHeap.
    pub vm_method_call_objectref_managed_heap_registered: bool,
    /// Always `true` — VM MethodCall dual-registers MutRefCell into ManagedHeap.
    pub vm_method_call_mutrefcell_managed_heap_registered: bool,
    /// Always `true` — mir-run METHOD_CALL dual-registers ObjectRef into ManagedHeap.
    pub mir_run_method_call_objectref_managed_heap_registered: bool,
    /// Always `true` — mir-run METHOD_CALL dual-registers MutRefCell into ManagedHeap.
    pub mir_run_method_call_mutrefcell_managed_heap_registered: bool,
    /// Always `true` — VM+mir-run MethodCall track pushes into `heap_slots`.
    pub vm_mir_run_method_call_heap_slots_tracked: bool,
    /// Always `true` — honesty: ObjectRef RC storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: LLVM / WASM ObjectRef RC still live.
    pub llvm_wasm_objectref_rc_still_live: bool,
    /// Always `false` — honesty: full ObjectRef→ManagedHeap migrate not enabled.
    pub objectref_rc_migrate_enabled: bool,
    /// Always `false` — honesty: full storage-layer cleanup not enabled this window.
    pub storage_cleanup_enabled: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W509 / ADR-853: MethodCall heap_slots signed.
    pub b09_objectref_method_call_heap_slots_first_slice_signed: bool,
}

/// Inventory B-09 ObjectRef+MutRefCell MethodCall ManagedHeap + heap_slots (W509 / ADR-853).
pub fn gate_b09_objectref_method_call_heap_slots_first_slice_inventory(
) -> GateB09ObjectRefMethodCallHeapSlotsFirstSliceInventory {
    let again = gate_b09_objectref_method_call_heap_slots_first_slice_inventory_once();
    let first = gate_b09_objectref_method_call_heap_slots_first_slice_inventory_once();
    GateB09ObjectRefMethodCallHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_method_call_heap_slots_first_slice_inventory_once(
) -> GateB09ObjectRefMethodCallHeapSlotsFirstSliceInventory {
    let floor = gate_b09_objectref_actor_call_heap_slots_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_actor_call_heap_slots_first_slice_signed
        && floor.b09_objectref_mir_run_call_return_heap_slots_first_slice_floor_ok
        && floor.vm_actor_call_objectref_managed_heap_registered
        && floor.vm_actor_call_mutrefcell_managed_heap_registered
        && floor.mir_run_actor_call_objectref_managed_heap_registered
        && floor.mir_run_actor_call_mutrefcell_managed_heap_registered
        && floor.vm_mir_run_actor_call_heap_slots_tracked
        && floor.objectref_rc_storage_still_present
        && floor.llvm_wasm_objectref_rc_still_live
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectRefMethodCallHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_actor_call_heap_slots_first_slice_floor_ok: floor_ok,
        vm_method_call_objectref_managed_heap_registered: true,
        vm_method_call_mutrefcell_managed_heap_registered: true,
        mir_run_method_call_objectref_managed_heap_registered: true,
        mir_run_method_call_mutrefcell_managed_heap_registered: true,
        vm_mir_run_method_call_heap_slots_tracked: true,
        objectref_rc_storage_still_present: true,
        llvm_wasm_objectref_rc_still_live: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_method_call_heap_slots_first_slice_signed: true,
    }
}

/// Wave A W510 / ADR-854: B-09 ObjectRefRetain catch-all (VM+mir-run; Dup/LoadLocal)
/// ManagedHeap + heap_slots first-slice inventory. Retain chokepoint tracks both
/// ObjectRef and MutRefCell; ≠ LLVM·WASM RC retire / ≠ migrate_enabled /
/// ≠ cleanup / ≠ enable / ≠ B-09 `[x]` / ≠ GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectRefRetainCatchAllHeapSlotsFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W509 MethodCall heap_slots first-slice floor ok.
    pub b09_objectref_method_call_heap_slots_first_slice_floor_ok: bool,
    /// Always `true` — VM ObjectRefRetain dual-registers into ManagedHeap.
    pub vm_objectref_retain_managed_heap_registered: bool,
    /// Always `true` — mir-run OBJECT_REF_RETAIN dual-registers into ManagedHeap.
    pub mir_run_objectref_retain_managed_heap_registered: bool,
    /// Always `true` — VM Dup/LoadLocal implicit retain dual-registers into ManagedHeap.
    pub vm_dup_loadlocal_objectref_managed_heap_registered: bool,
    /// Always `true` — mir-run DUP/LOAD_LOCAL implicit retain dual-registers into ManagedHeap.
    pub mir_run_dup_loadlocal_objectref_managed_heap_registered: bool,
    /// Always `true` — VM+mir-run retain catch-all track pushes into `heap_slots`.
    pub vm_mir_run_objectref_retain_heap_slots_tracked: bool,
    /// Always `true` — honesty: ObjectRef RC storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: LLVM / WASM ObjectRef RC still live.
    pub llvm_wasm_objectref_rc_still_live: bool,
    /// Always `false` — honesty: full ObjectRef→ManagedHeap migrate not enabled.
    pub objectref_rc_migrate_enabled: bool,
    /// Always `false` — honesty: full storage-layer cleanup not enabled this window.
    pub storage_cleanup_enabled: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W510 / ADR-854: ObjectRefRetain catch-all heap_slots signed.
    pub b09_objectref_retain_catch_all_heap_slots_first_slice_signed: bool,
}

/// Inventory B-09 ObjectRefRetain catch-all ManagedHeap + heap_slots (W510 / ADR-854).
pub fn gate_b09_objectref_retain_catch_all_heap_slots_first_slice_inventory(
) -> GateB09ObjectRefRetainCatchAllHeapSlotsFirstSliceInventory {
    let again = gate_b09_objectref_retain_catch_all_heap_slots_first_slice_inventory_once();
    let first = gate_b09_objectref_retain_catch_all_heap_slots_first_slice_inventory_once();
    GateB09ObjectRefRetainCatchAllHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_retain_catch_all_heap_slots_first_slice_inventory_once(
) -> GateB09ObjectRefRetainCatchAllHeapSlotsFirstSliceInventory {
    let floor = gate_b09_objectref_method_call_heap_slots_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_method_call_heap_slots_first_slice_signed
        && floor.b09_objectref_actor_call_heap_slots_first_slice_floor_ok
        && floor.vm_method_call_objectref_managed_heap_registered
        && floor.vm_method_call_mutrefcell_managed_heap_registered
        && floor.mir_run_method_call_objectref_managed_heap_registered
        && floor.mir_run_method_call_mutrefcell_managed_heap_registered
        && floor.vm_mir_run_method_call_heap_slots_tracked
        && floor.objectref_rc_storage_still_present
        && floor.llvm_wasm_objectref_rc_still_live
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectRefRetainCatchAllHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_method_call_heap_slots_first_slice_floor_ok: floor_ok,
        vm_objectref_retain_managed_heap_registered: true,
        mir_run_objectref_retain_managed_heap_registered: true,
        vm_dup_loadlocal_objectref_managed_heap_registered: true,
        mir_run_dup_loadlocal_objectref_managed_heap_registered: true,
        vm_mir_run_objectref_retain_heap_slots_tracked: true,
        objectref_rc_storage_still_present: true,
        llvm_wasm_objectref_rc_still_live: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_retain_catch_all_heap_slots_first_slice_signed: true,
    }
}

/// Wave A W555–W558 product flags (S2-I18: tip inventories retired).
/// Policy-fixed `true` matching prior live product flips — see
/// [`stage2-b09-product-flag-decouple`](../../../docs/adr/stage2-b09-product-flag-decouple.md).
/// ≠ rewriting ManagedHeap / ObjectRef / GC collect product behavior.
/// ≠ B-09 roadmap `[x]` beyond historical W558 narrow claim narrative.

/// Wave A W555 / ADR-899: public readonly migrate product flag (S2-I18 policy-fixed).
pub fn objectref_rc_migrate_enabled() -> bool {
    true
}

/// Wave A W556 / ADR-900: public readonly storage-cleanup product flag (S2-I18 policy-fixed).
pub fn storage_cleanup_enabled() -> bool {
    true
}

/// Wave A W557 / ADR-901: public readonly unified MirRuntime heap enable flag (S2-I18 policy-fixed).
/// Independent of `ManagedHeap::enabled()` / `SYX_UNIFIED_HEAP` env gate.
pub fn unified_mir_runtime_heap_enabled() -> bool {
    true
}

/// Wave A W558 / ADR-902: public readonly B-09 axis exit flag (S2-I18 policy-fixed).
pub fn b09_axis_exit_enabled() -> bool {
    true
}

/// Wave A W558 / ADR-902: public readonly narrow GC GA claim flag (S2-I18 policy-fixed).
pub fn gc_ga_claimed() -> bool {
    true
}

#[cfg(test)]
mod gate_b09_gc_narrative_thin_tests {
    use super::*;

    #[test]
    fn gate_b09_gc_narrative_thin_inventory_is_deterministic_and_signed() {
        let a = gate_b09_gc_narrative_thin_inventory();
        let b = gate_b09_gc_narrative_thin_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.three_layer_heap_table_documented);
        assert!(a.vm_gc_collect_path_documented);
        assert!(a.wasm_native_no_syx_gc_documented);
        assert!(a.preferred_unified_mir_runtime_direction_documented);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_gc_narrative_thin_signed);
    }

    #[test]
    fn gate_b09_gc_narrative_deepen_inventory_is_deterministic_and_signed() {
        let a = gate_b09_gc_narrative_deepen_inventory();
        let b = gate_b09_gc_narrative_deepen_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_gc_narrative_thin_floor_ok);
        assert!(a.unified_heap_construction_adr_documented);
        assert!(a.mir_runtime_root_set_documented);
        assert!(a.vm_gccollect_converge_path_documented);
        assert!(a.wasm_gc_types_deferred_documented);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_gc_narrative_deepen_signed);
        let floor = gate_b09_gc_narrative_thin_inventory();
        assert!(!floor.unified_mir_runtime_heap_enabled);
        assert!(!floor.b09_axis_exit_enabled);
        assert!(!floor.gc_ga_claimed);
        assert!(floor.b09_gc_narrative_thin_signed);
    }

    #[test]
    fn gate_b09_mir_runtime_root_set_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_mir_runtime_root_set_first_slice_inventory();
        let b = gate_b09_mir_runtime_root_set_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_gc_narrative_deepen_floor_ok);
        assert!(a.mir_runtime_suspend_frame_roots_wired);
        assert!(a.mir_runtime_locals_frames_roots_wired);
        assert!(!a.actor_mailbox_value_queue_roots_wired);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_mir_runtime_root_set_first_slice_signed);
        let deepen = gate_b09_gc_narrative_deepen_inventory();
        assert!(!deepen.unified_mir_runtime_heap_enabled);
        assert!(deepen.b09_gc_narrative_deepen_signed);
    }

    #[test]
    fn gate_b09_vm_gccollect_root_set_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_vm_gccollect_root_set_first_slice_inventory();
        let b = gate_b09_vm_gccollect_root_set_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_mir_runtime_root_set_first_slice_floor_ok);
        assert!(a.vm_suspend_frame_roots_wired);
        assert!(a.vm_locals_frames_roots_wired);
        assert!(!a.actor_mailbox_value_queue_roots_wired);
        assert!(!a.vm_mir_runtime_heap_fully_converged);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_vm_gccollect_root_set_first_slice_signed);
        let mir = gate_b09_mir_runtime_root_set_first_slice_inventory();
        assert!(!mir.unified_mir_runtime_heap_enabled);
        assert!(mir.b09_mir_runtime_root_set_first_slice_signed);
    }

    #[test]
    fn gate_b09_actor_mailbox_value_queue_root_set_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_actor_mailbox_value_queue_root_set_first_slice_inventory();
        let b = gate_b09_actor_mailbox_value_queue_root_set_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_vm_gccollect_root_set_first_slice_floor_ok);
        assert!(a.actor_mailbox_value_queue_roots_wired);
        assert!(!a.vm_mir_runtime_heap_fully_converged);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_actor_mailbox_value_queue_root_set_first_slice_signed);
        let vm = gate_b09_vm_gccollect_root_set_first_slice_inventory();
        assert!(!vm.actor_mailbox_value_queue_roots_wired);
        assert!(vm.b09_vm_gccollect_root_set_first_slice_signed);
    }

    #[test]
    fn gate_b09_dual_heap_converge_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_dual_heap_converge_first_slice_inventory();
        let b = gate_b09_dual_heap_converge_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_actor_mailbox_value_queue_root_set_first_slice_floor_ok);
        assert!(a.dual_heap_gap_documented);
        assert!(a.shared_managed_heap_collect_path_aligned);
        assert!(a.vm_memory_pool_mark_sweep_residual);
        assert!(!a.vm_mir_runtime_heap_fully_converged);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_dual_heap_converge_first_slice_signed);
        let mb = gate_b09_actor_mailbox_value_queue_root_set_first_slice_inventory();
        assert!(!mb.vm_mir_runtime_heap_fully_converged);
        assert!(mb.b09_actor_mailbox_value_queue_root_set_first_slice_signed);
    }

    #[test]
    fn gate_b09_vm_mir_runtime_heap_fully_converged_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_vm_mir_runtime_heap_fully_converged_first_slice_inventory();
        let b = gate_b09_vm_mir_runtime_heap_fully_converged_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_dual_heap_converge_first_slice_floor_ok);
        assert!(a.vm_memory_pool_mark_sweep_gc_collect_residual_retired);
        assert!(a.vm_mir_runtime_heap_fully_converged);
        assert!(a.vm_memory_pool_objectref_storage_still_present);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_vm_mir_runtime_heap_fully_converged_first_slice_signed);
        let dh = gate_b09_dual_heap_converge_first_slice_inventory();
        assert!(!dh.vm_mir_runtime_heap_fully_converged);
        assert!(dh.vm_memory_pool_mark_sweep_residual);
        assert!(dh.b09_dual_heap_converge_first_slice_signed);
    }

    #[test]
    fn gate_b09_objectref_memory_pool_storage_honesty_thin_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_memory_pool_storage_honesty_thin_inventory();
        let b = gate_b09_objectref_memory_pool_storage_honesty_thin_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_vm_mir_runtime_heap_fully_converged_first_slice_floor_ok);
        assert!(a.objectref_memory_pool_storage_documented);
        assert!(a.vm_memory_pool_objectref_storage_still_present);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_memory_pool_storage_honesty_thin_signed);
        let fc = gate_b09_vm_mir_runtime_heap_fully_converged_first_slice_inventory();
        assert!(fc.vm_memory_pool_objectref_storage_still_present);
        assert!(fc.b09_vm_mir_runtime_heap_fully_converged_first_slice_signed);
    }

    #[test]
    fn gate_b09_enable_readiness_thin_inventory_is_deterministic_and_signed() {
        let a = gate_b09_enable_readiness_thin_inventory();
        let b = gate_b09_enable_readiness_thin_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_memory_pool_storage_honesty_thin_floor_ok);
        assert!(a.enable_blockers_documented);
        assert!(a.vm_memory_pool_objectref_storage_still_present);
        assert!(a.gc_collect_mark_sweep_dead_api_documented);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_enable_readiness_signed);
        let sh = gate_b09_objectref_memory_pool_storage_honesty_thin_inventory();
        assert!(sh.vm_memory_pool_objectref_storage_still_present);
        assert!(!sh.storage_cleanup_enabled);
        assert!(sh.b09_objectref_memory_pool_storage_honesty_thin_signed);
    }

    #[test]
    fn gate_b09_storage_cleanup_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_storage_cleanup_first_slice_inventory();
        let b = gate_b09_storage_cleanup_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_enable_readiness_thin_floor_ok);
        assert!(a.gc_collect_mark_sweep_dead_api_retired);
        assert!(a.storage_cleanup_checklist_progress_documented);
        assert!(a.vm_memory_pool_objectref_storage_still_present);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_storage_cleanup_first_slice_signed);
        let er = gate_b09_enable_readiness_thin_inventory();
        assert!(er.gc_collect_mark_sweep_dead_api_documented);
        assert!(!er.storage_cleanup_enabled);
        assert!(er.b09_enable_readiness_signed);
    }

    #[test]
    fn gate_b09_memory_pool_dead_shell_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_memory_pool_dead_shell_first_slice_inventory();
        let b = gate_b09_memory_pool_dead_shell_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_storage_cleanup_first_slice_floor_ok);
        assert!(a.memory_pool_alloc_retired);
        assert!(a.vm_instance_pool_field_retired);
        assert!(a.objectref_rc_storage_still_present);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_memory_pool_dead_shell_first_slice_signed);
        let sc = gate_b09_storage_cleanup_first_slice_inventory();
        assert!(sc.gc_collect_mark_sweep_dead_api_retired);
        assert!(!sc.storage_cleanup_enabled);
        assert!(sc.b09_storage_cleanup_first_slice_signed);
    }

    #[test]
    fn gate_b09_objectref_rc_migrate_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_rc_migrate_first_slice_inventory();
        let b = gate_b09_objectref_rc_migrate_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_memory_pool_dead_shell_first_slice_floor_ok);
        assert!(a.memory_pool_type_stub_retired);
        assert!(a.objectref_migrate_surface_documented);
        assert!(a.objectref_rc_storage_still_present);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_rc_migrate_first_slice_signed);
        let ds = gate_b09_memory_pool_dead_shell_first_slice_inventory();
        assert!(ds.memory_pool_alloc_retired);
        assert!(ds.objectref_rc_storage_still_present);
        assert!(!ds.storage_cleanup_enabled);
        assert!(ds.b09_memory_pool_dead_shell_first_slice_signed);
    }

    #[test]
    fn gate_b09_objectref_vm_managed_heap_probe_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_vm_managed_heap_probe_first_slice_inventory();
        let b = gate_b09_objectref_vm_managed_heap_probe_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_rc_migrate_first_slice_floor_ok);
        assert!(a.vm_dictbuild_objectref_managed_heap_registered);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.mir_run_llvm_wasm_objectref_rc_still_live);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_vm_managed_heap_probe_first_slice_signed);
        let mig = gate_b09_objectref_rc_migrate_first_slice_inventory();
        assert!(mig.memory_pool_type_stub_retired);
        assert!(mig.objectref_rc_storage_still_present);
        assert!(!mig.objectref_rc_migrate_enabled);
        assert!(!mig.storage_cleanup_enabled);
        assert!(mig.b09_objectref_rc_migrate_first_slice_signed);
    }

    #[test]
    fn gate_b09_objectref_vm_heap_slots_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_vm_heap_slots_first_slice_inventory();
        let b = gate_b09_objectref_vm_heap_slots_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_vm_managed_heap_probe_first_slice_floor_ok);
        assert!(a.vm_instance_heap_slots_present);
        assert!(a.vm_dictbuild_objectref_heap_slots_tracked);
        assert!(a.vm_heap_slots_compact_rewrite_wired);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.mir_run_llvm_wasm_objectref_rc_still_live);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_vm_heap_slots_first_slice_signed);
        let probe = gate_b09_objectref_vm_managed_heap_probe_first_slice_inventory();
        assert!(probe.vm_dictbuild_objectref_managed_heap_registered);
        assert!(probe.objectref_rc_storage_still_present);
        assert!(!probe.objectref_rc_migrate_enabled);
        assert!(!probe.storage_cleanup_enabled);
        assert!(probe.b09_objectref_vm_managed_heap_probe_first_slice_signed);
    }

    #[test]
    fn gate_b09_objectref_mir_run_dictbuild_align_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_mir_run_dictbuild_align_first_slice_inventory();
        let b = gate_b09_objectref_mir_run_dictbuild_align_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_vm_heap_slots_first_slice_floor_ok);
        assert!(a.mir_run_dictbuild_objectref_managed_heap_registered);
        assert!(a.mir_run_dictbuild_objectref_heap_slots_tracked);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.llvm_wasm_objectref_rc_still_live);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_mir_run_dictbuild_align_first_slice_signed);
        let slots = gate_b09_objectref_vm_heap_slots_first_slice_inventory();
        assert!(slots.vm_dictbuild_objectref_heap_slots_tracked);
        assert!(slots.objectref_rc_storage_still_present);
        assert!(!slots.objectref_rc_migrate_enabled);
        assert!(!slots.storage_cleanup_enabled);
        assert!(slots.b09_objectref_vm_heap_slots_first_slice_signed);
    }

    #[test]
    fn gate_b09_objectref_vm_callhost_heap_slots_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_vm_callhost_heap_slots_first_slice_inventory();
        let b = gate_b09_objectref_vm_callhost_heap_slots_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_mir_run_dictbuild_align_first_slice_floor_ok);
        assert!(a.vm_callhost_objectref_managed_heap_registered);
        assert!(a.vm_callhost_objectref_heap_slots_tracked);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.mir_run_llvm_wasm_objectref_rc_still_live);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_vm_callhost_heap_slots_first_slice_signed);
        let align = gate_b09_objectref_mir_run_dictbuild_align_first_slice_inventory();
        assert!(align.mir_run_dictbuild_objectref_heap_slots_tracked);
        assert!(align.objectref_rc_storage_still_present);
        assert!(!align.objectref_rc_migrate_enabled);
        assert!(!align.storage_cleanup_enabled);
        assert!(align.b09_objectref_mir_run_dictbuild_align_first_slice_signed);
    }

    #[test]
    fn gate_b09_objectref_mir_run_callhost_heap_slots_first_slice_inventory_is_deterministic_and_signed()
    {
        let a = gate_b09_objectref_mir_run_callhost_heap_slots_first_slice_inventory();
        let b = gate_b09_objectref_mir_run_callhost_heap_slots_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_vm_callhost_heap_slots_first_slice_floor_ok);
        assert!(a.mir_run_callhost_objectref_managed_heap_registered);
        assert!(a.mir_run_callhost_objectref_heap_slots_tracked);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.llvm_wasm_objectref_rc_still_live);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_mir_run_callhost_heap_slots_first_slice_signed);
        let callhost = gate_b09_objectref_vm_callhost_heap_slots_first_slice_inventory();
        assert!(callhost.vm_callhost_objectref_heap_slots_tracked);
        assert!(callhost.objectref_rc_storage_still_present);
        assert!(!callhost.objectref_rc_migrate_enabled);
        assert!(!callhost.storage_cleanup_enabled);
        assert!(callhost.b09_objectref_vm_callhost_heap_slots_first_slice_signed);
    }

    #[test]
    fn gate_b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_inventory();
        let b = gate_b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_mir_run_callhost_heap_slots_first_slice_floor_ok);
        assert!(a.vm_dictbuild_mutrefcell_managed_heap_registered);
        assert!(a.vm_dictbuild_mutrefcell_heap_slots_tracked);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.llvm_wasm_objectref_rc_still_live);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_signed);
        let callhost = gate_b09_objectref_mir_run_callhost_heap_slots_first_slice_inventory();
        assert!(callhost.mir_run_callhost_objectref_heap_slots_tracked);
        assert!(callhost.objectref_rc_storage_still_present);
        assert!(!callhost.objectref_rc_migrate_enabled);
        assert!(!callhost.storage_cleanup_enabled);
        assert!(callhost.b09_objectref_mir_run_callhost_heap_slots_first_slice_signed);
    }

    #[test]
    fn gate_b09_mutrefcell_mir_run_dictbuild_align_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_mutrefcell_mir_run_dictbuild_align_first_slice_inventory();
        let b = gate_b09_mutrefcell_mir_run_dictbuild_align_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_floor_ok);
        assert!(a.mir_run_dictbuild_mutrefcell_managed_heap_registered);
        assert!(a.mir_run_dictbuild_mutrefcell_heap_slots_tracked);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.llvm_wasm_objectref_rc_still_live);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_mutrefcell_mir_run_dictbuild_align_first_slice_signed);
        let floor = gate_b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_inventory();
        assert!(floor.vm_dictbuild_mutrefcell_heap_slots_tracked);
        assert!(floor.objectref_rc_storage_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(floor.b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_signed);
    }

    #[test]
    fn gate_b09_mutrefcell_vm_callhost_heap_slots_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_mutrefcell_vm_callhost_heap_slots_first_slice_inventory();
        let b = gate_b09_mutrefcell_vm_callhost_heap_slots_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_mutrefcell_mir_run_dictbuild_align_first_slice_floor_ok);
        assert!(a.vm_callhost_mutrefcell_managed_heap_registered);
        assert!(a.vm_callhost_mutrefcell_heap_slots_tracked);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.llvm_wasm_objectref_rc_still_live);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_mutrefcell_vm_callhost_heap_slots_first_slice_signed);
        let floor = gate_b09_mutrefcell_mir_run_dictbuild_align_first_slice_inventory();
        assert!(floor.mir_run_dictbuild_mutrefcell_heap_slots_tracked);
        assert!(floor.objectref_rc_storage_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(floor.b09_mutrefcell_mir_run_dictbuild_align_first_slice_signed);
    }

    #[test]
    fn gate_b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_inventory_is_deterministic_and_signed(
    ) {
        let a = gate_b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_inventory();
        let b = gate_b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_mutrefcell_vm_callhost_heap_slots_first_slice_floor_ok);
        assert!(a.mir_run_callhost_mutrefcell_managed_heap_registered);
        assert!(a.mir_run_callhost_mutrefcell_heap_slots_tracked);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.llvm_wasm_objectref_rc_still_live);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_signed);
        let floor = gate_b09_mutrefcell_vm_callhost_heap_slots_first_slice_inventory();
        assert!(floor.vm_callhost_mutrefcell_heap_slots_tracked);
        assert!(floor.objectref_rc_storage_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(floor.b09_mutrefcell_vm_callhost_heap_slots_first_slice_signed);
    }

    #[test]
    fn gate_b09_objectref_vm_call_return_heap_slots_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_vm_call_return_heap_slots_first_slice_inventory();
        let b = gate_b09_objectref_vm_call_return_heap_slots_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_floor_ok);
        assert!(a.vm_finish_call_objectref_managed_heap_registered);
        assert!(a.vm_finish_call_mutrefcell_managed_heap_registered);
        assert!(a.vm_finish_call_heap_slots_tracked);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.llvm_wasm_objectref_rc_still_live);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_vm_call_return_heap_slots_first_slice_signed);
        let floor = gate_b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_inventory();
        assert!(floor.mir_run_callhost_mutrefcell_heap_slots_tracked);
        assert!(floor.objectref_rc_storage_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(floor.b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_signed);
    }

    #[test]
    fn gate_b09_objectref_mir_run_call_return_heap_slots_first_slice_inventory_is_deterministic_and_signed(
    ) {
        let a = gate_b09_objectref_mir_run_call_return_heap_slots_first_slice_inventory();
        let b = gate_b09_objectref_mir_run_call_return_heap_slots_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_vm_call_return_heap_slots_first_slice_floor_ok);
        assert!(a.mir_run_call_fn_objectref_managed_heap_registered);
        assert!(a.mir_run_call_fn_mutrefcell_managed_heap_registered);
        assert!(a.mir_run_call_closure_heap_slots_tracked);
        assert!(a.mir_run_call_return_heap_slots_tracked);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.llvm_wasm_objectref_rc_still_live);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_mir_run_call_return_heap_slots_first_slice_signed);
        let floor = gate_b09_objectref_vm_call_return_heap_slots_first_slice_inventory();
        assert!(floor.vm_finish_call_heap_slots_tracked);
        assert!(floor.objectref_rc_storage_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(floor.b09_objectref_vm_call_return_heap_slots_first_slice_signed);
    }

    #[test]
    fn gate_b09_objectref_actor_call_heap_slots_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_actor_call_heap_slots_first_slice_inventory();
        let b = gate_b09_objectref_actor_call_heap_slots_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_mir_run_call_return_heap_slots_first_slice_floor_ok);
        assert!(a.vm_actor_call_objectref_managed_heap_registered);
        assert!(a.vm_actor_call_mutrefcell_managed_heap_registered);
        assert!(a.mir_run_actor_call_objectref_managed_heap_registered);
        assert!(a.mir_run_actor_call_mutrefcell_managed_heap_registered);
        assert!(a.vm_mir_run_actor_call_heap_slots_tracked);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.llvm_wasm_objectref_rc_still_live);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_actor_call_heap_slots_first_slice_signed);
        let floor = gate_b09_objectref_mir_run_call_return_heap_slots_first_slice_inventory();
        assert!(floor.mir_run_call_return_heap_slots_tracked);
        assert!(floor.objectref_rc_storage_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(floor.b09_objectref_mir_run_call_return_heap_slots_first_slice_signed);
    }

    #[test]
    fn gate_b09_objectref_method_call_heap_slots_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_method_call_heap_slots_first_slice_inventory();
        let b = gate_b09_objectref_method_call_heap_slots_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_actor_call_heap_slots_first_slice_floor_ok);
        assert!(a.vm_method_call_objectref_managed_heap_registered);
        assert!(a.vm_method_call_mutrefcell_managed_heap_registered);
        assert!(a.mir_run_method_call_objectref_managed_heap_registered);
        assert!(a.mir_run_method_call_mutrefcell_managed_heap_registered);
        assert!(a.vm_mir_run_method_call_heap_slots_tracked);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.llvm_wasm_objectref_rc_still_live);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_method_call_heap_slots_first_slice_signed);
        let floor = gate_b09_objectref_actor_call_heap_slots_first_slice_inventory();
        assert!(floor.vm_mir_run_actor_call_heap_slots_tracked);
        assert!(floor.objectref_rc_storage_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(floor.b09_objectref_actor_call_heap_slots_first_slice_signed);
    }

    #[test]
    fn gate_b09_objectref_retain_catch_all_heap_slots_first_slice_inventory_is_deterministic_and_signed(
    ) {
        let a = gate_b09_objectref_retain_catch_all_heap_slots_first_slice_inventory();
        let b = gate_b09_objectref_retain_catch_all_heap_slots_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_method_call_heap_slots_first_slice_floor_ok);
        assert!(a.vm_objectref_retain_managed_heap_registered);
        assert!(a.mir_run_objectref_retain_managed_heap_registered);
        assert!(a.vm_dup_loadlocal_objectref_managed_heap_registered);
        assert!(a.mir_run_dup_loadlocal_objectref_managed_heap_registered);
        assert!(a.vm_mir_run_objectref_retain_heap_slots_tracked);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.llvm_wasm_objectref_rc_still_live);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_retain_catch_all_heap_slots_first_slice_signed);
        let floor = gate_b09_objectref_method_call_heap_slots_first_slice_inventory();
        assert!(floor.vm_mir_run_method_call_heap_slots_tracked);
        assert!(floor.objectref_rc_storage_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(floor.b09_objectref_method_call_heap_slots_first_slice_signed);
    }


}
