# B-09 ObjectRef+MutRefCell VM call-return (`finish_call`) ManagedHeap + heap_slots first-slice (W506 / ADR-850)

## Status

**B-09 VM `finish_call` ObjectRef+MutRefCell ManagedHeap + heap_slots first-slice met**: after W505 MutRefCell mir-run CallHost track, VM nested-call success (`finish_call` after `stack.push(result)`) dual-registers stack-top `ObjectRef` and `MutRefCell` into ManagedHeap and pushes `heap_slots` (reuse W500/W504 helpers; identity-dedup; covers Call/init/closure returns and `@Lazy` materialize beyond CallHost). Throw unwind is unchanged. This window does **not** change DictBuild/CallHost tracks, change mir-run, enable MutRefCell/ObjectRef full migrate, flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-fifty-sixth` / `A-W506`.

## Index

| Prior | Status |
|-------|--------|
| W476–W505 narrative → ObjectRef/MutRefCell DictBuild + CallHost (VM+mir-run) | **met** |
| This window VM `finish_call` ObjectRef+MutRefCell track | **met** |
| mir-run call-return (`call_fn` + `call_closure`) | **met** (W507 / ADR-851) |
| ActorCall runtime (VM+mir-run) | **met** (W508 / ADR-852) |
| MethodCall | **met** (W509 / ADR-853) |
| ObjectRefRetain catch-all | **met** (W510 / ADR-854) |
| Full MutRefCell / ObjectRef→ManagedHeap migrate | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Cleanup ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-i | Dead API → ObjectRef tracks → MutRefCell DictBuild + CallHost (VM+mir-run) | **met** (W494–W505) |
| 4c-ii-j. VM `finish_call` track | Nested Call/init/closure / `@Lazy` return → ManagedHeap + `heap_slots` | **met** (this window) |
| 4c-ii. Storage-layer cleanup | Full ObjectRef/MutRefCell→ManagedHeap; flip cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Call-return track contract (hard)

| Item | Status | Meaning |
|------|--------|---------|
| `maybe_track_stack_top_objectref` after `finish_call` success | **wired** | Stack-top ObjectRef → `ManagedHeap::alloc` + `heap_slots.push` when enabled |
| `maybe_track_stack_top_mutrefcell` after `finish_call` success | **wired** | Stack-top MutRefCell → ManagedHeap + `heap_slots` (identity-dedup) |
| Throw unwind (`propagating_throw`) | **unchanged** | No track on throw path |
| DictBuild / CallHost ObjectRef+MutRefCell tracks | **retained** | Floor W497–W505 |
| mir-run CALL_FN / call-return | **met** (W507 / ADR-851) | `call_fn` + `call_closure` dual track |
| ActorCall runtime | **met** (W508 / ADR-852) | Reuses `finish_call` dual track |
| MethodCall | **met** (W509 / ADR-853) | Dispatch-site dual track |
| ObjectRefRetain catch-all | **unchanged** | W510+ |
| ObjectRef retain/release (all backends) | **still present** | Live RC |
| `objectref_rc_migrate_enabled` | **false** | Full migrate unmet |
| `storage_cleanup_enabled` | **false** | Full cleanup unmet |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectRefVmCallReturnHeapSlotsFirstSliceInventory` / `gate_b09_objectref_vm_call_return_heap_slots_first_slice_inventory` |
| Flags | `b09_objectref_vm_call_return_heap_slots_first_slice_signed=true`; `b09_mutrefcell_mir_run_callhost_heap_slots_first_slice_floor_ok=true`; `vm_finish_call_objectref_managed_heap_registered=true`; `vm_finish_call_mutrefcell_managed_heap_registered=true`; `vm_finish_call_heap_slots_tracked=true`; `objectref_rc_storage_still_present=true`; `llvm_wasm_objectref_rc_still_live=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | MutRefCell full migrate / flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / full ObjectRef deleted / scan-all auto / RT ABI delete / HostFn+1 / MIR distance change / LLVM·WASM RC retire |
| Honesty | VM `finish_call` track **≠** full migrate / **≠** mir-run call-return / **≠** ActorCall/MethodCall / **≠** storage cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) — `finish_call` success + `maybe_track_stack_top_objectref` / `maybe_track_stack_top_mutrefcell` / ADR-850
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectRefVmCallReturnHeapSlotsFirstSliceInventory`
- Meta: `b09_objectref_vm_call_return_heap_slots_first_slice.rs`
- Floor: [`gate-b09-mutrefcell-mir-run-callhost-heap-slots-first-slice.md`](gate-b09-mutrefcell-mir-run-callhost-heap-slots-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Full MutRefCell / ObjectRef→ManagedHeap migrate / deleting live retain/release this window / DictBuild-CallHost change / mir-run call-return / ActorCall / MethodCall / LLVM·WASM migrate
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / deleting `syx_rt_load_mir_blob` RT ABI
- Claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w505_*.plan.md`

## Next

W505 B-09 MutRefCell mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-849）→ W506 B-09 ObjectRef+MutRefCell VM call-return（finish_call）ManagedHeap + heap_slots first-slice **已交**（ADR-850）→ W507 B-09 ObjectRef+MutRefCell mir-run call-return（call_fn + call_closure）ManagedHeap + heap_slots first-slice **已交**（ADR-851）→ W508 B-09 ObjectRef+MutRefCell ActorCall（VM+mir-run runtime）ManagedHeap + heap_slots first-slice **已交**（ADR-852）→ W509 B-09 ObjectRef+MutRefCell MethodCall（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-853）→ W510 B-09 ObjectRefRetain catch-all（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-854）→ W511 B-09 LLVM·WASM ObjectRefRetain/Release emit-path 退役 first-slice **已交**（ADR-855）→ W512 B-09 ObjectRef ObjectGet residual re-entry + birth inventory ManagedHeap + heap_slots first-slice **已交**（ADR-856）→ W513 B-09 ObjectRef ArrayGet（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-857）→ W515 B-09 ObjectSet MutRefCell 写回（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-859）→ W516 B-09 LoadGlobal（VM+mir-run/SSA）ManagedHeap + heap_slots first-slice **已交**（ADR-860）→ W517 B-09 SSA LoadLocal ManagedHeap + heap_slots first-slice **已交**（ADR-861）→ W518 B-09 ObjectRef RC migrate-readiness thin **已交**（ADR-862）→ W519+ — 优先 Value/VM/mir-run RC 与 RT ABI retain/release/wrap **诚实退役** → 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
