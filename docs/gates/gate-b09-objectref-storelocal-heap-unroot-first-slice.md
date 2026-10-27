# B-09 StoreLocal ObjectRef ManagedHeap unroot first-slice (W525 / ADR-869)

## Status

**B-09 StoreLocal ObjectRef ManagedHeap/heap_slots unroot first-slice met**: after W524 ObjectRefRelease opcode RC true-retire, VM `Instruction::StoreLocal` and mir-run `STORE_LOCAL` wire ObjectRef dual-track **death-path** unroot (`maybe_untrack_objectref_value` + `ManagedHeap::free_slot`) on overwrite. StoreLocal / `run_class_deinits` ObjectRef RC + Type_deinit **still live**. `Value::object_ref_*` APIs and RT ABI retain/release/wrap **still present**. This window does **not** retire StoreLocal RC, strip deinits, delete Value RC APIs, delete RT ABI, flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-seventy-fifth` / `A-W525`.

## Index

| Prior | Status |
|-------|--------|
| W476–W524 narrative → Release opcode RC true-retire | **met** |
| This window StoreLocal ManagedHeap/heap_slots unroot | **met** |
| `run_class_deinits` heap unroot | **still unmet** |
| StoreLocal / deinit ObjectRef RC true retire | **still unmet** |
| Value RC / wrap·retain·release ABI true delete | **still unmet** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Cleanup ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-y3a | Tracks → Release opcode RC retire | **met** (W494–W524) |
| 4c-ii-y3b-i. StoreLocal heap unroot | Death-path dual-track prune on overwrite | **met** (this window) |
| 4c-ii-y3b-ii. deinit heap unroot | `run_class_deinits` death-path | **still unmet** |
| 4c-ii-y3b-iii. StoreLocal/deinit RC retire | Stop RC after GC↔Type_deinit bridge | **still unmet** |
| 4c-ii-y. Value RC true delete | Remove `Value::object_ref_*` | **still unmet** |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## StoreLocal unroot contract (hard)

| Item | Status | Meaning |
|------|--------|---------|
| VM StoreLocal heap_slots unroot | **wired** | `maybe_untrack_objectref_value` + `free_slot` |
| mir-run STORE_LOCAL heap_slots unroot | **wired** | parity with VM |
| StoreLocal ObjectRef RC / Type_deinit | **still live** | `storelocal_objectref_rc_still_live` |
| `run_class_deinits` ObjectRef RC | **still live** | `run_class_deinits_objectref_rc_still_live` |
| `Value::object_ref_retain/release` API | **still present** | `value_object_ref_retain_release_api_still_present` |
| RT ABI retain/release/wrap | **still present** | `rt_abi_object_ref_retain_release_wrap_still_present` |
| `objectref_rc_migrate_enabled` | **false** | Full migrate unmet |
| `storage_cleanup_enabled` | **false** | Full cleanup unmet |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectrefStorelocalHeapUnrootFirstSliceInventory` / `gate_b09_objectref_storelocal_heap_unroot_first_slice_inventory` |
| Flags | `b09_objectref_storelocal_heap_unroot_first_slice_signed=true`; `b09_objectref_rc_release_opcode_retire_first_slice_floor_ok=true`; `vm_mir_run_storelocal_objectref_heap_unroot_wired=true`; `storelocal_objectref_rc_still_live=true`; `run_class_deinits_objectref_rc_still_live=true`; `value_object_ref_retain_release_api_still_present=true`; `rt_abi_object_ref_retain_release_wrap_still_present=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | StoreLocal RC true retire / deinit strip / Value RC full delete / ABI delete / flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / HostFn+1 / MIR distance change |
| Honesty | StoreLocal heap unroot **≠** StoreLocal RC true retire / **≠** deinit retire / **≠** Value RC full delete / **≠** ABI delete / **≠** migrate flip / **≠** cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) — StoreLocal unroot + RC kept / ADR-869
- [`crates/syx-mir-run/src/runtime.rs`](../../crates/syx-mir-run/src/runtime.rs) — STORE_LOCAL unroot + RC kept / ADR-869
- [`crates/syx-runtime-core/src/managed_heap.rs`](../../crates/syx-runtime-core/src/managed_heap.rs) — `ManagedHeap::free_slot`
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectrefStorelocalHeapUnrootFirstSliceInventory`
- Meta: `b09_objectref_storelocal_heap_unroot_first_slice.rs`
- Floor: [`gate-b09-objectref-rc-release-opcode-retire-first-slice.md`](gate-b09-objectref-rc-release-opcode-retire-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Stripping StoreLocal / `run_class_deinits` `object_ref_release`; deleting `Value::object_ref_*`; deleting RT ABI retain/release/wrap
- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Nested field dual-track this window
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w524_*.plan.md`

## Next

W524 B-09 ObjectRefRelease opcode RC true-retire first-slice **已交**（ADR-868）→ W525 B-09 StoreLocal ObjectRef ManagedHeap unroot first-slice **已交**（ADR-869）→ W526 B-09 run_class_deinits ObjectRef ManagedHeap unroot first-slice **已交**（ADR-870）→ W527 B-09 GC↔Type_deinit sweep-finalizer first-slice **已交**（ADR-871）→ W528 B-09 StoreLocal/`run_class_deinits` ObjectRef RC true-retire first-slice **已交**（ADR-872）→ W529+ — 优先删 `Value::object_ref_*`（慎）→ 删 `Value::object_ref_*` → wrap/retain/release ABI 真删（慎）→ 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
