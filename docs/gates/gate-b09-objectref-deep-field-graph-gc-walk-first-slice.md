# B-09 ObjectRef deep field-graph GC walk true first-slice (W539 / ADR-883)

## Status

**B-09 ObjectRef deep field-graph GC walk true first-slice met**: after W538 deep-walk readiness, product deep field-graph GC walk is **signed** with a behavioral nail — parent ObjectRef as collect root keeps nested field ObjectRef live via `ManagedHeap::trace` + `object_ref_field_values` BFS; unreachable orphan ObjectRef is swept. `object_handle_*` table and mir-run `OBJECT_REF_*` opcodes **still present**. RC storage **deleted** (W537). `objectref_rc_migrate_enabled` **still false**. This window does **not** peel the handle table or mir-run opcodes, flip migrate / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**（退出加权 ≈81–84%）；≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-eighty-ninth` / `A-W539`.

## Index

| Prior | Status |
|-------|--------|
| W476–W538 narrative → deep-walk readiness | **met** |
| This window deep field-graph GC walk true first-slice | **met** |
| Peel `object_handle_*` table / mir-run `OBJECT_REF_*` | **still unmet** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-w0 | Tracks → strip-RC → deep-walk readiness | **met** (W494–W538) |
| 4c-ii-w. Deep field-graph GC walk | Product walk signed + behavioral nail | **met** (this window) |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Deep-walk honesty table (hard)

| Item | Present? | Meaning |
|------|----------|---------|
| Product deep field-graph GC walk | **met** | `deep_field_graph_gc_walk_met` |
| Behavioral nail (nested live / orphan swept) | **ok** | `deep_field_graph_gc_walk_behavioral_nail_ok` |
| `ManagedHeap::trace` ObjectRef field BFS | **wired** | `managed_heap_trace_object_ref_fields_wired` |
| `object_handle_*` table API | **still present** | peel still open |
| mir-run `OBJECT_REF_*` opcodes | **still present** | peel still open |
| `Value::ObjectRef { rc }` storage | **deleted** | W537 floor |
| Dual-track ManagedHeap/`heap_slots` coexistence | **noted** | honesty |
| `objectref_rc_migrate_enabled` | **no** | still false |
| B-09 axis exit (`[x]`) / GC GA | **no** | walk ≠ exit |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| Deep-walk true first-slice signed | **yes** | This window |
| Full migrate enabled | **no** | Still open |
| Storage cleanup enabled | **no** | Still open |
| Unified heap enabled | **no** | Frozen |
| B-09 `[x]` / GC GA | **no** | walk ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectrefDeepFieldGraphGcWalkFirstSliceInventory` / `gate_b09_objectref_deep_field_graph_gc_walk_first_slice_inventory` |
| Flags | `b09_objectref_deep_field_graph_gc_walk_first_slice_signed=true`; `b09_objectref_deep_field_graph_gc_walk_readiness_thin_floor_ok=true`; `objectref_rc_storage_deleted=true`; `object_handle_table_api_still_present=true`; `mir_run_object_ref_opcodes_still_present=true`; `deep_field_graph_gc_walk_met=true`; `deep_field_graph_gc_walk_behavioral_nail_ok=true`; `managed_heap_trace_object_ref_fields_wired=true`; `dual_track_and_heap_coexistence_noted=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | peeling handle table / stripping mir-run opcodes / flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / HostFn+1 / MIR distance change / claiming walk as migrate |
| Honesty | deep walk met **≠** peel 表 / **≠** peel opcodes / **≠** migrate flip / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA；W518/W531/W535/W536/W537/W538 historical inventory snapshots **not rewritten**（W538 `deep_field_graph_gc_walk_still_unmet=true` remains historical） |

## Evidence

- [`crates/syx-runtime-core/src/managed_heap.rs`](../../crates/syx-runtime-core/src/managed_heap.rs) — `trace` + `deep_field_graph_gc_walk_keeps_nested_sweeps_orphan` nail / ADR-883
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectrefDeepFieldGraphGcWalkFirstSliceInventory`
- Meta: `b09_objectref_deep_field_graph_gc_walk_first_slice.rs`
- Floor: [`gate-b09-objectref-deep-field-graph-gc-walk-readiness-thin.md`](gate-b09-objectref-deep-field-graph-gc-walk-readiness-thin.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Peeling `object_handle_*` table; stripping mir-run `OBJECT_REF_*`
- Flipping `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Rewriting W518/W531/W535/W536/W537/W538 Contract table field claims as if current
- Claiming ArraySet nested GA / full graph GA beyond this ObjectRef field-walk nail
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w538_*.plan.md`

## Next

W538 B-09 ObjectRef deep field-graph GC walk readiness thin **已交**（ADR-882）→ W539 B-09 ObjectRef deep field-graph GC walk true first-slice **已交**（ADR-883）→ W540 B-09 ObjectRef peel 表·opcodes readiness thin **已交**（ADR-884）→ W541 B-09 ObjectRef peel 表·opcodes true-delete first-slice **已交**（ADR-885）→ W542 B-09 ObjectRef RC migrate-readiness post-peel refresh thin **已交**（ADR-886）→ W543 B-09 ObjectRef MirOp/VM residual peel readiness thin **已交**（ADR-887）→ W544 B-09 ObjectRef MirOp/VM residual peel true-delete first-slice **已交**（ADR-888）→ W545 B-09 ObjectRef RC migrate-readiness post-mirop-peel refresh thin **已交**（ADR-889）→ W546 B-09 ObjectRef dual-track collapse readiness thin **已交**（ADR-890）→ W547 B-09 ObjectRef dual-track collapse true-delete first-slice **已交**（ADR-891）→ W548 B-09 ObjectRef RC migrate-readiness post-collapse refresh thin **已交**（ADR-892）→ W549 B-09 ObjectRef re-entry maybe_track residual peel readiness thin **已交**（ADR-893）→ W550 B-09 ObjectRef re-entry maybe_track residual peel true-delete first-slice **已交**（ADR-894）→ W551 B-09 ObjectRef RC migrate-readiness post-reentry-peel refresh thin **已交**（ADR-895）→ W552 B-09 MutRefCell maybe_track residual peel readiness thin **已交**（ADR-896）→ W553 B-09 MutRefCell maybe_track residual peel true-delete first-slice **已交**（ADR-897）→ W554 B-09 ObjectRef RC migrate-readiness post-mutrefcell-peel refresh thin **已交**（ADR-898）→ W555 B-09 ObjectRef RC migrate product flip first-slice **已交**（ADR-899）→ W556 B-09 storage-cleanup product flip first-slice **已交**（ADR-900）→ W557 B-09 unified MirRuntime heap enable product flip first-slice **已交**（ADR-901）→ W558+ — B-09 `[x]` / GC GA；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
