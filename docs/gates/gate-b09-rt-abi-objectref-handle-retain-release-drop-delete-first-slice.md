# B-09 RT ABI ObjectRef handle retain/release/drop delete first-slice (W534 / ADR-878)

## Status

**B-09 RT ABI ObjectRef handle retain/release/drop true-delete first-slice met**: after W533 handle retire-readiness, C ABI symbols `syx_rt_object_retain` / `syx_rt_object_release` / `syx_rt_object_drop` are **deleted**. Handle-table API `object_handle_*` **still present**. mir-run `OBJECT_REF_*` opcodes **still present**. `Value::ObjectRef { rc }` storage **still present**. Opcode wrap/retain/release ABI **deleted** (W530). Value ObjectRef RC API **deleted** (W529). Nested write-path dual-track **met** (W532). Deep field-graph GC walk **still unmet**. `objectref_rc_migrate_enabled` **still false**. Collect sweep Type_deinit (W527) **still wired**. This window does **not** peel the handle table, strip mir-run opcodes, strip RC storage, flip migrate / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**（退出加权 ≈74–75%）；≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-eighty-fourth` / `A-W534`.

## Index

| Prior | Status |
|-------|--------|
| W476–W533 narrative → handle retire-readiness | **met** |
| This window handle RT ABI true delete | **met** |
| Handle-table `object_handle_*` API | **still present** |
| mir-run `OBJECT_REF_*` opcodes | **still present** |
| `Value::ObjectRef { rc }` storage | **still present** |
| Deep field-graph GC walk | **still unmet** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Cleanup ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-w | Tracks → RC retire → nested write-path → handle retire-readiness | **met** (W494–W533) |
| 4c-ii-x. Handle RT ABI true delete | Remove `syx_rt_object_retain/release/drop` | **met** (this window) |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Handle ABI delete contract (hard)

| Item | Status | Meaning |
|------|--------|---------|
| `syx_rt_object_retain` / `release` / `drop` | **deleted** | `rt_abi_object_handle_retain_release_drop_deleted` |
| `object_handle_*` table API | **still present** | `object_handle_table_api_still_present` |
| mir-run `OBJECT_REF_*` opcodes | **still present** | `mir_run_object_ref_opcodes_still_present` |
| `Value::ObjectRef { rc }` storage | **still present** | `objectref_rc_storage_still_present` |
| Nested write-path dual-track | **met** | W532 floor |
| Deep field-graph GC walk | **unmet** | `deep_field_graph_gc_walk_still_unmet` |
| Value `object_ref_retain` / `object_ref_release` API | **deleted** | W529 floor |
| RT opcode wrap/retain/release ABI | **deleted** | W530 floor |
| collect ObjectRef Type_deinit | **still wired** | W527 floor |
| `objectref_rc_migrate_enabled` | **false** | Full migrate unmet |
| `storage_cleanup_enabled` | **false** | Full cleanup unmet |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09RtAbiObjectrefHandleRetainReleaseDropDeleteFirstSliceInventory` / `gate_b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_inventory` |
| Flags | `b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_signed=true`; `b09_rt_abi_objectref_handle_retire_readiness_thin_floor_ok=true`; `rt_abi_object_handle_retain_release_drop_deleted=true`; `object_handle_table_api_still_present=true`; `mir_run_object_ref_opcodes_still_present=true`; `objectref_rc_storage_still_present=true`; `nested_field_write_path_dual_track_met=true`; `deep_field_graph_gc_walk_still_unmet=true`; `value_object_ref_retain_release_api_deleted=true`; `rt_abi_object_ref_wrap_retain_release_deleted=true`; `managed_heap_collect_objectref_type_deinit_still_wired=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | peeling handle table / stripping mir-run opcodes / stripping RC storage / flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / HostFn+1 / MIR distance change / claiming handle C-ABI delete as migrate |
| Honesty | handle C-ABI true delete **≠** peel handle table / **≠** strip opcodes / **≠** strip RC storage / **≠** full migrate / **≠** cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-rt/src/abi.rs`](../../crates/syx-rt/src/abi.rs) — `syx_rt_object_retain`/`release`/`drop` deleted / ADR-878; comment nails mir-run + `object_handle_*` retained
- [`crates/syx-llvm/src/lib.rs`](../../crates/syx-llvm/src/lib.rs) — declares + allowlist omitted
- [`crates/syx-wasm-run/src/lib.rs`](../../crates/syx-wasm-run/src/lib.rs) — `link_syx_rt_object_abi` retired
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09RtAbiObjectrefHandleRetainReleaseDropDeleteFirstSliceInventory`
- Meta: `b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice.rs`
- Floor: [`gate-b09-rt-abi-objectref-handle-retire-readiness-thin.md`](gate-b09-rt-abi-objectref-handle-retire-readiness-thin.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Peeling `object_handle_*` table; stripping mir-run `OBJECT_REF_*`; stripping `Value::ObjectRef { rc }` storage; deep field-graph GC walk
- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Hanging Type_deinit on `free_slot`; stripping heap unroot / collect Type_deinit
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w533_*.plan.md`

## Next

W533 B-09 RT ABI handle retain·release·drop retire-readiness thin **已交**（ADR-877）→ W534 B-09 RT ABI handle retain·release·drop true-delete first-slice **已交**（ADR-878）→ W535 B-09 ObjectRef RC migrate-readiness post-handle-delete refresh thin **已交**（ADR-879）→ W536 B-09 ObjectRef strip-RC readiness thin **已交**（ADR-880）→ W537 B-09 ObjectRef strip-RC true-delete first-slice **已交**（ADR-881）→ W538 B-09 ObjectRef deep field-graph GC walk readiness thin **已交**（ADR-882）→ W539 B-09 ObjectRef deep field-graph GC walk true first-slice **已交**（ADR-883）→ W540 B-09 ObjectRef peel 表·opcodes readiness thin **已交**（ADR-884）→ W541 B-09 ObjectRef peel 表·opcodes true-delete first-slice **已交**（ADR-885）→ W542+ — 真产品迁 / 翻 cleanup / 真 enable；或另钉 MirOp/VM residual peel；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
