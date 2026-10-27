# B-04 Short multi-root production wire deepen (W398 / ADR-742)

## Status

**B-04 Short multi-root production wire partial met** (W398 `_with_roots` wire). Live: **default emit multi-root first-slice met** (W440 / ADR-784; `MirModule.aot_export_roots`; empty roots **still main-only**); **live B-04 `[x]`** via W446 / ADR-790 narrow exit.

Fingerprint: `two-hundred-forty-eighth` / `A-W398` (wire charter); live default multi-root: `A-W440`.

## Index

| Prior | Status |
|-------|--------|
| W332 Short cutover readiness | **met** (export chain uses wired emit≡roots) |
| W338 Short honest defer | **met** (defers cutover; wire/size-CI may be on) |
| W396 production reopen thin | **met**; `multi_root_reopen_gap_open=false` |
| W397 cutover deepen thin | **met**; wire gap closed |
| Multi-root production wire | **partial met** (`production_multi_root_enabled=true`, `emit_uses_export_roots=true` when extras provided) |
| Default emit (no extras / empty `aot_export_roots`) | **still main-only** (compat); non-empty roots → default multi-root **met** (W440) |
| Size-CI gate | **partial met** (W399) — see [`gate-b04-size-ci-gate.md`](gate-b04-size-ci-gate.md) |
| `cutover_enabled` | **partial met** (W400; `true`) — W398 signing retained `cutover_enabled=false` |
| Production cutover honesty | **partial met** — see [`gate-b04-production-cutover.md`](gate-b04-production-cutover.md) |
| Default emit auto multi-root | **met** (W440 / ADR-784) — see [`gate-b04-default-multi-root.md`](gate-b04-default-multi-root.md) |

## Wire API

| API | Role |
|-----|------|
| `reachable_fn_names_from_roots` | BFS from `main` ∪ explicit extras |
| `llvm_emit_function_names_with_roots` | Emit eligibility with extras |
| `shaken_module_for_aot_blob_with_roots` / `compact_module_for_aot_blob_with_roots` | Blob eligibility align |
| Default `llvm_emit_function_names` / compact / shaken | Empty `aot_export_roots` ≡ main-only; non-empty → module roots (W440) |

Honesty: empty roots ≠ fake cutover; semantic `Module.exports` auto-discovery **out of scope** (W398–W439 historical default was empty-extras main-only; W440 module-field cutover).

## Inventory

- `AotShortHorizonMultiRootWireInventory` / `aot_short_horizon_multi_root_wire_inventory`
- Flags: `multi_root_wire_signed=true`; `production_multi_root_enabled=true`; `multi_root_reopen_gap_open=false`; `cutover_enabled=true` (W400; W398 retained `cutover_enabled=false`); `size_ci_gate_enabled=true` (W399); `size_ci_reopen_gap_open=false` (W399)

Cross: [`gate-b04-default-multi-root.md`](gate-b04-default-multi-root.md), [`gate-b04-production-cutover.md`](gate-b04-production-cutover.md), [`gate-b04-size-ci-gate.md`](gate-b04-size-ci-gate.md), [`gate-b04-production-cutover-deepen.md`](gate-b04-production-cutover-deepen.md), [`gate-b04-production-reopen.md`](gate-b04-production-reopen.md), [`gate-b04-mir-tree-shake.md`](gate-b04-mir-tree-shake.md).

## Frozen

- Language semantic `Module.exports` → MIR automatic roots discovery
- Claiming B-05/B-06 `[x]` / claiming B-04 `[x]` without ADR-790 exit context
- Deleting native MIR blob / MIR-free emit — **out of scope** (Long; already gone; ≠ this window)
- TLS HostFn status raise (delivered later W441) / unbounded multi-conn (delivered later W442) (≠ this window)

## Next

W446 B-04 Short `[x]` exit charter **已交**（ADR-790）→ W447+ — B-05 `[x]` exit charter；或无-idle 长驻 API；或语言 exports→`aot_export_roots`；**≠** language exports auto-discovery / **≠** B-05/B-06 `[x]` / **≠** MIR blob delete this window unless chosen as next charter.
