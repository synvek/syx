# B-09 Value/VM/mir-run ObjectRef RC retire-readiness thin (W521 / ADR-865)

## Status

**B-09 Value/VM/mir-run ObjectRef RC retire-readiness thin met** (policy+inventory): after W520 retain/release RT ABI retire-readiness, Value RC retire **blockers / preconditions** are signed — LLVM/WASM ObjectRef RC emit path retired (W511/W514), RT ABI retain/release/wrap still present, Value/VM/mir-run ObjectRef RC still live (Dup/LoadLocal/`OBJECT_REF_*` hot paths), and migrate remains **off**. This window does **not** delete Value/VM/mir-run RC, delete retain/release/wrap ABI, flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-seventy-first` / `A-W521`.

## Index

| Prior | Status |
|-------|--------|
| W476–W520 narrative → retain/release RT ABI retire-readiness | **met** |
| LLVM/WASM ObjectRef RC emit retire (W511/W514) | **met** |
| This window Value RC retire-readiness thin | **met** |
| Value/VM/mir-run ObjectRef RC true delete | **still unmet** |
| Retain/release / wrap RT ABI true delete | **still unmet** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-w | Tracks → retain/release RT ABI retire-readiness | **met** (W494–W520) |
| 4c-ii-x. Value RC retire-readiness | Blockers documented; Value RC still live | **met** (this window) |
| 4c-ii-y. Value RC true delete | Remove `Value::object_ref_*` / VM·mir-run retain·release | **still unmet** |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Value RC retire blocker table (hard)

| Blocker | Present? | Meaning |
|---------|----------|---------|
| LLVM/WASM ObjectRef RC emit path | **retired** | W511/W514 floors (`llvm_wasm_objectref_rc_emit_path_retired`) |
| RT ABI retain/release/wrap | **yes** | `rt_abi_object_ref_retain_release_wrap_still_present` |
| Value / VM / mir-run ObjectRef RC | **yes** | `value_vm_mir_run_objectref_rc_still_live` (Dup/LoadLocal/`OBJECT_REF_*`) |
| `objectref_rc_migrate_enabled` | **no** | still false |
| B-09 axis exit (`[x]`) / GC GA | **no** | readiness ≠ exit |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| Value RC retire-readiness signed | **yes** | This window |
| Value RC deleted | **no** | Still open |
| Retain/release / wrap RT ABI deleted | **no** | Still open |
| Full migrate enabled | **no** | Still open |
| Storage cleanup enabled | **no** | Still open |
| Unified heap enabled | **no** | Frozen |
| B-09 `[x]` / GC GA | **no** | readiness ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectrefRcRetireReadinessThinInventory` / `gate_b09_objectref_rc_retire_readiness_thin_inventory` |
| Flags | `b09_objectref_rc_retire_readiness_thin_signed=true`; `b09_rt_abi_objectref_retain_release_retire_readiness_thin_floor_ok=true`; `value_vm_mir_run_objectref_rc_still_live=true`; `rt_abi_object_ref_retain_release_wrap_still_present=true`; `llvm_wasm_objectref_rc_emit_path_retired=true`; `rc_retire_blockers_documented=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | deleting Value RC / deleting retain/release/wrap ABI / flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / scan-all auto / HostFn+1 / MIR distance change |
| Honesty | Value RC retire-readiness **≠** Value RC delete / **≠** ABI delete / **≠** migrate flip / **≠** cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectrefRcRetireReadinessThinInventory`
- Meta: ~~`b09_objectref_rc_retire_readiness_thin.rs`~~ retired（F-10 W599）；proof = this SSOT + `GateB09ObjectrefRcRetireReadinessThinInventory`
- Floor: [`gate-b09-rt-abi-objectref-retain-release-retire-readiness-thin.md`](gate-b09-rt-abi-objectref-retain-release-retire-readiness-thin.md), [`gate-b09-objectref-rc-migrate-readiness-thin.md`](gate-b09-objectref-rc-migrate-readiness-thin.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Deleting `Value::object_ref_*` / stripping VM·mir-run retain·release; deleting RT ABI retain/release/wrap
- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Nested field dual-track this window
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w520_*.plan.md`

## Next

W520 B-09 RT ABI ObjectRefRetain/Release retire-readiness thin **已交**（ADR-864）→ W521 B-09 Value/VM/mir-run ObjectRef RC retire-readiness thin **已交**（ADR-865）→ W522 B-09 ObjectRefRetain opcode RC true-retire first-slice **已交**（ADR-866）→ W523 B-09 Dup+LoadLocal ObjectRef RC true-retire first-slice **已交**（ADR-867）→ W524+ — 优先 Release/StoreLocal/deinit **迁 ManagedHeap/GC**（分窗）→ Value API 删 → wrap/retain/release ABI 真删（慎）→ 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
