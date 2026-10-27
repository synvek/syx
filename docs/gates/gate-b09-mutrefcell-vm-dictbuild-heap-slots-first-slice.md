# B-09 MutRefCell VM DictBuild ManagedHeap + heap_slots first-slice (W502 / ADR-846)

## Status

**B-09 MutRefCell VM DictBuild ManagedHeap + heap_slots first-slice met**: after W501 mir-run CallHost ObjectRef track, VM `DictBuild` `__type==MutRefCell` dual-registers into ManagedHeap and pushes `heap_slots` (identity-dedup via `mut_ref_cell_identity`; aligned with ObjectRef DictBuild W497/W498). This window does **not** enable MutRefCell full migrate, flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-fifty-second` / `A-W502`.

## Index

| Prior | Status |
|-------|--------|
| W476–W501 narrative → ObjectRef CallHost tracks | **met** |
| This window MutRefCell VM DictBuild track | **met** |
| mir-run MutRefCell DictBuild align | **still unmet** |
| Full MutRefCell / ObjectRef→ManagedHeap migrate | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Cleanup ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-e | Dead API → ObjectRef probe/slots/DictBuild/CallHost (VM+mir-run) | **met** (W494–W501) |
| 4c-ii-f. MutRefCell VM DictBuild track | DictBuild MutRefCell → ManagedHeap + `heap_slots` (identity-dedup) | **met** (this window) |
| 4c-ii. Storage-layer cleanup | Full ObjectRef/MutRefCell→ManagedHeap; flip cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## DictBuild MutRefCell track contract (hard)

| Item | Status | Meaning |
|------|--------|---------|
| `maybe_track_mutrefcell_value` after DictBuild MutRefCell | **wired** | MutRefCell → `ManagedHeap::alloc` + `heap_slots.push` when enabled |
| Same SharedCell Arc identity already in `heap_slots` | **deduped** | Avoid duplicate alloc |
| ObjectRef DictBuild / CallHost helpers | **unchanged** | Floor W497–W501 |
| mir-run MutRefCell DictBuild branch | **unchanged** | Deferred W503+ |
| ObjectRef retain/release (all backends) | **still present** | Live RC |
| `objectref_rc_migrate_enabled` | **false** | Full migrate unmet |
| `storage_cleanup_enabled` | **false** | Full cleanup unmet |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09MutRefCellVmDictbuildHeapSlotsFirstSliceInventory` / `gate_b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_inventory` |
| Flags | `b09_mutrefcell_vm_dictbuild_heap_slots_first_slice_signed=true`; `b09_objectref_mir_run_callhost_heap_slots_first_slice_floor_ok=true`; `vm_dictbuild_mutrefcell_managed_heap_registered=true`; `vm_dictbuild_mutrefcell_heap_slots_tracked=true`; `objectref_rc_storage_still_present=true`; `llvm_wasm_objectref_rc_still_live=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | MutRefCell full migrate / flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / full ObjectRef deleted / scan-all auto / RT ABI delete / HostFn+1 / MIR distance change |
| Honesty | MutRefCell VM DictBuild track **≠** full MutRefCell migrate / **≠** full ObjectRef migrate / **≠** storage cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) — DictBuild MutRefCell + `maybe_track_mutrefcell_value` / `mut_ref_cell_identity` dedup / ADR-846
- [`crates/syx-runtime-core/src/value.rs`](../../crates/syx-runtime-core/src/value.rs) — `mut_ref_cell_identity`
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09MutRefCellVmDictbuildHeapSlotsFirstSliceInventory`
- Meta: `b09_mutrefcell_vm_dictbuild_heap_slots_first_slice.rs`
- Floor: [`gate-b09-objectref-mir-run-callhost-heap-slots-first-slice.md`](gate-b09-objectref-mir-run-callhost-heap-slots-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Full MutRefCell / ObjectRef→ManagedHeap migrate / deleting live retain/release this window / mir-run MutRefCell change / LLVM·WASM migrate
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / deleting `syx_rt_load_mir_blob` RT ABI
- Claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w501_*.plan.md`

## Next

W501 B-09 ObjectRef mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-845）→ W502 B-09 MutRefCell VM DictBuild ManagedHeap + heap_slots first-slice **已交**（ADR-846）→ W503 B-09 MutRefCell mir-run DictBuild ManagedHeap + heap_slots align **已交**（ADR-847）→ W504 B-09 MutRefCell VM CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-848）→ W505 B-09 MutRefCell mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-849）→ W506 B-09 ObjectRef+MutRefCell VM call-return（finish_call）ManagedHeap + heap_slots first-slice **已交**（ADR-850）→ W507 B-09 ObjectRef+MutRefCell mir-run call-return（call_fn + call_closure）ManagedHeap + heap_slots first-slice **已交**（ADR-851）→ W508 B-09 ObjectRef+MutRefCell ActorCall（VM+mir-run runtime）ManagedHeap + heap_slots first-slice **已交**（ADR-852）→ W509 B-09 ObjectRef+MutRefCell MethodCall（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-853）→ W510 B-09 ObjectRefRetain catch-all（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-854）→ W511 B-09 LLVM·WASM ObjectRefRetain/Release emit-path 退役 first-slice **已交**（ADR-855）→ W512 B-09 ObjectRef ObjectGet residual re-entry + birth inventory ManagedHeap + heap_slots first-slice **已交**（ADR-856）→ W513 B-09 ObjectRef ArrayGet（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-857）→ W515 B-09 ObjectSet MutRefCell 写回（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-859）→ W516 B-09 LoadGlobal（VM+mir-run/SSA）ManagedHeap + heap_slots first-slice **已交**（ADR-860）→ W517 B-09 SSA LoadLocal ManagedHeap + heap_slots first-slice **已交**（ADR-861）→ W518 B-09 ObjectRef RC migrate-readiness thin **已交**（ADR-862）→ W519+ — 优先 Value/VM/mir-run RC 与 RT ABI retain/release/wrap **诚实退役** → 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
