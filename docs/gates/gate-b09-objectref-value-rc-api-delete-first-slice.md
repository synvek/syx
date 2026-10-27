# B-09 Value ObjectRef RC API delete first-slice (W529 / ADR-873)

## Status

**B-09 Value ObjectRef RC API delete first-slice met**: after W528 StoreLocal/`run_class_deinits` ObjectRef RC true-retire, `Value::object_ref_retain` / `object_ref_release` are **deleted**. `object_handle_drop` is **self-contained** RC (uses `Value::release_object_ref_for_handle_drop` / inline handle-table decrement; no longer calls `Value::object_ref_release`). RT ABI wrap/retain/release (`syx_rt_object_ref_*`) **still present**. `objectref_rc_migrate_enabled` **still false**. Collect sweep Type_deinit (W527) **still wired**. This window does **not** delete RT ABI, flip migrate / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-seventy-ninth` / `A-W529`.

## Index

| Prior | Status |
|-------|--------|
| W476–W528 narrative → StoreLocal/deinit RC retire | **met** |
| This window Value ObjectRef RC API delete | **met** |
| `object_handle_drop` self-contained RC | **met** |
| RT ABI wrap/retain/release | **still present** |
| collect sweep Type_deinit | **still wired** |
| RT ABI wrap·retain·release true delete | **still unmet** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Cleanup ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-y3b-iii | Tracks → deinits unroot → sweep Type_deinit → StoreLocal/deinit RC retire | **met** (W494–W528) |
| 4c-ii-y. Value RC API true delete | Remove `Value::object_ref_retain`/`object_ref_release` | **met** (this window) |
| 4c-ii-y2. RT ABI true delete | Remove wrap/retain/release opcode ABI | **still unmet** |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Value RC API delete contract (hard)

| Item | Status | Meaning |
|------|--------|---------|
| `Value::object_ref_retain` / `object_ref_release` | **deleted** | `value_object_ref_retain_release_api_deleted` |
| `object_handle_drop` self-contained RC | **met** | no call through deleted Value API |
| RT ABI retain/release/wrap | **still present** | `rt_abi_object_ref_retain_release_wrap_still_present` |
| collect ObjectRef Type_deinit | **still wired** | W527 floor |
| StoreLocal/deinits RC retire | **kept** | W528 floor |
| `objectref_rc_migrate_enabled` | **false** | Full migrate unmet |
| `storage_cleanup_enabled` | **false** | Full cleanup unmet |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectrefValueRcApiDeleteFirstSliceInventory` / `gate_b09_objectref_value_rc_api_delete_first_slice_inventory` |
| Flags | `b09_objectref_value_rc_api_delete_first_slice_signed=true`; `b09_objectref_storelocal_deinits_rc_retire_first_slice_floor_ok=true`; `value_object_ref_retain_release_api_deleted=true`; `object_handle_drop_self_contained_rc=true`; `rt_abi_object_ref_retain_release_wrap_still_present=true`; `managed_heap_collect_objectref_type_deinit_still_wired=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | RT ABI true delete / flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / HostFn+1 / MIR distance change / Type_deinit on `free_slot` / stripping heap unroot |
| Honesty | Value RC API true delete **≠** ABI true delete / **≠** full migrate / **≠** cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-runtime-core/src/value.rs`](../../crates/syx-runtime-core/src/value.rs) — `object_ref_retain`/`object_ref_release` deleted / ADR-873
- [`crates/syx-runtime-core/src/object_handles.rs`](../../crates/syx-runtime-core/src/object_handles.rs) — `object_handle_drop` self-contained RC / ADR-873
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectrefValueRcApiDeleteFirstSliceInventory`
- Meta: `b09_objectref_value_rc_api_delete_first_slice.rs`
- Floor: [`gate-b09-objectref-storelocal-deinits-rc-retire-first-slice.md`](gate-b09-objectref-storelocal-deinits-rc-retire-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Deleting RT ABI retain/release/wrap; flipping `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Hanging Type_deinit on `free_slot`; stripping heap unroot
- Nested field dual-track this window
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w528_*.plan.md`

## Next

W528 B-09 StoreLocal/`run_class_deinits` ObjectRef RC true-retire first-slice **已交**（ADR-872）→ W529 B-09 Value ObjectRef RC API delete first-slice **已交**（ADR-873）→ W530 ABI delete **已交** → W531+ — 优先翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
