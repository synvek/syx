# SYX 改进路线图（收敛期 · C-*）

> **活 backlog。** 编号刀 = **C-0…**（Convergence）。  
> **状态叙事：** [`01-project-status.md`](01-project-status.md)  
> **外部审计输入：** [`syx-status-final-report.md`](syx-status-final-report.md)  
> **上期详账（只读）：** [`archive/02-improvement-roadmap-sa-r-u-ledger.md`](archive/02-improvement-roadmap-sa-r-u-ledger.md)（SA-0…R-49 + U-0…U-40）  
> **最近更新：** 2026-11-24 · **C-3b MIR CFG floor `[x]`**；C-3 `[~]`；默认下一 = **C-3c**（薄 DefId / FuncId）。

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

### C-2 · Backend / CI 支持矩阵诚实（P0） — `[x]` SSOT + mirror + unpaid demote + paid floor + JIT label + ci.job 对齐

**命名：** `-N` = N 个历史 `+`（`C-2b-1`≈`C-2b+`，`C-2b-2`≈`C-2b++`，`C-2b-3`≈旧 `C-2b+++`）。新默认下一禁止写 `+++`。

**目标：** 文档与 CI 对 VM / MIR / Wasm / Native / JIT 的「支持」说法与真实证据一致。  
**SSOT：** [`docs/gates/backend-ci-matrix.md`](../docs/gates/backend-ci-matrix.md)

| 子项 | 状态 | 内容 |
|------|------|------|
| **C-2a** | `[x]` | 默认硬闸 = VM + MIR + Wasm（46/46）+ browser 0；native L006 / JIT = soft·experimental·非默认 KPI；census [`c-2a-backend-ci-matrix-census`](../docs/census/c-2a-backend-ci-matrix-census.md) |
| **C-2b** | `[x]` | 禁 `run_native_strict_or_mirror_vm` 假绿；GA 无 AOT → skip；census [`c-2b-mirror-vm-honesty-census`](../docs/census/c-2b-mirror-vm-honesty-census.md) |
| **C-2b+** / **C-2b-1** | `[x]` | unpaid `native=real`→`partial`；禁 soft_native L006→exit 0；census [`c-2b-plus-native-real-honesty-census`](../docs/census/c-2b-plus-native-real-honesty-census.md) |
| **C-2b++** / **C-2b-2** | `[x]` | required `full-native-linux`（LLVM 21.1.6 + hello `--native`）；census [`c-2b-plusplus-full-native-ci-census`](../docs/census/c-2b-plusplus-full-native-ci-census.md) |
| **C-2b-3** | `[x]` | path_file GA 进 paid job；`io.path` + `io.file.read_all_bytes` → `native=real`；census [`c-2b-3-path-file-raise-census`](../docs/census/c-2b-3-path-file-raise-census.md) |
| **C-2b-4** | `[x]` | tcp GA 进 paid job；`net.tcp` + `net.tcp.listener` → `native=real`；census [`c-2b-4-tcp-raise-census`](../docs/census/c-2b-4-tcp-raise-census.md) |
| **C-2b-5** | `[x]` | sql GA 进 paid job；`data.sqlite` → `native=real`（≠ pool/row）；census [`c-2b-5-sql-raise-census`](../docs/census/c-2b-5-sql-raise-census.md) |
| **C-2b-6** | `[x]` | http GA 进 paid job；`net.http` → `native=real`（≠ server/types/stream/timeout_tls）；census [`c-2b-6-http-raise-census`](../docs/census/c-2b-6-http-raise-census.md) |
| **C-2b-7** | `[x]` | pool/row llvm 进 paid job；`data.sqlite.pool` + `data.sqlite.row` → `native=real`；census [`c-2b-7-pool-row-raise-census`](../docs/census/c-2b-7-pool-row-raise-census.md) |
| **C-2b-8** | `[x]` | types llvm 进 paid job；`net.http.types` → `native=real`（≠ server/stream/timeout_tls）；census [`c-2b-8-http-types-raise-census`](../docs/census/c-2b-8-http-types-raise-census.md) |
| **C-2b-9** | `[x]` | server llvm 进 paid job；`net.http.server` → `native=real`（≠ stream/timeout_tls）；census [`c-2b-9-http-server-raise-census`](../docs/census/c-2b-9-http-server-raise-census.md) |
| **C-2b-10** | `[x]` | stream llvm 进 paid job；`net.http.stream` → `native=real`（≠ timeout_tls）；census [`c-2b-10-http-stream-raise-census`](../docs/census/c-2b-10-http-stream-raise-census.md) |
| **C-2b-11** | `[x]` | timeout llvm 进 paid job；`net.http.timeout_tls` → `native=real`（≠ tls.client）；census [`c-2b-11-timeout-tls-raise-census`](../docs/census/c-2b-11-timeout-tls-raise-census.md) |
| **C-2b-12** | `[x]` | raii/stream/watch llvm 进 paid job；`io.file.raii`+`stream`+`watch` → `native=real`（≠ tls）；census [`c-2b-12-io-file-raise-census`](../docs/census/c-2b-12-io-file-raise-census.md) |
| **C-2b-13** | `[x]` | tls.client aot 进 paid job；`net.tls.client` → `native=real`（≠ server/daemon）；census [`c-2b-13-tls-client-raise-census`](../docs/census/c-2b-13-tls-client-raise-census.md) |
| **C-2c** | `[x]` | JIT 公开 Stub/default-on → experimental-real-subset · default off · non_default_kpi；census [`c-2c-jit-experimental-label-census`](../docs/census/c-2c-jit-experimental-label-census.md) |
| **C-2d** | `[x]` | 8× `ci.job.*` surface 对齐 yaml job；census [`c-2d-ci-job-align-census`](../docs/census/c-2d-ci-job-align-census.md) |

**验收（C-2d）：** surface 与 yaml 对齐；soft/hard 不变；16× `native=real` 不动；C-2 `[x]`；硬 KPI 不破。

### C-3 · MIR 为唯一可验证后端输入（P1） — `[~]`

DefId / 稳定 identity；MIR verifier；删除或降级 quality heuristic 作为正确性闸。子项：

| 子项 | 状态 | 说明 |
|------|------|------|
| **C-3a** | `[x]` | 禁止 codegen 静默 quality 选同名 MIR body → MIR001 fail-closed；同名 overload 上游 arity 唯一化；census [`c-3a-mir-quality-demote-census`](../docs/census/c-3a-mir-quality-demote-census.md) |
| **C-3b** | `[x]` | 薄 verifier 地板：`verify_mir_cfg_floor` → MIR002（entry / OOB jump·successor / missing terminator）；挂钩 unify + codegen；census [`c-3b-mir-cfg-floor-census`](../docs/census/c-3b-mir-cfg-floor-census.md) |
| **C-3c** | `[ ]` **默认下一** | 薄 DefId / 稳定 `FuncId` 表（按名→id；≠ ModuleId 图式 / 全量 MirOp verifier） |

**验收（C-3b）：** 非法 entry/OOB jump/缺 terminator → MIR002；合法流水线不破；负向钉绿；硬 KPI 不破；**不**宣称 C-3 `[x]`。

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
