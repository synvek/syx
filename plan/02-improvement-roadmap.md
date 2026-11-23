# SYX 改进路线图（收敛期 · C-*）

> **活 backlog。** 编号刀 = **C-0…**（Convergence）。  
> **状态叙事：** [`01-project-status.md`](01-project-status.md)  
> **外部审计输入：** [`syx-status-final-report.md`](syx-status-final-report.md)  
> **上期详账（只读）：** [`archive/02-improvement-roadmap-sa-r-u-ledger.md`](archive/02-improvement-roadmap-sa-r-u-ledger.md)（SA-0…R-49 + U-0…U-40）  
> **最近更新：** 2026-11-23 · **C-1d churn 叙事清扫 `[x]`**；**C-1 轴完成**；C-0 薄闸 `[x]`；默认下一 = **C-2**（C-0a+/e park）。

## 0. 时代切换

| 时代 | 编号 | 状态 | 目标 |
|------|------|------|------|
| 可信度 / Wave / Stage | Stage-0…4 | `[x]` | 证据链与治理 |
| Swift 对齐章程 | SA-0…SA-6 | `[x]` | 对齐边界与 ADR |
| Object / 平台 / 诊断 | R-0…R-49 | `[x]` | 收口；**无 R-50** |
| 语言表面薄刀 | U-0…U-40 | `[x]` | FilePath→字段默认→extension lit |
| **收敛期** | **C-0…** | **活** | 语义 soundness · 堆安全 · 后端诚实 · SSOT |

**默认下一不再是 U-41 / Ch15 MD / HostFn+1。** 新窗用 `C-N`；厚项继续 park。

## 1. 硬 KPI 与硬否

每窗退出必须复跑（U-40 地板）：

| 闸 | 门槛 |
|----|------|
| Wasm | **46/46** |
| Browser | **0** failed |
| Object residual | **≤180 / ≤10** |
| simd residual | **≥159** |
| `sy,output` | **12/12** |

**硬否：** 无 R-50；不以 HostFn+1 / 文档章节数 / 测试条数冒充成熟度；不以 L006 soft-skip 冒充 native 绿；不把 ManagedHeap stdout churn 当成功；不默认开 JIT / SyxHost / CST 深刀 / 真隐式 `CallSuper`。

## 2. 收敛期活轴（C-*）

### C-0 · 类型 soundness（P0） — `[x]` 薄闸完成

**目标：** 未知/错误类型不得 silently 穿过 check；函数 return、call arity、`inout`/`&mut`、narrowing 有负向闸。

**窗序（相对纸面 a→e 的执行序）：** **C-0b → C-0c → C-0d → C-0a**；**C-0e park**；**C-0a+**（完整三分）park。

| 子项 | 状态 | 内容 |
|------|------|------|
| **C-0b** | `[x]` | wrong-type / 裸非 Void **return** → **S004**；方法 mangled `current_function`；census [`c-0b-return-type-census`](../docs/census/c-0b-return-type-census.md) |
| **C-0c** | `[x]` | 非泛型自由函数 **arity** → **S061**（计 default；标签仍 S042；类型错仍 S004）；census [`c-0c-call-arity-census`](../docs/census/c-0c-call-arity-census.md) |
| **C-0d** | `[x]` | `&mut` 裸值传参拒收（AST `Borrow{mutable:true}`；剥 Group；勿改 `deref_type` / is_compatible 臂）；census [`c-0d-mut-ref-arg-census`](../docs/census/c-0d-mut-ref-arg-census.md) |
| **C-0a** | `[x]` 薄闸 | Unknown→地面具体汇点（无注解形参 / 缺失 Member）；**未**关 `(_, Unknown)`；census [`c-0a-unknown-sink-census`](../docs/census/c-0a-unknown-sink-census.md) |
| **C-0a+** | `[-]` park | `ErrorType` / `InferenceVar` 三分；关软臂；调用实参 Unknown |
| **C-0e** | `[-]` park | integer narrowing / 书 S008 对齐 |

**验收：** 新负向用例进 gate；正向回归不破硬 KPI；census 写清「拒收面」而非「多了多少通过」。  
**不做：** 完整 Swift 泛型；整语言 rewrite；C-0b 不做全覆盖 missing-return CFG。

### C-1 · ManagedHeap 安全默认（P0） — `[x]` 默认降级+identity+immutable+叙事

**目标：** 默认路径不再依赖已知危险的 generational/compact；weak / immutable 语义可测；禁 churn=GC 绿叙事。

| 子项 | 状态 | 内容 |
|------|------|------|
| **C-1a** | `[x]` | 默认堆策略降级（v9/v10/v11/v13/compact opt-in；churn stdout 门闩）；census [`c-1a-heap-default-degrade-census`](../docs/census/c-1a-heap-default-degrade-census.md) |
| **C-1b** | `[x]` | weak load 存活保 canonical identity；死后 / mark_dead 仅 Nil；census [`c-1b-weak-identity-census`](../docs/census/c-1b-weak-identity-census.md) |
| **C-1c** | `[x]` | `mutable=false` ObjectRef 写拒收（不 insert、返回原 handle）；census [`c-1c-immutable-write-census`](../docs/census/c-1c-immutable-write-census.md) |
| **C-1d** | `[x]` | 禁「stdout churn = GC 成功」叙事；Host label echo ≠ GC KPI；census [`c-1d-churn-narrative-census`](../docs/census/c-1d-churn-narrative-census.md) |

**验收（C-1d）：** 测例/文档不再把 print/`gc:churn:` 回显冒充 GC 绿；C-1a 门闩不破；硬 KPI 不破。

### C-2 · Backend / CI 支持矩阵诚实（P0） — `[ ]` **默认下一**

**目标：** 文档与 CI 对 VM / MIR / Wasm / Native / JIT 的「支持」说法与真实证据一致。

| 子项 | 内容 |
|------|------|
| C-2a | 默认硬闸 = VM + MIR + Wasm（46/46）+ browser 0 |
| C-2b | `native=real` 必须跑 full-native；失败 = 红，禁 L006 soft-skip 冒充 |
| C-2c | JIT / experimental backends 明确 `experimental`；非默认 KPI |
| C-2d | CI job 矩阵与 `01`/`capability` 文案对齐 |

**验收：** CI 与 docs 同表；无「软绿」。

### C-3 · MIR 为唯一可验证后端输入（P1） — `[ ]`

DefId / 稳定 identity；MIR verifier；删除或降级 quality heuristic 作为正确性闸。

### C-4 · Package 默认可复现（P1） — `[ ]`

lockfile / git 依赖默认钉死；publish 与测试顺序一致；失败路径可诊断。

### C-5 · 宏 / embed 隔离诚实（P1） — `[ ]`

去掉虚假「sandbox」安全承诺，或落地真隔离边界；文档降级到 threat model。

### C-6 · Version / capability 单一 SSOT（P1） — `[ ]`

消除 0.1.3 / 0.2.x / 0.3.0 多源漂移；生成 `capability` / banner / crate version 同源。

### C-7 · 测试信号与维护成本（P2） — `[ ]`

矩阵分 shard；fuzz/benchmark 真失败信号；巨型模块拆分；实验 scaffold 移出 stable。

### C-8 · 反馈质量 / 文档诚实（P2–P3） — `[ ]`

formatter CST lossless（若开刀则独立窗）；Ch15 隐式 `super.init` MD 与实现对齐或标未实现；语言书与 parser 分叉清单收敛。

## 3. 显式 park（非默认下一）

| 项 | 原因 |
|----|------|
| U-41+ 新语法表面 | 收敛期优先 soundness；表面增长冻结除非 unblock P0 |
| **C-0e** integer narrowing / 书 S008 | 与实现 S008（actor）冲突；独立窗 |
| Ch15 隐式 `super.init` / 真 `CallSuper` | 厚语义；需独立 ADR |
| CST postfix 深刀 / codegen Break | 高成本；非 P0 |
| JIT Suite-1 扩面 | pilot only |
| simd Object wipe / residual 降 | 易破 KPI；禁默认 |
| SyxHost / HostFn+1 | 伞与膨胀风险 |
| 产品伞 GA（Farm / SynUI / marketplace） | 冻结至 ADR |
| 官方模块名 `Foundation` | 永久禁 |

## 4. 窗口工作法

1. **只开一窗**；编号 `C-N`（或报告批准的子项 `C-Na`）。  
2. 先写负向/支持矩阵验收，再改实现。  
3. 退出：硬 KPI + 本窗 census；更新本文件勾选与 `01` 一句。  
4. 厚项 → park + ADR，不塞进默认下一。  
5. 施工草稿可落 `.cursor/plans/`；**勾选 SSOT = 本文件**。

## 5. 与外部报告映射

| 报告优先级 | 收敛刀 |
|------------|--------|
| P0 语义 soundness | **C-0** |
| P0 ManagedHeap | **C-1** |
| P0 Native/CI 矩阵 | **C-2** |
| P1 MIR / lock / sandbox / version | **C-3…C-6** |
| P2/P3 测试·文档·维护 | **C-7…C-8** |

报告全文只读；**活勾选只在本文件**。

## 6. 上期账本指针

完整 SA / R / U 条目、验收句与历史「默认下一」叙事：

→ [`archive/02-improvement-roadmap-sa-r-u-ledger.md`](archive/02-improvement-roadmap-sa-r-u-ledger.md)

勿在收敛期重新打开 R-50 或把 U-41 设为默认下一，除非 `01` 显式改成熟度并 ADR 解锁。
