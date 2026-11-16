# SYX 项目整体状态

> **维护：** 对外能力叙事、阶段退出或标准库边界变更时更新。  
> **活 backlog：** [`02-improvement-roadmap.md`](02-improvement-roadmap.md)（**post-SA Residual · R-***）  
> **分析底稿：** [`todo-003.md`](todo-003.md) · [`ProjectAnalysisAndImprovementPlan.zh-CN.md`](ProjectAnalysisAndImprovementPlan.zh-CN.md)  
> **上期冻结：** [`archive/01-project-status-credibility-stage-ledger.md`](archive/01-project-status-credibility-stage-ledger.md) · [`archive/02-improvement-roadmap-credibility-stage-ledger.md`](archive/02-improvement-roadmap-credibility-stage-ledger.md)  
> **最近更新：** 2026-11-10 · **R-21 `[x]`**（embedded 与文件路径 struct 字段分歧修复 · [r-21 计划](../.cursor/plans/r-21_path_parity_6f2b9e38.plan.md)）；下一 = **R-22+ residual**。

## 1. 一句话现状

**可信度 0→4 与 SA-0…SA-6 已退出；post-SA Residual：R-0…R-15、R-20 已交（R-16…R-19 为 W001 结案挂账）。**  
`FileHandle.withOpen` → `Function`；Channel payload + `Select` → `Sendable`/`Array<Channel>`；Mutex/Semaphore/`Channel.handle`/`UdpSocket` → `Int`；`File`/`TcpStream.readAsync`/`UdpSocket.recvFrom`/`HttpBodyStream.open*`/`Url.query`/`WebSocket`/`SqlRow`/`Concurrency` 任务袋 → `Dictionary`；`Fmt`/`assertEq` 及 `Concurrency` 值位 → `Sendable`，`test(fn:)` → `Function`；struct/data class `==`/`!=` → 结构相等（R-20）。并发套件 **37/0**（R-14）；`syx-driver --lib` **58/0**（R-15）。下一 = 公开 Object inventory 下一最大可交切片（R-21+；simd/collections…）。对齐章程：[stdlib-swift-alignment](../docs/adr/stdlib-swift-alignment.md) · 退出 ADR：[sa-exit](../docs/adr/sa-exit.md)。

## 2. 版本与冻结

| 项 | 值 |
|----|-----|
| Crate / workspace | **0.1.3** |
| 对外 semver | **0.1.3** |
| HostFn / MirOp | **629** HostFn · **136** MirOp |
| 程序轨道 | 可信度 0→4 **已退出**；SA **已退出**；现开 **post-SA Residual（R-*）** |
| 架构 SSOT | [`docs/adr/architecture-decisions.md`](../docs/adr/architecture-decisions.md) |
| 语言面 SSOT | [`docs/adr/language-book-ssot.md`](../docs/adr/language-book-ssot.md) + `books/language-guide` |
| 标准库现状索引 | [`packages/README.md`](../packages/README.md) · [`packages/internal/manifest.json`](../packages/internal/manifest.json) · [`packages/internal/stability.toml`](../packages/internal/stability.toml) |

**默认冻结（直至显式 ADR 解锁）：**

- 产品伞 GA unpark（P-ABI-v2 / P-FAER / P-SURF / Farm·SynUI 等）
- F-08 RT ABI 全清；HostFn+1 作为 KPI
- 以 fingerprint / 源码 `contains` 作为 release KPI
- 命名任何官方模块为 `Foundation`（避免与 Apple Foundation 兼容期待绑定）

品牌：产品名 **SYX**；源文件 **`.sy`**；工程清单 **`syx.toml`**。

## 3. 完成度（指导）

| 轨道 | 状态 | 说明 |
|------|------|------|
| Truth M0–M4 | 收官（归档） | `archive/roadmap-m0-m4-task-list.md` |
| Wave A–F | 软完成 @ 52/53（归档） | `archive/*-wave-af-ledger.md`；`F-08` 仍 park |
| **阶段 0 止血自洽** | **`[x]` 退出** | S0-01…08 |
| **阶段 1 语言核心** | **`[x]` 退出（S1-I27）** | S1-04/05 = residual |
| **阶段 2 架构收敛** | **`[x]` 退出（S2-I266）** | S2-01…07 scoped `[x]` |
| **阶段 3 DX / 性能** | **`[x]` 退出（S3-I87）** · **7/7** | S3-01…07 scoped `[x]` |
| **阶段 4 生态增强** | **`[x]` 退出（S3-I109）** · **5/5** | S4-01…05 scoped `[x]`；post-exit Residual / 维护 |
| **SA 标准库对齐** | **SA-0…6 `[x]` 退出** | [sa-exit](../docs/adr/sa-exit.md) |
| **post-SA Residual** | **R-0…R-15、R-20、R-21 `[x]`** | R-16…R-19 为 W001 结案挂账；下一 = R-22+ residual |

```mermaid
flowchart LR
  cred["Credibility 0-4 exited"] --> sa["SA-0to6 exited"]
  sa --> r0["R-0 residual"]
  r0 --> r1["R-1 withOpen"]
  r1 --> r2["R-2 Sendable"]
  r2 --> r3["R-3 recv Sendable"]
  r3 --> r4["R-4 Select"]
  r4 --> r5["R-5 MutexSem Int"]
  r5 --> r6["R-6 handle Int"]
  r6 --> r7["R-7 UdpSocket Int"]
  r7 --> r8["R-8 HttpBodyStream Dict"]
  r8 --> r9["R-9 Url query Dict"]
  r9 --> r10["R-10 WebSocket Dict"]
  r10 --> r11["R-11 SqlRow Dict"]
  r11 --> r12["R-12 Fmt Testing"]
  r12 --> r13["R-13 Concurrency Typed"]
  r13 --> r14["R-14 Concurrency Debt"]
  r14 --> r15["R-15 Lib Test Debt"]
  r15 --> r16["R-16..R-19 W001 debt"]
  r16 --> r20["R-20 DataClass ops"]
  r20 --> r21["R-21 Path parity"]
  r21 --> r22["R-22 plus residual"]
```

## 4. 成熟度快照（指导用）

| 领域 | /5 | 判断 |
|------|---:|------|
| 公开仓自洽 / CI | 4.0 | 阶段 0 退出；书 KPI en 硬闸 |
| Lexer / Parser | 3.8 | `do/catch` + `throws -> T`；`public data class` 拒绝 |
| Semantic / 泛型 | 2.6 | `enum: Error` 闸可用；Sequence/Collection residual |
| MIR / VM | 3.5 | 参考路径可用；错误栈模型未重做 |
| 标准库表面 | 4.0 | System/Networking/SQL/Text/Testing/Concurrency 示范；`File`/`withOpen`/Channel/`Select`/Mutex·Semaphore/`handle`/`Udp`/`HttpBodyStream`/`Url`/`WebSocket`/`SqlRow`/`Fmt`/`assertEq`/`Concurrency` 任务袋离 Object；simd/collections… → R-16+ |
| Prelude 边界 | 3.5 | SA-1：26 模块 = `prelude_core.toml` target；IO/Math 等需 `import` |
| library-guide | 3.5 | SA-5-01 已对齐模块名；language-guide 全扫仍 residual |
| fmt / LSP / pkg | 3.0 | `syx doctest` MVP 已有；`syx migrate` / stability 接线仍 residual |

## 5. 开放债（对接 `02` 阶段 R）

### P0 — 阻断「像 Swift」

- 公开 Object inventory 仍 >0（simd / collections… → R-21+；代码行口径 `object_sig_residual_count=198`）；Networking/`SqlRow`/`Fmt`/`Testing`/`Concurrency` 公开 Object 已清
- Collections 合并 defer（OrderedSet/Deque）；Testing auto-import defer
- Mutex/Semaphore/`Channel.handle`/`UdpSocket` 已 `Int`；`HttpBodyStream`/`Url`/`WebSocket`/`SqlRow`/`Concurrency` 袋 → `Dictionary`；`Fmt`/`assertEq`/`Concurrency` 值位 → `Sendable`（R-5…R-13）；下一最大切片 → **R-21+**
- 已知缺口挂账（[`r-20-known-gaps`](../docs/census/r-20-known-gaps.md)）：
  - **wasm emit W001**：46 例全红，七假设证伪，需 MIR 层栈断言后再启
  - **struct/data class 运算符**：`==`/`!=` 已合成（R-20）；`+ - * / < <= > >= & | ^ << >>` 按设计不合成，给出可操作诊断
  - **embedded 与 CLI 的 `ObjectGet` 分歧**：已修复（R-21，`merge_mir_shell_functions`）；`r21_path_parity_contract` 3/3 钉住两路径逐行相等
  - **`syx-mir` 3 例既有失败**：`class_iterator_witness_in_init` / `lowers_generic_instance_param` / `mir_closure_names_unique_across_functions`（A/B 确认非 R-20 引入）

### P1 — 重组与重写

- 模块改名已落地；深 Swiftify / async 统一 / 错误枚举批量 → SA-4-03…07 residual
- language-guide 旧名薄扫；tools 深化
- manifest↔stability 接线（SA-5-06）

### P2 — 工具与收口

- `syx migrate` 产品化
- Observation / UI / DI 边界诚实化（SA-0-05）

## 6. 文档边界

| 位置 | 角色 |
|------|------|
| **`plan/01` + `plan/02`** | 活：状态 + **R / residual** backlog |
| **`plan/todo-003.md`** | Swift 对齐分析底稿（本轮输入） |
| **`plan/archive/*-credibility-stage-ledger.md`** | 冻：可信度 0→4 全账 |
| **`docs/adr/`** | 架构决议（SA 章程 ADR 见 `02` SA-0-01） |
| **`packages/`** | 嵌入标准库源 |
| **`.cursor/plans/`** | 当前窗施工草稿 |

新工作默认：先改 `02` 勾选 → 必要时 ADR → 再开 `.cursor/plans` 窗口。

## 7. 上期退出指针（契约 / 维护）

可信度阶段收官事实不变，详账见 archive。**阶段 0 / 1 / 2 已退出**；阶段 3 / 4 亦已 `[x]` 退出。关键钉：

- Stage-2 exit：**S2-I266** · [stage2-exit](../docs/adr/stage2-exit.md)
- Stage-3：**S3-01…S3-07** `[x]` · exit **S3-I87** · 7/7 · [stage3-exit](../docs/adr/stage3-exit.md)  
  - S3-01 claim **S3-I41**；S3-02 open **S3-I42** → demangle **S3-I43** → debug **S3-I44** → runtime **S3-I45** → meta **S3-I46** → claim **S3-I47**  
  - S3-03 claim **S3-I55**；S3-04 claim **S3-I71**；S3-05 open **S3-I72** → claim **S3-I77**  
  - S3-06：**S3-I79** / residual **S3-I80** / claim **S3-I81**；S3-07 open **S3-I82** → default-vm **S3-I83** → residual **S3-I84** → claim **S3-I85**；exit-open **S3-I86**
- Stage-4：**S4-01…S4-05** `[x]` · exit **S3-I109** · 5/5 · [stage4-exit](../docs/adr/stage4-exit.md)  
  - open **S3-I88**（[stage4-open](../docs/adr/stage4-open.md)）；S4-01 SSOT **S3-I89** / residual **S3-I90** / claim **S3-I91**  
  - S4-02 open **S3-I92** → hinge **S3-I93** → residual **S3-I94** → claim **S3-I95**  
  - S4-03 open **S3-I96** → hinge **S3-I97** → residual **S3-I98** → claim **S3-I99**  
  - S4-04 open **S3-I100** → hinge **S3-I101** → residual **S3-I102** → claim **S3-I103**  
  - S4-05 open **S3-I104** → hinge **S3-I105** → residual **S3-I106** → claim **S3-I107**  
  - exit-open **S3-I108** → exit **S3-I109**
- 下一维护面曾记为 **post-exit Residual / 维护**；现由 **SA** 吸收可施工 residual（doctest / 模块边界），其余仍 park
