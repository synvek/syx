# B-04 MIR Tree-Shake (Wave B W300 thin + W302–W308 + W320 layered size baseline + W324 size golden candidate + W326 export-root reachability + W328 export-root cutover readiness + W330 size golden CI cutover readiness + W332 Short horizon cutover readiness / export×size-CI rollup inventory deepen)

> **Status:** **B-04 thin floor signed** (ADR-644 / W300) + **W302 analysis-only deepen signed** (ADR-646 / `reachable_fn_names`) + **W303 emit integration deepen signed** (ADR-647 / `llvm_emit_function_names`) + **W304 blob body-strip deepen signed** (ADR-648 / `shaken_module_for_aot_blob` / `encode_module_for_aot`) + **W305 fn-delete analysis deepen signed** (ADR-649 / `compact_module_for_aot_blob` / remap helpers) + **W306 AOT compact wire deepen signed** (ADR-650 / production compact encode + index alignment) + **W307 size inventory floor deepen signed** (ADR-651 / `AotBlobSizeReport` / `aot_blob_size_report`) + **W308 size fixture inventory deepen signed** (ADR-652 / `aot_size_fixture_*`) + **W320 layered size baseline inventory deepen signed** (ADR-664 / `AotBlobSizeBaselineInventory`) + **W324 Size golden candidate inventory signed** (ADR-668 / `AotBlobSizeGoldenCandidateInventory`) + **W326 Export-root reachability inventory signed** (ADR-670 / `AotReachabilityExportRootInventory`) + **W328 Export-root cutover readiness inventory signed** (ADR-672 / `AotReachabilityExportRootCutoverReadinessInventory`) + **W330 Size golden CI cutover readiness / CI-gate alignment inventory signed** (ADR-674 / `AotBlobSizeGoldenCiCutoverReadinessInventory`) + **W332 Short horizon cutover readiness / export×size-CI rollup inventory signed** (ADR-676 / `AotShortHorizonCutoverReadinessInventory`).  
> **W446 cross-index:** B-04 Short `[x]` exit charter signed ([`gate-b04-short-exit.md`](../../plan/archive/gates/gate-b04-short-exit.md)（F-10 W588 archived） / ADR-790); roadmap B-04 is **`[x]`** (narrow construction chain); **≠** language exports auto-discovery / **≠** B-05/B-06 `[x]`.
> **W340 cross-index:** Wave B structural exit charter signed ([`wave-b-exit.md`](../lang/wave-b-exit.md) / ADR-684); Progress may claim **Wave B 退出条件已满足**; B-04 axis closed separately in W446.
> **W339 cross-index:** B-05 Medium 诚实延后签收 signed ([`wave-b-exit-triad-b05-defer.md`](../lang/wave-b-exit-triad-b05-defer.md) / ADR-683); exit triad 3/3 defer met; **Not** Wave B exit met (W340+).
> **W338 cross-index:** B-04 Short 诚实延后签收 signed ([`wave-b-exit-triad-b04-defer.md`](../lang/wave-b-exit-triad-b04-defer.md) / ADR-682); exit triad row 1 met via defer; B-05 triad still unmet; **Not** Wave B exit met.
> **W337 cross-index:** B-06 Missing 清零诚实延后签收 signed ([`wave-b-exit-triad-b06-defer.md`](../lang/wave-b-exit-triad-b06-defer.md) / ADR-681); exit triad row 3 defer met; B-04/B-05 triad still unmet; **Not** Wave B exit met.
> **W336 cross-index:** Wave B structural exit readiness signed ([`wave-b-exit-readiness.md`](../lang/wave-b-exit-readiness.md) / ADR-680); axes remain `[~]`.
> **W335 cross-index:** Wave B tri-axis horizon structural readiness signed ([`wave-b-horizon-readiness.md`](../lang/wave-b-horizon-readiness.md) / ADR-679); this axis remains `[~]`.


## Scope

| Cluster | Status | Evidence (existing nails) |
|---------|--------|---------------------------|
| **mir-aot Short goal** | **in scope** | [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md) — reachability from `main`+exports before emit; drop unreachable stdlib leaves from LLVM **and** blob |
| **Intra-fn DCE** | **in scope** | `crates/syx-mir/src/lib.rs` — `dead_code_elim` / `remove_unreachable_blocks` (called from `optimize`) |
| **Cross-fn reachability analysis** | **in scope (W302)** | `reachable_fn_names` BFS from `main` via direct named call edges |
| **LLVM emit integration** | **in scope (W303)** | `llvm_emit_function_names` → `emit_llvm_ir` + `syx_rt_register_native_fn` aligned filter |
| **Blob body-strip integration** | **in scope (W304)** | `shaken_module_for_aot_blob` + `encode_module_for_aot` (stub inventory path) |
| **Fn-delete analysis** | **in scope (W305)** | `compact_module_for_aot_blob` + `build_fn_index_remap` + `remap_fn_indices_in_module` |
| **AOT compact wire** | **in scope (W306)** | `append_aot_runtime_shim` + `compile_native_c_stub` use compact encode; register/main/emit indices align |
| **Size inventory floor** | **in scope (W307)** | `aot_blob_size_report` / `AotBlobSizeReport` — stub vs **production** compact+omit+strip (compat; see W320 layered) |
| **Size fixture inventory** | **in scope (W308)** | `aot_size_fixture_main_with_orphan_body` / `aot_size_fixture_call_indirect_full_emit` — reproducible synthetic baselines |
| **Layered size baseline inventory** | **in scope (W320)** | `encode_module_compact_only_for_inventory` + `aot_blob_size_baseline_inventory` / `AotBlobSizeBaselineInventory` — stub / compact-only / production layers |
| **Size golden candidate inventory** | **in scope (W324)** | `aot_blob_size_golden_candidate_inventory` / `AotBlobSizeGoldenCandidateInventory` — relational CI-hook predicates; `ci_gate_enabled=false` |
| **Export-root reachability inventory** | **in scope (W326)** | `aot_reachability_export_root_inventory` / `AotReachabilityExportRootInventory` — main-only vs main∪export roots gap; production still main-only |
| **Export-root cutover readiness inventory** | **in scope (W328)** | `aot_reachability_export_root_cutover_readiness_inventory` / `AotReachabilityExportRootCutoverReadinessInventory` — gap measurable + emit≡main-only; cutover still off |
| **Size golden CI cutover readiness / CI-gate alignment inventory** | **in scope (W330)** | `aot_blob_size_golden_ci_cutover_readiness_inventory` / `AotBlobSizeGoldenCiCutoverReadinessInventory` — candidate chain ready measurable; CI cutover still off |
| **Short horizon cutover readiness / export×size-CI rollup inventory** | **in scope (W332)** | `aot_short_horizon_cutover_readiness_inventory` / `AotShortHorizonCutoverReadinessInventory` — export cutover ∧ size-CI cutover rollup; Short cutover still off |

## Thin-open definition

B-04 **thin floor** = W300 policy/meta indexing Short goal + intra-fn DCE.

**W302–W305** delivered analysis → emit filter → stub blob → fn-delete helpers.

**W306 AOT compact wire deepen** switches production AOT encode to compact and aligns `register_native_fn` / `syx_rt_run_main_fn` / emit `fn_name_to_index` with compact indices. **Does not** claim measurable size golden CI or B-04 `[x]`.

**W307 size inventory floor deepen** exposes a public stub vs compact blob size report and synthetic measurable tests. **Still Not** size golden CI / **Still Not** B-04 `[x]` / **Still Not** native linked-binary golden.

**W308 size fixture inventory deepen** exposes public reproducible MIR fixtures for `aot_blob_size_report` with monotonic/relational assertions. **Still Not** size golden CI / **Still Not** absolute byte threshold table / **Still Not** B-04 `[x]`.

**W320 layered size baseline inventory deepen** splits stub / compact-only (fn-delete) / production (compact→omit→strip) MIR-blob byte layers so omit+strip savings are distinguishable from fn-delete. Compat `aot_blob_size_report.compact_bytes` remains the **production** path. **Still Not** size golden CI / **Still Not** absolute thresholds / **Still Not** B-04 `[x]`.

**W324 Size golden candidate inventory deepen** aggregates W308 fixtures + W320 layered baseline into three relational predicates a future size-golden CI *would* assert (`ci_gate_enabled=false`, `absolute_threshold_count=0`). **Still Not** size golden CI gate / **Still Not** absolute thresholds / **Still Not** B-04 `[x]`.

**W326 Export-root reachability inventory deepen** measures inventory-only main-only vs main∪extra-export-roots reachable-count gap (`production_roots_main_only=true`, `emit_uses_export_roots=false`). **Still Not** production multi-root cutover / **Still Not** size golden CI / **Still Not** B-04 `[x]`.

**W328 Export-root cutover readiness inventory deepen** aggregates W326 gap with emit≡main-only alignment predicates (`cutover_enabled=false`). **Still Not** production multi-root cutover / **Still Not** size golden CI / **Still Not** B-04 `[x]`.

**Evidence nails:** `AotShortHorizon*Inventory` / syx-mir tree-shake path + roadmap B-04 `[x]` (W446) + syx-mir/syx-llvm tests（F-10 W588–W589：exit + ~~`gate_b04_tree_shake_thin.rs`~~ retired；proof = this SSOT + Inventories）。

## Analysis-only deepen (W302 / ADR-646)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `reachable_fn_names(module)` | BFS from `main`; direct named edges only | Emit filtering (→ W303) |
| syx-mir tests | main→callee reachable; orphan excluded | Size golden |

## Emit integration deepen (W303 / ADR-647)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `llvm_emit_function_names(module)` | Default = reachable; CallIndirect/CallWitness/AwaitSuspend → full emit | Blob filter (→ W304) |
| `emit_llvm_ir` | Skips orphan LLVM `define` bodies | Size golden |
| `syx_user_init` | Skips `register_native_fn` for non-emitted fns | B-04 `[x]` |
| syx-llvm tests | orphan absent from IR; CallIndirect fallback keeps orphan | Blob differential |

## Blob body-strip deepen (W304 / ADR-648)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `shaken_module_for_aot_blob(module)` | Ineligible fn → minimal `Return` stub; eligible fn unchanged | Fn vec delete |
| `encode_module_for_aot(module)` | bincode after shake; fn count/order unchanged | Size golden |
| AOT paths (pre-W306) | Used stub encode | Superseded by W306 wire for production |
| syx-mir tests | orphan stubbed; fn.len unchanged; CallIndirect fallback; round-trip decode | Fn delete differential |

Stub encode remains available for inventory comparison against compact.

## Fn-delete analysis deepen (W305 / ADR-649)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `build_fn_index_remap` | Eligible fns kept in order; old→new remap (`FN_INDEX_DELETED` for dropped) | AOT production wire (→ W306) |
| `remap_fn_indices_in_module` | Updates `CallIndirect` / `CallWitness` / `AwaitSuspend.resume_fn`; `MakeClosure` unchanged | — |
| `compact_module_for_aot_blob` | Deletes ineligible entries + remap; full-emit fallback → clone | Size golden CI |
| `encode_module_compact_for_aot` | Compact bincode helper | Was inventory-only until W306; now production omit+strip (W316/W318) |
| syx-mir tests | orphan deleted; CallIndirect remap; fallback; round-trip; stub vs compact inventory | B-04 `[x]` |

## AOT compact wire deepen (W306 / ADR-650)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `append_aot_runtime_shim` | Compact blob embed; `register_native_fn` uses compact indices | Size golden CI |
| `compile_native_c_stub` | Compact blob + compact `main_idx` | B-04 `[x]` |
| `emit_llvm_ir` | `fn_name_to_index` from compact module | — |
| syx-llvm tests | orphan register at compact idx; CallIndirect full register; compact < stub inventory | CI golden gate |

Production AOT no longer calls stub `encode_module_for_aot`. Stub helper remains for inventory / honesty comparison.

## Size inventory floor deepen (W307 / ADR-651)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `AotBlobSizeReport` | `stub_bytes` / `compact_bytes` / `saved_bytes` | CI size golden thresholds |
| `aot_blob_size_report(module)` | stub = `encode_module_for_aot`; compact = **production** `encode_module_compact_for_aot` (omit+strip); saved = saturating_sub | Native linked-binary size golden |
| syx-mir tests | orphan body → `saved_bytes > 0`; CallIndirect layered compact-only saved==0 | Absolute byte threshold table / B-04 `[x]` |

This floor is a **synthetic MIR-blob byte-diff** report for policy/meta citation. It is **not** a CI golden gate and **not** a claim about post-link native binary size. W320 layered inventory separates fn-delete vs omit+strip.

## Size fixture inventory deepen (W308 / ADR-652)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `aot_size_fixture_main_with_orphan_body(n)` | Reproducible `main` + `n` ConstI32-bearing orphan fns | Absolute byte threshold table |
| `aot_size_fixture_call_indirect_full_emit()` | CallIndirect full-emit fallback fixture | CI size golden gate |
| syx-mir tests | `saved(128) > saved(64) > 0`; CallIndirect compact-only `saved == 0` | Native binary golden / B-04 `[x]` |

Honesty: stub body-strip collapses a single orphan's instruction stream to `Return`, so fixture **scale is orphan fn count** (each carrying one `ConstI32`), not fat single-body length. Assertions are **monotonic/relational** only.

## Layered size baseline inventory deepen (W320 / ADR-664)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `encode_module_compact_only_for_inventory` | fn-delete compact encode **without** omit/strip | Production path change |
| `AotBlobSizeBaselineInventory` | stub / compact_only / production + three saved layers | Absolute thresholds / CI gate |
| `aot_blob_size_baseline_inventory(module)` | layered saturating diffs | B-04 `[x]` / linked binary golden |
| syx-mir tests | orphan compact>0 + production≤compact_only + monotonic; CallIndirect compact==0; fat+debug omit_strip>0 | |

Honesty: measurable **layered** MIR-blob size baseline (stub vs compact-only vs production omit+strip). ≠ size golden CI; ≠ absolute byte threshold table; ≠ B-04 `[x]`.


## Size golden candidate inventory deepen (W324 / ADR-668)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `AotBlobSizeGoldenCandidateInventory` | orphan/CallIndirect/fat+debug saved layers + 3 relational preds + honesty flags | CI gate / absolute thresholds |
| `aot_blob_size_golden_candidate_inventory()` | read-only reuse of fixtures + baseline inventory | Production encode change / CI config |
| syx-mir tests | three preds true; gate off; baseline overlap | B-04 `[x]` / linked binary golden |

Honesty: measurable **candidate** relational CI-hook inventory only. ≠ size golden CI gate; ≠ absolute byte threshold table; ≠ production encode change; ≠ B-04 `[x]`.

## Export-root reachability inventory deepen (W326 / ADR-670)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `AotReachabilityExportRootInventory` | main-only vs +export counts + 2 relational preds + honesty flags | Production multi-root cutover |
| `aot_reachability_export_root_inventory(module, extra)` | inventory-only BFS from `main` ∪ extra; does not change `reachable_fn_names` | Emit / compact / size golden CI |
| `aot_reachability_fixture_main_and_export_orphan` | `main`→`callee_a`; independent `export_lib`→`callee_b` | Production export-root wire |
| syx-mir tests | two preds true; honesty flags; overlap with `reachable_fn_names`; empty extra gap 0 | B-04 `[x]` |

Honesty: W326 Export-root reachability inventory = **measurable main vs +export gap with production still main-only**. Inventory signed; **production wire Frozen**. ≠ production multi-root cutover; ≠ size golden CI; ≠ B-04 `[x]`.

## Export-root cutover readiness inventory deepen (W328 / ADR-672)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `AotReachabilityExportRootCutoverReadinessInventory` | export gap + emit≡main-only + inventory overlap + 3 preds + honesty flags | Production multi-root cutover |
| `aot_reachability_export_root_cutover_readiness_inventory(module, extra)` | read-only reuse of export-root inventory + `llvm_emit_function_names` | Emit / compact / size golden CI |
| syx-mir tests | three preds true; honesty flags; overlap; empty extra gap 0 | B-04 `[x]` |

Honesty: W328 Export-root cutover readiness = **gap measurable + emit still main-only + cutover off**. ≠ production multi-root cutover; ≠ size golden CI; ≠ B-04 `[x]`.

## Size golden CI cutover readiness / CI-gate alignment inventory deepen (W330 / ADR-674)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `AotBlobSizeGoldenCiCutoverReadinessInventory` | three relational preds + honesty CI/threshold/cutover flags | True size golden CI / absolute thresholds |
| `aot_blob_size_golden_ci_cutover_readiness_inventory()` | read-only reuse of golden candidate inventory | encode/compact change / CI config |
| syx-mir tests | three preds true; honesty flags; candidate overlap; encode unchanged | B-04 `[x]` |

Honesty: measurable size golden CI **cutover readiness / CI-gate alignment** inventory only (`cutover_enabled=false`). ≠ true CI gate; ≠ absolute thresholds; ≠ production multi-root; ≠ B-04 `[x]`.

## Short horizon cutover readiness / export×size-CI rollup inventory deepen (W332 / ADR-676)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `AotShortHorizonCutoverReadinessInventory` | export cutover chain + size-CI cutover chain + overlap preds + honesty flags | Production multi-root / true size golden CI |
| `aot_short_horizon_cutover_readiness_inventory(module, extra)` | read-only reuse of W328 + W330 cutover inventories | encode/emit/compact change / CI config |
| syx-mir tests | three preds true; honesty flags; axis overlap; empty-export gap false; encode unchanged | B-04 `[x]` |

Honesty: measurable Short-horizon **cutover readiness rollup** only (`cutover_enabled=false`, `production_multi_root_enabled=false`, `size_ci_gate_enabled=false`). ≠ production multi-root; ≠ true CI gate; ≠ absolute thresholds; ≠ B-04 `[x]`.


## Short 诚实延后签收 deepen (W338 / ADR-682)

W338 signs the Wave B exit-triad **honest defer** arm for B-04 Short production:

- `AotShortHorizonHonestDeferInventory` / `aot_short_horizon_honest_defer_inventory`
- Reuses Short-horizon cutover readiness floors
- Honesty: `honest_defer_signed=true`; `cutover_enabled=false`; `production_multi_root_enabled=false`; `size_ci_gate_enabled=false`

≠ production multi-root; ≠ true size CI; ≠ B-04 `[x]`; ≠ Wave B exit met. See [`wave-b-exit-triad-b04-defer.md`](../lang/wave-b-exit-triad-b04-defer.md).

## Short production reopen thin-open (W396 / ADR-740)

W396 signs the **production reopen thin-open inventory** (reverse-bound of W338 defer):

- `AotShortHorizonProductionReopenThinInventory` / `aot_short_horizon_production_reopen_thin_inventory`
- `production_reopen_thin_signed=true`; gaps `multi_root_reopen_gap_open` / `size_ci_reopen_gap_open` still **open**
- Honesty: `cutover_enabled=false`; `production_multi_root_enabled=false`; `size_ci_gate_enabled=false`; W338 `honest_defer_signed=true` retained

≠ production cutover flip; ≠ B-04 `[x]`; ≠ MIR-free / delete `@syx_mir_blob`. See [`gate-b04-production-reopen.md`](gate-b04-production-reopen.md).

## Short cutover deepen thin-open (W397 / ADR-741)

W397 signs the **cutover deepen thin-open inventory** (ordered ladder after W396 reopen thin):

- `AotShortHorizonCutoverDeepenThinInventory` / `aot_short_horizon_cutover_deepen_thin_inventory`
- `cutover_deepen_thin_signed=true`; gaps `multi_root_reopen_gap_open` / `size_ci_reopen_gap_open` still **open**
- Honesty: three flags still false; `emit_uses_export_roots=false`; W396 reopen + W338 defer retained

≠ multi-root production wire; ≠ size-CI gate; ≠ `cutover_enabled` flip; ≠ B-04 `[x]`; ≠ MIR-free. See [`gate-b04-production-cutover-deepen.md`](gate-b04-production-cutover-deepen.md).

## Short multi-root production wire deepen (W398 / ADR-742)

W398 wires explicit export roots into emit/compact eligibility:

- `reachable_fn_names_from_roots` / `llvm_emit_function_names_with_roots` / shaken+compact `_with_roots`
- Default empty extras remain **main-only**
- `production_multi_root_enabled=true`; `multi_root_reopen_gap_open=false`; size-CI closed in W399; cutover honesty on in W400 (`cutover_enabled=true`; default emit still main-only); W398 retained `cutover_enabled=false`

≠ default multi-root behavior; ≠ B-04 `[x]`; ≠ MIR-free. See [`gate-b04-multi-root-wire.md`](gate-b04-multi-root-wire.md).

## Short size golden CI gate deepen (W399 / ADR-743)

W399 flips the relational size golden CI gate:

- `ci_gate_enabled=true` when W324 three relational preds hold; `size_ci_gate_enabled=true`
- `size_ci_reopen_gap_open=false`; `absolute_threshold_count=0`; W399 retained `cutover_enabled=false`
- `AotShortHorizonSizeCiGateInventory` / `size_ci_gate_signed=true`

≠ absolute thresholds; ≠ B-04 `[x]`; ≠ MIR-free. See [`gate-b04-size-ci-gate.md`](gate-b04-size-ci-gate.md).

## Short production cutover honesty flip (W400 / ADR-744)

W400 flips Short production cutover honesty:

- Short inventory chain `cutover_enabled=true`; `production_cutover_signed=true`
- Default `llvm_emit_function_names` / compact empty extras **still main-only**
- `AotShortHorizonProductionCutoverInventory` / [`gate-b04-production-cutover.md`](gate-b04-production-cutover.md)

≠ default emit multi-root behavior; ≠ absolute thresholds; ≠ B-04 `[x]`; ≠ MIR-free / delete `@syx_mir_blob` (Long; ≠ this window).


## Frozen

- Claiming B-04 `[x]` without ADR-790 exit context / claiming B-05/B-06 `[x]`
- Language semantic `Module.exports` → MIR automatic roots discovery (W440 default multi-root is module-field only)
- B-05 execution-MIR vs metadata blob split
- A-06 / Q-S2 `[x]`; `timeout_tls` / `tls.client` native=partial→real
- B-06 / B-07 `[x]`; capabilities native batch rewrite

Honesty: W324–W399 relational size/wire gates + W400 cutover honesty; W439 absolute thresholds; W440 default multi-root (`aot_export_roots`); W446 narrow exit; empty roots still main-only; ≠ language exports discovery; ≠ MIR-free; B-04 **`[x]`** only via ADR-790.

**Next:** W446 B-04 Short `[x]` exit charter **已交**（ADR-790）→ W447+ — B-05 `[x]` exit charter；或无-idle 长驻 API；或语言 exports→`aot_export_roots`；**≠** language exports auto-discovery / **≠** B-05/B-06 `[x]` / **≠** MIR blob delete this window unless chosen as next charter.
