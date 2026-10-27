# B-09 MutRefCell maybe_track residual peel true-delete first-slice (W553 / ADR-897)

## Status

**B-09 MutRefCell maybe_track residual peel true-delete first-slice met**: after W552 MutRefCell peel readiness, MutRefCell `maybe_track_mutrefcell_*` / `maybe_track_stack_top_mutrefcell` deleted in VM + mir-run (CallHost/ObjectGet/Dup/LoadLocal/DictBuild/…). ObjectRef re-entry peel **met** (W550). Post-reentry-peel migrate-readiness refresh **met** (W551). MutRefCell peel readiness **met** (W552). DictBuild ObjectRef heap-primary **met** (W547). `objectref_rc_migrate_enabled` **still false**. This window does **not** flip migrate / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**（退出加权 ≈96–98%）；≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `four-hundred-and-third` / `A-W553`.

## Index

| Prior | Status |
|-------|--------|
| W476–W552 narrative → MutRefCell peel readiness | **met** |
| This window MutRefCell maybe_track residual peel true-delete | **met** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-ii-m0. MutRefCell maybe_track peel readiness | Blockers documented | **met** (W552) |
| 4c-ii-m. True MutRefCell maybe_track peel | Delete `maybe_track_mutrefcell_*` | **met** (this window) |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Peel honesty table (hard)

| Item | Present? | Meaning |
|------|----------|---------|
| MutRefCell maybe_track peel readiness thin signed | **met** | W552 |
| MutRefCell `maybe_track_mutrefcell_*` | **deleted** | this window |
| True MutRefCell maybe_track peel | **met** | this window |
| ObjectRef re-entry maybe_track peel | **met** | W550 |
| DictBuild ObjectRef heap-primary birth | **met** | W547 |
| `objectref_rc_migrate_enabled` | **no** | still false |
| B-09 axis exit (`[x]`) / GC GA | **no** | peel ≠ exit |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| MutRefCell maybe_track peel true-delete signed | **yes** | This window |
| Full migrate enabled | **no** | Still open |
| Storage cleanup enabled | **no** | Still open |
| Unified heap enabled | **no** | Frozen |
| B-09 `[x]` / GC GA | **no** | peel ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09MutrefcellMaybeTrackPeelDeleteFirstSliceInventory` / `gate_b09_mutrefcell_maybe_track_peel_delete_first_slice_inventory` |
| Flags | `b09_mutrefcell_maybe_track_peel_delete_first_slice_signed=true`; `b09_mutrefcell_maybe_track_peel_readiness_thin_floor_ok=true`; `objectref_reentry_maybe_track_deleted=true`; `reentry_maybe_track_peel_met=true`; `dual_track_collapse_met=true`; `dictbuild_objectref_heap_primary_birth_met=true`; `migrate_flip_blockers_refreshed_after_reentry_peel=true`; `mutrefcell_maybe_track_deleted=true`; `mutrefcell_peel_met=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / HostFn+1 / MIR distance change / claiming peel as migrate |
| Honesty | MutRefCell maybe_track peel met **≠** migrate flip / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA；W518/W531/W535–W552 historical inventory snapshots **not rewritten** |

## Evidence

- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) — MutRefCell `maybe_track_*` deleted; ObjectRef untrack retained
- [`crates/syx-mir-run/src/runtime.rs`](../../crates/syx-mir-run/src/runtime.rs) — MutRefCell `maybe_track_*` deleted
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09MutrefcellMaybeTrackPeelDeleteFirstSliceInventory`
- Meta: `b09_mutrefcell_maybe_track_peel_delete_first_slice.rs`
- Floor: [`gate-b09-mutrefcell-maybe-track-peel-readiness-thin.md`](gate-b09-mutrefcell-maybe-track-peel-readiness-thin.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)
- ADR-897 / fingerprint `A-W553` / `qs1-platform-gate-v0:driver-gate-exec-four-hundred-and-third`

## Frozen

- Flipping `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Rewriting W518/W531/W535–W552 Contract table field claims as if current
- Claiming peel as migrate flip
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w552_*.plan.md`

## Next

W552 B-09 MutRefCell maybe_track residual peel readiness thin **已交**（ADR-896）→ W553 B-09 MutRefCell maybe_track residual peel true-delete first-slice **已交**（ADR-897）→ W554 B-09 ObjectRef RC migrate-readiness post-mutrefcell-peel refresh thin **已交**（ADR-898）→ W555 B-09 ObjectRef RC migrate product flip first-slice **已交**（ADR-899）→ W556 B-09 storage-cleanup product flip first-slice **已交**（ADR-900）→ W557 B-09 unified MirRuntime heap enable product flip first-slice **已交**（ADR-901）→ W558+ — B-09 `[x]` / GC GA；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
