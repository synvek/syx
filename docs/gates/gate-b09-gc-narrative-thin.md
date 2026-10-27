# B-09 GC narrative thin (W476 / ADR-820)

## Status

**B-09 GC narrative thin met** (policy+inventory): the three-layer heap honesty table (VM `MemoryPool`+`GcCollect` vs WASM/Native MirRuntime/Rust heap with **no** SYX GC) is signed, and the preferred direction (one MirRuntime mark-sweep shared eventually with VM) is documented. This window does **not** enable a unified heap product switch, claim GC GA, or flip B-09 to `[x]`. HostFn count **554**; B-01 **`[x]`**; B-02 later `[x]` (W479/W482); B-03～B-08 **`[x]`**; ≠ `Decl::Export` / ≠ daemon GA / ≠ MIR blob delete / ≠ B-02 `[x]` at W476.

**Follow-on:** W485 / ADR-829 GC narrative **deepen** signs construction checklist (root set / VM converge / wasm-gc deferred) while keeping product switches **false** — see [`gate-b09-gc-narrative-deepen.md`](gate-b09-gc-narrative-deepen.md). B-09 remains **`[~]`**.

Fingerprint: `three-hundred-and-twenty-sixth` / `A-W476`.

## Index

| Prior | Status |
|-------|--------|
| [`gc-parity.md`](../../plan/topics/gc-parity.md) three-layer table | **present** (floor) |
| This window GC narrative thin | **met** |
| Unified heap construction deepen | **met** (W485 / ADR-829) |
| Unified MirRuntime mark-sweep product switch | **still unmet** |
| B-09 `[x]` / GC GA | **still unmet** |
| B-02 dual-track retired / `[x]` | **met** later (W479/W482); unmet at W476 signing |

## Narrative ladder

| Step | Meaning | Status |
|------|---------|--------|
| 1. Three-layer heap honesty | VM vs WASM vs Native | **met** (topic + this window) |
| 2. Preferred unified direction documented | MirRuntime mark-sweep | **met** (this window) |
| 3. Unified heap product enable / deepen | construction deepen | **deepen met** (W485); **enable still unmet** |
| 4. B-09 `[x]` | axis exit | **still unmet** |

## Three-layer table (hard)

| Path | Heap story | SYX GC |
|------|------------|--------|
| `syx run` (VM) | VM `MemoryPool` + weak `GcCollect` | Experimental / limited |
| `syx run --wasm` | Guest WASM → host **`MirRuntime`** (Rust heap) | No SYX GC on objects |
| `syx build --native` | Native code + MIR fallback → **`MirRuntime` / Rust heap** | No SYX GC |

Cross-index: [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md).

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| Three-layer table documented | **yes** | This window |
| Preferred MirRuntime direction documented | **yes** | This window |
| Construction deepen documented | **yes** | W485 |
| Unified MirRuntime heap enabled | **no** | Product switch off |
| GC GA claimed | **no** | Frozen |
| B-09 axis exit (`[x]`) | **no** | thin/deepen ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB09GcNarrativeThinInventory` / `gate_b09_gc_narrative_thin_inventory` |
| Flags | `b09_gc_narrative_thin_signed=true`; `three_layer_heap_table_documented=true`; `vm_gc_collect_path_documented=true`; `wasm_native_no_syx_gc_documented=true`; `preferred_unified_mir_runtime_direction_documented=true`; `unified_mir_runtime_heap_enabled=false`; `b09_axis_exit_enabled=false`; `gc_ga_claimed=false` |
| Forbidden claim | unified heap enabled / GC GA / B-09 `[x]` / B-02 `[x]` (at W476) / `Decl::Export` / daemon GA / MIR blob re-embed / HostFn+1 |
| Honesty | narrative thin **≠** unified heap implementation / **≠** B-09 `[x]` / **≠** GC GA |

## Evidence

- [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) `GateB09GcNarrativeThinInventory`
- Meta: ~~`b09_gc_narrative_thin.rs`~~ retired（F-10 W597）；proof = this SSOT + `GateB09GcNarrativeThinInventory`
- Topic: [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md)
- Cross: [`gate-b09-gc-narrative-deepen.md`](gate-b09-gc-narrative-deepen.md), [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-09

## Frozen

- Enabling unified MirRuntime mark-sweep product switch / rewriting VM·WASM·Native heap / claiming GC GA / wasm-gc
- Claiming B-09 `[x]` / B-02 `[x]` / dual-track retired (at W476 window)
- `TokenKind::Export` / `Decl::Export` / parser `export`
- Claiming daemon GA / MIR blob re-embed / HostFn+1 / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w475_*.plan.md`
- Mutating W476 thin inventory floor flags

## Next

W476 B-09 GC narrative thin **已交**（ADR-820）→ … → W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487 B-09 MirRuntime 根集首片 **已交**（ADR-831）→ W488 B-09 VM GcCollect 根集首片 **已交**（ADR-832）→ W489 B-09 mailbox Value 队列根集首片 **已交**（ADR-833）→ W490 B-09 dual-heap converge 首片 **已交**（ADR-834）→ W491 B-09 fully_converged 窄片 **已交**（ADR-835）→ W492 B-09 ObjectRef/MemoryPool storage honesty thin **已交**（ADR-836）→ W493 B-09 enable-readiness thin **已交**（ADR-837）→ W494 B-09 storage-cleanup first-slice（死 API 退役）**已交**（ADR-838）→ W495 B-09 MemoryPool dead-shell first-slice **已交**（ADR-839）→ W496 B-09 ObjectRef RC migrate first-slice（窄）**已交**（ADR-840）→ W497 B-09 ObjectRef VM ManagedHeap probe first-slice **已交**（ADR-841）→ W498 B-09 ObjectRef VM heap_slots first-slice **已交**（ADR-842）→ W499 B-09 ObjectRef mir-run DictBuild ManagedHeap align first-slice **已交**（ADR-843）→ W500 B-09 ObjectRef VM CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-844）→ W501 B-09 ObjectRef mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-845）→ W502 B-09 MutRefCell VM DictBuild ManagedHeap + heap_slots first-slice **已交**（ADR-846）→ W503 B-09 MutRefCell mir-run DictBuild ManagedHeap + heap_slots align **已交**（ADR-847）→ W504 B-09 MutRefCell VM CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-848）→ W505 B-09 MutRefCell mir-run CallHost ManagedHeap + heap_slots first-slice **已交**（ADR-849）→ W506 B-09 ObjectRef+MutRefCell VM call-return（finish_call）ManagedHeap + heap_slots first-slice **已交**（ADR-850）→ W507 B-09 ObjectRef+MutRefCell mir-run call-return（call_fn + call_closure）ManagedHeap + heap_slots first-slice **已交**（ADR-851）→ W508 B-09 ObjectRef+MutRefCell ActorCall（VM+mir-run runtime）ManagedHeap + heap_slots first-slice **已交**（ADR-852）→ W509 B-09 ObjectRef+MutRefCell MethodCall（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-853）→ W510 B-09 ObjectRefRetain catch-all（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-854）→ W511 B-09 LLVM·WASM ObjectRefRetain/Release emit-path 退役 first-slice **已交**（ADR-855）→ W512 B-09 ObjectRef ObjectGet residual re-entry + birth inventory ManagedHeap + heap_slots first-slice **已交**（ADR-856）→ W513 B-09 ObjectRef ArrayGet（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-857）→ W515 B-09 ObjectSet MutRefCell 写回（VM+mir-run）ManagedHeap + heap_slots first-slice **已交**（ADR-859）→ W516 B-09 LoadGlobal（VM+mir-run/SSA）ManagedHeap + heap_slots first-slice **已交**（ADR-860）→ W517 B-09 SSA LoadLocal ManagedHeap + heap_slots first-slice **已交**（ADR-861）→ W518 B-09 ObjectRef RC migrate-readiness thin **已交**（ADR-862）→ W519+ — 优先 Value/VM/mir-run RC 与 RT ABI retain/release/wrap **诚实退役** → 翻 `objectref_rc_migrate_enabled` / 翻 cleanup / 真 enable；或 nested field dual-track；或另钉扫全 auto；或 exports GA；或 RT ABI 全清（慎，撞 B-05）
