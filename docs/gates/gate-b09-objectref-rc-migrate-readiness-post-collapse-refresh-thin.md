# B-09 ObjectRef RC migrate-readiness post-collapse refresh thin (W548 / ADR-892)

## Status

**B-09 ObjectRef RC migrate-readiness post-collapse refresh thin met** (policy+inventory): after W547 DictBuild dual-track collapse true-delete, the W545 migrate-blocker table is **stale** (still claimed DictBuild dual-track birth `kept`). This window **re-signs** migrate-flip blockers: DictBuild dual-track birth **deleted**; heap-primary birth **met**; dual-track collapse **met** (W547); MirOp/VM residual peel **met** (W544); re-entry `maybe_track_*` residual **noted** (still present); MutRefCell dual-track **out of flip scope**; `objectref_rc_migrate_enabled` **still false**. This window does **not** flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, peel re-entry residual, collapse MutRefCell dual-track, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**（退出加权 ≈93–95%）；≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-ninety-eighth` / `A-W548`.

## Index

| Prior | Status |
|-------|--------|
| W476–W547 narrative → dual-track collapse true-delete | **met** |
| W545 post-mirop-peel migrate-readiness refresh thin (historical snapshot) | **met** (table **stale** after W547; refresh lives here) |
| This window post-collapse migrate-readiness refresh thin | **met** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … MirOp/VM peel + post-mirop refresh | Tracks → peel → post-mirop migrate-readiness refresh | **met** (W494–W545) |
| 4c-ii-d0. Dual-track collapse readiness | Blockers documented | **met** (W546) |
| 4c-ii-d. True dual-track collapse | Productize collapse of Arc + `maybe_track_*` DictBuild birth | **met** (W547) |
| 4c-ii-u6. Post-collapse migrate-readiness refresh | Blockers re-signed after W547; migrate still off | **met** (this window) |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Migrate blocker table (post-collapse refresh; hard)

| Blocker | Present? | Meaning |
|---------|----------|---------|
| DictBuild ObjectRef dual-track birth | **deleted** | W547 (`dictbuild_objectref_dual_track_birth_deleted`) |
| DictBuild ObjectRef heap-primary birth | **met** | W547 (`dictbuild_objectref_heap_primary_birth_met`) |
| Dual-track collapse | **met** | W547 (`dual_track_collapse_met`) |
| MirOp/VM residual peel | **met** | W544 (`mirop_vm_residual_peel_met`) |
| Re-entry `maybe_track_*` residual | **noted** | CallHost/ObjectGet/… (`objectref_reentry_maybe_track_residual_noted`) |
| MutRefCell dual-track | **out of scope** | (`mutrefcell_dual_track_out_of_scope`) |
| `objectref_rc_migrate_enabled` | **no** | still false |
| B-09 axis exit (`[x]`) / GC GA | **no** | refresh ≠ exit |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| Post-collapse migrate-readiness refresh signed | **yes** | This window |
| Full migrate enabled | **no** | Still open |
| Storage cleanup enabled | **no** | Still open |
| Unified heap enabled | **no** | Frozen |
| B-09 `[x]` / GC GA | **no** | refresh ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectrefRcMigrateReadinessPostCollapseRefreshThinInventory` / `gate_b09_objectref_rc_migrate_readiness_post_collapse_refresh_thin_inventory` |
| Flags | `b09_objectref_rc_migrate_readiness_post_collapse_refresh_thin_signed=true`; `b09_objectref_dual_track_collapse_delete_first_slice_floor_ok=true`; `dictbuild_objectref_dual_track_birth_deleted=true`; `dictbuild_objectref_heap_primary_birth_met=true`; `dual_track_collapse_met=true`; `mirop_vm_residual_peel_met=true`; `migrate_flip_blockers_refreshed_after_collapse=true`; `objectref_reentry_maybe_track_residual_noted=true`; `mutrefcell_dual_track_out_of_scope=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / HostFn+1 / MIR distance change / claiming refresh as full migrate / peeling re-entry residual / MutRefCell collapse / rewriting W545–W547 Contract fields as if current |
| Honesty | post-collapse migrate-readiness **refresh** **≠** migrate flip / **≠** re-entry peel / **≠** MutRefCell collapse / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA；W518/W531/W535–W547 historical inventory snapshots **not rewritten** |

## Evidence

- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectrefRcMigrateReadinessPostCollapseRefreshThinInventory`
- Meta: ~~`b09_objectref_rc_migrate_readiness_post_collapse_refresh_thin.rs`~~ retired（F-10 W607）；proof = this SSOT + `GateB09ObjectrefRcMigrateReadinessPostCollapseRefreshThinInventory`
- Floor: [`gate-b09-objectref-dual-track-collapse-delete-first-slice.md`](gate-b09-objectref-dual-track-collapse-delete-first-slice.md)
- Historical (do not rewrite Contract fields): [`gate-b09-objectref-rc-migrate-readiness-post-mirop-peel-refresh-thin.md`](gate-b09-objectref-rc-migrate-readiness-post-mirop-peel-refresh-thin.md), [`gate-b09-objectref-dual-track-collapse-readiness-thin.md`](gate-b09-objectref-dual-track-collapse-readiness-thin.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)
- ADR-892 / fingerprint `A-W548` / `qs1-platform-gate-v0:driver-gate-exec-three-hundred-and-ninety-eighth`

## Frozen

- Flipping `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Peeling re-entry `maybe_track_*` residual this window
- MutRefCell dual-track collapse this window
- Rewriting W518/W531/W535–W547 Contract table field claims as if current; rewriting W545 `kept` Contract fields as deleted
- Claiming refresh as migrate flip
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w547_*.plan.md`

## Next

W547 B-09 ObjectRef dual-track collapse true-delete first-slice **已交**（ADR-891）→ W548 B-09 ObjectRef RC migrate-readiness post-collapse refresh thin **已交**（ADR-892）→ W549 B-09 ObjectRef re-entry maybe_track residual peel readiness thin **已交**（ADR-893）→ W550 B-09 ObjectRef re-entry maybe_track residual peel true-delete first-slice **已交**（ADR-894）→ W551 B-09 ObjectRef RC migrate-readiness post-reentry-peel refresh thin **已交**（ADR-895）→ W552 B-09 MutRefCell maybe_track residual peel readiness thin **已交**（ADR-896）→ W553 B-09 MutRefCell maybe_track residual peel true-delete first-slice **已交**（ADR-897）→ W554 B-09 ObjectRef RC migrate-readiness post-mutrefcell-peel refresh thin **已交**（ADR-898）→ W555 B-09 ObjectRef RC migrate product flip first-slice **已交**（ADR-899）→ W556 B-09 storage-cleanup product flip first-slice **已交**（ADR-900）→ W557 B-09 unified MirRuntime heap enable product flip first-slice **已交**（ADR-901）→ W558+ — B-09 `[x]` / GC GA；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
