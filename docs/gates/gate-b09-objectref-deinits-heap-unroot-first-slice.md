# B-09 run_class_deinits ObjectRef ManagedHeap unroot first-slice (W526 / ADR-870)

## Status

**B-09 run_class_deinits ObjectRef ManagedHeap/heap_slots unroot first-slice met**: after W525 StoreLocal heap unroot, VM `run_class_deinits_if_enabled` and mir-run `run_class_deinits` wire ObjectRef dual-track **death-path** unroot (`maybe_untrack_objectref_value` + `ManagedHeap::free_slot`) before RC release on scope exit. StoreLocal / `run_class_deinits` ObjectRef RC + Type_deinit **still live**. `Value::object_ref_*` APIs and RT ABI retain/release/wrap **still present**. This window does **not** retire StoreLocal/deinit RC, delete Value RC APIs, delete RT ABI, flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-seventy-sixth` / `A-W526`.

## Index

| Prior | Status |
|-------|--------|
| W476–W525 narrative → StoreLocal heap unroot | **met** |
| This window `run_class_deinits` ManagedHeap/heap_slots unroot | **met** |
| StoreLocal / deinit ObjectRef RC true retire | **still unmet** |
| Value RC / wrap·retain·release ABI true delete | **still unmet** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Cleanup ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-y3b-i | Tracks → StoreLocal heap unroot | **met** (W494–W525) |
| 4c-ii-y3b-ii. deinits heap unroot | Scope-exit death-path dual-track prune | **met** (this window) |
| 4c-ii-y3b-iii. StoreLocal/deinit RC retire | Stop RC after GC↔Type_deinit bridge | **still unmet** |
| 4c-ii-y. Value RC true delete | Remove `Value::object_ref_*` | **still unmet** |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## deinits unroot contract (hard)

| Item | Status | Meaning |
|------|--------|---------|
| VM `run_class_deinits_if_enabled` heap_slots unroot | **wired** | `maybe_untrack_objectref_value` before release |
| mir-run `run_class_deinits` heap_slots unroot | **wired** | parity with VM |
| StoreLocal ObjectRef RC / Type_deinit | **still live** | `storelocal_objectref_rc_still_live` |
| `run_class_deinits` ObjectRef RC / Type_deinit | **still live** | `run_class_deinits_objectref_rc_still_live` |
| `Value::object_ref_retain/release` API | **still present** | `value_object_ref_retain_release_api_still_present` |
| RT ABI retain/release/wrap | **still present** | `rt_abi_object_ref_retain_release_wrap_still_present` |
| `objectref_rc_migrate_enabled` | **false** | Full migrate unmet |
| `storage_cleanup_enabled` | **false** | Full cleanup unmet |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectrefDeinitsHeapUnrootFirstSliceInventory` / `gate_b09_objectref_deinits_heap_unroot_first_slice_inventory` |
| Flags | `b09_objectref_deinits_heap_unroot_first_slice_signed=true`; `b09_objectref_storelocal_heap_unroot_first_slice_floor_ok=true`; `vm_mir_run_deinits_objectref_heap_unroot_wired=true`; `storelocal_objectref_rc_still_live=true`; `run_class_deinits_objectref_rc_still_live=true`; `value_object_ref_retain_release_api_still_present=true`; `rt_abi_object_ref_retain_release_wrap_still_present=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | StoreLocal/deinit RC true retire / Value RC full delete / ABI delete / flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / HostFn+1 / MIR distance change |
| Honesty | deinits heap unroot **≠** StoreLocal/deinit RC true retire / **≠** Value RC full delete / **≠** ABI delete / **≠** migrate flip / **≠** cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) — `run_class_deinits_if_enabled` unroot + RC kept / ADR-870
- [`crates/syx-mir-run/src/runtime.rs`](../../crates/syx-mir-run/src/runtime.rs) — `run_class_deinits` unroot + RC kept / ADR-870
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectrefDeinitsHeapUnrootFirstSliceInventory`
- Meta: `b09_objectref_deinits_heap_unroot_first_slice.rs`
- Floor: [`gate-b09-objectref-storelocal-heap-unroot-first-slice.md`](gate-b09-objectref-storelocal-heap-unroot-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Stripping StoreLocal / `run_class_deinits` `object_ref_release`; deleting `Value::object_ref_*`; deleting RT ABI retain/release/wrap
- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Nested field dual-track this window
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w525_*.plan.md`

## Next

W525 B-09 StoreLocal ObjectRef ManagedHeap unroot first-slice **已交**（ADR-869）→ W526 B-09 run_class_deinits ObjectRef ManagedHeap unroot first-slice **已交**（ADR-870）→ W527 B-09 GC↔Type_deinit sweep-finalizer first-slice **已交**（ADR-871）→ W528 B-09 StoreLocal/`run_class_deinits` ObjectRef RC true-retire first-slice **已交**（ADR-872）→ W529+ — 优先删 `Value::object_ref_*` → 删 `Value::object_ref_*` → wrap/retain/release ABI 真删（慎）→ 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
