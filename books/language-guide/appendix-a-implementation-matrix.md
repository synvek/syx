# 附录 A：实现状态矩阵

本附录指向统一维护的特性状态总表，避免与 [shared/implementation-status.md](../shared/implementation-status.md) 重复。

**完整矩阵：** [books/shared/implementation-status.md](../shared/implementation-status.md)

## 快速摘要（SYX 117.0 · Phase 139 · Truth M0）

| 领域 | 已实现 / GA | 部分实现 | 未实现 / 债务 |
|------|-------------|----------|---------------|
| 基础语法与类型 | 数值、集合、控制流、`Array<T>`、`Dictionary`、tuple / `"""` / `...` / call labels（P139）；Iterator for-in（ADR-223）；collection mutation write-back（ADR-224）；Deque&lt;Int&gt; MVP（ADR-248） | `Sequence.makeIterator` / 全量 Collection Index / Dict 下标插入 | Heap/lazy sequence |
| String / Character | UTF-8、Character distinct scalar MVP（P139） | Unicode Character 全集、Index 模型 | — |
| OOP / class 体系 | class/struct/enum、Properties v6、final（sealed 文档已清除） | deinit 全表面 | — |
| 并发 | async/Task/actor、Actor shared pool | ThreadPoolExecutor 全集 | Swift 6 strict concurrency |
| 内存 | ARC + ManagedHeap | borrow 完整 | — |
| 宏 / DI | M28 + DI v19 继承 | 完整 token / AspectJ / Spring | **140+** |
| SIMD | guest-v22 all/any true + 继承 bitmask/… | SIMD16 全集 | — |
| Net / Time | HTTP client/server VM；TCP **integer handle**（M0）；Clock.nowMs；DateTime 窄历法核心 **implemented**（ADR-222） | IANA / `formatWithTimezone` **unsupported**（U001） | 真 WS/gRPC 协议、IANA DateTime GA |
| GUI / Farm / Dist | SynUI v12 / Farm·Dist v19 **冻结** | — | SynUI v13 / Farm v20 |
| 工具链 | LLVM sole native；`syx-cache-v1`；**fmt fail-closed**（M0-A02）；JIT **experimental-real-subset** opt-in（`SYX_JIT_TIER2` / Suite-1 Cranelift；M0-A01） | lossless CST formatter | Cranelift（已移除 AOT；JIT tier-2 仍为 opt-in） |

> **Truth M0 W1–W2（ADR-132）：** 停止伪成功（fmt/DateTime/TCP/Dict.remove/PBKDF2/JIT）；详见 [`plan/archive/roadmap-m0-m4-task-list.md`](../../plan/archive/roadmap-m0-m4-task-list.md)（冻结）与 [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)。

> **Truth M0 W2–W4（ADR-133）：** capability manifest + 四后端 differential；生成矩阵见 [capabilities-generated.md](../shared/capabilities-generated.md)。

> **本 Phase 139 关键变化：** multiline / tuple / nested path / range / Character / call labels / sealed docs；SIMD16 all/any；audit 2600/3050；`syx-cache-v1`。


## 三后端 parity 总览

| 后端 | 范围 | 状态 |
|------|------|------|
| **VM**（默认） | 解释执行 + GC | **GA** |
| **Native (LLVM)** | MIR → LLVM IR → 静态链接 | **GA**（Phase 77 sole backend） |
| **WASM** | MIR 直编译 + wasmtime host 跑 SYX VM | **GA** |
| Cranelift | MIR → Cranelift IR | **已移除**（Phase 77） |

## 状态档位

| 档位 | 含义 |
|------|------|
| **已实现** / **GA** | `syx run` 或 parity / 硬闸可验证 |
| **部分实现** / **partial GA** | 可编译但语义不完整 |
| **计划中** | roadmap 有记录 |
| **未实现** | 无对应 AST/语义 |

> **更新策略：** 请只修改 `shared/implementation-status.md`；本附录仅作快速摘要。

---

**参见** → [语言参考附录](../language-reference/appendix-implementation-matrix.md) · [实现状态总表](../shared/implementation-status.md) · [architecture-decisions.md](../../docs/adr/architecture-decisions.md)
