# B-09 ObjectRef RC migrate-readiness thin (W518 / ADR-862)

## Status

**B-09 ObjectRef RC migrate-readiness thin met** (policy+inventory): after W517 SSA LoadLocal, migrate-flip **blockers / preconditions** are signed — Value/VM/mir-run ObjectRef RC still live, RT ABI retain/release/wrap still present, LLVM/WASM emit RC path retired (W511/W514), dual-track birth surfaces inventoried (LoadGlobal/ObjectSet/SSA LoadLocal…), nested field residual noted, and migrate remains **off**. This window does **not** flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, delete Value/VM/mir-run RC or RT ABI, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-sixty-eighth` / `A-W518`.

## Index

| Prior | Status |
|-------|--------|
| W476–W517 narrative → SSA LoadLocal | **met** |
| This window migrate-readiness thin | **met** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-t | Tracks → SSA LoadLocal | **met** (W494–W517) |
| 4c-ii-u. Migrate-readiness | Blockers documented; migrate still off | **met** (this window) |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Migrate blocker table (hard)

| Blocker | Present? | Meaning |
|---------|----------|---------|
| Value / VM / mir-run ObjectRef RC | **yes** | `value_vm_mir_run_objectref_rc_still_live` |
| RT ABI `syx_rt_object_ref_retain/release/wrap` | **yes** | `rt_abi_object_ref_retain_release_wrap_still_present` |
| LLVM/WASM ObjectRef RC emit path | **retired** | W511/W514 floors (`llvm_wasm_objectref_rc_emit_path_retired`) |
| Dual-track birth surfaces inventoried | **yes** | LoadGlobal / ObjectSet / SSA LoadLocal… |
| Nested field without re-entry track | **noted** | residual birth; ≠ this window track |
| `objectref_rc_migrate_enabled` | **no** | still false |
| B-09 axis exit (`[x]`) / GC GA | **no** | readiness ≠ exit |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| Migrate-readiness signed | **yes** | This window |
| Full migrate enabled | **no** | Still open |
| Storage cleanup enabled | **no** | Still open |
| Unified heap enabled | **no** | Frozen |
| B-09 `[x]` / GC GA | **no** | readiness ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectRefRcMigrateReadinessThinInventory` / `gate_b09_objectref_rc_migrate_readiness_thin_inventory` |
| Flags | `b09_objectref_rc_migrate_readiness_thin_signed=true`; `b09_objectref_ssa_loadlocal_heap_slots_first_slice_floor_ok=true`; `value_vm_mir_run_objectref_rc_still_live=true`; `rt_abi_object_ref_retain_release_wrap_still_present=true`; `llvm_wasm_objectref_rc_emit_path_retired=true`; `dual_track_birth_surfaces_inventoried=true`; `nested_field_residual_noted=true`; `migrate_blockers_documented=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / Value RC deleted / RT ABI delete / scan-all auto / HostFn+1 / MIR distance change |
| Honesty | migrate-readiness **≠** migrate flip / **≠** RC retire / **≠** cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectRefRcMigrateReadinessThinInventory`
- Meta: ~~`b09_objectref_rc_migrate_readiness_thin.rs`~~ retired（F-10 W598）；proof = this SSOT + `GateB09ObjectRefRcMigrateReadinessThinInventory`
- Floor: [`gate-b09-objectref-ssa-loadlocal-heap-slots-first-slice.md`](gate-b09-objectref-ssa-loadlocal-heap-slots-first-slice.md)
- Prior: [`gate-b09-objectref-rc-migrate-first-slice.md`](gate-b09-objectref-rc-migrate-first-slice.md), [`gate-b09-enable-readiness-thin.md`](gate-b09-enable-readiness-thin.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Deleting Value / VM / mir-run RC / deleting RT ABI / nested field dual-track this window
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w517_*.plan.md`

## Next

W517 B-09 SSA LoadLocal ManagedHeap + heap_slots first-slice **已交**（ADR-861）→ W518 B-09 ObjectRef RC migrate-readiness thin **已交**（ADR-862）→ W519 B-09 RT ABI ObjectRefWrap retire-readiness thin **已交**（ADR-863）→ W520 B-09 RT ABI ObjectRefRetain/Release retire-readiness thin **已交**（ADR-864）→ W521 B-09 Value/VM/mir-run ObjectRef RC retire-readiness thin **已交**（ADR-865）→ W522 B-09 ObjectRefRetain opcode RC true-retire first-slice **已交**（ADR-866）→ W523 B-09 Dup+LoadLocal ObjectRef RC true-retire first-slice **已交**（ADR-867）→ W524+ — 优先 Release/StoreLocal/deinit **迁 ManagedHeap/GC**（分窗）→ Value API 删 → wrap/retain/release ABI 真删（慎）→ 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）

**Note (W531 / ADR-875):** migrate-readiness **refresh** **已交** — post-W529/W530 blocker refresh lives in [`gate-b09-objectref-rc-migrate-readiness-refresh-thin.md`](gate-b09-objectref-rc-migrate-readiness-refresh-thin.md). This document remains the **historical** W518 inventory snapshot; **do not** rewrite Contract-table field claims here (Value RC / wrap·retain·release ABI “still present” was true at W518 and is intentionally frozen as history).
