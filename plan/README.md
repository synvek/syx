# plan/ — SYX 规划与状态 SSOT

本目录是 **维护者规划主入口**。当前活轨道 = **Swift 标准库对齐期（SA）**。用户面向文档仍在 `docs/` 与 `books/`。

> **plan-001（开源清理）：** 窗账本 / tip / `.cursor/plans` 迁往**内部私有兄弟仓**（`ledger/`）。本仓保留 [`plan-001-cleanup.md`](plan-001-cleanup.md) 与 [`cleanup-manifest/`](cleanup-manifest/)；**冻结**新增 `qs1_*_first_slice` 与功能窗 tip bump。双仓政策见 [`docs/dual-repo.md`](../docs/dual-repo.md)。

## 读什么

| 文件 | 用途 |
|------|------|
| [01-project-status.md](01-project-status.md) | **当前整体状态**（SA 期） |
| [02-improvement-roadmap.md](02-improvement-roadmap.md) | **活 backlog**（SA-0…SA-6；附录保留可信度 0→4 契约钉） |
| [todo-003.md](todo-003.md) | Swift 对齐分析底稿（本轮输入） |
| [topics/](topics/) | 短专题：JIT、MIR/AOT、GC、profiles、HostFn、LLVM、插件 ABI v2 |
| [archive/](archive/) | 冻结历史：M0–M4、Wave A–F、**可信度 0→4**、[`window-plans/`](archive/window-plans/) |

产品名：**SYX**（sy-ex）；源扩展名 **`.sy`**；工程清单 **`syx.toml`**。

## 与其它目录的边界

| 位置 | 角色 |
|------|------|
| `plan/` | 活规划与状态 |
| `plan/archive/*-pre-w559-ledger.md` | Wave A/B 逐窗详账（冻结；CI 详述备份） |
| `plan/archive/window-plans/` | 历史 `.cursor/plans` 窗草稿（W559 F-01 归档） |
| `docs/adr/` | 绑定架构决议 |
| `books/shared/implementation-status.md` | 能力现状表 |
| `docs/*` | 技术/使用说明 |
| `docs/gates/` | **工程闸门契约**（B/A rename 已齐；**F-10** 退役纯 meta · W575–W607 / ADR-919–951 **`[x]`**；a0* 归零；b06/b05/b04/b07/b02/b01/b09 轴归零；b09-exit + b09-thin 第一至第十一簇已退；a04/a06/a07/a08 + b06 audit + b05/b04/b07/b02/b01/b09 施工 SSOT 仍活） |
| `docs/lang/` | 语言手册（`a0*` 闸门路径已非法；行为手册仍可留） |
| `plan/archive/gates/` | **已退役闸门契约**（F-10；a02–a08 floors/exit + b06-missing-align-exit + b05-medium-exit + b04-short-exit + b07-real-hostfn-lowering-exit + b02-dual-track-exit + b01-mir-consumer-exit + b09-axis-exit；a04-exit 无 archive md） |
| `.cursor/plans/` | 当前窗施工草稿（历史在 `archive/window-plans/`；**不**替代本目录） |

## 如何更新

1. 勾选 [02-improvement-roadmap.md](02-improvement-roadmap.md) 对应 **SA-*** ID。  
2. 冻结/成熟度变了 → 改 [01-project-status.md](01-project-status.md)。  
3. 对外能力变了 → 同步 `implementation-status.md` + `capabilities.toml`。  
4. 方向变更 → 先 `docs/adr/`，再改 `02`。  
5. **不要**在 `archive/*-ledger.md` 上继续当 backlog 打勾。  
6. 开新窗口：引用 `02` 的 **SA-*** ID；需要施工步骤时再建 `.cursor/plans/*.plan.md`。

**当前提示：** 可信度阶段 0→4 已全部退出（详账见 `archive/*-credibility-stage-ledger.md`）。活施工轴 = **SA-0 章程**（分析底稿 [`todo-003.md`](todo-003.md)）。产品伞 / F-08 / Foundation 命名仍 park。
