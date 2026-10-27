# B-04 Short absolute-threshold first-slice (W439 / ADR-783)

## Status

**B-04 Short absolute-threshold first-slice met** (fixed 3-row `production_bytes` ceiling table; `absolute_threshold_count=3`; empty `aot_export_roots` **still main-only**; W440 default multi-root **met**; **live B-04 `[x]`** via W446 / ADR-790 narrow exit; ≠ language exports / ≠ encode change).

Fingerprint: `two-hundred-eighty-ninth` / `A-W439` (threshold charter); live default multi-root: `A-W440`.

## Index

| Prior | Status |
|-------|--------|
| W399 Size golden CI relational gate | **partial met** |
| W400 production cutover honesty | **partial met** (`cutover_enabled=true`) |
| W398 multi-root production wire | **partial met** (`production_multi_root_enabled=true`) |
| This window absolute thresholds | **met** (`absolute_threshold_count=3`) |
| Default emit auto multi-root | **met** (W440 / ADR-784; module `aot_export_roots`) |
| B-04 axis `[x]` | **met** (W446 / ADR-790 narrow exit) |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Table | orphan(64) ≤ 86; orphan(128) ≤ 86; CallIndirect ≤ 165 (`production_bytes`; ceil(measured×1.10)) |
| Count | `absolute_threshold_count = 3` on Short inventory chain |
| Inventory | `AotShortHorizonAbsoluteThresholdInventory` / `absolute_threshold_signed=true` |
| Empty roots | **still main-only** (`relational_default_main_only_ok` via cleared clone) |
| Forbidden claim | Language exports auto-discovery / B-04 `[x]` without ADR-790 exit / encode algorithm change / MIR blob delete / TLS HostFn raise |

## Evidence

- [`crates/syx-mir/src/lib.rs`](../../crates/syx-mir/src/lib.rs) `AOT_ABS_THRESH_*` + `aot_short_horizon_absolute_threshold_inventory`
- Cross: [`gate-b04-default-multi-root.md`](gate-b04-default-multi-root.md), [`gate-b04-size-ci-gate.md`](gate-b04-size-ci-gate.md), [`gate-b04-production-cutover.md`](gate-b04-production-cutover.md), [`gate-b04-multi-root-wire.md`](gate-b04-multi-root-wire.md)

## Frozen

- Language-level `Module.exports` → MIR automatic roots
- Claiming B-05/B-06 `[x]` / claiming B-04 `[x]` without ADR-790 exit context
- Changing compact/encode algorithms (ceilings only)
- Deleting native MIR blob / MIR-free emit (already gone; ≠ this window)
- TLS `TlsServe`→`implemented` (delivered later W441) / unbounded multi-conn (delivered later W442)
- Adding HostFn (keep 554)

## Next

W446 B-04 Short `[x]` exit charter **已交**（ADR-790）→ W447+ — B-05 `[x]` exit charter；或无-idle 长驻 API；或语言 exports→`aot_export_roots`；**≠** language exports auto-discovery / **≠** B-05/B-06 `[x]` / **≠** MIR blob delete this window unless chosen as next charter.
