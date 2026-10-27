# Long sidecar-omit emit first-slice (W431 / ADR-775)

## Status

**Long sidecar-omit emit first-slice met** (llvm/C stub stop `@syx_metadata_sidecar`; production `syx_user_init` → empty install + `register_native_fn`; `mir_free_enabled` still false).

Fingerprint: `two-hundred-eighty-first` / `A-W431`.

## Index

| Prior | Status |
|-------|--------|
| W430 Long sidecar-omit C stub align wire | **met** (`sidecar_omit_c_stub_align_wire_first_slice_signed=true`; `c_stub_register_loop_gap_open=false`) |
| W429 Register-only / W428 Empty-pool | **met** |
| W413 Long blob emit-omit first-slice | **partial met** (`@syx_mir_blob` omitted; that window still nested sidecar) |
| This window sidecar emit | **met** (`sidecar_omit_emit_first_slice_signed=true`; `sidecar_constant_still_embedded=false`) |
| MIR-free / `mir_free_enabled` | **still unmet** (`false`) |

## Emit contract (hard)

| Item | Meaning |
|------|---------|
| Embed | **no** `@syx_metadata_sidecar` / `syx_metadata_sidecar[]` |
| Init | `syx_rt_install_empty_runtime()` then `syx_rt_register_native_fn` loop |
| Forbidden in user_init | `syx_rt_install_metadata_sidecar` / `syx_rt_load_mir_blob` |
| Honesty | sidecar emit **≠** MIR-free / **≠** B-05 `[x]` / **≠** named stub clear |

## Public inventory

- `MirModuleLongHorizonSidecarOmitEmitFirstSliceInventory`
- `mir_module_long_horizon_sidecar_omit_emit_first_slice_inventory`
- Predicates: c-stub align ok + sidecar emit ok + sidecar not embedded + MIR-free off + inventory overlap (policy-fixed)
- Flags: `sidecar_omit_emit_first_slice_signed=true`; **`sidecar_constant_still_embedded=false`**; c-stub/register/empty gaps closed; `blob_still_embedded=false`; `mir_free_enabled=false`

Historical note: W413–W430 inventories keep `sidecar_constant_still_embedded=true` as that-window honesty.

Cross: [`long-mir-free-sidecar-omit-c-stub-align-wire.md`](long-mir-free-sidecar-omit-c-stub-align-wire.md), [`long-mir-free-sidecar-omit-deepen.md`](long-mir-free-sidecar-omit-deepen.md), [`long-mir-free-emit-omit.md`](long-mir-free-emit-omit.md), [`wave-b-horizon-readiness.md`](wave-b-horizon-readiness.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md).

## Frozen

- flipping `mir_free_enabled=true`
- Batch-rewriting named HostFn stubs still on `rt_direct_exec(CALL_HOST)`
- Claiming B-06 `[x]` without ADR-792 exit narrow (live B-06 `[x]` via ADR-792 / W448 — [`b06-missing-align-exit.md`](b06-missing-align-exit.md); ≠ Missing 清零 / stub rewrite / B-07 `[x]` / this window)
- TLS `status=implemented` / handler deepen
- Short default emit auto multi-root; absolute byte thresholds
- Clearing W413–W430 historical inventory sidecar-embedded honesty
- Claiming full MIR-free / production-default JIT

## Next

W431 Long sidecar-omit emit first-slice **已交**（ADR-775）→ W432 Long MIR-free first-slice **已交**（ADR-776）→ W433 Long named HostFn stub CALL_HOST clear first-slice **已交**（ADR-777）→ W434 Long delete_syx_mir_blob_enabled first-slice **已交**（ADR-778）→ W448 B-06 Missing/align `[x]` exit **已交**（ADR-792）→ W449 B-07 thin-open **已交**（ADR-793）→ W450 B-07 deepen thin **已交**（ADR-794）→ W451 B-07 HttpGet first-slice wire **已交**（ADR-795；≠ `[x]`）→ W452 B-07 FileRead second-slice wire **已交**（ADR-796）→ W453 B-07 TcpRead third-slice wire **已交**（ADR-797）→ W454 B-07 TcpWrite fourth-slice wire **已交**（ADR-798）→ W455 B-07 TcpHandleRelease fifth-slice wire **已交**（ADR-799）→ W456 B-07 `[x]` exit charter **已交**（ADR-800；窄口径）→ W457 TLS no-idle first-slice **已交**（ADR-801）→ W458 TLS language noIdle first-slice **已交**（ADR-802）→ W459 MIR residual honesty first-slice **已交**（ADR-803）→ W460 language exports thin-open **已交**（ADR-804）→ W461 B-03 FrontendPipeline entry matrix **已交**（ADR-805）→ W462 language exports syntax-spec deepen **已交**（ADR-806）→ W463 B-01 MIR-consumer thin-open **已交**（ADR-807）→ W464 B-01 MIR-consumer deepen thin **已交**（ADR-808）→ W465 B-02 dual-track thin-open **已交**（ADR-809）→ W466 B-02 dual-track deepen thin **已交**（ADR-810）→ W467 B-01 Struct MIR shell migrate **已交**（ADR-811）→ W468 B-01 FinalClass MIR shell migrate **已交**（ADR-812）→ W469 B-01 VarGlobal MIR shell migrate **已交**（ADR-813）→ W470 B-01 Extension MIR shell migrate **已交**（ADR-814）→ W471 B-01 Class MIR shell migrate **已交**（ADR-815）→ W472 B-01 Actor MIR shell migrate **已交**（ADR-816）→ W473 B-01 no-op residual honesty **已交**（ADR-817）→ W474 B-01 MIR-consumer narrow `[x]` exit **已交**（ADR-818）→ W475 B-02 retire-readiness thin **已交**（ADR-819）→ W476 B-09 GC narrative thin **已交**（ADR-820）→ W477 B-02 compile_into 首片 **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
