# B-09 RT ABI ObjectRef wrap/retain/release delete first-slice (W530 / ADR-874)

## Status

**B-09 RT ABI ObjectRef wrap/retain/release true-delete first-slice met**: after W529 Value ObjectRef RC API delete, RT opcode ABI symbols `syx_rt_object_ref_wrap` / `syx_rt_object_ref_retain` / `syx_rt_object_ref_release` are **deleted**. Handle ABI `syx_rt_object_retain` / `syx_rt_object_release` / `syx_rt_object_drop` **still present**. Value ObjectRef RC API **deleted** (W529 floor). `objectref_rc_migrate_enabled` **still false**. Collect sweep Type_deinit (W527) **still wired**. This window does **not** delete handle ABI, flip migrate / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-eightieth` / `A-W530`.

## Index

| Prior | Status |
|-------|--------|
| W476–W529 narrative → Value RC API delete | **met** |
| This window RT opcode ABI wrap/retain/release true delete | **met** |
| Handle ABI `syx_rt_object_retain/release/drop` | **still present** |
| Value ObjectRef RC API | **deleted** (W529 floor) |
| collect sweep Type_deinit | **still wired** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Cleanup ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-y | Tracks → Value RC API true delete | **met** (W494–W529) |
| 4c-ii-y2. RT opcode ABI true delete | Remove wrap/retain/release opcode ABI | **met** (this window) |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## RT opcode ABI delete contract (hard)

| Item | Status | Meaning |
|------|--------|---------|
| `syx_rt_object_ref_wrap` / `retain` / `release` | **deleted** | `rt_abi_object_ref_wrap_retain_release_deleted` |
| Handle ABI `syx_rt_object_retain/release/drop` | **still present** | `rt_abi_object_handle_retain_release_drop_still_present` |
| Value `object_ref_retain` / `object_ref_release` API | **deleted** | W529 floor (`value_object_ref_retain_release_api_deleted`) |
| collect ObjectRef Type_deinit | **still wired** | W527 floor |
| `objectref_rc_migrate_enabled` | **false** | Full migrate unmet |
| `storage_cleanup_enabled` | **false** | Full cleanup unmet |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09RtAbiObjectrefWrapRetainReleaseDeleteFirstSliceInventory` / `gate_b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_inventory` |
| Flags | `b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_signed=true`; `b09_objectref_value_rc_api_delete_first_slice_floor_ok=true`; `rt_abi_object_ref_wrap_retain_release_deleted=true`; `rt_abi_object_handle_retain_release_drop_still_present=true`; `value_object_ref_retain_release_api_deleted=true`; `managed_heap_collect_objectref_type_deinit_still_wired=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | deleting handle ABI / flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / HostFn+1 / MIR distance change / stripping mir-run opcode paths / stripping heap unroot |
| Honesty | RT opcode ABI true delete **≠** handle ABI delete / **≠** full migrate / **≠** cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-rt/src/abi.rs`](../../crates/syx-rt/src/abi.rs) — `syx_rt_object_ref_wrap`/`retain`/`release` deleted / ADR-874; handle ABI retained
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09RtAbiObjectrefWrapRetainReleaseDeleteFirstSliceInventory`
- Meta: `b09_rt_abi_objectref_wrap_retain_release_delete_first_slice.rs` (parent)
- Floor: [`gate-b09-objectref-value-rc-api-delete-first-slice.md`](gate-b09-objectref-value-rc-api-delete-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Deleting handle ABI `syx_rt_object_retain` / `release` / `drop`; flipping `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Stripping mir-run `OBJECT_REF_*` opcode paths; hanging Type_deinit on `free_slot`; stripping heap unroot
- Nested field dual-track this window
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w529_*.plan.md`

## Next

W529 B-09 Value ObjectRef RC API delete first-slice **已交**（ADR-873）→ W530 B-09 RT ABI wrap·retain·release true-delete first-slice **已交**（ADR-874）→ W531 B-09 ObjectRef RC migrate-readiness refresh thin **已交**（ADR-875）→ W532 B-09 ObjectRef nested field dual-track first-slice **已交**（ADR-876）→ W533 B-09 RT ABI handle retain·release·drop retire-readiness thin **已交**（ADR-877）→ W534 B-09 RT ABI handle retain·release·drop true-delete first-slice **已交**（ADR-878）→ W535 B-09 ObjectRef RC migrate-readiness post-handle-delete refresh thin **已交**（ADR-879）→ W536 B-09 ObjectRef strip-RC readiness thin **已交**（ADR-880）→ W537 B-09 ObjectRef strip-RC true-delete first-slice **已交**（ADR-881）→ W538 B-09 ObjectRef deep field-graph GC walk readiness thin **已交**（ADR-882）→ W539 B-09 ObjectRef deep field-graph GC walk true first-slice **已交**（ADR-883）→ W540 B-09 ObjectRef peel 表·opcodes readiness thin **已交**（ADR-884）→ W541 B-09 ObjectRef peel 表·opcodes true-delete first-slice **已交**（ADR-885）→ W542+ — 真产品迁 / 翻 cleanup / 真 enable；或另钉 MirOp/VM residual peel；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
