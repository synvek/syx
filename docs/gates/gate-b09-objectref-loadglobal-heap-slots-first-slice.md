# B-09 LoadGlobal（VM+mir-run/SSA）ManagedHeap + heap_slots first-slice (W516 / ADR-860)

## Status

**B-09 LoadGlobal ManagedHeap + heap_slots first-slice met**: after W515 ObjectSet MutRefCell write-back, VM `Instruction::LoadGlobal`, mir-run `op::LOAD_GLOBAL`, and SSA `MirOp::LoadGlobal` success paths dual-register stack-top `ObjectRef` and `MutRefCell` into ManagedHeap and push `heap_slots` (identity-dedup; after push; **no** `object_ref_retain`). This window does **not** flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, delete Value/VM/mir-run RC, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-sixty-sixth` / `A-W516`.

## Index

| Prior | Status |
|-------|--------|
| W476–W515 narrative → ObjectSet write-back | **met** |
| This window LoadGlobal (VM+bytecode+SSA) dual-track | **met** |
| SSA LoadLocal dual-track | **met** (W517 / ADR-861) |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Cleanup ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-r | Tracks → ObjectSet write-back | **met** (W494–W515) |
| 4c-ii-s. LoadGlobal | LoadGlobal → ManagedHeap + `heap_slots` | **met** (this window) |
| 4c-ii. Storage-layer cleanup | Full ObjectRef/MutRefCell→ManagedHeap; flip cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## LoadGlobal contract (hard)

| Item | Status | Meaning |
|------|--------|---------|
| VM LoadGlobal → dual helpers | **wired** | After push; no retain |
| mir-run `LOAD_GLOBAL` | **wired** | Parity with VM |
| SSA `MirOp::LoadGlobal` | **wired** | After `set_ssa_reg` + push |
| ObjectSet / ArrayGet / wrap floors | **retained** | W513–W515 |
| SSA LoadLocal dual-track | **met** (W517 / ADR-861) | W517+ |
| `objectref_rc_migrate_enabled` | **false** | Full migrate unmet |
| `storage_cleanup_enabled` | **false** | Full cleanup unmet |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectRefLoadGlobalHeapSlotsFirstSliceInventory` / `gate_b09_objectref_loadglobal_heap_slots_first_slice_inventory` |
| Flags | `b09_objectref_loadglobal_heap_slots_first_slice_signed=true`; `b09_objectref_objectset_heap_slots_first_slice_floor_ok=true`; `vm_loadglobal_objectref_managed_heap_registered=true`; `mir_run_loadglobal_objectref_managed_heap_registered=true`; `mir_run_ssa_loadglobal_objectref_managed_heap_registered=true`; `vm_mir_run_loadglobal_heap_slots_tracked=true`; `objectref_rc_storage_still_present=true`; `llvm_wasm_objectref_rc_still_live=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / LoadGlobal `object_ref_retain` / SSA LoadLocal covered / Value RC deleted / scan-all auto / RT ABI delete / HostFn+1 / MIR distance change |
| Honesty | LoadGlobal track **≠** full migrate / **≠** storage cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA / **≠** SSA LoadLocal |

## Evidence

- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) — LoadGlobal + dual helpers / ADR-860
- [`crates/syx-mir-run/src/runtime.rs`](../../crates/syx-mir-run/src/runtime.rs) — `LOAD_GLOBAL` + SSA `LoadGlobal` + dual helpers / ADR-860
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectRefLoadGlobalHeapSlotsFirstSliceInventory`
- Meta: `b09_objectref_loadglobal_heap_slots_first_slice.rs`
- Floor: [`gate-b09-objectref-objectset-heap-slots-first-slice.md`](gate-b09-objectref-objectset-heap-slots-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Deleting Value / VM / mir-run RC / deleting RT ABI / adding LoadGlobal `object_ref_retain` / SSA LoadLocal dual-track (done W517)
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w515_*.plan.md`

## Next

W515 B-09 ObjectSet MutRefCell 写回（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-859）→ W516 B-09 LoadGlobal（VM+mir-run/SSA）ManagedHeap + heap_slots first-slice **已交**（ADR-860）→ W517 B-09 SSA LoadLocal ManagedHeap + heap_slots first-slice **已交**（ADR-861）→ W518 B-09 ObjectRef RC migrate-readiness thin **已交**（ADR-862）→ W519+ — 优先 Value/VM/mir-run RC 与 RT ABI retain/release/wrap **诚实退役** → 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
