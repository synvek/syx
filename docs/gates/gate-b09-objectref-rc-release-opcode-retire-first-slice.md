# B-09 ObjectRefRelease opcode RC true-retire first-slice (W524 / ADR-868)

## Status

**B-09 ObjectRefRelease opcode RC true-retire first-slice met**: after W523 Dup+LoadLocal ObjectRef RC true-retire, VM `Instruction::ObjectRefRelease` and mir-run `OBJECT_REF_RELEASE` **no longer** call `Value::object_ref_release` / opcode-path Type_deinit (RC bump retired; **still pop**). StoreLocal / `run_class_deinits` release, `Value::object_ref_*` APIs, and RT ABI retain/release/wrap **still live**. This window does **not** strip StoreLocal/deinit release, delete Value RC APIs, delete RT ABI, flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-seventy-fourth` / `A-W524`.

## Index

| Prior | Status |
|-------|--------|
| W476–W523 narrative → Dup+LoadLocal RC true-retire | **met** |
| This window ObjectRefRelease opcode RC true-retire | **met** |
| StoreLocal / `run_class_deinits` RC → ManagedHeap/GC | **still unmet** |
| Value RC / wrap·retain·release ABI true delete | **still unmet** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Cleanup ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-y2 | Tracks → Dup/LoadLocal RC retire | **met** (W494–W523) |
| 4c-ii-y3a. ObjectRefRelease opcode RC retire | Stop RC + opcode-path Type_deinit; keep pop | **met** (this window) |
| 4c-ii-y3b. StoreLocal/deinit migrate | StoreLocal / `run_class_deinits` → ManagedHeap/GC | **still unmet** |
| 4c-ii-y. Value RC true delete | Remove `Value::object_ref_*` | **still unmet** |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Release opcode retire contract (hard)

| Item | Status | Meaning |
|------|--------|---------|
| VM `ObjectRefRelease` RC / Type_deinit | **retired** | no `release_object_ref`; still pop |
| mir-run `OBJECT_REF_RELEASE` RC / Type_deinit | **retired** | no `object_ref_release` / `run_deinit_for_value`; still pop |
| StoreLocal / `run_class_deinits` ObjectRef RC | **still live** | `storelocal_deinit_objectref_rc_still_live` |
| `Value::object_ref_retain/release` API | **still present** | `value_object_ref_retain_release_api_still_present` |
| RT ABI retain/release/wrap | **still present** | `rt_abi_object_ref_retain_release_wrap_still_present` |
| `objectref_rc_migrate_enabled` | **false** | Full migrate unmet |
| `storage_cleanup_enabled` | **false** | Full cleanup unmet |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectrefRcReleaseOpcodeRetireFirstSliceInventory` / `gate_b09_objectref_rc_release_opcode_retire_first_slice_inventory` |
| Flags | `b09_objectref_rc_release_opcode_retire_first_slice_signed=true`; `b09_objectref_rc_dup_loadlocal_retire_first_slice_floor_ok=true`; `b09_objectref_rc_retain_opcode_retire_first_slice_floor_ok=true`; `vm_mir_run_objectref_release_opcode_rc_retired=true`; `storelocal_deinit_objectref_rc_still_live=true`; `value_object_ref_retain_release_api_still_present=true`; `rt_abi_object_ref_retain_release_wrap_still_present=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | Value RC full delete / StoreLocal·deinit strip / ABI delete / flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / HostFn+1 / MIR distance change |
| Honesty | Release opcode RC retire **≠** Value RC full delete / **≠** StoreLocal/deinit retire / **≠** ABI delete / **≠** migrate flip / **≠** cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) — `ObjectRefRelease` RC bump retired / ADR-868
- [`crates/syx-mir-run/src/runtime.rs`](../../crates/syx-mir-run/src/runtime.rs) — `OBJECT_REF_RELEASE` RC bump retired / ADR-868
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectrefRcReleaseOpcodeRetireFirstSliceInventory`
- Meta: `b09_objectref_rc_release_opcode_retire_first_slice.rs`
- Floor: [`gate-b09-objectref-rc-dup-loadlocal-retire-first-slice.md`](gate-b09-objectref-rc-dup-loadlocal-retire-first-slice.md), [`gate-b09-objectref-rc-retain-opcode-retire-first-slice.md`](gate-b09-objectref-rc-retain-opcode-retire-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Deleting `Value::object_ref_*`; stripping StoreLocal / `run_class_deinits` release; deleting RT ABI retain/release/wrap
- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Nested field dual-track this window
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w523_*.plan.md`

## Next

W523 B-09 Dup+LoadLocal ObjectRef RC true-retire first-slice **已交**（ADR-867）→ W524 B-09 ObjectRefRelease opcode RC true-retire first-slice **已交**（ADR-868）→ W525 B-09 StoreLocal ObjectRef ManagedHeap unroot first-slice **已交**（ADR-869）→ W526 B-09 run_class_deinits ObjectRef ManagedHeap unroot first-slice **已交**（ADR-870）→ W527 B-09 GC↔Type_deinit sweep-finalizer first-slice **已交**（ADR-871）→ W528 B-09 StoreLocal/`run_class_deinits` ObjectRef RC true-retire first-slice **已交**（ADR-872）→ W529+ — 优先删 `Value::object_ref_*`（慎）→ 删 `Value::object_ref_*` → wrap/retain/release ABI 真删（慎）→ 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
