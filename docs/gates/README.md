# docs/gates/ — engineering gate contracts

**Role:** Wave / ADR / inventory first-slice contracts.  
**Not** end-user language manuals (those stay under [`../lang/`](../lang/)).

Moved from `docs/lang/b0*.md` in **W560 / ADR-904** (Wave F **F-04**).  
Historical ADR Decision text may still mention `docs/lang/b0*` or pre-rename paths; live paths and `include_str` nails use **`docs/gates/`** from W560 onward.

## Naming convention

### Wave B（W562–W568 / ADR-906–912 · F-06）— **已交**

| Kind | Convention | Example |
|------|------------|---------|
| Native test module / file | `gate_bNN_<role>.rs` | `gate_b04_default_multi_root.rs` / `gate_b07_real_hostfn_lowering_wire.rs` |
| Gate markdown | `gate-bNN-<role>.md` | `gate-b06-llvm-stub-audit.md` |

F-06 rename 债已归零（含 b09 最后一轴）。

### Wave A（W569–W574 / ADR-913–918 · F-09）— **已交**

| Kind | Convention | Example |
|------|------------|---------|
| Stdlib behavior nails | product-semantic `.rs` under `tests/stdlib/` | `edge_overflow.rs` / `crypto_rfc_vectors.rs` / `datetime_deepen.rs` |
| Gate markdown | `gate-aNN-<role>.md` | `gate-a04-edge-semantics.md` … `gate-a07-crypto-vectors.md` … `gate-a08-datetime-vectors.md` |

- Prefer semantic test module names (`edge_*`, `crypto_*`, `capability_*`); map to roadmap **A-NN** via the gate markdown, not the `.rs` filename.
- **One axis (or small cluster) per window**; W569–W574 delivered **a02+a05+a04+a07+a08+a06**（含 syx-rt `GateA06*`）.
- **F-09 rename 债归零**；`docs/lang/a0*` 闸门路径不再合法（历史 ADR Decision 字面量可保留）。
- **F-10 note:** Wave A `a0*` floors/exit meta **已退役归零**；a04 edge + a06 Inventory + a07 crypto-vectors/rfc + a08 datetime-vectors/deepen 仍活（模块已语义化）。

### F-10 退役（W575–W607 / ADR-919–951）— **已收官 `[x]`**（a0* 归零；**b06/b05/b04/b07/b02/b01/b09 轴归零**；b09-exit + b09-thin 第一至第十一簇已退；纯 exit 已无残债；0× b09 thin）

纯 meta 闸门（`*_spec_signed` / exit charter md）在行为/能力表可独立证明后，按一轴一窗退役：

1. 先改活 `include_str` / Evidence 指向行为地板（differential / Inventories / 产品钉）。
2. 纯 exit / policy-index charter md → `git mv` 至 [`plan/archive/gates/`](../../plan/archive/gates/)；**行为 SSOT md（如 a04 edge / a07 crypto-vectors / a08 datetime-vectors / b06 llvm-stub-audit / b05 Medium 施工簇 / b04 Short 施工簇 / b07 HostFn 施工簇 / b02 施工簇 / b01 MIR-consumer 施工簇 / b09 GC 施工簇）不得 archive**。
3. 删除对应 `*_spec_signed` / `*_ql*_exit` / exit / thin meta 测模块。

**已退役（Wave A a0*）：**
- `gate-a02-ql2-floor.md` + `gate_a02_ql2_exit.rs` → archive；Q-L2 = Suite-1/2 differential（W575）
- `gate-a05-qs1-floor.md` + `gate_a05_qs1_exit.rs` → archive；Q-S1 = qs1 五域 + 矩阵（W576）
- `gate_a04_ql4_exit.rs` meta deleted；**`gate-a04-edge-semantics.md` + 6 行为测仍活**（W577）
- `gate-a06-qs2-exit.md` + `gate_a06_qs2_exit.rs` → archive/deleted；Q-S2 exit meta（W578）
- `gate-a06-network-negatives.md` + `gate-a06-qs2-floor.md` → archive；删 a06 thin meta；Q-S2 = 产品钉 + Inventory（W579）
- `gate-a07-qs3-exit.md` + `gate_a07_qs3_exit.rs` → archive/deleted；Q-S3 exit meta（W580）
- `gate-a07-qs3-floor.md` → archive；删 a07 thin meta；Q-S3 = crypto-vectors + crypto_rfc（W581）
- `gate-a08-qs4-exit.md` + `gate_a08_qs4_exit.rs` → archive/deleted；Q-S4 exit meta（W582）
- `gate-a08-qs4-floor.md` → archive；删 `gate_a08_qs4_thin` / `qs4_floor_thin`；Q-S4 = datetime-vectors + deepen（W583）

**已退役（b0\*）：**
- `gate-b06-missing-align-exit.md` + `gate_b06_missing_align_exit.rs` → archive/deleted（W584）
- `gate_b06_llvm_audit_thin` / `gate_b06_missing_clear_honest_defer_thin` → deleted；B-06 = llvm-stub-audit + Inventories + triad defer（W585）；**b06 轴 F-10 归零**
- `gate-b05-medium-exit.md` + `gate_b05_medium_exit.rs` → archive/deleted（W586）
- 4× `gate_b05_*_thin`（metadata_blob / production_reopen / cutover_deepen / medium_honest_defer）→ deleted；B-05 = Medium 施工 SSOT + wire + Inventories + triad defer（W587）；**b05 轴 F-10 归零**
- `gate-b04-short-exit.md` + `gate_b04_short_exit.rs` → archive/deleted（W588）
- 4× `gate_b04_*_thin`（tree_shake / production_reopen / cutover_deepen / short_honest_defer）→ deleted；B-04 = Short 施工 SSOT + 行为/wire + Inventories + triad defer（W589）；**b04 轴 F-10 归零**
- `gate-b07-real-hostfn-lowering-exit.md` + `gate_b07_real_hostfn_lowering_exit.rs` → archive/deleted（W590）
- `gate_b07_real_hostfn_lowering_thin` / `gate_b07_real_hostfn_lowering_deepen_thin` → deleted；B-07 = HostFn 施工 SSOT + 5× wire + Inventories（W591）；**b07 轴 F-10 归零**
- `gate-b02-dual-track-exit.md` + `gate_b02_dual_track_exit.rs` → archive/deleted（W592）；**保留** B-02 施工 SSOT + Inventory
- `gate_b02_dual_track_thin` / `gate_b02_dual_track_deepen_thin` / `gate_b02_retire_readiness_thin` → deleted；B-02 = 施工 SSOT + 3× 行为测 + Inventories（W593）；**b02 轴 F-10 归零**
- `gate-b01-mir-consumer-exit.md` + `gate_b01_mir_consumer_exit.rs` → archive/deleted（W594）；**保留** B-01 施工 SSOT + Inventory
- `gate_b01_mir_consumer_thin` / `gate_b01_mir_consumer_deepen_thin` → deleted；B-01 = 施工 SSOT + 7× 行为测 + Inventories（W595）；**b01 轴 F-10 归零**
- `gate-b09-axis-exit-gc-ga-product-first-slice.md` + `gate_b09_axis_exit_gc_ga_product_first_slice.rs` → archive/deleted（W596）；**保留** B-09 施工 SSOT + thin + Inventory/API；**b09 轴未归零**
- `gate_b09_gc_narrative_thin.rs` + `gate_b09_enable_readiness_thin.rs` → deleted（W597）；施工 md 留活；proof = `GateB09GcNarrativeThinInventory` / `GateB09EnableReadinessThinInventory`；**b09 轴未归零**
- `gate_b09_objectref_memory_pool_storage_honesty_thin.rs` + `gate_b09_objectref_rc_migrate_readiness_thin.rs` → deleted（W598）；施工 md 留活；proof = `GateB09ObjectRefMemoryPoolStorageHonestyThinInventory` / `GateB09ObjectRefRcMigrateReadinessThinInventory`；**b09 轴未归零**
- `gate_b09_objectref_rc_retire_readiness_thin.rs` + `gate_b09_objectref_strip_rc_readiness_thin.rs` → deleted（W599）；施工 md 留活；proof = `GateB09ObjectrefRcRetireReadinessThinInventory` / `GateB09ObjectrefStripRcReadinessThinInventory`；**b09 轴未归零**
- `gate_b09_rt_abi_objectref_wrap_retire_readiness_thin.rs` + `gate_b09_rt_abi_objectref_handle_retire_readiness_thin.rs` → deleted（W600）；施工 md 留活；proof = `GateB09RtAbiObjectrefWrapRetireReadinessThinInventory` / `GateB09RtAbiObjectrefHandleRetireReadinessThinInventory`；**b09 轴未归零**
- `gate_b09_rt_abi_objectref_retain_release_retire_readiness_thin.rs` + `gate_b09_objectref_peel_readiness_thin.rs` → deleted（W601）；施工 md 留活；proof = `GateB09RtAbiObjectrefRetainReleaseRetireReadinessThinInventory` / `GateB09ObjectrefPeelReadinessThinInventory`；**b09 轴未归零**
- `gate_b09_objectref_deep_field_graph_gc_walk_readiness_thin.rs` + `gate_b09_objectref_mirop_vm_residual_peel_readiness_thin.rs` → deleted（W602）；施工 md 留活；proof = `GateB09ObjectrefDeepFieldGraphGcWalkReadinessThinInventory` / `GateB09ObjectrefMiropVmResidualPeelReadinessThinInventory`；**b09 轴未归零**
- `gate_b09_objectref_rc_migrate_readiness_refresh_thin.rs` + `gate_b09_objectref_rc_migrate_readiness_post_mirop_peel_refresh_thin.rs` → deleted（W603）；施工 md 留活；proof = `GateB09ObjectrefRcMigrateReadinessRefreshThinInventory` / `GateB09ObjectrefRcMigrateReadinessPostMiropPeelRefreshThinInventory`；**b09 轴未归零**
- `gate_b09_objectref_rc_migrate_readiness_post_handle_delete_refresh_thin.rs` + `gate_b09_objectref_rc_migrate_readiness_post_peel_refresh_thin.rs` → deleted（W604）；施工 md 留活；proof = `GateB09ObjectrefRcMigrateReadinessPostHandleDeleteRefreshThinInventory` / `GateB09ObjectrefRcMigrateReadinessPostPeelRefreshThinInventory`；**b09 轴未归零**
- `gate_b09_objectref_dual_track_collapse_readiness_thin.rs` + `gate_b09_objectref_reentry_maybe_track_peel_readiness_thin.rs` → deleted（W605）；施工 md 留活；proof = `GateB09ObjectrefDualTrackCollapseReadinessThinInventory` / `GateB09ObjectrefReentryMaybeTrackPeelReadinessThinInventory`；**b09 轴未归零**
- `gate_b09_mutrefcell_maybe_track_peel_readiness_thin.rs` + `gate_b09_objectref_rc_migrate_readiness_post_mutrefcell_peel_refresh_thin.rs` → deleted（W606）；施工 md 留活；proof = `GateB09MutrefcellMaybeTrackPeelReadinessThinInventory` / `GateB09ObjectrefRcMigrateReadinessPostMutrefcellPeelRefreshThinInventory`；**b09 轴未归零**
- `gate_b09_objectref_rc_migrate_readiness_post_reentry_peel_refresh_thin.rs` + `gate_b09_objectref_rc_migrate_readiness_post_collapse_refresh_thin.rs` → deleted（W607）；施工 md 留活；proof = `GateB09ObjectrefRcMigrateReadinessPostReentryPeelRefreshThinInventory` / `GateB09ObjectrefRcMigrateReadinessPostCollapseRefreshThinInventory`；**0× thin**；**b09 轴 F-10 归零**；**F-10 `[x]`**

**仍活（勿本窗删）：** a04 edge + 6 行为（`edge_*.rs`）；`gate-a06-exit-readiness` + syx-rt Inventory；`gate-a07-crypto-vectors` + `crypto_rfc_vectors`；`gate-a08-datetime-vectors` + `datetime_deepen`；`gate-b06-llvm-stub-audit` + Inventories；全部活 `gate-b05-*`（施工簇）+ `wave-b-exit-triad-b05-defer`；全部活 `gate-b04-*`（施工簇）+ 5× 行为/wire + `wave-b-exit-triad-b04-defer`；全部活 `gate-b07-real-hostfn-lowering*`（施工簇）+ 5× wire；全部活 `gate-b02-*`（施工簇）+ 3× 行为测；全部活 `gate-b01-*`（施工簇）+ 7× 行为测；全部活 `gate-b09-*`（施工簇）+ 产品测（0× thin）。

## Gate lifecycle（ADR-913）

三层寿命不同——不要混清：

1. **行为地板**（actor/overflow/qs1 负例、differential、Inventories…）→ **长期保留**（可改名，不删语义）。
2. **闸门契约 / meta 钉**（`*_spec_signed`、厚 Frozen）→ 轴已 `[x]` 且行为覆盖充分后，按 **F-10** 退役 ADR 收缩（W575 起）。
3. **施工考古学**（历史窗号 / 超长 ADR OR）→ 活路径最终不应硬钉；按 **F-11** 先改测再缩附录（活测正向窗号 OR 归零；附录叙事首缩；gate fingerprint 轮转已收官；**F-11 `[x]`**）；git/archive/ADR 日志可永久留。

终点心智：活路径 ≈ 行为测 + 能力表诚实；闸门文档可空或极少；编号噪音出活树。
