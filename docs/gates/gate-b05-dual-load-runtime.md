# B-05 Medium dual-load runtime first-slice (W407 / ADR-751)

## Status

**B-05 Medium dual-load runtime first-slice partial met** (`syx_rt_load_mir_blob` peels SYXD execution **and** installs sidecar into RT string pool).

Fingerprint: `two-hundred-fifty-seventh` / `A-W407`.

> **W447 cross-index:** live roadmap B-05 is `[x]` via narrow exit ([`gate-b05-medium-exit.md`](../../plan/archive/gates/gate-b05-medium-exit.md)（F-10 W586 archived） / ADR-791); **this window** still ≠ dual-blob GA / ≠ B-06 `[x]`.

## Index

| Prior | Status |
|-------|--------|
| W333 Medium cutover readiness | **met**; Medium chain cutover/dual/codec on |
| W401–W406 reopen / deepen / codec / dual-wire / cutover / sidecar embed | **met / partial met** |
| AOT sidecar embed | **partial met** (`aot_embeds_sidecar=true`; `@syx_mir_blob`=SYXD package) |
| Dual-load runtime | **partial met** (`dual_load_runtime_enabled=true`; `load_path_is_whole_module=false`) |
| Default AOT `@syx_mir_blob` | **still embedded** (SYXD package; constant name retained) |
| MIR-free / delete `@syx_mir_blob` | **out of scope** (Long; ≠ this window) |

## Dual-load contract (hard)

| Item | Meaning |
|------|---------|
| Peel | `peel_dual_blob_package_execution` + `peel_dual_blob_package_sidecar` |
| Load | SYXD → decode execution → `load_mir_module` → `install_metadata_sidecar_view` (string_literals → string pool) |
| Legacy | Non-SYXD blobs still whole-module `decode_module` only |
| Inventory | `dual_load_runtime_signed=true`; `dual_load_runtime_enabled=true`; `load_path_is_whole_module=false` |
| Honesty | Execution still decoded; sidecar **also** installed into RT; **≠** MIR-free / **≠** blob delete |

## Inventory

- `MirModuleMediumHorizonDualLoadRuntimeInventory` / `mir_module_medium_horizon_dual_load_runtime_inventory`
- Flags: `dual_load_runtime_signed=true`; `dual_load_runtime_enabled=true`; `aot_embeds_sidecar=true`; cutover/dual/codec retained; `load_path_is_whole_module=false`
- Predicates: sidecar embed signed + bilateral peel + sidecar decode + inventory overlap

Cross: [`gate-b05-aot-sidecar-embed.md`](gate-b05-aot-sidecar-embed.md), [`gate-b05-production-cutover.md`](gate-b05-production-cutover.md), [`gate-b05-dual-blob-production-wire.md`](gate-b05-dual-blob-production-wire.md), [`gate-b05-real-codec-first-slice.md`](gate-b05-real-codec-first-slice.md), [`gate-b05-production-cutover-deepen.md`](gate-b05-production-cutover-deepen.md), [`gate-b05-production-reopen.md`](gate-b05-production-reopen.md), [`gate-b05-mir-metadata-blob.md`](gate-b05-mir-metadata-blob.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md).

## Frozen

- Deleting native MIR blob / MIR-free emit — **out of scope** (Long; ≠ this window)
- Omitting entire execution segment from default AOT embed (string-table strip is W408)
- Claiming B-06 `[x]` (live B-05 `[x]` via ADR-791 / W447 — [`gate-b05-medium-exit.md`](../../plan/archive/gates/gate-b05-medium-exit.md)（F-10 W586 archived）; ≠ this window)
- TLS `status=implemented` / handler deepen
- Short default emit auto multi-root; absolute byte thresholds

## Next

W414 Long closure-async MIR-free thin **已交**（ADR-758）→ W415 Long closure-async MIR-free deepen thin **已交**（ADR-759）→ W416 Long closure-async wire first-slice（CallFn）**已交**（ADR-760）→ W417 Long closure-async Await wire first-slice **已交**（ADR-761）→ W418 Long closure-async Task wire first-slice **已交**（ADR-762）→ W419 Long Actor wire first-slice **已交**（ADR-763）→ W420 Long gap #3 close **已交**（ADR-764）→ W421 Long host-bridge thin **已交**（ADR-765）→ W422 Long host-bridge deepen thin **已交**（ADR-766）→ W423 Long host-bridge Print wire first-slice **已交**（ADR-767）→ W424 Long host-bridge CallHost wire first-slice **已交**（ADR-768）→ W425 Long host-bridge pure-native gap close **已交**（ADR-769）→ W426 Long sidecar-omit thin **已交**（ADR-770）→ W427 Long sidecar-omit deepen thin **已交**（ADR-771）→ W428 Long sidecar-omit wire first-slice（Empty-pool）**已交**（ADR-772）→ W429 Long sidecar-omit Register-only wire first-slice **已交**（ADR-773）→ W430 Long sidecar-omit C stub align wire first-slice **已交**（ADR-774）→ W431 Long sidecar-omit emit first-slice **已交**（ADR-775）→ W432 Long MIR-free first-slice **已交**（ADR-776）→ W433 Long named HostFn stub CALL_HOST clear first-slice **已交**（ADR-777）→ W434 Long delete_syx_mir_blob_enabled first-slice **已交**（ADR-778）→ W447 B-05 Medium `[x]` exit charter **已交**（ADR-791）→ W448+ — B-06 `[x]` exit charter；或无-idle 长驻 API；或语言 exports→`aot_export_roots`；TLS 再施工 **out of scope**（≠ this window） unless chosen as next charter.
