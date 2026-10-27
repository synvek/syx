# B-09 RT ABI ObjectRef handle retain/release/drop retire-readiness thin (W533 / ADR-877)

## Status

**B-09 RT ABI ObjectRef handle retain/release/drop retire-readiness thin met** (policy+inventory): after W532 nested field write-path dual-track, handle RT ABI retire **blockers / preconditions** are signed — opcode wrap/retain/release ABI deleted (W530), Value RC API deleted (W529), nested write-path met (W532), while handle ABI `syx_rt_object_retain` / `release` / `drop` **still present**, RC storage still present, deep field-graph GC walk still unmet, and migrate remains **off**. This window does **not** delete handle ABI, flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**（退出加权 ≈73–74%）；≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-eighty-third` / `A-W533`.

## Index

| Prior | Status |
|-------|--------|
| W476–W532 narrative → nested write-path | **met** |
| RT opcode wrap/retain/release ABI delete (W530) | **met** |
| This window handle RT ABI retire-readiness thin | **met** |
| Handle RT ABI true delete (`syx_rt_object_retain/release/drop`) | **met** (W534 / ADR-878) |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-v | Tracks → RC retire → nested write-path | **met** (W494–W532) |
| 4c-ii-w. Handle RT ABI retire-readiness | Blockers documented; handle ABI still present | **met** (this window) |
| 4c-ii-x. Handle RT ABI true delete | Remove `syx_rt_object_retain/release/drop` | **met** (W534 / ADR-878) |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Handle retire blocker table (hard)

| Blocker | Present? | Meaning |
|---------|----------|---------|
| RT opcode wrap/retain/release ABI | **deleted** | W530 floor |
| Value `object_ref_retain/release` API | **deleted** | W529 floor |
| Nested write-path dual-track | **met** | W532 floor |
| RT ABI `syx_rt_object_retain/release/drop` | **yes** | `rt_abi_object_handle_retain_release_drop_still_present` |
| `Value::ObjectRef { rc }` storage | **yes** | `objectref_rc_storage_still_present` |
| Deep field-graph GC walk | **no** | `deep_field_graph_gc_walk_still_unmet` |
| `objectref_rc_migrate_enabled` | **no** | still false |
| B-09 axis exit (`[x]`) / GC GA | **no** | readiness ≠ exit |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| Handle retire-readiness signed | **yes** | This window |
| Handle RT ABI deleted | **yes** | W534 / ADR-878 |
| Full migrate enabled | **no** | Still open |
| Storage cleanup enabled | **no** | Still open |
| Unified heap enabled | **no** | Frozen |
| B-09 `[x]` / GC GA | **no** | readiness ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09RtAbiObjectrefHandleRetireReadinessThinInventory` / `gate_b09_rt_abi_objectref_handle_retire_readiness_thin_inventory` |
| Flags | `b09_rt_abi_objectref_handle_retire_readiness_thin_signed=true`; `b09_objectref_nested_field_dual_track_first_slice_floor_ok=true`; `rt_abi_object_handle_retain_release_drop_still_present=true`; `nested_field_write_path_dual_track_met=true`; `objectref_rc_storage_still_present=true`; `dual_track_and_rc_storage_coexistence_noted=true`; `deep_field_graph_gc_walk_still_unmet=true`; `handle_retire_blockers_documented=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | deleting handle ABI / flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / scan-all auto / HostFn+1 / MIR distance change / claiming readiness as handle delete |
| Honesty | handle retire-readiness **≠** handle ABI delete / **≠** migrate flip / **≠** cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09RtAbiObjectrefHandleRetireReadinessThinInventory`
- [`crates/syx-rt/src/abi.rs`](../../crates/syx-rt/src/abi.rs) — at W533 signing `syx_rt_object_retain` / `release` / `drop` still present；W534 / ADR-878 later **deleted**
- Meta: ~~`b09_rt_abi_objectref_handle_retire_readiness_thin.rs`~~ retired（F-10 W600）；proof = this SSOT + `GateB09RtAbiObjectrefHandleRetireReadinessThinInventory`
- Floor: [`gate-b09-objectref-nested-field-dual-track-first-slice.md`](gate-b09-objectref-nested-field-dual-track-first-slice.md)
- Shape: [`gate-b09-rt-abi-objectref-wrap-retire-readiness-thin.md`](gate-b09-rt-abi-objectref-wrap-retire-readiness-thin.md), [`gate-b09-rt-abi-objectref-retain-release-retire-readiness-thin.md`](gate-b09-rt-abi-objectref-retain-release-retire-readiness-thin.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Peeling RC storage; deep field-graph GC walk
- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w532_*.plan.md`

## Next

W532 B-09 ObjectRef nested field dual-track first-slice **已交**（ADR-876）→ W534 B-09 RT ABI handle retain·release·drop true-delete first-slice **已交**（ADR-878）→ W535 B-09 ObjectRef RC migrate-readiness post-handle-delete refresh thin **已交**（ADR-879）→ W536 B-09 ObjectRef strip-RC readiness thin **已交**（ADR-880）→ W537 B-09 ObjectRef strip-RC true-delete first-slice **已交**（ADR-881）→ W538 B-09 ObjectRef deep field-graph GC walk readiness thin **已交**（ADR-882）→ W539 B-09 ObjectRef deep field-graph GC walk true first-slice **已交**（ADR-883）→ W540 B-09 ObjectRef peel 表·opcodes readiness thin **已交**（ADR-884）→ W541 B-09 ObjectRef peel 表·opcodes true-delete first-slice **已交**（ADR-885）→ W542+ — 真产品迁 / 翻 cleanup / 真 enable；或另钉 MirOp/VM residual peel；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
