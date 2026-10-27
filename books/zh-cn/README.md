# SYX 语言电子书

本目录包含三套中文 Markdown 电子书，面向 SYX 语言用户与实现者：

| 书籍 | 目录 | 说明 |
|------|------|------|
| **SYX 语言指南** | [language-guide/](language-guide/) | 叙事式教程，从入门到高级特性，含完整范例与 Swift 差异 |
| **SYX 语言参考** | [language-reference/](language-reference/) | 词法、语法、类型、声明、模式、特性（Attributes）的形式化参考 |
| **SYX 标准库指南** | [library-guide/](library-guide/) | 嵌入包用户面：`import Linalg` / `Os` / `Log` / `IO` 等；诚实边界与仓库范例 |

共享资源：[shared/implementation-status.md](../shared/implementation-status.md)——**当前**特性实现状态总表（SYX **117.0** / Phase **139**）。

**版本对齐：** SYX 117.0（Phase 139）· cache `syx-cache-v1` · audit 2600/3050 · 架构决议见 [docs/adr/architecture-decisions.md](../../docs/adr/architecture-decisions.md)

## 阅读建议

| 你是… | 建议路径 |
|------|---------|
| 第一次接触 SYX | [前言](language-guide/00-preface.md) → [快速入门](language-guide/01-getting-started.md) → 按 `language-guide/SUMMARY.md` 顺序读完 |
| 查嵌入包（Linalg / Os / Log） | [标准库指南](library-guide/00-preface.md) → [包一览](library-guide/01-overview.md) |
| 查阅某个语法/类型 | [语言参考 · 目录](language-reference/00-about-this-reference.md) → 对应章节 |
| 看某个特性 GA/partial | [实现状态总表](../shared/implementation-status.md)（**单一** ground truth） |
| 写编译器或 SDK | [docs/README.md](../../docs/README.md) → [架构决议](../../docs/adr/architecture-decisions.md) |
| 装 CLI、跑程序、debug | [docs/9_UsageGuide.md](../../docs/9_UsageGuide.md) |

## 构建 HTML / EPUB / PDF

### mdBook（推荐）

```bash
# 语言指南
cd books/zh-cn/language-guide && mdbook build

# 语言参考
cd books/zh-cn/language-reference && mdbook build

# 标准库指南
cd books/zh-cn/library-guide && mdbook build
```

每本书已提供 `book.toml`；`SUMMARY.md` 即为章节目录。`mdbook serve` 启动本地热重载预览。

### Pandoc 一键 EPUB

```bash
pandoc books/zh-cn/language-guide/SUMMARY.md \
  $(ls books/zh-cn/language-guide/*.md | grep -v SUMMARY) \
  -o syx-language-guide.epub \
  --toc --metadata title="SYX 语言指南"

pandoc books/zh-cn/language-reference/SUMMARY.md \
  $(ls books/zh-cn/language-reference/*.md | grep -v SUMMARY) \
  -o syx-language-reference.epub \
  --toc --metadata title="SYX 语言参考"
```

### VS Code

安装 Markdown 预览增强插件，打开任意章节即可阅读；`SUMMARY.md` 提供导航。

## 实现状态标记

全书统一使用四档标记（详见 [实现状态总表](../shared/implementation-status.md)）：

| 标记 | 含义 |
|------|------|
| **已实现** / **GA** | `syx run` 或 parity / 硬闸可验证 |
| **部分实现** | 可解析/编译但语义不完整 |
| **计划中** | roadmap 有记录，实现未完成 |
| **未实现** | 无对应 AST/语义 |

带「**部分 GA**」前缀的项为该 Phase 已交付一个最小工作子集，余下全集随后续 Phase 推进。带「**拒绝**」的项为有意 fail-closed（如计算属性观察器 S056）。

## 与现有 docs 的关系

| 文档 | 角色 |
|------|------|
| `books/`（本目录） | 语言教程 + 形式化参考 + [标准库指南](library-guide/00-preface.md) |
| `docs/8_QuickReference.md` | 语法速查（与 parser 严格对齐的简版） |
| `docs/9_UsageGuide.md` | 工具链、CLI、标准库与 Native/WASM 编译 |
| [`docs/adr/architecture-decisions.md`](../../docs/adr/architecture-decisions.md) | 架构决议 SSOT |
| `plan/topics/hostfn-extension.md` | Runtime 扩展分层与维护者清单 |
| `docs/1–5_*.md` | 编译器实现技术文档（按 1→2→3→4→5 顺序） |
| `packages/README.md` | 嵌入标准库（`import Di` / `import Os` / `import UI` 等） |

## 当前 Phase 139 能力摘要

- **Gate0：** audit **2600/3050**；manifest **117.0** / phase **139**；EXCLUDE 历史 behavioral fill
- **语言面：** multiline / tuple / nested path / closed range / Character / call labels（详见 status）
- **GUI：** SynUI deferred（非本阶段 GA）
- **Farm/Dist：** frozen at v19
- **Wasm SIMD16：** i8x16 all_true/any_true guest-v22 MVP 及既有 guest 表面
- **详情：** [shared/implementation-status.md](../shared/implementation-status.md)
- **Farm / Dist：** **冻结 v19**（hsm-key-id / peer-announce-hold）；本阶段无新表面
- **Os（部分 GA）：** `primaryIpv4()` MVP；disk/load / hostname/uptime / cpu/mem/user 继承；**非**完整 sysinfo → **139+**
- **DX：** `driver_gate` / LSP audit 见 status；cache **`syx-cache-v1`**；许可证 **Apache-2.0**
- **原生 AOT：** **LLVM sole**（Cranelift AOT 已移除；JIT tier-2 opt-in）
- **Core builtins：** [`packages/core/builtins/`](../../packages/core/builtins/)（数值模板 [`_templates/`](../../packages/core/builtins/_templates/)）
