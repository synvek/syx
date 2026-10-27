# B-09 ObjectRef RC migrate first-slice (W496 / ADR-840)

## Status

**B-09 ObjectRef RC migrate first-slice met** (narrow): after W495 MemoryPool dead-shell, delete empty `MemoryPool` type stub and document ObjectRef RC migrate surface across backends. This window does **not** enable full ObjectRef→ManagedHeap migrate, flip `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-forty-sixth` / `A-W496`.

## Index

| Prior | Status |
|-------|--------|
| W476–W494 narrative → dead-API | **met** |
| W495 MemoryPool dead-shell | **met** |
| This window ObjectRef migrate first-slice (narrow) | **met** |
| Full ObjectRef→ManagedHeap migrate | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Cleanup ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i. Dead API retire | Delete `Gc::collect_mark_sweep` / `Gc::collect` | **met** (W494) |
| 4c-i-b. MemoryPool dead-shell | Delete unused `alloc` + `VmInstance.pool` | **met** (W495) |
| 4c-i-c. MemoryPool type stub retire + migrate surface | Delete empty type; document ObjectRef surface | **met** (this window) |
| 4c-ii. Storage-layer cleanup | Full ObjectRef→ManagedHeap; flip storage cleanup enabled | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## ObjectRef migrate surface (hard)

| Layer | Symbols / path | Status |
|-------|----------------|--------|
| Value RC | `Value::ObjectRef` / `object_ref_retain` / `object_ref_release` (`syx-runtime-core`) | **still live** |
| VM bytecode | `ObjectRefRetain` / `ObjectRefRelease`; LoadLocal/Dup/StoreLocal implicit RC | **still live** |
| mir-run | retain/release + StoreLocal/LoadLocal | **still live** |
| RT ABI | `syx_rt_object_ref_retain` / `syx_rt_object_ref_release` | **still live** |
| LLVM | `call @syx_rt_object_ref_*` | **still live** |
| WASM | AOT imports + host retain/drop | **still live** |

Documenting this surface **≠** migrate completed.

## Checklist progress (hard)

| Item | Status | Meaning |
|------|--------|---------|
| `MemoryPool::alloc` / `VmInstance.pool` | **retired** | W495 |
| `MemoryPool` type stub | **retired** | Deleted this window |
| ObjectRef retain/release (all backends) | **still present** | Live RC |
| Migrate surface documented | **met** | This window |
| Full ObjectRef→ManagedHeap | **false** | `objectref_rc_migrate_enabled=false` |
| `storage_cleanup_enabled` | **false** | Full cleanup unmet |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectRefRcMigrateFirstSliceInventory` / `gate_b09_objectref_rc_migrate_first_slice_inventory` |
| Flags | `b09_objectref_rc_migrate_first_slice_signed=true`; `b09_memory_pool_dead_shell_first_slice_floor_ok=true`; `memory_pool_type_stub_retired=true`; `objectref_migrate_surface_documented=true`; `objectref_rc_storage_still_present=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | flipping storage cleanup enabled / unified heap enabled / B-09 `[x]` / GC GA / wasm-gc / full ObjectRef deleted / scan-all auto / RT ABI delete / HostFn+1 / MIR distance change |
| Honesty | migrate first-slice (narrow) **≠** full migrate / **≠** storage cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) — no `pub struct MemoryPool`; ObjectRef retained; ADR-840
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectRefRcMigrateFirstSliceInventory`
- Meta: `b09_objectref_rc_migrate_first_slice.rs`
- Floor: [`gate-b09-memory-pool-dead-shell-first-slice.md`](gate-b09-memory-pool-dead-shell-first-slice.md) / [`gate-b09-storage-cleanup-first-slice.md`](gate-b09-storage-cleanup-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Enabling `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Full ObjectRef→ManagedHeap migrate / deleting live retain/release this window
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / deleting `syx_rt_load_mir_blob` RT ABI
- Claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w495_*.plan.md`

## Next

W495 B-09 MemoryPool dead-shell first-slice **已交**（ADR-839）→ W496 B-09 ObjectRef RC migrate first-slice（窄）**已交**（ADR-840）→ W497 B-09 ObjectRef VM ManagedHeap probe first-slice **已交**（ADR-841）→ W498 B-09 ObjectRef VM heap_slots first-slice **已交**（ADR-842）→ W499 B-09 ObjectRef mir-run DictBuild ManagedHeap align first-slice **已交**（ADR-843）→ W500 B-09 ObjectRef VM CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-844）→ W501 B-09 ObjectRef mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-845）→ W502 B-09 MutRefCell VM DictBuild ManagedHeap + heap_slots first-slice **已交**（ADR-846）→ W503 B-09 MutRefCell mir-run DictBuild ManagedHeap + heap_slots align **已交**（ADR-847）→ W504 B-09 MutRefCell VM CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-848）→ W505 B-09 MutRefCell mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-849）→ W506 B-09 ObjectRef+MutRefCell VM call-return（finish_call）ManagedHeap + heap_slots first-slice **已交**（ADR-850）→ W507 B-09 ObjectRef+MutRefCell mir-run call-return（call_fn + call_closure）ManagedHeap + heap_slots first-slice **已交**（ADR-851）→ W508 B-09 ObjectRef+MutRefCell ActorCall（VM+mir-run runtime）ManagedHeap + heap_slots first-slice **已交**（ADR-852）→ W509 B-09 ObjectRef+MutRefCell MethodCall（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-853）→ W510 B-09 ObjectRefRetain catch-all（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-854）→ W511 B-09 LLVM·WASM ObjectRefRetain/Release emit-path 退役 first-slice **已交**（ADR-855）→ W512 B-09 ObjectRef ObjectGet residual re-entry + birth inventory ManagedHeap + heap_slots first-slice **已交**（ADR-856）→ W513 B-09 ObjectRef ArrayGet（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-857）→ W515 B-09 ObjectSet MutRefCell 写回（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-859）→ W516 B-09 LoadGlobal（VM+mir-run/SSA）ManagedHeap + heap_slots first-slice **已交**（ADR-860）→ W517 B-09 SSA LoadLocal ManagedHeap + heap_slots first-slice **已交**（ADR-861）→ W518 B-09 ObjectRef RC migrate-readiness thin **已交**（ADR-862）→ W519+ — 优先 Value/VM/mir-run RC 与 RT ABI retain/release/wrap **诚实退役** → 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
