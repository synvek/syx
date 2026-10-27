# B-09 enable-readiness thin (W493 / ADR-837)

## Status

**B-09 enable-readiness thin met** (policy+inventory): after W492 ObjectRef/MemoryPool storage honesty thin, enable **preconditions** are signed — storage still present, cleanup off, three-layer table still documents no product SYX GC on WASM/Native, and enable blockers are documented. This window does **not** enable unified MirRuntime heap, run storage cleanup, delete MemoryPool, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-forty-third` / `A-W493`.

## Index

| Prior | Status |
|-------|--------|
| W476 B-09 GC narrative thin | **met** |
| W485 B-09 GC narrative deepen | **met** |
| W487–W489 root-set first-slices | **met** |
| W490 dual-heap converge first-slice | **met** |
| W491 fully_converged first-slice | **met** |
| W492 ObjectRef/MemoryPool storage honesty thin | **met** |
| This window enable-readiness | **met** |
| Storage cleanup / MemoryPool delete | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 1. Thin-open | Three-layer heap honesty | **met** (W476 / ADR-820) |
| 2. Deepen thin | Construction checklist | **met** (W485 / ADR-829) |
| 3. Root-set / dual-heap / fully_converged | Construction floors | **met** (W487–W491) |
| 4a. Storage honesty thin | Storage still present documented | **met** (W492 / ADR-836) |
| 4b. Enable-readiness | Blockers documented; preconditions signed | **met** (this window / ADR-837) |
| 4c. Storage cleanup / true enable | Axis construction | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Enable blocker table (hard)

| Blocker | Present? | Meaning |
|---------|----------|---------|
| ObjectRef / `MemoryPool` storage layer | **yes** | W492 `storage_still_present` |
| Storage cleanup | **no** | `storage_cleanup_enabled=false` |
| Unified MirRuntime heap product enable | **no** | `unified_mir_runtime_heap_enabled=false` |
| Three-layer table WASM/Native product SYX GC | **no** | narrative still no product SYX GC |
| `Gc::collect_mark_sweep` dead API | **yes（cosmetic）** | no callers; not the primary blocker |
| B-09 axis exit (`[x]`) / GC GA | **no** | readiness ≠ exit |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| Storage honesty floor | **yes** | W492 signed |
| Enable-readiness signed | **yes** | This window |
| Storage cleanup enabled | **no** | Still open |
| Unified heap enabled | **no** | Frozen |
| B-09 `[x]` / GC GA | **no** | readiness ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09EnableReadinessThinInventory` / `gate_b09_enable_readiness_thin_inventory` |
| Flags | `b09_enable_readiness_signed=true`; `b09_objectref_memory_pool_storage_honesty_thin_floor_ok=true`; `enable_blockers_documented=true`; `vm_memory_pool_objectref_storage_still_present=true`; `gc_collect_mark_sweep_dead_api_documented=true`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | unified heap enabled / storage cleanup / B-09 `[x]` / GC GA / wasm-gc / MemoryPool deleted / scan-all auto / RT ABI delete / HostFn+1 / MIR distance change |
| Honesty | enable-readiness **≠** enable / **≠** cleanup / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09EnableReadinessThinInventory`
- Meta: ~~`b09_enable_readiness_thin.rs`~~ retired（F-10 W597）；proof = this SSOT + `GateB09EnableReadinessThinInventory`
- Floor: [`gate-b09-objectref-memory-pool-storage-honesty-thin.md`](gate-b09-objectref-memory-pool-storage-honesty-thin.md) / [`gate-b09-vm-mir-runtime-heap-fully-converged-first-slice.md`](gate-b09-vm-mir-runtime-heap-fully-converged-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Enabling `unified_mir_runtime_heap_enabled` / `storage_cleanup_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Deleting `MemoryPool` type / migrating ObjectRef / deleting dead `Gc::collect_mark_sweep` this window
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / deleting `syx_rt_load_mir_blob` RT ABI
- Claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w492_*.plan.md`

## Next

W492 B-09 ObjectRef/MemoryPool storage honesty thin **已交**（ADR-836）→ W493 B-09 enable-readiness thin **已交**（ADR-837）→ W494 B-09 storage-cleanup first-slice（死 API 退役）**已交**（ADR-838）→ W495 B-09 MemoryPool dead-shell first-slice **已交**（ADR-839）→ W496 B-09 ObjectRef RC migrate first-slice（窄）**已交**（ADR-840）→ W497 B-09 ObjectRef VM ManagedHeap probe first-slice **已交**（ADR-841）→ W498 B-09 ObjectRef VM heap_slots first-slice **已交**（ADR-842）→ W499 B-09 ObjectRef mir-run DictBuild ManagedHeap align first-slice **已交**（ADR-843）→ W500 B-09 ObjectRef VM CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-844）→ W501 B-09 ObjectRef mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-845）→ W502 B-09 MutRefCell VM DictBuild ManagedHeap + heap_slots first-slice **已交**（ADR-846）→ W503 B-09 MutRefCell mir-run DictBuild ManagedHeap + heap_slots align **已交**（ADR-847）→ W504 B-09 MutRefCell VM CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-848）→ W505 B-09 MutRefCell mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-849）→ W506 B-09 ObjectRef+MutRefCell VM call-return（finish_call）ManagedHeap + heap_slots first-slice **已交**（ADR-850）→ W507 B-09 ObjectRef+MutRefCell mir-run call-return（call_fn + call_closure）ManagedHeap + heap_slots first-slice **已交**（ADR-851）→ W508 B-09 ObjectRef+MutRefCell ActorCall（VM+mir-run runtime）ManagedHeap + heap_slots first-slice **已交**（ADR-852）→ W509 B-09 ObjectRef+MutRefCell MethodCall（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-853）→ W510 B-09 ObjectRefRetain catch-all（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-854）→ W511 B-09 LLVM·WASM ObjectRefRetain/Release emit-path 退役 first-slice **已交**（ADR-855）→ W512 B-09 ObjectRef ObjectGet residual re-entry + birth inventory ManagedHeap + heap_slots first-slice **已交**（ADR-856）→ W513 B-09 ObjectRef ArrayGet（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-857）→ W515 B-09 ObjectSet MutRefCell 写回（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-859）→ W516 B-09 LoadGlobal（VM+mir-run/SSA）ManagedHeap + heap_slots first-slice **已交**（ADR-860）→ W517 B-09 SSA LoadLocal ManagedHeap + heap_slots first-slice **已交**（ADR-861）→ W518 B-09 ObjectRef RC migrate-readiness thin **已交**（ADR-862）→ W519+ — 优先 Value/VM/mir-run RC 与 RT ABI retain/release/wrap **诚实退役** → 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
