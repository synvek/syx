# B-09 ObjectRef RC migrate-readiness post-handle-delete refresh thin (W535 / ADR-879)

## Status

**B-09 ObjectRef RC migrate-readiness post-handle-delete refresh thin met** (policy+inventory): after W534 handle C ABI true-delete, the W531 migrate-blocker table is **stale** (still claimed handle ABI present + nested dual-track unmet). This window **re-signs** migrate-flip blockers: emit/opcode RC, Value API, opcode ABI, nested write-path, and handle C ABI are **cleared**; remaining open are `object_handle_*` table, mir-run `OBJECT_REF_*` opcodes, ObjectRef `{ rc }` storage, dual-track↔RC coexistence, deep field-graph GC walk, and `objectref_rc_migrate_enabled` **still false**. This window does **not** flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, peel the handle table, strip RC storage, deep-walk, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**（退出加权 ≈75–76%）；≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-eighty-fifth` / `A-W535`.

## Index

| Prior | Status |
|-------|--------|
| W476–W534 narrative → handle C ABI true delete | **met** |
| W531 migrate-readiness refresh thin (historical snapshot) | **met** (table **stale** after W532/W534; refresh lives here) |
| This window post-handle-delete migrate-readiness refresh thin | **met** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-x | Tracks → RC retire → nested write-path → handle C ABI true delete | **met** (W494–W534) |
| 4c-ii-u2. Migrate-readiness refresh (W531) | Historical blockers signed; migrate still off | **met** (historical; table stale) |
| 4c-ii-u3. Post-handle-delete migrate-readiness refresh | Blockers re-signed after W532/W534; migrate still off | **met** (this window) |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Migrate blocker table (post-handle-delete refresh; hard)

| Blocker | Present? | Meaning |
|---------|----------|---------|
| LLVM/WASM ObjectRef RC emit path | **cleared** | W511/W514 floors |
| Opcode RC (Retain/Dup/LoadLocal/Release/StoreLocal/deinit) | **cleared** | W522–W528 floors |
| Value `object_ref_retain` / `object_ref_release` API | **cleared** | W529 |
| RT opcode ABI wrap/retain/release | **cleared** | W530 |
| Nested write-path dual-track | **met** | W532 (`nested_field_write_path_dual_track_met`) |
| Handle C ABI `syx_rt_object_retain/release/drop` | **deleted** | W534 (`rt_abi_object_handle_retain_release_drop_deleted`) |
| `object_handle_*` table API | **still present** | `object_handle_table_api_still_present` |
| mir-run `OBJECT_REF_*` opcodes | **still present** | `mir_run_object_ref_opcodes_still_present` |
| `Value::ObjectRef { rc }` storage | **still present** | `objectref_rc_storage_still_present` |
| Dual-track ManagedHeap/`heap_slots` ↔ RC storage coexistence | **noted** | `dual_track_and_rc_storage_coexistence_noted` |
| Deep field-graph GC walk | **still unmet** | `deep_field_graph_gc_walk_still_unmet` |
| `objectref_rc_migrate_enabled` | **no** | still false |
| B-09 axis exit (`[x]`) / GC GA | **no** | refresh ≠ exit |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| Post-handle-delete migrate-readiness refresh signed | **yes** | This window |
| Full migrate enabled | **no** | Still open |
| Storage cleanup enabled | **no** | Still open |
| Unified heap enabled | **no** | Frozen |
| B-09 `[x]` / GC GA | **no** | refresh ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectrefRcMigrateReadinessPostHandleDeleteRefreshThinInventory` / `gate_b09_objectref_rc_migrate_readiness_post_handle_delete_refresh_thin_inventory` |
| Flags | `b09_objectref_rc_migrate_readiness_post_handle_delete_refresh_thin_signed=true`; `b09_rt_abi_objectref_handle_retain_release_drop_delete_first_slice_floor_ok=true`; `rt_abi_object_handle_retain_release_drop_deleted=true`; `nested_field_write_path_dual_track_met=true`; `value_object_ref_retain_release_api_deleted=true`; `rt_abi_object_ref_wrap_retain_release_deleted=true`; `object_handle_table_api_still_present=true`; `mir_run_object_ref_opcodes_still_present=true`; `objectref_rc_storage_still_present=true`; `deep_field_graph_gc_walk_still_unmet=true`; `dual_track_and_rc_storage_coexistence_noted=true`; `migrate_flip_blockers_refreshed_after_handle_delete=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / peeling handle table / stripping RC storage / deep walk / HostFn+1 / MIR distance change / claiming refresh as full migrate |
| Honesty | post-handle-delete migrate-readiness **refresh** **≠** migrate flip / **≠** strip RC / **≠** deep walk / **≠** peel 表 / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA；W518/W531 historical inventory snapshots **not rewritten** |

## Evidence

- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectrefRcMigrateReadinessPostHandleDeleteRefreshThinInventory`
- Meta: ~~`b09_objectref_rc_migrate_readiness_post_handle_delete_refresh_thin.rs`~~ retired（F-10 W604）；proof = this SSOT + `GateB09ObjectrefRcMigrateReadinessPostHandleDeleteRefreshThinInventory`
- Floor: [`gate-b09-rt-abi-objectref-handle-retain-release-drop-delete-first-slice.md`](gate-b09-rt-abi-objectref-handle-retain-release-drop-delete-first-slice.md)
- Historical (do not rewrite Contract fields): [`gate-b09-objectref-rc-migrate-readiness-refresh-thin.md`](gate-b09-objectref-rc-migrate-readiness-refresh-thin.md), [`gate-b09-objectref-rc-migrate-readiness-thin.md`](gate-b09-objectref-rc-migrate-readiness-thin.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Flipping `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Peeling `object_handle_*` table; stripping mir-run `OBJECT_REF_*`; stripping `Value::ObjectRef { rc }` storage; deep field-graph GC walk
- Rewriting W518/W531 Contract table field claims as if current; rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w534_*.plan.md`

## Next

W534 B-09 RT ABI handle retain·release·drop true-delete first-slice **已交**（ADR-878）→ W535 B-09 ObjectRef RC migrate-readiness post-handle-delete refresh thin **已交**（ADR-879）→ W536 B-09 ObjectRef strip-RC readiness thin **已交**（ADR-880）→ W537 B-09 ObjectRef strip-RC true-delete first-slice **已交**（ADR-881）→ W538 B-09 ObjectRef deep field-graph GC walk readiness thin **已交**（ADR-882）→ W539 B-09 ObjectRef deep field-graph GC walk true first-slice **已交**（ADR-883）→ W540 B-09 ObjectRef peel 表·opcodes readiness thin **已交**（ADR-884）→ W541 B-09 ObjectRef peel 表·opcodes true-delete first-slice **已交**（ADR-885）→ W542 B-09 ObjectRef RC migrate-readiness post-peel refresh thin **已交**（ADR-886）→ W543 B-09 ObjectRef MirOp/VM residual peel readiness thin **已交**（ADR-887）→ W544 B-09 ObjectRef MirOp/VM residual peel true-delete first-slice **已交**（ADR-888）→ W545 B-09 ObjectRef RC migrate-readiness post-mirop-peel refresh thin **已交**（ADR-889）→ W546 B-09 ObjectRef dual-track collapse readiness thin **已交**（ADR-890）→ W547 B-09 ObjectRef dual-track collapse true-delete first-slice **已交**（ADR-891）→ W548 B-09 ObjectRef RC migrate-readiness post-collapse refresh thin **已交**（ADR-892）→ W549 B-09 ObjectRef re-entry maybe_track residual peel readiness thin **已交**（ADR-893）→ W550 B-09 ObjectRef re-entry maybe_track residual peel true-delete first-slice **已交**（ADR-894）→ W551 B-09 ObjectRef RC migrate-readiness post-reentry-peel refresh thin **已交**（ADR-895）→ W552 B-09 MutRefCell maybe_track residual peel readiness thin **已交**（ADR-896）→ W553 B-09 MutRefCell maybe_track residual peel true-delete first-slice **已交**（ADR-897）→ W554 B-09 ObjectRef RC migrate-readiness post-mutrefcell-peel refresh thin **已交**（ADR-898）→ W555 B-09 ObjectRef RC migrate product flip first-slice **已交**（ADR-899）→ W556 B-09 storage-cleanup product flip first-slice **已交**（ADR-900）→ W557 B-09 unified MirRuntime heap enable product flip first-slice **已交**（ADR-901）→ W558+ — B-09 `[x]` / GC GA；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
