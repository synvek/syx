# B-09 ObjectRef mir-run DictBuild ManagedHeap align first-slice (W499 / ADR-843)

## Status

**B-09 ObjectRef mir-run DictBuild ManagedHeap align first-slice met**: after W498 VM heap_slots, mir-run `DictBuild` `__type` ObjectRef path dual-registers into ManagedHeap and pushes `heap_slots` (aligned with `object_ref_wrap` and VM W497/W498). This window does **not** enable full ObjectRef→ManagedHeap migrate, flip `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-forty-ninth` / `A-W499`.

## Index

| Prior | Status |
|-------|--------|
| W476–W497 narrative → ManagedHeap probe | **met** |
| W498 ObjectRef VM heap_slots | **met** |
| This window mir-run DictBuild align | **met** |
| Full ObjectRef→ManagedHeap migrate | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Cleanup ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-b | Dead API → probe → VM heap_slots | **met** (W494–W498) |
| 4c-ii-c. mir-run DictBuild align | DictBuild `__type` → ManagedHeap + `heap_slots` | **met** (this window) |
| 4c-ii. Storage-layer cleanup | Full ObjectRef→ManagedHeap; flip cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Align contract (hard)

| Item | Status | Meaning |
|------|--------|---------|
| mir-run DictBuild `__type` → `object_ref_from_map` | **aligned** | Then `ManagedHeap::alloc` + `heap_slots.push` when enabled |
| mir-run `object_ref_wrap` | **unchanged** | Prior alignment target |
| VM DictBuild heap_slots (W498) | **floor** | Unchanged this window |
| MutRefCell DictBuild branch | **unchanged** | Frozen |
| ObjectRef retain/release (all backends) | **still present** | Live RC |
| `objectref_rc_migrate_enabled` | **false** | Full migrate unmet |
| `storage_cleanup_enabled` | **false** | Full cleanup unmet |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectRefMirRunDictbuildAlignFirstSliceInventory` / `gate_b09_objectref_mir_run_dictbuild_align_first_slice_inventory` |
| Flags | `b09_objectref_mir_run_dictbuild_align_first_slice_signed=true`; `b09_objectref_vm_heap_slots_first_slice_floor_ok=true`; `mir_run_dictbuild_objectref_managed_heap_registered=true`; `mir_run_dictbuild_objectref_heap_slots_tracked=true`; `objectref_rc_storage_still_present=true`; `llvm_wasm_objectref_rc_still_live=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / full ObjectRef deleted / scan-all auto / RT ABI delete / HostFn+1 / MIR distance change |
| Honesty | mir-run DictBuild align **≠** full migrate / **≠** storage cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-mir-run/src/runtime.rs`](../../crates/syx-mir-run/src/runtime.rs) — DictBuild `__type` + `ManagedHeap` / `heap.alloc` / `heap_slots.push` / ADR-843
- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) — VM DictBuild floor (W497/W498)
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectRefMirRunDictbuildAlignFirstSliceInventory`
- Meta: `b09_objectref_mir_run_dictbuild_align_first_slice.rs`
- Floor: [`gate-b09-objectref-vm-heap-slots-first-slice.md`](gate-b09-objectref-vm-heap-slots-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Full ObjectRef→ManagedHeap migrate / deleting live retain/release this window / MutRefCell migrate / LLVM·WASM migrate
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / deleting `syx_rt_load_mir_blob` RT ABI
- Claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w498_*.plan.md`

## Next

W499 B-09 ObjectRef mir-run DictBuild ManagedHeap align first-slice **已交**（ADR-843）→ W500 B-09 ObjectRef VM CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-844）→ W501 已交（ADR-845）→ W502 B-09 MutRefCell VM DictBuild ManagedHeap + heap_slots first-slice **已交**（ADR-846）→ W503 B-09 MutRefCell mir-run DictBuild ManagedHeap + heap_slots align **已交**（ADR-847）→ W504 B-09 MutRefCell VM CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-848）→ W505 B-09 MutRefCell mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-849）→ W506 B-09 ObjectRef+MutRefCell VM call-return（finish_call）ManagedHeap + heap_slots first-slice **已交**（ADR-850）→ W507 B-09 ObjectRef+MutRefCell mir-run call-return（call_fn + call_closure）ManagedHeap + heap_slots first-slice **已交**（ADR-851）→ W508 B-09 ObjectRef+MutRefCell ActorCall（VM+mir-run runtime）ManagedHeap + heap_slots first-slice **已交**（ADR-852）→ W509 B-09 ObjectRef+MutRefCell MethodCall（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-853）→ W510 B-09 ObjectRefRetain catch-all（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-854）→ W511 B-09 LLVM·WASM ObjectRefRetain/Release emit-path 退役 first-slice **已交**（ADR-855）→ W512 B-09 ObjectRef ObjectGet residual re-entry + birth inventory ManagedHeap + heap_slots first-slice **已交**（ADR-856）→ W513 B-09 ObjectRef ArrayGet（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-857）→ W515 B-09 ObjectSet MutRefCell 写回（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-859）→ W516 B-09 LoadGlobal（VM+mir-run/SSA）ManagedHeap + heap_slots first-slice **已交**（ADR-860）→ W517 B-09 SSA LoadLocal ManagedHeap + heap_slots first-slice **已交**（ADR-861）→ W518 B-09 ObjectRef RC migrate-readiness thin **已交**（ADR-862）→ W519+ — 优先 Value/VM/mir-run RC 与 RT ABI retain/release/wrap **诚实退役** → 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
