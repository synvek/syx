# B-09 ObjectRef peel 表·opcodes true-delete first-slice (W541 / ADR-885)

## Status

**B-09 ObjectRef peel 表·opcodes true-delete first-slice met**: after W540 peel readiness, `object_handle_*` table API is **deleted** and mir-run `OBJECT_REF_*` opcode constants + `exec()` arms are **stripped**. `object_ref_wrap` keeps ManagedHeap dual-track without handle registration. `MirOp::ObjectRef*` and VM `Instruction::ObjectRefRetain/Release` **still present** (honesty residual). Product deep walk **met** (W539). RC storage **deleted** (W537). `objectref_rc_migrate_enabled` **still false**. This window does **not** flip migrate / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**（退出加权 ≈84–87%）；≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-ninety-first` / `A-W541`.

## Index

| Prior | Status |
|-------|--------|
| W476–W540 narrative → peel readiness | **met** |
| This window peel 表·opcodes true-delete | **met** |
| MirOp / VM ObjectRef residual peel | **still unmet** (still present) |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-p0 | Tracks → deep-walk → peel readiness | **met** (W494–W540) |
| 4c-ii-p. True peel (table + mir-run opcodes) | Delete table + strip opcodes | **met** (this window) |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Peel honesty table (hard)

| Item | Present? | Meaning |
|------|----------|---------|
| Handle table + mir-run opcode peel | **met** | this window |
| `object_handle_*` table API | **deleted** | peel target |
| mir-run `OBJECT_REF_*` opcodes | **deleted** | peel target |
| Wrap without handle registration | **wired** | dual-track only |
| `MirOp::ObjectRefWrap/Retain/Release` | **still present** | residual |
| VM `ObjectRefRetain/Release` | **still present** | residual |
| Product deep field-graph GC walk | **met** | W539 floor |
| `Value::ObjectRef { rc }` storage | **deleted** | W537 floor |
| Dual-track ManagedHeap/`heap_slots` coexistence | **noted** | honesty |
| `objectref_rc_migrate_enabled` | **no** | still false |
| B-09 axis exit (`[x]`) / GC GA | **no** | peel ≠ exit |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| Peel true-delete signed | **yes** | This window |
| MirOp/VM residual peel | **no** | Still open |
| Full migrate enabled | **no** | Still open |
| Storage cleanup enabled | **no** | Still open |
| Unified heap enabled | **no** | Frozen |
| B-09 `[x]` / GC GA | **no** | peel ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectrefPeelDeleteFirstSliceInventory` / `gate_b09_objectref_peel_delete_first_slice_inventory` |
| Flags | `b09_objectref_peel_delete_first_slice_signed=true`; `b09_objectref_peel_readiness_thin_floor_ok=true`; `deep_field_graph_gc_walk_met=true`; `objectref_rc_storage_deleted=true`; `object_handle_table_api_deleted=true`; `mir_run_object_ref_opcodes_deleted=true`; `handle_table_mir_run_opcode_peel_met=true`; `object_ref_wrap_without_handle_registration_wired=true`; `run_class_deinits_no_object_handle_drop_wired=true`; `release_object_ref_for_handle_drop_deleted=true`; `mir_op_objectref_wrap_retain_release_still_present=true`; `vm_instruction_objectref_retain_release_still_present=true`; `dual_track_and_heap_coexistence_noted=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / HostFn+1 / MIR distance change / claiming MirOp/VM peel / claiming peel as migrate |
| Honesty | peel met **≠** MirOp/VM peel / **≠** migrate flip / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA；W518/W531/W535–W540 historical inventory snapshots **not rewritten** |

## Evidence

- Deleted: `crates/syx-runtime-core/src/object_handles.rs`
- [`crates/syx-mir-run/src/runtime.rs`](../../crates/syx-mir-run/src/runtime.rs) — wrap without register; `exec_mir_op` ObjectRef arms
- [`crates/syx-mir-run/src/op.rs`](../../crates/syx-mir-run/src/op.rs) — OBJECT_REF_* constants removed
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectrefPeelDeleteFirstSliceInventory`
- Meta: `b09_objectref_peel_delete_first_slice.rs`
- Floor: [`gate-b09-objectref-peel-readiness-thin.md`](gate-b09-objectref-peel-readiness-thin.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Deleting `MirOp::ObjectRef*` enum / VM `ObjectRefRetain/Release` instructions this window
- Flipping `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Rewriting W518/W531/W535–W540 Contract table field claims as if current
- Claiming peel as migrate flip
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w540_*.plan.md`

## Next

W540 B-09 ObjectRef peel 表·opcodes readiness thin **已交**（ADR-884）→ W541 B-09 ObjectRef peel 表·opcodes true-delete first-slice **已交**（ADR-885）→ W542 B-09 ObjectRef RC migrate-readiness post-peel refresh thin **已交**（ADR-886）→ W543 B-09 ObjectRef MirOp/VM residual peel readiness thin **已交**（ADR-887）→ W544 B-09 ObjectRef MirOp/VM residual peel true-delete first-slice **已交**（ADR-888）→ W545 B-09 ObjectRef RC migrate-readiness post-mirop-peel refresh thin **已交**（ADR-889）→ W546 B-09 ObjectRef dual-track collapse readiness thin **已交**（ADR-890）→ W547 B-09 ObjectRef dual-track collapse true-delete first-slice **已交**（ADR-891）→ W548 B-09 ObjectRef RC migrate-readiness post-collapse refresh thin **已交**（ADR-892）→ W549 B-09 ObjectRef re-entry maybe_track residual peel readiness thin **已交**（ADR-893）→ W550 B-09 ObjectRef re-entry maybe_track residual peel true-delete first-slice **已交**（ADR-894）→ W551 B-09 ObjectRef RC migrate-readiness post-reentry-peel refresh thin **已交**（ADR-895）→ W552 B-09 MutRefCell maybe_track residual peel readiness thin **已交**（ADR-896）→ W553 B-09 MutRefCell maybe_track residual peel true-delete first-slice **已交**（ADR-897）→ W554 B-09 ObjectRef RC migrate-readiness post-mutrefcell-peel refresh thin **已交**（ADR-898）→ W555 B-09 ObjectRef RC migrate product flip first-slice **已交**（ADR-899）→ W556 B-09 storage-cleanup product flip first-slice **已交**（ADR-900）→ W557 B-09 unified MirRuntime heap enable product flip first-slice **已交**（ADR-901）→ W558+ — B-09 `[x]` / GC GA；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
