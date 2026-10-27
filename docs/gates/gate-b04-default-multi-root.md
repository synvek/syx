# B-04 Short default emit multi-root first-slice (W440 / ADR-784)

## Status

**B-04 Short default emit multi-root first-slice met** (`MirModule.aot_export_roots` drives default emit/compact; empty roots **still main-only**; export-orphan fixture default includes export closure; **live B-04 `[x]`** via W446 / ADR-790 narrow exit; ≠ language `Module.exports` auto-discovery).

Fingerprint: `two-hundred-ninetieth` / `A-W440`.

## Index

| Prior | Status |
|-------|--------|
| W398 multi-root production wire | **partial met** (`_with_roots` API) |
| W399 size-CI relational gate | **partial met** |
| W400 production cutover honesty | **partial met** |
| W439 absolute thresholds | **met** (`absolute_threshold_count=3`) |
| This window default emit multi-root | **met** (module-field driven) |
| Language `Module.exports` → MIR roots | **thin-open met** (W460 / ADR-804); auto-discovery **still unmet** |
| B-04 axis `[x]` | **met** (W446 / ADR-790 narrow exit) |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Field | `MirModule.aot_export_roots: Vec<SmolStr>` (`#[serde(default)]`; empty default) |
| Default emit | `llvm_emit_function_names` / `compact_module_for_aot_blob` read module roots |
| Empty roots | **still main-only** (compat) |
| Fixture | `aot_reachability_fixture_main_and_export_orphan` sets `["export_lib"]` |
| Inventory | `AotShortHorizonDefaultMultiRootInventory` / `default_multi_root_signed=true` |
| Forbidden claim | Language exports auto-discovery / B-04 `[x]` without ADR-790 exit / MIR blob delete / TLS HostFn raise |

## Evidence

- [`crates/syx-mir/src/lib.rs`](../../crates/syx-mir/src/lib.rs) `aot_export_roots` + `aot_short_horizon_default_multi_root_inventory`
- Cross: [`gate-b04-absolute-thresholds.md`](gate-b04-absolute-thresholds.md), [`gate-b04-multi-root-wire.md`](gate-b04-multi-root-wire.md), [`gate-b04-production-cutover.md`](gate-b04-production-cutover.md)

## Frozen

- Language-level `Module.exports` → MIR automatic roots discovery
- Claiming B-05/B-06 `[x]` / claiming B-04 `[x]` without ADR-790 exit context
- Deleting native MIR blob / MIR-free emit (already gone; ≠ this window)
- TLS `TlsServe`→`implemented` (delivered later in W441 / ADR-785 — historical W440 Frozen) / unbounded multi-conn (delivered later in W442 / ADR-786 — historical W440 Frozen)
- Adding HostFn (keep 554)
- Changing absolute ceiling numeric table / encode algorithms

## Next

W446 B-04 Short `[x]` exit charter **已交**（ADR-790）→ W460 language exports thin-open **已交**（ADR-804；≠ auto-discovery）→ W461 B-03 FrontendPipeline entry matrix **已交**（ADR-805）→ W462 language exports syntax-spec deepen **已交**（ADR-806）→ W463 B-01 MIR-consumer thin-open **已交**（ADR-807）→ W464 B-01 MIR-consumer deepen thin **已交**（ADR-808）→ W465 B-02 dual-track thin-open **已交**（ADR-809）→ W466 B-02 dual-track deepen thin **已交**（ADR-810）→ W467 B-01 Struct MIR shell migrate **已交**（ADR-811）→ W468 B-01 FinalClass MIR shell migrate **已交**（ADR-812）→ W469 B-01 VarGlobal MIR shell migrate **已交**（ADR-813）→ W470 B-01 Extension MIR shell migrate **已交**（ADR-814）→ W471 B-01 Class MIR shell migrate **已交**（ADR-815）→ W472 B-01 Actor MIR shell migrate **已交**（ADR-816）→ W473 B-01 no-op residual honesty **已交**（ADR-817）→ W474 B-01 MIR-consumer narrow `[x]` exit **已交**（ADR-818）→ W475 B-02 retire-readiness thin **已交**（ADR-819）→ W476 B-09 GC narrative thin **已交**（ADR-820）→ W477 B-02 compile_into 首片 **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA；**≠** language exports auto-discovery / **≠** MIR blob delete this window unless chosen as next charter.
