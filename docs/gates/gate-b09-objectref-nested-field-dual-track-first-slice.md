# B-09 ObjectRef nested field dual-track first-slice (W532 / ADR-876)

## Status

**B-09 ObjectRef nested field dual-track first-slice met** (write-path): after W531 migrate-readiness refresh, VM `Instruction::ObjectSet` and mir-run `object_set` dual-register the nested write-path `val`/`value` into ManagedHeap + `heap_slots` **before** `mut_ref_cell_set_field`, while **keeping** the W515 container stack-top dual-track. This window does **not** deep field-graph GC walk, ArraySet nested track, delete handle ABI, flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**（退出加权 ≈72–73%）；≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-eighty-second` / `A-W532`.

## Index

| Prior | Status |
|-------|--------|
| W476–W531 narrative → migrate-readiness refresh | **met** |
| ObjectSet container dual-track (W515) | **met** (retained) |
| This window nested write-path dual-track | **met** |
| Deep field-graph GC walk | **still unmet** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-u2 | Tracks → RC retire → Value/ABI delete → migrate-readiness refresh | **met** (W494–W531) |
| 4c-ii-v. Nested field write-path dual-track | ObjectSet nested `val` → ManagedHeap + `heap_slots` | **met** (this window) |
| 4c-ii-w. Deep field-graph GC walk | Walk nested ObjectRef fields as GC roots | **still unmet** |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Blockers (honesty)

| Gap | Present? | Meaning |
|-----|----------|---------|
| Nested write-path dual-track signed | **yes** | This window |
| Deep field-graph GC walk | **no** | Still open |
| Handle ABI `syx_rt_object_*` | **yes (still live)** | W533+ |
| `Value::ObjectRef { rc }` storage | **yes (still present)** | Still open |
| Full migrate enabled | **no** | Still open |
| B-09 `[x]` / GC GA | **no** | write-path ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectrefNestedFieldDualTrackFirstSliceInventory` / `gate_b09_objectref_nested_field_dual_track_first_slice_inventory` |
| Flags | `b09_objectref_nested_field_dual_track_first_slice_signed=true`; `b09_objectref_rc_migrate_readiness_refresh_thin_floor_ok=true`; `vm_objectset_nested_value_dual_track_wired=true`; `mir_run_objectset_nested_value_dual_track_wired=true`; `nested_field_write_path_dual_track_met=true`; `objectref_rc_storage_still_present=true`; `rt_abi_object_handle_retain_release_drop_still_present=true`; `dual_track_and_rc_storage_coexistence_noted=true`; `deep_field_graph_gc_walk_still_unmet=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / deep walk met / handle ABI deleted / HostFn+1 / MIR distance change / claiming write-path as full migrate |
| Honesty | nested write-path dual-track **≠** deep field-graph GC walk / **≠** migrate flip / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA；W531 historical inventory snapshot **not rewritten**（`nested_field_dual_track_still_unmet` remains historical W531 truth） |

## Evidence

- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) — ObjectSet nested `val` dual-track / ADR-876
- [`crates/syx-mir-run/src/runtime.rs`](../../crates/syx-mir-run/src/runtime.rs) — `object_set` nested dual-track / ADR-876
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectrefNestedFieldDualTrackFirstSliceInventory`
- Meta: `b09_objectref_nested_field_dual_track_first_slice.rs`
- Floor: [`gate-b09-objectref-rc-migrate-readiness-refresh-thin.md`](gate-b09-objectref-rc-migrate-readiness-refresh-thin.md)
- ObjectSet container floor: [`gate-b09-objectref-objectset-heap-slots-first-slice.md`](gate-b09-objectref-objectset-heap-slots-first-slice.md)
- Residual birth: [`gate-b09-objectref-residual-birth-heap-slots-first-slice.md`](gate-b09-objectref-residual-birth-heap-slots-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Flipping `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Deep field-graph GC walk; ArraySet nested track; deleting handle ABI; stripping mir-run `OBJECT_REF_*`; peeling RC storage
- Rewriting W531 Contract table field claims as if current; rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w531_*.plan.md`

## Next

W531 B-09 ObjectRef RC migrate-readiness refresh thin **已交**（ADR-875）→ W532 B-09 ObjectRef nested field dual-track first-slice **已交**（ADR-876）→ W533 B-09 RT ABI handle retain·release·drop retire-readiness thin **已交**（ADR-877）→ W534 B-09 RT ABI handle retain·release·drop true-delete first-slice **已交**（ADR-878）→ W535 B-09 ObjectRef RC migrate-readiness post-handle-delete refresh thin **已交**（ADR-879）→ W536 B-09 ObjectRef strip-RC readiness thin **已交**（ADR-880）→ W537 B-09 ObjectRef strip-RC true-delete first-slice **已交**（ADR-881）→ W538 B-09 ObjectRef deep field-graph GC walk readiness thin **已交**（ADR-882）→ W539 B-09 ObjectRef deep field-graph GC walk true first-slice **已交**（ADR-883）→ W540 B-09 ObjectRef peel 表·opcodes readiness thin **已交**（ADR-884）→ W541 B-09 ObjectRef peel 表·opcodes true-delete first-slice **已交**（ADR-885）→ W542+ — 真产品迁 / 翻 cleanup / 真 enable；或另钉 MirOp/VM residual peel；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
