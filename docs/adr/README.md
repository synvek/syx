# Architecture Decision Records

**开源读者请从这里开始：** [capabilities-summary.md](capabilities-summary.md)（现行能力摘要，按域，无窗 tip）。

开源清理（plan-001）后 ADR **双轨**：

| 文档 | 角色 | 可见性 |
|------|------|--------|
| [capabilities-summary.md](capabilities-summary.md) | **现行能力主入口**（按域；无窗 tip） | 公开 |
| [language-book-ssot.md](language-book-ssot.md) | **S1-01**：书（Swift 风格）为语言面 SSOT + 一周期兼容弃用 | 公开 |
| [stage1-s1-07-overflow-wrap-scope.md](stage1-s1-07-overflow-wrap-scope.md) | **S1-07 scoped**：整数溢出 **wrap SSOT**；≠ trap / R011 / `&+` / 阶段1退出 | 公开 |
| [stage1-s1-08-cli-multidiag-scope.md](stage1-s1-08-cli-multidiag-scope.md) | **S1-08 scoped**：LSP + 单文件 check + 项目 check parse 多诊；≠ build·run 多诊 / 阶段1退出 | 公开 |
| [stage1-s1-09-book-kpi-ci-scope.md](stage1-s1-09-book-kpi-ci-scope.md) | **S1-09 scoped**：en `book_pass_rate` CI 硬闸（100% + parse_fail=0）；≠ zh 矩阵 / 阶段1退出 | 公开 |
| [stage1-s1-10-vm-inventory-scope.md](stage1-s1-10-vm-inventory-scope.md) | **S1-10 scoped**：VM allowlist + CI = 验收完成；≠ 拉满 / 三后端 / 阶段1退出 | 公开 |
| [stage1-exit-examples-leg.md](stage1-exit-examples-leg.md) | **阶段1退出 examples 腿**：S1-10 VM + S0 三后端薄冒烟；≠ 拉满 / ≠ allowlist 全三后端 / ≠ 自动退出宣称 | 公开 |
| [architecture-decisions.md](architecture-decisions.md) | 历史全量决议（**过渡**；非现行主叙事） | 公开过渡 |
| 私有仓 `ledger/adr-window-stream.md` | 窗级 Decision 流水 | **仅内部** |
| 私有仓 `ledger/architecture-decisions.full.md` | ADR 全量副本 | **仅内部** |

Numbered charters (`NNN-*-charter.md`) are **retired**. Do **not** add new numbered ADR files.

**维护约定（plan-001）：**

- 公开叙事改能力边界 → 更新 `capabilities-summary.md`
- 内部窗级进度 / tip → 只写内部 ledger；**禁止**在公开 CI 钉 tip / 序词指纹
- 功能语义变更仍须有架构依据；新功能 PR 只进本仓（public）
