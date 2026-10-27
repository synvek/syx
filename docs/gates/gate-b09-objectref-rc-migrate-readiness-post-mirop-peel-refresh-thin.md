# B-09 ObjectRef RC migrate-readiness post-mirop-peel refresh thin (W545 / ADR-889)

## Status

**B-09 ObjectRef RC migrate-readiness post-mirop-peel refresh thin met** (policy+inventory): after W544 MirOp/VM residual peel true-delete, the W542 migrate-blocker table is **stale** (still claimed MirOp/VM ObjectRef residual `still_present`). This window **re-signs** migrate-flip blockers: handle-table + mir-run opcode peel **met** (W541); MirOp/VM residual peel **met** (W544 — `MirOp::ObjectRef*` and VM `ObjectRefRetain/Release` **deleted**); DictBuild dual-track birth **kept**; remaining open are dual-track↔heap coexistence and `objectref_rc_migrate_enabled` **still false**. This window does **not** flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**（退出加权 ≈89–92%）；≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-ninety-fifth` / `A-W545`.

## Index

| Prior | Status |
|-------|--------|
| W476–W544 narrative → MirOp/VM residual peel true-delete | **met** |
| W542 post-peel migrate-readiness refresh thin (historical snapshot) | **met** (table **stale** after W543–W544; refresh lives here) |
| This window post-mirop-peel migrate-readiness refresh thin | **met** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … MirOp/VM peel true-delete | Tracks → peel → MirOp/VM peel | **met** (W494–W544) |
| 4c-ii-u4. Post-peel migrate-readiness refresh (W542) | Historical blockers signed; migrate still off | **met** (historical; table stale after W544) |
| 4c-ii-u5. Post-mirop-peel migrate-readiness refresh | Blockers re-signed after W544; migrate still off | **met** (this window) |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Migrate blocker table (post-mirop-peel refresh; hard)

| Blocker | Present? | Meaning |
|---------|----------|---------|
| Handle-table + mir-run opcode peel | **met** | W541 |
| MirOp `ObjectRefWrap/Retain/Release` | **deleted** | W544 (`mir_op_objectref_wrap_retain_release_deleted`) |
| VM `ObjectRefRetain/Release` | **deleted** | W544 (`vm_instruction_objectref_retain_release_deleted`) |
| MirOp/VM residual peel | **met** | W544 (`mirop_vm_residual_peel_met`) |
| DictBuild ObjectRef dual-track birth | **kept** | W544 (`dictbuild_objectref_dual_track_birth_kept`) |
| Dual-track ManagedHeap/`heap_slots` coexistence | **noted** | `dual_track_and_heap_coexistence_noted` |
| `objectref_rc_migrate_enabled` | **no** | still false |
| B-09 axis exit (`[x]`) / GC GA | **no** | refresh ≠ exit |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| Post-mirop-peel migrate-readiness refresh signed | **yes** | This window |
| Full migrate enabled | **no** | Still open |
| Storage cleanup enabled | **no** | Still open |
| Unified heap enabled | **no** | Frozen |
| B-09 `[x]` / GC GA | **no** | refresh ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectrefRcMigrateReadinessPostMiropPeelRefreshThinInventory` / `gate_b09_objectref_rc_migrate_readiness_post_mirop_peel_refresh_thin_inventory` |
| Flags | `b09_objectref_rc_migrate_readiness_post_mirop_peel_refresh_thin_signed=true`; `b09_objectref_mirop_vm_residual_peel_delete_first_slice_floor_ok=true`; `handle_table_mir_run_opcode_peel_met=true`; `mirop_vm_residual_peel_met=true`; `mir_op_objectref_wrap_retain_release_deleted=true`; `vm_instruction_objectref_retain_release_deleted=true`; `dictbuild_objectref_dual_track_birth_kept=true`; `dual_track_and_heap_coexistence_noted=true`; `migrate_flip_blockers_refreshed_after_mirop_peel=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / HostFn+1 / MIR distance change / claiming refresh as full migrate / rewriting W542 Contract `still_present` as if current |
| Honesty | post-mirop-peel migrate-readiness **refresh** **≠** migrate flip / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA；W518/W531/W535–W544 historical inventory snapshots **not rewritten** |

## Evidence

- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectrefRcMigrateReadinessPostMiropPeelRefreshThinInventory`
- Meta: ~~`b09_objectref_rc_migrate_readiness_post_mirop_peel_refresh_thin.rs`~~ retired（F-10 W603）；proof = this SSOT + `GateB09ObjectrefRcMigrateReadinessPostMiropPeelRefreshThinInventory`
- Floor: [`gate-b09-objectref-mirop-vm-residual-peel-delete-first-slice.md`](gate-b09-objectref-mirop-vm-residual-peel-delete-first-slice.md)
- Historical (do not rewrite Contract fields): [`gate-b09-objectref-rc-migrate-readiness-post-peel-refresh-thin.md`](gate-b09-objectref-rc-migrate-readiness-post-peel-refresh-thin.md), [`gate-b09-objectref-mirop-vm-residual-peel-readiness-thin.md`](gate-b09-objectref-mirop-vm-residual-peel-readiness-thin.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Flipping `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Rewriting W518/W531/W535–W544 Contract table field claims as if current; rewriting W542 `still_present` Contract fields as deleted
- Claiming refresh as migrate flip
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w544_*.plan.md`

## Next

W544 B-09 ObjectRef MirOp/VM residual peel true-delete first-slice **已交**（ADR-888）→ W545 B-09 ObjectRef RC migrate-readiness post-mirop-peel refresh thin **已交**（ADR-889）→ W546 B-09 ObjectRef dual-track collapse readiness thin **已交**（ADR-890）→ W547 B-09 ObjectRef dual-track collapse true-delete first-slice **已交**（ADR-891）→ W548 B-09 ObjectRef RC migrate-readiness post-collapse refresh thin **已交**（ADR-892）→ W549 B-09 ObjectRef re-entry maybe_track residual peel readiness thin **已交**（ADR-893）→ W550 B-09 ObjectRef re-entry maybe_track residual peel true-delete first-slice **已交**（ADR-894）→ W551 B-09 ObjectRef RC migrate-readiness post-reentry-peel refresh thin **已交**（ADR-895）→ W552 B-09 MutRefCell maybe_track residual peel readiness thin **已交**（ADR-896）→ W553 B-09 MutRefCell maybe_track residual peel true-delete first-slice **已交**（ADR-897）→ W554 B-09 ObjectRef RC migrate-readiness post-mutrefcell-peel refresh thin **已交**（ADR-898）→ W555 B-09 ObjectRef RC migrate product flip first-slice **已交**（ADR-899）→ W556 B-09 storage-cleanup product flip first-slice **已交**（ADR-900）→ W557 B-09 unified MirRuntime heap enable product flip first-slice **已交**（ADR-901）→ W558+ — B-09 `[x]` / GC GA；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
