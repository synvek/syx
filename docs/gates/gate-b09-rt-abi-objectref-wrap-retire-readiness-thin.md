# B-09 RT ABI ObjectRefWrap retire-readiness thin (W519 / ADR-863)

## Status

**B-09 RT ABI ObjectRefWrap retire-readiness thin met** (policy+inventory): after W518 migrate-readiness, wrap RT ABI retire **blockers / preconditions** are signed — LLVM ObjectRefWrap emit retired (W514), RT ABI `syx_rt_object_ref_wrap` still present, RT ABI retain/release still present, Value/VM/mir-run ObjectRef RC still live, and migrate remains **off**. This window does **not** delete wrap/retain/release ABI, delete Value/VM/mir-run RC, flip `objectref_rc_migrate_enabled` / `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-sixty-ninth` / `A-W519`.

## Index

| Prior | Status |
|-------|--------|
| W476–W518 narrative → migrate-readiness | **met** |
| LLVM ObjectRefWrap emit retire (W514) | **met** |
| This window wrap RT ABI retire-readiness thin | **met** |
| Wrap RT ABI true delete (`syx_rt_object_ref_wrap`) | **still unmet** |
| Retain/release RT ABI retire / delete | **still unmet** |
| Full ObjectRef→ManagedHeap migrate (`objectref_rc_migrate_enabled`) | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Readiness ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-u | Tracks → migrate-readiness | **met** (W494–W518) |
| 4c-ii-v. Wrap RT ABI retire-readiness | Blockers documented; wrap ABI still present | **met** (this window) |
| 4c-ii-w. Wrap RT ABI true delete | Remove `syx_rt_object_ref_wrap` | **still unmet** |
| 4c-ii. Storage-layer cleanup / full migrate | Flip migrate + cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## Wrap retire blocker table (hard)

| Blocker | Present? | Meaning |
|---------|----------|---------|
| LLVM ObjectRefWrap emit | **retired** | W514 floor (`llvm_objectref_wrap_emit_retired`) |
| RT ABI `syx_rt_object_ref_wrap` | **yes** | `rt_abi_object_ref_wrap_still_present` |
| RT ABI `syx_rt_object_ref_retain/release` | **yes** | `rt_abi_object_ref_retain_release_still_present` |
| Value / VM / mir-run ObjectRef RC | **yes** | `value_vm_mir_run_objectref_rc_still_live` |
| `objectref_rc_migrate_enabled` | **no** | still false |
| B-09 axis exit (`[x]`) / GC GA | **no** | readiness ≠ exit |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| Wrap retire-readiness signed | **yes** | This window |
| Wrap RT ABI deleted | **no** | Still open |
| Retain/release ABI retired | **no** | Still open |
| Full migrate enabled | **no** | Still open |
| Storage cleanup enabled | **no** | Still open |
| Unified heap enabled | **no** | Frozen |
| B-09 `[x]` / GC GA | **no** | readiness ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09RtAbiObjectrefWrapRetireReadinessThinInventory` / `gate_b09_rt_abi_objectref_wrap_retire_readiness_thin_inventory` |
| Flags | `b09_rt_abi_objectref_wrap_retire_readiness_thin_signed=true`; `b09_objectref_rc_migrate_readiness_thin_floor_ok=true`; `llvm_objectref_wrap_emit_retired=true`; `rt_abi_object_ref_wrap_still_present=true`; `rt_abi_object_ref_retain_release_still_present=true`; `value_vm_mir_run_objectref_rc_still_live=true`; `wrap_retire_blockers_documented=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | deleting wrap/retain/release ABI / Value RC deleted / flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / scan-all auto / HostFn+1 / MIR distance change |
| Honesty | wrap retire-readiness **≠** wrap ABI delete / **≠** retain/release retire / **≠** migrate flip / **≠** cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09RtAbiObjectrefWrapRetireReadinessThinInventory`
- Meta: ~~`b09_rt_abi_objectref_wrap_retire_readiness_thin.rs`~~ retired（F-10 W600）；proof = this SSOT + `GateB09RtAbiObjectrefWrapRetireReadinessThinInventory`
- Floor: [`gate-b09-objectref-rc-migrate-readiness-thin.md`](gate-b09-objectref-rc-migrate-readiness-thin.md), [`gate-b09-llvm-objectref-wrap-emit-retire-first-slice.md`](gate-b09-llvm-objectref-wrap-emit-retire-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Deleting `syx_rt_object_ref_wrap` / retain / release; deleting Value / VM / mir-run RC
- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Nested field dual-track this window
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w518_*.plan.md`

## Next

W518 B-09 ObjectRef RC migrate-readiness thin **已交**（ADR-862）→ W519 B-09 RT ABI ObjectRefWrap retire-readiness thin **已交**（ADR-863）→ W520 B-09 RT ABI ObjectRefRetain/Release retire-readiness thin **已交**（ADR-864）→ W521 B-09 Value/VM/mir-run ObjectRef RC retire-readiness thin **已交**（ADR-865）→ W522 B-09 ObjectRefRetain opcode RC true-retire first-slice **已交**（ADR-866）→ W523 B-09 Dup+LoadLocal ObjectRef RC true-retire first-slice **已交**（ADR-867）→ W524+ — 优先 Release/StoreLocal/deinit **迁 ManagedHeap/GC**（分窗）→ Value API 删 → wrap/retain/release ABI 真删（慎）→ 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
