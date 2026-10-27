# B-09 VM GcCollect root-set first-slice (W488 / ADR-832)

> **Status:** **VM GcCollect root-set first-slice signed** (ADR-832 / W488) — `VmInstance::collect_live_roots` wires `suspend_frame` + `locals_frames` into `GcCollect` / `HeapCompact` (compact rewrite syncs those frames).  
> Cross-index: [`gate-b09-mir-runtime-root-set-first-slice.md`](gate-b09-mir-runtime-root-set-first-slice.md) / ADR-831；[`gate-b09-gc-narrative-deepen.md`](gate-b09-gc-narrative-deepen.md) / ADR-829；[`gate-b09-gc-narrative-thin.md`](gate-b09-gc-narrative-thin.md) / ADR-820；[`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md).  
> Roadmap B-02 **`[x]`**；B-09 **`[~]`**（VM 根集首片 ≠ enable ≠ `[x]`）.  
> **Not** `unified_mir_runtime_heap_enabled=true`. **Not** actor mailbox Value-queue roots. **Not** VM↔MirRuntime heap fully converged. **Not** B-09 `[x]` / GC GA. **Not** scan-all auto / exports GA. **Not** RT ABI delete. **Not** MIR blob delete.

Fingerprint: `three-hundred-and-thirty-eighth` / `A-W488`.

## Scope narrow (ADR-832)

VM GcCollect root-set first-slice **complete** (narrow) =
- W476 thin + W485 deepen + W487 MirRuntime root-set floors
- This window: VM live roots include async `suspend_frame` + nested `locals_frames`; nested call paths promote outer locals to field `locals_frames`; compact rewrite syncs those frames
- Honesty: VM `actor_mailbox` remains depth-only (no Value message queue) → mailbox Value roots **still unmet**; dual heap **still not** fully converged

Unified heap product enable / B-09 `[x]` / GC GA = **unmet**.

## Acceptance

| Prerequisite | Status | Evidence |
|--------------|--------|----------|
| MirRuntime root-set first-slice | **met** (W487 / ADR-831) | [`gate-b09-mir-runtime-root-set-first-slice.md`](gate-b09-mir-runtime-root-set-first-slice.md) |
| VM GcCollect root-set first-slice | **signed** (W488 / ADR-832) | this document + `GateB09VmGcCollectRootSetFirstSliceInventory` |

**Evidence nails:** `crates/syx-driver/tests/native/gate_b09_vm_gccollect_root_set_first_slice.rs` + `gate_b09_vm_gccollect_root_set_first_slice_inventory`.

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09VmGcCollectRootSetFirstSliceInventory` / `gate_b09_vm_gccollect_root_set_first_slice_inventory` |
| Flags | `b09_vm_gccollect_root_set_first_slice_signed=true`; `b09_mir_runtime_root_set_first_slice_floor_ok=true`; `vm_suspend_frame_roots_wired=true`; `vm_locals_frames_roots_wired=true`; `actor_mailbox_value_queue_roots_wired=false`; `vm_mir_runtime_heap_fully_converged=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Honesty floor | W476/W485/W487 inventories keep historical product flags **false** |
| Forbidden claim | unified heap enabled / mailbox Value queue complete / dual heap converged / B-09 `[x]` / GC GA / wasm-gc / scan-all auto / RT ABI delete / HostFn+1 / MIR distance change |
| Honesty | VM GcCollect root-set first-slice **≠** unified heap enable / **≠** B-09 `[x]` / **≠** GC GA |

## Frozen

- Enabling `unified_mir_runtime_heap_enabled` / claiming mailbox Value-queue roots complete / claiming VM↔MirRuntime heap fully converged / rewriting three-layer table as “WASM/Native already has SYX GC”
- Claiming B-09 `[x]` / GC GA / wasm-gc
- Scan-all auto / exports GA / deleting `syx_rt_load_mir_blob` RT ABI
- Claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w487_*.plan.md`

## Next

W487 B-09 MirRuntime 根集首片 **已交**（ADR-831）→ W488 B-09 VM GcCollect 根集首片 **已交**（ADR-832）→ W489 B-09 mailbox Value 队列根集首片 **已交**（ADR-833）→ W490 B-09 dual-heap converge 首片 **已交**（ADR-834）→ W491 B-09 fully_converged 窄片 **已交**（ADR-835）→ W492 B-09 ObjectRef/MemoryPool storage honesty thin **已交**（ADR-836）→ W493 B-09 enable-readiness thin **已交**（ADR-837）→ W494 B-09 storage-cleanup first-slice（死 API 退役）**已交**（ADR-838）→ W495 B-09 MemoryPool dead-shell first-slice **已交**（ADR-839）→ W496 B-09 ObjectRef RC migrate first-slice（窄）**已交**（ADR-840）→ W497 B-09 ObjectRef VM ManagedHeap probe first-slice **已交**（ADR-841）→ W498 B-09 ObjectRef VM heap_slots first-slice **已交**（ADR-842）→ W499 B-09 ObjectRef mir-run DictBuild ManagedHeap align first-slice **已交**（ADR-843）→ W500 B-09 ObjectRef VM CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-844）→ W501 B-09 ObjectRef mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-845）→ W502 B-09 MutRefCell VM DictBuild ManagedHeap + heap_slots first-slice **已交**（ADR-846）→ W503 B-09 MutRefCell mir-run DictBuild ManagedHeap + heap_slots align **已交**（ADR-847）→ W504 B-09 MutRefCell VM CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-848）→ W505 B-09 MutRefCell mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-849）→ W506 B-09 ObjectRef+MutRefCell VM call-return（finish_call）ManagedHeap + heap_slots first-slice **已交**（ADR-850）→ W507 B-09 ObjectRef+MutRefCell mir-run call-return（call_fn + call_closure）ManagedHeap + heap_slots first-slice **已交**（ADR-851）→ W508 B-09 ObjectRef+MutRefCell ActorCall（VM+mir-run runtime）ManagedHeap + heap_slots first-slice **已交**（ADR-852）→ W509 B-09 ObjectRef+MutRefCell MethodCall（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-853）→ W510 B-09 ObjectRefRetain catch-all（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-854）→ W511 B-09 LLVM·WASM ObjectRefRetain/Release emit-path 退役 first-slice **已交**（ADR-855）→ W512 B-09 ObjectRef ObjectGet residual re-entry + birth inventory ManagedHeap + heap_slots first-slice **已交**（ADR-856）→ W513 B-09 ObjectRef ArrayGet（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-857）→ W515 B-09 ObjectSet MutRefCell 写回（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-859）→ W516 B-09 LoadGlobal（VM+mir-run/SSA）ManagedHeap + heap_slots first-slice **已交**（ADR-860）→ W517 B-09 SSA LoadLocal ManagedHeap + heap_slots first-slice **已交**（ADR-861）→ W518 B-09 ObjectRef RC migrate-readiness thin **已交**（ADR-862）→ W519+ — 优先 Value/VM/mir-run RC 与 RT ABI retain/release/wrap **诚实退役** → 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
