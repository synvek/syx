# B-05 Medium production cutover honesty flip (W405 / ADR-749)

## Status

**B-05 Medium production cutover honesty partial met** (honesty flag flip; after W406 default AOT embeds SYXD sidecar package — see [`gate-b05-aot-sidecar-embed.md`](gate-b05-aot-sidecar-embed.md)).

Fingerprint: `two-hundred-fifty-fifth` / `A-W405`.

> **W447 cross-index:** live roadmap B-05 is `[x]` via narrow exit ([`gate-b05-medium-exit.md`](../../plan/archive/gates/gate-b05-medium-exit.md)（F-10 W586 archived） / ADR-791); **this window** still ≠ dual-blob GA / ≠ B-06 `[x]`.

## Index

| Prior | Status |
|-------|--------|
| W333 Medium cutover readiness | **met**; Medium chain `cutover_enabled=true` |
| W339 Medium honest defer | **met**; production honesty requires cutover on |
| W401–W404 reopen / deepen / codec / dual-blob wire | **met / partial met**; gaps closed |
| Real codec first-slice | **partial met** (`real_codec_enabled=true`) |
| Dual-blob production wire | **partial met** (`production_dual_load_enabled=true`; `dual_blob_reopen_gap_open=false`) |
| Medium `cutover_enabled` honesty | **partial met** (`true`) |
| Default AOT `@syx_mir_blob` | **W406: SYXD package embeds sidecar** (`aot_embeds_sidecar=true`; see [`gate-b05-aot-sidecar-embed.md`](gate-b05-aot-sidecar-embed.md)) |
| `syx_rt_load_mir_blob` | **peels execution whole-module** (`load_path_is_whole_module=false` (W407); ≠ dual-load RT) |
| W400 Short cutover honesty | **partial met** — see [`gate-b04-production-cutover.md`](gate-b04-production-cutover.md) |

## Cutover honesty (hard)

| Item | Meaning |
|------|---------|
| Flag | Medium inventory chain `cutover_enabled=true` |
| Predicates | readiness / defer / dual-wire / horizon require cutover on |
| Behavior nail | Cutover honesty on; AOT sidecar embed **partial met** after W406 (`aot_embeds_sidecar=true`; load still execution whole-module) |
| Dual-blob package API | Unchanged (W404) |
| Horizon | `wave_b_horizon.medium_cutover_enabled=true`; AOT sidecar embed **on** after W406 |

## Inventory

- `MirModuleMediumHorizonProductionCutoverInventory` / `mir_module_medium_horizon_production_cutover_inventory`
- Flags: `production_cutover_signed=true`; `cutover_enabled=true`; dual/codec wire on; gaps closed; after W406 `aot_embeds_sidecar=true`; `load_path_is_whole_module=false` (W407); `relational_default_aot_whole_ok`

Cross: [`gate-b05-dual-load-runtime.md`](gate-b05-dual-load-runtime.md), [`gate-b05-aot-sidecar-embed.md`](gate-b05-aot-sidecar-embed.md), [`gate-b05-dual-blob-production-wire.md`](gate-b05-dual-blob-production-wire.md), [`gate-b05-real-codec-first-slice.md`](gate-b05-real-codec-first-slice.md), [`gate-b05-production-cutover-deepen.md`](gate-b05-production-cutover-deepen.md), [`gate-b05-production-reopen.md`](gate-b05-production-reopen.md), [`gate-b05-mir-metadata-blob.md`](gate-b05-mir-metadata-blob.md), [`gate-b04-production-cutover.md`](gate-b04-production-cutover.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md).

## Frozen

- Dual-load runtime (sidecar→RT metadata tables; ≠ peel-only load)
- Claiming B-06 `[x]` (live B-05 `[x]` via ADR-791 / W447 — [`gate-b05-medium-exit.md`](../../plan/archive/gates/gate-b05-medium-exit.md)（F-10 W586 archived）; ≠ this window)
- Deleting native MIR blob / MIR-free emit — **out of scope** (Long; ≠ this window)
- TLS `status=implemented` / handler deepen
- Short default emit auto multi-root; absolute byte thresholds

## Next

W405 B-05 Medium production cutover honesty **已交**（ADR-749）→ W409 B-05 Medium AOT execution-segment omit first-slice **已交**（ADR-753）→ W414 Long closure-async MIR-free thin **已交**（ADR-758）→ W415 Long closure-async MIR-free deepen thin **已交**（ADR-759）→ W416 Long closure-async wire first-slice（CallFn）**已交**（ADR-760）→ W417 Long closure-async Await wire first-slice **已交**（ADR-761）→ W418 Long closure-async Task wire first-slice **已交**（ADR-762）→ W419 Long Actor wire first-slice **已交**（ADR-763）→ W420 Long gap #3 close **已交**（ADR-764）→ W421 Long host-bridge thin **已交**（ADR-765）→ W422 Long host-bridge deepen thin **已交**（ADR-766）→ W423 Long host-bridge Print wire first-slice **已交**（ADR-767）→ W424 Long host-bridge CallHost wire first-slice **已交**（ADR-768）→ W425 Long host-bridge pure-native gap close **已交**（ADR-769）→ W426 Long sidecar-omit thin **已交**（ADR-770）→ W427 Long sidecar-omit deepen thin **已交**（ADR-771）→ W428 Long sidecar-omit wire first-slice（Empty-pool）**已交**（ADR-772）→ W429 Long sidecar-omit Register-only wire first-slice **已交**（ADR-773）→ W430 Long sidecar-omit C stub align wire first-slice **已交**（ADR-774）→ W431 Long sidecar-omit emit first-slice **已交**（ADR-775）→ W432 Long MIR-free first-slice **已交**（ADR-776）→ W433 Long named HostFn stub CALL_HOST clear first-slice **已交**（ADR-777）→ W434 Long delete_syx_mir_blob_enabled first-slice **已交**（ADR-778）→ W447 B-05 Medium `[x]` exit charter **已交**（ADR-791）→ W448+ — B-06 `[x]` exit charter；或无-idle 长驻 API；或语言 exports→`aot_export_roots`；TLS 再施工 **out of scope**（≠ this window） unless chosen as next charter.
