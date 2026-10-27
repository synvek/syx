# B-05 Medium dual-blob production wire (W404 / ADR-748)

## Status

**B-05 Medium dual-blob production wire partial met**.

Fingerprint: `two-hundred-fifty-fourth` / `A-W404`.

> **W447 cross-index:** live roadmap B-05 is `[x]` via narrow exit ([`gate-b05-medium-exit.md`](../../plan/archive/gates/gate-b05-medium-exit.md)（F-10 W586 archived） / ADR-791); **this window** still ≠ dual-blob GA / ≠ B-06 `[x]`.

## Index

| Prior | Status |
|-------|--------|
| W333 Medium cutover readiness | **met** |
| W339 Medium honest defer | **met** (`honest_defer_signed=true`) |
| W401 production reopen thin | **met** (`production_reopen_thin_signed=true`) |
| W402 cutover deepen thin | **met** (`cutover_deepen_thin_signed=true`) |
| W403 real codec first-slice | **partial met** (`real_codec_enabled=true`; `real_codec_reopen_gap_open=false`) |
| Dual-blob production wire | **partial met** (`production_dual_load_enabled=true`; `dual_blob_reopen_gap_open=false`) |
| Default AOT `@syx_mir_blob` | **W406 SYXD package embeds sidecar** (`aot_embeds_sidecar=true`) |
| Medium `cutover_enabled` | **partial met** (W405; see [`gate-b05-production-cutover.md`](gate-b05-production-cutover.md); `true`) |
| W400 Short cutover honesty | **partial met** — see [`gate-b04-production-cutover.md`](gate-b04-production-cutover.md) |

## Wire API

| API | Role |
|-----|------|
| `encode_module_dual_blob_package` | Framed package: sidecar segment + whole-module execution segment |
| `decode_module_dual_blob_package` | Round-trip partner → `MirModuleDualBlobPackage` |
| `dual_blob_package_sidecar_bytes` / `dual_blob_package_execution_bytes` | Assertion helpers |
| `encode_module_metadata_sidecar` / `decode_module_metadata_sidecar` | Sidecar segment (W403; unchanged) |
| Generic `encode_module` / AOT compact | **unchanged**; default `@syx_mir_blob` shape unchanged |

Framing: magic `SYXD` + version + reserved(3) + sidecar_len(u32 LE) + execution_len(u32 LE) + sidecar bytes + execution bytes.

Honesty: dual-blob production wire ≠ default AOT sidecar embed; Medium cutover honesty may be on (W405); default AOT path still whole-module single blob; `syx_rt_load_mir_blob` still whole-module load.

## Inventory

- `MirModuleMediumHorizonDualBlobProductionWireInventory` / `mir_module_medium_horizon_dual_blob_production_wire_inventory`
- Flags: `dual_blob_production_wire_signed=true`; `production_dual_load_enabled=true`; `dual_blob_reopen_gap_open=false`; `real_codec_enabled=true`; `aot_embeds_sidecar=true` (W406); `cutover_enabled=true` (W405); `load_path_is_whole_module=false` (W407)

Cross: [`gate-b05-dual-load-runtime.md`](gate-b05-dual-load-runtime.md), [`gate-b05-aot-sidecar-embed.md`](gate-b05-aot-sidecar-embed.md), [`gate-b05-production-cutover.md`](gate-b05-production-cutover.md), [`gate-b05-real-codec-first-slice.md`](gate-b05-real-codec-first-slice.md), [`gate-b05-production-cutover-deepen.md`](gate-b05-production-cutover-deepen.md), [`gate-b05-production-reopen.md`](gate-b05-production-reopen.md), [`gate-b05-mir-metadata-blob.md`](gate-b05-mir-metadata-blob.md), [`gate-b04-multi-root-wire.md`](gate-b04-multi-root-wire.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md).

## Frozen

- Dual-load runtime (sidecar→RT metadata tables; peel-only load ≠ dual-load)
- Claiming B-06 `[x]` (live B-05 `[x]` via ADR-791 / W447 — [`gate-b05-medium-exit.md`](../../plan/archive/gates/gate-b05-medium-exit.md)（F-10 W586 archived）; ≠ this window)
- Deleting native MIR blob / MIR-free emit — **out of scope** (Long; ≠ this window)
- TLS `status=implemented` / handler deepen
- Short default emit auto multi-root; absolute byte thresholds
- Changing generic `encode_module` whole-module wire (execution segment still compact-for-aot in AOT embed)
## Next

W414 Long closure-async MIR-free thin **已交**（ADR-758）→ W415 Long closure-async MIR-free deepen thin **已交**（ADR-759）→ W416 Long closure-async wire first-slice（CallFn）**已交**（ADR-760）→ W417 Long closure-async Await wire first-slice **已交**（ADR-761）→ W418 Long closure-async Task wire first-slice **已交**（ADR-762）→ W419 Long Actor wire first-slice **已交**（ADR-763）→ W420 Long gap #3 close **已交**（ADR-764）→ W421 Long host-bridge thin **已交**（ADR-765）→ W422 Long host-bridge deepen thin **已交**（ADR-766）→ W423 Long host-bridge Print wire first-slice **已交**（ADR-767）→ W424 Long host-bridge CallHost wire first-slice **已交**（ADR-768）→ W425 Long host-bridge pure-native gap close **已交**（ADR-769）→ W426 Long sidecar-omit thin **已交**（ADR-770）→ W427 Long sidecar-omit deepen thin **已交**（ADR-771）→ W428 Long sidecar-omit wire first-slice（Empty-pool）**已交**（ADR-772）→ W429 Long sidecar-omit Register-only wire first-slice **已交**（ADR-773）→ W430 Long sidecar-omit C stub align wire first-slice **已交**（ADR-774）→ W431 Long sidecar-omit emit first-slice **已交**（ADR-775）→ W432 Long MIR-free first-slice **已交**（ADR-776）→ W433 Long named HostFn stub CALL_HOST clear first-slice **已交**（ADR-777）→ W434 Long delete_syx_mir_blob_enabled first-slice **已交**（ADR-778）→ W447 B-05 Medium `[x]` exit charter **已交**（ADR-791）→ W448+ — B-06 `[x]` exit charter；或无-idle 长驻 API；或语言 exports→`aot_export_roots`；TLS 再施工 **out of scope**（≠ this window） unless chosen as next charter.
