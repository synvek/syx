# B-09 ObjectRef deep field-graph GC walk readiness thin (W538 / ADR-882)

## Status

**B-09 ObjectRef deep field-graph GC walk readiness thin met** (policy+inventory): after W537 strip-RC true-delete, deep-walk **blockers / preconditions** are signed — RC storage deleted (W537), nested write-path met (W532), handle C ABI deleted (W534), while product deep field-graph GC walk **still unmet**, `object_handle_*` table and mir-run `OBJECT_REF_*` opcodes still present, `ManagedHeap::trace` already BFS-pushes `object_ref_field_values` (**noted ≠ product deep-walk signed**), dual-track↔heap coexistence noted, and migrate remains **off**. This window does **not** claim deep walk met, peel the handle table or mir-run opcodes, flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**（退出加权 ≈79–81%）；≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-eighty-eighth` / `A-W538`.

## Index

| Prior | Status |
|-------|--------|
| W476–W537 narrative → strip-RC true-delete | **met** |
| This window deep field-graph GC walk readiness thin | **met** |
| True deep field-graph GC walk (product signed + behavioral nail) | **still unmet** |
| Peel `object_handle_*` table / mir-run `OBJECT_REF_*` | **still unmet** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-z | Tracks → RC retire → strip-RC true-delete | **met** (W494–W537) |
| 4c-ii-w0. Deep field-graph GC walk readiness | Blockers documented; walk still unmet | **met** (this window) |
| 4c-ii-w. Deep field-graph GC walk | Product walk signed + behavioral nail | **still unmet** |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Deep-walk blocker table (hard)

| Blocker | Present? | Meaning |
|---------|----------|---------|
| Strip-RC true-delete (W537) | **met** | `objectref_rc_storage_deleted` |
| Nested write-path dual-track (W532) | **met** | floor |
| Handle C ABI deleted (W534) | **met** | floor |
| `ManagedHeap::trace` ObjectRef field BFS | **noted** | `managed_heap_trace_object_ref_fields_noted` ≠ product signed |
| Product deep field-graph GC walk | **still unmet** | `deep_field_graph_gc_walk_still_unmet` |
| `object_handle_*` table API | **still present** | peel still open |
| mir-run `OBJECT_REF_*` opcodes | **still present** | peel still open |
| Dual-track ManagedHeap/`heap_slots` coexistence | **noted** | honesty |
| `objectref_rc_migrate_enabled` | **no** | still false |
| B-09 axis exit (`[x]`) / GC GA | **no** | readiness ≠ exit |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| Deep-walk readiness signed | **yes** | This window |
| Deep walk product met | **no** | Still open |
| Full migrate enabled | **no** | Still open |
| Storage cleanup enabled | **no** | Still open |
| Unified heap enabled | **no** | Frozen |
| B-09 `[x]` / GC GA | **no** | readiness ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectrefDeepFieldGraphGcWalkReadinessThinInventory` / `gate_b09_objectref_deep_field_graph_gc_walk_readiness_thin_inventory` |
| Flags | `b09_objectref_deep_field_graph_gc_walk_readiness_thin_signed=true`; `b09_objectref_strip_rc_delete_first_slice_floor_ok=true`; `objectref_rc_storage_deleted=true`; `object_handle_table_api_still_present=true`; `mir_run_object_ref_opcodes_still_present=true`; `deep_field_graph_gc_walk_still_unmet=true`; `deep_field_graph_gc_walk_blockers_documented=true`; `managed_heap_trace_object_ref_fields_noted=true`; `dual_track_and_heap_coexistence_noted=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | claiming deep walk met / peeling handle table / stripping mir-run opcodes / flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / HostFn+1 / MIR distance change / claiming readiness as deep walk |
| Honesty | deep-walk readiness **≠** deep walk met / **≠** peel 表 / **≠** peel opcodes / **≠** migrate flip / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA；W518/W531/W535/W536/W537 historical inventory snapshots **not rewritten** |

## Evidence

- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectrefDeepFieldGraphGcWalkReadinessThinInventory`
- Meta: ~~`b09_objectref_deep_field_graph_gc_walk_readiness_thin.rs`~~ retired（F-10 W602）；proof = this SSOT + `GateB09ObjectrefDeepFieldGraphGcWalkReadinessThinInventory`
- Floor: [`gate-b09-objectref-strip-rc-delete-first-slice.md`](gate-b09-objectref-strip-rc-delete-first-slice.md)
- Trace note: [`crates/syx-runtime-core/src/managed_heap.rs`](../../crates/syx-runtime-core/src/managed_heap.rs) `trace` + `object_ref_field_values`
- Shape: [`gate-b09-objectref-strip-rc-readiness-thin.md`](gate-b09-objectref-strip-rc-readiness-thin.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Claiming product deep field-graph GC walk met; peeling `object_handle_*` table; stripping mir-run `OBJECT_REF_*`
- Flipping `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Rewriting W518/W531/W535/W536/W537 Contract table field claims as if current
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w537_*.plan.md`

## Next

W537 B-09 ObjectRef strip-RC true-delete first-slice **已交**（ADR-881）→ W538 B-09 ObjectRef deep field-graph GC walk readiness thin **已交**（ADR-882）→ W539 B-09 ObjectRef deep field-graph GC walk true first-slice **已交**（ADR-883）→ W540 B-09 ObjectRef peel 表·opcodes readiness thin **已交**（ADR-884）→ W541 B-09 ObjectRef peel 表·opcodes true-delete first-slice **已交**（ADR-885）→ W542 B-09 ObjectRef RC migrate-readiness post-peel refresh thin **已交**（ADR-886）→ W543 B-09 ObjectRef MirOp/VM residual peel readiness thin **已交**（ADR-887）→ W544 B-09 ObjectRef MirOp/VM residual peel true-delete first-slice **已交**（ADR-888）→ W545 B-09 ObjectRef RC migrate-readiness post-mirop-peel refresh thin **已交**（ADR-889）→ W546 B-09 ObjectRef dual-track collapse readiness thin **已交**（ADR-890）→ W547 B-09 ObjectRef dual-track collapse true-delete first-slice **已交**（ADR-891）→ W548 B-09 ObjectRef RC migrate-readiness post-collapse refresh thin **已交**（ADR-892）→ W549 B-09 ObjectRef re-entry maybe_track residual peel readiness thin **已交**（ADR-893）→ W550 B-09 ObjectRef re-entry maybe_track residual peel true-delete first-slice **已交**（ADR-894）→ W551 B-09 ObjectRef RC migrate-readiness post-reentry-peel refresh thin **已交**（ADR-895）→ W552 B-09 MutRefCell maybe_track residual peel readiness thin **已交**（ADR-896）→ W553 B-09 MutRefCell maybe_track residual peel true-delete first-slice **已交**（ADR-897）→ W554 B-09 ObjectRef RC migrate-readiness post-mutrefcell-peel refresh thin **已交**（ADR-898）→ W555 B-09 ObjectRef RC migrate product flip first-slice **已交**（ADR-899）→ W556 B-09 storage-cleanup product flip first-slice **已交**（ADR-900）→ W557 B-09 unified MirRuntime heap enable product flip first-slice **已交**（ADR-901）→ W558+ — B-09 `[x]` / GC GA；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
