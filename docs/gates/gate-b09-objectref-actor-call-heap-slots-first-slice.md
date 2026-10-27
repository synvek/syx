# B-09 ObjectRef+MutRefCell ActorCall（VM+mir-run runtime）ManagedHeap + heap_slots first-slice (W508 / ADR-852)

## Status

**B-09 ActorCall runtime ObjectRef+MutRefCell ManagedHeap + heap_slots first-slice met**: after W507 mir-run call-return track, ActorCall **with runtime** success dual-registers stack-top `ObjectRef` and `MutRefCell` into ManagedHeap and pushes `heap_slots` (VM reuses `finish_call` / W506 helpers; mir-run reuses `maybe_track_stack_top_*`; identity-dedup). ActorCall **without runtime** already met via VM `finish_call` (W506) and mir-run `call_fn` (W507). This window does **not** change MethodCall, DictBuild/CallHost/`finish_call`/`call_fn`/`call_closure` floors, enable full migrate, flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-fifty-eighth` / `A-W508`.

## Index

| Prior | Status |
|-------|--------|
| W476–W507 narrative → ObjectRef/MutRefCell DictBuild + CallHost + call-return | **met** |
| This window ActorCall runtime track (VM+mir-run) | **met** |
| ActorCall no-runtime | **met** (floor W506/W507) |
| MethodCall | **met** (W509 / ADR-853) |
| ObjectRefRetain catch-all | **met** (W510 / ADR-854) |
| Full MutRefCell / ObjectRef→ManagedHeap migrate | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Cleanup ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-k | Dead API → tracks → finish_call / call_fn+call_closure | **met** (W494–W507) |
| 4c-ii-l. ActorCall runtime track | Runtime ActorCall return → ManagedHeap + `heap_slots` | **met** (this window) |
| 4c-ii. Storage-layer cleanup | Full ObjectRef/MutRefCell→ManagedHeap; flip cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## ActorCall track contract (hard)

| Item | Status | Meaning |
|------|--------|---------|
| VM ActorCall + runtime Ok → `finish_call` | **wired** | Reuse W506 dual track |
| mir-run `actor_call` + runtime Ok → dual helpers | **wired** | Stack-top ObjectRef+MutRefCell → ManagedHeap + `heap_slots` |
| ActorCall no-runtime | **floor** | VM `finish_call` / mir-run `call_fn` already track |
| MethodCall | **met** (W509 / ADR-853) | Dispatch-site dual track |
| ObjectRefRetain catch-all | **unchanged** | W510+ |
| DictBuild / CallHost / finish_call / call_fn / call_closure | **retained** | Floor W497–W507 |
| ObjectRef retain/release (all backends) | **still present** | Live RC |
| `objectref_rc_migrate_enabled` | **false** | Full migrate unmet |
| `storage_cleanup_enabled` | **false** | Full cleanup unmet |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectRefActorCallHeapSlotsFirstSliceInventory` / `gate_b09_objectref_actor_call_heap_slots_first_slice_inventory` |
| Flags | `b09_objectref_actor_call_heap_slots_first_slice_signed=true`; `b09_objectref_mir_run_call_return_heap_slots_first_slice_floor_ok=true`; `vm_actor_call_objectref_managed_heap_registered=true`; `vm_actor_call_mutrefcell_managed_heap_registered=true`; `mir_run_actor_call_objectref_managed_heap_registered=true`; `mir_run_actor_call_mutrefcell_managed_heap_registered=true`; `vm_mir_run_actor_call_heap_slots_tracked=true`; `objectref_rc_storage_still_present=true`; `llvm_wasm_objectref_rc_still_live=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | MethodCall done / MutRefCell full migrate / flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / full ObjectRef deleted / scan-all auto / RT ABI delete / HostFn+1 / MIR distance change / LLVM·WASM RC retire |
| Honesty | ActorCall runtime track **≠** MethodCall / **≠** full migrate / **≠** storage cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) — ActorCall runtime Ok → `finish_call` / ADR-852
- [`crates/syx-mir-run/src/runtime.rs`](../../crates/syx-mir-run/src/runtime.rs) — `actor_call` runtime Ok + `maybe_track_stack_top_objectref` / `maybe_track_stack_top_mutrefcell` / ADR-852
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectRefActorCallHeapSlotsFirstSliceInventory`
- Meta: `b09_objectref_actor_call_heap_slots_first_slice.rs`
- Floor: [`gate-b09-objectref-mir-run-call-return-heap-slots-first-slice.md`](gate-b09-objectref-mir-run-call-return-heap-slots-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Full MutRefCell / ObjectRef→ManagedHeap migrate / deleting live retain/release this window / MethodCall change / LLVM·WASM migrate
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / deleting `syx_rt_load_mir_blob` RT ABI
- Claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w507_*.plan.md`

## Next

W507 B-09 ObjectRef+MutRefCell mir-run call-return（call_fn + call_closure）ManagedHeap + heap_slots first-slice **已交**（ADR-851）→ W508 B-09 ObjectRef+MutRefCell ActorCall（VM+mir-run runtime）ManagedHeap + heap_slots first-slice **已交**（ADR-852）→ W509 B-09 ObjectRef+MutRefCell MethodCall（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-853）→ W510 B-09 ObjectRefRetain catch-all（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-854）→ W511 B-09 LLVM·WASM ObjectRefRetain/Release emit-path 退役 first-slice **已交**（ADR-855）→ W512 B-09 ObjectRef ObjectGet residual re-entry + birth inventory ManagedHeap + heap_slots first-slice **已交**（ADR-856）→ W513 B-09 ObjectRef ArrayGet（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-857）→ W515 B-09 ObjectSet MutRefCell 写回（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-859）→ W516 B-09 LoadGlobal（VM+mir-run/SSA）ManagedHeap + heap_slots first-slice **已交**（ADR-860）→ W517 B-09 SSA LoadLocal ManagedHeap + heap_slots first-slice **已交**（ADR-861）→ W518 B-09 ObjectRef RC migrate-readiness thin **已交**（ADR-862）→ W519+ — 优先 Value/VM/mir-run RC 与 RT ABI retain/release/wrap **诚实退役** → 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
