# B-06 LLVM Stub Audit (Wave B W299 thin + W313 report + W314 align + W315 declare-diff + W331 Missing residual / clear-readiness + W334 Missing clear×align horizon readiness + W337 Missing 清零诚实延后)

> **Status:** **B-06 thin floor signed** (ADR-643 / W299) + **W313 audit report deepen signed** (ADR-657) + **W314 align inventory deepen signed** (ADR-658) + **W315 declare-diff inventory deepen signed** (ADR-659 / `LlvmStubDeclareDiffInventory` / `llvm_stub_declare_diff_inventory`) + **W331 Missing residual / clear-readiness inventory signed** (ADR-675 / `LlvmStubMissingClearReadinessInventory` / `llvm_stub_missing_clear_readiness_inventory`) + **W334 Missing clear×align horizon readiness inventory signed** (ADR-678 / `LlvmStubClearAlignHorizonReadinessInventory` / `llvm_stub_clear_align_horizon_readiness_inventory`) + **W337 Missing 清零诚实延后签收 signed** (ADR-681 / `LlvmStubMissingClearHonestDeferInventory` / `llvm_stub_missing_clear_honest_defer_inventory`).  
> **W340 cross-index:** Wave B structural exit charter signed ([`wave-b-exit.md`](../lang/wave-b-exit.md) / ADR-684); Progress may claim **Wave B 退出条件已满足**; live B-06 is `[x]` via ADR-792 narrow exit.
> Stub classification, HostFn-llvm cross counts, emit-declare↔stubs diff, Missing clear-readiness, clear×align horizon readiness, and Missing clear honest defer are public syx-rt APIs (no stub semantic change).  
> **W339 cross-index:** B-05 Medium 诚实延后签收 signed ([`wave-b-exit-triad-b05-defer.md`](../lang/wave-b-exit-triad-b05-defer.md) / ADR-683); exit triad 3/3 defer met; **Not** Wave B exit met (W340+).
> **W338 cross-index:** B-04 Short 诚实延后签收 signed ([`wave-b-exit-triad-b04-defer.md`](../lang/wave-b-exit-triad-b04-defer.md) / ADR-682); exit triad row 1 defer met; B-05 triad still unmet; **Not** Wave B exit met.
> **W337 cross-index:** B-06 Missing 清零诚实延后签收 signed ([`wave-b-exit-triad-b06-defer.md`](../lang/wave-b-exit-triad-b06-defer.md) / ADR-681); exit triad row 3 met via defer; B-04/B-05 triad still unmet; **Not** Wave B exit met.  
> **W448 cross-index:** B-06 Missing/align `[x]` exit signed ([`gate-b06-missing-align-exit.md`](../../plan/archive/gates/gate-b06-missing-align-exit.md) / ADR-792；F-10 W584 archived)；**live roadmap B-06 is `[x]`** via narrow exit + Inventories. Historical construction chain (W299–W337) still ≠ Missing 清零 complete / ≠ stub rewrite / ≠ absolute align / ≠ B-07 `[x]`. Live B-07 is `[~]` via ADR-793 / W449 thin + ADR-794 / W450 deepen + ADR-795 / W451 HttpGet wire + ADR-796 / W452 FileRead wire ([`gate-b07-real-hostfn-lowering.md`](gate-b07-real-hostfn-lowering.md) / [`gate-b07-real-hostfn-lowering-deepen.md`](gate-b07-real-hostfn-lowering-deepen.md) / [`gate-b07-real-hostfn-lowering-wire.md`](gate-b07-real-hostfn-lowering-wire.md) / [`gate-b07-real-hostfn-lowering-file-read-wire.md`](gate-b07-real-hostfn-lowering-file-read-wire.md); ≠ batch rewrite / ≠ `[x]`).  
> **W336 cross-index:** Wave B structural exit readiness signed ([`wave-b-exit-readiness.md`](../lang/wave-b-exit-readiness.md) / ADR-680); axes remain `[~]`.
> **W335 cross-index:** Wave B tri-axis horizon structural readiness signed ([`wave-b-horizon-readiness.md`](../lang/wave-b-horizon-readiness.md) / ADR-679); this axis remains `[~]`.

> **Live roadmap B-06 is `[x]`** via W448/ADR-792 narrow exit. **Not** Missing 清零 complete. **Not** stub rewrite. **Not** absolute HostFn↔stub matrix exit. **Not** B-07 `[x]` / global HostFn lowering / batch stub rewrite. Live B-07 is `[~]` via ADR-793 / W449 thin + ADR-794 / W450 deepen + ADR-795 / W451 HttpGet wire + ADR-796 / W452 FileRead wire ([`gate-b07-real-hostfn-lowering.md`](gate-b07-real-hostfn-lowering.md) / [`gate-b07-real-hostfn-lowering-deepen.md`](gate-b07-real-hostfn-lowering-deepen.md) / [`gate-b07-real-hostfn-lowering-wire.md`](gate-b07-real-hostfn-lowering-wire.md) / [`gate-b07-real-hostfn-lowering-file-read-wire.md`](gate-b07-real-hostfn-lowering-file-read-wire.md); ≠ batch rewrite / ≠ `[x]`).

## Scope

| Cluster | Status | Evidence (existing nails) |
|---------|--------|---------------------------|
| **Wired vs WiredNoop** | **in scope** | `classify_llvm_stub_block` — `rt_direct_exec` + `op::POP` → WiredNoop (≠ Wired-GA) |
| **Count floors** | **in scope** | `llvm_stub_audit_report` — `total≥80`, `wired_noop≥1`, `wired+wired_noop≥20`, `cold≥1`, `noop≤5` |
| **POP≠GA honesty** | **in scope** | syx-rt + driver — POP sample → WiredNoop; non-POP `rt_direct_exec` → Wired |
| **Audit report API** | **in scope (W313)** | `LlvmStubAuditReport` / `llvm_stub_audit_report` |
| **Align inventory API** | **in scope (W314)** | `LlvmStubCapabilityAlignInventory` / `llvm_stub_capability_align_inventory` |
| **Declare-diff inventory API** | **in scope (W315)** | `LlvmStubDeclareDiffInventory` / `llvm_stub_declare_diff_inventory` — emit `declare @syx_rt_*` × `llvm_stubs` × abi cover |
| **Missing residual / clear-readiness inventory** | **in scope (W331)** | `LlvmStubMissingClearReadinessInventory` / `llvm_stub_missing_clear_readiness_inventory` — declare gap measurable + unlinkable=0; clear still off |
| **Missing clear×align horizon readiness inventory** | **in scope (W334)** | `LlvmStubClearAlignHorizonReadinessInventory` / `llvm_stub_clear_align_horizon_readiness_inventory` — clear chain ∧ align chain rollup; clear/rewrite/absolute exit still off |
| **Missing 清零诚实延后签收 inventory** | **in scope (W337)** | `LlvmStubMissingClearHonestDeferInventory` / `llvm_stub_missing_clear_honest_defer_inventory` — honest_defer_signed=true; clear still off |

## Thin-open definition

B-06 **thin floor** = this document indexes the clusters above to **pre-existing** audit nails (M0-A07 lineage). W299 added **policy + meta** only; it did **not** rewrite `llvm_stubs.rs`, expand real HostFn lowering (B-07), or claim capability-table full alignment exit.

**Evidence nails:** syx-rt `LlvmStub*` Inventories / `crates/syx-driver/tests/native/llvm_stub_audit.rs` + roadmap B-06 `[x]`（F-10 W584–W585：exit + thin meta retired；proof = this SSOT + Inventories）。

## Audit report deepen (W313 / ADR-657)

W313 elevates the former driver-private classifier into a **public, citeable** report:

- `LlvmStubAuditReport { total, wired, wired_noop, noop, unknown, cold }`
- `llvm_stub_audit_report()` — classifies `include_str!(llvm_stubs.rs)` with the same rules as W299/M0-A07
- Relational floors + bucket-sum identity + POP≠Wired samples (syx-rt unit + driver)

Honesty: W313 audit report = **measurable llvm_stubs classification count floor**; report ≠ capability-table full alignment exit; ≠ stub semantic rewrite; ≠ B-07; ≠ B-06 `[x]`.

## Align inventory deepen (W314 / ADR-658)

W314 adds a **cross inventory** against [`packages/internal/host_fn_capabilities.toml`](../../packages/internal/host_fn_capabilities.toml) `backends.llvm` (not product `capabilities.toml`, not qs1 behavior matrix):

- Stub side aligned with `llvm_stub_audit_report()`
- `hostfn_llvm_real` / `partial` / `other` counts
- Named `*_aot` Wired vs WiredNoop; `HostFnId::` bound stubs; `call_host`/`call_host_aot` Wired path

Honesty: W314 align inventory = **measurable stub 桶 × HostFn-llvm 计数 × named-AOT/HostFnId 交叉诚实钉**; **no** 1:1 HostFn↔`syx_rt_*` map; `hostfn_llvm_real` ≠ `stub_wired` (different categories); ≠ absolute matrix exit; ≠ stub rewrite; ≠ B-07; ≠ B-06 `[x]`.

## Declare-diff inventory deepen (W315 / ADR-659)

W315 adds an **emit-declare ↔ stubs** diff inventory:

- `declared_count` from `emit_llvm_ir` `push_str("declare … @syx_rt_*")` literals in [`syx-llvm`](../../crates/syx-llvm/src/lib.rs)
- `defined_count` from `llvm_stubs.rs` (aligned with `llvm_stub_audit_report().total`)
- `missing_in_stubs` / `extra_in_stubs` set differences
- `abi_cover_count` / `missing_not_in_rt` — honesty that most “Missing” live in [`abi.rs`](../../crates/syx-rt/src/abi.rs) (incl. stack-op macros), **not** unlinkable holes

Honesty: W315 declare-diff = **可计量 declare×stubs 差集 + abi 覆盖诚实钉**；`missing_in_stubs` ≠ unlinkable；≠ Missing 清零 exit；≠ stub rewrite；≠ B-07；≠ B-06 `[x]`。

## Missing residual / clear-readiness inventory deepen (W331 / ADR-675)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `LlvmStubMissingClearReadinessInventory` | three relational preds + honesty clear/rewrite flags + declare-diff fields | Missing 清零 / stub rewrite |
| `llvm_stub_missing_clear_readiness_inventory()` | read-only reuse of declare-diff inventory | stubs/abi/emit change |
| syx-rt tests | three preds true; honesty flags; declare-diff overlap | B-06 `[x]` / B-07 |

Honesty: measurable Missing **residual / clear-readiness** inventory only (`clear_enabled=false`). ≠ Missing 清零 exit; ≠ stub rewrite; ≠ B-07; ≠ B-06 `[x]`.


## Missing clear×align horizon readiness inventory deepen (W334 / ADR-678)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `LlvmStubClearAlignHorizonReadinessInventory` | clear chain + align chain + overlap preds + honesty flags | Missing 清零 / stub rewrite / absolute align exit |
| `llvm_stub_clear_align_horizon_readiness_inventory()` | read-only reuse of W331 clear-readiness + W314 align | stubs/abi/emit change |
| syx-rt tests | three preds true; honesty flags; clear/align axis overlap | B-06 `[x]` / B-07 |

Honesty: measurable Missing **clear×align horizon readiness** inventory only (`clear_enabled=false`, `stub_rewrite_enabled=false`, `absolute_align_matrix_exit_enabled=false`). ≠ Missing 清零; ≠ stub rewrite; ≠ absolute HostFn↔stub matrix exit; ≠ B-07; ≠ B-06 `[x]`.


## Missing 清零诚实延后签收 deepen (W337 / ADR-681)

W337 signs the Wave B exit-triad **honest defer** arm for B-06:

- `LlvmStubMissingClearHonestDeferInventory` / `llvm_stub_missing_clear_honest_defer_inventory`
- Reuses clear×align horizon + clear-readiness residual floors
- Honesty: `honest_defer_signed=true`; `clear_enabled=false`; `stub_rewrite_enabled=false`

≠ Missing 清零完成; ≠ stub rewrite; ≠ B-06 `[x]`; ≠ B-07; ≠ Wave B exit met. See [`wave-b-exit-triad-b06-defer.md`](../lang/wave-b-exit-triad-b06-defer.md).

## Frozen

- Claiming B-06 `[x]` without ADR-792 exit narrow (live B-06 `[x]` via ADR-792 / W448 — [`gate-b06-missing-align-exit.md`](../../plan/archive/gates/gate-b06-missing-align-exit.md)；F-10 W584 archived；≠ Missing 清零 / stub rewrite / absolute align / B-07 `[x]`)
- Claiming B-07 `[x]` / global HostFn lowering expansion / batch stub rewrite (live HttpGet + FileRead wires via ADR-795 / W451 + ADR-796 / W452 — [`gate-b07-real-hostfn-lowering-wire.md`](gate-b07-real-hostfn-lowering-wire.md) / [`gate-b07-real-hostfn-lowering-file-read-wire.md`](gate-b07-real-hostfn-lowering-file-read-wire.md); ≠ batch rewrite / ≠ `[x]`) / Missing 清零 complete / true HostFn MIR→llvm differential or golden expansion
- Semantic rewrite of `crates/syx-rt/src/llvm_stubs.rs` (POP or dispatch)
- Absolute HostFn↔stub alignment matrix exit (inventory alone does not claim it)
- Missing 清零 exit（declare−stubs 清零 / abi 合并叙事属后续评估；clear-readiness signed W331；clear×align horizon signed W334 with `clear_enabled=false`）
- No-idle TLS daemon / language `Module.exports`→`aot_export_roots` / MIR blob delete this window
- A-06 / Q-S2 `[x]`; `timeout_tls` / `tls.client` native=partial→real
- Capabilities native batch rewrite; Suite-2/3/4 wasm skip full clear

Honesty: W315 declare-diff = **差集可计量 + unlinkable=0**；W331 clear-readiness = **差集就绪可计量 + 清零仍关**；W334 clear×align horizon = **clear×align 链就绪可计量 + 清零/rewrite/绝对矩阵 exit 仍关**；≠ Missing 清零；≠ stub rewrite；≠ B-06 `[x]`。

**Next:** W448 B-06 Missing/align `[x]` exit **已交**（ADR-792）→ W449 B-07 thin-open **已交**（ADR-793）→ W450 B-07 deepen thin **已交**（ADR-794）→ W451 B-07 HttpGet first-slice wire **已交**（ADR-795；≠ `[x]`）→ W452 B-07 FileRead second-slice wire **已交**（ADR-796）→ W453 B-07 TcpRead third-slice wire **已交**（ADR-797）→ W454 B-07 TcpWrite fourth-slice wire **已交**（ADR-798）→ W455 B-07 TcpHandleRelease fifth-slice wire **已交**（ADR-799）→ W456 B-07 `[x]` exit charter **已交**（ADR-800；窄口径）→ W457 TLS no-idle first-slice **已交**（ADR-801）→ W458 TLS language noIdle first-slice **已交**（ADR-802）→ W459 MIR residual honesty first-slice **已交**（ADR-803）→ W460 language exports thin-open **已交**（ADR-804）→ W461 B-03 FrontendPipeline entry matrix **已交**（ADR-805）→ W462 language exports syntax-spec deepen **已交**（ADR-806）→ W463 B-01 MIR-consumer thin-open **已交**（ADR-807）→ W464 B-01 MIR-consumer deepen thin **已交**（ADR-808）→ W465 B-02 dual-track thin-open **已交**（ADR-809）→ W466 B-02 dual-track deepen thin **已交**（ADR-810）→ W467 B-01 Struct MIR shell migrate **已交**（ADR-811）→ W468 B-01 FinalClass MIR shell migrate **已交**（ADR-812）→ W469 B-01 VarGlobal MIR shell migrate **已交**（ADR-813）→ W470 B-01 Extension MIR shell migrate **已交**（ADR-814）→ W471 B-01 Class MIR shell migrate **已交**（ADR-815）→ W472 B-01 Actor MIR shell migrate **已交**（ADR-816）→ W473 B-01 no-op residual honesty **已交**（ADR-817）→ W474 B-01 MIR-consumer narrow `[x]` exit **已交**（ADR-818）→ W475 B-02 retire-readiness thin **已交**（ADR-819）→ W476 B-09 GC narrative thin **已交**（ADR-820）→ W477 B-02 compile_into 首片 **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
