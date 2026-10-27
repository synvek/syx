# B-09 StoreLocal/`run_class_deinits` ObjectRef RC true-retire first-slice (W528 / ADR-872)

## Status

**B-09 StoreLocal/`run_class_deinits` ObjectRef RC true-retire first-slice met**: after W527 GC↔Type_deinit sweep bridge, VM `Instruction::StoreLocal` + `run_class_deinits_if_enabled` and mir-run `STORE_LOCAL` + `run_class_deinits` **stop** calling `release_object_ref` / `Value::object_ref_release` (and the eager Type_deinit that followed last-ref release). Heap unroot (`maybe_untrack_objectref_value` + `ManagedHeap::free_slot`) **kept**. Type_deinit for unreachable ObjectRef remains on the W527 collect sweep path (`run_gc_collect` / `heap_collect`; skip `__syx_weak_dead`). `Value::object_ref_*` APIs and RT ABI retain/release/wrap **still present**. This window does **not** delete Value RC APIs, delete RT ABI, flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-seventy-eighth` / `A-W528`.

## Index

| Prior | Status |
|-------|--------|
| W476–W527 narrative → sweep Type_deinit bridge | **met** |
| This window StoreLocal/deinit ObjectRef RC true retire | **met** |
| Heap unroot on StoreLocal/deinits | **kept** |
| collect sweep Type_deinit | **still wired** |
| Value RC / wrap·retain·release ABI true delete | **still unmet** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Cleanup ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-y3b-iib | Tracks → deinits unroot → sweep Type_deinit | **met** (W494–W527) |
| 4c-ii-y3b-iii. StoreLocal/deinit RC retire | Stop death-path RC after sweep bridge | **met** (this window) |
| 4c-ii-y. Value RC true delete | Remove `Value::object_ref_*` | **still unmet** |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## RC retire contract (hard)

| Item | Status | Meaning |
|------|--------|---------|
| StoreLocal ObjectRef RC / eager Type_deinit | **retired** | no `release_object_ref` / `object_ref_release` |
| `run_class_deinits` ObjectRef RC / eager Type_deinit | **retired** | same |
| StoreLocal/deinits heap unroot | **kept** | `vm_mir_run_storelocal_deinits_heap_unroot_kept` |
| collect ObjectRef Type_deinit | **still wired** | W527 floor |
| `free_slot` Type_deinit | **still skip** | unroot ≠ death |
| `Value::object_ref_retain/release` API | **still present** | `value_object_ref_retain_release_api_still_present` |
| RT ABI retain/release/wrap | **still present** | `rt_abi_object_ref_retain_release_wrap_still_present` |
| `objectref_rc_migrate_enabled` | **false** | Full migrate unmet |
| `storage_cleanup_enabled` | **false** | Full cleanup unmet |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectrefStorelocalDeinitsRcRetireFirstSliceInventory` / `gate_b09_objectref_storelocal_deinits_rc_retire_first_slice_inventory` |
| Flags | `b09_objectref_storelocal_deinits_rc_retire_first_slice_signed=true`; `b09_gc_type_deinit_sweep_bridge_first_slice_floor_ok=true`; `vm_mir_run_storelocal_objectref_rc_retired=true`; `vm_mir_run_deinits_objectref_rc_retired=true`; `vm_mir_run_storelocal_deinits_heap_unroot_kept=true`; `managed_heap_collect_objectref_type_deinit_still_wired=true`; `value_object_ref_retain_release_api_still_present=true`; `rt_abi_object_ref_retain_release_wrap_still_present=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | Value RC full delete / ABI delete / flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / HostFn+1 / MIR distance change / Type_deinit on `free_slot` / stripping heap unroot |
| Honesty | StoreLocal/deinit RC true retire **≠** Value RC full delete / **≠** ABI delete / **≠** migrate flip / **≠** cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) — StoreLocal + `run_class_deinits_if_enabled` RC retire / ADR-872; untrack kept
- [`crates/syx-mir-run/src/runtime.rs`](../../crates/syx-mir-run/src/runtime.rs) — STORE_LOCAL + `run_class_deinits` RC retire / ADR-872; untrack kept
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectrefStorelocalDeinitsRcRetireFirstSliceInventory`
- Meta: `b09_objectref_storelocal_deinits_rc_retire_first_slice.rs`
- Floor: [`gate-b09-gc-type-deinit-sweep-bridge-first-slice.md`](gate-b09-gc-type-deinit-sweep-bridge-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Deleting `Value::object_ref_*`; deleting RT ABI retain/release/wrap; hanging Type_deinit on `free_slot`; stripping heap unroot
- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Nested field dual-track this window
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w527_*.plan.md`

## Next

W527 B-09 GC↔Type_deinit sweep-finalizer first-slice **已交**（ADR-871）→ W528 B-09 StoreLocal/`run_class_deinits` ObjectRef RC true-retire first-slice **已交**（ADR-872）→ W529 B-09 Value ObjectRef RC API delete first-slice **已交**（ADR-873）→ W530+ — 优先 wrap/retain/release ABI 真删（慎）→ 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
