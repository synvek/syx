# B-09 ObjectRef RC migrate-readiness refresh thin (W531 / ADR-875)

## Status

**B-09 ObjectRef RC migrate-readiness refresh thin met** (policy+inventory): after W530 RT opcode ABI wrap/retain/release true-delete, the W518 migrate-blocker table is **stale** (still claimed Value RC API + wrap/retain/release ABI present). This window **re-signs** migrate-flip blockers: emit RC, opcode RC, Value API, and opcode ABI are **cleared**; remaining open are ObjectRef `{ rc }` storage, handle ABI, dual-track↔RC storage coexistence, nested field dual-track unmet, and `objectref_rc_migrate_enabled` **still false**. This window does **not** flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, delete handle ABI, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-eighty-first` / `A-W531`.

## Index

| Prior | Status |
|-------|--------|
| W476–W530 narrative → RT opcode ABI true delete | **met** |
| W518 migrate-readiness thin (historical snapshot) | **met** (table **stale** after W529/W530; refresh lives here) |
| This window migrate-readiness refresh thin | **met** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-y2 | Tracks → Value API delete → RT opcode ABI true delete | **met** (W494–W530) |
| 4c-ii-u. Migrate-readiness (W518) | Historical blockers signed; migrate still off | **met** (historical; table stale) |
| 4c-ii-u2. Migrate-readiness refresh | Blockers re-signed after W529/W530; migrate still off | **met** (this window) |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Migrate blocker table (refresh; hard)

| Blocker | Present? | Meaning |
|---------|----------|---------|
| LLVM/WASM ObjectRef RC emit path | **cleared** | W511/W514 floors |
| Opcode RC (Retain/Dup/LoadLocal/Release/StoreLocal/deinit) | **cleared** | W522–W528 floors |
| Value `object_ref_retain` / `object_ref_release` API | **cleared** | W529 (`value_object_ref_retain_release_api_deleted`) |
| RT opcode ABI wrap/retain/release | **cleared** | W530 (`rt_abi_object_ref_wrap_retain_release_deleted`) |
| `Value::ObjectRef { rc }` storage | **still present** | `objectref_rc_storage_still_present` |
| Handle ABI `syx_rt_object_retain/release/drop` | **still present** | `rt_abi_object_handle_retain_release_drop_still_present` |
| Dual-track ManagedHeap/`heap_slots` ↔ RC storage coexistence | **noted** | `dual_track_and_rc_storage_coexistence_noted` |
| Nested field dual-track | **still unmet** | `nested_field_dual_track_still_unmet` |
| mir-run `OBJECT_REF_*` opcode paths | **still present** | honesty residual (not product emit) |
| `objectref_rc_migrate_enabled` | **no** | still false |
| B-09 axis exit (`[x]`) / GC GA | **no** | refresh ≠ exit |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| Migrate-readiness refresh signed | **yes** | This window |
| Full migrate enabled | **no** | Still open |
| Storage cleanup enabled | **no** | Still open |
| Unified heap enabled | **no** | Frozen |
| B-09 `[x]` / GC GA | **no** | refresh ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectrefRcMigrateReadinessRefreshThinInventory` / `gate_b09_objectref_rc_migrate_readiness_refresh_thin_inventory` |
| Flags | `b09_objectref_rc_migrate_readiness_refresh_thin_signed=true`; `b09_rt_abi_objectref_wrap_retain_release_delete_first_slice_floor_ok=true`; `value_object_ref_retain_release_api_deleted=true`; `rt_abi_object_ref_wrap_retain_release_deleted=true`; `objectref_rc_storage_still_present=true`; `rt_abi_object_handle_retain_release_drop_still_present=true`; `dual_track_and_rc_storage_coexistence_noted=true`; `nested_field_dual_track_still_unmet=true`; `migrate_flip_blockers_refreshed=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / deleting handle ABI / HostFn+1 / MIR distance change / claiming refresh as full migrate |
| Honesty | migrate-readiness **refresh** **≠** migrate flip / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA；W518 historical inventory snapshot **not rewritten** |

## Evidence

- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectrefRcMigrateReadinessRefreshThinInventory`
- Meta: ~~`b09_objectref_rc_migrate_readiness_refresh_thin.rs`~~ retired（F-10 W603）；proof = this SSOT + `GateB09ObjectrefRcMigrateReadinessRefreshThinInventory`
- Floor: [`gate-b09-rt-abi-objectref-wrap-retain-release-delete-first-slice.md`](gate-b09-rt-abi-objectref-wrap-retain-release-delete-first-slice.md)
- Historical (do not rewrite Contract fields): [`gate-b09-objectref-rc-migrate-readiness-thin.md`](gate-b09-objectref-rc-migrate-readiness-thin.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Peeling RC storage / stripping mir-run `OBJECT_REF_*` this window (historical Frozen at W531); nested field dual-track was later met at W532; handle C ABI later deleted at W534
- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Rewriting W518 Contract table field claims as if current; rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w530_*.plan.md`

## Next

W530 B-09 RT ABI wrap·retain·release true-delete first-slice **已交**（ADR-874）→ W531 B-09 ObjectRef RC migrate-readiness refresh thin **已交**（ADR-875）→ W532 B-09 ObjectRef nested field dual-track first-slice **已交**（ADR-876）→ W533 B-09 RT ABI handle retain·release·drop retire-readiness thin **已交**（ADR-877）→ W534 B-09 RT ABI handle retain·release·drop true-delete first-slice **已交**（ADR-878）→ W535 B-09 ObjectRef RC migrate-readiness post-handle-delete refresh thin **已交**（ADR-879）→ W536 B-09 ObjectRef strip-RC readiness thin **已交**（ADR-880）→ W537 B-09 ObjectRef strip-RC true-delete first-slice **已交**（ADR-881）→ W538 B-09 ObjectRef deep field-graph GC walk readiness thin **已交**（ADR-882）→ W539 B-09 ObjectRef deep field-graph GC walk true first-slice **已交**（ADR-883）→ W540 B-09 ObjectRef peel 表·opcodes readiness thin **已交**（ADR-884）→ W541 B-09 ObjectRef peel 表·opcodes true-delete first-slice **已交**（ADR-885）→ W542+ — 真产品迁 / 翻 cleanup / 真 enable；或另钉 MirOp/VM residual peel；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
