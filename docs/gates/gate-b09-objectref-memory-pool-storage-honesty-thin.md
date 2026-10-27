# B-09 ObjectRef/MemoryPool storage honesty thin (W492 / ADR-836)

> **Status:** **ObjectRef/MemoryPool storage honesty thin signed** (ADR-836 / W492) — documents that VM `MemoryPool` + `ObjectRef` RC storage **still present**; cleanup **not enabled**; blocks honest product enable.  
> Cross-index: [`gate-b09-vm-mir-runtime-heap-fully-converged-first-slice.md`](gate-b09-vm-mir-runtime-heap-fully-converged-first-slice.md) / ADR-835；[`gate-b09-gc-narrative-thin.md`](gate-b09-gc-narrative-thin.md) / ADR-820；[`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md).  
> Roadmap B-02 **`[x]`**；B-09 **`[~]`**（storage honesty thin ≠ enable ≠ cleanup ≠ `[x]`）.  
> **Not** `storage_cleanup_enabled=true`. **Not** `unified_mir_runtime_heap_enabled=true`. **Not** B-09 `[x]` / GC GA. **Not** MemoryPool type deleted. **Not** scan-all auto / exports GA. **Not** RT ABI delete. **Not** MIR blob delete.

Fingerprint: `three-hundred-and-forty-second` / `A-W492`.

## Storage layer (ADR-836)

| Layer | Status this window |
|-------|--------------------|
| GcCollect path (ManagedHeap-only) | **met** (W491 fully_converged narrow) |
| `MemoryPool` type + ObjectRef RC | **Still present** |
| Storage cleanup / migrate to ManagedHeap | **still unmet** (`storage_cleanup_enabled=false`) |
| Product enable / B-09 `[x]` / GC GA | **still unmet** |

## Scope narrow (ADR-836)

Storage honesty thin **complete** (narrow) =
- W476–W491 floors (incl. fully_converged first-slice)
- This window: policy+inventory document storage still present + cleanup off + blocks enable
- **No** production path change; **no** MemoryPool delete
- Honesty: enable / cleanup / B-09 `[x]` / GC GA **still unmet**

## Acceptance

| Prerequisite | Status | Evidence |
|--------------|--------|----------|
| Fully_converged first-slice | **met** (W491 / ADR-835) | [`gate-b09-vm-mir-runtime-heap-fully-converged-first-slice.md`](gate-b09-vm-mir-runtime-heap-fully-converged-first-slice.md) |
| Storage honesty thin | **signed** (W492 / ADR-836) | this document + `GateB09ObjectRefMemoryPoolStorageHonestyThinInventory` |

**Evidence nails:** ~~`gate_b09_objectref_memory_pool_storage_honesty_thin.rs`~~ retired（F-10 W598）；proof = this SSOT + `GateB09ObjectRefMemoryPoolStorageHonestyThinInventory`.

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectRefMemoryPoolStorageHonestyThinInventory` / `gate_b09_objectref_memory_pool_storage_honesty_thin_inventory` |
| Flags | `b09_objectref_memory_pool_storage_honesty_thin_signed=true`; `b09_vm_mir_runtime_heap_fully_converged_first_slice_floor_ok=true`; `objectref_memory_pool_storage_documented=true`; `vm_memory_pool_objectref_storage_still_present=true`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Honesty floor | W476–W491 inventories keep historical product flags |
| Forbidden claim | storage cleanup / unified heap enabled / B-09 `[x]` / GC GA / wasm-gc / MemoryPool deleted / scan-all auto / RT ABI delete / HostFn+1 / MIR distance change |
| Honesty | storage honesty thin **≠** enable / **≠** cleanup / **≠** B-09 `[x]` / **≠** GC GA |

## Frozen

- Enabling `unified_mir_runtime_heap_enabled` / `storage_cleanup_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Deleting `MemoryPool` type / forcing ObjectRef→ManagedHeap migrate this window
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / deleting `syx_rt_load_mir_blob` RT ABI
- Claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w491_*.plan.md`

## Next

W491 B-09 fully_converged 窄片 **已交**（ADR-835）→ W492 B-09 ObjectRef/MemoryPool storage honesty thin **已交**（ADR-836）→ W493 B-09 enable-readiness thin **已交**（ADR-837）→ W494 B-09 storage-cleanup first-slice（死 API 退役）**已交**（ADR-838）→ W495 B-09 MemoryPool dead-shell first-slice **已交**（ADR-839）→ W496 B-09 ObjectRef RC migrate first-slice（窄）**已交**（ADR-840）→ W497 B-09 ObjectRef VM ManagedHeap probe first-slice **已交**（ADR-841）→ W498 B-09 ObjectRef VM heap_slots first-slice **已交**（ADR-842）→ W499 B-09 ObjectRef mir-run DictBuild ManagedHeap align first-slice **已交**（ADR-843）→ W500 B-09 ObjectRef VM CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-844）→ W501 B-09 ObjectRef mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-845）→ W502 B-09 MutRefCell VM DictBuild ManagedHeap + heap_slots first-slice **已交**（ADR-846）→ W503 B-09 MutRefCell mir-run DictBuild ManagedHeap + heap_slots align **已交**（ADR-847）→ W504 B-09 MutRefCell VM CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-848）→ W505 B-09 MutRefCell mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-849）→ W506 B-09 ObjectRef+MutRefCell VM call-return（finish_call）ManagedHeap + heap_slots first-slice **已交**（ADR-850）→ W507 B-09 ObjectRef+MutRefCell mir-run call-return（call_fn + call_closure）ManagedHeap + heap_slots first-slice **已交**（ADR-851）→ W508 B-09 ObjectRef+MutRefCell ActorCall（VM+mir-run runtime）ManagedHeap + heap_slots first-slice **已交**（ADR-852）→ W509 B-09 ObjectRef+MutRefCell MethodCall（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-853）→ W510 B-09 ObjectRefRetain catch-all（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-854）→ W511 B-09 LLVM·WASM ObjectRefRetain/Release emit-path 退役 first-slice **已交**（ADR-855）→ W512 B-09 ObjectRef ObjectGet residual re-entry + birth inventory ManagedHeap + heap_slots first-slice **已交**（ADR-856）→ W513 B-09 ObjectRef ArrayGet（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-857）→ W515 B-09 ObjectSet MutRefCell 写回（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-859）→ W516 B-09 LoadGlobal（VM+mir-run/SSA）ManagedHeap + heap_slots first-slice **已交**（ADR-860）→ W517 B-09 SSA LoadLocal ManagedHeap + heap_slots first-slice **已交**（ADR-861）→ W518 B-09 ObjectRef RC migrate-readiness thin **已交**（ADR-862）→ W519+ — 优先 Value/VM/mir-run RC 与 RT ABI retain/release/wrap **诚实退役** → 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
