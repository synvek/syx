# B-09 ObjectSet MutRefCell write-back（VM+mir-run）ManagedHeap + heap_slots first-slice (W515 / ADR-859)

## Status

**B-09 ObjectSet MutRefCell write-back ManagedHeap + heap_slots first-slice met**: after W514 LLVM ObjectRefWrap emit retire, VM `Instruction::ObjectSet` and mir-run `OBJECT_SET` / `object_set` success paths (including `mut_ref_cell_set_field` write-back) dual-register stack-top `ObjectRef` and `MutRefCell` into ManagedHeap and push `heap_slots` (identity-dedup; all push branches). This window does **not** flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, dual-track LoadGlobal, delete Value/VM/mir-run RC, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-sixty-fifth` / `A-W515`.

## Index

| Prior | Status |
|-------|--------|
| W476–W514 narrative → LLVM wrap emit retire | **met** |
| This window ObjectSet MutRefCell write-back dual-track | **met** |
| LoadGlobal dual-track | **met** (W516 / ADR-860) |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Cleanup ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-q | Tracks → ArrayGet → wrap emit retire | **met** (W494–W514) |
| 4c-ii-r. ObjectSet write-back | ObjectSet → ManagedHeap + `heap_slots` | **met** (this window) |
| 4c-ii. Storage-layer cleanup | Full ObjectRef/MutRefCell→ManagedHeap; flip cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## ObjectSet contract (hard)

| Item | Status | Meaning |
|------|--------|---------|
| VM ObjectSet → dual helpers | **wired** | After all push branches |
| mir-run `object_set` / OBJECT_SET | **wired** | Parity with VM |
| ArrayGet / ObjectGet / wrap emit floors | **retained** | W512–W514 |
| LoadGlobal dual-track | **met** (W516 / ADR-860) | W516+ |
| `objectref_rc_migrate_enabled` | **false** | Full migrate unmet |
| `storage_cleanup_enabled` | **false** | Full cleanup unmet |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectRefObjectSetHeapSlotsFirstSliceInventory` / `gate_b09_objectref_objectset_heap_slots_first_slice_inventory` |
| Flags | `b09_objectref_objectset_heap_slots_first_slice_signed=true`; `b09_llvm_objectref_wrap_emit_retire_first_slice_floor_ok=true`; `vm_objectset_objectref_managed_heap_registered=true`; `mir_run_objectset_objectref_managed_heap_registered=true`; `vm_mir_run_objectset_heap_slots_tracked=true`; `objectref_rc_storage_still_present=true`; `llvm_wasm_objectref_rc_still_live=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / LoadGlobal covered / Value RC deleted / scan-all auto / RT ABI delete / HostFn+1 / MIR distance change |
| Honesty | ObjectSet track **≠** LoadGlobal cover / **≠** full migrate / **≠** storage cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) — ObjectSet + dual helpers / ADR-859
- [`crates/syx-mir-run/src/runtime.rs`](../../crates/syx-mir-run/src/runtime.rs) — `object_set` + dual helpers / ADR-859
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectRefObjectSetHeapSlotsFirstSliceInventory`
- Meta: `b09_objectref_objectset_heap_slots_first_slice.rs`
- Floor: [`gate-b09-llvm-objectref-wrap-emit-retire-first-slice.md`](gate-b09-llvm-objectref-wrap-emit-retire-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Deleting Value / VM / mir-run RC / deleting RT ABI / LoadGlobal dual-track (done W516; SSA LoadLocal still unmet)
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w514_*.plan.md`

## Next

W515 B-09 ObjectSet MutRefCell 写回（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-859）→ W516 B-09 LoadGlobal（VM+mir-run/SSA）ManagedHeap + heap_slots first-slice **已交**（ADR-860）→ W517 B-09 SSA LoadLocal ManagedHeap + heap_slots first-slice **已交**（ADR-861）→ W518 B-09 ObjectRef RC migrate-readiness thin **已交**（ADR-862）→ W519+ — 优先 Value/VM/mir-run RC 与 RT ABI retain/release/wrap **诚实退役** → 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
