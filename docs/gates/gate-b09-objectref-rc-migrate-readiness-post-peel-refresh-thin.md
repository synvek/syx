# B-09 ObjectRef RC migrate-readiness post-peel refresh thin (W542 / ADR-886)

## Status

**B-09 ObjectRef RC migrate-readiness post-peel refresh thin met** (policy+inventory): after W541 peel table·opcodes true-delete, the W535 migrate-blocker table is **stale** (still claimed `object_handle_*` table / mir-run opcodes / RC storage / deep walk open). This window **re-signs** migrate-flip blockers: emit/opcode RC, Value API, opcode ABI, nested write-path, handle C ABI, RC storage, deep walk, and handle-table + mir-run opcode peel are **cleared/met**; remaining open are MirOp `ObjectRefWrap/Retain/Release` + VM `ObjectRefRetain/Release` residual, dual-track↔heap coexistence, and `objectref_rc_migrate_enabled` **still false**. This window does **not** flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, peel MirOp/VM ObjectRef residual, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**（退出加权 ≈85–88%）；≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-ninety-second` / `A-W542`.

## Index

| Prior | Status |
|-------|--------|
| W476–W541 narrative → peel table·opcodes true-delete | **met** |
| W535 post-handle-delete migrate-readiness refresh thin (historical snapshot) | **met** (table **stale** after W537–W541; refresh lives here) |
| This window post-peel migrate-readiness refresh thin | **met** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| MirOp/VM ObjectRef residual peel | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … peel true-delete | Tracks → RC retire → strip-RC → deep-walk → peel | **met** (W494–W541) |
| 4c-ii-u3. Post-handle-delete migrate-readiness refresh (W535) | Historical blockers signed; migrate still off | **met** (historical; table stale) |
| 4c-ii-u4. Post-peel migrate-readiness refresh | Blockers re-signed after W537–W541; migrate still off | **met** (this window) |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Migrate blocker table (post-peel refresh; hard)

| Blocker | Present? | Meaning |
|---------|----------|---------|
| LLVM/WASM ObjectRef RC emit path | **cleared** | W511/W514 floors |
| Opcode RC (Retain/Dup/LoadLocal/Release/StoreLocal/deinit) | **cleared** | W522–W528 floors |
| Value `object_ref_retain` / `object_ref_release` API | **cleared** | W529 |
| RT opcode ABI wrap/retain/release | **cleared** | W530 |
| Nested write-path dual-track | **met** | W532 (`nested_field_write_path_dual_track_met`) |
| Handle C ABI `syx_rt_object_retain/release/drop` | **deleted** | W534 (`rt_abi_object_handle_retain_release_drop_deleted`) |
| `Value::ObjectRef { rc }` storage | **deleted** | W537 (`objectref_rc_storage_deleted`) |
| Deep field-graph GC walk | **met** | W539 (`deep_field_graph_gc_walk_met`) |
| `object_handle_*` table API | **deleted** | W541 (`object_handle_table_api_deleted`) |
| mir-run `OBJECT_REF_*` opcodes | **deleted** | W541 (`mir_run_object_ref_opcodes_deleted`) |
| Handle-table + mir-run opcode peel | **met** | W541 (`handle_table_mir_run_opcode_peel_met`) |
| MirOp `ObjectRefWrap/Retain/Release` | **still present** | `mir_op_objectref_wrap_retain_release_still_present` |
| VM `ObjectRefRetain/Release` | **still present** | `vm_instruction_objectref_retain_release_still_present` |
| Dual-track ManagedHeap/`heap_slots` coexistence | **noted** | `dual_track_and_heap_coexistence_noted` |
| `objectref_rc_migrate_enabled` | **no** | still false |
| B-09 axis exit (`[x]`) / GC GA | **no** | refresh ≠ exit |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| Post-peel migrate-readiness refresh signed | **yes** | This window |
| Full migrate enabled | **no** | Still open |
| Storage cleanup enabled | **no** | Still open |
| Unified heap enabled | **no** | Frozen |
| MirOp/VM residual peel | **no** | Still open |
| B-09 `[x]` / GC GA | **no** | refresh ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectrefRcMigrateReadinessPostPeelRefreshThinInventory` / `gate_b09_objectref_rc_migrate_readiness_post_peel_refresh_thin_inventory` |
| Flags | `b09_objectref_rc_migrate_readiness_post_peel_refresh_thin_signed=true`; `b09_objectref_peel_delete_first_slice_floor_ok=true`; `handle_table_mir_run_opcode_peel_met=true`; `object_handle_table_api_deleted=true`; `mir_run_object_ref_opcodes_deleted=true`; `objectref_rc_storage_deleted=true`; `deep_field_graph_gc_walk_met=true`; `nested_field_write_path_dual_track_met=true`; `rt_abi_object_handle_retain_release_drop_deleted=true`; `mir_op_objectref_wrap_retain_release_still_present=true`; `vm_instruction_objectref_retain_release_still_present=true`; `dual_track_and_heap_coexistence_noted=true`; `migrate_flip_blockers_refreshed_after_peel=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / peeling MirOp/VM ObjectRef residual / HostFn+1 / MIR distance change / claiming refresh as full migrate |
| Honesty | post-peel migrate-readiness **refresh** **≠** migrate flip / **≠** MirOp/VM peel / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA；W518/W531/W535–W541 historical inventory snapshots **not rewritten** |

## Evidence

- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectrefRcMigrateReadinessPostPeelRefreshThinInventory`
- Meta: ~~`b09_objectref_rc_migrate_readiness_post_peel_refresh_thin.rs`~~ retired（F-10 W604）；proof = this SSOT + `GateB09ObjectrefRcMigrateReadinessPostPeelRefreshThinInventory`
- Floor: [`gate-b09-objectref-peel-delete-first-slice.md`](gate-b09-objectref-peel-delete-first-slice.md)
- Historical (do not rewrite Contract fields): [`gate-b09-objectref-rc-migrate-readiness-post-handle-delete-refresh-thin.md`](gate-b09-objectref-rc-migrate-readiness-post-handle-delete-refresh-thin.md), [`gate-b09-objectref-rc-migrate-readiness-refresh-thin.md`](gate-b09-objectref-rc-migrate-readiness-refresh-thin.md), [`gate-b09-objectref-rc-migrate-readiness-thin.md`](gate-b09-objectref-rc-migrate-readiness-thin.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Flipping `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Deleting `MirOp::ObjectRef*` enum / VM `ObjectRefRetain/Release` instructions this window
- Rewriting W518/W531/W535–W541 Contract table field claims as if current; rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w541_*.plan.md`

## Next

W541 B-09 ObjectRef peel 表·opcodes true-delete first-slice **已交**（ADR-885）→ W542 B-09 ObjectRef RC migrate-readiness post-peel refresh thin **已交**（ADR-886）→ W543 B-09 ObjectRef MirOp/VM residual peel readiness thin **已交**（ADR-887）→ W544 B-09 ObjectRef MirOp/VM residual peel true-delete first-slice **已交**（ADR-888）→ W545 B-09 ObjectRef RC migrate-readiness post-mirop-peel refresh thin **已交**（ADR-889）→ W546 B-09 ObjectRef dual-track collapse readiness thin **已交**（ADR-890）→ W547 B-09 ObjectRef dual-track collapse true-delete first-slice **已交**（ADR-891）→ W548 B-09 ObjectRef RC migrate-readiness post-collapse refresh thin **已交**（ADR-892）→ W549 B-09 ObjectRef re-entry maybe_track residual peel readiness thin **已交**（ADR-893）→ W550 B-09 ObjectRef re-entry maybe_track residual peel true-delete first-slice **已交**（ADR-894）→ W551 B-09 ObjectRef RC migrate-readiness post-reentry-peel refresh thin **已交**（ADR-895）→ W552 B-09 MutRefCell maybe_track residual peel readiness thin **已交**（ADR-896）→ W553 B-09 MutRefCell maybe_track residual peel true-delete first-slice **已交**（ADR-897）→ W554 B-09 ObjectRef RC migrate-readiness post-mutrefcell-peel refresh thin **已交**（ADR-898）→ W555 B-09 ObjectRef RC migrate product flip first-slice **已交**（ADR-899）→ W556 B-09 storage-cleanup product flip first-slice **已交**（ADR-900）→ W557 B-09 unified MirRuntime heap enable product flip first-slice **已交**（ADR-901）→ W558+ — B-09 `[x]` / GC GA；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
