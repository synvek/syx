# B-09 ObjectRef residual re-entry（ObjectGet）+ birth surface ManagedHeap + heap_slots first-slice (W512 / ADR-856)

## Status

**B-09 ObjectRef residual re-entry (ObjectGet) + birth surface inventory ManagedHeap + heap_slots first-slice met**: after W511 LLVM·WASM Retain/Release emit-path retire, VM `Instruction::ObjectGet` and mir-run `OBJECT_GET` success paths dual-register stack-top `ObjectRef` and `MutRefCell` into ManagedHeap and push `heap_slots` (identity-dedup). Residual birth surfaces are inventoried (DictBuild / wrap / CallHost lazy+weak floors already covered). This window does **not** flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, retire LLVM `object_ref_wrap` emit, delete Value/VM/mir-run RC, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-sixty-second` / `A-W512`.

## Index

| Prior | Status |
|-------|--------|
| W476–W511 narrative → emit-path retire | **met** |
| This window ObjectGet residual re-entry + birth inventory | **met** |
| ArrayGet residual | **met** (W513 / ADR-857) |
| Nested field / ObjectSet / LoadGlobal | ObjectSet/LoadGlobal **met** (W515/W516); nested write-path **met** (W532 / ADR-876); deep field-graph walk **still unmet** |
| LLVM ObjectRefWrap emit residual | **met** (W514 / ADR-858) |
| SSA LoadLocal dual-track | **met** (W517 / ADR-861) |
| Migrate-readiness thin | **met** (W518 / ADR-862) |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Cleanup ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-o | Dead API → tracks → emit-path retire | **met** (W494–W511) |
| 4c-ii-p. ObjectGet residual re-entry | ObjectGet → ManagedHeap + `heap_slots` | **met** (this window) |
| 4c-ii. Storage-layer cleanup | Full ObjectRef/MutRefCell→ManagedHeap; flip cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Birth / re-entry surface inventory (hard)

| Surface | Status | Meaning |
|---------|--------|---------|
| DictBuild ObjectRef / MutRefCell | **covered** | W497–W503 |
| `object_ref_wrap` (mir-run) | **covered** | W499 align |
| CallHost return (lazy proxy / weak_load) | **covered** | W500/W504 dual track |
| ObjectRefRetain / Dup / LoadLocal | **covered** | W510 catch-all |
| **ObjectGet re-entry** | **wired** (this window) | VM + mir-run dual helpers |
| ArrayGet / other element get | **met** (W513 / ADR-857) | dual-track |
| Nested field without re-entry track | **write-path met** (W532 / ADR-876) | deep field-graph GC walk **still unmet** |
| LLVM `@syx_rt_object_ref_wrap` emit | **met** (W514 / ADR-858) | dual-track RT kept |
| Value / VM / mir-run RC | **still live** | ≠ full migrate |

> **Note (W532):** nested write-path dual-track **met** ≠ deep field-graph GC walk ≠ migrate flip ≠ enable ≠ B-09 `[x]`. Historical Contract Flags below remain W512 snapshot — do not rewrite as current.

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectRefResidualBirthHeapSlotsFirstSliceInventory` / `gate_b09_objectref_residual_birth_heap_slots_first_slice_inventory` |
| Flags | `b09_objectref_residual_birth_heap_slots_first_slice_signed=true`; `b09_llvm_wasm_objectref_rc_emit_retire_first_slice_floor_ok=true`; `objectref_residual_birth_surface_inventoried=true`; `vm_objectget_objectref_managed_heap_registered=true`; `mir_run_objectget_objectref_managed_heap_registered=true`; `vm_mir_run_objectget_heap_slots_tracked=true`; `host_lazy_weak_objectref_callhost_floor_ok=true`; `objectref_rc_storage_still_present=true`; `llvm_wasm_objectref_rc_still_live=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / Value RC deleted / LLVM wrap emit retired / scan-all auto / RT ABI delete / HostFn+1 / MIR distance change |
| Honesty | ObjectGet residual track **≠** full migrate / **≠** storage cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) — ObjectGet + dual helpers / ADR-856
- [`crates/syx-mir-run/src/runtime.rs`](../../crates/syx-mir-run/src/runtime.rs) — `object_get` + dual helpers / ADR-856
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectRefResidualBirthHeapSlotsFirstSliceInventory`
- Meta: `b09_objectref_residual_birth_heap_slots_first_slice.rs`
- Floor: [`gate-b09-llvm-wasm-objectref-rc-emit-retire-first-slice.md`](gate-b09-llvm-wasm-objectref-rc-emit-retire-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Deleting Value / VM / mir-run RC / deleting RT ABI / LLVM ObjectRefWrap emit retire this window
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w511_*.plan.md`

## Next

W511 B-09 LLVM·WASM ObjectRefRetain/Release emit-path 退役 first-slice **已交**（ADR-855）→ W512 B-09 ObjectRef ObjectGet residual re-entry + birth inventory ManagedHeap + heap_slots first-slice **已交**（ADR-856）→ W513 B-09 ObjectRef ArrayGet（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-857）→ W515 B-09 ObjectSet MutRefCell 写回（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-859）→ W516 B-09 LoadGlobal（VM+mir-run/SSA）ManagedHeap + heap_slots first-slice **已交**（ADR-860）→ W517 B-09 SSA LoadLocal ManagedHeap + heap_slots first-slice **已交**（ADR-861）→ W518 B-09 ObjectRef RC migrate-readiness thin **已交**（ADR-862）→ W519+ — 优先 Value/VM/mir-run RC 与 RT ABI retain/release/wrap **诚实退役** → 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
