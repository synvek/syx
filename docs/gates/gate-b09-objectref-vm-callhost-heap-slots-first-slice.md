# B-09 ObjectRef VM CallHost ManagedHeap + heap_slots first-slice (W500 / ADR-844)

## Status

**B-09 ObjectRef VM CallHost ManagedHeap + heap_slots first-slice met**: after W499 mir-run DictBuild align, VM `CallHost` success paths dual-register stack-top `ObjectRef` into ManagedHeap and push `heap_slots` (identity-dedup for reused host handles such as `make_lazy_proxy`). This window does **not** enable full ObjectRef→ManagedHeap migrate, flip `storage_cleanup_enabled`, enable unified MirRuntime heap, claim B-09 `[x]`, or claim GC GA. HostFn count **554**; B-02 **`[x]`**; B-01/B-03～B-08 **`[x]`**; B-09 **still `[~]`**; ≠ scan-all auto / ≠ exports GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-fiftieth` / `A-W500`.

## Index

| Prior | Status |
|-------|--------|
| W476–W498 narrative → VM heap_slots | **met** |
| W499 ObjectRef mir-run DictBuild align | **met** |
| This window VM CallHost ObjectRef track | **met** |
| mir-run CallHost ObjectRef track | **still unmet** (W501+) |
| Full ObjectRef→ManagedHeap migrate | **still unmet** |
| Full storage cleanup (`storage_cleanup_enabled`) | **still unmet** |
| Unified MirRuntime heap product enable | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |

## Cleanup ladder

| Step | Meaning | Status |
|------|---------|--------|
| 4c-i … 4c-ii-c | Dead API → probe → VM heap_slots → mir-run DictBuild align | **met** (W494–W499) |
| 4c-ii-d. VM CallHost ObjectRef track | CallHost success → ManagedHeap + `heap_slots` (identity-dedup) | **met** (this window) |
| 4c-ii. Storage-layer cleanup | Full ObjectRef→ManagedHeap; flip cleanup | **still unmet** |
| 4d. True enable | `unified_mir_runtime_heap_enabled` | **still unmet** |
| 5. B-09 `[x]` / GC GA | Axis exit | **still unmet** |

## CallHost track contract (hard)

| Item | Status | Meaning |
|------|--------|---------|
| `maybe_track_stack_top_objectref` after CallHost Ok | **wired** | Stack-top ObjectRef → `ManagedHeap::alloc` + `heap_slots.push` when enabled |
| Same Arc identity already in `heap_slots` | **deduped** | Avoid duplicate alloc for cached host handles |
| DictBuild ObjectRef path | **reuses helper** | Behavior unchanged vs W497/W498 |
| MutRefCell DictBuild branch | **unchanged** | Frozen |
| mir-run `call_host` | **unchanged** | W501+ |
| ObjectRef retain/release (all backends) | **still present** | Live RC |
| `objectref_rc_migrate_enabled` | **false** | Full migrate unmet |
| `storage_cleanup_enabled` | **false** | Full cleanup unmet |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09ObjectRefVmCallhostHeapSlotsFirstSliceInventory` / `gate_b09_objectref_vm_callhost_heap_slots_first_slice_inventory` |
| Flags | `b09_objectref_vm_callhost_heap_slots_first_slice_signed=true`; `b09_objectref_mir_run_dictbuild_align_first_slice_floor_ok=true`; `vm_callhost_objectref_managed_heap_registered=true`; `vm_callhost_objectref_heap_slots_tracked=true`; `objectref_rc_storage_still_present=true`; `mir_run_llvm_wasm_objectref_rc_still_live=true`; `objectref_rc_migrate_enabled=false`; `storage_cleanup_enabled=false`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | flipping migrate_enabled / storage cleanup / unified heap / B-09 `[x]` / GC GA / wasm-gc / full ObjectRef deleted / scan-all auto / RT ABI delete / HostFn+1 / MIR distance change |
| Honesty | VM CallHost track **≠** full migrate / **≠** storage cleanup enabled / **≠** enable / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) — `exec_call_host` + `maybe_track_stack_top_objectref` / `object_ref_identity` dedup / ADR-844
- [`crates/syx-vm/src/stdlib_phase120_hosts.rs`](../../crates/syx-vm/src/stdlib_phase120_hosts.rs) — host ObjectRef example (`make_lazy_proxy`) covered by CallHost track
- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09ObjectRefVmCallhostHeapSlotsFirstSliceInventory`
- Meta: `b09_objectref_vm_callhost_heap_slots_first_slice.rs`
- Floor: [`gate-b09-objectref-mir-run-dictbuild-align-first-slice.md`](gate-b09-objectref-mir-run-dictbuild-align-first-slice.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09, [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Enabling `objectref_rc_migrate_enabled` / `storage_cleanup_enabled` / `unified_mir_runtime_heap_enabled` / claiming B-09 `[x]` / GC GA / wasm-gc
- Full ObjectRef→ManagedHeap migrate / deleting live retain/release this window / MutRefCell migrate / LLVM·WASM migrate / mir-run `call_host` change
- Rewriting three-layer table as “WASM/Native already has SYX GC”
- Scan-all auto / exports GA / deleting `syx_rt_load_mir_blob` RT ABI
- Claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w499_*.plan.md`

## Next

W500 B-09 ObjectRef VM CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-844）→ W501 B-09 ObjectRef mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-845）→ W502 B-09 MutRefCell VM DictBuild ManagedHeap + heap_slots first-slice **已交**（ADR-846）→ W503 B-09 MutRefCell mir-run DictBuild ManagedHeap + heap_slots align **已交**（ADR-847）→ W504 B-09 MutRefCell VM CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-848）→ W505 B-09 MutRefCell mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-849）→ W506 B-09 ObjectRef+MutRefCell VM call-return（finish_call）ManagedHeap + heap_slots first-slice **已交**（ADR-850）→ W507 B-09 ObjectRef+MutRefCell mir-run call-return（call_fn + call_closure）ManagedHeap + heap_slots first-slice **已交**（ADR-851）→ W508 B-09 ObjectRef+MutRefCell ActorCall（VM+mir-run runtime）ManagedHeap + heap_slots first-slice **已交**（ADR-852）→ W509 B-09 ObjectRef+MutRefCell MethodCall（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-853）→ W510 B-09 ObjectRefRetain catch-all（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-854）→ W511 B-09 LLVM·WASM ObjectRefRetain/Release emit-path 退役 first-slice **已交**（ADR-855）→ W512 B-09 ObjectRef ObjectGet residual re-entry + birth inventory ManagedHeap + heap_slots first-slice **已交**（ADR-856）→ W513 B-09 ObjectRef ArrayGet（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-857）→ W515 B-09 ObjectSet MutRefCell 写回（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-859）→ W516 B-09 LoadGlobal（VM+mir-run/SSA）ManagedHeap + heap_slots first-slice **已交**（ADR-860）→ W517 B-09 SSA LoadLocal ManagedHeap + heap_slots first-slice **已交**（ADR-861）→ W518 B-09 ObjectRef RC migrate-readiness thin **已交**（ADR-862）→ W519+ — 优先 Value/VM/mir-run RC 与 RT ABI retain/release/wrap **诚实退役** → 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
