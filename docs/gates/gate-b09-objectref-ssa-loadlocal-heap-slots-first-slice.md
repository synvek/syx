# B-09 SSA LoadLocal ManagedHeap + heap_slots first-slice (W517 / ADR-861)

## Status

**B-09 SSA LoadLocal ManagedHeap + heap_slots first-slice met**: after W516 LoadGlobal, mir-run SSA `MirOp::LoadLocal` success paths dual-register stack-top `ObjectRef` and `MutRefCell` into ManagedHeap and push `heap_slots` (identity-dedup; after `set_ssa_reg` + push; **no** `object_ref_retain`). Bytecode/VM `LoadLocal` W510 floors unchanged. This window does **not** flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, delete Value/VM/mir-run RC, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-sixty-seventh` / `A-W517`.

## Index

| Prior | Status |
|-------|--------|
| W476–W516 narrative → LoadGlobal | **met** |
| This window SSA `MirOp::LoadLocal` dual-track | **met** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Cleanup ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-s | Tracks → LoadGlobal | **met** (W494–W516) |
| 4c-ii-t. SSA LoadLocal | SSA LoadLocal → ManagedHeap + `heap_slots` | **met** (this window) |
| 4c-ii. Storage-layer cleanup | Full ObjectRef/MutRefCell→ManagedHeap; flip cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## SSA LoadLocal contract (hard)

| Item | Status | Meaning |
|------|--------|---------|
| SSA `MirOp::LoadLocal` → dual helpers | **wired** | After `set_ssa_reg` + push; no retain |
| bytecode/VM LoadLocal (W510) | **retained** | Unchanged |
| LoadGlobal / ObjectSet floors | **retained** | W515–W516 |
| `objectref_rc_migrate_enabled` | **false** | Full migrate unmet |
| `storage_cleanup_enabled` | **false** | Full cleanup unmet |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectRefSsaLoadlocalHeapSlotsFirstSliceInventory` / `gate_b09_objectref_ssa_loadlocal_heap_slots_first_slice_inventory` |
| Flags | `b09_objectref_ssa_loadlocal_heap_slots_first_slice_signed=true`; `b09_objectref_loadglobal_heap_slots_first_slice_floor_ok=true`; `mir_run_ssa_loadlocal_objectref_managed_heap_registered=true`; `mir_run_ssa_loadlocal_heap_slots_tracked=true`; `objectref_rc_storage_still_present=true`; `llvm_wasm_objectref_rc_still_live=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / SSA LoadLocal `object_ref_retain` / Value RC deleted / scan-all auto / RT ABI delete / HostFn+1 / MIR distance change |
| Honesty | SSA LoadLocal track **≠** full migrate / **≠** storage cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA / **≠** bytecode retain change |

## Evidence

- [`crates/syx-mir-run/src/runtime.rs`](../../crates/syx-mir-run/src/runtime.rs) — SSA `MirOp::LoadLocal` + dual helpers / ADR-861
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectRefSsaLoadlocalHeapSlotsFirstSliceInventory`
- Meta: `b09_objectref_ssa_loadlocal_heap_slots_first_slice.rs`
- Floor: [`gate-b09-objectref-loadglobal-heap-slots-first-slice.md`](gate-b09-objectref-loadglobal-heap-slots-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Deleting Value / VM / mir-run RC / deleting RT ABI / adding SSA LoadLocal `object_ref_retain` this window
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w516_*.plan.md`

## Next

W516 B-09 LoadGlobal（VM+mir-run/SSA）ManagedHeap + heap_slots first-slice **已交**（ADR-860）→ W517 B-09 SSA LoadLocal ManagedHeap + heap_slots first-slice **已交**（ADR-861）→ W518 B-09 ObjectRef RC migrate-readiness thin **已交**（ADR-862）→ W519 B-09 RT ABI ObjectRefWrap retire-readiness thin **已交**（ADR-863）→ W520+ — 优先 RT ABI retain/release retire-readiness / Value/VM/mir-run RC **诚实退役** → 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
