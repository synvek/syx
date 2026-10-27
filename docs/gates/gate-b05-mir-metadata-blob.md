# B-05 MIR / Metadata Blob (Wave B W301 thin + W309–W323 honesty/layout/partition/strip/wire/omit/table/dual-blob shape/encode-probe/decode-probe + W325 codec readiness + W327 AOT-path / embed-contract + W329 dual-blob cutover readiness + W333 Medium horizon cutover readiness / codec×dual-blob rollup)

> **Status:** **B-05 thin floor signed** (ADR-645 / W301) + **W309 encode/load honesty deepen signed** (ADR-653 / `MirModuleBlobStats`) + **W310 Medium layout inventory deepen signed** (ADR-654 / `MirModuleLayoutInventory`) + **W311 Medium codec partition inventory deepen signed** (ADR-655 / `MirModuleCodecPartitionInventory`) + **W312 Medium debug-metadata strip inventory deepen signed** (ADR-656 / `MirModuleDebugStripInventory`) + **W316 production debug-strip wire signed** (ADR-660 / `module_with_debug_locals_stripped`) + **W317 Medium native-instruction-omit inventory signed** (ADR-661 / `MirModuleNativeOmitInventory`) + **W318 production native-instruction-omit wire signed** (ADR-662 / `module_with_native_instructions_omitted` / AOT compact omit+strip) + **W319 Medium metadata-table inventory signed** (ADR-663 / `MirModuleMetadataTableInventory`) + **W321 Medium dual-blob shape inventory signed** (ADR-665 / `MirModuleDualBlobShapeInventory`) + **W322 Medium dual-blob encode-design / sidecar probe inventory signed** (ADR-666 / `MirModuleDualBlobEncodeProbe`) + **W323 Medium dual-blob decode-design / sidecar round-trip probe inventory signed** (ADR-667 / `MirModuleDualBlobDecodeProbe`) + **W325 Medium dual-blob codec readiness / load-contract inventory signed** (ADR-669 / `MirModuleDualBlobCodecReadinessInventory`) + **W327 Medium dual-blob AOT-path / embed-contract inventory signed** (ADR-671 / `MirModuleDualBlobAotEmbedInventory`) + **W329 Medium dual-blob cutover readiness / dual-load-contract inventory signed** (ADR-673 / `MirModuleDualBlobCutoverReadinessInventory`) + **W333 Medium horizon cutover readiness / codec×dual-blob rollup inventory signed** (ADR-677 / `MirModuleMediumHorizonCutoverReadinessInventory`).  
> **W447 cross-index:** live roadmap B-05 is `[x]` via narrow exit ([`gate-b05-medium-exit.md`](../../plan/archive/gates/gate-b05-medium-exit.md)（F-10 W586 archived） / ADR-791); **this axis window** still ≠ dual-blob GA / ≠ B-06 `[x]`.
> **W340 cross-index:** Wave B structural exit charter signed ([`wave-b-exit.md`](../lang/wave-b-exit.md) / ADR-684); Progress may claim **Wave B 退出条件已满足**; B-05 was `[~]` at W340 (historical; live `[x]` via ADR-791).
> **This window:** **Not** execution-MIR vs metadata-table split. **Not** real codec split. **Not** dual-blob GA full.
> **W339 cross-index:** B-05 Medium 诚实延后签收 signed ([`wave-b-exit-triad-b05-defer.md`](../lang/wave-b-exit-triad-b05-defer.md) / ADR-683); exit triad row 2 met via defer; triad 3/3 defer met.
> **W338 cross-index:** B-04 Short 诚实延后签收 signed ([`wave-b-exit-triad-b04-defer.md`](../lang/wave-b-exit-triad-b04-defer.md) / ADR-682); exit triad row 1 defer met.
> **W337 cross-index:** B-06 Missing 清零诚实延后签收 signed ([`wave-b-exit-triad-b06-defer.md`](../lang/wave-b-exit-triad-b06-defer.md) / ADR-681); exit triad row 3 defer met.
> **W336 cross-index:** Wave B structural exit readiness signed ([`wave-b-exit-readiness.md`](../lang/wave-b-exit-readiness.md) / ADR-680); axes remain `[~]`.
> **W335 cross-index:** Wave B tri-axis horizon structural readiness signed ([`wave-b-horizon-readiness.md`](../lang/wave-b-horizon-readiness.md) / ADR-679); this axis remains `[~]`.


## Scope

| Cluster | Status | Evidence (existing nails) |
|---------|--------|---------------------------|
| **mir-aot Medium goal** | **in scope** | [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md) — split execution MIR vs metadata (string/type/fn tables); omit instruction streams for 100% native fns |
| **Whole-module encode** | **in scope** | `crates/syx-mir/src/lib.rs` — `encode_module` / `decode_module` (full `MirModule` bincode) |
| **Whole-module load** | **in scope** | `crates/syx-rt/src/abi.rs` — `syx_rt_load_mir_blob` → `decode_module` → install functions |
| **Split honesty gap** | **documented** | Blob remains **one** full MIR module — Medium split **not** delivered this window |
| **Encode/load honesty** | **in scope (W309)** | `mir_module_blob_stats` / `MirModuleBlobStats` — fn_count + encoded_bytes; round-trip preserves names |
| **Medium layout inventory** | **in scope (W310)** | `mir_module_layout_inventory` / `MirModuleLayoutInventory` — fn_count + instruction_count + encoded_bytes; monotonic relational tests |
| **Medium codec partition inventory** | **in scope (W311)** | `mir_module_codec_partition_inventory` / `MirModuleCodecPartitionInventory` — execution instructions vs debug metadata slots; encode unchanged |
| **Medium debug-metadata strip inventory** | **in scope (W312)** | `aot_codec_fixture_main_with_debug_locals` + `mir_module_debug_strip_inventory` / `MirModuleDebugStripInventory` — synthetic clear of `local_names`/`local_types` byte gap |
| **Production debug-strip wire** | **in scope (W316)** | `module_with_debug_locals_stripped` + AOT compact encode strip; generic `encode_module` unchanged |
| **Medium native-instruction-omit inventory** | **in scope (W317)** | `mir_module_native_instruction_omit_inventory` / `MirModuleNativeOmitInventory` — synthetic stub of `llvm_emit_function_names` eligible instruction streams |
| **Production native-instruction-omit wire** | **in scope (W318)** | `module_with_native_instructions_omitted` + `encode_module_compact_for_aot` / syx-llvm `aot_compact_view` — AOT compact blob omits eligible instruction streams then strips debug locals; generic `encode_module` unchanged |
| **Medium metadata-table inventory** | **in scope (W319)** | `aot_codec_fixture_main_with_const_strs` + `mir_module_metadata_table_inventory` / `MirModuleMetadataTableInventory` — conceptual string/fn/type table gap; encode unchanged |
| **Medium dual-blob shape inventory** | **in scope (W321)** | `mir_module_dual_blob_shape_inventory` / `MirModuleDualBlobShapeInventory` — conceptual metadata vs execution payload attribution on whole-module bincode; encode unchanged |
| **Medium dual-blob encode-design / sidecar probe inventory** | **in scope (W322)** | `mir_module_metadata_sidecar_view` + `mir_module_dual_blob_encode_probe` / `MirModuleDualBlobEncodeProbe` — inventory-only sidecar bincode + framing overhead; encode unchanged |
| **Medium dual-blob decode-design / sidecar round-trip probe inventory** | **in scope (W323)** | `mir_module_dual_blob_decode_probe` / `MirModuleDualBlobDecodeProbe` — inventory-only sidecar bincode deserialize ≡ view; encode unchanged |
| **Medium dual-blob codec readiness / load-contract inventory** | **in scope (W325)** | `mir_module_dual_blob_codec_readiness_inventory` / `MirModuleDualBlobCodecReadinessInventory` — design-probe chain + whole-module load honesty; encode/load unchanged |
| **Medium dual-blob AOT-path / embed-contract inventory** | **in scope (W327)** | `mir_module_dual_blob_aot_embed_inventory` / `MirModuleDualBlobAotEmbedInventory` — production AOT vs whole vs sidecar probe; AOT still single whole-module blob |
| **Medium dual-blob cutover readiness / dual-load-contract inventory** | **in scope (W329)** | `mir_module_dual_blob_cutover_readiness_inventory` / `MirModuleDualBlobCutoverReadinessInventory` — design+AOT-embed ready measurable; dual-blob cutover still off |
| **Medium horizon cutover readiness / codec×dual-blob rollup inventory** | **in scope (W333)** | `mir_module_medium_horizon_cutover_readiness_inventory` / `MirModuleMediumHorizonCutoverReadinessInventory` — codec readiness ∧ dual-blob cutover rollup; Medium cutover still off |

## Thin-open definition

B-05 **thin floor** = this document indexes the Medium horizon goal and pre-existing **whole-module** encode/load path. W301 added **policy + meta** only (~~`gate_b05_metadata_blob_thin.rs`~~ F-10 W587 retired); it does **not** split codec layers or claim Medium fully delivered.

**W309–W318** delivered honesty / layout / partition / strip inventory / AOT strip+omit wire floors. **Still Not** real codec split / **Still Not** B-05 `[x]`.

**W318 production native-instruction-omit wire deepen** wires omit onto the **AOT compact encode** path (`encode_module_compact_for_aot` = compact → omit → strip → encode; `aot_compact_view` blob bytes). Index/emit views keep full instructions. Generic `encode_module` / VM MIR paths do **not** auto-omit. Blob **still** whole-module bincode (stub bodies + empty debug slots). **Still Not** real codec split / **Still Not** B-05 `[x]`.

**W319 Medium metadata-table inventory deepen** measures conceptual Medium string/fn/type table bytes (`ConstStr`, fn names, debug local slots) against whole-module encode size. Does **not** introduce a dual blob or change encode format. **Still Not** real codec split / **Still Not** B-05 `[x]`.

**W321 Medium dual-blob shape inventory deepen** measures conceptual Medium dual-blob shape: metadata payload bytes (fn names + `ConstStr` + debug local strings) vs execution residual attribution on whole-module bincode (`execution_payload_bytes` = whole − metadata). Does **not** write a second blob or change encode format. **Still Not** real codec / **Still Not** dual-blob production / **Still Not** B-05 `[x]`.

**Evidence nails:** `MirModuleMediumHorizon*Inventory` / syx-mir encode-load path + roadmap B-05 `[x]` (ADR-791) + syx-mir/llvm stats/layout/partition/strip/omit/wire/table/dual-blob-shape/encode-probe/decode-probe/codec-readiness tests（F-10 W587：`gate_b05_metadata_blob_thin` retired；proof = this SSOT + Inventories）。

## Encode/load honesty deepen (W309 / ADR-653)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `MirModuleBlobStats` | `fn_count` / `encoded_bytes` | Codec layer split |
| `mir_module_blob_stats(module)` | `fn_count = functions.len()`; `encoded_bytes = encode_module(m).len()` | Absolute byte threshold / B-05 `[x]` |
| syx-mir tests | non-empty round-trip preserves fn names/count; empty-module stats self-consistent | |

Honesty: whole-module encode/decode inventory + round-trip. Stats ≠ Medium metadata/execution split.

## Medium layout inventory deepen (W310 / ADR-654)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `MirModuleLayoutInventory` | `fn_count` / `instruction_count` / `encoded_bytes` | Codec layer split |
| `mir_module_layout_inventory(module)` | sum of instruction streams + encode size | Absolute thresholds / B-05 `[x]` |
| syx-mir tests | non-empty / empty / orphan monotonic | |

Honesty: measurable whole-module instruction-stream / layout gap inventory.

## Medium codec partition inventory deepen (W311 / ADR-655)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `MirModuleCodecPartitionInventory` | execution vs metadata slot counts + encode size | Real codec / new format |
| `mir_module_codec_partition_inventory(module)` | conceptual partition counts | Absolute thresholds / B-05 `[x]` |
| syx-mir tests | non-empty / empty / monotonic; encode unchanged by inventory | |

Honesty: measurable metadata-slot vs execution-instruction gap inventory.

## Medium debug-metadata strip inventory deepen (W312 / ADR-656)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `aot_codec_fixture_main_with_debug_locals` | fixture with debug locals | Real codec |
| `MirModuleDebugStripInventory` / `mir_module_debug_strip_inventory` | synthetic clear byte gap | Absolute thresholds / B-05 `[x]` |
| syx-mir tests | debug saved>0; orphan/empty saved==0; monotonic | |

Honesty: measurable debug-metadata byte gap inventory. (Production AOT strip = W316.)

## Production debug-strip wire deepen (W316 / ADR-660)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `module_with_debug_locals_stripped` | clone; clear only `local_names`/`local_types` | Real codec |
| AOT compact path | strip after compact (W318: after omit) | Generic `encode_module` auto-strip |
| syx-mir / syx-llvm tests | decode empty locals; plain encode keeps locals | Absolute thresholds / B-05 `[x]` |

Honesty: AOT production compact path clears debug locals. ≠ real codec; ≠ B-05 `[x]`.

## Medium native-instruction-omit inventory deepen (W317 / ADR-661)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `MirModuleNativeOmitInventory` | encode size gap + eligible/omit counts | Real codec |
| `mir_module_native_instruction_omit_inventory(module)` | clone via omit helper; encode unchanged by inventory | Absolute thresholds / B-05 `[x]` |
| syx-mir tests | large eligible saved>0; orphan saved==0; CallIndirect all-eligible | |

Honesty: measurable Medium native-instruction-omit byte gap inventory. (Production AOT omit = W318.)

## Production native-instruction-omit wire deepen (W318 / ADR-662)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `module_with_native_instructions_omitted` | clone; stub `llvm_emit_function_names` eligible bodies to `Return` | Real codec / new encode format |
| `encode_module_compact_for_aot` | compact → omit → strip → `encode_module` | Generic `encode_module` auto-omit |
| syx-llvm `aot_compact_view` | blob bytes = omit+strip encode; index view stays compact (full instructions) | Absolute byte threshold / B-05 `[x]` |
| syx-mir / syx-llvm tests | compact+omit+strip `<` compact+strip-only; decode stub+empty locals; CallIndirect stubs; plain encode keeps bodies | |

Honesty: this floor is **AOT production compact+omit+strip wire** (still whole-module bincode with stub bodies). ≠ real codec split; ≠ universal encode omit; ≠ Medium full delivery; ≠ B-05 `[x]`.

## Medium metadata-table inventory deepen (W319 / ADR-663)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `aot_codec_fixture_main_with_const_strs` | fixture with N distinct `ConstStr` | Real string-table codec |
| `MirModuleMetadataTableInventory` | fn name / `ConstStr` / debug slots / execution / encode size | Dual blob / new encode format |
| `mir_module_metadata_table_inventory(module)` | conceptual table counts; encode unchanged | Absolute thresholds / B-05 `[x]` |
| syx-mir tests | empty zeros; ConstStr monotonic; encode stable; overlaps W311 partition | |

Honesty: measurable Medium metadata-table (string/fn/type) gap inventory. ≠ real codec split; ≠ dual blob; ≠ encode format change; ≠ B-05 `[x]`.


## Medium dual-blob shape inventory deepen (W321 / ADR-665)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `MirModuleDualBlobShapeInventory` | fn/string/exec counts + metadata/execution payload attribution + share bps | Real dual blob / new encode format |
| `mir_module_dual_blob_shape_inventory(module)` | conceptual shape; encode unchanged | Absolute thresholds / B-05 `[x]` |
| syx-mir tests | empty self-consistent; ConstStr monotonic; encode stable; overlaps W319 table | |

Honesty: measurable Medium dual-blob **design shape** on whole-module bincode. ≠ real codec; ≠ dual-blob production encode; ≠ B-05 `[x]`.


## Medium dual-blob encode-design / sidecar probe inventory deepen (W322 / ADR-666)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `MirModuleMetadataSidecarView` | inventory-only fn/string/local tables | Production second blob / wire format |
| `MirModuleDualBlobEncodeProbe` | sidecar encoded bytes + overhead + residual + share bps | Real codec / AOT dual-load |
| `mir_module_dual_blob_encode_probe(module)` | bincode sidecar probe; encode unchanged | Absolute thresholds / B-05 `[x]` |
| syx-mir tests | empty self-consistent; ConstStr monotonic; encode stable; overlaps W321 shape | |

Honesty: measurable Medium dual-blob **encode-design sidecar probe** (inventory-only). ≠ real codec; ≠ dual-blob production; ≠ B-05 `[x]`.


## Medium dual-blob decode-design / sidecar round-trip probe inventory deepen (W323 / ADR-667)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `MirModuleDualBlobDecodeProbe` | roundtrip_ok + view_eq + decoded counts + sidecar/whole sizes | Real codec / AOT dual-load |
| `mir_module_dual_blob_decode_probe(module)` | inventory-only serialize→deserialize; encode unchanged | Absolute thresholds / B-05 `[x]` |
| syx-mir tests | empty roundtrip; ConstStr monotonic; overlaps W322 encode / W321 shape | |

Honesty: measurable Medium dual-blob **decode-design sidecar round-trip** (inventory-only). ≠ real codec; ≠ dual-blob production; ≠ B-05 `[x]`.


## Medium dual-blob codec readiness / load-contract inventory deepen (W325 / ADR-669)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `MirModuleDualBlobCodecReadinessInventory` | four design preds + chain + honesty load-contract flags | Real codec / dual-blob production |
| `mir_module_dual_blob_codec_readiness_inventory(module)` | read-only reuse of W321–W323 probes | encode/load/AOT change |
| syx-mir tests | ConstStr four preds true; honesty flags; probe overlap | B-05 `[x]` / absolute thresholds |

Honesty: measurable Medium dual-blob **codec readiness / load-contract** inventory only. ≠ real codec; ≠ dual-blob production; ≠ encode/load/AOT change; ≠ B-05 `[x]`.

## Medium dual-blob AOT-path / embed-contract inventory deepen (W327 / ADR-671)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `MirModuleDualBlobAotEmbedInventory` | whole vs production AOT vs sidecar probe + 3 relational preds + honesty flags | Real codec / AOT sidecar embed |
| `mir_module_dual_blob_aot_embed_inventory(module)` | read-only reuse of encode/compact/probe/readiness | encode/load/AOT change |
| syx-mir tests | orphan AOT-save pred; ConstStr sidecar+readiness; honesty flags | B-05 `[x]` / dual-blob load |

Honesty: measurable Medium dual-blob **AOT-path / embed-contract** inventory only. ≠ real codec; ≠ AOT sidecar embed; ≠ dual-blob production load; ≠ B-05 `[x]`.

## Medium dual-blob cutover readiness / dual-load-contract inventory deepen (W329 / ADR-673)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `MirModuleDualBlobCutoverReadinessInventory` | three relational preds + honesty cutover/load flags | Real codec / dual-blob cutover |
| `mir_module_dual_blob_cutover_readiness_inventory(module)` | read-only reuse of readiness + AOT-embed inventories | encode/load/AOT change |
| syx-mir tests | ConstStr three preds true; honesty flags; readiness/aot_embed overlap | B-05 `[x]` / dual-blob production |

Honesty: measurable Medium dual-blob **cutover readiness / dual-load-contract** inventory only (`cutover_enabled=false`). ≠ real codec; ≠ AOT sidecar embed; ≠ dual-blob production load; ≠ B-05 `[x]`.


## Medium horizon cutover readiness / codec×dual-blob rollup inventory deepen (W333 / ADR-677)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `MirModuleMediumHorizonCutoverReadinessInventory` | codec readiness chain + dual-blob cutover chain + overlap preds + honesty flags | Real codec / dual-blob production |
| `mir_module_medium_horizon_cutover_readiness_inventory(module)` | read-only reuse of W325 + W329 inventories | encode/load/AOT change |
| syx-mir tests | ConstStr three preds true; honesty flags; axis overlap; empty-module codec false; encode unchanged | B-05 `[x]` |

Honesty: measurable Medium-horizon **cutover readiness rollup** with W404 dual-blob production wire on (`cutover_enabled=true`, `real_codec_enabled=true`, `production_dual_load_enabled=true`, `aot_embeds_sidecar=true` (W406)). AOT sidecar embed **partial met** (W406); ≠ dual-load runtime; ≠ B-05 `[x]`.


## Medium 诚实延后 / exit-triad B-05 defer inventory (W339 / ADR-683)

W339 signs the Wave B exit-triad **honest defer** arm for B-05 Medium production:

- `MirModuleMediumHorizonHonestDeferInventory` / `mir_module_medium_horizon_honest_defer_inventory`
- Reuses Medium-horizon cutover readiness floors
- Honesty: `honest_defer_signed=true`; `cutover_enabled=true` (W405); `real_codec_enabled=true` (W403+); `production_dual_load_enabled=true` (W404); `aot_embeds_sidecar=true` (W406)

≠ real codec; ≠ dual-blob production; ≠ B-05 `[x]`. Charter signed via [`wave-b-exit.md`](../lang/wave-b-exit.md). See [`wave-b-exit-triad-b05-defer.md`](../lang/wave-b-exit-triad-b05-defer.md).

## Frozen

- Claiming B-06 `[x]` / dual-blob GA full (live B-05 `[x]` via ADR-791 / W447 — [`gate-b05-medium-exit.md`](../../plan/archive/gates/gate-b05-medium-exit.md)（F-10 W586 archived）; this window still ≠ dual-blob GA)
- Real codec split implementation / new encode format / dual blob
- AOT metadata-sidecar embed / dual-blob production load / dual-blob cutover
- Generic `encode_module` / non-AOT paths automatic omit or debug strip
- B-04 `[x]` / size golden CI / absolute byte threshold table
- A-06 / Q-S2 `[x]`; `timeout_tls` / `tls.client` native=partial→real
- B-06 / B-07 `[x]`; capabilities native batch rewrite

Honesty: W325 Medium dual-blob codec readiness / load-contract = **design-probe chain ready + production still whole-module single-load**; ≠ dual-blob production; ≠ Medium full delivery; ≠ B-05 `[x]`. W327 AOT-path / embed-contract = **AOT vs whole vs sidecar measurable**; ≠ AOT sidecar embed; ≠ B-05 `[x]`. W329 cutover readiness / dual-load-contract = **design+AOT-embed ready measurable + cutover still off**; ≠ dual-blob cutover; ≠ B-05 `[x]`. W333 Medium-horizon rollup = **codec×dual-blob cutover chains ready measurable + Medium cutover still off**; ≠ dual-blob production; ≠ B-05 `[x]`. W401 production reopen thin = **gap inventory signed**; ≠ MIR-free; ≠ B-05 `[x]`. W402 cutover deepen thin = **deepen ladder signed**; ≠ dual-blob production; ≠ Medium cutover flip; ≠ MIR-free; ≠ B-05 `[x]`. W403 real codec first-slice = **sidecar encode/decode wire + `real_codec_enabled=true`**; ≠ dual-blob production embed/load; ≠ Medium cutover; ≠ MIR-free; ≠ B-05 `[x]`.

**Next:** W414 Long closure-async MIR-free thin **已交**（ADR-758）→ W415 Long closure-async MIR-free deepen thin **已交**（ADR-759）→ W416 Long closure-async wire first-slice（CallFn）**已交**（ADR-760）→ W417 Long closure-async Await wire first-slice **已交**（ADR-761）→ W418 Long closure-async Task wire first-slice **已交**（ADR-762）→ W419 Long Actor wire first-slice **已交**（ADR-763）→ W420 Long gap #3 close **已交**（ADR-764）→ W421 Long host-bridge thin **已交**（ADR-765）→ W422 Long host-bridge deepen thin **已交**（ADR-766）→ W423 Long host-bridge Print wire first-slice **已交**（ADR-767）→ W424 Long host-bridge CallHost wire first-slice **已交**（ADR-768）→ W425 Long host-bridge pure-native gap close **已交**（ADR-769）→ W426 Long sidecar-omit thin **已交**（ADR-770）→ W427 Long sidecar-omit deepen thin **已交**（ADR-771）→ W428 Long sidecar-omit wire first-slice（Empty-pool）**已交**（ADR-772）→ W429 Long sidecar-omit Register-only wire first-slice **已交**（ADR-773）→ W430 Long sidecar-omit C stub align wire first-slice **已交**（ADR-774）→ W431 Long sidecar-omit emit first-slice **已交**（ADR-775）→ W432 Long MIR-free first-slice **已交**（ADR-776）→ W433 Long named HostFn stub CALL_HOST clear first-slice **已交**（ADR-777）→ W434 Long delete_syx_mir_blob_enabled first-slice **已交**（ADR-778）→ W447 B-05 Medium `[x]` exit charter **已交**（ADR-791）→ W448+ — B-06 `[x]` exit charter；或无-idle 长驻 API；或语言 exports→`aot_export_roots`；TLS 再施工 **out of scope**（≠ this window） unless chosen as next charter.
