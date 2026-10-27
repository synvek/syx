# B-04 Short production reopen thin-open inventory (W396 / ADR-740)

## Status

**B-04 Short production reopen inventory thin met** (policy+meta only; W396). Live: **default emit multi-root first-slice met** (W440 / ADR-784); **live B-04 `[x]`** via W446 / ADR-790 narrow exit (separate charter).

Fingerprint: `two-hundred-forty-sixth` / `A-W396` (reopen charter); live default multi-root: `A-W440`.

## Index

| Prior | Status |
|-------|--------|
| W332 Short cutover readiness | **met** |
| W338 Short honest defer | **met** (`honest_defer_signed=true`) |
| Production cutover honesty | **partial met** (W400) — see [`gate-b04-production-cutover.md`](gate-b04-production-cutover.md); W396 signing retained `cutover_enabled=false` |
| Multi-root wire (W398) | **partial met** (`production_multi_root_enabled=true`; `multi_root_reopen_gap_open=false`) |
| Size-CI gate (W399) | **partial met** — see [`gate-b04-size-ci-gate.md`](gate-b04-size-ci-gate.md) |
| This window reopen thin inventory | **met** (`production_reopen_thin_signed=true`) |
| W397 cutover deepen thin | **met** — see [`gate-b04-production-cutover-deepen.md`](gate-b04-production-cutover-deepen.md) |
| Default emit auto multi-root | **met** (W440 / ADR-784) — see [`gate-b04-default-multi-root.md`](gate-b04-default-multi-root.md) |

## Gap table (reopen thin — documented unmet)

| Gap | Meaning | This window |
|-----|---------|-------------|
| Multi-root emit / `reachable_fn_names` / compact align | Production wire with export roots | **closed (W398)** (`multi_root_reopen_gap_open=false`) |
| True size golden CI gate | Relational size report as CI gate | **closed (W399)** (`size_ci_reopen_gap_open=false`) |
| Flip `cutover_enabled` | Production Short cutover honesty | **closed (W400)** — see [`gate-b04-production-cutover.md`](gate-b04-production-cutover.md) |
| B-04 `[x]` | Axis close | **met** (W446 / ADR-790; separate exit charter) |
| Delete `@syx_mir_blob` / MIR-free AOT | Long horizon | **out of scope** (≠ this window) |

## Public inventory

- `AotShortHorizonProductionReopenThinInventory`
- `aot_short_horizon_production_reopen_thin_inventory`
- Predicates: Short readiness chain + honest defer signed + cutover signed + inventory overlap
- Flags: `production_reopen_thin_signed=true`; `honest_defer_signed=true`; live `cutover_enabled=true` (W400; W396 retained `cutover_enabled=false`); `size_ci_gate_enabled=true` (W399); `size_ci_reopen_gap_open=false` (W399)

Cross: [`gate-b04-default-multi-root.md`](gate-b04-default-multi-root.md), [`gate-b04-production-cutover.md`](gate-b04-production-cutover.md), [`gate-b04-size-ci-gate.md`](gate-b04-size-ci-gate.md), [`gate-b04-multi-root-wire.md`](gate-b04-multi-root-wire.md), [`gate-b04-production-cutover-deepen.md`](gate-b04-production-cutover-deepen.md), [`gate-b04-mir-tree-shake.md`](gate-b04-mir-tree-shake.md), [`wave-b-exit-triad-b04-defer.md`](../lang/wave-b-exit-triad-b04-defer.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md).

## Frozen

- Language semantic `Module.exports` → MIR automatic roots discovery
- Claiming B-05/B-06 `[x]` / claiming B-04 `[x]` without ADR-790 exit context
- Deleting native MIR blob / MIR-free emit — **out of scope** (Long; already gone; ≠ this window)
- TLS HostFn status raise (delivered later W441) / unbounded multi-conn (delivered later W442) (≠ this window)
- Clearing W338 honest defer

## Next

W446 B-04 Short `[x]` exit charter **已交**（ADR-790）→ W447+ — B-05 `[x]` exit charter；或无-idle 长驻 API；或语言 exports→`aot_export_roots`；**≠** language exports auto-discovery / **≠** B-05/B-06 `[x]` / **≠** MIR blob delete this window unless chosen as next charter.
