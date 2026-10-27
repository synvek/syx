# B-09 ObjectRef+MutRefCell MethodCall（VM+mir-run）ManagedHeap + heap_slots first-slice (W509 / ADR-853)

## Status

**B-09 MethodCall ObjectRef+MutRefCell ManagedHeap + heap_slots first-slice met**: after W508 ActorCall runtime track, MethodCall / METHOD_CALL dispatch success dual-registers stack-top `ObjectRef` and `MutRefCell` into ManagedHeap and pushes `heap_slots` (CallHost-style dispatch-site helpers; covers HOF/HTTP/SQL/`call_receiver_method` Ok exits; identity-dedup). This window does **not** change ActorCall/DictBuild/CallHost/`finish_call`/`call_fn`/`call_closure`, enable ObjectRefRetain catch-all, flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-fifty-ninth` / `A-W509`.

## Index

| Prior | Status |
|-------|--------|
| W476–W508 narrative → ObjectRef/MutRefCell DictBuild + CallHost + call-return + ActorCall | **met** |
| This window MethodCall track (VM+mir-run) | **met** |
| ObjectRefRetain catch-all | **met** (W510 / ADR-854) |
| LLVM·WASM ObjectRef RC emit-path retire | **met** (W511 / ADR-855) |
| Full MutRefCell / ObjectRef→ManagedHeap migrate | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Cleanup ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-l | Dead API → tracks → finish_call / call_fn+call_closure / ActorCall runtime | **met** (W494–W508) |
| 4c-ii-m. MethodCall track | MethodCall return → ManagedHeap + `heap_slots` | **met** (this window) |
| 4c-ii. Storage-layer cleanup | Full ObjectRef/MutRefCell→ManagedHeap; flip cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## MethodCall track contract (hard)

| Item | Status | Meaning |
|------|--------|---------|
| VM MethodCall Ok → dual helpers | **wired** | Dispatch-site after `call_method` |
| mir-run METHOD_CALL Ok → dual helpers | **wired** | Parity with CALL_HOST / VM |
| ActorCall / DictBuild / CallHost / finish_call / call_fn / call_closure | **retained** | Floor W497–W508 |
| ObjectRefRetain catch-all | **met** (W510 / ADR-854) | Retain/Dup/LoadLocal dual track |
| LLVM·WASM ObjectRef RC retire | **unchanged** | W511+ |
| ObjectRef retain/release (all backends) | **still present** | Live RC |
| `objectref_rc_migrate_enabled` | **false** | Full migrate unmet |
| `storage_cleanup_enabled` | **false** | Full cleanup unmet |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectRefMethodCallHeapSlotsFirstSliceInventory` / `gate_b09_objectref_method_call_heap_slots_first_slice_inventory` |
| Flags | `b09_objectref_method_call_heap_slots_first_slice_signed=true`; `b09_objectref_actor_call_heap_slots_first_slice_floor_ok=true`; `vm_method_call_objectref_managed_heap_registered=true`; `vm_method_call_mutrefcell_managed_heap_registered=true`; `mir_run_method_call_objectref_managed_heap_registered=true`; `mir_run_method_call_mutrefcell_managed_heap_registered=true`; `vm_mir_run_method_call_heap_slots_tracked=true`; `objectref_rc_storage_still_present=true`; `llvm_wasm_objectref_rc_still_live=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | MutRefCell full migrate / flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / full ObjectRef deleted / scan-all auto / RT ABI delete / HostFn+1 / MIR distance change / LLVM·WASM RC retire |
| Honesty | MethodCall track **≠** LLVM·WASM RC retire / **≠** full migrate / **≠** storage cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA（ObjectRefRetain catch-all **met** in W510） |

## Evidence

- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) — MethodCall dispatch + dual helpers / ADR-853
- [`crates/syx-mir-run/src/runtime.rs`](../../crates/syx-mir-run/src/runtime.rs) — METHOD_CALL dispatch + dual helpers / ADR-853
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectRefMethodCallHeapSlotsFirstSliceInventory`
- Meta: `b09_objectref_method_call_heap_slots_first_slice.rs`
- Floor: [`gate-b09-objectref-actor-call-heap-slots-first-slice.md`](gate-b09-objectref-actor-call-heap-slots-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Full MutRefCell / ObjectRef→ManagedHeap migrate / deleting live retain/release this window / LLVM·WASM RC retire / `call_method`→`finish_call` refactor
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / deleting `syx_rt_load_mir_blob` RT ABI
- Claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w508_*.plan.md`

## Next

W508 B-09 ObjectRef+MutRefCell ActorCall（VM+mir-run runtime）ManagedHeap + heap_slots first-slice **已交**（ADR-852）→ W509 B-09 ObjectRef+MutRefCell MethodCall（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-853）→ W510 B-09 ObjectRefRetain catch-all（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-854）→ W511 B-09 LLVM·WASM ObjectRefRetain/Release emit-path 退役 first-slice **已交**（ADR-855）→ W512 B-09 ObjectRef ObjectGet residual re-entry + birth inventory ManagedHeap + heap_slots first-slice **已交**（ADR-856）→ W513 B-09 ObjectRef ArrayGet（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-857）→ W515 B-09 ObjectSet MutRefCell 写回（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-859）→ W516 B-09 LoadGlobal（VM+mir-run/SSA）ManagedHeap + heap_slots first-slice **已交**（ADR-860）→ W517 B-09 SSA LoadLocal ManagedHeap + heap_slots first-slice **已交**（ADR-861）→ W518 B-09 ObjectRef RC migrate-readiness thin **已交**（ADR-862）→ W519+ — 优先 Value/VM/mir-run RC 与 RT ABI retain/release/wrap **诚实退役** → 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
