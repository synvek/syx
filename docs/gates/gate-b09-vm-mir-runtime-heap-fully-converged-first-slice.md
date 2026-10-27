# B-09 VM↔MirRuntime heap fully_converged first-slice (W491 / ADR-835)

> **Status:** **fully_converged first-slice signed** (ADR-835 / W491) — `VmInstance::run_gc_collect` is ManagedHeap-only (`collect_global`); MemoryPool mark-sweep **retired** from GcCollect path; ObjectRef/`MemoryPool` storage **still present**.  
> Cross-index: [`gate-b09-dual-heap-converge-first-slice.md`](gate-b09-dual-heap-converge-first-slice.md) / ADR-834；[`gate-b09-actor-mailbox-value-queue-root-set-first-slice.md`](gate-b09-actor-mailbox-value-queue-root-set-first-slice.md) / ADR-833；[`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md).  
> Roadmap B-02 **`[x]`**；B-09 **`[~]`**（fully_converged 窄片；storage honesty thin **met** W492；≠ enable ≠ cleanup ≠ `[x]`）.  
> **Is** `vm_mir_runtime_heap_fully_converged=true` (narrow: GcCollect path).  
> **Not** `unified_mir_runtime_heap_enabled=true`. **Not** B-09 `[x]` / GC GA. **Not** MemoryPool type deleted. **Not** scan-all auto / exports GA. **Not** RT ABI delete. **Not** MIR blob delete.

Fingerprint: `three-hundred-and-forty-first` / `A-W491`.

## Narrow meaning (ADR-835)

| Layer | Status this window |
|-------|--------------------|
| GcCollect / `run_gc_collect` | ManagedHeap-only — **converged** with MirRuntime `heap_collect` |
| MemoryPool `collect_mark_sweep` on GcCollect | **Retired** |
| MemoryPool type + ObjectRef RC | **Still present** (storage honesty) |
| Product enable / B-09 `[x]` / GC GA | **Still unmet** |

## Scope narrow (ADR-835)

Fully_converged first-slice **complete** (narrow) =
- W476–W490 floors (incl. dual-heap converge first-slice)
- This window: stop calling MemoryPool mark-sweep from `run_gc_collect`; sign converged=true + residual retired + ObjectRef storage present
- Honesty: enable / B-09 `[x]` / GC GA / MemoryPool API delete **still unmet**

## Acceptance

| Prerequisite | Status | Evidence |
|--------------|--------|----------|
| Dual-heap converge first-slice | **met** (W490 / ADR-834) | [`gate-b09-dual-heap-converge-first-slice.md`](gate-b09-dual-heap-converge-first-slice.md) |
| Fully_converged first-slice | **signed** (W491 / ADR-835) | this document + `GateB09VmMirRuntimeHeapFullyConvergedFirstSliceInventory` |

**Evidence nails:** `crates/syx-driver/tests/native/gate_b09_vm_mir_runtime_heap_fully_converged_first_slice.rs` + `gate_b09_vm_mir_runtime_heap_fully_converged_first_slice_inventory`.

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09VmMirRuntimeHeapFullyConvergedFirstSliceInventory` / `gate_b09_vm_mir_runtime_heap_fully_converged_first_slice_inventory` |
| Flags | `b09_vm_mir_runtime_heap_fully_converged_first_slice_signed=true`; `b09_dual_heap_converge_first_slice_floor_ok=true`; `vm_memory_pool_mark_sweep_gc_collect_residual_retired=true`; `vm_mir_runtime_heap_fully_converged=true`; `vm_memory_pool_objectref_storage_still_present=true`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Honesty floor | W476–W490 inventories keep historical product flags (incl. W490 `fully_converged=false` / `vm_memory_pool_mark_sweep_residual=true`) |
| Forbidden claim | unified heap enabled / B-09 `[x]` / GC GA / wasm-gc / MemoryPool type deleted / scan-all auto / RT ABI delete / HostFn+1 / MIR distance change |
| Honesty | fully_converged 窄片 **≠** enable / **≠** B-09 `[x]` / **≠** GC GA / **≠** ObjectRef storage cleared |

## Frozen

- Enabling `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Deleting `MemoryPool` type / forcing full ObjectRef→ManagedHeap migrate this window
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / deleting `syx_rt_load_mir_blob` RT ABI
- Claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w490_*.plan.md`

## Next

W490 B-09 dual-heap converge 首片 **已交**（ADR-834）→ W491 B-09 fully_converged 窄片 **已交**（ADR-835）→ W492 B-09 ObjectRef/MemoryPool storage honesty thin **已交**（ADR-836）→ W493 B-09 enable-readiness thin **已交**（ADR-837）→ W494 B-09 storage-cleanup first-slice（死 API 退役）**已交**（ADR-838）→ W495 B-09 MemoryPool dead-shell first-slice **已交**（ADR-839）→ W496 B-09 ObjectRef RC migrate first-slice（窄）**已交**（ADR-840）→ W497 B-09 ObjectRef VM ManagedHeap probe first-slice **已交**（ADR-841）→ W498 B-09 ObjectRef VM heap_slots first-slice **已交**（ADR-842）→ W499 B-09 ObjectRef mir-run DictBuild ManagedHeap align first-slice **已交**（ADR-843）→ W500 B-09 ObjectRef VM CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-844）→ W501 B-09 ObjectRef mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-845）→ W502 B-09 MutRefCell VM DictBuild ManagedHeap + heap_slots first-slice **已交**（ADR-846）→ W503 B-09 MutRefCell mir-run DictBuild ManagedHeap + heap_slots align **已交**（ADR-847）→ W504 B-09 MutRefCell VM CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-848）→ W505 B-09 MutRefCell mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-849）→ W506 B-09 ObjectRef+MutRefCell VM call-return（finish_call）ManagedHeap + heap_slots first-slice **已交**（ADR-850）→ W507 B-09 ObjectRef+MutRefCell mir-run call-return（call_fn + call_closure）ManagedHeap + heap_slots first-slice **已交**（ADR-851）→ W508 B-09 ObjectRef+MutRefCell ActorCall（VM+mir-run runtime）ManagedHeap + heap_slots first-slice **已交**（ADR-852）→ W509 B-09 ObjectRef+MutRefCell MethodCall（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-853）→ W510 B-09 ObjectRefRetain catch-all（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-854）→ W511 B-09 LLVM·WASM ObjectRefRetain/Release emit-path 退役 first-slice **已交**（ADR-855）→ W512 B-09 ObjectRef ObjectGet residual re-entry + birth inventory ManagedHeap + heap_slots first-slice **已交**（ADR-856）→ W513 B-09 ObjectRef ArrayGet（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-857）→ W515 B-09 ObjectSet MutRefCell 写回（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-859）→ W516 B-09 LoadGlobal（VM+mir-run/SSA）ManagedHeap + heap_slots first-slice **已交**（ADR-860）→ W517 B-09 SSA LoadLocal ManagedHeap + heap_slots first-slice **已交**（ADR-861）→ W518 B-09 ObjectRef RC migrate-readiness thin **已交**（ADR-862）→ W519+ — 优先 Value/VM/mir-run RC 与 RT ABI retain/release/wrap **诚实退役** → 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
