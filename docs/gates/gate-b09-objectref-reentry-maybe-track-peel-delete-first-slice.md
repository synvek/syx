# B-09 ObjectRef re-entry maybe_track residual peel true-delete first-slice (W550 / ADR-894)

## Status

**B-09 ObjectRef re-entry maybe_track residual peel true-delete first-slice met**: after W549 re-entry peel readiness, ObjectRef `maybe_track_objectref_*` / `maybe_track_stack_top_objectref` deleted in VM + mir-run (CallHost/ObjectGet/Dup/LoadLocal/…); MutRefCell `maybe_track_*` **kept**. DictBuild heap-primary birth **met** (W547). Re-entry peel readiness **met** (W549). `objectref_rc_migrate_enabled` **still false**. This window does **not** flip migrate / `storage_cleanup_enabled`, collapse MutRefCell dual-track, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**（退出加权 ≈94–98%）；≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `four-hundredth` / `A-W550`.

## Index

| Prior | Status |
|-------|--------|
| W476–W549 narrative → re-entry peel readiness | **met** |
| This window re-entry maybe_track residual peel true-delete | **met** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-ii-r0. Re-entry maybe_track peel readiness | Blockers documented | **met** (W549) |
| 4c-ii-r. True re-entry maybe_track peel | Delete CallHost/ObjectGet/… `maybe_track_objectref_*` | **met** (this window) |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Peel honesty table (hard)

| Item | Present? | Meaning |
|------|----------|---------|
| Re-entry maybe_track peel readiness thin signed | **met** | W549 |
| Re-entry ObjectRef `maybe_track_objectref_*` | **deleted** | this window |
| True re-entry maybe_track peel | **met** | this window |
| MutRefCell `maybe_track_*` | **kept** | out of ObjectRef peel |
| DictBuild ObjectRef heap-primary birth | **met** | W547 |
| `objectref_rc_migrate_enabled` | **no** | still false |
| B-09 axis exit (`[x]`) / GC GA | **no** | peel ≠ exit |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| Re-entry maybe_track peel true-delete signed | **yes** | This window |
| Full migrate enabled | **no** | Still open |
| Storage cleanup enabled | **no** | Still open |
| Unified heap enabled | **no** | Frozen |
| B-09 `[x]` / GC GA | **no** | peel ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectrefReentryMaybeTrackPeelDeleteFirstSliceInventory` / `gate_b09_objectref_reentry_maybe_track_peel_delete_first_slice_inventory` |
| Flags | `b09_objectref_reentry_maybe_track_peel_delete_first_slice_signed=true`; `b09_objectref_reentry_maybe_track_peel_readiness_thin_floor_ok=true`; `dual_track_collapse_met=true`; `dictbuild_objectref_heap_primary_birth_met=true`; `objectref_reentry_maybe_track_deleted=true`; `reentry_maybe_track_peel_met=true`; `mutrefcell_maybe_track_kept=true`; `mutrefcell_dual_track_out_of_scope=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / HostFn+1 / MIR distance change / MutRefCell peel / claiming peel as migrate |
| Honesty | re-entry maybe_track peel met **≠** migrate flip / **≠** MutRefCell collapse / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA；W518/W531/W535–W549 historical inventory snapshots **not rewritten** |

## Evidence

- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) — ObjectRef `maybe_track_*` deleted; MutRefCell kept
- [`crates/syx-mir-run/src/runtime.rs`](../../crates/syx-mir-run/src/runtime.rs) — ObjectRef `maybe_track_*` deleted; MutRefCell kept
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectrefReentryMaybeTrackPeelDeleteFirstSliceInventory`
- Meta: `b09_objectref_reentry_maybe_track_peel_delete_first_slice.rs`
- Floor: [`gate-b09-objectref-reentry-maybe-track-peel-readiness-thin.md`](gate-b09-objectref-reentry-maybe-track-peel-readiness-thin.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)
- ADR-894 / fingerprint `A-W550` / `qs1-platform-gate-v0:driver-gate-exec-four-hundredth`

## Frozen

- Flipping `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Deleting MutRefCell `maybe_track_*` this window
- Rewriting W518/W531/W535–W549 Contract table field claims as if current
- Claiming peel as migrate flip
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w549_*.plan.md`

## Next

W549 B-09 ObjectRef re-entry maybe_track residual peel readiness thin **已交**（ADR-893）→ W550 B-09 ObjectRef re-entry maybe_track residual peel true-delete first-slice **已交**（ADR-894）→ W551 B-09 ObjectRef RC migrate-readiness post-reentry-peel refresh thin **已交**（ADR-895）→ W552 B-09 MutRefCell maybe_track residual peel readiness thin **已交**（ADR-896）→ W553 B-09 MutRefCell maybe_track residual peel true-delete first-slice **已交**（ADR-897）→ W554 B-09 ObjectRef RC migrate-readiness post-mutrefcell-peel refresh thin **已交**（ADR-898）→ W555 B-09 ObjectRef RC migrate product flip first-slice **已交**（ADR-899）→ W556 B-09 storage-cleanup product flip first-slice **已交**（ADR-900）→ W557 B-09 unified MirRuntime heap enable product flip first-slice **已交**（ADR-901）→ W558+ — B-09 `[x]` / GC GA；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
