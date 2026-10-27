# B-09 GC↔Type_deinit sweep-finalizer first-slice (W527 / ADR-871)

## Status

**B-09 ManagedHeap collect sweep → Type_deinit first-slice met**: after W526 `run_class_deinits` heap unroot, `ManagedHeap::collect` / `collect_mark_sweep` return swept `ObjectRef` values before nilling dead slots; VM `run_gc_collect` and mir-run `heap_collect` invoke existing `run_deinit_for_value` (skip already `__syx_weak_dead`). `free_slot` still **does not** run Type_deinit. StoreLocal / `run_class_deinits` ObjectRef RC + Type_deinit **still live**. `Value::object_ref_*` APIs and RT ABI retain/release/wrap **still present**. This window does **not** retire StoreLocal/deinit RC, delete Value RC APIs, delete RT ABI, flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-seventy-seventh` / `A-W527`.

## Index

| Prior | Status |
|-------|--------|
| W476–W526 narrative → deinits heap unroot | **met** |
| This window collect sweep Type_deinit bridge | **met** |
| `free_slot` Type_deinit | **still skip** (unroot ≠ death) |
| StoreLocal / deinit ObjectRef RC true retire | **met** (W528 / ADR-872) |
| Value RC / wrap·retain·release ABI true delete | **still unmet** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Cleanup ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-y3b-ii | Tracks → deinits heap unroot | **met** (W494–W526) |
| 4c-ii-y3b-iib. collect sweep Type_deinit | GcCollect death-path finalizer | **met** (this window) |
| 4c-ii-y3b-iii. StoreLocal/deinit RC retire | Stop RC after sweep bridge | **met** (W528 / ADR-872) |
| 4c-ii-y. Value RC true delete | Remove `Value::object_ref_*` | **still unmet** |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Sweep bridge contract (hard)

| Item | Status | Meaning |
|------|--------|---------|
| ManagedHeap collect ObjectRef Type_deinit | **wired** | `collect` returns swept ObjectRef; VM/mir-run run Type_deinit |
| `free_slot` Type_deinit | **still skip** | `free_slot_still_skips_type_deinit` |
| StoreLocal ObjectRef RC / Type_deinit | **still live** | `storelocal_objectref_rc_still_live` |
| `run_class_deinits` ObjectRef RC / Type_deinit | **still live** | `run_class_deinits_objectref_rc_still_live` |
| `Value::object_ref_retain/release` API | **still present** | `value_object_ref_retain_release_api_still_present` |
| RT ABI retain/release/wrap | **still present** | `rt_abi_object_ref_retain_release_wrap_still_present` |
| `objectref_rc_migrate_enabled` | **false** | Full migrate unmet |
| `storage_cleanup_enabled` | **false** | Full cleanup unmet |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09GcTypeDeinitSweepBridgeFirstSliceInventory` / `gate_b09_gc_type_deinit_sweep_bridge_first_slice_inventory` |
| Flags | `b09_gc_type_deinit_sweep_bridge_first_slice_signed=true`; `b09_objectref_deinits_heap_unroot_first_slice_floor_ok=true`; `managed_heap_collect_objectref_type_deinit_wired=true`; `free_slot_still_skips_type_deinit=true`; `storelocal_objectref_rc_still_live=true`; `run_class_deinits_objectref_rc_still_live=true`; `value_object_ref_retain_release_api_still_present=true`; `rt_abi_object_ref_retain_release_wrap_still_present=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | StoreLocal/deinit RC true retire / Value RC full delete / ABI delete / flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / HostFn+1 / MIR distance change / Type_deinit on `free_slot` |
| Honesty | sweep Type_deinit **≠** StoreLocal/deinit RC true retire / **≠** Value RC full delete / **≠** ABI delete / **≠** migrate flip / **≠** cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-runtime-core/src/managed_heap.rs`](../../crates/syx-runtime-core/src/managed_heap.rs) — `collect_mark_sweep` returns swept ObjectRef / ADR-871; `free_slot` still skips Type_deinit
- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) — `run_gc_collect(program)` sweep Type_deinit / ADR-871
- [`crates/syx-mir-run/src/runtime.rs`](../../crates/syx-mir-run/src/runtime.rs) — `heap_collect` sweep Type_deinit / ADR-871
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09GcTypeDeinitSweepBridgeFirstSliceInventory`
- Meta: `b09_gc_type_deinit_sweep_bridge_first_slice.rs`
- Floor: [`gate-b09-objectref-deinits-heap-unroot-first-slice.md`](gate-b09-objectref-deinits-heap-unroot-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Stripping StoreLocal / `run_class_deinits` `object_ref_release`; hanging Type_deinit on `free_slot`; deleting `Value::object_ref_*`; deleting RT ABI retain/release/wrap
- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Nested field dual-track this window
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w526_*.plan.md`

## Next

W526 B-09 run_class_deinits ObjectRef ManagedHeap unroot first-slice **已交**（ADR-870）→ W527 B-09 GC↔Type_deinit sweep-finalizer first-slice **已交**（ADR-871）→ W528 B-09 StoreLocal/`run_class_deinits` ObjectRef RC true-retire first-slice **已交**（ADR-872）→ W529+ — 优先删 `Value::object_ref_*` → wrap/retain/release ABI 真删（慎）→ 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
