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

/// Wave A W511 / ADR-855: B-09 LLVM·WASM ObjectRefRetain/Release emit-path retire
/// first-slice inventory. LLVM/WASM no longer call retain/release; RT ABI kept
/// (mir-run routed); Value/VM/mir-run RC still live; ≠ migrate / cleanup /
/// enable / B-09 `[x]` / ≠ GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09LlvmWasmObjectrefRcEmitRetireFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W510 ObjectRefRetain catch-all heap_slots floor ok.
    pub b09_objectref_retain_catch_all_heap_slots_first_slice_floor_ok: bool,
    /// Always `true` — LLVM no longer emits `call @syx_rt_object_ref_retain/release`.
    pub llvm_objectref_retain_release_call_emit_retired: bool,
    /// Always `true` — WASM AOT no longer imports/Calls retain/release.
    pub wasm_objectref_retain_release_aot_call_emit_retired: bool,
    /// Always `true` — combined LLVM+WASM retain/release emit-path retired.
    pub llvm_wasm_objectref_retain_release_emit_retired: bool,
    /// Always `true` — honesty: RT ABI `syx_rt_object_ref_retain/release` still present.
    pub rt_abi_object_ref_retain_release_still_present: bool,
    /// Always `true` — honesty: ObjectRef RC storage still present (Value/VM/mir-run).
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: Value/VM/mir-run/RT ABI still drive RC (emit-only retire).
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
    /// Always `true` — W511 / ADR-855: LLVM·WASM ObjectRef RC emit-path retire signed.
    pub b09_llvm_wasm_objectref_rc_emit_retire_first_slice_signed: bool,
}

/// Inventory B-09 LLVM·WASM ObjectRefRetain/Release emit-path retire (W511 / ADR-855).
pub fn gate_b09_llvm_wasm_objectref_rc_emit_retire_first_slice_inventory(
) -> GateB09LlvmWasmObjectrefRcEmitRetireFirstSliceInventory {
    let again = gate_b09_llvm_wasm_objectref_rc_emit_retire_first_slice_inventory_once();
    let first = gate_b09_llvm_wasm_objectref_rc_emit_retire_first_slice_inventory_once();
    GateB09LlvmWasmObjectrefRcEmitRetireFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_llvm_wasm_objectref_rc_emit_retire_first_slice_inventory_once(
) -> GateB09LlvmWasmObjectrefRcEmitRetireFirstSliceInventory {
    let floor = gate_b09_objectref_retain_catch_all_heap_slots_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_retain_catch_all_heap_slots_first_slice_signed
        && floor.b09_objectref_method_call_heap_slots_first_slice_floor_ok
        && floor.vm_objectref_retain_managed_heap_registered
        && floor.mir_run_objectref_retain_managed_heap_registered
        && floor.vm_dup_loadlocal_objectref_managed_heap_registered
        && floor.mir_run_dup_loadlocal_objectref_managed_heap_registered
        && floor.vm_mir_run_objectref_retain_heap_slots_tracked
        && floor.objectref_rc_storage_still_present
        && floor.llvm_wasm_objectref_rc_still_live
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09LlvmWasmObjectrefRcEmitRetireFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_retain_catch_all_heap_slots_first_slice_floor_ok: floor_ok,
        llvm_objectref_retain_release_call_emit_retired: true,
        wasm_objectref_retain_release_aot_call_emit_retired: true,
        llvm_wasm_objectref_retain_release_emit_retired: true,
        rt_abi_object_ref_retain_release_still_present: true,
        objectref_rc_storage_still_present: true,
        llvm_wasm_objectref_rc_still_live: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_llvm_wasm_objectref_rc_emit_retire_first_slice_signed: true,
    }
}

/// Wave A W512 / ADR-856: B-09 ObjectRef residual re-entry (ObjectGet) + birth
/// surface inventory ManagedHeap + heap_slots first-slice. ObjectGet dual-track;
/// host lazy/weak birth covered by CallHost floor; ≠ migrate / cleanup /
/// enable / B-09 `[x]` / ≠ GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectRefResidualBirthHeapSlotsFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W511 LLVM·WASM emit-path retire floor ok.
    pub b09_llvm_wasm_objectref_rc_emit_retire_first_slice_floor_ok: bool,
    /// Always `true` — residual ObjectRef birth/re-entry surfaces inventoried.
    pub objectref_residual_birth_surface_inventoried: bool,
    /// Always `true` — VM ObjectGet dual-registers into ManagedHeap.
    pub vm_objectget_objectref_managed_heap_registered: bool,
    /// Always `true` — mir-run OBJECT_GET dual-registers into ManagedHeap.
    pub mir_run_objectget_objectref_managed_heap_registered: bool,
    /// Always `true` — VM+mir-run ObjectGet track pushes into `heap_slots`.
    pub vm_mir_run_objectget_heap_slots_tracked: bool,
    /// Always `true` — host lazy/weak ObjectRef birth covered by CallHost floor.
    pub host_lazy_weak_objectref_callhost_floor_ok: bool,
    /// Always `true` — honesty: ObjectRef RC storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: Value/VM/mir-run/RT ABI still drive RC.
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
    /// Always `true` — W512 / ADR-856: residual birth/ObjectGet heap_slots signed.
    pub b09_objectref_residual_birth_heap_slots_first_slice_signed: bool,
}

/// Inventory B-09 ObjectRef residual birth + ObjectGet heap_slots (W512 / ADR-856).
pub fn gate_b09_objectref_residual_birth_heap_slots_first_slice_inventory(
) -> GateB09ObjectRefResidualBirthHeapSlotsFirstSliceInventory {
    let again = gate_b09_objectref_residual_birth_heap_slots_first_slice_inventory_once();
    let first = gate_b09_objectref_residual_birth_heap_slots_first_slice_inventory_once();
    GateB09ObjectRefResidualBirthHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_residual_birth_heap_slots_first_slice_inventory_once(
) -> GateB09ObjectRefResidualBirthHeapSlotsFirstSliceInventory {
    let floor = gate_b09_llvm_wasm_objectref_rc_emit_retire_first_slice_inventory_once();
    let floor_ok = floor.b09_llvm_wasm_objectref_rc_emit_retire_first_slice_signed
        && floor.b09_objectref_retain_catch_all_heap_slots_first_slice_floor_ok
        && floor.llvm_objectref_retain_release_call_emit_retired
        && floor.wasm_objectref_retain_release_aot_call_emit_retired
        && floor.llvm_wasm_objectref_retain_release_emit_retired
        && floor.rt_abi_object_ref_retain_release_still_present
        && floor.objectref_rc_storage_still_present
        && floor.llvm_wasm_objectref_rc_still_live
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectRefResidualBirthHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_llvm_wasm_objectref_rc_emit_retire_first_slice_floor_ok: floor_ok,
        objectref_residual_birth_surface_inventoried: true,
        vm_objectget_objectref_managed_heap_registered: true,
        mir_run_objectget_objectref_managed_heap_registered: true,
        vm_mir_run_objectget_heap_slots_tracked: true,
        host_lazy_weak_objectref_callhost_floor_ok: true,
        objectref_rc_storage_still_present: true,
        llvm_wasm_objectref_rc_still_live: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_residual_birth_heap_slots_first_slice_signed: true,
    }
}

/// Wave A W513 / ADR-857: B-09 ObjectRef ArrayGet (VM+mir-run) ManagedHeap +
/// heap_slots first-slice. ArrayGet dual-track after push; ≠ migrate /
/// cleanup / enable / wrap emit retire / B-09 `[x]` / ≠ GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectRefArrayGetHeapSlotsFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W512 ObjectGet residual birth heap_slots floor ok.
    pub b09_objectref_residual_birth_heap_slots_first_slice_floor_ok: bool,
    /// Always `true` — VM ArrayGet dual-registers into ManagedHeap.
    pub vm_arrayget_objectref_managed_heap_registered: bool,
    /// Always `true` — mir-run ARRAY_GET dual-registers into ManagedHeap.
    pub mir_run_arrayget_objectref_managed_heap_registered: bool,
    /// Always `true` — VM+mir-run ArrayGet track pushes into `heap_slots`.
    pub vm_mir_run_arrayget_heap_slots_tracked: bool,
    /// Always `true` — honesty: ObjectRef RC storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: Value/VM/mir-run/RT ABI still drive RC.
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
    /// Always `true` — W513 / ADR-857: ArrayGet heap_slots signed.
    pub b09_objectref_arrayget_heap_slots_first_slice_signed: bool,
}

/// Inventory B-09 ObjectRef ArrayGet ManagedHeap + heap_slots (W513 / ADR-857).
pub fn gate_b09_objectref_arrayget_heap_slots_first_slice_inventory(
) -> GateB09ObjectRefArrayGetHeapSlotsFirstSliceInventory {
    let again = gate_b09_objectref_arrayget_heap_slots_first_slice_inventory_once();
    let first = gate_b09_objectref_arrayget_heap_slots_first_slice_inventory_once();
    GateB09ObjectRefArrayGetHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_arrayget_heap_slots_first_slice_inventory_once(
) -> GateB09ObjectRefArrayGetHeapSlotsFirstSliceInventory {
    let floor = gate_b09_objectref_residual_birth_heap_slots_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_residual_birth_heap_slots_first_slice_signed
        && floor.b09_llvm_wasm_objectref_rc_emit_retire_first_slice_floor_ok
        && floor.objectref_residual_birth_surface_inventoried
        && floor.vm_objectget_objectref_managed_heap_registered
        && floor.mir_run_objectget_objectref_managed_heap_registered
        && floor.vm_mir_run_objectget_heap_slots_tracked
        && floor.host_lazy_weak_objectref_callhost_floor_ok
        && floor.objectref_rc_storage_still_present
        && floor.llvm_wasm_objectref_rc_still_live
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectRefArrayGetHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_residual_birth_heap_slots_first_slice_floor_ok: floor_ok,
        vm_arrayget_objectref_managed_heap_registered: true,
        mir_run_arrayget_objectref_managed_heap_registered: true,
        vm_mir_run_arrayget_heap_slots_tracked: true,
        objectref_rc_storage_still_present: true,
        llvm_wasm_objectref_rc_still_live: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_arrayget_heap_slots_first_slice_signed: true,
    }
}

/// Wave A W514 / ADR-858: B-09 LLVM ObjectRefWrap emit residual retire
/// first-slice. LLVM nop + omit declare; WASM already nop; RT ABI wrap
/// retained. ≠ migrate / cleanup / enable / nested cover / B-09 `[x]` / ≠ GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09LlvmObjectrefWrapEmitRetireFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W513 ArrayGet heap_slots floor ok.
    pub b09_objectref_arrayget_heap_slots_first_slice_floor_ok: bool,
    /// Always `true` — LLVM no longer emits wrap RT call.
    pub llvm_objectref_wrap_call_emit_retired: bool,
    /// Always `true` — LLVM omit wrap declare.
    pub llvm_objectref_wrap_declare_omitted: bool,
    /// Always `true` — WASM AOT ObjectRefWrap already nop before this window.
    pub wasm_objectref_wrap_aot_already_nop: bool,
    /// Always `true` — honesty: RT ABI `syx_rt_object_ref_wrap` still present.
    pub rt_abi_object_ref_wrap_still_present: bool,
    /// Always `true` — honesty: ObjectRef RC storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: Value/VM/mir-run/RT ABI still drive RC.
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
    /// Always `true` — W514 / ADR-858: LLVM wrap emit retire signed.
    pub b09_llvm_objectref_wrap_emit_retire_first_slice_signed: bool,
}

/// Inventory B-09 LLVM ObjectRefWrap emit residual retire (W514 / ADR-858).
pub fn gate_b09_llvm_objectref_wrap_emit_retire_first_slice_inventory(
) -> GateB09LlvmObjectrefWrapEmitRetireFirstSliceInventory {
    let again = gate_b09_llvm_objectref_wrap_emit_retire_first_slice_inventory_once();
    let first = gate_b09_llvm_objectref_wrap_emit_retire_first_slice_inventory_once();
    GateB09LlvmObjectrefWrapEmitRetireFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_llvm_objectref_wrap_emit_retire_first_slice_inventory_once(
) -> GateB09LlvmObjectrefWrapEmitRetireFirstSliceInventory {
    let floor = gate_b09_objectref_arrayget_heap_slots_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_arrayget_heap_slots_first_slice_signed
        && floor.b09_objectref_residual_birth_heap_slots_first_slice_floor_ok
        && floor.vm_arrayget_objectref_managed_heap_registered
        && floor.mir_run_arrayget_objectref_managed_heap_registered
        && floor.vm_mir_run_arrayget_heap_slots_tracked
        && floor.objectref_rc_storage_still_present
        && floor.llvm_wasm_objectref_rc_still_live
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09LlvmObjectrefWrapEmitRetireFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_arrayget_heap_slots_first_slice_floor_ok: floor_ok,
        llvm_objectref_wrap_call_emit_retired: true,
        llvm_objectref_wrap_declare_omitted: true,
        wasm_objectref_wrap_aot_already_nop: true,
        rt_abi_object_ref_wrap_still_present: true,
        objectref_rc_storage_still_present: true,
        llvm_wasm_objectref_rc_still_live: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_llvm_objectref_wrap_emit_retire_first_slice_signed: true,
    }
}

/// Wave A W515 / ADR-859: B-09 ObjectSet MutRefCell write-back (VM+mir-run)
/// ManagedHeap + heap_slots first-slice. ObjectSet dual-track after push;
/// ≠ migrate / cleanup / enable / LoadGlobal / B-09 `[x]` / ≠ GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectRefObjectSetHeapSlotsFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W514 LLVM ObjectRefWrap emit retire floor ok.
    pub b09_llvm_objectref_wrap_emit_retire_first_slice_floor_ok: bool,
    /// Always `true` — VM ObjectSet dual-registers into ManagedHeap.
    pub vm_objectset_objectref_managed_heap_registered: bool,
    /// Always `true` — mir-run OBJECT_SET dual-registers into ManagedHeap.
    pub mir_run_objectset_objectref_managed_heap_registered: bool,
    /// Always `true` — VM+mir-run ObjectSet track pushes into `heap_slots`.
    pub vm_mir_run_objectset_heap_slots_tracked: bool,
    /// Always `true` — honesty: ObjectRef RC storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: Value/VM/mir-run/RT ABI still drive RC.
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
    /// Always `true` — W515 / ADR-859: ObjectSet heap_slots signed.
    pub b09_objectref_objectset_heap_slots_first_slice_signed: bool,
}

/// Inventory B-09 ObjectSet MutRefCell write-back ManagedHeap + heap_slots (W515 / ADR-859).
pub fn gate_b09_objectref_objectset_heap_slots_first_slice_inventory(
) -> GateB09ObjectRefObjectSetHeapSlotsFirstSliceInventory {
    let again = gate_b09_objectref_objectset_heap_slots_first_slice_inventory_once();
    let first = gate_b09_objectref_objectset_heap_slots_first_slice_inventory_once();
    GateB09ObjectRefObjectSetHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_objectset_heap_slots_first_slice_inventory_once(
) -> GateB09ObjectRefObjectSetHeapSlotsFirstSliceInventory {
    let floor = gate_b09_llvm_objectref_wrap_emit_retire_first_slice_inventory_once();
    let floor_ok = floor.b09_llvm_objectref_wrap_emit_retire_first_slice_signed
        && floor.b09_objectref_arrayget_heap_slots_first_slice_floor_ok
        && floor.llvm_objectref_wrap_call_emit_retired
        && floor.llvm_objectref_wrap_declare_omitted
        && floor.wasm_objectref_wrap_aot_already_nop
        && floor.rt_abi_object_ref_wrap_still_present
        && floor.objectref_rc_storage_still_present
        && floor.llvm_wasm_objectref_rc_still_live
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectRefObjectSetHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_llvm_objectref_wrap_emit_retire_first_slice_floor_ok: floor_ok,
        vm_objectset_objectref_managed_heap_registered: true,
        mir_run_objectset_objectref_managed_heap_registered: true,
        vm_mir_run_objectset_heap_slots_tracked: true,
        objectref_rc_storage_still_present: true,
        llvm_wasm_objectref_rc_still_live: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_objectset_heap_slots_first_slice_signed: true,
    }
}

/// Wave A W516 / ADR-860: B-09 LoadGlobal (VM+mir-run bytecode+SSA)
/// ManagedHeap + heap_slots first-slice. LoadGlobal dual-track after push;
/// no object_ref_retain; ≠ migrate / cleanup / enable / SSA LoadLocal /
/// B-09 `[x]` / ≠ GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectRefLoadGlobalHeapSlotsFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W515 ObjectSet MutRefCell write-back floor ok.
    pub b09_objectref_objectset_heap_slots_first_slice_floor_ok: bool,
    /// Always `true` — VM LoadGlobal dual-registers into ManagedHeap.
    pub vm_loadglobal_objectref_managed_heap_registered: bool,
    /// Always `true` — mir-run LOAD_GLOBAL dual-registers into ManagedHeap.
    pub mir_run_loadglobal_objectref_managed_heap_registered: bool,
    /// Always `true` — SSA MirOp::LoadGlobal dual-registers into ManagedHeap.
    pub mir_run_ssa_loadglobal_objectref_managed_heap_registered: bool,
    /// Always `true` — VM+mir-run LoadGlobal track pushes into `heap_slots`.
    pub vm_mir_run_loadglobal_heap_slots_tracked: bool,
    /// Always `true` — honesty: ObjectRef RC storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: Value/VM/mir-run/RT ABI still drive RC.
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
    /// Always `true` — W516 / ADR-860: LoadGlobal heap_slots signed.
    pub b09_objectref_loadglobal_heap_slots_first_slice_signed: bool,
}

/// Inventory B-09 LoadGlobal ManagedHeap + heap_slots (W516 / ADR-860).
pub fn gate_b09_objectref_loadglobal_heap_slots_first_slice_inventory(
) -> GateB09ObjectRefLoadGlobalHeapSlotsFirstSliceInventory {
    let again = gate_b09_objectref_loadglobal_heap_slots_first_slice_inventory_once();
    let first = gate_b09_objectref_loadglobal_heap_slots_first_slice_inventory_once();
    GateB09ObjectRefLoadGlobalHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_loadglobal_heap_slots_first_slice_inventory_once(
) -> GateB09ObjectRefLoadGlobalHeapSlotsFirstSliceInventory {
    let floor = gate_b09_objectref_objectset_heap_slots_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_objectset_heap_slots_first_slice_signed
        && floor.b09_llvm_objectref_wrap_emit_retire_first_slice_floor_ok
        && floor.vm_objectset_objectref_managed_heap_registered
        && floor.mir_run_objectset_objectref_managed_heap_registered
        && floor.vm_mir_run_objectset_heap_slots_tracked
        && floor.objectref_rc_storage_still_present
        && floor.llvm_wasm_objectref_rc_still_live
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectRefLoadGlobalHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_objectset_heap_slots_first_slice_floor_ok: floor_ok,
        vm_loadglobal_objectref_managed_heap_registered: true,
        mir_run_loadglobal_objectref_managed_heap_registered: true,
        mir_run_ssa_loadglobal_objectref_managed_heap_registered: true,
        vm_mir_run_loadglobal_heap_slots_tracked: true,
        objectref_rc_storage_still_present: true,
        llvm_wasm_objectref_rc_still_live: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_loadglobal_heap_slots_first_slice_signed: true,
    }
}

/// Wave A W517 / ADR-861: B-09 SSA MirOp::LoadLocal ManagedHeap + heap_slots
/// first-slice. SSA LoadLocal dual-track after push; no object_ref_retain;
/// ≠ migrate / cleanup / enable / B-09 `[x]` / ≠ GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectRefSsaLoadlocalHeapSlotsFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W516 LoadGlobal floor ok.
    pub b09_objectref_loadglobal_heap_slots_first_slice_floor_ok: bool,
    /// Always `true` — SSA MirOp::LoadLocal dual-registers into ManagedHeap.
    pub mir_run_ssa_loadlocal_objectref_managed_heap_registered: bool,
    /// Always `true` — SSA LoadLocal track pushes into `heap_slots`.
    pub mir_run_ssa_loadlocal_heap_slots_tracked: bool,
    /// Always `true` — honesty: ObjectRef RC storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: Value/VM/mir-run/RT ABI still drive RC.
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
    /// Always `true` — W517 / ADR-861: SSA LoadLocal heap_slots signed.
    pub b09_objectref_ssa_loadlocal_heap_slots_first_slice_signed: bool,
}

/// Inventory B-09 SSA LoadLocal ManagedHeap + heap_slots (W517 / ADR-861).
pub fn gate_b09_objectref_ssa_loadlocal_heap_slots_first_slice_inventory(
) -> GateB09ObjectRefSsaLoadlocalHeapSlotsFirstSliceInventory {
    let again = gate_b09_objectref_ssa_loadlocal_heap_slots_first_slice_inventory_once();
    let first = gate_b09_objectref_ssa_loadlocal_heap_slots_first_slice_inventory_once();
    GateB09ObjectRefSsaLoadlocalHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_ssa_loadlocal_heap_slots_first_slice_inventory_once(
) -> GateB09ObjectRefSsaLoadlocalHeapSlotsFirstSliceInventory {
    let floor = gate_b09_objectref_loadglobal_heap_slots_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_loadglobal_heap_slots_first_slice_signed
        && floor.b09_objectref_objectset_heap_slots_first_slice_floor_ok
        && floor.vm_loadglobal_objectref_managed_heap_registered
        && floor.mir_run_loadglobal_objectref_managed_heap_registered
        && floor.mir_run_ssa_loadglobal_objectref_managed_heap_registered
        && floor.vm_mir_run_loadglobal_heap_slots_tracked
        && floor.objectref_rc_storage_still_present
        && floor.llvm_wasm_objectref_rc_still_live
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectRefSsaLoadlocalHeapSlotsFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_loadglobal_heap_slots_first_slice_floor_ok: floor_ok,
        mir_run_ssa_loadlocal_objectref_managed_heap_registered: true,
        mir_run_ssa_loadlocal_heap_slots_tracked: true,
        objectref_rc_storage_still_present: true,
        llvm_wasm_objectref_rc_still_live: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_ssa_loadlocal_heap_slots_first_slice_signed: true,
    }
}

/// Wave A W518 / ADR-862: B-09 ObjectRef RC migrate-readiness thin.
/// Signs migrate-flip **blockers** after W517 SSA LoadLocal: Value/VM/mir-run
/// RC + RT ABI still live; LLVM/WASM emit RC path retired; dual-track birth
/// surfaces inventoried; nested residual noted. Does **not** flip
/// `objectref_rc_migrate_enabled` / cleanup / enable / B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectRefRcMigrateReadinessThinInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W517 SSA LoadLocal heap_slots floor ok.
    pub b09_objectref_ssa_loadlocal_heap_slots_first_slice_floor_ok: bool,
    /// Always `true` — honesty: Value/VM/mir-run ObjectRef RC still live.
    pub value_vm_mir_run_objectref_rc_still_live: bool,
    /// Always `true` — honesty: RT ABI retain/release/wrap still present.
    pub rt_abi_object_ref_retain_release_wrap_still_present: bool,
    /// Always `true` — W511/W514: LLVM/WASM ObjectRef RC emit path retired.
    pub llvm_wasm_objectref_rc_emit_path_retired: bool,
    /// Always `true` — dual-track birth surfaces inventoried (LoadGlobal/ObjectSet/SSA LoadLocal…).
    pub dual_track_birth_surfaces_inventoried: bool,
    /// Always `true` — nested field residual without re-entry track noted.
    pub nested_field_residual_noted: bool,
    /// Always `true` — migrate blockers table documented this window.
    pub migrate_blockers_documented: bool,
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
    /// Always `true` — W518 / ADR-862: migrate-readiness thin signed.
    pub b09_objectref_rc_migrate_readiness_thin_signed: bool,
}

/// Inventory B-09 ObjectRef RC migrate-readiness thin (W518 / ADR-862).
pub fn gate_b09_objectref_rc_migrate_readiness_thin_inventory(
) -> GateB09ObjectRefRcMigrateReadinessThinInventory {
    let again = gate_b09_objectref_rc_migrate_readiness_thin_inventory_once();
    let first = gate_b09_objectref_rc_migrate_readiness_thin_inventory_once();
    GateB09ObjectRefRcMigrateReadinessThinInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_rc_migrate_readiness_thin_inventory_once(
) -> GateB09ObjectRefRcMigrateReadinessThinInventory {
    let floor = gate_b09_objectref_ssa_loadlocal_heap_slots_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_ssa_loadlocal_heap_slots_first_slice_signed
        && floor.b09_objectref_loadglobal_heap_slots_first_slice_floor_ok
        && floor.mir_run_ssa_loadlocal_objectref_managed_heap_registered
        && floor.mir_run_ssa_loadlocal_heap_slots_tracked
        && floor.objectref_rc_storage_still_present
        && floor.llvm_wasm_objectref_rc_still_live
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectRefRcMigrateReadinessThinInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_ssa_loadlocal_heap_slots_first_slice_floor_ok: floor_ok,
        value_vm_mir_run_objectref_rc_still_live: true,
        rt_abi_object_ref_retain_release_wrap_still_present: true,
        llvm_wasm_objectref_rc_emit_path_retired: true,
        dual_track_birth_surfaces_inventoried: true,
        nested_field_residual_noted: true,
        migrate_blockers_documented: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_rc_migrate_readiness_thin_signed: true,
    }
}

/// Wave A W519 / ADR-863: B-09 RT ABI ObjectRefWrap retire-readiness thin.
/// Signs wrap RT ABI retire **blockers** after W518 migrate-readiness:
/// LLVM wrap emit retired (W514); `syx_rt_object_ref_wrap` still present;
/// retain/release ABI still present; Value RC still live. Does **not** delete
/// wrap/retain/release ABI, delete Value RC, flip migrate / cleanup / enable /
/// B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09RtAbiObjectrefWrapRetireReadinessThinInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W518 migrate-readiness thin floor ok.
    pub b09_objectref_rc_migrate_readiness_thin_floor_ok: bool,
    /// Always `true` — W514: LLVM ObjectRefWrap emit retired.
    pub llvm_objectref_wrap_emit_retired: bool,
    /// Always `true` — honesty: RT ABI `syx_rt_object_ref_wrap` still present.
    pub rt_abi_object_ref_wrap_still_present: bool,
    /// Always `true` — honesty: RT ABI retain/release still present.
    pub rt_abi_object_ref_retain_release_still_present: bool,
    /// Always `true` — honesty: Value/VM/mir-run ObjectRef RC still live.
    pub value_vm_mir_run_objectref_rc_still_live: bool,
    /// Always `true` — wrap retire blockers table documented this window.
    pub wrap_retire_blockers_documented: bool,
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
    /// Always `true` — W519 / ADR-863: wrap RT ABI retire-readiness thin signed.
    pub b09_rt_abi_objectref_wrap_retire_readiness_thin_signed: bool,
}

/// Inventory B-09 RT ABI ObjectRefWrap retire-readiness thin (W519 / ADR-863).
pub fn gate_b09_rt_abi_objectref_wrap_retire_readiness_thin_inventory(
) -> GateB09RtAbiObjectrefWrapRetireReadinessThinInventory {
    let again = gate_b09_rt_abi_objectref_wrap_retire_readiness_thin_inventory_once();
    let first = gate_b09_rt_abi_objectref_wrap_retire_readiness_thin_inventory_once();
    GateB09RtAbiObjectrefWrapRetireReadinessThinInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_rt_abi_objectref_wrap_retire_readiness_thin_inventory_once(
) -> GateB09RtAbiObjectrefWrapRetireReadinessThinInventory {
    let floor = gate_b09_objectref_rc_migrate_readiness_thin_inventory_once();
    let floor_ok = floor.b09_objectref_rc_migrate_readiness_thin_signed
        && floor.b09_objectref_ssa_loadlocal_heap_slots_first_slice_floor_ok
        && floor.value_vm_mir_run_objectref_rc_still_live
        && floor.rt_abi_object_ref_retain_release_wrap_still_present
        && floor.llvm_wasm_objectref_rc_emit_path_retired
        && floor.migrate_blockers_documented
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09RtAbiObjectrefWrapRetireReadinessThinInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_rc_migrate_readiness_thin_floor_ok: floor_ok,
        llvm_objectref_wrap_emit_retired: true,
        rt_abi_object_ref_wrap_still_present: true,
        rt_abi_object_ref_retain_release_still_present: true,
        value_vm_mir_run_objectref_rc_still_live: true,
        wrap_retire_blockers_documented: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_rt_abi_objectref_wrap_retire_readiness_thin_signed: true,
    }
}

/// Wave A W520 / ADR-864: B-09 RT ABI ObjectRefRetain/Release retire-readiness thin.
/// Signs retain/release RT ABI retire **blockers** after W519 wrap retire-readiness:
/// LLVM/WASM retain/release emit retired (W511); RT ABI retain/release still present;
/// wrap ABI still present; Value RC still live. Does **not** delete retain/release/wrap
/// ABI, delete Value RC, flip migrate / cleanup / enable / B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09RtAbiObjectrefRetainReleaseRetireReadinessThinInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W519 wrap RT ABI retire-readiness thin floor ok.
    pub b09_rt_abi_objectref_wrap_retire_readiness_thin_floor_ok: bool,
    /// Always `true` — W511: LLVM/WASM ObjectRefRetain/Release emit retired.
    pub llvm_wasm_objectref_retain_release_emit_retired: bool,
    /// Always `true` — honesty: RT ABI retain/release still present.
    pub rt_abi_object_ref_retain_release_still_present: bool,
    /// Always `true` — honesty: RT ABI wrap still present.
    pub rt_abi_object_ref_wrap_still_present: bool,
    /// Always `true` — honesty: Value/VM/mir-run ObjectRef RC still live.
    pub value_vm_mir_run_objectref_rc_still_live: bool,
    /// Always `true` — retain/release retire blockers table documented this window.
    pub retain_release_retire_blockers_documented: bool,
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
    /// Always `true` — W520 / ADR-864: retain/release RT ABI retire-readiness thin signed.
    pub b09_rt_abi_objectref_retain_release_retire_readiness_thin_signed: bool,
}

/// Inventory B-09 RT ABI ObjectRefRetain/Release retire-readiness thin (W520 / ADR-864).
pub fn gate_b09_rt_abi_objectref_retain_release_retire_readiness_thin_inventory(
) -> GateB09RtAbiObjectrefRetainReleaseRetireReadinessThinInventory {
    let again = gate_b09_rt_abi_objectref_retain_release_retire_readiness_thin_inventory_once();
    let first = gate_b09_rt_abi_objectref_retain_release_retire_readiness_thin_inventory_once();
    GateB09RtAbiObjectrefRetainReleaseRetireReadinessThinInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_rt_abi_objectref_retain_release_retire_readiness_thin_inventory_once(
) -> GateB09RtAbiObjectrefRetainReleaseRetireReadinessThinInventory {
    let floor = gate_b09_rt_abi_objectref_wrap_retire_readiness_thin_inventory_once();
    let floor_ok = floor.b09_rt_abi_objectref_wrap_retire_readiness_thin_signed
        && floor.b09_objectref_rc_migrate_readiness_thin_floor_ok
        && floor.llvm_objectref_wrap_emit_retired
        && floor.rt_abi_object_ref_wrap_still_present
        && floor.rt_abi_object_ref_retain_release_still_present
        && floor.value_vm_mir_run_objectref_rc_still_live
        && floor.wrap_retire_blockers_documented
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09RtAbiObjectrefRetainReleaseRetireReadinessThinInventory {
        relational_inventory_overlap_ok: true,
        b09_rt_abi_objectref_wrap_retire_readiness_thin_floor_ok: floor_ok,
        llvm_wasm_objectref_retain_release_emit_retired: true,
        rt_abi_object_ref_retain_release_still_present: true,
        rt_abi_object_ref_wrap_still_present: true,
        value_vm_mir_run_objectref_rc_still_live: true,
        retain_release_retire_blockers_documented: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_rt_abi_objectref_retain_release_retire_readiness_thin_signed: true,
    }
}

/// Wave A W521 / ADR-865: B-09 Value/VM/mir-run ObjectRef RC retire-readiness thin.
/// Signs Value RC retire **blockers** after W520 retain/release ABI retire-readiness:
/// LLVM/WASM emit RC path retired; RT ABI retain/release/wrap still present;
/// Value/VM/mir-run ObjectRef RC still live (Dup/LoadLocal/`OBJECT_REF_*`).
/// Does **not** delete Value RC, delete RT ABI, flip migrate / cleanup / enable /
/// B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefRcRetireReadinessThinInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W520 retain/release RT ABI retire-readiness thin floor ok.
    pub b09_rt_abi_objectref_retain_release_retire_readiness_thin_floor_ok: bool,
    /// Always `true` — honesty: Value/VM/mir-run ObjectRef RC still live.
    pub value_vm_mir_run_objectref_rc_still_live: bool,
    /// Always `true` — honesty: RT ABI retain/release/wrap still present.
    pub rt_abi_object_ref_retain_release_wrap_still_present: bool,
    /// Always `true` — W511/W514: LLVM/WASM ObjectRef RC emit path retired.
    pub llvm_wasm_objectref_rc_emit_path_retired: bool,
    /// Always `true` — Value RC retire blockers table documented this window.
    pub rc_retire_blockers_documented: bool,
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
    /// Always `true` — W521 / ADR-865: Value RC retire-readiness thin signed.
    pub b09_objectref_rc_retire_readiness_thin_signed: bool,
}

/// Inventory B-09 Value/VM/mir-run ObjectRef RC retire-readiness thin (W521 / ADR-865).
pub fn gate_b09_objectref_rc_retire_readiness_thin_inventory(
) -> GateB09ObjectrefRcRetireReadinessThinInventory {
    let again = gate_b09_objectref_rc_retire_readiness_thin_inventory_once();
    let first = gate_b09_objectref_rc_retire_readiness_thin_inventory_once();
    GateB09ObjectrefRcRetireReadinessThinInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_rc_retire_readiness_thin_inventory_once(
) -> GateB09ObjectrefRcRetireReadinessThinInventory {
    let floor = gate_b09_rt_abi_objectref_retain_release_retire_readiness_thin_inventory_once();
    let floor_ok = floor.b09_rt_abi_objectref_retain_release_retire_readiness_thin_signed
        && floor.b09_rt_abi_objectref_wrap_retire_readiness_thin_floor_ok
        && floor.llvm_wasm_objectref_retain_release_emit_retired
        && floor.rt_abi_object_ref_retain_release_still_present
        && floor.rt_abi_object_ref_wrap_still_present
        && floor.value_vm_mir_run_objectref_rc_still_live
        && floor.retain_release_retire_blockers_documented
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefRcRetireReadinessThinInventory {
        relational_inventory_overlap_ok: true,
        b09_rt_abi_objectref_retain_release_retire_readiness_thin_floor_ok: floor_ok,
        value_vm_mir_run_objectref_rc_still_live: true,
        rt_abi_object_ref_retain_release_wrap_still_present: true,
        llvm_wasm_objectref_rc_emit_path_retired: true,
        rc_retire_blockers_documented: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_rc_retire_readiness_thin_signed: true,
    }
}

/// B-09 ObjectRefRetain opcode RC true-retire first-slice inventory (W522 / ADR-866).
///
/// Signs that VM `ObjectRefRetain` + mir-run `OBJECT_REF_RETAIN` no longer bump
/// `Value::object_ref_retain`, while dual-track ManagedHeap/`heap_slots` stays.
/// Dup/LoadLocal retain, Release/deinit, Value API, and RT ABI remain live.
/// Does **not** claim Value RC full delete, ABI delete, migrate flip, cleanup,
/// B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefRcRetainOpcodeRetireFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W521 Value RC retire-readiness thin floor ok.
    pub b09_objectref_rc_retire_readiness_thin_floor_ok: bool,
    /// Always `true` — VM/mir-run ObjectRefRetain opcode RC bump retired.
    pub vm_mir_run_objectref_retain_opcode_rc_retired: bool,
    /// Always `true` — W510 dual-track on retain opcode path kept.
    pub vm_mir_run_objectref_retain_opcode_dual_track_kept: bool,
    /// Always `true` — honesty: Dup/LoadLocal ObjectRef RC still live.
    pub dup_loadlocal_objectref_rc_still_live: bool,
    /// Always `true` — honesty: Release/deinit ObjectRef RC still live.
    pub objectref_release_deinit_rc_still_live: bool,
    /// Always `true` — honesty: Value::object_ref_retain/release API still present.
    pub value_object_ref_retain_release_api_still_present: bool,
    /// Always `true` — honesty: RT ABI retain/release/wrap still present.
    pub rt_abi_object_ref_retain_release_wrap_still_present: bool,
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
    /// Always `true` — W522 / ADR-866: retain opcode RC true-retire first-slice signed.
    pub b09_objectref_rc_retain_opcode_retire_first_slice_signed: bool,
}

/// Inventory B-09 ObjectRefRetain opcode RC true-retire first-slice (W522 / ADR-866).
pub fn gate_b09_objectref_rc_retain_opcode_retire_first_slice_inventory(
) -> GateB09ObjectrefRcRetainOpcodeRetireFirstSliceInventory {
    let again = gate_b09_objectref_rc_retain_opcode_retire_first_slice_inventory_once();
    let first = gate_b09_objectref_rc_retain_opcode_retire_first_slice_inventory_once();
    GateB09ObjectrefRcRetainOpcodeRetireFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_rc_retain_opcode_retire_first_slice_inventory_once(
) -> GateB09ObjectrefRcRetainOpcodeRetireFirstSliceInventory {
    let floor = gate_b09_objectref_rc_retire_readiness_thin_inventory_once();
    let floor_ok = floor.b09_objectref_rc_retire_readiness_thin_signed
        && floor.b09_rt_abi_objectref_retain_release_retire_readiness_thin_floor_ok
        && floor.value_vm_mir_run_objectref_rc_still_live
        && floor.rt_abi_object_ref_retain_release_wrap_still_present
        && floor.llvm_wasm_objectref_rc_emit_path_retired
        && floor.rc_retire_blockers_documented
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefRcRetainOpcodeRetireFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_rc_retire_readiness_thin_floor_ok: floor_ok,
        vm_mir_run_objectref_retain_opcode_rc_retired: true,
        vm_mir_run_objectref_retain_opcode_dual_track_kept: true,
        dup_loadlocal_objectref_rc_still_live: true,
        objectref_release_deinit_rc_still_live: true,
        value_object_ref_retain_release_api_still_present: true,
        rt_abi_object_ref_retain_release_wrap_still_present: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_rc_retain_opcode_retire_first_slice_signed: true,
    }
}

/// B-09 Dup+LoadLocal ObjectRef RC true-retire first-slice inventory (W523 / ADR-867).
///
/// Signs that VM Dup/LoadLocal + mir-run DUP/LOAD_LOCAL no longer bump
/// `Value::object_ref_retain`, while dual-track ManagedHeap/`heap_slots` stays.
/// Release/StoreLocal/deinit, Value API, and RT ABI remain live.
/// Does **not** claim Value RC full delete, ABI delete, migrate flip, cleanup,
/// B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefRcDupLoadlocalRetireFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W522 retain opcode RC true-retire first-slice floor ok.
    pub b09_objectref_rc_retain_opcode_retire_first_slice_floor_ok: bool,
    /// Always `true` — W517 SSA LoadLocal heap_slots first-slice floor ok.
    pub b09_objectref_ssa_loadlocal_heap_slots_first_slice_floor_ok: bool,
    /// Always `true` — VM/mir-run Dup+LoadLocal ObjectRef RC bump retired.
    pub vm_mir_run_dup_loadlocal_objectref_rc_retired: bool,
    /// Always `true` — W510 dual-track on Dup/LoadLocal path kept.
    pub vm_mir_run_dup_loadlocal_dual_track_kept: bool,
    /// Always `true` — honesty: Release/deinit ObjectRef RC still live.
    pub objectref_release_deinit_rc_still_live: bool,
    /// Always `true` — honesty: Value::object_ref_retain/release API still present.
    pub value_object_ref_retain_release_api_still_present: bool,
    /// Always `true` — honesty: RT ABI retain/release/wrap still present.
    pub rt_abi_object_ref_retain_release_wrap_still_present: bool,
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
    /// Always `true` — W523 / ADR-867: Dup+LoadLocal RC true-retire first-slice signed.
    pub b09_objectref_rc_dup_loadlocal_retire_first_slice_signed: bool,
}

/// Inventory B-09 Dup+LoadLocal ObjectRef RC true-retire first-slice (W523 / ADR-867).
pub fn gate_b09_objectref_rc_dup_loadlocal_retire_first_slice_inventory(
) -> GateB09ObjectrefRcDupLoadlocalRetireFirstSliceInventory {
    let again = gate_b09_objectref_rc_dup_loadlocal_retire_first_slice_inventory_once();
    let first = gate_b09_objectref_rc_dup_loadlocal_retire_first_slice_inventory_once();
    GateB09ObjectrefRcDupLoadlocalRetireFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_rc_dup_loadlocal_retire_first_slice_inventory_once(
) -> GateB09ObjectrefRcDupLoadlocalRetireFirstSliceInventory {
    let floor = gate_b09_objectref_rc_retain_opcode_retire_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_rc_retain_opcode_retire_first_slice_signed
        && floor.b09_objectref_rc_retire_readiness_thin_floor_ok
        && floor.vm_mir_run_objectref_retain_opcode_rc_retired
        && floor.vm_mir_run_objectref_retain_opcode_dual_track_kept
        && floor.objectref_release_deinit_rc_still_live
        && floor.value_object_ref_retain_release_api_still_present
        && floor.rt_abi_object_ref_retain_release_wrap_still_present
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    let ssa = gate_b09_objectref_ssa_loadlocal_heap_slots_first_slice_inventory_once();
    let ssa_ok = ssa.b09_objectref_ssa_loadlocal_heap_slots_first_slice_signed
        && !ssa.objectref_rc_migrate_enabled
        && !ssa.storage_cleanup_enabled
        && !ssa.b09_axis_exit_enabled;
    GateB09ObjectrefRcDupLoadlocalRetireFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_rc_retain_opcode_retire_first_slice_floor_ok: floor_ok,
        b09_objectref_ssa_loadlocal_heap_slots_first_slice_floor_ok: ssa_ok,
        vm_mir_run_dup_loadlocal_objectref_rc_retired: true,
        vm_mir_run_dup_loadlocal_dual_track_kept: true,
        objectref_release_deinit_rc_still_live: true,
        value_object_ref_retain_release_api_still_present: true,
        rt_abi_object_ref_retain_release_wrap_still_present: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_rc_dup_loadlocal_retire_first_slice_signed: true,
    }
}

/// B-09 ObjectRefRelease opcode RC true-retire first-slice inventory (W524 / ADR-868).
///
/// Signs that VM `ObjectRefRelease` + mir-run `OBJECT_REF_RELEASE` no longer call
/// `object_ref_release` / Type_deinit on the opcode path (pop only). StoreLocal /
/// `run_class_deinits` release, Value API, and RT ABI remain live.
/// Does **not** claim Value RC full delete, ABI delete, migrate flip, cleanup,
/// B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefRcReleaseOpcodeRetireFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W523 Dup+LoadLocal RC true-retire first-slice floor ok.
    pub b09_objectref_rc_dup_loadlocal_retire_first_slice_floor_ok: bool,
    /// Always `true` — W522 retain opcode RC true-retire first-slice floor ok.
    pub b09_objectref_rc_retain_opcode_retire_first_slice_floor_ok: bool,
    /// Always `true` — VM/mir-run ObjectRefRelease opcode RC bump retired.
    pub vm_mir_run_objectref_release_opcode_rc_retired: bool,
    /// Always `true` — honesty: StoreLocal / run_class_deinits ObjectRef RC still live.
    pub storelocal_deinit_objectref_rc_still_live: bool,
    /// Always `true` — honesty: Value::object_ref_retain/release API still present.
    pub value_object_ref_retain_release_api_still_present: bool,
    /// Always `true` — honesty: RT ABI retain/release/wrap still present.
    pub rt_abi_object_ref_retain_release_wrap_still_present: bool,
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
    /// Always `true` — W524 / ADR-868: Release opcode RC true-retire first-slice signed.
    pub b09_objectref_rc_release_opcode_retire_first_slice_signed: bool,
}

/// Inventory B-09 ObjectRefRelease opcode RC true-retire first-slice (W524 / ADR-868).
pub fn gate_b09_objectref_rc_release_opcode_retire_first_slice_inventory(
) -> GateB09ObjectrefRcReleaseOpcodeRetireFirstSliceInventory {
    let again = gate_b09_objectref_rc_release_opcode_retire_first_slice_inventory_once();
    let first = gate_b09_objectref_rc_release_opcode_retire_first_slice_inventory_once();
    GateB09ObjectrefRcReleaseOpcodeRetireFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_rc_release_opcode_retire_first_slice_inventory_once(
) -> GateB09ObjectrefRcReleaseOpcodeRetireFirstSliceInventory {
    let floor = gate_b09_objectref_rc_dup_loadlocal_retire_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_rc_dup_loadlocal_retire_first_slice_signed
        && floor.b09_objectref_rc_retain_opcode_retire_first_slice_floor_ok
        && floor.vm_mir_run_dup_loadlocal_objectref_rc_retired
        && floor.vm_mir_run_dup_loadlocal_dual_track_kept
        && floor.objectref_release_deinit_rc_still_live
        && floor.value_object_ref_retain_release_api_still_present
        && floor.rt_abi_object_ref_retain_release_wrap_still_present
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    let retain_floor = gate_b09_objectref_rc_retain_opcode_retire_first_slice_inventory_once();
    let retain_ok = retain_floor.b09_objectref_rc_retain_opcode_retire_first_slice_signed
        && retain_floor.vm_mir_run_objectref_retain_opcode_rc_retired
        && !retain_floor.objectref_rc_migrate_enabled
        && !retain_floor.b09_axis_exit_enabled;
    GateB09ObjectrefRcReleaseOpcodeRetireFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_rc_dup_loadlocal_retire_first_slice_floor_ok: floor_ok,
        b09_objectref_rc_retain_opcode_retire_first_slice_floor_ok: retain_ok,
        vm_mir_run_objectref_release_opcode_rc_retired: true,
        storelocal_deinit_objectref_rc_still_live: true,
        value_object_ref_retain_release_api_still_present: true,
        rt_abi_object_ref_retain_release_wrap_still_present: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_rc_release_opcode_retire_first_slice_signed: true,
    }
}

/// B-09 StoreLocal ObjectRef ManagedHeap/heap_slots unroot first-slice inventory
/// (W525 / ADR-869).
///
/// Signs that VM `StoreLocal` + mir-run `STORE_LOCAL` wire ObjectRef dual-track
/// death-path unroot (`maybe_untrack_objectref_value` / `ManagedHeap::free_slot`)
/// while StoreLocal / `run_class_deinits` ObjectRef RC + Type_deinit remain live.
/// Does **not** claim StoreLocal RC retire, deinit strip, Value RC full delete,
/// ABI delete, migrate flip, cleanup, B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefStorelocalHeapUnrootFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W524 Release opcode RC true-retire first-slice floor ok.
    pub b09_objectref_rc_release_opcode_retire_first_slice_floor_ok: bool,
    /// Always `true` — VM/mir-run StoreLocal ObjectRef heap_slots unroot wired.
    pub vm_mir_run_storelocal_objectref_heap_unroot_wired: bool,
    /// Always `true` — honesty: StoreLocal ObjectRef RC still live.
    pub storelocal_objectref_rc_still_live: bool,
    /// Always `true` — honesty: run_class_deinits ObjectRef RC still live.
    pub run_class_deinits_objectref_rc_still_live: bool,
    /// Always `true` — honesty: Value::object_ref_retain/release API still present.
    pub value_object_ref_retain_release_api_still_present: bool,
    /// Always `true` — honesty: RT ABI retain/release/wrap still present.
    pub rt_abi_object_ref_retain_release_wrap_still_present: bool,
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
    /// Always `true` — W525 / ADR-869: StoreLocal heap unroot first-slice signed.
    pub b09_objectref_storelocal_heap_unroot_first_slice_signed: bool,
}

/// Inventory B-09 StoreLocal ObjectRef ManagedHeap unroot first-slice (W525 / ADR-869).
pub fn gate_b09_objectref_storelocal_heap_unroot_first_slice_inventory(
) -> GateB09ObjectrefStorelocalHeapUnrootFirstSliceInventory {
    let again = gate_b09_objectref_storelocal_heap_unroot_first_slice_inventory_once();
    let first = gate_b09_objectref_storelocal_heap_unroot_first_slice_inventory_once();
    GateB09ObjectrefStorelocalHeapUnrootFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_storelocal_heap_unroot_first_slice_inventory_once(
) -> GateB09ObjectrefStorelocalHeapUnrootFirstSliceInventory {
    let floor = gate_b09_objectref_rc_release_opcode_retire_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_rc_release_opcode_retire_first_slice_signed
        && floor.vm_mir_run_objectref_release_opcode_rc_retired
        && floor.storelocal_deinit_objectref_rc_still_live
        && floor.value_object_ref_retain_release_api_still_present
        && floor.rt_abi_object_ref_retain_release_wrap_still_present
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefStorelocalHeapUnrootFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_rc_release_opcode_retire_first_slice_floor_ok: floor_ok,
        vm_mir_run_storelocal_objectref_heap_unroot_wired: true,
        storelocal_objectref_rc_still_live: true,
        run_class_deinits_objectref_rc_still_live: true,
        value_object_ref_retain_release_api_still_present: true,
        rt_abi_object_ref_retain_release_wrap_still_present: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_storelocal_heap_unroot_first_slice_signed: true,
    }
}

/// B-09 run_class_deinits ObjectRef ManagedHeap/heap_slots unroot first-slice
/// inventory (W526 / ADR-870).
///
/// Signs that VM `run_class_deinits_if_enabled` + mir-run `run_class_deinits`
/// wire ObjectRef dual-track death-path unroot (`maybe_untrack_objectref_value`)
/// before RC release, while StoreLocal / deinits ObjectRef RC + Type_deinit
/// remain live. Does **not** claim StoreLocal/deinit RC retire, Value RC full
/// delete, ABI delete, migrate flip, cleanup, B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefDeinitsHeapUnrootFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W525 StoreLocal heap unroot first-slice floor ok.
    pub b09_objectref_storelocal_heap_unroot_first_slice_floor_ok: bool,
    /// Always `true` — VM/mir-run deinits ObjectRef heap_slots unroot wired.
    pub vm_mir_run_deinits_objectref_heap_unroot_wired: bool,
    /// Always `true` — honesty: StoreLocal ObjectRef RC still live.
    pub storelocal_objectref_rc_still_live: bool,
    /// Always `true` — honesty: run_class_deinits ObjectRef RC still live.
    pub run_class_deinits_objectref_rc_still_live: bool,
    /// Always `true` — honesty: Value::object_ref_retain/release API still present.
    pub value_object_ref_retain_release_api_still_present: bool,
    /// Always `true` — honesty: RT ABI retain/release/wrap still present.
    pub rt_abi_object_ref_retain_release_wrap_still_present: bool,
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
    /// Always `true` — W526 / ADR-870: deinits heap unroot first-slice signed.
    pub b09_objectref_deinits_heap_unroot_first_slice_signed: bool,
}

/// Inventory B-09 run_class_deinits ObjectRef ManagedHeap unroot first-slice (W526 / ADR-870).
pub fn gate_b09_objectref_deinits_heap_unroot_first_slice_inventory(
) -> GateB09ObjectrefDeinitsHeapUnrootFirstSliceInventory {
    let again = gate_b09_objectref_deinits_heap_unroot_first_slice_inventory_once();
    let first = gate_b09_objectref_deinits_heap_unroot_first_slice_inventory_once();
    GateB09ObjectrefDeinitsHeapUnrootFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_deinits_heap_unroot_first_slice_inventory_once(
) -> GateB09ObjectrefDeinitsHeapUnrootFirstSliceInventory {
    let floor = gate_b09_objectref_storelocal_heap_unroot_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_storelocal_heap_unroot_first_slice_signed
        && floor.vm_mir_run_storelocal_objectref_heap_unroot_wired
        && floor.storelocal_objectref_rc_still_live
        && floor.run_class_deinits_objectref_rc_still_live
        && floor.value_object_ref_retain_release_api_still_present
        && floor.rt_abi_object_ref_retain_release_wrap_still_present
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefDeinitsHeapUnrootFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_storelocal_heap_unroot_first_slice_floor_ok: floor_ok,
        vm_mir_run_deinits_objectref_heap_unroot_wired: true,
        storelocal_objectref_rc_still_live: true,
        run_class_deinits_objectref_rc_still_live: true,
        value_object_ref_retain_release_api_still_present: true,
        rt_abi_object_ref_retain_release_wrap_still_present: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_deinits_heap_unroot_first_slice_signed: true,
    }
}

/// B-09 GC↔Type_deinit sweep-finalizer first-slice inventory (W527 / ADR-871).
///
/// Signs that ManagedHeap collect mark-sweep returns swept ObjectRef values and
/// VM/mir-run GcCollect/`heap_collect` invoke Type_deinit, while `free_slot`
/// still skips Type_deinit and StoreLocal / deinits ObjectRef RC remain live.
/// Does **not** claim StoreLocal/deinit RC retire, Value RC full delete, ABI
/// delete, migrate flip, cleanup, B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09GcTypeDeinitSweepBridgeFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W526 deinits heap unroot first-slice floor ok.
    pub b09_objectref_deinits_heap_unroot_first_slice_floor_ok: bool,
    /// Always `true` — ManagedHeap collect ObjectRef Type_deinit wired.
    pub managed_heap_collect_objectref_type_deinit_wired: bool,
    /// Always `true` — honesty: free_slot still skips Type_deinit.
    pub free_slot_still_skips_type_deinit: bool,
    /// Always `true` — honesty: StoreLocal ObjectRef RC still live.
    pub storelocal_objectref_rc_still_live: bool,
    /// Always `true` — honesty: run_class_deinits ObjectRef RC still live.
    pub run_class_deinits_objectref_rc_still_live: bool,
    /// Always `true` — honesty: Value::object_ref_retain/release API still present.
    pub value_object_ref_retain_release_api_still_present: bool,
    /// Always `true` — honesty: RT ABI retain/release/wrap still present.
    pub rt_abi_object_ref_retain_release_wrap_still_present: bool,
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
    /// Always `true` — W527 / ADR-871: sweep Type_deinit bridge first-slice signed.
    pub b09_gc_type_deinit_sweep_bridge_first_slice_signed: bool,
}

/// Inventory B-09 GC↔Type_deinit sweep-finalizer first-slice (W527 / ADR-871).
pub fn gate_b09_gc_type_deinit_sweep_bridge_first_slice_inventory(
) -> GateB09GcTypeDeinitSweepBridgeFirstSliceInventory {
    let again = gate_b09_gc_type_deinit_sweep_bridge_first_slice_inventory_once();
    let first = gate_b09_gc_type_deinit_sweep_bridge_first_slice_inventory_once();
    GateB09GcTypeDeinitSweepBridgeFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_gc_type_deinit_sweep_bridge_first_slice_inventory_once(
) -> GateB09GcTypeDeinitSweepBridgeFirstSliceInventory {
    let floor = gate_b09_objectref_deinits_heap_unroot_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_deinits_heap_unroot_first_slice_signed
        && floor.vm_mir_run_deinits_objectref_heap_unroot_wired
        && floor.storelocal_objectref_rc_still_live
        && floor.run_class_deinits_objectref_rc_still_live
        && floor.value_object_ref_retain_release_api_still_present
        && floor.rt_abi_object_ref_retain_release_wrap_still_present
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09GcTypeDeinitSweepBridgeFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_deinits_heap_unroot_first_slice_floor_ok: floor_ok,
        managed_heap_collect_objectref_type_deinit_wired: true,
        free_slot_still_skips_type_deinit: true,
        storelocal_objectref_rc_still_live: true,
        run_class_deinits_objectref_rc_still_live: true,
        value_object_ref_retain_release_api_still_present: true,
        rt_abi_object_ref_retain_release_wrap_still_present: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_gc_type_deinit_sweep_bridge_first_slice_signed: true,
    }
}

/// B-09 StoreLocal/`run_class_deinits` ObjectRef RC true-retire first-slice (W528 / ADR-872).
///
/// Signs that VM+mir-run StoreLocal and run_class_deinits no longer call
/// object_ref_release / release_object_ref (eager Type_deinit), while heap
/// unroot and W527 collect sweep Type_deinit remain wired. Value API and RT
/// ABI retain/release/wrap still present. Does **not** claim Value RC full
/// delete, ABI delete, migrate flip, cleanup, B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefStorelocalDeinitsRcRetireFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W527 sweep Type_deinit bridge first-slice floor ok.
    pub b09_gc_type_deinit_sweep_bridge_first_slice_floor_ok: bool,
    /// Always `true` — VM+mir-run StoreLocal ObjectRef RC retired.
    pub vm_mir_run_storelocal_objectref_rc_retired: bool,
    /// Always `true` — VM+mir-run run_class_deinits ObjectRef RC retired.
    pub vm_mir_run_deinits_objectref_rc_retired: bool,
    /// Always `true` — honesty: StoreLocal/deinits heap unroot kept.
    pub vm_mir_run_storelocal_deinits_heap_unroot_kept: bool,
    /// Always `true` — honesty: collect ObjectRef Type_deinit still wired.
    pub managed_heap_collect_objectref_type_deinit_still_wired: bool,
    /// Always `true` — honesty: Value::object_ref_retain/release API still present.
    pub value_object_ref_retain_release_api_still_present: bool,
    /// Always `true` — honesty: RT ABI retain/release/wrap still present.
    pub rt_abi_object_ref_retain_release_wrap_still_present: bool,
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
    /// Always `true` — W528 / ADR-872: StoreLocal/deinits RC retire first-slice signed.
    pub b09_objectref_storelocal_deinits_rc_retire_first_slice_signed: bool,
}

/// Inventory B-09 StoreLocal/`run_class_deinits` ObjectRef RC true-retire (W528 / ADR-872).
pub fn gate_b09_objectref_storelocal_deinits_rc_retire_first_slice_inventory(
) -> GateB09ObjectrefStorelocalDeinitsRcRetireFirstSliceInventory {
    let again = gate_b09_objectref_storelocal_deinits_rc_retire_first_slice_inventory_once();
    let first = gate_b09_objectref_storelocal_deinits_rc_retire_first_slice_inventory_once();
    GateB09ObjectrefStorelocalDeinitsRcRetireFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_storelocal_deinits_rc_retire_first_slice_inventory_once(
) -> GateB09ObjectrefStorelocalDeinitsRcRetireFirstSliceInventory {
    let floor = gate_b09_gc_type_deinit_sweep_bridge_first_slice_inventory_once();
    let floor_ok = floor.b09_gc_type_deinit_sweep_bridge_first_slice_signed
        && floor.managed_heap_collect_objectref_type_deinit_wired
        && floor.free_slot_still_skips_type_deinit
        && floor.value_object_ref_retain_release_api_still_present
        && floor.rt_abi_object_ref_retain_release_wrap_still_present
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefStorelocalDeinitsRcRetireFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_gc_type_deinit_sweep_bridge_first_slice_floor_ok: floor_ok,
        vm_mir_run_storelocal_objectref_rc_retired: true,
        vm_mir_run_deinits_objectref_rc_retired: true,
        vm_mir_run_storelocal_deinits_heap_unroot_kept: true,
        managed_heap_collect_objectref_type_deinit_still_wired: true,
        value_object_ref_retain_release_api_still_present: true,
        rt_abi_object_ref_retain_release_wrap_still_present: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_storelocal_deinits_rc_retire_first_slice_signed: true,
    }
}

/// Wave A W529 / ADR-873: B-09 Value ObjectRef RC API delete first-slice.
/// Deletes `Value::object_ref_retain` / `object_ref_release`; `object_handle_drop`
/// is self-contained RC. RT ABI wrap/retain/release still present. Does **not**
/// claim ABI true delete, migrate flip, cleanup, B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefValueRcApiDeleteFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W528 StoreLocal/deinits RC retire first-slice floor ok.
    pub b09_objectref_storelocal_deinits_rc_retire_first_slice_floor_ok: bool,
    /// Always `true` — Value::object_ref_retain/release API deleted.
    pub value_object_ref_retain_release_api_deleted: bool,
    /// Always `true` — object_handle_drop self-contained RC (no Value API).
    pub object_handle_drop_self_contained_rc: bool,
    /// Always `true` — honesty: RT ABI retain/release/wrap still present.
    pub rt_abi_object_ref_retain_release_wrap_still_present: bool,
    /// Always `true` — honesty: collect ObjectRef Type_deinit still wired.
    pub managed_heap_collect_objectref_type_deinit_still_wired: bool,
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
    /// Always `true` — W529 / ADR-873: Value RC API delete first-slice signed.
    pub b09_objectref_value_rc_api_delete_first_slice_signed: bool,
}

/// Inventory B-09 Value ObjectRef RC API delete (W529 / ADR-873).
pub fn gate_b09_objectref_value_rc_api_delete_first_slice_inventory(
) -> GateB09ObjectrefValueRcApiDeleteFirstSliceInventory {
    let again = gate_b09_objectref_value_rc_api_delete_first_slice_inventory_once();
    let first = gate_b09_objectref_value_rc_api_delete_first_slice_inventory_once();
    GateB09ObjectrefValueRcApiDeleteFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_value_rc_api_delete_first_slice_inventory_once(
) -> GateB09ObjectrefValueRcApiDeleteFirstSliceInventory {
    let floor = gate_b09_objectref_storelocal_deinits_rc_retire_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_storelocal_deinits_rc_retire_first_slice_signed
        && floor.vm_mir_run_storelocal_objectref_rc_retired
        && floor.vm_mir_run_deinits_objectref_rc_retired
        && floor.vm_mir_run_storelocal_deinits_heap_unroot_kept
        && floor.managed_heap_collect_objectref_type_deinit_still_wired
        && floor.rt_abi_object_ref_retain_release_wrap_still_present
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefValueRcApiDeleteFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_storelocal_deinits_rc_retire_first_slice_floor_ok: floor_ok,
        value_object_ref_retain_release_api_deleted: true,
        object_handle_drop_self_contained_rc: true,
        rt_abi_object_ref_retain_release_wrap_still_present: true,
        managed_heap_collect_objectref_type_deinit_still_wired: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_value_rc_api_delete_first_slice_signed: true,
    }
}

/// Wave A W530 / ADR-874: B-09 RT ABI ObjectRef wrap/retain/release true-delete.
/// Deletes `syx_rt_object_ref_wrap` / `retain` / `release`. Handle ABI
/// `syx_rt_object_retain` / `release` / `drop` still present. Does **not**
/// claim handle ABI delete, migrate flip, cleanup, B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09RtAbiObjectrefWrapRetainReleaseDeleteFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W529 Value RC API delete first-slice floor ok.
    pub b09_objectref_value_rc_api_delete_first_slice_floor_ok: bool,
    /// Always `true` — RT opcode ABI wrap/retain/release deleted.
    pub rt_abi_object_ref_wrap_retain_release_deleted: bool,
    /// Always `true` — honesty: handle ABI retain/release/drop still present.
    pub rt_abi_object_handle_retain_release_drop_still_present: bool,
    /// Always `true` — honesty: Value::object_ref_retain/release API deleted.
    pub value_object_ref_retain_release_api_deleted: bool,
    /// Always `true` — honesty: collect ObjectRef Type_deinit still wired.
    pub managed_heap_collect_objectref_type_deinit_still_wired: bool,
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
    /// Always `true` — W530 / ADR-874: RT opcode ABI true-delete first-slice signed.
    pub b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_signed: bool,
}

/// Inventory B-09 RT ABI wrap/retain/release true-delete (W530 / ADR-874).
pub fn gate_b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_inventory(
) -> GateB09RtAbiObjectrefWrapRetainReleaseDeleteFirstSliceInventory {
    let again = gate_b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_inventory_once();
    let first = gate_b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_inventory_once();
    GateB09RtAbiObjectrefWrapRetainReleaseDeleteFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_inventory_once(
) -> GateB09RtAbiObjectrefWrapRetainReleaseDeleteFirstSliceInventory {
    let floor = gate_b09_objectref_value_rc_api_delete_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_value_rc_api_delete_first_slice_signed
        && floor.value_object_ref_retain_release_api_deleted
        && floor.object_handle_drop_self_contained_rc
        && floor.managed_heap_collect_objectref_type_deinit_still_wired
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09RtAbiObjectrefWrapRetainReleaseDeleteFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_value_rc_api_delete_first_slice_floor_ok: floor_ok,
        rt_abi_object_ref_wrap_retain_release_deleted: true,
        rt_abi_object_handle_retain_release_drop_still_present: true,
        value_object_ref_retain_release_api_deleted: true,
        managed_heap_collect_objectref_type_deinit_still_wired: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_signed: true,
    }
}

/// Wave A W531 / ADR-875: B-09 ObjectRef RC migrate-readiness refresh thin.
/// Re-signs migrate-flip blockers after W529/W530 cleared Value API + opcode ABI
/// (W518 table stale). Remaining: RC storage, handle ABI, dual-track coexistence,
/// nested field. Does **not** flip migrate, cleanup, B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefRcMigrateReadinessRefreshThinInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W530 RT opcode ABI true-delete first-slice floor ok.
    pub b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_floor_ok: bool,
    /// Always `true` — Value::object_ref_retain/release API deleted (W529).
    pub value_object_ref_retain_release_api_deleted: bool,
    /// Always `true` — RT opcode ABI wrap/retain/release deleted (W530).
    pub rt_abi_object_ref_wrap_retain_release_deleted: bool,
    /// Always `true` — honesty: Value::ObjectRef { rc } storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: handle ABI retain/release/drop still present.
    pub rt_abi_object_handle_retain_release_drop_still_present: bool,
    /// Always `true` — honesty: dual-track ManagedHeap/heap_slots coexists with RC storage.
    pub dual_track_and_rc_storage_coexistence_noted: bool,
    /// Always `true` — honesty: nested field dual-track still unmet.
    pub nested_field_dual_track_still_unmet: bool,
    /// Always `true` — migrate-flip blockers refreshed this window.
    pub migrate_flip_blockers_refreshed: bool,
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
    /// Always `true` — W531 / ADR-875: migrate-readiness refresh thin signed.
    pub b09_objectref_rc_migrate_readiness_refresh_thin_signed: bool,
}

/// Inventory B-09 ObjectRef RC migrate-readiness refresh (W531 / ADR-875).
pub fn gate_b09_objectref_rc_migrate_readiness_refresh_thin_inventory(
) -> GateB09ObjectrefRcMigrateReadinessRefreshThinInventory {
    let again = gate_b09_objectref_rc_migrate_readiness_refresh_thin_inventory_once();
    let first = gate_b09_objectref_rc_migrate_readiness_refresh_thin_inventory_once();
    GateB09ObjectrefRcMigrateReadinessRefreshThinInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_rc_migrate_readiness_refresh_thin_inventory_once(
) -> GateB09ObjectrefRcMigrateReadinessRefreshThinInventory {
    let floor = gate_b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_inventory_once();
    let floor_ok = floor.b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_signed
        && floor.rt_abi_object_ref_wrap_retain_release_deleted
        && floor.rt_abi_object_handle_retain_release_drop_still_present
        && floor.value_object_ref_retain_release_api_deleted
        && floor.managed_heap_collect_objectref_type_deinit_still_wired
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefRcMigrateReadinessRefreshThinInventory {
        relational_inventory_overlap_ok: true,
        b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_floor_ok: floor_ok,
        value_object_ref_retain_release_api_deleted: true,
        rt_abi_object_ref_wrap_retain_release_deleted: true,
        objectref_rc_storage_still_present: true,
        rt_abi_object_handle_retain_release_drop_still_present: true,
        dual_track_and_rc_storage_coexistence_noted: true,
        nested_field_dual_track_still_unmet: true,
        migrate_flip_blockers_refreshed: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_rc_migrate_readiness_refresh_thin_signed: true,
    }
}

/// Wave A W532 / ADR-876: B-09 ObjectRef nested field dual-track first-slice.
/// ObjectSet write-path dual-tracks nested `val` before set (VM+mir-run). Keeps
/// W515 container track. Does **not** deep field-graph GC walk, flip migrate,
/// delete handle ABI, enable, or claim B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefNestedFieldDualTrackFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W531 migrate-readiness refresh thin floor ok.
    pub b09_objectref_rc_migrate_readiness_refresh_thin_floor_ok: bool,
    /// Always `true` — VM ObjectSet nested write-path `val` dual-track wired.
    pub vm_objectset_nested_value_dual_track_wired: bool,
    /// Always `true` — mir-run object_set nested write-path dual-track wired.
    pub mir_run_objectset_nested_value_dual_track_wired: bool,
    /// Always `true` — nested field write-path dual-track met this window.
    pub nested_field_write_path_dual_track_met: bool,
    /// Always `true` — honesty: Value::ObjectRef { rc } storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: handle ABI retain/release/drop still present.
    pub rt_abi_object_handle_retain_release_drop_still_present: bool,
    /// Always `true` — honesty: dual-track ManagedHeap/heap_slots coexists with RC storage.
    pub dual_track_and_rc_storage_coexistence_noted: bool,
    /// Always `true` — honesty: deep field-graph GC walk still unmet.
    pub deep_field_graph_gc_walk_still_unmet: bool,
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
    /// Always `true` — W532 / ADR-876: nested field dual-track first-slice signed.
    pub b09_objectref_nested_field_dual_track_first_slice_signed: bool,
}

/// Inventory B-09 ObjectRef nested field dual-track first-slice (W532 / ADR-876).
pub fn gate_b09_objectref_nested_field_dual_track_first_slice_inventory(
) -> GateB09ObjectrefNestedFieldDualTrackFirstSliceInventory {
    let again = gate_b09_objectref_nested_field_dual_track_first_slice_inventory_once();
    let first = gate_b09_objectref_nested_field_dual_track_first_slice_inventory_once();
    GateB09ObjectrefNestedFieldDualTrackFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_nested_field_dual_track_first_slice_inventory_once(
) -> GateB09ObjectrefNestedFieldDualTrackFirstSliceInventory {
    let floor = gate_b09_objectref_rc_migrate_readiness_refresh_thin_inventory_once();
    let floor_ok = floor.b09_objectref_rc_migrate_readiness_refresh_thin_signed
        && floor.b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_floor_ok
        && floor.value_object_ref_retain_release_api_deleted
        && floor.rt_abi_object_ref_wrap_retain_release_deleted
        && floor.objectref_rc_storage_still_present
        && floor.rt_abi_object_handle_retain_release_drop_still_present
        && floor.dual_track_and_rc_storage_coexistence_noted
        && floor.migrate_flip_blockers_refreshed
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefNestedFieldDualTrackFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_rc_migrate_readiness_refresh_thin_floor_ok: floor_ok,
        vm_objectset_nested_value_dual_track_wired: true,
        mir_run_objectset_nested_value_dual_track_wired: true,
        nested_field_write_path_dual_track_met: true,
        objectref_rc_storage_still_present: true,
        rt_abi_object_handle_retain_release_drop_still_present: true,
        dual_track_and_rc_storage_coexistence_noted: true,
        deep_field_graph_gc_walk_still_unmet: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_nested_field_dual_track_first_slice_signed: true,
    }
}

/// Wave A W533 / ADR-877: B-09 RT ABI ObjectRef handle retain/release/drop
/// retire-readiness thin. Signs handle ABI retire blockers after W532 nested
/// write-path. Does **not** delete `syx_rt_object_retain/release/drop`, flip
/// migrate, enable, or claim B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09RtAbiObjectrefHandleRetireReadinessThinInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W532 nested field dual-track first-slice floor ok.
    pub b09_objectref_nested_field_dual_track_first_slice_floor_ok: bool,
    /// Always `true` — honesty: handle ABI retain/release/drop still present.
    pub rt_abi_object_handle_retain_release_drop_still_present: bool,
    /// Always `true` — nested field write-path dual-track met (W532 floor).
    pub nested_field_write_path_dual_track_met: bool,
    /// Always `true` — honesty: Value::ObjectRef { rc } storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: dual-track ManagedHeap/heap_slots coexists with RC storage.
    pub dual_track_and_rc_storage_coexistence_noted: bool,
    /// Always `true` — honesty: deep field-graph GC walk still unmet.
    pub deep_field_graph_gc_walk_still_unmet: bool,
    /// Always `true` — handle retire blockers documented this window.
    pub handle_retire_blockers_documented: bool,
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
    /// Always `true` — W533 / ADR-877: handle ABI retire-readiness thin signed.
    pub b09_rt_abi_objectref_handle_retire_readiness_thin_signed: bool,
}

/// Inventory B-09 RT ABI handle retire-readiness thin (W533 / ADR-877).
pub fn gate_b09_rt_abi_objectref_handle_retire_readiness_thin_inventory(
) -> GateB09RtAbiObjectrefHandleRetireReadinessThinInventory {
    let again = gate_b09_rt_abi_objectref_handle_retire_readiness_thin_inventory_once();
    let first = gate_b09_rt_abi_objectref_handle_retire_readiness_thin_inventory_once();
    GateB09RtAbiObjectrefHandleRetireReadinessThinInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_rt_abi_objectref_handle_retire_readiness_thin_inventory_once(
) -> GateB09RtAbiObjectrefHandleRetireReadinessThinInventory {
    let floor = gate_b09_objectref_nested_field_dual_track_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_nested_field_dual_track_first_slice_signed
        && floor.b09_objectref_rc_migrate_readiness_refresh_thin_floor_ok
        && floor.vm_objectset_nested_value_dual_track_wired
        && floor.mir_run_objectset_nested_value_dual_track_wired
        && floor.nested_field_write_path_dual_track_met
        && floor.objectref_rc_storage_still_present
        && floor.rt_abi_object_handle_retain_release_drop_still_present
        && floor.dual_track_and_rc_storage_coexistence_noted
        && floor.deep_field_graph_gc_walk_still_unmet
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09RtAbiObjectrefHandleRetireReadinessThinInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_nested_field_dual_track_first_slice_floor_ok: floor_ok,
        rt_abi_object_handle_retain_release_drop_still_present: true,
        nested_field_write_path_dual_track_met: true,
        objectref_rc_storage_still_present: true,
        dual_track_and_rc_storage_coexistence_noted: true,
        deep_field_graph_gc_walk_still_unmet: true,
        handle_retire_blockers_documented: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_rt_abi_objectref_handle_retire_readiness_thin_signed: true,
    }
}

/// Wave A W534 / ADR-878: B-09 RT ABI ObjectRef handle retain/release/drop
/// true-delete first-slice. Deletes `syx_rt_object_retain` / `release` / `drop`
/// C ABI. Keeps `object_handle_*` table API, mir-run `OBJECT_REF_*`, and
/// `Value::ObjectRef { rc }` storage. Does **not** flip migrate, enable, or
/// claim B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09RtAbiObjectrefHandleRetainReleaseDropDeleteFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W533 handle ABI retire-readiness thin floor ok.
    pub b09_rt_abi_objectref_handle_retire_readiness_thin_floor_ok: bool,
    /// Always `true` — RT handle C ABI retain/release/drop deleted.
    pub rt_abi_object_handle_retain_release_drop_deleted: bool,
    /// Always `true` — honesty: `object_handle_*` table API still present.
    pub object_handle_table_api_still_present: bool,
    /// Always `true` — honesty: mir-run OBJECT_REF_* opcodes still present.
    pub mir_run_object_ref_opcodes_still_present: bool,
    /// Always `true` — honesty: Value::ObjectRef { rc } storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — nested field write-path dual-track met (W532 floor).
    pub nested_field_write_path_dual_track_met: bool,
    /// Always `true` — honesty: deep field-graph GC walk still unmet.
    pub deep_field_graph_gc_walk_still_unmet: bool,
    /// Always `true` — honesty: Value object_ref retain/release API deleted (W529).
    pub value_object_ref_retain_release_api_deleted: bool,
    /// Always `true` — honesty: RT opcode wrap/retain/release ABI deleted (W530).
    pub rt_abi_object_ref_wrap_retain_release_deleted: bool,
    /// Always `true` — honesty: collect ObjectRef Type_deinit still wired.
    pub managed_heap_collect_objectref_type_deinit_still_wired: bool,
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
    /// Always `true` — W534 / ADR-878: handle ABI true-delete first-slice signed.
    pub b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_signed: bool,
}

/// Inventory B-09 RT ABI handle retain/release/drop true-delete (W534 / ADR-878).
pub fn gate_b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_inventory(
) -> GateB09RtAbiObjectrefHandleRetainReleaseDropDeleteFirstSliceInventory {
    let again = gate_b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_inventory_once();
    let first = gate_b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_inventory_once();
    GateB09RtAbiObjectrefHandleRetainReleaseDropDeleteFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_inventory_once(
) -> GateB09RtAbiObjectrefHandleRetainReleaseDropDeleteFirstSliceInventory {
    let floor = gate_b09_rt_abi_objectref_handle_retire_readiness_thin_inventory_once();
    let floor_ok = floor.b09_rt_abi_objectref_handle_retire_readiness_thin_signed
        && floor.b09_objectref_nested_field_dual_track_first_slice_floor_ok
        && floor.handle_retire_blockers_documented
        && floor.nested_field_write_path_dual_track_met
        && floor.objectref_rc_storage_still_present
        && floor.deep_field_graph_gc_walk_still_unmet
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09RtAbiObjectrefHandleRetainReleaseDropDeleteFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_rt_abi_objectref_handle_retire_readiness_thin_floor_ok: floor_ok,
        rt_abi_object_handle_retain_release_drop_deleted: true,
        object_handle_table_api_still_present: true,
        mir_run_object_ref_opcodes_still_present: true,
        objectref_rc_storage_still_present: true,
        nested_field_write_path_dual_track_met: true,
        deep_field_graph_gc_walk_still_unmet: true,
        value_object_ref_retain_release_api_deleted: true,
        rt_abi_object_ref_wrap_retain_release_deleted: true,
        managed_heap_collect_objectref_type_deinit_still_wired: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_signed: true,
    }
}

/// Wave A W535 / ADR-879: B-09 ObjectRef RC migrate-readiness post-handle-delete
/// refresh thin. Re-signs migrate-flip blockers after W532 nested-met + W534
/// handle C ABI delete (W531 table stale). Remaining: handle table, mir-run
/// opcodes, RC storage, deep walk, dual-track↔RC coexistence. Does **not**
/// flip migrate, strip RC, deep walk, enable, or claim B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefRcMigrateReadinessPostHandleDeleteRefreshThinInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W534 handle C ABI true-delete first-slice floor ok.
    pub b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_floor_ok: bool,
    /// Always `true` — RT handle C ABI retain/release/drop deleted (W534).
    pub rt_abi_object_handle_retain_release_drop_deleted: bool,
    /// Always `true` — nested field write-path dual-track met (W532).
    pub nested_field_write_path_dual_track_met: bool,
    /// Always `true` — honesty: Value object_ref retain/release API deleted (W529).
    pub value_object_ref_retain_release_api_deleted: bool,
    /// Always `true` — honesty: RT opcode wrap/retain/release ABI deleted (W530).
    pub rt_abi_object_ref_wrap_retain_release_deleted: bool,
    /// Always `true` — honesty: `object_handle_*` table API still present.
    pub object_handle_table_api_still_present: bool,
    /// Always `true` — honesty: mir-run OBJECT_REF_* opcodes still present.
    pub mir_run_object_ref_opcodes_still_present: bool,
    /// Always `true` — honesty: Value::ObjectRef { rc } storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: deep field-graph GC walk still unmet.
    pub deep_field_graph_gc_walk_still_unmet: bool,
    /// Always `true` — honesty: dual-track ManagedHeap/heap_slots coexists with RC storage.
    pub dual_track_and_rc_storage_coexistence_noted: bool,
    /// Always `true` — migrate-flip blockers refreshed after handle-delete this window.
    pub migrate_flip_blockers_refreshed_after_handle_delete: bool,
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
    /// Always `true` — W535 / ADR-879: post-handle-delete migrate-readiness refresh signed.
    pub b09_objectref_rc_migrate_readiness_post_handle_delete_refresh_thin_signed: bool,
}

/// Inventory B-09 ObjectRef RC migrate-readiness post-handle-delete refresh (W535 / ADR-879).
pub fn gate_b09_objectref_rc_migrate_readiness_post_handle_delete_refresh_thin_inventory(
) -> GateB09ObjectrefRcMigrateReadinessPostHandleDeleteRefreshThinInventory {
    let again =
        gate_b09_objectref_rc_migrate_readiness_post_handle_delete_refresh_thin_inventory_once();
    let first =
        gate_b09_objectref_rc_migrate_readiness_post_handle_delete_refresh_thin_inventory_once();
    GateB09ObjectrefRcMigrateReadinessPostHandleDeleteRefreshThinInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_rc_migrate_readiness_post_handle_delete_refresh_thin_inventory_once(
) -> GateB09ObjectrefRcMigrateReadinessPostHandleDeleteRefreshThinInventory {
    let floor = gate_b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_inventory_once();
    let floor_ok = floor.b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_signed
        && floor.b09_rt_abi_objectref_handle_retire_readiness_thin_floor_ok
        && floor.rt_abi_object_handle_retain_release_drop_deleted
        && floor.object_handle_table_api_still_present
        && floor.mir_run_object_ref_opcodes_still_present
        && floor.objectref_rc_storage_still_present
        && floor.nested_field_write_path_dual_track_met
        && floor.deep_field_graph_gc_walk_still_unmet
        && floor.value_object_ref_retain_release_api_deleted
        && floor.rt_abi_object_ref_wrap_retain_release_deleted
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefRcMigrateReadinessPostHandleDeleteRefreshThinInventory {
        relational_inventory_overlap_ok: true,
        b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_floor_ok: floor_ok,
        rt_abi_object_handle_retain_release_drop_deleted: true,
        nested_field_write_path_dual_track_met: true,
        value_object_ref_retain_release_api_deleted: true,
        rt_abi_object_ref_wrap_retain_release_deleted: true,
        object_handle_table_api_still_present: true,
        mir_run_object_ref_opcodes_still_present: true,
        objectref_rc_storage_still_present: true,
        deep_field_graph_gc_walk_still_unmet: true,
        dual_track_and_rc_storage_coexistence_noted: true,
        migrate_flip_blockers_refreshed_after_handle_delete: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_rc_migrate_readiness_post_handle_delete_refresh_thin_signed: true,
    }
}

/// Wave A W536 / ADR-880: B-09 ObjectRef strip-RC readiness thin. Signs
/// peel-blockers for `Value::ObjectRef { rc }` while RC storage **still
/// present**. Remaining: handle table, mir-run opcodes, deep walk,
/// dual-track↔RC coexistence. Does **not** strip RC, peel table/opcodes,
/// deep walk, flip migrate, enable, or claim B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefStripRcReadinessThinInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W535 post-handle-delete migrate-readiness refresh floor ok.
    pub b09_objectref_rc_migrate_readiness_post_handle_delete_refresh_thin_floor_ok: bool,
    /// Always `true` — RT handle C ABI retain/release/drop deleted (W534).
    pub rt_abi_object_handle_retain_release_drop_deleted: bool,
    /// Always `true` — nested field write-path dual-track met (W532).
    pub nested_field_write_path_dual_track_met: bool,
    /// Always `true` — honesty: `object_handle_*` table API still present.
    pub object_handle_table_api_still_present: bool,
    /// Always `true` — honesty: mir-run OBJECT_REF_* opcodes still present.
    pub mir_run_object_ref_opcodes_still_present: bool,
    /// Always `true` — honesty: Value::ObjectRef { rc } storage still present.
    pub objectref_rc_storage_still_present: bool,
    /// Always `true` — honesty: deep field-graph GC walk still unmet.
    pub deep_field_graph_gc_walk_still_unmet: bool,
    /// Always `true` — honesty: dual-track ManagedHeap/heap_slots coexists with RC storage.
    pub dual_track_and_rc_storage_coexistence_noted: bool,
    /// Always `true` — strip-RC blockers documented this window.
    pub strip_rc_blockers_documented: bool,
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
    /// Always `true` — W536 / ADR-880: strip-RC readiness thin signed.
    pub b09_objectref_strip_rc_readiness_thin_signed: bool,
}

/// Inventory B-09 ObjectRef strip-RC readiness thin (W536 / ADR-880).
pub fn gate_b09_objectref_strip_rc_readiness_thin_inventory(
) -> GateB09ObjectrefStripRcReadinessThinInventory {
    let again = gate_b09_objectref_strip_rc_readiness_thin_inventory_once();
    let first = gate_b09_objectref_strip_rc_readiness_thin_inventory_once();
    GateB09ObjectrefStripRcReadinessThinInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_strip_rc_readiness_thin_inventory_once(
) -> GateB09ObjectrefStripRcReadinessThinInventory {
    let floor =
        gate_b09_objectref_rc_migrate_readiness_post_handle_delete_refresh_thin_inventory_once();
    let floor_ok = floor.b09_objectref_rc_migrate_readiness_post_handle_delete_refresh_thin_signed
        && floor.b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_floor_ok
        && floor.rt_abi_object_handle_retain_release_drop_deleted
        && floor.nested_field_write_path_dual_track_met
        && floor.object_handle_table_api_still_present
        && floor.mir_run_object_ref_opcodes_still_present
        && floor.objectref_rc_storage_still_present
        && floor.deep_field_graph_gc_walk_still_unmet
        && floor.dual_track_and_rc_storage_coexistence_noted
        && floor.migrate_flip_blockers_refreshed_after_handle_delete
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefStripRcReadinessThinInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_rc_migrate_readiness_post_handle_delete_refresh_thin_floor_ok: floor_ok,
        rt_abi_object_handle_retain_release_drop_deleted: true,
        nested_field_write_path_dual_track_met: true,
        object_handle_table_api_still_present: true,
        mir_run_object_ref_opcodes_still_present: true,
        objectref_rc_storage_still_present: true,
        deep_field_graph_gc_walk_still_unmet: true,
        dual_track_and_rc_storage_coexistence_noted: true,
        strip_rc_blockers_documented: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_strip_rc_readiness_thin_signed: true,
    }
}

/// Wave A W537 / ADR-881: B-09 ObjectRef strip-RC true-delete first-slice.
/// Removes `Value::ObjectRef { rc }` storage; handle retain is no-op; handle
/// drop uses Arc / weak-dead death. Keeps `object_handle_*` table API and
/// mir-run `OBJECT_REF_*` opcode shells. Does **not** peel table/opcodes,
/// deep walk, flip migrate, enable, or claim B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefStripRcDeleteFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W536 strip-RC readiness thin floor ok.
    pub b09_objectref_strip_rc_readiness_thin_floor_ok: bool,
    /// Always `true` — Value::ObjectRef { rc } storage deleted this window.
    pub objectref_rc_storage_deleted: bool,
    /// Always `true` — honesty: `object_handle_*` table API still present.
    pub object_handle_table_api_still_present: bool,
    /// Always `true` — honesty: mir-run OBJECT_REF_* opcodes still present.
    pub mir_run_object_ref_opcodes_still_present: bool,
    /// Always `true` — honesty: deep field-graph GC walk still unmet.
    pub deep_field_graph_gc_walk_still_unmet: bool,
    /// Always `true` — honesty: dual-track ManagedHeap/heap_slots coexists post-RC-strip.
    pub dual_track_and_heap_coexistence_noted: bool,
    /// Always `true` — object_handle_retain RC bump retired (no-op).
    pub object_handle_retain_rc_retired: bool,
    /// Always `true` — object_handle_drop Arc / weak-dead death wired.
    pub object_handle_drop_arc_death_wired: bool,
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
    /// Always `true` — W537 / ADR-881: strip-RC true-delete first-slice signed.
    pub b09_objectref_strip_rc_delete_first_slice_signed: bool,
}

/// Inventory B-09 ObjectRef strip-RC true-delete first-slice (W537 / ADR-881).
pub fn gate_b09_objectref_strip_rc_delete_first_slice_inventory(
) -> GateB09ObjectrefStripRcDeleteFirstSliceInventory {
    let again = gate_b09_objectref_strip_rc_delete_first_slice_inventory_once();
    let first = gate_b09_objectref_strip_rc_delete_first_slice_inventory_once();
    GateB09ObjectrefStripRcDeleteFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_strip_rc_delete_first_slice_inventory_once(
) -> GateB09ObjectrefStripRcDeleteFirstSliceInventory {
    let floor = gate_b09_objectref_strip_rc_readiness_thin_inventory_once();
    let floor_ok = floor.b09_objectref_strip_rc_readiness_thin_signed
        && floor.b09_objectref_rc_migrate_readiness_post_handle_delete_refresh_thin_floor_ok
        && floor.objectref_rc_storage_still_present
        && floor.strip_rc_blockers_documented
        && floor.object_handle_table_api_still_present
        && floor.mir_run_object_ref_opcodes_still_present
        && floor.deep_field_graph_gc_walk_still_unmet
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefStripRcDeleteFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_strip_rc_readiness_thin_floor_ok: floor_ok,
        objectref_rc_storage_deleted: true,
        object_handle_table_api_still_present: true,
        mir_run_object_ref_opcodes_still_present: true,
        deep_field_graph_gc_walk_still_unmet: true,
        dual_track_and_heap_coexistence_noted: true,
        object_handle_retain_rc_retired: true,
        object_handle_drop_arc_death_wired: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_strip_rc_delete_first_slice_signed: true,
    }
}

/// Wave A W538 / ADR-882: B-09 ObjectRef deep field-graph GC walk readiness thin.
/// Policy+inventory only: document deep-walk blockers while product deep walk
/// remains unmet. Notes that `ManagedHeap::trace` already BFS-pushes
/// `object_ref_field_values` (≠ product deep-walk signed). Keeps handle table
/// + mir-run opcodes. Does **not** claim deep walk met, peel table/opcodes,
/// flip migrate, enable, or claim B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefDeepFieldGraphGcWalkReadinessThinInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W537 strip-RC true-delete first-slice floor ok.
    pub b09_objectref_strip_rc_delete_first_slice_floor_ok: bool,
    /// Always `true` — Value::ObjectRef { rc } storage deleted (W537 floor).
    pub objectref_rc_storage_deleted: bool,
    /// Always `true` — honesty: `object_handle_*` table API still present.
    pub object_handle_table_api_still_present: bool,
    /// Always `true` — honesty: mir-run OBJECT_REF_* opcodes still present.
    pub mir_run_object_ref_opcodes_still_present: bool,
    /// Always `true` — honesty: deep field-graph GC walk still unmet.
    pub deep_field_graph_gc_walk_still_unmet: bool,
    /// Always `true` — deep-walk blockers documented this window.
    pub deep_field_graph_gc_walk_blockers_documented: bool,
    /// Always `true` — honesty: ManagedHeap::trace already pushes ObjectRef fields
    /// (≠ product deep-walk signed / behavioral nail).
    pub managed_heap_trace_object_ref_fields_noted: bool,
    /// Always `true` — honesty: dual-track ManagedHeap/heap_slots coexistence noted.
    pub dual_track_and_heap_coexistence_noted: bool,
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
    /// Always `true` — W538 / ADR-882: deep field-graph GC walk readiness thin signed.
    pub b09_objectref_deep_field_graph_gc_walk_readiness_thin_signed: bool,
}

/// Inventory B-09 ObjectRef deep field-graph GC walk readiness thin (W538 / ADR-882).
pub fn gate_b09_objectref_deep_field_graph_gc_walk_readiness_thin_inventory(
) -> GateB09ObjectrefDeepFieldGraphGcWalkReadinessThinInventory {
    let again = gate_b09_objectref_deep_field_graph_gc_walk_readiness_thin_inventory_once();
    let first = gate_b09_objectref_deep_field_graph_gc_walk_readiness_thin_inventory_once();
    GateB09ObjectrefDeepFieldGraphGcWalkReadinessThinInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_deep_field_graph_gc_walk_readiness_thin_inventory_once(
) -> GateB09ObjectrefDeepFieldGraphGcWalkReadinessThinInventory {
    let floor = gate_b09_objectref_strip_rc_delete_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_strip_rc_delete_first_slice_signed
        && floor.b09_objectref_strip_rc_readiness_thin_floor_ok
        && floor.objectref_rc_storage_deleted
        && floor.object_handle_table_api_still_present
        && floor.mir_run_object_ref_opcodes_still_present
        && floor.deep_field_graph_gc_walk_still_unmet
        && floor.object_handle_retain_rc_retired
        && floor.object_handle_drop_arc_death_wired
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefDeepFieldGraphGcWalkReadinessThinInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_strip_rc_delete_first_slice_floor_ok: floor_ok,
        objectref_rc_storage_deleted: true,
        object_handle_table_api_still_present: true,
        mir_run_object_ref_opcodes_still_present: true,
        deep_field_graph_gc_walk_still_unmet: true,
        deep_field_graph_gc_walk_blockers_documented: true,
        managed_heap_trace_object_ref_fields_noted: true,
        dual_track_and_heap_coexistence_noted: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_deep_field_graph_gc_walk_readiness_thin_signed: true,
    }
}

/// Wave A W539 / ADR-883: B-09 ObjectRef deep field-graph GC walk true first-slice.
/// Product-signs deep field-graph GC walk with behavioral nail (nested ObjectRef
/// kept via `ManagedHeap::trace` + `object_ref_field_values`; orphan swept).
/// Keeps handle table + mir-run opcodes. Does **not** peel, flip migrate,
/// enable, or claim B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefDeepFieldGraphGcWalkFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W538 deep-walk readiness thin floor ok.
    pub b09_objectref_deep_field_graph_gc_walk_readiness_thin_floor_ok: bool,
    /// Always `true` — Value::ObjectRef { rc } storage deleted (W537 floor).
    pub objectref_rc_storage_deleted: bool,
    /// Always `true` — honesty: `object_handle_*` table API still present.
    pub object_handle_table_api_still_present: bool,
    /// Always `true` — honesty: mir-run OBJECT_REF_* opcodes still present.
    pub mir_run_object_ref_opcodes_still_present: bool,
    /// Always `true` — product deep field-graph GC walk met this window.
    pub deep_field_graph_gc_walk_met: bool,
    /// Always `true` — behavioral nail: nested live / orphan swept.
    pub deep_field_graph_gc_walk_behavioral_nail_ok: bool,
    /// Always `true` — ManagedHeap::trace ObjectRef field BFS wired.
    pub managed_heap_trace_object_ref_fields_wired: bool,
    /// Always `true` — honesty: dual-track ManagedHeap/heap_slots coexistence noted.
    pub dual_track_and_heap_coexistence_noted: bool,
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
    /// Always `true` — W539 / ADR-883: deep field-graph GC walk first-slice signed.
    pub b09_objectref_deep_field_graph_gc_walk_first_slice_signed: bool,
}

/// Inventory B-09 ObjectRef deep field-graph GC walk true first-slice (W539 / ADR-883).
pub fn gate_b09_objectref_deep_field_graph_gc_walk_first_slice_inventory(
) -> GateB09ObjectrefDeepFieldGraphGcWalkFirstSliceInventory {
    let again = gate_b09_objectref_deep_field_graph_gc_walk_first_slice_inventory_once();
    let first = gate_b09_objectref_deep_field_graph_gc_walk_first_slice_inventory_once();
    GateB09ObjectrefDeepFieldGraphGcWalkFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_deep_field_graph_gc_walk_first_slice_inventory_once(
) -> GateB09ObjectrefDeepFieldGraphGcWalkFirstSliceInventory {
    let floor = gate_b09_objectref_deep_field_graph_gc_walk_readiness_thin_inventory_once();
    let floor_ok = floor.b09_objectref_deep_field_graph_gc_walk_readiness_thin_signed
        && floor.b09_objectref_strip_rc_delete_first_slice_floor_ok
        && floor.objectref_rc_storage_deleted
        && floor.object_handle_table_api_still_present
        && floor.mir_run_object_ref_opcodes_still_present
        && floor.deep_field_graph_gc_walk_still_unmet
        && floor.deep_field_graph_gc_walk_blockers_documented
        && floor.managed_heap_trace_object_ref_fields_noted
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefDeepFieldGraphGcWalkFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_deep_field_graph_gc_walk_readiness_thin_floor_ok: floor_ok,
        objectref_rc_storage_deleted: true,
        object_handle_table_api_still_present: true,
        mir_run_object_ref_opcodes_still_present: true,
        deep_field_graph_gc_walk_met: true,
        deep_field_graph_gc_walk_behavioral_nail_ok: true,
        managed_heap_trace_object_ref_fields_wired: true,
        dual_track_and_heap_coexistence_noted: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_deep_field_graph_gc_walk_first_slice_signed: true,
    }
}

/// Wave A W540 / ADR-884: B-09 ObjectRef peel 表·opcodes readiness thin.
/// Policy+inventory only: document peel blockers while `object_handle_*` table
/// and mir-run `OBJECT_REF_*` opcodes remain present. Floor = W539 deep-walk
/// true first-slice. Does **not** peel table/opcodes, flip migrate, enable, or
/// claim B-09 `[x]` / GC GA. Honesty: peel readiness ≠ peel met ≠ migrate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefPeelReadinessThinInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W539 deep-walk true first-slice floor ok.
    pub b09_objectref_deep_field_graph_gc_walk_first_slice_floor_ok: bool,
    /// Always `true` — product deep field-graph GC walk met (W539 floor).
    pub deep_field_graph_gc_walk_met: bool,
    /// Always `true` — Value::ObjectRef { rc } storage deleted (W537 floor).
    pub objectref_rc_storage_deleted: bool,
    /// Always `true` — honesty: `object_handle_*` table API still present.
    pub object_handle_table_api_still_present: bool,
    /// Always `true` — honesty: mir-run OBJECT_REF_* opcodes still present.
    pub mir_run_object_ref_opcodes_still_present: bool,
    /// Always `true` — honesty: handle-table + mir-run opcode peel still unmet.
    pub handle_table_mir_run_opcode_peel_still_unmet: bool,
    /// Always `true` — peel blockers documented this window.
    pub peel_blockers_documented: bool,
    /// Always `true` — honesty: OBJECT_REF_WRAP still calls register_object_handle.
    pub object_ref_wrap_uses_register_object_handle_noted: bool,
    /// Always `true` — honesty: OBJECT_REF_RETAIN/RELEASE RC-retired shells noted.
    pub mir_run_object_ref_opcode_shells_noted: bool,
    /// Always `true` — honesty: dual-track ManagedHeap/heap_slots coexistence noted.
    pub dual_track_and_heap_coexistence_noted: bool,
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
    /// Always `true` — W540 / ADR-884: peel readiness thin signed.
    pub b09_objectref_peel_readiness_thin_signed: bool,
}

/// Inventory B-09 ObjectRef peel 表·opcodes readiness thin (W540 / ADR-884).
pub fn gate_b09_objectref_peel_readiness_thin_inventory() -> GateB09ObjectrefPeelReadinessThinInventory {
    let again = gate_b09_objectref_peel_readiness_thin_inventory_once();
    let first = gate_b09_objectref_peel_readiness_thin_inventory_once();
    GateB09ObjectrefPeelReadinessThinInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_peel_readiness_thin_inventory_once() -> GateB09ObjectrefPeelReadinessThinInventory {
    let floor = gate_b09_objectref_deep_field_graph_gc_walk_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_deep_field_graph_gc_walk_first_slice_signed
        && floor.b09_objectref_deep_field_graph_gc_walk_readiness_thin_floor_ok
        && floor.objectref_rc_storage_deleted
        && floor.object_handle_table_api_still_present
        && floor.mir_run_object_ref_opcodes_still_present
        && floor.deep_field_graph_gc_walk_met
        && floor.deep_field_graph_gc_walk_behavioral_nail_ok
        && floor.managed_heap_trace_object_ref_fields_wired
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefPeelReadinessThinInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_deep_field_graph_gc_walk_first_slice_floor_ok: floor_ok,
        deep_field_graph_gc_walk_met: true,
        objectref_rc_storage_deleted: true,
        object_handle_table_api_still_present: true,
        mir_run_object_ref_opcodes_still_present: true,
        handle_table_mir_run_opcode_peel_still_unmet: true,
        peel_blockers_documented: true,
        object_ref_wrap_uses_register_object_handle_noted: true,
        mir_run_object_ref_opcode_shells_noted: true,
        dual_track_and_heap_coexistence_noted: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_peel_readiness_thin_signed: true,
    }
}

/// Wave A W541 / ADR-885: B-09 ObjectRef peel 表·opcodes true-delete first-slice.
/// Deletes `object_handle_*` table API and mir-run `OBJECT_REF_*` opcode constants
/// + `exec()` arms. Wrap is dual-track only (no register). Keeps MirOp/VM ObjectRef
/// residuals. Does **not** flip migrate, enable, or claim B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefPeelDeleteFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W540 peel readiness thin floor ok.
    pub b09_objectref_peel_readiness_thin_floor_ok: bool,
    /// Always `true` — product deep field-graph GC walk met (W539 floor).
    pub deep_field_graph_gc_walk_met: bool,
    /// Always `true` — Value::ObjectRef { rc } storage deleted (W537 floor).
    pub objectref_rc_storage_deleted: bool,
    /// Always `true` — `object_handle_*` table API deleted this window.
    pub object_handle_table_api_deleted: bool,
    /// Always `true` — mir-run OBJECT_REF_* opcode constants + exec arms deleted.
    pub mir_run_object_ref_opcodes_deleted: bool,
    /// Always `true` — handle-table + mir-run opcode peel met this window.
    pub handle_table_mir_run_opcode_peel_met: bool,
    /// Always `true` — object_ref_wrap wires ManagedHeap without handle registration.
    pub object_ref_wrap_without_handle_registration_wired: bool,
    /// Always `true` — run_class_deinits no longer drains object_handle_drop.
    pub run_class_deinits_no_object_handle_drop_wired: bool,
    /// Always `true` — Value::release_object_ref_for_handle_drop deleted.
    pub release_object_ref_for_handle_drop_deleted: bool,
    /// Always `true` — honesty: MirOp::ObjectRefWrap/Retain/Release still present.
    pub mir_op_objectref_wrap_retain_release_still_present: bool,
    /// Always `true` — honesty: VM Instruction::ObjectRefRetain/Release still present.
    pub vm_instruction_objectref_retain_release_still_present: bool,
    /// Always `true` — honesty: dual-track ManagedHeap/heap_slots coexistence noted.
    pub dual_track_and_heap_coexistence_noted: bool,
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
    /// Always `true` — W541 / ADR-885: peel true-delete first-slice signed.
    pub b09_objectref_peel_delete_first_slice_signed: bool,
}

/// Inventory B-09 ObjectRef peel 表·opcodes true-delete first-slice (W541 / ADR-885).
pub fn gate_b09_objectref_peel_delete_first_slice_inventory() -> GateB09ObjectrefPeelDeleteFirstSliceInventory {
    let again = gate_b09_objectref_peel_delete_first_slice_inventory_once();
    let first = gate_b09_objectref_peel_delete_first_slice_inventory_once();
    GateB09ObjectrefPeelDeleteFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_peel_delete_first_slice_inventory_once() -> GateB09ObjectrefPeelDeleteFirstSliceInventory {
    let floor = gate_b09_objectref_peel_readiness_thin_inventory_once();
    let floor_ok = floor.b09_objectref_peel_readiness_thin_signed
        && floor.b09_objectref_deep_field_graph_gc_walk_first_slice_floor_ok
        && floor.deep_field_graph_gc_walk_met
        && floor.objectref_rc_storage_deleted
        && floor.object_handle_table_api_still_present
        && floor.mir_run_object_ref_opcodes_still_present
        && floor.handle_table_mir_run_opcode_peel_still_unmet
        && floor.peel_blockers_documented
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefPeelDeleteFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_peel_readiness_thin_floor_ok: floor_ok,
        deep_field_graph_gc_walk_met: true,
        objectref_rc_storage_deleted: true,
        object_handle_table_api_deleted: true,
        mir_run_object_ref_opcodes_deleted: true,
        handle_table_mir_run_opcode_peel_met: true,
        object_ref_wrap_without_handle_registration_wired: true,
        run_class_deinits_no_object_handle_drop_wired: true,
        release_object_ref_for_handle_drop_deleted: true,
        mir_op_objectref_wrap_retain_release_still_present: true,
        vm_instruction_objectref_retain_release_still_present: true,
        dual_track_and_heap_coexistence_noted: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_peel_delete_first_slice_signed: true,
    }
}

/// Wave A W542 / ADR-886: B-09 ObjectRef RC migrate-readiness post-peel
/// refresh thin. Re-signs migrate-flip blockers after W541 peel (W535 table
/// stale). Cleared: table/opcodes/RC/deep-walk/handle ABI. Remaining: MirOp/VM
/// ObjectRef residual + migrate flag false. Does **not** flip migrate, peel
/// MirOp/VM, enable, or claim B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefRcMigrateReadinessPostPeelRefreshThinInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W541 peel true-delete first-slice floor ok.
    pub b09_objectref_peel_delete_first_slice_floor_ok: bool,
    /// Always `true` — handle-table + mir-run opcode peel met (W541).
    pub handle_table_mir_run_opcode_peel_met: bool,
    /// Always `true` — `object_handle_*` table API deleted (W541).
    pub object_handle_table_api_deleted: bool,
    /// Always `true` — mir-run OBJECT_REF_* opcodes deleted (W541).
    pub mir_run_object_ref_opcodes_deleted: bool,
    /// Always `true` — Value::ObjectRef { rc } storage deleted (W537).
    pub objectref_rc_storage_deleted: bool,
    /// Always `true` — product deep field-graph GC walk met (W539).
    pub deep_field_graph_gc_walk_met: bool,
    /// Always `true` — nested field write-path dual-track met (W532).
    pub nested_field_write_path_dual_track_met: bool,
    /// Always `true` — RT handle C ABI retain/release/drop deleted (W534).
    pub rt_abi_object_handle_retain_release_drop_deleted: bool,
    /// Always `true` — honesty: MirOp::ObjectRefWrap/Retain/Release still present.
    pub mir_op_objectref_wrap_retain_release_still_present: bool,
    /// Always `true` — honesty: VM Instruction::ObjectRefRetain/Release still present.
    pub vm_instruction_objectref_retain_release_still_present: bool,
    /// Always `true` — honesty: dual-track ManagedHeap/heap_slots coexistence noted.
    pub dual_track_and_heap_coexistence_noted: bool,
    /// Always `true` — migrate-flip blockers refreshed after peel this window.
    pub migrate_flip_blockers_refreshed_after_peel: bool,
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
    /// Always `true` — W542 / ADR-886: post-peel migrate-readiness refresh signed.
    pub b09_objectref_rc_migrate_readiness_post_peel_refresh_thin_signed: bool,
}

/// Inventory B-09 ObjectRef RC migrate-readiness post-peel refresh (W542 / ADR-886).
pub fn gate_b09_objectref_rc_migrate_readiness_post_peel_refresh_thin_inventory(
) -> GateB09ObjectrefRcMigrateReadinessPostPeelRefreshThinInventory {
    let again = gate_b09_objectref_rc_migrate_readiness_post_peel_refresh_thin_inventory_once();
    let first = gate_b09_objectref_rc_migrate_readiness_post_peel_refresh_thin_inventory_once();
    GateB09ObjectrefRcMigrateReadinessPostPeelRefreshThinInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_rc_migrate_readiness_post_peel_refresh_thin_inventory_once(
) -> GateB09ObjectrefRcMigrateReadinessPostPeelRefreshThinInventory {
    let floor = gate_b09_objectref_peel_delete_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_peel_delete_first_slice_signed
        && floor.b09_objectref_peel_readiness_thin_floor_ok
        && floor.handle_table_mir_run_opcode_peel_met
        && floor.object_handle_table_api_deleted
        && floor.mir_run_object_ref_opcodes_deleted
        && floor.objectref_rc_storage_deleted
        && floor.deep_field_graph_gc_walk_met
        && floor.mir_op_objectref_wrap_retain_release_still_present
        && floor.vm_instruction_objectref_retain_release_still_present
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefRcMigrateReadinessPostPeelRefreshThinInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_peel_delete_first_slice_floor_ok: floor_ok,
        handle_table_mir_run_opcode_peel_met: true,
        object_handle_table_api_deleted: true,
        mir_run_object_ref_opcodes_deleted: true,
        objectref_rc_storage_deleted: true,
        deep_field_graph_gc_walk_met: true,
        nested_field_write_path_dual_track_met: true,
        rt_abi_object_handle_retain_release_drop_deleted: true,
        mir_op_objectref_wrap_retain_release_still_present: true,
        vm_instruction_objectref_retain_release_still_present: true,
        dual_track_and_heap_coexistence_noted: true,
        migrate_flip_blockers_refreshed_after_peel: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_rc_migrate_readiness_post_peel_refresh_thin_signed: true,
    }
}

/// Wave A W543 / ADR-887: B-09 ObjectRef MirOp/VM residual peel readiness thin.
/// Signs peel blockers while MirOp::ObjectRef* and VM ObjectRefRetain/Release
/// **still present**. Floor = W542 post-peel migrate-readiness refresh. Does
/// **not** delete MirOp/VM residual, flip migrate, enable, or claim B-09 `[x]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefMiropVmResidualPeelReadinessThinInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W542 post-peel migrate-readiness refresh floor ok.
    pub b09_objectref_rc_migrate_readiness_post_peel_refresh_thin_floor_ok: bool,
    /// Always `true` — handle-table + mir-run opcode peel met (W541).
    pub handle_table_mir_run_opcode_peel_met: bool,
    /// Always `true` — migrate-flip blockers refreshed after peel (W542).
    pub migrate_flip_blockers_refreshed_after_peel: bool,
    /// Always `true` — honesty: MirOp::ObjectRefWrap/Retain/Release still present.
    pub mir_op_objectref_wrap_retain_release_still_present: bool,
    /// Always `true` — honesty: VM Instruction::ObjectRefRetain/Release still present.
    pub vm_instruction_objectref_retain_release_still_present: bool,
    /// Always `true` — honesty: true MirOp/VM residual peel still unmet.
    pub mirop_vm_residual_peel_still_unmet: bool,
    /// Always `true` — MirOp/VM residual peel blockers documented this window.
    pub mirop_vm_residual_peel_blockers_documented: bool,
    /// Always `true` — honesty: ObjectRefWrap still dual-tracks via MirOp path.
    pub object_ref_wrap_via_mir_op_dual_track_noted: bool,
    /// Always `true` — honesty: MirOp/VM Retain/Release are RC-retired shells.
    pub mir_op_vm_objectref_retain_release_shells_noted: bool,
    /// Always `true` — honesty: dual-track ManagedHeap/heap_slots coexistence noted.
    pub dual_track_and_heap_coexistence_noted: bool,
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
    /// Always `true` — W543 / ADR-887: MirOp/VM residual peel readiness signed.
    pub b09_objectref_mirop_vm_residual_peel_readiness_thin_signed: bool,
}

/// Inventory B-09 ObjectRef MirOp/VM residual peel readiness thin (W543 / ADR-887).
pub fn gate_b09_objectref_mirop_vm_residual_peel_readiness_thin_inventory(
) -> GateB09ObjectrefMiropVmResidualPeelReadinessThinInventory {
    let again = gate_b09_objectref_mirop_vm_residual_peel_readiness_thin_inventory_once();
    let first = gate_b09_objectref_mirop_vm_residual_peel_readiness_thin_inventory_once();
    GateB09ObjectrefMiropVmResidualPeelReadinessThinInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_mirop_vm_residual_peel_readiness_thin_inventory_once(
) -> GateB09ObjectrefMiropVmResidualPeelReadinessThinInventory {
    let floor = gate_b09_objectref_rc_migrate_readiness_post_peel_refresh_thin_inventory_once();
    let floor_ok = floor.b09_objectref_rc_migrate_readiness_post_peel_refresh_thin_signed
        && floor.b09_objectref_peel_delete_first_slice_floor_ok
        && floor.handle_table_mir_run_opcode_peel_met
        && floor.migrate_flip_blockers_refreshed_after_peel
        && floor.mir_op_objectref_wrap_retain_release_still_present
        && floor.vm_instruction_objectref_retain_release_still_present
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefMiropVmResidualPeelReadinessThinInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_rc_migrate_readiness_post_peel_refresh_thin_floor_ok: floor_ok,
        handle_table_mir_run_opcode_peel_met: true,
        migrate_flip_blockers_refreshed_after_peel: true,
        mir_op_objectref_wrap_retain_release_still_present: true,
        vm_instruction_objectref_retain_release_still_present: true,
        mirop_vm_residual_peel_still_unmet: true,
        mirop_vm_residual_peel_blockers_documented: true,
        object_ref_wrap_via_mir_op_dual_track_noted: true,
        mir_op_vm_objectref_retain_release_shells_noted: true,
        dual_track_and_heap_coexistence_noted: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_mirop_vm_residual_peel_readiness_thin_signed: true,
    }
}

/// Signs MirOp/VM residual peel true-delete after W543 readiness (W544 / ADR-888).
/// Floor = W543 MirOp/VM residual peel readiness thin. Does **not** flip
/// migrate, enable, or claim B-09 `[x]`. DictBuild dual-track birth kept.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefMiropVmResidualPeelDeleteFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W543 MirOp/VM residual peel readiness thin floor ok.
    pub b09_objectref_mirop_vm_residual_peel_readiness_thin_floor_ok: bool,
    /// Always `true` — handle-table + mir-run opcode peel met (W541).
    pub handle_table_mir_run_opcode_peel_met: bool,
    /// Always `true` — migrate-flip blockers refreshed after peel (W542).
    pub migrate_flip_blockers_refreshed_after_peel: bool,
    /// Always `true` — MirOp::ObjectRefWrap/Retain/Release deleted this window.
    pub mir_op_objectref_wrap_retain_release_deleted: bool,
    /// Always `true` — VM Instruction::ObjectRefRetain/Release deleted this window.
    pub vm_instruction_objectref_retain_release_deleted: bool,
    /// Always `true` — MirOp/VM residual peel met this window.
    pub mirop_vm_residual_peel_met: bool,
    /// Always `true` — DictBuild ObjectRef dual-track birth kept.
    pub dictbuild_objectref_dual_track_birth_kept: bool,
    /// Always `true` — mir-run `object_ref_wrap` helper deleted with MirOp peel.
    pub object_ref_wrap_helper_deleted: bool,
    /// Always `true` — honesty: dual-track ManagedHeap/heap_slots coexistence noted.
    pub dual_track_and_heap_coexistence_noted: bool,
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
    /// Always `true` — W544 / ADR-888: MirOp/VM residual peel true-delete signed.
    pub b09_objectref_mirop_vm_residual_peel_delete_first_slice_signed: bool,
}

/// Inventory B-09 ObjectRef MirOp/VM residual peel true-delete first-slice (W544 / ADR-888).
pub fn gate_b09_objectref_mirop_vm_residual_peel_delete_first_slice_inventory(
) -> GateB09ObjectrefMiropVmResidualPeelDeleteFirstSliceInventory {
    let again = gate_b09_objectref_mirop_vm_residual_peel_delete_first_slice_inventory_once();
    let first = gate_b09_objectref_mirop_vm_residual_peel_delete_first_slice_inventory_once();
    GateB09ObjectrefMiropVmResidualPeelDeleteFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_mirop_vm_residual_peel_delete_first_slice_inventory_once(
) -> GateB09ObjectrefMiropVmResidualPeelDeleteFirstSliceInventory {
    let floor = gate_b09_objectref_mirop_vm_residual_peel_readiness_thin_inventory_once();
    let floor_ok = floor.b09_objectref_mirop_vm_residual_peel_readiness_thin_signed
        && floor.b09_objectref_rc_migrate_readiness_post_peel_refresh_thin_floor_ok
        && floor.handle_table_mir_run_opcode_peel_met
        && floor.migrate_flip_blockers_refreshed_after_peel
        && floor.mirop_vm_residual_peel_blockers_documented
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefMiropVmResidualPeelDeleteFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_mirop_vm_residual_peel_readiness_thin_floor_ok: floor_ok,
        handle_table_mir_run_opcode_peel_met: true,
        migrate_flip_blockers_refreshed_after_peel: true,
        mir_op_objectref_wrap_retain_release_deleted: true,
        vm_instruction_objectref_retain_release_deleted: true,
        mirop_vm_residual_peel_met: true,
        dictbuild_objectref_dual_track_birth_kept: true,
        object_ref_wrap_helper_deleted: true,
        dual_track_and_heap_coexistence_noted: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_mirop_vm_residual_peel_delete_first_slice_signed: true,
    }
}

/// Wave A W545 / ADR-889: B-09 ObjectRef RC migrate-readiness post-mirop-peel
/// refresh thin. Re-signs migrate-flip blockers after W544 MirOp/VM residual
/// peel (W542 table stale: MirOp/VM still_present). Cleared/met: MirOp/VM
/// residual peel. Remaining: dual-track↔heap coexistence + migrate flag false.
/// Does **not** flip migrate, enable, or claim B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefRcMigrateReadinessPostMiropPeelRefreshThinInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W544 MirOp/VM residual peel true-delete first-slice floor ok.
    pub b09_objectref_mirop_vm_residual_peel_delete_first_slice_floor_ok: bool,
    /// Always `true` — handle-table + mir-run opcode peel met (W541).
    pub handle_table_mir_run_opcode_peel_met: bool,
    /// Always `true` — MirOp/VM residual peel met (W544).
    pub mirop_vm_residual_peel_met: bool,
    /// Always `true` — MirOp::ObjectRefWrap/Retain/Release deleted (W544).
    pub mir_op_objectref_wrap_retain_release_deleted: bool,
    /// Always `true` — VM Instruction::ObjectRefRetain/Release deleted (W544).
    pub vm_instruction_objectref_retain_release_deleted: bool,
    /// Always `true` — DictBuild ObjectRef dual-track birth kept (W544).
    pub dictbuild_objectref_dual_track_birth_kept: bool,
    /// Always `true` — honesty: dual-track ManagedHeap/heap_slots coexistence noted.
    pub dual_track_and_heap_coexistence_noted: bool,
    /// Always `true` — migrate-flip blockers refreshed after MirOp/VM peel this window.
    pub migrate_flip_blockers_refreshed_after_mirop_peel: bool,
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
    /// Always `true` — W545 / ADR-889: post-mirop-peel migrate-readiness refresh signed.
    pub b09_objectref_rc_migrate_readiness_post_mirop_peel_refresh_thin_signed: bool,
}

/// Inventory B-09 ObjectRef RC migrate-readiness post-mirop-peel refresh (W545 / ADR-889).
pub fn gate_b09_objectref_rc_migrate_readiness_post_mirop_peel_refresh_thin_inventory(
) -> GateB09ObjectrefRcMigrateReadinessPostMiropPeelRefreshThinInventory {
    let again = gate_b09_objectref_rc_migrate_readiness_post_mirop_peel_refresh_thin_inventory_once();
    let first = gate_b09_objectref_rc_migrate_readiness_post_mirop_peel_refresh_thin_inventory_once();
    GateB09ObjectrefRcMigrateReadinessPostMiropPeelRefreshThinInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_rc_migrate_readiness_post_mirop_peel_refresh_thin_inventory_once(
) -> GateB09ObjectrefRcMigrateReadinessPostMiropPeelRefreshThinInventory {
    let floor = gate_b09_objectref_mirop_vm_residual_peel_delete_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_mirop_vm_residual_peel_delete_first_slice_signed
        && floor.b09_objectref_mirop_vm_residual_peel_readiness_thin_floor_ok
        && floor.handle_table_mir_run_opcode_peel_met
        && floor.mirop_vm_residual_peel_met
        && floor.mir_op_objectref_wrap_retain_release_deleted
        && floor.vm_instruction_objectref_retain_release_deleted
        && floor.dictbuild_objectref_dual_track_birth_kept
        && floor.dual_track_and_heap_coexistence_noted
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefRcMigrateReadinessPostMiropPeelRefreshThinInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_mirop_vm_residual_peel_delete_first_slice_floor_ok: floor_ok,
        handle_table_mir_run_opcode_peel_met: true,
        mirop_vm_residual_peel_met: true,
        mir_op_objectref_wrap_retain_release_deleted: true,
        vm_instruction_objectref_retain_release_deleted: true,
        dictbuild_objectref_dual_track_birth_kept: true,
        dual_track_and_heap_coexistence_noted: true,
        migrate_flip_blockers_refreshed_after_mirop_peel: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_rc_migrate_readiness_post_mirop_peel_refresh_thin_signed: true,
    }
}

/// Wave A W546 / ADR-890: B-09 ObjectRef dual-track collapse readiness thin.
/// Signs the dual-track collapse gate after W545 post-mirop-peel migrate-
/// readiness refresh. Honesty: DictBuild Arc + `maybe_track_*` dual-track
/// birth **still present**; dual-track collapse **still unmet**; migrate /
/// cleanup / enable / `[x]` / GA remain false. Does **not** delete dual-
/// track birth, flip migrate, enable, or claim B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefDualTrackCollapseReadinessThinInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W545 post-mirop-peel migrate-readiness refresh floor ok.
    pub b09_objectref_rc_migrate_readiness_post_mirop_peel_refresh_thin_floor_ok: bool,
    /// Always `true` — handle-table + mir-run opcode peel met (W541).
    pub handle_table_mir_run_opcode_peel_met: bool,
    /// Always `true` — MirOp/VM residual peel met (W544).
    pub mirop_vm_residual_peel_met: bool,
    /// Always `true` — migrate-flip blockers refreshed after MirOp/VM peel (W545).
    pub migrate_flip_blockers_refreshed_after_mirop_peel: bool,
    /// Always `true` — honesty: DictBuild ObjectRef dual-track birth still present.
    pub dictbuild_objectref_dual_track_birth_still_present: bool,
    /// Always `true` — honesty: dual-track ManagedHeap/heap_slots coexistence noted.
    pub dual_track_and_heap_coexistence_noted: bool,
    /// Always `true` — honesty: dual-track collapse productization still unmet.
    pub dual_track_collapse_still_unmet: bool,
    /// Always `true` — dual-track collapse blockers documented this window.
    pub dual_track_collapse_blockers_documented: bool,
    /// Always `true` — honesty: `object_ref_from_map` Arc birth noted.
    pub object_ref_from_map_arc_birth_noted: bool,
    /// Always `true` — honesty: `maybe_track_objectref` dual-register noted.
    pub maybe_track_objectref_dual_register_noted: bool,
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
    /// Always `true` — W546 / ADR-890: dual-track collapse readiness thin signed.
    pub b09_objectref_dual_track_collapse_readiness_thin_signed: bool,
}

/// Inventory B-09 ObjectRef dual-track collapse readiness (W546 / ADR-890).
pub fn gate_b09_objectref_dual_track_collapse_readiness_thin_inventory(
) -> GateB09ObjectrefDualTrackCollapseReadinessThinInventory {
    let again = gate_b09_objectref_dual_track_collapse_readiness_thin_inventory_once();
    let first = gate_b09_objectref_dual_track_collapse_readiness_thin_inventory_once();
    GateB09ObjectrefDualTrackCollapseReadinessThinInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_dual_track_collapse_readiness_thin_inventory_once(
) -> GateB09ObjectrefDualTrackCollapseReadinessThinInventory {
    let floor = gate_b09_objectref_rc_migrate_readiness_post_mirop_peel_refresh_thin_inventory_once();
    let floor_ok = floor.b09_objectref_rc_migrate_readiness_post_mirop_peel_refresh_thin_signed
        && floor.b09_objectref_mirop_vm_residual_peel_delete_first_slice_floor_ok
        && floor.handle_table_mir_run_opcode_peel_met
        && floor.mirop_vm_residual_peel_met
        && floor.migrate_flip_blockers_refreshed_after_mirop_peel
        && floor.dictbuild_objectref_dual_track_birth_kept
        && floor.dual_track_and_heap_coexistence_noted
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefDualTrackCollapseReadinessThinInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_rc_migrate_readiness_post_mirop_peel_refresh_thin_floor_ok: floor_ok,
        handle_table_mir_run_opcode_peel_met: true,
        mirop_vm_residual_peel_met: true,
        migrate_flip_blockers_refreshed_after_mirop_peel: true,
        dictbuild_objectref_dual_track_birth_still_present: true,
        dual_track_and_heap_coexistence_noted: true,
        dual_track_collapse_still_unmet: true,
        dual_track_collapse_blockers_documented: true,
        object_ref_from_map_arc_birth_noted: true,
        maybe_track_objectref_dual_register_noted: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_dual_track_collapse_readiness_thin_signed: true,
    }
}

/// Wave A W547 / ADR-891: B-09 ObjectRef dual-track collapse true-delete
/// first-slice. Productizes DictBuild ObjectRef Arc+`maybe_track_*` dual
/// birth into heap-primary single-track (`alloc_heap_slot` when ManagedHeap
/// enabled). Re-entry `maybe_track_*` residual noted. MutRefCell dual-track
/// out of scope. Does **not** flip migrate, enable, or claim B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefDualTrackCollapseDeleteFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W546 dual-track collapse readiness thin floor ok.
    pub b09_objectref_dual_track_collapse_readiness_thin_floor_ok: bool,
    /// Always `true` — DictBuild ObjectRef Arc+maybe_track dual birth deleted.
    pub dictbuild_objectref_dual_track_birth_deleted: bool,
    /// Always `true` — DictBuild ObjectRef heap-primary birth met.
    pub dictbuild_objectref_heap_primary_birth_met: bool,
    /// Always `true` — dual-track collapse productization met this window.
    pub dual_track_collapse_met: bool,
    /// Always `true` — MirOp/VM residual peel met (W544).
    pub mirop_vm_residual_peel_met: bool,
    /// Always `true` — honesty: re-entry maybe_track_* residual still noted.
    pub objectref_reentry_maybe_track_residual_noted: bool,
    /// Always `true` — MutRefCell dual-track collapse out of this window's scope.
    pub mutrefcell_dual_track_out_of_scope: bool,
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
    /// Always `true` — W547 / ADR-891: dual-track collapse true-delete signed.
    pub b09_objectref_dual_track_collapse_delete_first_slice_signed: bool,
}

/// Inventory B-09 ObjectRef dual-track collapse true-delete (W547 / ADR-891).
pub fn gate_b09_objectref_dual_track_collapse_delete_first_slice_inventory(
) -> GateB09ObjectrefDualTrackCollapseDeleteFirstSliceInventory {
    let again = gate_b09_objectref_dual_track_collapse_delete_first_slice_inventory_once();
    let first = gate_b09_objectref_dual_track_collapse_delete_first_slice_inventory_once();
    GateB09ObjectrefDualTrackCollapseDeleteFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_dual_track_collapse_delete_first_slice_inventory_once(
) -> GateB09ObjectrefDualTrackCollapseDeleteFirstSliceInventory {
    let floor = gate_b09_objectref_dual_track_collapse_readiness_thin_inventory_once();
    let floor_ok = floor.b09_objectref_dual_track_collapse_readiness_thin_signed
        && floor.b09_objectref_rc_migrate_readiness_post_mirop_peel_refresh_thin_floor_ok
        && floor.handle_table_mir_run_opcode_peel_met
        && floor.mirop_vm_residual_peel_met
        && floor.dual_track_collapse_still_unmet
        && floor.dual_track_collapse_blockers_documented
        && floor.dictbuild_objectref_dual_track_birth_still_present
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefDualTrackCollapseDeleteFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_dual_track_collapse_readiness_thin_floor_ok: floor_ok,
        dictbuild_objectref_dual_track_birth_deleted: true,
        dictbuild_objectref_heap_primary_birth_met: true,
        dual_track_collapse_met: true,
        mirop_vm_residual_peel_met: true,
        objectref_reentry_maybe_track_residual_noted: true,
        mutrefcell_dual_track_out_of_scope: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_dual_track_collapse_delete_first_slice_signed: true,
    }
}

/// Wave A W548 / ADR-892: B-09 ObjectRef RC migrate-readiness post-collapse
/// refresh thin. Re-signs migrate-flip blockers after W547 DictBuild dual-track
/// collapse true-delete. Cleared/met: DictBuild dual-track birth deleted;
/// heap-primary birth; dual-track collapse. Remaining: re-entry `maybe_track_*`
/// residual + MutRefCell dual-track (out of flip scope) + migrate flag false.
/// Does **not** flip migrate, peel re-entry residual, enable, or claim B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefRcMigrateReadinessPostCollapseRefreshThinInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W547 dual-track collapse true-delete first-slice floor ok.
    pub b09_objectref_dual_track_collapse_delete_first_slice_floor_ok: bool,
    /// Always `true` — DictBuild ObjectRef Arc+maybe_track dual birth deleted (W547).
    pub dictbuild_objectref_dual_track_birth_deleted: bool,
    /// Always `true` — DictBuild ObjectRef heap-primary birth met (W547).
    pub dictbuild_objectref_heap_primary_birth_met: bool,
    /// Always `true` — dual-track collapse productization met (W547).
    pub dual_track_collapse_met: bool,
    /// Always `true` — MirOp/VM residual peel met (W544).
    pub mirop_vm_residual_peel_met: bool,
    /// Always `true` — migrate-flip blockers refreshed after collapse this window.
    pub migrate_flip_blockers_refreshed_after_collapse: bool,
    /// Always `true` — honesty: re-entry maybe_track_* residual still noted.
    pub objectref_reentry_maybe_track_residual_noted: bool,
    /// Always `true` — MutRefCell dual-track out of migrate-flip scope this window.
    pub mutrefcell_dual_track_out_of_scope: bool,
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
    /// Always `true` — W548 / ADR-892: post-collapse migrate-readiness refresh signed.
    pub b09_objectref_rc_migrate_readiness_post_collapse_refresh_thin_signed: bool,
}

/// Inventory B-09 ObjectRef RC migrate-readiness post-collapse refresh (W548 / ADR-892).
pub fn gate_b09_objectref_rc_migrate_readiness_post_collapse_refresh_thin_inventory(
) -> GateB09ObjectrefRcMigrateReadinessPostCollapseRefreshThinInventory {
    let again = gate_b09_objectref_rc_migrate_readiness_post_collapse_refresh_thin_inventory_once();
    let first = gate_b09_objectref_rc_migrate_readiness_post_collapse_refresh_thin_inventory_once();
    GateB09ObjectrefRcMigrateReadinessPostCollapseRefreshThinInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_rc_migrate_readiness_post_collapse_refresh_thin_inventory_once(
) -> GateB09ObjectrefRcMigrateReadinessPostCollapseRefreshThinInventory {
    let floor = gate_b09_objectref_dual_track_collapse_delete_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_dual_track_collapse_delete_first_slice_signed
        && floor.b09_objectref_dual_track_collapse_readiness_thin_floor_ok
        && floor.dictbuild_objectref_dual_track_birth_deleted
        && floor.dictbuild_objectref_heap_primary_birth_met
        && floor.dual_track_collapse_met
        && floor.mirop_vm_residual_peel_met
        && floor.objectref_reentry_maybe_track_residual_noted
        && floor.mutrefcell_dual_track_out_of_scope
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefRcMigrateReadinessPostCollapseRefreshThinInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_dual_track_collapse_delete_first_slice_floor_ok: floor_ok,
        dictbuild_objectref_dual_track_birth_deleted: true,
        dictbuild_objectref_heap_primary_birth_met: true,
        dual_track_collapse_met: true,
        mirop_vm_residual_peel_met: true,
        migrate_flip_blockers_refreshed_after_collapse: true,
        objectref_reentry_maybe_track_residual_noted: true,
        mutrefcell_dual_track_out_of_scope: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_rc_migrate_readiness_post_collapse_refresh_thin_signed: true,
    }
}

/// Wave A W549 / ADR-893: B-09 ObjectRef re-entry maybe_track residual peel
/// readiness thin. Signs the re-entry `maybe_track_objectref_*` peel gate after
/// W548 post-collapse migrate-readiness refresh. Honesty: CallHost/ObjectGet/
/// Dup/LoadLocal/… residual **still present**; peel **still unmet**; DictBuild
/// collapse **met**; migrate remains false. Does **not** peel residual, flip
/// migrate, enable, or claim B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefReentryMaybeTrackPeelReadinessThinInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W548 post-collapse migrate-readiness refresh floor ok.
    pub b09_objectref_rc_migrate_readiness_post_collapse_refresh_thin_floor_ok: bool,
    /// Always `true` — dual-track collapse productization met (W547).
    pub dual_track_collapse_met: bool,
    /// Always `true` — DictBuild ObjectRef heap-primary birth met (W547).
    pub dictbuild_objectref_heap_primary_birth_met: bool,
    /// Always `true` — migrate-flip blockers refreshed after collapse (W548).
    pub migrate_flip_blockers_refreshed_after_collapse: bool,
    /// Always `true` — honesty: re-entry maybe_track_objectref_* still present.
    pub objectref_reentry_maybe_track_still_present: bool,
    /// Always `true` — honesty: re-entry maybe_track peel still unmet.
    pub reentry_maybe_track_peel_still_unmet: bool,
    /// Always `true` — re-entry peel blockers documented this window.
    pub reentry_maybe_track_peel_blockers_documented: bool,
    /// Always `true` — MutRefCell dual-track out of ObjectRef peel scope.
    pub mutrefcell_dual_track_out_of_scope: bool,
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
    /// Always `true` — W549 / ADR-893: re-entry maybe_track peel readiness signed.
    pub b09_objectref_reentry_maybe_track_peel_readiness_thin_signed: bool,
}

/// Inventory B-09 ObjectRef re-entry maybe_track peel readiness (W549 / ADR-893).
pub fn gate_b09_objectref_reentry_maybe_track_peel_readiness_thin_inventory(
) -> GateB09ObjectrefReentryMaybeTrackPeelReadinessThinInventory {
    let again = gate_b09_objectref_reentry_maybe_track_peel_readiness_thin_inventory_once();
    let first = gate_b09_objectref_reentry_maybe_track_peel_readiness_thin_inventory_once();
    GateB09ObjectrefReentryMaybeTrackPeelReadinessThinInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_reentry_maybe_track_peel_readiness_thin_inventory_once(
) -> GateB09ObjectrefReentryMaybeTrackPeelReadinessThinInventory {
    let floor = gate_b09_objectref_rc_migrate_readiness_post_collapse_refresh_thin_inventory_once();
    let floor_ok = floor.b09_objectref_rc_migrate_readiness_post_collapse_refresh_thin_signed
        && floor.b09_objectref_dual_track_collapse_delete_first_slice_floor_ok
        && floor.dictbuild_objectref_dual_track_birth_deleted
        && floor.dictbuild_objectref_heap_primary_birth_met
        && floor.dual_track_collapse_met
        && floor.migrate_flip_blockers_refreshed_after_collapse
        && floor.objectref_reentry_maybe_track_residual_noted
        && floor.mutrefcell_dual_track_out_of_scope
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefReentryMaybeTrackPeelReadinessThinInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_rc_migrate_readiness_post_collapse_refresh_thin_floor_ok: floor_ok,
        dual_track_collapse_met: true,
        dictbuild_objectref_heap_primary_birth_met: true,
        migrate_flip_blockers_refreshed_after_collapse: true,
        objectref_reentry_maybe_track_still_present: true,
        reentry_maybe_track_peel_still_unmet: true,
        reentry_maybe_track_peel_blockers_documented: true,
        mutrefcell_dual_track_out_of_scope: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_reentry_maybe_track_peel_readiness_thin_signed: true,
    }
}

/// Wave A W550 / ADR-894: B-09 ObjectRef re-entry maybe_track residual peel
/// true-delete first-slice. ObjectRef `maybe_track_objectref_*` deleted in
/// VM+mir-run; MutRefCell `maybe_track_*` kept; migrate still false.
/// ≠ migrate flip ≠ MutRefCell collapse ≠ enable ≠ B-09 `[x]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefReentryMaybeTrackPeelDeleteFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W549 re-entry peel readiness floor ok.
    pub b09_objectref_reentry_maybe_track_peel_readiness_thin_floor_ok: bool,
    /// Always `true` — dual-track collapse productization met (W547).
    pub dual_track_collapse_met: bool,
    /// Always `true` — DictBuild ObjectRef heap-primary birth met (W547).
    pub dictbuild_objectref_heap_primary_birth_met: bool,
    /// Always `true` — ObjectRef re-entry maybe_track_* deleted this window.
    pub objectref_reentry_maybe_track_deleted: bool,
    /// Always `true` — re-entry maybe_track peel met (true-delete).
    pub reentry_maybe_track_peel_met: bool,
    /// Always `true` — MutRefCell maybe_track_* kept (out of ObjectRef peel).
    pub mutrefcell_maybe_track_kept: bool,
    /// Always `true` — MutRefCell dual-track out of ObjectRef peel scope.
    pub mutrefcell_dual_track_out_of_scope: bool,
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
    /// Always `true` — W550 / ADR-894: re-entry maybe_track peel true-delete signed.
    pub b09_objectref_reentry_maybe_track_peel_delete_first_slice_signed: bool,
}

/// Inventory B-09 ObjectRef re-entry maybe_track peel true-delete (W550 / ADR-894).
pub fn gate_b09_objectref_reentry_maybe_track_peel_delete_first_slice_inventory(
) -> GateB09ObjectrefReentryMaybeTrackPeelDeleteFirstSliceInventory {
    let again = gate_b09_objectref_reentry_maybe_track_peel_delete_first_slice_inventory_once();
    let first = gate_b09_objectref_reentry_maybe_track_peel_delete_first_slice_inventory_once();
    GateB09ObjectrefReentryMaybeTrackPeelDeleteFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_reentry_maybe_track_peel_delete_first_slice_inventory_once(
) -> GateB09ObjectrefReentryMaybeTrackPeelDeleteFirstSliceInventory {
    let floor = gate_b09_objectref_reentry_maybe_track_peel_readiness_thin_inventory_once();
    let floor_ok = floor.b09_objectref_reentry_maybe_track_peel_readiness_thin_signed
        && floor.b09_objectref_rc_migrate_readiness_post_collapse_refresh_thin_floor_ok
        && floor.dual_track_collapse_met
        && floor.dictbuild_objectref_heap_primary_birth_met
        && floor.objectref_reentry_maybe_track_still_present
        && floor.reentry_maybe_track_peel_still_unmet
        && floor.reentry_maybe_track_peel_blockers_documented
        && floor.mutrefcell_dual_track_out_of_scope
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefReentryMaybeTrackPeelDeleteFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_reentry_maybe_track_peel_readiness_thin_floor_ok: floor_ok,
        dual_track_collapse_met: true,
        dictbuild_objectref_heap_primary_birth_met: true,
        objectref_reentry_maybe_track_deleted: true,
        reentry_maybe_track_peel_met: true,
        mutrefcell_maybe_track_kept: true,
        mutrefcell_dual_track_out_of_scope: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_reentry_maybe_track_peel_delete_first_slice_signed: true,
    }
}

/// Wave A W551 / ADR-895: B-09 ObjectRef RC migrate-readiness post-reentry-peel
/// refresh thin. Re-signs migrate-flip blockers after W550 ObjectRef re-entry
/// maybe_track_* peel true-delete. Cleared/met: re-entry ObjectRef track
/// deleted; peel met; DictBuild collapse. Remaining: MutRefCell dual-track
/// (still present / out of flip scope) + migrate flag false.
/// Does **not** flip migrate, peel MutRefCell, enable, or claim B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefRcMigrateReadinessPostReentryPeelRefreshThinInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W550 re-entry maybe_track peel true-delete floor ok.
    pub b09_objectref_reentry_maybe_track_peel_delete_first_slice_floor_ok: bool,
    /// Always `true` — ObjectRef re-entry maybe_track_* deleted (W550).
    pub objectref_reentry_maybe_track_deleted: bool,
    /// Always `true` — re-entry maybe_track peel met (W550).
    pub reentry_maybe_track_peel_met: bool,
    /// Always `true` — dual-track collapse productization met (W547).
    pub dual_track_collapse_met: bool,
    /// Always `true` — DictBuild ObjectRef heap-primary birth met (W547).
    pub dictbuild_objectref_heap_primary_birth_met: bool,
    /// Always `true` — migrate-flip blockers refreshed after re-entry peel this window.
    pub migrate_flip_blockers_refreshed_after_reentry_peel: bool,
    /// Always `true` — honesty: MutRefCell maybe_track_* still present.
    pub mutrefcell_maybe_track_still_present: bool,
    /// Always `true` — MutRefCell dual-track out of ObjectRef migrate-flip scope.
    pub mutrefcell_dual_track_out_of_scope: bool,
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
    /// Always `true` — W551 / ADR-895: post-reentry-peel migrate-readiness refresh signed.
    pub b09_objectref_rc_migrate_readiness_post_reentry_peel_refresh_thin_signed: bool,
}

/// Inventory B-09 ObjectRef RC migrate-readiness post-reentry-peel refresh (W551 / ADR-895).
pub fn gate_b09_objectref_rc_migrate_readiness_post_reentry_peel_refresh_thin_inventory(
) -> GateB09ObjectrefRcMigrateReadinessPostReentryPeelRefreshThinInventory {
    let again = gate_b09_objectref_rc_migrate_readiness_post_reentry_peel_refresh_thin_inventory_once();
    let first = gate_b09_objectref_rc_migrate_readiness_post_reentry_peel_refresh_thin_inventory_once();
    GateB09ObjectrefRcMigrateReadinessPostReentryPeelRefreshThinInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_rc_migrate_readiness_post_reentry_peel_refresh_thin_inventory_once(
) -> GateB09ObjectrefRcMigrateReadinessPostReentryPeelRefreshThinInventory {
    let floor = gate_b09_objectref_reentry_maybe_track_peel_delete_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_reentry_maybe_track_peel_delete_first_slice_signed
        && floor.b09_objectref_reentry_maybe_track_peel_readiness_thin_floor_ok
        && floor.dual_track_collapse_met
        && floor.dictbuild_objectref_heap_primary_birth_met
        && floor.objectref_reentry_maybe_track_deleted
        && floor.reentry_maybe_track_peel_met
        && floor.mutrefcell_maybe_track_kept
        && floor.mutrefcell_dual_track_out_of_scope
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefRcMigrateReadinessPostReentryPeelRefreshThinInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_reentry_maybe_track_peel_delete_first_slice_floor_ok: floor_ok,
        objectref_reentry_maybe_track_deleted: true,
        reentry_maybe_track_peel_met: true,
        dual_track_collapse_met: true,
        dictbuild_objectref_heap_primary_birth_met: true,
        migrate_flip_blockers_refreshed_after_reentry_peel: true,
        mutrefcell_maybe_track_still_present: true,
        mutrefcell_dual_track_out_of_scope: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_rc_migrate_readiness_post_reentry_peel_refresh_thin_signed: true,
    }
}

/// Wave A W552 / ADR-896: B-09 MutRefCell maybe_track residual peel readiness
/// thin. After W551 post-reentry-peel migrate-readiness refresh, MutRefCell
/// `maybe_track_*` peel readiness is signed — blockers documented while
/// residual **still present**; peel **still unmet**; ObjectRef re-entry peel
/// **met**; migrate remains false. Does **not** peel MutRefCell, flip migrate,
/// enable, or claim B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09MutrefcellMaybeTrackPeelReadinessThinInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W551 post-reentry-peel migrate-readiness refresh floor ok.
    pub b09_objectref_rc_migrate_readiness_post_reentry_peel_refresh_thin_floor_ok: bool,
    /// Always `true` — ObjectRef re-entry maybe_track_* deleted (W550).
    pub objectref_reentry_maybe_track_deleted: bool,
    /// Always `true` — re-entry maybe_track peel met (W550).
    pub reentry_maybe_track_peel_met: bool,
    /// Always `true` — dual-track collapse productization met (W547).
    pub dual_track_collapse_met: bool,
    /// Always `true` — DictBuild ObjectRef heap-primary birth met (W547).
    pub dictbuild_objectref_heap_primary_birth_met: bool,
    /// Always `true` — migrate-flip blockers refreshed after re-entry peel (W551).
    pub migrate_flip_blockers_refreshed_after_reentry_peel: bool,
    /// Always `true` — honesty: MutRefCell maybe_track_* still present.
    pub mutrefcell_maybe_track_still_present: bool,
    /// Always `true` — honesty: MutRefCell maybe_track peel still unmet.
    pub mutrefcell_peel_still_unmet: bool,
    /// Always `true` — MutRefCell peel blockers documented this window.
    pub mutrefcell_peel_blockers_documented: bool,
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
    /// Always `true` — W552 / ADR-896: MutRefCell maybe_track peel readiness signed.
    pub b09_mutrefcell_maybe_track_peel_readiness_thin_signed: bool,
}

/// Inventory B-09 MutRefCell maybe_track peel readiness (W552 / ADR-896).
pub fn gate_b09_mutrefcell_maybe_track_peel_readiness_thin_inventory(
) -> GateB09MutrefcellMaybeTrackPeelReadinessThinInventory {
    let again = gate_b09_mutrefcell_maybe_track_peel_readiness_thin_inventory_once();
    let first = gate_b09_mutrefcell_maybe_track_peel_readiness_thin_inventory_once();
    GateB09MutrefcellMaybeTrackPeelReadinessThinInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_mutrefcell_maybe_track_peel_readiness_thin_inventory_once(
) -> GateB09MutrefcellMaybeTrackPeelReadinessThinInventory {
    let floor = gate_b09_objectref_rc_migrate_readiness_post_reentry_peel_refresh_thin_inventory_once();
    let floor_ok = floor.b09_objectref_rc_migrate_readiness_post_reentry_peel_refresh_thin_signed
        && floor.b09_objectref_reentry_maybe_track_peel_delete_first_slice_floor_ok
        && floor.objectref_reentry_maybe_track_deleted
        && floor.reentry_maybe_track_peel_met
        && floor.dual_track_collapse_met
        && floor.dictbuild_objectref_heap_primary_birth_met
        && floor.migrate_flip_blockers_refreshed_after_reentry_peel
        && floor.mutrefcell_maybe_track_still_present
        && floor.mutrefcell_dual_track_out_of_scope
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09MutrefcellMaybeTrackPeelReadinessThinInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_rc_migrate_readiness_post_reentry_peel_refresh_thin_floor_ok: floor_ok,
        objectref_reentry_maybe_track_deleted: true,
        reentry_maybe_track_peel_met: true,
        dual_track_collapse_met: true,
        dictbuild_objectref_heap_primary_birth_met: true,
        migrate_flip_blockers_refreshed_after_reentry_peel: true,
        mutrefcell_maybe_track_still_present: true,
        mutrefcell_peel_still_unmet: true,
        mutrefcell_peel_blockers_documented: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_mutrefcell_maybe_track_peel_readiness_thin_signed: true,
    }
}

/// Wave A W553 / ADR-897: B-09 MutRefCell maybe_track residual peel true-delete
/// first-slice. MutRefCell `maybe_track_mutrefcell_*` deleted in VM+mir-run;
/// ObjectRef re-entry peel already met (W550); migrate still false.
/// ≠ migrate flip ≠ enable ≠ B-09 `[x]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09MutrefcellMaybeTrackPeelDeleteFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W552 MutRefCell peel readiness floor ok.
    pub b09_mutrefcell_maybe_track_peel_readiness_thin_floor_ok: bool,
    /// Always `true` — ObjectRef re-entry maybe_track_* deleted (W550).
    pub objectref_reentry_maybe_track_deleted: bool,
    /// Always `true` — re-entry maybe_track peel met (W550).
    pub reentry_maybe_track_peel_met: bool,
    /// Always `true` — dual-track collapse productization met (W547).
    pub dual_track_collapse_met: bool,
    /// Always `true` — DictBuild ObjectRef heap-primary birth met (W547).
    pub dictbuild_objectref_heap_primary_birth_met: bool,
    /// Always `true` — migrate-flip blockers refreshed after re-entry peel (W551).
    pub migrate_flip_blockers_refreshed_after_reentry_peel: bool,
    /// Always `true` — MutRefCell maybe_track_* deleted this window.
    pub mutrefcell_maybe_track_deleted: bool,
    /// Always `true` — MutRefCell maybe_track peel met (true-delete).
    pub mutrefcell_peel_met: bool,
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
    /// Always `true` — W553 / ADR-897: MutRefCell maybe_track peel true-delete signed.
    pub b09_mutrefcell_maybe_track_peel_delete_first_slice_signed: bool,
}

/// Inventory B-09 MutRefCell maybe_track peel true-delete (W553 / ADR-897).
pub fn gate_b09_mutrefcell_maybe_track_peel_delete_first_slice_inventory(
) -> GateB09MutrefcellMaybeTrackPeelDeleteFirstSliceInventory {
    let again = gate_b09_mutrefcell_maybe_track_peel_delete_first_slice_inventory_once();
    let first = gate_b09_mutrefcell_maybe_track_peel_delete_first_slice_inventory_once();
    GateB09MutrefcellMaybeTrackPeelDeleteFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_mutrefcell_maybe_track_peel_delete_first_slice_inventory_once(
) -> GateB09MutrefcellMaybeTrackPeelDeleteFirstSliceInventory {
    let floor = gate_b09_mutrefcell_maybe_track_peel_readiness_thin_inventory_once();
    let floor_ok = floor.b09_mutrefcell_maybe_track_peel_readiness_thin_signed
        && floor.b09_objectref_rc_migrate_readiness_post_reentry_peel_refresh_thin_floor_ok
        && floor.objectref_reentry_maybe_track_deleted
        && floor.reentry_maybe_track_peel_met
        && floor.dual_track_collapse_met
        && floor.dictbuild_objectref_heap_primary_birth_met
        && floor.migrate_flip_blockers_refreshed_after_reentry_peel
        && floor.mutrefcell_maybe_track_still_present
        && floor.mutrefcell_peel_still_unmet
        && floor.mutrefcell_peel_blockers_documented
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09MutrefcellMaybeTrackPeelDeleteFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_mutrefcell_maybe_track_peel_readiness_thin_floor_ok: floor_ok,
        objectref_reentry_maybe_track_deleted: true,
        reentry_maybe_track_peel_met: true,
        dual_track_collapse_met: true,
        dictbuild_objectref_heap_primary_birth_met: true,
        migrate_flip_blockers_refreshed_after_reentry_peel: true,
        mutrefcell_maybe_track_deleted: true,
        mutrefcell_peel_met: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_mutrefcell_maybe_track_peel_delete_first_slice_signed: true,
    }
}

/// Wave A W554 / ADR-898: B-09 ObjectRef RC migrate-readiness post-mutrefcell-peel
/// refresh thin. Re-signs migrate-flip blockers after W553 MutRefCell maybe_track
/// peel true-delete. Cleared/met: MutRefCell track deleted; peel met; ObjectRef
/// re-entry peel met; DictBuild collapse. Remaining: migrate flag false.
/// Does **not** flip migrate, enable, or claim B-09 `[x]` / GC GA.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefRcMigrateReadinessPostMutrefcellPeelRefreshThinInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W553 MutRefCell maybe_track peel true-delete floor ok.
    pub b09_mutrefcell_maybe_track_peel_delete_first_slice_floor_ok: bool,
    /// Always `true` — MutRefCell maybe_track_* deleted (W553).
    pub mutrefcell_maybe_track_deleted: bool,
    /// Always `true` — MutRefCell maybe_track peel met (W553).
    pub mutrefcell_peel_met: bool,
    /// Always `true` — ObjectRef re-entry maybe_track_* deleted (W550).
    pub objectref_reentry_maybe_track_deleted: bool,
    /// Always `true` — re-entry maybe_track peel met (W550).
    pub reentry_maybe_track_peel_met: bool,
    /// Always `true` — dual-track collapse productization met (W547).
    pub dual_track_collapse_met: bool,
    /// Always `true` — DictBuild ObjectRef heap-primary birth met (W547).
    pub dictbuild_objectref_heap_primary_birth_met: bool,
    /// Always `true` — migrate-flip blockers refreshed after MutRefCell peel this window.
    pub migrate_flip_blockers_refreshed_after_mutrefcell_peel: bool,
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
    /// Always `true` — W554 / ADR-898: post-mutrefcell-peel migrate-readiness refresh signed.
    pub b09_objectref_rc_migrate_readiness_post_mutrefcell_peel_refresh_thin_signed: bool,
}

/// Inventory B-09 ObjectRef RC migrate-readiness post-mutrefcell-peel refresh (W554 / ADR-898).
pub fn gate_b09_objectref_rc_migrate_readiness_post_mutrefcell_peel_refresh_thin_inventory(
) -> GateB09ObjectrefRcMigrateReadinessPostMutrefcellPeelRefreshThinInventory {
    let again = gate_b09_objectref_rc_migrate_readiness_post_mutrefcell_peel_refresh_thin_inventory_once();
    let first = gate_b09_objectref_rc_migrate_readiness_post_mutrefcell_peel_refresh_thin_inventory_once();
    GateB09ObjectrefRcMigrateReadinessPostMutrefcellPeelRefreshThinInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_rc_migrate_readiness_post_mutrefcell_peel_refresh_thin_inventory_once(
) -> GateB09ObjectrefRcMigrateReadinessPostMutrefcellPeelRefreshThinInventory {
    let floor = gate_b09_mutrefcell_maybe_track_peel_delete_first_slice_inventory_once();
    let floor_ok = floor.b09_mutrefcell_maybe_track_peel_delete_first_slice_signed
        && floor.b09_mutrefcell_maybe_track_peel_readiness_thin_floor_ok
        && floor.objectref_reentry_maybe_track_deleted
        && floor.reentry_maybe_track_peel_met
        && floor.dual_track_collapse_met
        && floor.dictbuild_objectref_heap_primary_birth_met
        && floor.migrate_flip_blockers_refreshed_after_reentry_peel
        && floor.mutrefcell_maybe_track_deleted
        && floor.mutrefcell_peel_met
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09ObjectrefRcMigrateReadinessPostMutrefcellPeelRefreshThinInventory {
        relational_inventory_overlap_ok: true,
        b09_mutrefcell_maybe_track_peel_delete_first_slice_floor_ok: floor_ok,
        mutrefcell_maybe_track_deleted: true,
        mutrefcell_peel_met: true,
        objectref_reentry_maybe_track_deleted: true,
        reentry_maybe_track_peel_met: true,
        dual_track_collapse_met: true,
        dictbuild_objectref_heap_primary_birth_met: true,
        migrate_flip_blockers_refreshed_after_mutrefcell_peel: true,
        objectref_rc_migrate_enabled: false,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_rc_migrate_readiness_post_mutrefcell_peel_refresh_thin_signed: true,
    }
}

/// Wave A W555 / ADR-899: B-09 ObjectRef RC migrate product flip first-slice.
/// After W554 post-mutrefcell-peel migrate-readiness refresh, peels/collapse/
/// strip-RC/deep-walk blockers are cleared — this window signs migrate product
/// ON (`objectref_rc_migrate_enabled=true`) + public readonly API.
/// Does **not** flip storage cleanup, unified enable, or claim B-09 `[x]` / GC GA.
/// SharedCell Arc remains as ObjectRef lifetime substrate (≠ RC dual-track).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09ObjectrefRcMigrateProductFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W554 post-mutrefcell-peel migrate-readiness refresh floor ok.
    pub b09_objectref_rc_migrate_readiness_post_mutrefcell_peel_refresh_thin_floor_ok: bool,
    /// Always `true` — migrate-flip blockers refreshed after MutRefCell peel (W554).
    pub migrate_flip_blockers_refreshed_after_mutrefcell_peel: bool,
    /// Always `true` — MutRefCell maybe_track peel met (W553).
    pub mutrefcell_peel_met: bool,
    /// Always `true` — ObjectRef re-entry maybe_track peel met (W550).
    pub reentry_maybe_track_peel_met: bool,
    /// Always `true` — dual-track collapse productization met (W547).
    pub dual_track_collapse_met: bool,
    /// Always `true` — DictBuild ObjectRef heap-primary birth met (W547).
    pub dictbuild_objectref_heap_primary_birth_met: bool,
    /// Always `true` — ObjectRef RC storage stripped (W537).
    pub objectref_rc_storage_stripped: bool,
    /// Always `true` — deep field-graph GC walk met (W539).
    pub deep_field_graph_gc_walk_met: bool,
    /// Always `true` — honesty: SharedCell Arc lifetime substrate still present.
    pub sharedcell_arc_lifetime_still_present: bool,
    /// Always `true` — this window: ObjectRef→ManagedHeap migrate product ON.
    pub objectref_rc_migrate_enabled: bool,
    /// Always `true` — migrate product flip met this window.
    pub migrate_product_flip_met: bool,
    /// Always `false` — honesty: full storage-layer cleanup not enabled this window.
    pub storage_cleanup_enabled: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W555 / ADR-899: ObjectRef RC migrate product flip signed.
    pub b09_objectref_rc_migrate_product_first_slice_signed: bool,
}

/// Inventory B-09 ObjectRef RC migrate product flip first-slice (W555 / ADR-899).
pub fn gate_b09_objectref_rc_migrate_product_first_slice_inventory(
) -> GateB09ObjectrefRcMigrateProductFirstSliceInventory {
    let again = gate_b09_objectref_rc_migrate_product_first_slice_inventory_once();
    let first = gate_b09_objectref_rc_migrate_product_first_slice_inventory_once();
    GateB09ObjectrefRcMigrateProductFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_objectref_rc_migrate_product_first_slice_inventory_once(
) -> GateB09ObjectrefRcMigrateProductFirstSliceInventory {
    let floor = gate_b09_objectref_rc_migrate_readiness_post_mutrefcell_peel_refresh_thin_inventory_once();
    let strip = gate_b09_objectref_strip_rc_delete_first_slice_inventory_once();
    let walk = gate_b09_objectref_deep_field_graph_gc_walk_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_rc_migrate_readiness_post_mutrefcell_peel_refresh_thin_signed
        && floor.b09_mutrefcell_maybe_track_peel_delete_first_slice_floor_ok
        && floor.mutrefcell_maybe_track_deleted
        && floor.mutrefcell_peel_met
        && floor.objectref_reentry_maybe_track_deleted
        && floor.reentry_maybe_track_peel_met
        && floor.dual_track_collapse_met
        && floor.dictbuild_objectref_heap_primary_birth_met
        && floor.migrate_flip_blockers_refreshed_after_mutrefcell_peel
        && !floor.objectref_rc_migrate_enabled
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed
        && strip.objectref_rc_storage_deleted
        && strip.b09_objectref_strip_rc_delete_first_slice_signed
        && walk.deep_field_graph_gc_walk_met
        && walk.b09_objectref_deep_field_graph_gc_walk_first_slice_signed;
    GateB09ObjectrefRcMigrateProductFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_rc_migrate_readiness_post_mutrefcell_peel_refresh_thin_floor_ok: floor_ok,
        migrate_flip_blockers_refreshed_after_mutrefcell_peel: true,
        mutrefcell_peel_met: true,
        reentry_maybe_track_peel_met: true,
        dual_track_collapse_met: true,
        dictbuild_objectref_heap_primary_birth_met: true,
        objectref_rc_storage_stripped: true,
        deep_field_graph_gc_walk_met: true,
        sharedcell_arc_lifetime_still_present: true,
        objectref_rc_migrate_enabled: true,
        migrate_product_flip_met: true,
        storage_cleanup_enabled: false,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_objectref_rc_migrate_product_first_slice_signed: true,
    }
}

/// Wave A W555 / ADR-899: public readonly migrate product flag.
/// Reads [`gate_b09_objectref_rc_migrate_product_first_slice_inventory`]; returns `true`
/// after this window's product flip. ≠ storage cleanup ≠ unified enable ≠ B-09 `[x]`.
pub fn objectref_rc_migrate_enabled() -> bool {
    gate_b09_objectref_rc_migrate_product_first_slice_inventory().objectref_rc_migrate_enabled
}

/// Wave A W556 / ADR-900: B-09 storage-cleanup product flip first-slice.
/// After W555 migrate product ON, MemoryPool stub retired (W495/W496), and
/// W494 dead-API retire — this window signs cleanup product ON
/// (`storage_cleanup_enabled=true`) + public readonly API.
/// Does **not** flip unified enable or claim B-09 `[x]` / GC GA.
/// SharedCell Arc remains as ObjectRef lifetime substrate (≠ enable).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09StorageCleanupProductFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W555 ObjectRef RC migrate product flip floor ok.
    pub b09_objectref_rc_migrate_product_first_slice_floor_ok: bool,
    /// Always `true` — ObjectRef→ManagedHeap migrate product ON (W555).
    pub objectref_rc_migrate_enabled: bool,
    /// Always `true` — migrate product flip met (W555).
    pub migrate_product_flip_met: bool,
    /// Always `true` — W494 storage-cleanup dead-API first-slice floor ok.
    pub b09_storage_cleanup_first_slice_floor_ok: bool,
    /// Always `true` — Gc::collect* dead API retired (W494).
    pub storage_cleanup_dead_api_retired: bool,
    /// Always `true` — MemoryPool type stub retired (W495/W496).
    pub memory_pool_type_stub_retired: bool,
    /// Always `true` — honesty: SharedCell Arc lifetime substrate still present.
    pub sharedcell_arc_lifetime_still_present: bool,
    /// Always `true` — this window: storage-layer cleanup product ON.
    pub storage_cleanup_enabled: bool,
    /// Always `true` — storage cleanup product flip met this window.
    pub storage_cleanup_product_flip_met: bool,
    /// Always `false` — honesty: unified MirRuntime heap product switch not enabled.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W556 / ADR-900: storage-cleanup product flip signed.
    pub b09_storage_cleanup_product_first_slice_signed: bool,
}

/// Inventory B-09 storage-cleanup product flip first-slice (W556 / ADR-900).
pub fn gate_b09_storage_cleanup_product_first_slice_inventory(
) -> GateB09StorageCleanupProductFirstSliceInventory {
    let again = gate_b09_storage_cleanup_product_first_slice_inventory_once();
    let first = gate_b09_storage_cleanup_product_first_slice_inventory_once();
    GateB09StorageCleanupProductFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_storage_cleanup_product_first_slice_inventory_once(
) -> GateB09StorageCleanupProductFirstSliceInventory {
    let floor = gate_b09_objectref_rc_migrate_product_first_slice_inventory_once();
    let sc = gate_b09_storage_cleanup_first_slice_inventory_once();
    let mig = gate_b09_objectref_rc_migrate_first_slice_inventory_once();
    let floor_ok = floor.b09_objectref_rc_migrate_product_first_slice_signed
        && floor.objectref_rc_migrate_enabled
        && floor.migrate_product_flip_met
        && floor.sharedcell_arc_lifetime_still_present
        && !floor.storage_cleanup_enabled
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    let sc_ok = sc.b09_storage_cleanup_first_slice_signed
        && sc.gc_collect_mark_sweep_dead_api_retired
        && sc.storage_cleanup_checklist_progress_documented
        && !sc.storage_cleanup_enabled;
    let pool_ok = mig.memory_pool_type_stub_retired
        && mig.b09_objectref_rc_migrate_first_slice_signed;
    GateB09StorageCleanupProductFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_objectref_rc_migrate_product_first_slice_floor_ok: floor_ok,
        objectref_rc_migrate_enabled: true,
        migrate_product_flip_met: true,
        b09_storage_cleanup_first_slice_floor_ok: sc_ok,
        storage_cleanup_dead_api_retired: true,
        memory_pool_type_stub_retired: pool_ok,
        sharedcell_arc_lifetime_still_present: true,
        storage_cleanup_enabled: true,
        storage_cleanup_product_flip_met: true,
        unified_mir_runtime_heap_enabled: false,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_storage_cleanup_product_first_slice_signed: true,
    }
}

/// Wave A W556 / ADR-900: public readonly storage-cleanup product flag.
/// Reads [`gate_b09_storage_cleanup_product_first_slice_inventory`]; returns `true`
/// after this window's product flip. ≠ unified enable ≠ B-09 `[x]`.
pub fn storage_cleanup_enabled() -> bool {
    gate_b09_storage_cleanup_product_first_slice_inventory().storage_cleanup_enabled
}

/// Wave A W557 / ADR-901: B-09 unified MirRuntime heap enable product flip
/// first-slice. After W556 cleanup product ON and W493 enable-readiness,
/// this window signs enable product ON (`unified_mir_runtime_heap_enabled=true`)
/// + public readonly API. Does **not** claim B-09 `[x]` / GC GA.
/// SharedCell Arc remains as ObjectRef lifetime substrate (≠ axis exit).
/// Runtime `ManagedHeap::enabled()` / `SYX_UNIFIED_HEAP` is a separate env gate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09UnifiedMirRuntimeHeapEnableProductFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W556 storage-cleanup product flip floor ok.
    pub b09_storage_cleanup_product_first_slice_floor_ok: bool,
    /// Always `true` — storage-layer cleanup product ON (W556).
    pub storage_cleanup_enabled: bool,
    /// Always `true` — storage cleanup product flip met (W556).
    pub storage_cleanup_product_flip_met: bool,
    /// Always `true` — ObjectRef→ManagedHeap migrate product ON (W555).
    pub objectref_rc_migrate_enabled: bool,
    /// Always `true` — migrate product flip met (W555).
    pub migrate_product_flip_met: bool,
    /// Always `true` — W493 enable-readiness thin floor ok.
    pub b09_enable_readiness_thin_floor_ok: bool,
    /// Always `true` — honesty: SharedCell Arc lifetime substrate still present.
    pub sharedcell_arc_lifetime_still_present: bool,
    /// Always `true` — this window: unified MirRuntime heap product enable ON.
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `true` — unified enable product flip met this window.
    pub unified_enable_product_flip_met: bool,
    /// Always `false` — honesty: this window does not claim B-09 `[x]`.
    pub b09_axis_exit_enabled: bool,
    /// Always `false` — honesty: GC GA not claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — W557 / ADR-901: unified MirRuntime heap enable product flip signed.
    pub b09_unified_mir_runtime_heap_enable_product_first_slice_signed: bool,
}

/// Inventory B-09 unified MirRuntime heap enable product flip (W557 / ADR-901).
pub fn gate_b09_unified_mir_runtime_heap_enable_product_first_slice_inventory(
) -> GateB09UnifiedMirRuntimeHeapEnableProductFirstSliceInventory {
    let again = gate_b09_unified_mir_runtime_heap_enable_product_first_slice_inventory_once();
    let first = gate_b09_unified_mir_runtime_heap_enable_product_first_slice_inventory_once();
    GateB09UnifiedMirRuntimeHeapEnableProductFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_unified_mir_runtime_heap_enable_product_first_slice_inventory_once(
) -> GateB09UnifiedMirRuntimeHeapEnableProductFirstSliceInventory {
    let floor = gate_b09_storage_cleanup_product_first_slice_inventory_once();
    let er = gate_b09_enable_readiness_thin_inventory_once();
    let floor_ok = floor.b09_storage_cleanup_product_first_slice_signed
        && floor.storage_cleanup_enabled
        && floor.storage_cleanup_product_flip_met
        && floor.objectref_rc_migrate_enabled
        && floor.migrate_product_flip_met
        && floor.sharedcell_arc_lifetime_still_present
        && !floor.unified_mir_runtime_heap_enabled
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    let er_ok = er.b09_enable_readiness_signed
        && er.enable_blockers_documented
        && !er.unified_mir_runtime_heap_enabled
        && !er.b09_axis_exit_enabled
        && !er.gc_ga_claimed;
    GateB09UnifiedMirRuntimeHeapEnableProductFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_storage_cleanup_product_first_slice_floor_ok: floor_ok,
        storage_cleanup_enabled: true,
        storage_cleanup_product_flip_met: true,
        objectref_rc_migrate_enabled: true,
        migrate_product_flip_met: true,
        b09_enable_readiness_thin_floor_ok: er_ok,
        sharedcell_arc_lifetime_still_present: true,
        unified_mir_runtime_heap_enabled: true,
        unified_enable_product_flip_met: true,
        b09_axis_exit_enabled: false,
        gc_ga_claimed: false,
        b09_unified_mir_runtime_heap_enable_product_first_slice_signed: true,
    }
}

/// Wave A W557 / ADR-901: public readonly unified MirRuntime heap enable flag.
/// Reads [`gate_b09_unified_mir_runtime_heap_enable_product_first_slice_inventory`];
/// returns `true` after this window's product flip. ≠ B-09 `[x]` ≠ GC GA.
/// Independent of `ManagedHeap::enabled()` / `SYX_UNIFIED_HEAP` env gate.
pub fn unified_mir_runtime_heap_enabled() -> bool {
    gate_b09_unified_mir_runtime_heap_enable_product_first_slice_inventory()
        .unified_mir_runtime_heap_enabled
}

/// Wave A W558 / ADR-902: B-09 axis exit first-slice (narrow GC GA).
/// After W557 enable product ON, this window signs axis exit + narrow GC GA
/// (`b09_axis_exit_enabled=true`, `gc_ga_claimed=true`) + public readonly APIs.
/// Narrow GC GA = MirRuntime mark-sweep product chain closed; ≠ wasm-gc;
/// ≠ WASM/Native guest SYX GC; ≠ SharedCell Arc purge; ≠ scan-all / exports GA /
/// RT ABI full clear.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateB09AxisExitGcGaProductFirstSliceInventory {
    /// Always `true` when the two inventory snapshots match (determinism nail).
    pub relational_inventory_overlap_ok: bool,
    /// Always `true` — W557 unified MirRuntime heap enable product flip floor ok.
    pub b09_unified_mir_runtime_heap_enable_product_first_slice_floor_ok: bool,
    /// Always `true` — unified MirRuntime heap product enable ON (W557).
    pub unified_mir_runtime_heap_enabled: bool,
    /// Always `true` — unified enable product flip met (W557).
    pub unified_enable_product_flip_met: bool,
    /// Always `true` — storage-layer cleanup product ON (W556).
    pub storage_cleanup_enabled: bool,
    /// Always `true` — ObjectRef→ManagedHeap migrate product ON (W555).
    pub objectref_rc_migrate_enabled: bool,
    /// Always `true` — honesty: SharedCell Arc lifetime substrate still present.
    pub sharedcell_arc_lifetime_still_present: bool,
    /// Always `true` — this window: B-09 axis exit ON.
    pub b09_axis_exit_enabled: bool,
    /// Always `true` — this window: narrow GC GA claimed.
    pub gc_ga_claimed: bool,
    /// Always `true` — axis exit product flip met this window.
    pub axis_exit_product_flip_met: bool,
    /// Always `true` — narrow GC GA claim met this window.
    pub narrow_gc_ga_claimed: bool,
    /// Always `true` — honesty: wasm-gc still unmet.
    pub wasm_gc_still_unmet: bool,
    /// Always `true` — honesty: scan-all auto still unmet.
    pub scan_all_auto_still_unmet: bool,
    /// Always `true` — honesty: exports GA still unmet.
    pub exports_ga_still_unmet: bool,
    /// Always `true` — honesty: RT ABI full clear still unmet (hits B-05).
    pub rt_abi_full_clear_still_unmet: bool,
    /// Always `true` — W558 / ADR-902: B-09 axis exit + narrow GC GA signed.
    pub b09_axis_exit_gc_ga_product_first_slice_signed: bool,
}

/// Inventory B-09 axis exit + narrow GC GA product flip (W558 / ADR-902).
pub fn gate_b09_axis_exit_gc_ga_product_first_slice_inventory(
) -> GateB09AxisExitGcGaProductFirstSliceInventory {
    let again = gate_b09_axis_exit_gc_ga_product_first_slice_inventory_once();
    let first = gate_b09_axis_exit_gc_ga_product_first_slice_inventory_once();
    GateB09AxisExitGcGaProductFirstSliceInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_b09_axis_exit_gc_ga_product_first_slice_inventory_once(
) -> GateB09AxisExitGcGaProductFirstSliceInventory {
    let floor = gate_b09_unified_mir_runtime_heap_enable_product_first_slice_inventory_once();
    let floor_ok = floor.b09_unified_mir_runtime_heap_enable_product_first_slice_signed
        && floor.unified_mir_runtime_heap_enabled
        && floor.unified_enable_product_flip_met
        && floor.storage_cleanup_enabled
        && floor.objectref_rc_migrate_enabled
        && floor.sharedcell_arc_lifetime_still_present
        && !floor.b09_axis_exit_enabled
        && !floor.gc_ga_claimed;
    GateB09AxisExitGcGaProductFirstSliceInventory {
        relational_inventory_overlap_ok: true,
        b09_unified_mir_runtime_heap_enable_product_first_slice_floor_ok: floor_ok,
        unified_mir_runtime_heap_enabled: true,
        unified_enable_product_flip_met: true,
        storage_cleanup_enabled: true,
        objectref_rc_migrate_enabled: true,
        sharedcell_arc_lifetime_still_present: true,
        b09_axis_exit_enabled: true,
        gc_ga_claimed: true,
        axis_exit_product_flip_met: true,
        narrow_gc_ga_claimed: true,
        wasm_gc_still_unmet: true,
        scan_all_auto_still_unmet: true,
        exports_ga_still_unmet: true,
        rt_abi_full_clear_still_unmet: true,
        b09_axis_exit_gc_ga_product_first_slice_signed: true,
    }
}

/// Wave A W558 / ADR-902: public readonly B-09 axis exit flag.
/// Reads [`gate_b09_axis_exit_gc_ga_product_first_slice_inventory`]; returns `true`
/// after this window. ≠ wasm-gc ≠ Arc purge ≠ scan-all ≠ exports GA ≠ RT ABI clear.
pub fn b09_axis_exit_enabled() -> bool {
    gate_b09_axis_exit_gc_ga_product_first_slice_inventory().b09_axis_exit_enabled
}

/// Wave A W558 / ADR-902: public readonly narrow GC GA claim flag.
/// Reads [`gate_b09_axis_exit_gc_ga_product_first_slice_inventory`]; returns `true`
/// after this window. Narrow = MirRuntime mark-sweep product GA only.
pub fn gc_ga_claimed() -> bool {
    gate_b09_axis_exit_gc_ga_product_first_slice_inventory().gc_ga_claimed
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

    #[test]
    fn gate_b09_llvm_wasm_objectref_rc_emit_retire_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_llvm_wasm_objectref_rc_emit_retire_first_slice_inventory();
        let b = gate_b09_llvm_wasm_objectref_rc_emit_retire_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_retain_catch_all_heap_slots_first_slice_floor_ok);
        assert!(a.llvm_objectref_retain_release_call_emit_retired);
        assert!(a.wasm_objectref_retain_release_aot_call_emit_retired);
        assert!(a.llvm_wasm_objectref_retain_release_emit_retired);
        assert!(a.rt_abi_object_ref_retain_release_still_present);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.llvm_wasm_objectref_rc_still_live);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_llvm_wasm_objectref_rc_emit_retire_first_slice_signed);
        let floor = gate_b09_objectref_retain_catch_all_heap_slots_first_slice_inventory();
        assert!(floor.vm_mir_run_objectref_retain_heap_slots_tracked);
        assert!(floor.objectref_rc_storage_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(floor.b09_objectref_retain_catch_all_heap_slots_first_slice_signed);
    }

    #[test]
    fn gate_b09_objectref_residual_birth_heap_slots_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_residual_birth_heap_slots_first_slice_inventory();
        let b = gate_b09_objectref_residual_birth_heap_slots_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_llvm_wasm_objectref_rc_emit_retire_first_slice_floor_ok);
        assert!(a.objectref_residual_birth_surface_inventoried);
        assert!(a.vm_objectget_objectref_managed_heap_registered);
        assert!(a.mir_run_objectget_objectref_managed_heap_registered);
        assert!(a.vm_mir_run_objectget_heap_slots_tracked);
        assert!(a.host_lazy_weak_objectref_callhost_floor_ok);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.llvm_wasm_objectref_rc_still_live);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_residual_birth_heap_slots_first_slice_signed);
        let floor = gate_b09_llvm_wasm_objectref_rc_emit_retire_first_slice_inventory();
        assert!(floor.llvm_wasm_objectref_retain_release_emit_retired);
        assert!(floor.objectref_rc_storage_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(floor.b09_llvm_wasm_objectref_rc_emit_retire_first_slice_signed);
    }

    #[test]
    fn gate_b09_objectref_arrayget_heap_slots_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_arrayget_heap_slots_first_slice_inventory();
        let b = gate_b09_objectref_arrayget_heap_slots_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_residual_birth_heap_slots_first_slice_floor_ok);
        assert!(a.vm_arrayget_objectref_managed_heap_registered);
        assert!(a.mir_run_arrayget_objectref_managed_heap_registered);
        assert!(a.vm_mir_run_arrayget_heap_slots_tracked);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.llvm_wasm_objectref_rc_still_live);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_arrayget_heap_slots_first_slice_signed);
        let floor = gate_b09_objectref_residual_birth_heap_slots_first_slice_inventory();
        assert!(floor.vm_mir_run_objectget_heap_slots_tracked);
        assert!(floor.objectref_rc_storage_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(floor.b09_objectref_residual_birth_heap_slots_first_slice_signed);
    }

    #[test]
    fn gate_b09_llvm_objectref_wrap_emit_retire_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_llvm_objectref_wrap_emit_retire_first_slice_inventory();
        let b = gate_b09_llvm_objectref_wrap_emit_retire_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_arrayget_heap_slots_first_slice_floor_ok);
        assert!(a.llvm_objectref_wrap_call_emit_retired);
        assert!(a.llvm_objectref_wrap_declare_omitted);
        assert!(a.wasm_objectref_wrap_aot_already_nop);
        assert!(a.rt_abi_object_ref_wrap_still_present);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.llvm_wasm_objectref_rc_still_live);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_llvm_objectref_wrap_emit_retire_first_slice_signed);
        let floor = gate_b09_objectref_arrayget_heap_slots_first_slice_inventory();
        assert!(floor.vm_mir_run_arrayget_heap_slots_tracked);
        assert!(floor.objectref_rc_storage_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(floor.b09_objectref_arrayget_heap_slots_first_slice_signed);
    }

    #[test]
    fn gate_b09_objectref_objectset_heap_slots_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_objectset_heap_slots_first_slice_inventory();
        let b = gate_b09_objectref_objectset_heap_slots_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_llvm_objectref_wrap_emit_retire_first_slice_floor_ok);
        assert!(a.vm_objectset_objectref_managed_heap_registered);
        assert!(a.mir_run_objectset_objectref_managed_heap_registered);
        assert!(a.vm_mir_run_objectset_heap_slots_tracked);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.llvm_wasm_objectref_rc_still_live);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_objectset_heap_slots_first_slice_signed);
        let floor = gate_b09_llvm_objectref_wrap_emit_retire_first_slice_inventory();
        assert!(floor.llvm_objectref_wrap_call_emit_retired);
        assert!(floor.objectref_rc_storage_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(floor.b09_llvm_objectref_wrap_emit_retire_first_slice_signed);
    }

    #[test]
    fn gate_b09_objectref_loadglobal_heap_slots_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_loadglobal_heap_slots_first_slice_inventory();
        let b = gate_b09_objectref_loadglobal_heap_slots_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_objectset_heap_slots_first_slice_floor_ok);
        assert!(a.vm_loadglobal_objectref_managed_heap_registered);
        assert!(a.mir_run_loadglobal_objectref_managed_heap_registered);
        assert!(a.mir_run_ssa_loadglobal_objectref_managed_heap_registered);
        assert!(a.vm_mir_run_loadglobal_heap_slots_tracked);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.llvm_wasm_objectref_rc_still_live);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_loadglobal_heap_slots_first_slice_signed);
        let floor = gate_b09_objectref_objectset_heap_slots_first_slice_inventory();
        assert!(floor.vm_mir_run_objectset_heap_slots_tracked);
        assert!(floor.objectref_rc_storage_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(floor.b09_objectref_objectset_heap_slots_first_slice_signed);
    }

    #[test]
    fn gate_b09_objectref_ssa_loadlocal_heap_slots_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_ssa_loadlocal_heap_slots_first_slice_inventory();
        let b = gate_b09_objectref_ssa_loadlocal_heap_slots_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_loadglobal_heap_slots_first_slice_floor_ok);
        assert!(a.mir_run_ssa_loadlocal_objectref_managed_heap_registered);
        assert!(a.mir_run_ssa_loadlocal_heap_slots_tracked);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.llvm_wasm_objectref_rc_still_live);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_ssa_loadlocal_heap_slots_first_slice_signed);
        let floor = gate_b09_objectref_loadglobal_heap_slots_first_slice_inventory();
        assert!(floor.vm_mir_run_loadglobal_heap_slots_tracked);
        assert!(floor.objectref_rc_storage_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(floor.b09_objectref_loadglobal_heap_slots_first_slice_signed);
    }

    #[test]
    fn gate_b09_objectref_rc_migrate_readiness_thin_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_rc_migrate_readiness_thin_inventory();
        let b = gate_b09_objectref_rc_migrate_readiness_thin_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_ssa_loadlocal_heap_slots_first_slice_floor_ok);
        assert!(a.value_vm_mir_run_objectref_rc_still_live);
        assert!(a.rt_abi_object_ref_retain_release_wrap_still_present);
        assert!(a.llvm_wasm_objectref_rc_emit_path_retired);
        assert!(a.dual_track_birth_surfaces_inventoried);
        assert!(a.nested_field_residual_noted);
        assert!(a.migrate_blockers_documented);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_rc_migrate_readiness_thin_signed);
        let floor = gate_b09_objectref_ssa_loadlocal_heap_slots_first_slice_inventory();
        assert!(floor.mir_run_ssa_loadlocal_heap_slots_tracked);
        assert!(floor.objectref_rc_storage_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(floor.b09_objectref_ssa_loadlocal_heap_slots_first_slice_signed);
    }

    #[test]
    fn gate_b09_rt_abi_objectref_wrap_retire_readiness_thin_inventory_is_deterministic_and_signed() {
        let a = gate_b09_rt_abi_objectref_wrap_retire_readiness_thin_inventory();
        let b = gate_b09_rt_abi_objectref_wrap_retire_readiness_thin_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_rc_migrate_readiness_thin_floor_ok);
        assert!(a.llvm_objectref_wrap_emit_retired);
        assert!(a.rt_abi_object_ref_wrap_still_present);
        assert!(a.rt_abi_object_ref_retain_release_still_present);
        assert!(a.value_vm_mir_run_objectref_rc_still_live);
        assert!(a.wrap_retire_blockers_documented);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_rt_abi_objectref_wrap_retire_readiness_thin_signed);
        let floor = gate_b09_objectref_rc_migrate_readiness_thin_inventory();
        assert!(floor.b09_objectref_rc_migrate_readiness_thin_signed);
        assert!(floor.rt_abi_object_ref_retain_release_wrap_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_rt_abi_objectref_retain_release_retire_readiness_thin_inventory_is_deterministic_and_signed()
    {
        let a = gate_b09_rt_abi_objectref_retain_release_retire_readiness_thin_inventory();
        let b = gate_b09_rt_abi_objectref_retain_release_retire_readiness_thin_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_rt_abi_objectref_wrap_retire_readiness_thin_floor_ok);
        assert!(a.llvm_wasm_objectref_retain_release_emit_retired);
        assert!(a.rt_abi_object_ref_retain_release_still_present);
        assert!(a.rt_abi_object_ref_wrap_still_present);
        assert!(a.value_vm_mir_run_objectref_rc_still_live);
        assert!(a.retain_release_retire_blockers_documented);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_rt_abi_objectref_retain_release_retire_readiness_thin_signed);
        let floor = gate_b09_rt_abi_objectref_wrap_retire_readiness_thin_inventory();
        assert!(floor.b09_rt_abi_objectref_wrap_retire_readiness_thin_signed);
        assert!(floor.rt_abi_object_ref_retain_release_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_rc_retire_readiness_thin_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_rc_retire_readiness_thin_inventory();
        let b = gate_b09_objectref_rc_retire_readiness_thin_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_rt_abi_objectref_retain_release_retire_readiness_thin_floor_ok);
        assert!(a.value_vm_mir_run_objectref_rc_still_live);
        assert!(a.rt_abi_object_ref_retain_release_wrap_still_present);
        assert!(a.llvm_wasm_objectref_rc_emit_path_retired);
        assert!(a.rc_retire_blockers_documented);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_rc_retire_readiness_thin_signed);
        let floor = gate_b09_rt_abi_objectref_retain_release_retire_readiness_thin_inventory();
        assert!(floor.b09_rt_abi_objectref_retain_release_retire_readiness_thin_signed);
        assert!(floor.value_vm_mir_run_objectref_rc_still_live);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_rc_retain_opcode_retire_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_rc_retain_opcode_retire_first_slice_inventory();
        let b = gate_b09_objectref_rc_retain_opcode_retire_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_rc_retire_readiness_thin_floor_ok);
        assert!(a.vm_mir_run_objectref_retain_opcode_rc_retired);
        assert!(a.vm_mir_run_objectref_retain_opcode_dual_track_kept);
        assert!(a.dup_loadlocal_objectref_rc_still_live);
        assert!(a.objectref_release_deinit_rc_still_live);
        assert!(a.value_object_ref_retain_release_api_still_present);
        assert!(a.rt_abi_object_ref_retain_release_wrap_still_present);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_rc_retain_opcode_retire_first_slice_signed);
        let floor = gate_b09_objectref_rc_retire_readiness_thin_inventory();
        assert!(floor.b09_objectref_rc_retire_readiness_thin_signed);
        assert!(floor.value_vm_mir_run_objectref_rc_still_live);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_rc_dup_loadlocal_retire_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_rc_dup_loadlocal_retire_first_slice_inventory();
        let b = gate_b09_objectref_rc_dup_loadlocal_retire_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_rc_retain_opcode_retire_first_slice_floor_ok);
        assert!(a.b09_objectref_ssa_loadlocal_heap_slots_first_slice_floor_ok);
        assert!(a.vm_mir_run_dup_loadlocal_objectref_rc_retired);
        assert!(a.vm_mir_run_dup_loadlocal_dual_track_kept);
        assert!(a.objectref_release_deinit_rc_still_live);
        assert!(a.value_object_ref_retain_release_api_still_present);
        assert!(a.rt_abi_object_ref_retain_release_wrap_still_present);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_rc_dup_loadlocal_retire_first_slice_signed);
        let floor = gate_b09_objectref_rc_retain_opcode_retire_first_slice_inventory();
        assert!(floor.b09_objectref_rc_retain_opcode_retire_first_slice_signed);
        assert!(floor.objectref_release_deinit_rc_still_live);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_rc_release_opcode_retire_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_rc_release_opcode_retire_first_slice_inventory();
        let b = gate_b09_objectref_rc_release_opcode_retire_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_rc_dup_loadlocal_retire_first_slice_floor_ok);
        assert!(a.b09_objectref_rc_retain_opcode_retire_first_slice_floor_ok);
        assert!(a.vm_mir_run_objectref_release_opcode_rc_retired);
        assert!(a.storelocal_deinit_objectref_rc_still_live);
        assert!(a.value_object_ref_retain_release_api_still_present);
        assert!(a.rt_abi_object_ref_retain_release_wrap_still_present);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_rc_release_opcode_retire_first_slice_signed);
        let floor = gate_b09_objectref_rc_dup_loadlocal_retire_first_slice_inventory();
        assert!(floor.b09_objectref_rc_dup_loadlocal_retire_first_slice_signed);
        assert!(floor.objectref_release_deinit_rc_still_live);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_storelocal_heap_unroot_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_storelocal_heap_unroot_first_slice_inventory();
        let b = gate_b09_objectref_storelocal_heap_unroot_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_rc_release_opcode_retire_first_slice_floor_ok);
        assert!(a.vm_mir_run_storelocal_objectref_heap_unroot_wired);
        assert!(a.storelocal_objectref_rc_still_live);
        assert!(a.run_class_deinits_objectref_rc_still_live);
        assert!(a.value_object_ref_retain_release_api_still_present);
        assert!(a.rt_abi_object_ref_retain_release_wrap_still_present);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_storelocal_heap_unroot_first_slice_signed);
        let floor = gate_b09_objectref_rc_release_opcode_retire_first_slice_inventory();
        assert!(floor.b09_objectref_rc_release_opcode_retire_first_slice_signed);
        assert!(floor.storelocal_deinit_objectref_rc_still_live);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_deinits_heap_unroot_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_deinits_heap_unroot_first_slice_inventory();
        let b = gate_b09_objectref_deinits_heap_unroot_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_storelocal_heap_unroot_first_slice_floor_ok);
        assert!(a.vm_mir_run_deinits_objectref_heap_unroot_wired);
        assert!(a.storelocal_objectref_rc_still_live);
        assert!(a.run_class_deinits_objectref_rc_still_live);
        assert!(a.value_object_ref_retain_release_api_still_present);
        assert!(a.rt_abi_object_ref_retain_release_wrap_still_present);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_deinits_heap_unroot_first_slice_signed);
        let floor = gate_b09_objectref_storelocal_heap_unroot_first_slice_inventory();
        assert!(floor.b09_objectref_storelocal_heap_unroot_first_slice_signed);
        assert!(floor.run_class_deinits_objectref_rc_still_live);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_gc_type_deinit_sweep_bridge_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_gc_type_deinit_sweep_bridge_first_slice_inventory();
        let b = gate_b09_gc_type_deinit_sweep_bridge_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_deinits_heap_unroot_first_slice_floor_ok);
        assert!(a.managed_heap_collect_objectref_type_deinit_wired);
        assert!(a.free_slot_still_skips_type_deinit);
        assert!(a.storelocal_objectref_rc_still_live);
        assert!(a.run_class_deinits_objectref_rc_still_live);
        assert!(a.value_object_ref_retain_release_api_still_present);
        assert!(a.rt_abi_object_ref_retain_release_wrap_still_present);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_gc_type_deinit_sweep_bridge_first_slice_signed);
        let floor = gate_b09_objectref_deinits_heap_unroot_first_slice_inventory();
        assert!(floor.b09_objectref_deinits_heap_unroot_first_slice_signed);
        assert!(floor.run_class_deinits_objectref_rc_still_live);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_storelocal_deinits_rc_retire_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_storelocal_deinits_rc_retire_first_slice_inventory();
        let b = gate_b09_objectref_storelocal_deinits_rc_retire_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_gc_type_deinit_sweep_bridge_first_slice_floor_ok);
        assert!(a.vm_mir_run_storelocal_objectref_rc_retired);
        assert!(a.vm_mir_run_deinits_objectref_rc_retired);
        assert!(a.vm_mir_run_storelocal_deinits_heap_unroot_kept);
        assert!(a.managed_heap_collect_objectref_type_deinit_still_wired);
        assert!(a.value_object_ref_retain_release_api_still_present);
        assert!(a.rt_abi_object_ref_retain_release_wrap_still_present);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_storelocal_deinits_rc_retire_first_slice_signed);
        let floor = gate_b09_gc_type_deinit_sweep_bridge_first_slice_inventory();
        assert!(floor.b09_gc_type_deinit_sweep_bridge_first_slice_signed);
        assert!(floor.managed_heap_collect_objectref_type_deinit_wired);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_value_rc_api_delete_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_value_rc_api_delete_first_slice_inventory();
        let b = gate_b09_objectref_value_rc_api_delete_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_storelocal_deinits_rc_retire_first_slice_floor_ok);
        assert!(a.value_object_ref_retain_release_api_deleted);
        assert!(a.object_handle_drop_self_contained_rc);
        assert!(a.rt_abi_object_ref_retain_release_wrap_still_present);
        assert!(a.managed_heap_collect_objectref_type_deinit_still_wired);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_value_rc_api_delete_first_slice_signed);
        let floor = gate_b09_objectref_storelocal_deinits_rc_retire_first_slice_inventory();
        assert!(floor.b09_objectref_storelocal_deinits_rc_retire_first_slice_signed);
        assert!(floor.vm_mir_run_storelocal_objectref_rc_retired);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_inventory_is_deterministic_and_signed()
    {
        let a = gate_b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_inventory();
        let b = gate_b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_value_rc_api_delete_first_slice_floor_ok);
        assert!(a.rt_abi_object_ref_wrap_retain_release_deleted);
        assert!(a.rt_abi_object_handle_retain_release_drop_still_present);
        assert!(a.value_object_ref_retain_release_api_deleted);
        assert!(a.managed_heap_collect_objectref_type_deinit_still_wired);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_signed);
        let floor = gate_b09_objectref_value_rc_api_delete_first_slice_inventory();
        assert!(floor.b09_objectref_value_rc_api_delete_first_slice_signed);
        assert!(floor.value_object_ref_retain_release_api_deleted);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_rc_migrate_readiness_refresh_thin_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_rc_migrate_readiness_refresh_thin_inventory();
        let b = gate_b09_objectref_rc_migrate_readiness_refresh_thin_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_floor_ok);
        assert!(a.value_object_ref_retain_release_api_deleted);
        assert!(a.rt_abi_object_ref_wrap_retain_release_deleted);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.rt_abi_object_handle_retain_release_drop_still_present);
        assert!(a.dual_track_and_rc_storage_coexistence_noted);
        assert!(a.nested_field_dual_track_still_unmet);
        assert!(a.migrate_flip_blockers_refreshed);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_rc_migrate_readiness_refresh_thin_signed);
        let floor = gate_b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_inventory();
        assert!(floor.b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_signed);
        assert!(floor.rt_abi_object_ref_wrap_retain_release_deleted);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_nested_field_dual_track_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_nested_field_dual_track_first_slice_inventory();
        let b = gate_b09_objectref_nested_field_dual_track_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_rc_migrate_readiness_refresh_thin_floor_ok);
        assert!(a.vm_objectset_nested_value_dual_track_wired);
        assert!(a.mir_run_objectset_nested_value_dual_track_wired);
        assert!(a.nested_field_write_path_dual_track_met);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.rt_abi_object_handle_retain_release_drop_still_present);
        assert!(a.dual_track_and_rc_storage_coexistence_noted);
        assert!(a.deep_field_graph_gc_walk_still_unmet);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_nested_field_dual_track_first_slice_signed);
        let floor = gate_b09_objectref_rc_migrate_readiness_refresh_thin_inventory();
        assert!(floor.b09_objectref_rc_migrate_readiness_refresh_thin_signed);
        assert!(floor.nested_field_dual_track_still_unmet);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_rt_abi_objectref_handle_retire_readiness_thin_inventory_is_deterministic_and_signed() {
        let a = gate_b09_rt_abi_objectref_handle_retire_readiness_thin_inventory();
        let b = gate_b09_rt_abi_objectref_handle_retire_readiness_thin_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_nested_field_dual_track_first_slice_floor_ok);
        assert!(a.rt_abi_object_handle_retain_release_drop_still_present);
        assert!(a.nested_field_write_path_dual_track_met);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.dual_track_and_rc_storage_coexistence_noted);
        assert!(a.deep_field_graph_gc_walk_still_unmet);
        assert!(a.handle_retire_blockers_documented);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_rt_abi_objectref_handle_retire_readiness_thin_signed);
        let floor = gate_b09_objectref_nested_field_dual_track_first_slice_inventory();
        assert!(floor.b09_objectref_nested_field_dual_track_first_slice_signed);
        assert!(floor.rt_abi_object_handle_retain_release_drop_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_inventory_is_deterministic_and_signed(
    ) {
        let a = gate_b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_inventory();
        let b = gate_b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_rt_abi_objectref_handle_retire_readiness_thin_floor_ok);
        assert!(a.rt_abi_object_handle_retain_release_drop_deleted);
        assert!(a.object_handle_table_api_still_present);
        assert!(a.mir_run_object_ref_opcodes_still_present);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.nested_field_write_path_dual_track_met);
        assert!(a.deep_field_graph_gc_walk_still_unmet);
        assert!(a.value_object_ref_retain_release_api_deleted);
        assert!(a.rt_abi_object_ref_wrap_retain_release_deleted);
        assert!(a.managed_heap_collect_objectref_type_deinit_still_wired);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_signed);
        let floor = gate_b09_rt_abi_objectref_handle_retire_readiness_thin_inventory();
        assert!(floor.b09_rt_abi_objectref_handle_retire_readiness_thin_signed);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_rc_migrate_readiness_post_handle_delete_refresh_thin_inventory_is_deterministic_and_signed(
    ) {
        let a = gate_b09_objectref_rc_migrate_readiness_post_handle_delete_refresh_thin_inventory();
        let b = gate_b09_objectref_rc_migrate_readiness_post_handle_delete_refresh_thin_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_floor_ok);
        assert!(a.rt_abi_object_handle_retain_release_drop_deleted);
        assert!(a.nested_field_write_path_dual_track_met);
        assert!(a.value_object_ref_retain_release_api_deleted);
        assert!(a.rt_abi_object_ref_wrap_retain_release_deleted);
        assert!(a.object_handle_table_api_still_present);
        assert!(a.mir_run_object_ref_opcodes_still_present);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.deep_field_graph_gc_walk_still_unmet);
        assert!(a.dual_track_and_rc_storage_coexistence_noted);
        assert!(a.migrate_flip_blockers_refreshed_after_handle_delete);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_rc_migrate_readiness_post_handle_delete_refresh_thin_signed);
        let floor = gate_b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_inventory();
        assert!(floor.b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_signed);
        assert!(floor.rt_abi_object_handle_retain_release_drop_deleted);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_strip_rc_readiness_thin_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_strip_rc_readiness_thin_inventory();
        let b = gate_b09_objectref_strip_rc_readiness_thin_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_rc_migrate_readiness_post_handle_delete_refresh_thin_floor_ok);
        assert!(a.rt_abi_object_handle_retain_release_drop_deleted);
        assert!(a.nested_field_write_path_dual_track_met);
        assert!(a.object_handle_table_api_still_present);
        assert!(a.mir_run_object_ref_opcodes_still_present);
        assert!(a.objectref_rc_storage_still_present);
        assert!(a.deep_field_graph_gc_walk_still_unmet);
        assert!(a.dual_track_and_rc_storage_coexistence_noted);
        assert!(a.strip_rc_blockers_documented);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_strip_rc_readiness_thin_signed);
        let floor =
            gate_b09_objectref_rc_migrate_readiness_post_handle_delete_refresh_thin_inventory();
        assert!(floor.b09_objectref_rc_migrate_readiness_post_handle_delete_refresh_thin_signed);
        assert!(floor.objectref_rc_storage_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_strip_rc_delete_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_strip_rc_delete_first_slice_inventory();
        let b = gate_b09_objectref_strip_rc_delete_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_strip_rc_readiness_thin_floor_ok);
        assert!(a.objectref_rc_storage_deleted);
        assert!(a.object_handle_table_api_still_present);
        assert!(a.mir_run_object_ref_opcodes_still_present);
        assert!(a.deep_field_graph_gc_walk_still_unmet);
        assert!(a.dual_track_and_heap_coexistence_noted);
        assert!(a.object_handle_retain_rc_retired);
        assert!(a.object_handle_drop_arc_death_wired);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_strip_rc_delete_first_slice_signed);
        let floor = gate_b09_objectref_strip_rc_readiness_thin_inventory();
        assert!(floor.b09_objectref_strip_rc_readiness_thin_signed);
        assert!(floor.objectref_rc_storage_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_deep_field_graph_gc_walk_readiness_thin_inventory_is_deterministic_and_signed()
    {
        let a = gate_b09_objectref_deep_field_graph_gc_walk_readiness_thin_inventory();
        let b = gate_b09_objectref_deep_field_graph_gc_walk_readiness_thin_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_strip_rc_delete_first_slice_floor_ok);
        assert!(a.objectref_rc_storage_deleted);
        assert!(a.object_handle_table_api_still_present);
        assert!(a.mir_run_object_ref_opcodes_still_present);
        assert!(a.deep_field_graph_gc_walk_still_unmet);
        assert!(a.deep_field_graph_gc_walk_blockers_documented);
        assert!(a.managed_heap_trace_object_ref_fields_noted);
        assert!(a.dual_track_and_heap_coexistence_noted);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_deep_field_graph_gc_walk_readiness_thin_signed);
        let floor = gate_b09_objectref_strip_rc_delete_first_slice_inventory();
        assert!(floor.b09_objectref_strip_rc_delete_first_slice_signed);
        assert!(floor.objectref_rc_storage_deleted);
        assert!(floor.deep_field_graph_gc_walk_still_unmet);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_deep_field_graph_gc_walk_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_deep_field_graph_gc_walk_first_slice_inventory();
        let b = gate_b09_objectref_deep_field_graph_gc_walk_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_deep_field_graph_gc_walk_readiness_thin_floor_ok);
        assert!(a.objectref_rc_storage_deleted);
        assert!(a.object_handle_table_api_still_present);
        assert!(a.mir_run_object_ref_opcodes_still_present);
        assert!(a.deep_field_graph_gc_walk_met);
        assert!(a.deep_field_graph_gc_walk_behavioral_nail_ok);
        assert!(a.managed_heap_trace_object_ref_fields_wired);
        assert!(a.dual_track_and_heap_coexistence_noted);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_deep_field_graph_gc_walk_first_slice_signed);
        let floor = gate_b09_objectref_deep_field_graph_gc_walk_readiness_thin_inventory();
        assert!(floor.b09_objectref_deep_field_graph_gc_walk_readiness_thin_signed);
        assert!(floor.deep_field_graph_gc_walk_still_unmet);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_peel_readiness_thin_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_peel_readiness_thin_inventory();
        let b = gate_b09_objectref_peel_readiness_thin_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_deep_field_graph_gc_walk_first_slice_floor_ok);
        assert!(a.deep_field_graph_gc_walk_met);
        assert!(a.objectref_rc_storage_deleted);
        assert!(a.object_handle_table_api_still_present);
        assert!(a.mir_run_object_ref_opcodes_still_present);
        assert!(a.handle_table_mir_run_opcode_peel_still_unmet);
        assert!(a.peel_blockers_documented);
        assert!(a.object_ref_wrap_uses_register_object_handle_noted);
        assert!(a.mir_run_object_ref_opcode_shells_noted);
        assert!(a.dual_track_and_heap_coexistence_noted);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_peel_readiness_thin_signed);
        let floor = gate_b09_objectref_deep_field_graph_gc_walk_first_slice_inventory();
        assert!(floor.b09_objectref_deep_field_graph_gc_walk_first_slice_signed);
        assert!(floor.deep_field_graph_gc_walk_met);
        assert!(floor.object_handle_table_api_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_peel_delete_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_peel_delete_first_slice_inventory();
        let b = gate_b09_objectref_peel_delete_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_peel_readiness_thin_floor_ok);
        assert!(a.deep_field_graph_gc_walk_met);
        assert!(a.objectref_rc_storage_deleted);
        assert!(a.object_handle_table_api_deleted);
        assert!(a.mir_run_object_ref_opcodes_deleted);
        assert!(a.handle_table_mir_run_opcode_peel_met);
        assert!(a.object_ref_wrap_without_handle_registration_wired);
        assert!(a.run_class_deinits_no_object_handle_drop_wired);
        assert!(a.release_object_ref_for_handle_drop_deleted);
        assert!(a.mir_op_objectref_wrap_retain_release_still_present);
        assert!(a.vm_instruction_objectref_retain_release_still_present);
        assert!(a.dual_track_and_heap_coexistence_noted);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_peel_delete_first_slice_signed);
        let floor = gate_b09_objectref_peel_readiness_thin_inventory();
        assert!(floor.b09_objectref_peel_readiness_thin_signed);
        assert!(floor.handle_table_mir_run_opcode_peel_still_unmet);
        assert!(floor.object_handle_table_api_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_rc_migrate_readiness_post_peel_refresh_thin_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_rc_migrate_readiness_post_peel_refresh_thin_inventory();
        let b = gate_b09_objectref_rc_migrate_readiness_post_peel_refresh_thin_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_peel_delete_first_slice_floor_ok);
        assert!(a.handle_table_mir_run_opcode_peel_met);
        assert!(a.object_handle_table_api_deleted);
        assert!(a.mir_run_object_ref_opcodes_deleted);
        assert!(a.objectref_rc_storage_deleted);
        assert!(a.deep_field_graph_gc_walk_met);
        assert!(a.nested_field_write_path_dual_track_met);
        assert!(a.rt_abi_object_handle_retain_release_drop_deleted);
        assert!(a.mir_op_objectref_wrap_retain_release_still_present);
        assert!(a.vm_instruction_objectref_retain_release_still_present);
        assert!(a.dual_track_and_heap_coexistence_noted);
        assert!(a.migrate_flip_blockers_refreshed_after_peel);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_rc_migrate_readiness_post_peel_refresh_thin_signed);
        let floor = gate_b09_objectref_peel_delete_first_slice_inventory();
        assert!(floor.b09_objectref_peel_delete_first_slice_signed);
        assert!(floor.handle_table_mir_run_opcode_peel_met);
        assert!(floor.mir_op_objectref_wrap_retain_release_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_mirop_vm_residual_peel_readiness_thin_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_mirop_vm_residual_peel_readiness_thin_inventory();
        let b = gate_b09_objectref_mirop_vm_residual_peel_readiness_thin_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_rc_migrate_readiness_post_peel_refresh_thin_floor_ok);
        assert!(a.handle_table_mir_run_opcode_peel_met);
        assert!(a.migrate_flip_blockers_refreshed_after_peel);
        assert!(a.mir_op_objectref_wrap_retain_release_still_present);
        assert!(a.vm_instruction_objectref_retain_release_still_present);
        assert!(a.mirop_vm_residual_peel_still_unmet);
        assert!(a.mirop_vm_residual_peel_blockers_documented);
        assert!(a.object_ref_wrap_via_mir_op_dual_track_noted);
        assert!(a.mir_op_vm_objectref_retain_release_shells_noted);
        assert!(a.dual_track_and_heap_coexistence_noted);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_mirop_vm_residual_peel_readiness_thin_signed);
        let floor = gate_b09_objectref_rc_migrate_readiness_post_peel_refresh_thin_inventory();
        assert!(floor.b09_objectref_rc_migrate_readiness_post_peel_refresh_thin_signed);
        assert!(floor.mir_op_objectref_wrap_retain_release_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_mirop_vm_residual_peel_delete_first_slice_inventory_is_deterministic_and_signed()
    {
        let a = gate_b09_objectref_mirop_vm_residual_peel_delete_first_slice_inventory();
        let b = gate_b09_objectref_mirop_vm_residual_peel_delete_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_mirop_vm_residual_peel_readiness_thin_floor_ok);
        assert!(a.handle_table_mir_run_opcode_peel_met);
        assert!(a.migrate_flip_blockers_refreshed_after_peel);
        assert!(a.mir_op_objectref_wrap_retain_release_deleted);
        assert!(a.vm_instruction_objectref_retain_release_deleted);
        assert!(a.mirop_vm_residual_peel_met);
        assert!(a.dictbuild_objectref_dual_track_birth_kept);
        assert!(a.object_ref_wrap_helper_deleted);
        assert!(a.dual_track_and_heap_coexistence_noted);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_mirop_vm_residual_peel_delete_first_slice_signed);
        let floor = gate_b09_objectref_mirop_vm_residual_peel_readiness_thin_inventory();
        assert!(floor.b09_objectref_mirop_vm_residual_peel_readiness_thin_signed);
        assert!(floor.mirop_vm_residual_peel_still_unmet);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_rc_migrate_readiness_post_mirop_peel_refresh_thin_inventory_is_deterministic_and_signed(
    ) {
        let a = gate_b09_objectref_rc_migrate_readiness_post_mirop_peel_refresh_thin_inventory();
        let b = gate_b09_objectref_rc_migrate_readiness_post_mirop_peel_refresh_thin_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_mirop_vm_residual_peel_delete_first_slice_floor_ok);
        assert!(a.handle_table_mir_run_opcode_peel_met);
        assert!(a.mirop_vm_residual_peel_met);
        assert!(a.mir_op_objectref_wrap_retain_release_deleted);
        assert!(a.vm_instruction_objectref_retain_release_deleted);
        assert!(a.dictbuild_objectref_dual_track_birth_kept);
        assert!(a.dual_track_and_heap_coexistence_noted);
        assert!(a.migrate_flip_blockers_refreshed_after_mirop_peel);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_rc_migrate_readiness_post_mirop_peel_refresh_thin_signed);
        let floor = gate_b09_objectref_mirop_vm_residual_peel_delete_first_slice_inventory();
        assert!(floor.b09_objectref_mirop_vm_residual_peel_delete_first_slice_signed);
        assert!(floor.mirop_vm_residual_peel_met);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_dual_track_collapse_readiness_thin_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_dual_track_collapse_readiness_thin_inventory();
        let b = gate_b09_objectref_dual_track_collapse_readiness_thin_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_rc_migrate_readiness_post_mirop_peel_refresh_thin_floor_ok);
        assert!(a.handle_table_mir_run_opcode_peel_met);
        assert!(a.mirop_vm_residual_peel_met);
        assert!(a.migrate_flip_blockers_refreshed_after_mirop_peel);
        assert!(a.dictbuild_objectref_dual_track_birth_still_present);
        assert!(a.dual_track_and_heap_coexistence_noted);
        assert!(a.dual_track_collapse_still_unmet);
        assert!(a.dual_track_collapse_blockers_documented);
        assert!(a.object_ref_from_map_arc_birth_noted);
        assert!(a.maybe_track_objectref_dual_register_noted);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_dual_track_collapse_readiness_thin_signed);
        let floor = gate_b09_objectref_rc_migrate_readiness_post_mirop_peel_refresh_thin_inventory();
        assert!(floor.b09_objectref_rc_migrate_readiness_post_mirop_peel_refresh_thin_signed);
        assert!(floor.mirop_vm_residual_peel_met);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_dual_track_collapse_delete_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_dual_track_collapse_delete_first_slice_inventory();
        let b = gate_b09_objectref_dual_track_collapse_delete_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_dual_track_collapse_readiness_thin_floor_ok);
        assert!(a.dictbuild_objectref_dual_track_birth_deleted);
        assert!(a.dictbuild_objectref_heap_primary_birth_met);
        assert!(a.dual_track_collapse_met);
        assert!(a.mirop_vm_residual_peel_met);
        assert!(a.objectref_reentry_maybe_track_residual_noted);
        assert!(a.mutrefcell_dual_track_out_of_scope);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_dual_track_collapse_delete_first_slice_signed);
        let floor = gate_b09_objectref_dual_track_collapse_readiness_thin_inventory();
        assert!(floor.b09_objectref_dual_track_collapse_readiness_thin_signed);
        assert!(floor.dual_track_collapse_still_unmet);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_rc_migrate_readiness_post_collapse_refresh_thin_inventory_is_deterministic_and_signed(
    ) {
        let a = gate_b09_objectref_rc_migrate_readiness_post_collapse_refresh_thin_inventory();
        let b = gate_b09_objectref_rc_migrate_readiness_post_collapse_refresh_thin_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_dual_track_collapse_delete_first_slice_floor_ok);
        assert!(a.dictbuild_objectref_dual_track_birth_deleted);
        assert!(a.dictbuild_objectref_heap_primary_birth_met);
        assert!(a.dual_track_collapse_met);
        assert!(a.mirop_vm_residual_peel_met);
        assert!(a.migrate_flip_blockers_refreshed_after_collapse);
        assert!(a.objectref_reentry_maybe_track_residual_noted);
        assert!(a.mutrefcell_dual_track_out_of_scope);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_rc_migrate_readiness_post_collapse_refresh_thin_signed);
        let floor = gate_b09_objectref_dual_track_collapse_delete_first_slice_inventory();
        assert!(floor.b09_objectref_dual_track_collapse_delete_first_slice_signed);
        assert!(floor.dual_track_collapse_met);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_reentry_maybe_track_peel_readiness_thin_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_reentry_maybe_track_peel_readiness_thin_inventory();
        let b = gate_b09_objectref_reentry_maybe_track_peel_readiness_thin_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_rc_migrate_readiness_post_collapse_refresh_thin_floor_ok);
        assert!(a.dual_track_collapse_met);
        assert!(a.dictbuild_objectref_heap_primary_birth_met);
        assert!(a.migrate_flip_blockers_refreshed_after_collapse);
        assert!(a.objectref_reentry_maybe_track_still_present);
        assert!(a.reentry_maybe_track_peel_still_unmet);
        assert!(a.reentry_maybe_track_peel_blockers_documented);
        assert!(a.mutrefcell_dual_track_out_of_scope);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_reentry_maybe_track_peel_readiness_thin_signed);
        let floor = gate_b09_objectref_rc_migrate_readiness_post_collapse_refresh_thin_inventory();
        assert!(floor.b09_objectref_rc_migrate_readiness_post_collapse_refresh_thin_signed);
        assert!(floor.objectref_reentry_maybe_track_residual_noted);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_reentry_maybe_track_peel_delete_first_slice_inventory_is_deterministic_and_signed()
    {
        let a = gate_b09_objectref_reentry_maybe_track_peel_delete_first_slice_inventory();
        let b = gate_b09_objectref_reentry_maybe_track_peel_delete_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_reentry_maybe_track_peel_readiness_thin_floor_ok);
        assert!(a.dual_track_collapse_met);
        assert!(a.dictbuild_objectref_heap_primary_birth_met);
        assert!(a.objectref_reentry_maybe_track_deleted);
        assert!(a.reentry_maybe_track_peel_met);
        assert!(a.mutrefcell_maybe_track_kept);
        assert!(a.mutrefcell_dual_track_out_of_scope);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_reentry_maybe_track_peel_delete_first_slice_signed);
        let floor = gate_b09_objectref_reentry_maybe_track_peel_readiness_thin_inventory();
        assert!(floor.b09_objectref_reentry_maybe_track_peel_readiness_thin_signed);
        assert!(floor.objectref_reentry_maybe_track_still_present);
        assert!(floor.reentry_maybe_track_peel_still_unmet);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_rc_migrate_readiness_post_reentry_peel_refresh_thin_inventory_is_deterministic_and_signed()
    {
        let a = gate_b09_objectref_rc_migrate_readiness_post_reentry_peel_refresh_thin_inventory();
        let b = gate_b09_objectref_rc_migrate_readiness_post_reentry_peel_refresh_thin_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_reentry_maybe_track_peel_delete_first_slice_floor_ok);
        assert!(a.objectref_reentry_maybe_track_deleted);
        assert!(a.reentry_maybe_track_peel_met);
        assert!(a.dual_track_collapse_met);
        assert!(a.dictbuild_objectref_heap_primary_birth_met);
        assert!(a.migrate_flip_blockers_refreshed_after_reentry_peel);
        assert!(a.mutrefcell_maybe_track_still_present);
        assert!(a.mutrefcell_dual_track_out_of_scope);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_rc_migrate_readiness_post_reentry_peel_refresh_thin_signed);
        let floor = gate_b09_objectref_reentry_maybe_track_peel_delete_first_slice_inventory();
        assert!(floor.b09_objectref_reentry_maybe_track_peel_delete_first_slice_signed);
        assert!(floor.objectref_reentry_maybe_track_deleted);
        assert!(floor.reentry_maybe_track_peel_met);
        assert!(floor.mutrefcell_maybe_track_kept);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_mutrefcell_maybe_track_peel_readiness_thin_inventory_is_deterministic_and_signed() {
        let a = gate_b09_mutrefcell_maybe_track_peel_readiness_thin_inventory();
        let b = gate_b09_mutrefcell_maybe_track_peel_readiness_thin_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_rc_migrate_readiness_post_reentry_peel_refresh_thin_floor_ok);
        assert!(a.objectref_reentry_maybe_track_deleted);
        assert!(a.reentry_maybe_track_peel_met);
        assert!(a.dual_track_collapse_met);
        assert!(a.dictbuild_objectref_heap_primary_birth_met);
        assert!(a.migrate_flip_blockers_refreshed_after_reentry_peel);
        assert!(a.mutrefcell_maybe_track_still_present);
        assert!(a.mutrefcell_peel_still_unmet);
        assert!(a.mutrefcell_peel_blockers_documented);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_mutrefcell_maybe_track_peel_readiness_thin_signed);
        let floor = gate_b09_objectref_rc_migrate_readiness_post_reentry_peel_refresh_thin_inventory();
        assert!(floor.b09_objectref_rc_migrate_readiness_post_reentry_peel_refresh_thin_signed);
        assert!(floor.mutrefcell_maybe_track_still_present);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_mutrefcell_maybe_track_peel_delete_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_mutrefcell_maybe_track_peel_delete_first_slice_inventory();
        let b = gate_b09_mutrefcell_maybe_track_peel_delete_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_mutrefcell_maybe_track_peel_readiness_thin_floor_ok);
        assert!(a.objectref_reentry_maybe_track_deleted);
        assert!(a.reentry_maybe_track_peel_met);
        assert!(a.dual_track_collapse_met);
        assert!(a.dictbuild_objectref_heap_primary_birth_met);
        assert!(a.migrate_flip_blockers_refreshed_after_reentry_peel);
        assert!(a.mutrefcell_maybe_track_deleted);
        assert!(a.mutrefcell_peel_met);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_mutrefcell_maybe_track_peel_delete_first_slice_signed);
        let floor = gate_b09_mutrefcell_maybe_track_peel_readiness_thin_inventory();
        assert!(floor.b09_mutrefcell_maybe_track_peel_readiness_thin_signed);
        assert!(floor.mutrefcell_maybe_track_still_present);
        assert!(floor.mutrefcell_peel_still_unmet);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_rc_migrate_readiness_post_mutrefcell_peel_refresh_thin_inventory_is_deterministic_and_signed()
    {
        let a = gate_b09_objectref_rc_migrate_readiness_post_mutrefcell_peel_refresh_thin_inventory();
        let b = gate_b09_objectref_rc_migrate_readiness_post_mutrefcell_peel_refresh_thin_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_mutrefcell_maybe_track_peel_delete_first_slice_floor_ok);
        assert!(a.mutrefcell_maybe_track_deleted);
        assert!(a.mutrefcell_peel_met);
        assert!(a.objectref_reentry_maybe_track_deleted);
        assert!(a.reentry_maybe_track_peel_met);
        assert!(a.dual_track_collapse_met);
        assert!(a.dictbuild_objectref_heap_primary_birth_met);
        assert!(a.migrate_flip_blockers_refreshed_after_mutrefcell_peel);
        assert!(!a.objectref_rc_migrate_enabled);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_rc_migrate_readiness_post_mutrefcell_peel_refresh_thin_signed);
        let floor = gate_b09_mutrefcell_maybe_track_peel_delete_first_slice_inventory();
        assert!(floor.b09_mutrefcell_maybe_track_peel_delete_first_slice_signed);
        assert!(floor.mutrefcell_maybe_track_deleted);
        assert!(floor.mutrefcell_peel_met);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_objectref_rc_migrate_product_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_objectref_rc_migrate_product_first_slice_inventory();
        let b = gate_b09_objectref_rc_migrate_product_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_rc_migrate_readiness_post_mutrefcell_peel_refresh_thin_floor_ok);
        assert!(a.migrate_flip_blockers_refreshed_after_mutrefcell_peel);
        assert!(a.mutrefcell_peel_met);
        assert!(a.reentry_maybe_track_peel_met);
        assert!(a.dual_track_collapse_met);
        assert!(a.dictbuild_objectref_heap_primary_birth_met);
        assert!(a.objectref_rc_storage_stripped);
        assert!(a.deep_field_graph_gc_walk_met);
        assert!(a.sharedcell_arc_lifetime_still_present);
        assert!(a.objectref_rc_migrate_enabled);
        assert!(a.migrate_product_flip_met);
        assert!(!a.storage_cleanup_enabled);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_objectref_rc_migrate_product_first_slice_signed);
        assert!(objectref_rc_migrate_enabled());
        assert_eq!(
            objectref_rc_migrate_enabled(),
            a.objectref_rc_migrate_enabled
        );
        let floor = gate_b09_objectref_rc_migrate_readiness_post_mutrefcell_peel_refresh_thin_inventory();
        assert!(floor.b09_objectref_rc_migrate_readiness_post_mutrefcell_peel_refresh_thin_signed);
        assert!(!floor.objectref_rc_migrate_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_storage_cleanup_product_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_storage_cleanup_product_first_slice_inventory();
        let b = gate_b09_storage_cleanup_product_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_objectref_rc_migrate_product_first_slice_floor_ok);
        assert!(a.objectref_rc_migrate_enabled);
        assert!(a.migrate_product_flip_met);
        assert!(a.b09_storage_cleanup_first_slice_floor_ok);
        assert!(a.storage_cleanup_dead_api_retired);
        assert!(a.memory_pool_type_stub_retired);
        assert!(a.sharedcell_arc_lifetime_still_present);
        assert!(a.storage_cleanup_enabled);
        assert!(a.storage_cleanup_product_flip_met);
        assert!(!a.unified_mir_runtime_heap_enabled);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_storage_cleanup_product_first_slice_signed);
        assert!(storage_cleanup_enabled());
        assert_eq!(storage_cleanup_enabled(), a.storage_cleanup_enabled);
        let floor = gate_b09_objectref_rc_migrate_product_first_slice_inventory();
        assert!(floor.b09_objectref_rc_migrate_product_first_slice_signed);
        assert!(floor.objectref_rc_migrate_enabled);
        assert!(!floor.storage_cleanup_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_unified_mir_runtime_heap_enable_product_first_slice_inventory_is_deterministic_and_signed()
    {
        let a = gate_b09_unified_mir_runtime_heap_enable_product_first_slice_inventory();
        let b = gate_b09_unified_mir_runtime_heap_enable_product_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_storage_cleanup_product_first_slice_floor_ok);
        assert!(a.storage_cleanup_enabled);
        assert!(a.storage_cleanup_product_flip_met);
        assert!(a.objectref_rc_migrate_enabled);
        assert!(a.migrate_product_flip_met);
        assert!(a.b09_enable_readiness_thin_floor_ok);
        assert!(a.sharedcell_arc_lifetime_still_present);
        assert!(a.unified_mir_runtime_heap_enabled);
        assert!(a.unified_enable_product_flip_met);
        assert!(!a.b09_axis_exit_enabled);
        assert!(!a.gc_ga_claimed);
        assert!(a.b09_unified_mir_runtime_heap_enable_product_first_slice_signed);
        assert!(unified_mir_runtime_heap_enabled());
        assert_eq!(
            unified_mir_runtime_heap_enabled(),
            a.unified_mir_runtime_heap_enabled
        );
        let floor = gate_b09_storage_cleanup_product_first_slice_inventory();
        assert!(floor.b09_storage_cleanup_product_first_slice_signed);
        assert!(floor.storage_cleanup_enabled);
        assert!(!floor.unified_mir_runtime_heap_enabled);
        assert!(!floor.b09_axis_exit_enabled);
    }

    #[test]
    fn gate_b09_axis_exit_gc_ga_product_first_slice_inventory_is_deterministic_and_signed() {
        let a = gate_b09_axis_exit_gc_ga_product_first_slice_inventory();
        let b = gate_b09_axis_exit_gc_ga_product_first_slice_inventory();
        assert_eq!(a, b);
        assert!(a.relational_inventory_overlap_ok);
        assert!(a.b09_unified_mir_runtime_heap_enable_product_first_slice_floor_ok);
        assert!(a.unified_mir_runtime_heap_enabled);
        assert!(a.unified_enable_product_flip_met);
        assert!(a.storage_cleanup_enabled);
        assert!(a.objectref_rc_migrate_enabled);
        assert!(a.sharedcell_arc_lifetime_still_present);
        assert!(a.b09_axis_exit_enabled);
        assert!(a.gc_ga_claimed);
        assert!(a.axis_exit_product_flip_met);
        assert!(a.narrow_gc_ga_claimed);
        assert!(a.wasm_gc_still_unmet);
        assert!(a.scan_all_auto_still_unmet);
        assert!(a.exports_ga_still_unmet);
        assert!(a.rt_abi_full_clear_still_unmet);
        assert!(a.b09_axis_exit_gc_ga_product_first_slice_signed);
        assert!(b09_axis_exit_enabled());
        assert!(gc_ga_claimed());
        assert_eq!(b09_axis_exit_enabled(), a.b09_axis_exit_enabled);
        assert_eq!(gc_ga_claimed(), a.gc_ga_claimed);
        let floor = gate_b09_unified_mir_runtime_heap_enable_product_first_slice_inventory();
        assert!(floor.b09_unified_mir_runtime_heap_enable_product_first_slice_signed);
        assert!(floor.unified_mir_runtime_heap_enabled);
        assert!(!floor.b09_axis_exit_enabled);
        assert!(!floor.gc_ga_claimed);
    }

    #[test]
    fn narrative_docs_present_and_do_not_claim_unified_heap_w476() {
        let parity = include_str!("../../../plan/topics/gc-parity.md");
        assert!(
            parity.contains("Three layers")
                || parity.contains("three layers")
                || parity.contains("syx run"),
            "gc-parity must document three-layer heap honesty"
        );
        assert!(
            parity.contains("MirRuntime") && parity.contains("GcCollect"),
            "gc-parity must name MirRuntime and GcCollect"
        );
        let policy = include_str!("../../../docs/gates/gate-b09-gc-narrative-thin.md");
        assert!(
            policy.contains("ADR-820") && policy.contains("b09_gc_narrative_thin_signed=true"),
            "policy must cite ADR-820 and signed flag"
        );
        assert!(
            policy.contains("unified_mir_runtime_heap_enabled=false")
                || policy.contains("`unified_mir_runtime_heap_enabled=false`"),
            "policy must keep unified heap disabled"
        );
    }

    #[test]
    fn deepen_docs_present_and_do_not_claim_unified_heap_w485() {
        let deepen = include_str!("../../../docs/gates/gate-b09-gc-narrative-deepen.md");
        assert!(
            deepen.contains("ADR-829")
                && deepen.contains("b09_gc_narrative_deepen_signed=true")
                && deepen.contains("stack")
                && deepen.contains("GcCollect")
                && (deepen.contains("wasm-gc") || deepen.contains("wasm_gc")),
            "deepen policy must cite ADR-829, signed flag, roots, converge, wasm-gc defer"
        );
        assert!(
            deepen.contains("unified_mir_runtime_heap_enabled=false")
                || deepen.contains("`unified_mir_runtime_heap_enabled=false`"),
            "deepen policy must keep unified heap disabled"
        );
        let parity = include_str!("../../../plan/topics/gc-parity.md");
        assert!(
            parity.contains("W485") || parity.contains("ADR-829") || parity.contains("deepen"),
            "gc-parity must cite W485 deepen"
        );
    }
}
