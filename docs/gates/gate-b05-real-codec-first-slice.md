# B-05 Medium real codec first-slice wire (W403 / ADR-747)

## Status

**B-05 Medium real codec first-slice partial met**.

Fingerprint: `two-hundred-fifty-third` / `A-W403`.

> **W447 cross-index:** live roadmap B-05 is `[x]` via narrow exit ([`gate-b05-medium-exit.md`](../../plan/archive/gates/gate-b05-medium-exit.md)（F-10 W586 archived） / ADR-791); **this window** still ≠ dual-blob GA / ≠ B-06 `[x]`.

## Index

| Prior | Status |
|-------|--------|
| W333 Medium cutover readiness | **met** |
| W339 Medium honest defer | **met** (`honest_defer_signed=true`) |
| W401 production reopen thin | **met** (`production_reopen_thin_signed=true`) |
| W402 cutover deepen thin | **met** (`cutover_deepen_thin_signed=true`) |
| Real codec first-slice wire | **partial met** (`real_codec_enabled=true`; `real_codec_reopen_gap_open=false`) |
| Dual-blob production wire | **partial met** (W404; see [`gate-b05-dual-blob-production-wire.md`](gate-b05-dual-blob-production-wire.md); `production_dual_load_enabled=true`; `dual_blob_reopen_gap_open=false`; `aot_embeds_sidecar=true` (W406)) |
| Medium `cutover_enabled` | **partial met** (W405; `true`) |
| W400 Short cutover honesty | **partial met** — see [`gate-b04-production-cutover.md`](gate-b04-production-cutover.md) |

## Wire API

| API | Role |
|-----|------|
| `encode_module_metadata_sidecar` | Production encode of `MirModuleMetadataSidecarView` |
| `decode_module_metadata_sidecar` | Production decode / round-trip partner |
| `mir_module_metadata_sidecar_view` | View extraction (unchanged) |
| Generic `encode_module` / AOT compact | **unchanged** whole-module bincode; `@syx_mir_blob` shape unchanged |

Honesty: sidecar wire ≠ AOT sidecar embed; dual-blob production wire may be on (W404); ≠ Medium cutover; default AOT path still whole-module single blob.

## Inventory

- `MirModuleMediumHorizonRealCodecFirstSliceInventory` / `mir_module_medium_horizon_real_codec_first_slice_inventory`
- Flags: `real_codec_first_slice_signed=true`; `real_codec_enabled=true`; `real_codec_reopen_gap_open=false`; `cutover_enabled=true` (W405); `production_dual_load_enabled=true` (W404); `aot_embeds_sidecar=true` (W406); `dual_blob_reopen_gap_open=false` (W404)

Cross: [`gate-b05-dual-load-runtime.md`](gate-b05-dual-load-runtime.md), [`gate-b05-aot-sidecar-embed.md`](gate-b05-aot-sidecar-embed.md), [`gate-b05-dual-blob-production-wire.md`](gate-b05-dual-blob-production-wire.md), [`gate-b05-production-cutover-deepen.md`](gate-b05-production-cutover-deepen.md), [`gate-b05-production-reopen.md`](gate-b05-production-reopen.md), [`gate-b05-mir-metadata-blob.md`](gate-b05-mir-metadata-blob.md), [`gate-b04-multi-root-wire.md`](gate-b04-multi-root-wire.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md).

## Frozen

- Default AOT embeds metadata sidecar (`aot_embeds_sidecar` flip)
- Flipping Medium `cutover_enabled`
- Claiming B-06 `[x]` (live B-05 `[x]` via ADR-791 / W447 — [`gate-b05-medium-exit.md`](../../plan/archive/gates/gate-b05-medium-exit.md)（F-10 W586 archived）; ≠ this window)
- Deleting native MIR blob / MIR-free emit — **out of scope** (Long; ≠ this window)
- TLS `status=implemented` / handler deepen
- Short default emit auto multi-root; absolute byte thresholds
- Changing generic `encode_module` / AOT `@syx_mir_blob` shape

## Next

W414 Long closure-async MIR-free thin **已交**（ADR-758）→ W415 Long closure-async MIR-free deepen thin **已交**（ADR-759）→ W416 Long closure-async wire first-slice（CallFn）**已交**（ADR-760）→ W417 Long closure-async Await wire first-slice **已交**（ADR-761）→ W418 Long closure-async Task wire first-slice **已交**（ADR-762）→ W419 Long Actor wire first-slice **已交**（ADR-763）→ W420 Long gap #3 close **已交**（ADR-764）→ W421 Long host-bridge thin **已交**（ADR-765）→ W422 Long host-bridge deepen thin **已交**（ADR-766）→ W423 Long host-bridge Print wire first-slice **已交**（ADR-767）→ W424 Long host-bridge CallHost wire first-slice **已交**（ADR-768）→ W425 Long host-bridge pure-native gap close **已交**（ADR-769）→ W426 Long sidecar-omit thin **已交**（ADR-770）→ W427 Long sidecar-omit deepen thin **已交**（ADR-771）→ W428 Long sidecar-omit wire first-slice（Empty-pool）**已交**（ADR-772）→ W429 Long sidecar-omit Register-only wire first-slice **已交**（ADR-773）→ W430 Long sidecar-omit C stub align wire first-slice **已交**（ADR-774）→ W431 Long sidecar-omit emit first-slice **已交**（ADR-775）→ W432 Long MIR-free first-slice **已交**（ADR-776）→ W433 Long named HostFn stub CALL_HOST clear first-slice **已交**（ADR-777）→ W434 Long delete_syx_mir_blob_enabled first-slice **已交**（ADR-778）→ W447 B-05 Medium `[x]` exit charter **已交**（ADR-791）→ W448+ — B-06 `[x]` exit charter；或无-idle 长驻 API；或语言 exports→`aot_export_roots`；TLS 再施工 **out of scope**（≠ this window） unless chosen as next charter.
