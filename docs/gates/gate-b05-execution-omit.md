# B-05 Medium AOT execution-segment omit (W409 / ADR-753)

## Status

**B-05 Medium AOT execution-segment omit first-slice partial met** (default AOT embed SYXD execution is empty; RT installs sidecar-only fn stubs + string pool).

Fingerprint: `two-hundred-fifty-ninth` / `A-W409`.

> **W447 cross-index:** live roadmap B-05 is `[x]` via narrow exit ([`gate-b05-medium-exit.md`](../../plan/archive/gates/gate-b05-medium-exit.md)（F-10 W586 archived） / ADR-791); **this window** still ≠ dual-blob GA / ≠ B-06 `[x]`.

## Index

| Prior | Status |
|-------|--------|
| W333 Medium cutover readiness | **met**; Medium chain cutover/dual/codec on |
| W401–W408 reopen → sidecar-primary | **met / partial met** |
| Sidecar-primary | **partial met** (`sidecar_string_pool_primary=true`) |
| Execution-segment omit | **partial met** (`aot_execution_segment_omitted=true`; embed exec empty) |
| Default AOT `@syx_mir_blob` | **still embedded** (SYXD package; mostly sidecar; constant name retained) |
| MIR-free / delete `@syx_mir_blob` | **out of scope** for Medium (Long; see [`long-mir-free-readiness.md`](../lang/long-mir-free-readiness.md) W410 thin) |

## Execution-omit contract (hard)

| Item | Meaning |
|------|---------|
| Embed | `encode_module_aot_embed_blob` = sidecar(original) + **empty** execution (`execution_len=0`) |
| Load | SYXD empty exec → `install_runtime_from_sidecar_only` (fn stubs from `fn_names` + string pool); no `decode_module(exec)` |
| Non-empty exec | Legacy / inventory dual-blob package path retained |
| Observability | `aot_execution_segment_omitted=true`; `sidecar_string_pool_primary=true` |
| Inventory | `execution_omit_signed=true`; prior Medium flags retained |
| Honesty | `@syx_mir_blob` **still embedded**; **≠** MIR-free / **≠** blob delete / **≠** dual-blob GA (this window; live B-05 `[x]` via ADR-791) |

## Inventory

- `MirModuleMediumHorizonExecutionOmitInventory` / `mir_module_medium_horizon_execution_omit_inventory`
- Flags: `execution_omit_signed=true`; `aot_execution_segment_omitted=true`; sidecar-primary / dual-load / embed / cutover retained
- Predicates: sidecar-primary signed + peel exec empty + sidecar decode + inventory overlap

Cross: [`long-mir-free-readiness.md`](../lang/long-mir-free-readiness.md), [`gate-b05-sidecar-primary.md`](gate-b05-sidecar-primary.md), [`gate-b05-dual-load-runtime.md`](gate-b05-dual-load-runtime.md), [`gate-b05-aot-sidecar-embed.md`](gate-b05-aot-sidecar-embed.md), [`gate-b05-production-cutover.md`](gate-b05-production-cutover.md), [`gate-b05-dual-blob-production-wire.md`](gate-b05-dual-blob-production-wire.md), [`gate-b05-real-codec-first-slice.md`](gate-b05-real-codec-first-slice.md), [`gate-b05-production-cutover-deepen.md`](gate-b05-production-cutover-deepen.md), [`gate-b05-production-reopen.md`](gate-b05-production-reopen.md), [`gate-b05-mir-metadata-blob.md`](gate-b05-mir-metadata-blob.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md).

## Frozen

- Deleting native MIR blob / MIR-free emit — **out of scope** (Long; ≠ this window)
- Claiming B-06 `[x]` (live B-05 `[x]` via ADR-791 / W447 — [`gate-b05-medium-exit.md`](../../plan/archive/gates/gate-b05-medium-exit.md)（F-10 W586 archived）; ≠ this window)
- TLS `status=implemented` / handler deepen
- Short default emit auto multi-root; absolute byte thresholds

## Next

W414 Long closure-async MIR-free thin **已交**（ADR-758）→ W415 Long closure-async MIR-free deepen thin **已交**（ADR-759）→ W416 Long closure-async wire first-slice（CallFn）**已交**（ADR-760）→ W417 Long closure-async Await wire first-slice **已交**（ADR-761）→ W418 Long closure-async Task wire first-slice **已交**（ADR-762）→ W419 Long Actor wire first-slice **已交**（ADR-763）→ W420 Long gap #3 close **已交**（ADR-764）→ W421 Long host-bridge thin **已交**（ADR-765）→ W422 Long host-bridge deepen thin **已交**（ADR-766）→ W423 Long host-bridge Print wire first-slice **已交**（ADR-767）→ W424 Long host-bridge CallHost wire first-slice **已交**（ADR-768）→ W425 Long host-bridge pure-native gap close **已交**（ADR-769）→ W426 Long sidecar-omit thin **已交**（ADR-770）→ W427 Long sidecar-omit deepen thin **已交**（ADR-771）→ W428 Long sidecar-omit wire first-slice（Empty-pool）**已交**（ADR-772）→ W429 Long sidecar-omit Register-only wire first-slice **已交**（ADR-773）→ W430 Long sidecar-omit C stub align wire first-slice **已交**（ADR-774）→ W431 Long sidecar-omit emit first-slice **已交**（ADR-775）→ W432 Long MIR-free first-slice **已交**（ADR-776）→ W433 Long named HostFn stub CALL_HOST clear first-slice **已交**（ADR-777）→ W434 Long delete_syx_mir_blob_enabled first-slice **已交**（ADR-778）→ W447 B-05 Medium `[x]` exit charter **已交**（ADR-791）→ W448+ — B-06 `[x]` exit charter；或无-idle 长驻 API；或语言 exports→`aot_export_roots`；TLS 再施工 **out of scope**（≠ this window） unless chosen as next charter.
