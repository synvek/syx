# plan/ — SYX 规划与状态 SSOT

本目录是 **维护者规划主入口**。当前活轨道 = **收敛期（C-*)**。用户面向文档仍在 `docs/` 与 `books/`。

> **plan-001（开源清理）：** 窗账本 / tip / `.cursor/plans` 迁往**内部私有兄弟仓**（`ledger/`）。本仓保留 [`plan-001-cleanup.md`](plan-001-cleanup.md) 与 [`cleanup-manifest/`](cleanup-manifest/)；**冻结**新增 `qs1_*_first_slice` 与功能窗 tip bump。双仓政策见 [`docs/dual-repo.md`](../docs/dual-repo.md)。

## 读什么

| 文件 | 用途 |
|------|------|
| [01-project-status.md](01-project-status.md) | **当前整体状态**（收敛期；~2.5/5 研究型 beta） |
| [02-improvement-roadmap.md](02-improvement-roadmap.md) | **活 backlog**（C-0 / C-1 `[x]`；**C-2a…b++** `[x]`；默认下一 = **C-2b+++**） |
| [syx-status-final-report.md](syx-status-final-report.md) | 外部只读审计输入（2026-11） |
| [todo-003.md](todo-003.md) | Swift 对齐分析底稿（历史输入） |
| [topics/](topics/) | 短专题：JIT、MIR/AOT、GC、profiles、HostFn、LLVM、插件 ABI v2 |
| [archive/](archive/) | 冻结历史：M0–M4、Wave、可信度 0→4、**SA/R/U 全账**、[`window-plans/`](archive/window-plans/) |

产品名：**SYX**（sy-ex）；源扩展名 **`.sy`**；工程清单 **`syx.toml`**。

## 与其它目录的边界

| 位置 | 角色 |
|------|------|
| `plan/` | 活规划与状态（**C-***） |
| `plan/archive/02-improvement-roadmap-sa-r-u-ledger.md` | SA-0…R-49 + U-0…U-40 详账（冻结） |
| `plan/archive/01-project-status-pre-convergence.md` | 收敛前 `01` 快照 |
| `plan/archive/*-pre-w559-ledger.md` | Wave A/B 逐窗详账（冻结） |
| `plan/archive/window-plans/` | 历史 `.cursor/plans` 窗草稿 |
| `docs/adr/` | 绑定架构决议 |
| `books/shared/implementation-status.md` | 能力现状表 |
| `docs/*` | 技术/使用说明 |
| `docs/gates/` | 工程闸门契约 |
| `plan/archive/gates/` | 已退役闸门契约 |
| `.cursor/plans/` | 当前窗施工草稿（**不**替代本目录勾选） |

## 如何更新

1. 勾选 [02-improvement-roadmap.md](02-improvement-roadmap.md) 对应 **C-*** ID。  
2. 冻结/成熟度变了 → 改 [01-project-status.md](01-project-status.md)。  
3. 对外能力变了 → 同步 `implementation-status.md` + `capabilities.toml`。  
4. 方向变更 → 先 `docs/adr/`，再改 `02`。  
5. **不要**在 `archive/*-ledger.md` 上继续当 backlog 打勾。  
6. 开新窗口：引用 `02` 的 **C-*** ID；需要施工步骤时再建 `.cursor/plans/*.plan.md`。

**当前提示：** SA / R / U 编号账本已收口（详账见 [`archive/02-improvement-roadmap-sa-r-u-ledger.md`](archive/02-improvement-roadmap-sa-r-u-ledger.md)）。活施工轴 = **C-2b+++**（选择性 `native=real`；C-2a…b++ 已退出；C-2c/d · C-0a+/e park）。U-41+ / JIT / SyxHost / 真隐式 `CallSuper` / 产品伞 / Foundation 命名仍 park。
