# Long closure-async MIR-free gap #3 close (W420 / ADR-764)

## Status

**Long closure-async MIR-free gap #3 closed** (CallFn+Await+Task+Actor 齐后签收; host-bridge still open; sidecar still embedded).

Fingerprint: `two-hundred-seventieth` / `A-W420`.

## Index

| Prior | Status |
|-------|--------|
| W419 Long Actor wire first-slice | **met** (`closure_async_actor_wire_first_slice_signed=true`; `actor_mir_interpret_fallback_gap_open=false`) |
| W418 Long Task wire first-slice | **met** (`closure_async_task_wire_first_slice_signed=true`; `task_mir_interpret_fallback_gap_open=false`) |
| W417 Long Await wire first-slice | **met** (`closure_async_await_wire_first_slice_signed=true`; `await_mir_interpret_fallback_gap_open=false`) |
| W416 Long CallFn wire first-slice | **met** (`closure_async_wire_first_slice_signed=true`; `call_fn_mir_interpret_fallback_gap_open=false`) |
| W415 Long closure-async MIR-free deepen thin | **met** (`closure_async_mir_free_deepen_thin_signed=true`) |
| W414 Long closure-async MIR-free thin | **met** (`closure_async_mir_free_thin_signed=true`) |
| W413 Long emit-omit first-slice | **partial met** (`emit_omit_first_slice_signed=true`; `@syx_mir_blob` omitted) |
| This window gap #3 close | **met** (`closure_async_mir_free_gap_close_signed=true`) |
| `closure_async_mir_free_gap_open` | **closed** (`false`) |
| `host_bridge_pure_native_gap_open` | **this-window inventory open** (`true`); current horizon **closed** in W425 |
| `@syx_metadata_sidecar` | **still embedded** (`sidecar_constant_still_embedded=true`) |
| MIR-free / `mir_free_enabled` | **still unmet** (`false`) |

## Wire ladder

| Step | Meaning | Status |
|------|---------|--------|
| 1. Closure-async thin | Charter gap #3 | **met (W414)** |
| 2. Closure-async deepen thin | Per-class prerequisites | **met (W415)** |
| 3a. CallFn wire first-slice | `syx_rt_call_fn` no MIR interpret fallback | **met (W416)** |
| 3b. Await wire first-slice | Await stubs fail-closed; entry drops Await* | **met (W417)** |
| 3c. Task wire first-slice | Task stubs fail-closed; entry drops Task* | **met (W418)** |
| 3d. Actor wire first-slice | Actor stubs fail-closed; entry drops Actor* | **met (W419)** |
| 3e. Gap #3 close | CallFn+Await+Task+Actor 齐后签收 | **met (this window)** |
| Host-bridge / omit sidecar / MIR-free | Later Long | **out of scope** (≠ this window) |

## Gap #3 close contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `closure_async_mir_free_gap_close_signed=true`; **`closure_async_mir_free_gap_open=false`** |
| Four sub-gaps | CallFn/Await/Task/Actor `*_mir_interpret_fallback_gap_open=false` (unchanged) |
| RT / llvm | **No** stub or `mir_function_needs_mir_entry` mutation this window (already fail-closed / always false) |
| Host-bridge | `host_bridge_pure_native_gap_open=true` — **not closed** |
| Honesty | Gap #3 close **≠** MIR-free / **≠** delete sidecar / **≠** host-bridge closed / **≠** B-05 `[x]` |

## Public inventory

- `MirModuleLongHorizonClosureAsyncMirFreeGapCloseInventory`
- `mir_module_long_horizon_closure_async_mir_free_gap_close_inventory`
- Predicates: Actor wire ok + four sub-gaps closed + gap closed + MIR-free off + inventory overlap (policy-fixed; no exponential parent re-walk)
- Flags: `closure_async_mir_free_gap_close_signed=true`; **`closure_async_mir_free_gap_open=false`**; Actor/Task/Await/CallFn wire signed + sub-gaps closed; `host_bridge_pure_native_gap_open=true`; `blob_still_embedded=false`; `sidecar_constant_still_embedded=true`; `mir_free_enabled=false`

Historical note: W414–W419 inventories keep `closure_async_mir_free_gap_open=true` as that-window honesty; current horizon (`wave_b` + this inventory) reports **closed**.

Cross: [`long-mir-free-closure-async-actor-wire.md`](long-mir-free-closure-async-actor-wire.md), [`long-mir-free-closure-async-task-wire.md`](long-mir-free-closure-async-task-wire.md), [`long-mir-free-closure-async-await-wire.md`](long-mir-free-closure-async-await-wire.md), [`long-mir-free-closure-async-wire.md`](long-mir-free-closure-async-wire.md), [`long-mir-free-closure-async-deepen.md`](long-mir-free-closure-async-deepen.md), [`long-mir-free-closure-async.md`](long-mir-free-closure-async.md), [`long-mir-free-emit-omit.md`](long-mir-free-emit-omit.md), [`long-mir-free-readiness.md`](long-mir-free-readiness.md), [`wave-b-horizon-readiness.md`](wave-b-horizon-readiness.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md).

## Frozen

- Hard-closing `host_bridge_pure_native_gap_open`
- Deleting `@syx_metadata_sidecar` / flipping `mir_free_enabled=true`
- True native Actor/Task/Await runtime
- Claiming B-06 `[x]` without ADR-792 exit narrow (live B-06 `[x]` via ADR-792 / W448 — [`b06-missing-align-exit.md`](b06-missing-align-exit.md); ≠ Missing 清零 / stub rewrite / B-07 `[x]` / this window)
- TLS `status=implemented` / handler deepen
- Short default emit auto multi-root; absolute byte thresholds
- Clearing W414–W419 thin/deepen/CallFn/Await/Task/Actor wire historical inventories

## Next

W420 Long closure-async MIR-free gap #3 close **已交**（ADR-764）→ W421 Long host-bridge thin **已交**（ADR-765）→ W422 Long host-bridge deepen thin **已交**（ADR-766）→ W423 Long host-bridge Print wire first-slice **已交**（ADR-767）→ W424 Long host-bridge CallHost wire first-slice **已交**（ADR-768）→ W425 Long host-bridge pure-native gap close **已交**（ADR-769）→ W426 Long sidecar-omit thin **已交**（ADR-770）→ W427 Long sidecar-omit deepen thin **已交**（ADR-771）→ W428 Long sidecar-omit wire first-slice（Empty-pool）**已交**（ADR-772）→ W429 Long sidecar-omit Register-only wire first-slice **已交**（ADR-773）→ W430 Long sidecar-omit C stub align wire first-slice **已交**（ADR-774）→ W431 Long sidecar-omit emit first-slice **已交**（ADR-775）→ W432 Long MIR-free first-slice **已交**（ADR-776）→ W433 Long named HostFn stub CALL_HOST clear first-slice **已交**（ADR-777）→ W434 Long delete_syx_mir_blob_enabled first-slice **已交**（ADR-778）→ W448 B-06 Missing/align `[x]` exit **已交**（ADR-792）→ W449 B-07 thin-open **已交**（ADR-793）→ W450 B-07 deepen thin **已交**（ADR-794）→ W451 B-07 HttpGet first-slice wire **已交**（ADR-795；≠ `[x]`）→ W452 B-07 FileRead second-slice wire **已交**（ADR-796）→ W453 B-07 TcpRead third-slice wire **已交**（ADR-797）→ W454 B-07 TcpWrite fourth-slice wire **已交**（ADR-798）→ W455 B-07 TcpHandleRelease fifth-slice wire **已交**（ADR-799）→ W456 B-07 `[x]` exit charter **已交**（ADR-800；窄口径）→ W457 TLS no-idle first-slice **已交**（ADR-801）→ W458 TLS language noIdle first-slice **已交**（ADR-802）→ W459 MIR residual honesty first-slice **已交**（ADR-803）→ W460 language exports thin-open **已交**（ADR-804）→ W461 B-03 FrontendPipeline entry matrix **已交**（ADR-805）→ W462 language exports syntax-spec deepen **已交**（ADR-806）→ W463 B-01 MIR-consumer thin-open **已交**（ADR-807）→ W464 B-01 MIR-consumer deepen thin **已交**（ADR-808）→ W465 B-02 dual-track thin-open **已交**（ADR-809）→ W466 B-02 dual-track deepen thin **已交**（ADR-810）→ W467 B-01 Struct MIR shell migrate **已交**（ADR-811）→ W468 B-01 FinalClass MIR shell migrate **已交**（ADR-812）→ W469 B-01 VarGlobal MIR shell migrate **已交**（ADR-813）→ W470 B-01 Extension MIR shell migrate **已交**（ADR-814）→ W471 B-01 Class MIR shell migrate **已交**（ADR-815）→ W472 B-01 Actor MIR shell migrate **已交**（ADR-816）→ W473 B-01 no-op residual honesty **已交**（ADR-817）→ W474 B-01 MIR-consumer narrow `[x]` exit **已交**（ADR-818）→ W475 B-02 retire-readiness thin **已交**（ADR-819）→ W476 B-09 GC narrative thin **已交**（ADR-820）→ W477 B-02 compile_into 首片 **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
