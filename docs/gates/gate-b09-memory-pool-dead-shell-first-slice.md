# B-09 MemoryPool dead-shell first-slice (W495 / ADR-839)

## Status

**B-09 MemoryPool dead-shell first-slice met**: after W494 dead-API retire, delete unused `MemoryPool::alloc` and unused `VmInstance.pool` field; keep empty `MemoryPool` stub type name + live ObjectRef RC. This window does **not** flip `storage_cleanup_enabled`, migrate ObjectRef, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-forty-fifth` / `A-W495`.

## Index

| Prior | Status |
|-------|--------|
| W476–W493 narrative → readiness | **met** |
| W494 storage-cleanup first-slice (dead API) | **met** |
| This window MemoryPool dead-shell | **met** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| ObjectRef RC migrate / delete | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Cleanup ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i. Dead API retire | Delete `Gc::collect_mark_sweep` / `Gc::collect` | **met** (W494) |
| 4c-i-b. MemoryPool dead-shell | Delete unused `alloc` + `VmInstance.pool` | **met** (this window) |
| 4c-ii. Storage-layer cleanup | ObjectRef→ManagedHeap; flip storage cleanup enabled | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Checklist progress (hard)

| Item | Status | Meaning |
|------|--------|---------|
| Dead `Gc::collect_mark_sweep` API | **retired** | W494 |
| `MemoryPool::alloc` | **retired** | Deleted |
| `VmInstance.pool` field | **retired** | Deleted |
| `MemoryPool` type name (stub) | **retired** (W496) | Type deleted; honesty comments may retain name |
| ObjectRef retain/release | **still present** | Live RC |
| `storage_cleanup_enabled` | **false** | Full cleanup unmet |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09MemoryPoolDeadShellFirstSliceInventory` / `gate_b09_memory_pool_dead_shell_first_slice_inventory` |
| Flags | `b09_memory_pool_dead_shell_first_slice_signed=true`; `b09_storage_cleanup_first_slice_floor_ok=true`; `memory_pool_alloc_retired=true`; `vm_instance_pool_field_retired=true`; `objectref_rc_storage_still_present=true`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | flipping storage cleanup enabled / unified heap enabled / B-09 `[x]` / GC GA / wasm-gc / ObjectRef deleted / scan-all auto / RT ABI delete / HostFn+1 / MIR distance change |
| Honesty | dead-shell first-slice **≠** storage cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) — `MemoryPool` stub; no `fn alloc`; no `pool:` field; ObjectRef retained
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09MemoryPoolDeadShellFirstSliceInventory`
- Meta: `b09_memory_pool_dead_shell_first_slice.rs`
- Floor: [`gate-b09-storage-cleanup-first-slice.md`](gate-b09-storage-cleanup-first-slice.md) / [`gate-b09-enable-readiness-thin.md`](gate-b09-enable-readiness-thin.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Enabling `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Deleting live ObjectRef retain/release / migrating ObjectRef→ManagedHeap this window
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / deleting `syx_rt_load_mir_blob` RT ABI
- Claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w494_*.plan.md`

## Next

W494 B-09 storage-cleanup first-slice（死 API 退役）**已交**（ADR-838）→ W495 B-09 MemoryPool dead-shell first-slice **已交**（ADR-839）→ W496 B-09 ObjectRef RC migrate first-slice（窄）**已交**（ADR-840）→ W497 B-09 ObjectRef VM ManagedHeap probe first-slice **已交**（ADR-841）→ W498 B-09 ObjectRef VM heap_slots first-slice **已交**（ADR-842）→ W499 B-09 ObjectRef mir-run DictBuild ManagedHeap align first-slice **已交**（ADR-843）→ W500 B-09 ObjectRef VM CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-844）→ W501 B-09 ObjectRef mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-845）→ W502 B-09 MutRefCell VM DictBuild ManagedHeap + heap_slots first-slice **已交**（ADR-846）→ W503 B-09 MutRefCell mir-run DictBuild ManagedHeap + heap_slots align **已交**（ADR-847）→ W504 B-09 MutRefCell VM CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-848）→ W505 B-09 MutRefCell mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-849）→ W506 B-09 ObjectRef+MutRefCell VM call-return（finish_call）ManagedHeap + heap_slots first-slice **已交**（ADR-850）→ W507 B-09 ObjectRef+MutRefCell mir-run call-return（call_fn + call_closure）ManagedHeap + heap_slots first-slice **已交**（ADR-851）→ W508 B-09 ObjectRef+MutRefCell ActorCall（VM+mir-run runtime）ManagedHeap + heap_slots first-slice **已交**（ADR-852）→ W509 B-09 ObjectRef+MutRefCell MethodCall（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-853）→ W510 B-09 ObjectRefRetain catch-all（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-854）→ W511 B-09 LLVM·WASM ObjectRefRetain/Release emit-path 退役 first-slice **已交**（ADR-855）→ W512 B-09 ObjectRef ObjectGet residual re-entry + birth inventory ManagedHeap + heap_slots first-slice **已交**（ADR-856）→ W513 B-09 ObjectRef ArrayGet（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-857）→ W515 B-09 ObjectSet MutRefCell 写回（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-859）→ W516 B-09 LoadGlobal（VM+mir-run/SSA）ManagedHeap + heap_slots first-slice **已交**（ADR-860）→ W517 B-09 SSA LoadLocal ManagedHeap + heap_slots first-slice **已交**（ADR-861）→ W518 B-09 ObjectRef RC migrate-readiness thin **已交**（ADR-862）→ W519+ — 优先 Value/VM/mir-run RC 与 RT ABI retain/release/wrap **诚实退役** → 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
