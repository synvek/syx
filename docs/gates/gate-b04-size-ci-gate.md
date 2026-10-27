# B-04 Short size golden CI gate deepen (W399 / ADR-743)

## Status

**B-04 Short size golden CI gate partial met** (relational three-predicate gate; absolute thresholds **met** in W439). Live: **default emit multi-root first-slice met** (W440 / ADR-784); **live B-04 `[x]`** via W446 / ADR-790 narrow exit.

Fingerprint: `two-hundred-forty-ninth` / `A-W399` (size-CI charter); live default multi-root: `A-W440`.

## Index

| Prior | Status |
|-------|--------|
| W324 Size golden candidate | **met**; `ci_gate_enabled=true` when three relational preds hold |
| W330 Size golden CI cutover readiness | **met**; threshold honesty = `absolute_threshold_count==0` |
| W332 Short cutover readiness | **met**; `size_ci_gate_enabled=true` |
| W338 Short honest defer | **met**; defer history signed; cutover honesty on after W400 |
| W396–W398 reopen / deepen / multi-root wire | **met / partial met**; `size_ci_reopen_gap_open=false` |
| Size golden CI relational gate | **partial met** (`size_ci_gate_enabled=true`) |
| Absolute byte threshold table | **met** (W439 / ADR-783；`absolute_threshold_count=3`) |
| `cutover_enabled` | **partial met** (W400 / ADR-744; `true`) — W399 signing retained `cutover_enabled=false` |
| Default emit auto multi-root | **met** (W440 / ADR-784) — see [`gate-b04-default-multi-root.md`](gate-b04-default-multi-root.md) |

## Gate predicates (hard)

| Predicate | Meaning |
|-----------|---------|
| orphan monotonic | orphan(128) compact save > orphan(64) > 0 |
| CallIndirect zero | CallIndirect fixture compact save == 0 |
| omit+strip positive | fat+debug omit+strip save > 0 |

When all three hold → `ci_gate_enabled=true` / `size_ci_gate_enabled=true`. Inventory + syx-mir tests fail if any predicate fails while the gate is signed.

## Inventory

- `AotShortHorizonSizeCiGateInventory` / `aot_short_horizon_size_ci_gate_inventory`
- Flags: `size_ci_gate_signed=true`; `size_ci_gate_enabled=true`; `ci_gate_enabled=true`; `size_ci_reopen_gap_open=false`; `absolute_threshold_count=3` (W439); `cutover_enabled=true` (W400); multi-root wire remains on

Cross: [`gate-b04-default-multi-root.md`](gate-b04-default-multi-root.md), [`gate-b04-absolute-thresholds.md`](gate-b04-absolute-thresholds.md), [`gate-b04-production-cutover.md`](gate-b04-production-cutover.md), [`gate-b04-multi-root-wire.md`](gate-b04-multi-root-wire.md), [`gate-b04-production-cutover-deepen.md`](gate-b04-production-cutover-deepen.md), [`gate-b04-production-reopen.md`](gate-b04-production-reopen.md), [`gate-b04-mir-tree-shake.md`](gate-b04-mir-tree-shake.md).

## Frozen

- Language semantic `Module.exports` → MIR automatic roots discovery
- Claiming B-05/B-06 `[x]` / claiming B-04 `[x]` without ADR-790 exit context
- Deleting native MIR blob / MIR-free emit — **out of scope** (Long; already gone; ≠ this window)
- TLS HostFn status raise (delivered later W441) / unbounded multi-conn (delivered later W442) (≠ this window)

## Next

W446 B-04 Short `[x]` exit charter **已交**（ADR-790）→ W447+ — B-05 `[x]` exit charter；或无-idle 长驻 API；或语言 exports→`aot_export_roots`；**≠** language exports auto-discovery / **≠** B-05/B-06 `[x]` / **≠** MIR blob delete this window unless chosen as next charter.