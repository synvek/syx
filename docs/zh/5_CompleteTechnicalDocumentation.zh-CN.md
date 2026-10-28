# SYX 编译器 — 完整技术文档（Rust 实现）

[English](../5_CompleteTechnicalDocumentation.md)


## 目录

1. [项目概述](#1-项目概述)
2. [Cargo Workspace 与项目结构](#2-cargo-workspace-与项目结构)
3. [架构设计与编译流水线](#3-架构设计与编译流水线)
4. [Crate 职责详解](#4-crate-职责详解)
5. [编译驱动（syx-driver）](#5-编译驱动syx-driver)
6. [命令行工具（syx-cli）](#6-命令行工具syx-cli)
7. [模块系统与依赖管理](#7-模块系统与依赖管理)
8. [增量编译缓存](#8-增量编译缓存)
9. [Meta 与 Async 编译期 Pass](#9-meta-与-async-编译期-pass)
10. [前端：词法、语法、语义](#10-前端词法语法语义)
11. [代码生成与中间表示](#11-代码生成与中间表示)
12. [虚拟机与运行时](#12-虚拟机与运行时)
13. [标准库与 Host Functions](#13-标准库与-host-functions)
14. [工具链 Crate（P2/P3）](#14-工具链-cratep2p3)
15. [错误处理与诊断](#15-错误处理与诊断)
16. [测试体系](#16-测试体系)
17. [构建与 CI](#17-构建与-ci)
18. [附录](#18-附录)

---

## 1. 项目概述

### 1.1 项目简介

SYX 是一门受 Swift 语法启发的编程语言。编译器与工具链 **完全由 Rust 实现**，以 Cargo workspace 组织，覆盖从源码到可执行字节码的完整编译流程。

核心能力：

| 阶段 | 实现 Crate | 说明 |
|------|-----------|------|
| 词法分析 | `syx-lexer` | 源码 → Token 流 |
| 语法分析 | `syx-parser` | Token → AST（`syx-ast`） |
| 语义分析 | `syx-semantic` | 类型检查、符号表、模块导入 |
| 元信息 Pass | `syx-meta` | 数据类注册、纯函数/副作用校验 |
| 异步校验 | `syx-async` | `async`/`await` 语法约束 |
| 代码生成 | `syx-codegen` | AST 或 MIR → VM 指令 |
| 执行 | `syx-vm` | 栈式虚拟机 + Host Functions |
| 驱动编排 | `syx-driver` | 模块图、缓存、多后端入口 |
| 用户入口 | `syx-cli` | `syx` 命令行 |

### 1.2 实现阶段划分

| 阶段 | 范围 | Rust 实现状态 |
|------|------|---------------|
| **P0** | 模块化、字符串、文件 I/O、HTTP、数组/迭代 | 基本完成，集成测试覆盖 |
| **P1** | 空安全、数据类、guard let、效果标注 | 部分完成 |
| **P2** | MIR、格式化、Lint、文档生成、调试会话 | 脚手架 + MVP |
| **P3** | LLVM 原生、WASM 发射、JIT、元编程 | 脚手架 + MVP |

---

## 2. Cargo Workspace 与项目结构

### 2.1 Workspace 配置

根目录 `Cargo.toml` 定义 workspace：

- **members**：`crates/*`（共 19 个 crate）
- **edition**：2021
- **rust-version**：1.80
- **共享依赖**：`miette`、`thiserror`、`clap`、`serde`、`toml`、`bincode`、`wasm-encoder`、`reqwest` 等

### 2.2 项目目录树

```
synswx/
├── Cargo.toml                    # Workspace 根配置
├── rustfmt.toml                  # Rust 格式化规则
├── .github/workflows/ci.yml      # CI：fmt / clippy / test
├── crates/
│   ├── syx-cli/                  # CLI 入口（syx 命令）
│   ├── syx-driver/             # 编译驱动、模块图、缓存、多后端
│   ├── syx-diagnostics/        # 诊断、Span、SourceFile
│   ├── syx-lexer/                # 词法分析
│   ├── syx-ast/                  # AST 定义与 Visitor trait
│   ├── syx-parser/               # 递归下降解析器
│   ├── syx-semantic/             # 语义分析与类型检查
│   ├── syx-codegen/              # VM 字节码生成
│   ├── syx-vm/                   # 栈式 VM、GC、Host、JIT 桩
│   ├── syx-stdlib/               # 内置类型与方法注册表
│   ├── syx-mir/                  # MIR 降低与优化
│   ├── syx-async/                # async/await 编译期校验与运行时桩
│   ├── syx-meta/                 # 数据类、效果系统、反射元数据
│   ├── syx-fmt/                  # 源码格式化
│   ├── syx-lint/                 # 静态 Lint 规则
│   ├── syx-doc/                  # Markdown 文档生成
│   ├── syx-debug/                # 调试会话与 DAP 桩
│   ├── syx-mir-run/              # 共享 MIR 解释器（WASM + native FFI）
│   ├── syx-runtime-core/         # Value、协议、Hasher（无 codegen）
│   ├── syx-cranelift/            # MIR → 原生机器码（Object file）
│   ├── syx-rt/                   # Native C ABI 运行时、链接 helper
│   ├── syx-llvm/                 # LLVM IR 发射（实验性）
│   └── syx-wasm-emit/            # MIR → WebAssembly 二进制
├── examples/                     # 示例 .sy 程序与 syx.toml
│   ├── hello.sy
│   ├── p0_demo.sy
│   ├── main.sy
│   ├── math.sy
│   ├── module_main.sy
│   └── syx.toml
├── docs/                         # 语言与编译器文档
└── README.md
```

### 2.3 Crate 依赖关系（简化）

```
syx-diagnostics
    ↑
syx-lexer → syx-ast → syx-parser
                ↑           ↑
                └── syx-semantic ← syx-stdlib
                        ↑
                   syx-codegen ← syx-mir
                        ↑
                     syx-vm
                        ↑
              syx-async / syx-meta
                        ↑
                   syx-driver ← syx-fmt / syx-lint / syx-doc / syx-debug
                        ↑              syx-llvm / syx-wasm-emit
                     syx-cli
```

---

## 3. 架构设计与编译流水线

### 3.1 总体架构

```
┌──────────────────────────────────────────────────────────────────┐
│                     源代码 (.sy)                                 │
└────────────────────────────┬─────────────────────────────────────┘
                             │
                             ▼
┌──────────────────────────────────────────────────────────────────┐
│  syx-lexer          词法扫描 → Token 序列                          │
└────────────────────────────┬─────────────────────────────────────┘
                             │
                             ▼
┌──────────────────────────────────────────────────────────────────┐
│  syx-parser         递归下降解析 → syx-ast::Module                │
└────────────────────────────┬─────────────────────────────────────┘
                             │
              ┌──────────────┼──────────────┐
              ▼              ▼              ▼
┌─────────────────┐ ┌──────────────┐ ┌─────────────────┐
│ syx-meta        │ │ syx-async    │ │ syx-macro 宏展开 │
│ 数据类/效果校验  │ │ async 约束   │ │ expand_module   │
└────────┬────────┘ └──────┬───────┘ └────────┬────────┘
         └─────────────────┼──────────────────┘
                           ▼
┌──────────────────────────────────────────────────────────────────┐
│  syx-semantic       类型检查 + 符号解析 + 模块导入符号合并          │
└────────────────────────────┬─────────────────────────────────────┘
                             │
              ┌──────────────┴──────────────┐
              ▼                             ▼
   ┌─────────────────────┐       ┌─────────────────────┐
   │ 默认路径（VM）       │       │ SYX_USE_MIR=1       │
   │ AST → syx-codegen   │       │ AST → syx-mir       │
   │                     │       │   → optimize      │
   │                     │       │   → syx-codegen   │
   └──────────┬──────────┘       └──────────┬──────────┘
              │                             │
              └──────────────┬──────────────┘
                             ▼
┌──────────────────────────────────────────────────────────────────┐
│  CompiledProgram（函数表 + Instruction 序列）                      │
└────────────────────────────┬─────────────────────────────────────┘
                             │
         ┌───────────────────┼───────────────────┐
         ▼                   ▼                   ▼
┌──────────────┐   ┌─────────────────┐   ┌─────────────────┐
│ syx-vm       │   │ syx-cranelift   │   │ syx-wasm-emit   │
│ 解释执行      │   │ MIR → 机器码     │   │ MIR → .wasm     │
│ （默认）      │   │ + syx-rt 链接    │   │                 │
└──────────────┘   └─────────────────┘   └─────────────────┘
         │                   │                   │
         │           syx-mir-run（共享 MIR 解释器）│
         │           syx-runtime-core（Value/协议）│
         └───────────────────┴───────────────────┘
```

### 3.2 主要编译流水线

默认用户路径以 **MIR** 为统一中端；`syx run` 再落到 VM 字节码，`--native` / `--wasm` 从同一 MIR 图出发。

#### 3.2.1 默认流水线（`syx run` → VM）

**触发**：`syx run`、`syx check`（及多数日常开发）

```
Source → Lexer → Parser → Macro / Meta / Async Pass → Semantic
       → syx_mir::lower_module → optimize
       → CodeGenerator（MIR → 字节码）
       → VmInstance::run_program（别名 WasmVM）
```

- 完整标准库与 Host Functions 可用
- 适合日常开发与行为验证

#### 3.2.2 原生流水线（`syx build --native`，**仅 LLVM**）

**触发**：`syx build --native [path]`（可选 `--object`、`--release`；需 `full-native` / 本机 LLVM 时见 UsageGuide）

```
Source → … → Semantic → MIR → optimize
       → syx_llvm（MIR → LLVM IR / object）
       → link libsyx_rt.a → app（或 --object 仅 .o）
```

- **Cranelift AOT 已移除**；CLI 不再提供 `--backend=cranelift`
- **`syx-rt`**：C ABI / 运行时；**`syx-mir-run`**：共享 MIR 解释（WASM host 等）
- 细节与 parity：[10_BackendParity.md](./10_BackendParity.zh-CN.md)

#### 3.2.3 WebAssembly 流水线

**触发**：`syx build --wasm` / `syx run --wasm`

```
Source → … → MIR → optimize
       → syx_wasm_emit::emit_wasm(MIR) → main.wasm
```

- CLI `syx run --wasm`：guest + host `syx.mir_exec`（`MirRuntime`），能力接近本机 `syx run`
- 浏览器 `syx-compiler-wasm` / `examples/wasm_web` 为**另一条**子集路径，勿与 CLI WASM 混称

---

## 4. Crate 职责详解

### 4.1 前端 Crate

| Crate | 职责 | 关键类型/API |
|-------|------|-------------|
| `syx-diagnostics` | 源码位置、Span、结构化诊断 | `SourceFile`、`SyxDiagnostic`、`SyxResult` |
| `syx-lexer` | 单遍扫描、关键字识别 | `Token`、`Lexer::scan` |
| `syx-ast` | AST 节点、访问者 trait | `Module`、`Decl`、`Expr`、`Stmt`、`ExprVisitor` |
| `syx-parser` | 递归下降 + Pratt 表达式 | `Parser::parse` |
| `syx-semantic` | 两遍分析、作用域、类型兼容 | `SemanticAnalyzer`、`TypeId`、`Scope` |

### 4.2 中端与后端 Crate

| Crate | 职责 | 关键类型/API |
|-------|------|-------------|
| `syx-mir` | AST 降低、CFG 块、优化 Pass | `MirModule`、`MirOp`、`lower_module`、`optimize` |
| `syx-codegen` | 指令选择与函数表构建 | `CodeGenerator`、`CompiledProgram`、`Instruction` |
| `syx-vm` | 栈式解释器、ManagedHeap / GC | `VmInstance`（`WasmVM` 别名）、`Value` |
| `syx-stdlib` | 内置类型/方法/Host 注册 | `HostFnId`、`MethodId`、`BUILTIN_TYPES`、manifest |
| `syx-llvm` | 原生 AOT（sole backend） | MIR → LLVM / object → `app` |
| `syx-wasm-emit` | MIR → WASM 二进制 | `emit_wasm` |
| `syx-cranelift-jit` | 实验 JIT tier-2（opt-in） | `SYX_JIT_TIER2` / Suite-1；**非** AOT |

### 4.3 编译期扩展 Crate

| Crate | 职责 | 实现深度 |
|-------|------|----------|
| `syx-meta` | 数据类元数据、Pure/IO 效果推断与校验、类型反射 | P1 可用 |
| `syx-async` | `async` 函数 `await` 点计数校验、Task 运行时桩 | P2 MVP |

### 4.4 工具链 Crate

| Crate | 职责 | 实现深度 |
|-------|------|----------|
| `syx-fmt` | AST 驱动的源码格式化 | P2 MVP |
| `syx-lint` | 命名规范、重复定义、空函数体等规则 | P2 MVP |
| `syx-doc` | 从 AST 生成 Markdown API 文档 | P2 MVP |
| `syx-debug` | 调试会话、断点表、DAP 协议桩 | P3 脚手架 |

### 4.5 编排与入口 Crate

| Crate | 职责 |
|-------|------|
| `syx-driver` | 模块图构建、缓存、多后端调度、REPL、依赖管理 |
| `syx-cli` | Clap 子命令解析，调用 `syx-driver` 公开 API |

---

## 5. 编译驱动（syx-driver）

`syx-driver` 是编译器的核心编排层，对外暴露项目级与文件级 API。

### 5.1 核心数据结构

| 类型 | 说明 |
|------|------|
| `SyxManifest` | 解析 `syx.toml`：name、version、main、dependencies |
| `SyxLockfile` | `syx.lock` 锁定依赖解析结果 |
| `ModuleUnit` | 单模块编译产物：路径、源码、导入列表、`CompiledProgram` |
| `ModuleGraph` | 模块依赖图：有序编译列表 + 模块表 |
| `CompiledProject` | 完整项目：manifest + 入口模块 + 图 |

### 5.2 主要公开 API

| 函数 | 功能 |
|------|------|
| `compile_file` / `compile_linked` | 单文件编译（含依赖模块递归） |
| `compile_project` | 按 `syx.toml` 编译整个项目 |
| `compile_with_cache` | 带 `.sy-cache` 增量缓存的编译 |
| `check_project` | 仅类型检查，不执行 |
| `run_file` / `run_project` | 编译并在 VM 中执行 |
| `build_native` / `build_native_with_options` | MIR → native `.o`（后端由 `DriverOptions.native_backend` 选择） |
| `build_native_executable` | 同上 + 链接 `syx-rt` → `app` |
| `build_wasm` | MIR → `main.wasm` |
| `repl_eval` | REPL 单行求值 |
| `format_project` / `lint_project` / `doc_project` | 工具链操作 |
| `debug_project` | 创建 `DebugSession`（DAP 断点表见 `syx-debug`） |

三后端调试与链接排查见 [12_BackendDebuggingGuide.md](./12_BackendDebuggingGuide.zh-CN.md)。
| `add_dependency` / `update_dependencies` | 依赖写入 manifest 与 lockfile |
| `reflect_type_info` | 查询 `syx-meta` 注册的类型元数据 |

### 5.3 编译编排流程

`compile_module_recursive` 执行以下步骤：

1. 规范化路径，检测循环依赖（已在图中则直接返回）
2. 读取源码 → `Parser::parse`
3. `apply_meta_passes`（数据类注册、效果校验、纯函数校验）
4. `prepare_async_module`（async/await 约束）
5. 递归编译 `import` 声明引用的模块
6. `compile_file_with_context`：合并导入符号 → 语义分析 → 代码生成
7. 写入 `ModuleGraph`

`compile_to_program` 根据 `SYX_USE_MIR` 选择 AST 直编或 MIR 路径。

---

## 6. 命令行工具（syx-cli）

`syx-cli` 基于 `clap` 派生宏，将用户命令映射到 `syx-driver` API。

### 6.1 子命令一览

| 命令 | 说明 | 对应 driver API |
|------|------|-----------------|
| `syx build [path]` | 编译文件或项目（VM 字节码） | `compile_project` / `compile_file` |
| `syx build --native [path]` | 生成 `app`（**LLVM** sole；`--object` 仅 `.o`） | `build_native_executable` / `build_native` |
| `syx build --wasm [path]` | 生成 `main.wasm` | `build_wasm` |
| `syx run [path]` | 编译并执行 | `run_project` / `run_file` |
| `syx check [path]` | 仅类型检查 | `check_project` / `compile_file` |
| `syx test [path]` | 运行 `tests/**/*.sy`（递归）或项目入口 / 单文件 | `run_file` 循环 / `run_project` |
| `syx format [path]` | 格式化入口文件 | `format_project` |
| `syx lint [path]` | 静态检查 | `lint_project` |
| `syx debug [path]` | 启动调试会话摘要 | `debug_project` |
| `syx repl` | 交互式 REPL | `repl_eval` |
| `syx add <name> <path>` | 添加依赖 | `add_dependency` |
| `syx update [path]` | 刷新 lockfile | `update_dependencies` |
| `syx doc [path]` | 生成 `docs/api.md` | `doc_project` |

### 6.2 路径解析规则

- 未指定 `path` 时默认为当前目录 `.`
- 目录：读取 `syx.toml` 的 `main` 字段作为入口
- 文件：直接作为编译单元

### 6.3 构建与安装

```bash
# 开发模式
cargo run -p syx-cli -- run examples/hello.sy

# 安装到 PATH
cargo install --path crates/syx-cli
syx run examples/hello.sy
```

---

## 7. 模块系统与依赖管理

### 7.1 模块语法

```syx
// math.sy
public func add(a: Int, b: Int) -> Int {
    return a + b
}

// main.sy
import math
func main() {
    print(math.add(1, 2))
}
```

### 7.2 模块解析策略

`resolve_module` 按以下顺序查找模块：

1. **嵌入标准库**（`enable_std=true`，默认）：`math`、`io` 等
2. **`syx.lock` 已安装依赖**：manifest 中声明的包名 → lock 中的 `path` / `.sy/vendor/` 目录
3. **导入文件同目录**相对路径：`./foo.sy`
4. **`--module-dir`** 额外搜索路径

带 `dependencies` 的项目须先 `syx install` 生成 lock，否则 **P001**。未找到模块时 **M001**。

### 7.3 可见性控制

- `public` 声明的函数可被其他模块导入
- 非 `public` 符号在跨模块访问时触发 `S005`（internal symbol access）错误
- 语义分析阶段通过 `import_exports` 合并导出符号表

### 7.4 依赖清单格式

**syx.toml：**

```toml
name = "my-app"
version = "0.1.0"
main = "main.sy"

[dependencies.math]
version = "1.0"
path = "../math"
```

**syx.lock：**

```toml
[packages.local_math]
source = "path"
version = "1.0.0"
path = "../deps_math"
```

包管理由 **`syx-pkg`** 实现：`install_project` 解析 path/git/registry 依赖并写入 lock；`load_resolved_packages` 供 driver 编译时使用。

Registry API：`GET /v1/packages/{name}`、`GET /v1/packages/{name}/{version}/download`、`PUT /v1/packages/{name}/{version}`（发布）。本地测试可用 `file:///path/to/registry`。

---

## 8. 增量编译缓存

### 8.1 缓存机制

| 项目 | 说明 |
|------|------|
| 缓存目录 | `{project_root}/.sy-cache/` |
| 缓存文件 | `{sha256(source + "syx-cache-v1")}.bin` |
| 序列化格式 | `bincode` 编码的 `CachedModuleUnit` |
| 缓存内容 | 路径、源码、导入模块列表、`CompiledProgram` |

### 8.2 缓存命中流程

```
compile_linked(path)
  → incremental_cache_path(root, path)
  → 若 .bin 存在且 bincode 反序列化成功 → 直接返回 ModuleUnit
  → 否则完整编译 → 写入缓存
```

### 8.3 失效策略

当前实现以**源码内容哈希**为键，源码变更后自动失效。尚未实现跨模块依赖变更的细粒度失效（依赖模块修改不会自动使依赖方缓存失效，需手动清理 `.sy-cache`）。

### 8.4 相关 API

- `module_hash(path)` — SHA-256 哈希
- `incremental_cache_path(root, module)` — 缓存文件路径
- `compile_with_cache(path, root)` — 显式缓存编译入口

---

## 9. Meta 与 Async 编译期 Pass

### 9.1 Meta Pass（syx-meta）

在语义分析之前由 `apply_meta_passes` 调用：

| Pass | 函数 | 行为 |
|------|------|------|
| 数据类注册 | `register_data_classes` | 扫描 `data class` 声明，写入全局 `TYPE_REGISTRY` |
| 效果推断 | `check_effects` | 推断函数 Pure/IO 副作用，含名称启发式（如 `fetch`） |
| 纯函数校验 | `validate_pure_functions` | `requires Pure` 函数体内不得含 I/O，违反报 `E001` |
| 类型反射 | `reflect_type` | 供 driver 查询已注册类型的字段元数据 |

**P3 脚手架 API**（尚未接入主流程）：

- `compile_time_eval` — 编译期求值桩
- `type_level_contains` — 类型级约束桩
- `service_wiring` — Phase 125 Service DI v6：基类型/协议型多态 inject + 字段 `@Qualifier` + `@Primary` + **`@Around` 窄 MVP**（`@Service`/`@Component` 实例方法，非 AspectJ）；M14 `Di.withRequest` / Qualifier/Primary/Profile 继承

### 9.2 Async Pass（syx-async）

在语义分析之前由 `prepare_async_module` 调用：

| 规则 | 诊断码 | 说明 |
|------|--------|------|
| 非 async 函数不得使用 `await` | `A002` | 编译期拒绝 |
| async 函数最多一个 `await` 点 | `A001` | MVP 限制 |

**运行时组件**（P2 脚手架）：

- `Runtime` — 基于 `Future` 的任务队列
- `Task<T>` — 异步任务句柄
- `blocking` / `blocking_poll` — 同步阻塞执行桩

---

## 10. 前端：词法、语法、语义

### 10.1 词法分析器（syx-lexer）

- **策略**：单遍扫描，O(1) 关键字查找
- **位置追踪**：`SourceFile::line_col` 支持行列报告
- **Token 类别**：声明关键字、控制流、访问控制、运算符、分隔符、字面量（含字符串插值）

### 10.2 语法分析器（syx-parser）

- **策略**：递归下降 + Pratt 优先级爬升
- **错误恢复**：同步到语句边界，尽可能收集多个语法错误
- **支持特性**：函数、类/结构体/枚举、协议、泛型、闭包、模式匹配、属性包装器、try-catch、import 等

### 10.3 语义分析器（syx-semantic）

**两遍分析：**

1. **符号收集**：遍历声明，建立全局符号表与导出表
2. **详细分析**：表达式/语句类型检查、访问控制、方法解析

**类型系统（`TypeId`）：**

```
Void / Numeric(NumericKind) / Bool / String
Optional<T> / Array<T> / Set<T> / Dictionary<K,V>
Named(name) / Unknown
```

平台 `Int`/`UInt` 由 `syx-stdlib::numeric::PLATFORM_INT` / `PLATFORM_UINT` 在编译 host 上解析为具体 `NumericKind`。

**作用域链**：`Scope` 嵌套 `parent` 指针，支持块级与函数级作用域。

**模块导入**：`analyze_with_imports` 接收 `(模块名, 导出符号表)` 列表，校验跨模块引用。

---

## 11. 代码生成与中间表示

### 11.1 直接 AST 代码生成

`CodeGenerator::compile_into` 遍历 AST，为每种节点发射 VM 指令：

| AST 节点 | 典型指令 |
|----------|----------|
| 二元数值表达式 | `NumericAdd/Sub/Mul/Div`（按 `NumericKind`） |
| 数值字面量 | `ConstNumeric { kind, bits }` |
| 类型转换 | `NumericCast` |
| 函数调用 | `Call`、`MethodCall` |
| 集合字面量 | `ArrayBuild`、`SetBuild`、`DictBuild` |
| 条件分支 | `JumpIfFalse`、`Jump` |
| 变量声明 | `StoreLocal` |
| return | `Return` |

### 11.2 MIR 中间表示

`syx-mir` 将 AST 降低为 SSA 风格的寄存器指令：

- **MirOp**：`ConstI32`、`LoadLocal`、`Add`、`Call`、`JumpIfFalse`、`Return` 等
- **MirBlock**：基本块 + 后继边
- **MirFunction**：函数级指令序列与入口块

**优化 Pass（`optimize`）：**

- 常量折叠
- 死代码消除
- 窥孔优化（局部指令合并）

### 11.3 后端差异

| 后端 | 输入 | 输出 | 完整度 |
|------|------|------|--------|
| VM（codegen） | AST 或 MIR | `CompiledProgram` | 完整（P0/P1） |
| LLVM | MIR | `.o` 或 IR 文本 | MVP 子集 |
| WASM | MIR | `.wasm` | MVP 子集 |

---

## 12. 虚拟机与运行时

### 12.1 执行模型

`VmInstance`（别名 `WasmVM`）采用**栈式解释执行**：

```
操作数栈 + 局部变量表 + 全局变量 + 函数表
```

示例：`a + b`

```
[] → local_get(0) → [a] → local_get(1) → [a,b] → i32_add → [a+b]
```

### 12.2 值表示（`Value`）

| 变体 | 说明 |
|------|------|
| `I32` / `F64` / `Bool` / `Str` | 基本类型 |
| `Nil` | 空值 |
| `Array` | 动态数组 |
| `Object` | 键值对象（HTTP 响应等） |
| `Iterator` | for-in 迭代状态 |

### 12.3 内存管理

| 组件 | 算法/策略 |
|------|-----------|
| `MemoryPool` | 对象池 + 空闲列表复用 |
| `Gc` | 标记-清除（简化版） |
| `WeakReference` | 弱引用桩 |

### 12.4 JIT（P3 脚手架）

`syx-vm::jit::JitCompiler`：

- 热路径计数（默认阈值 1000 次调用）
- `maybe_compile` 生成 JIT 桩字节，尚未真正编译为机器码
- `run_with_jit` 在计数达标后仍回退解释执行

---

## 13. 标准库与 Host Functions

### 13.1 内置类型（syx-stdlib）

| 类型 | 方法示例 |
|------|----------|
| `Int` / `Double` / `Bool` / `String` | `toString`、`length`、`substring` |
| `Array` | `push`、`pop`、`map`、`filter`、`len` |
| `String` | `split`、`trim`、`hasPrefix`、`lowercased`、`matches` |
| `File` / `Directory` | 通过 Host Functions |
| `HTTP` / `HTTPResponse` | `get`、`post`、`json` |

### 13.2 Host Functions 注册

VM 启动时通过 `hosts.rs` 注册，语义分析通过 `HostFnId` 枚举校验：

| Host Function | SYX API |
|---------------|---------|
| `__syx_file_read` | `File.read(path)` |
| `__syx_file_write` | `File.write(path, content)` |
| `__syx_file_exists` | `File.exists(path)` |
| `__syx_file_read_lines` | `File.readLines(path)` |
| `__syx_file_append` | `File.append(path, content)` |
| `__syx_file_delete` | `File.delete(path)` |
| `__syx_file_copy` | `File.copy(from, to)` |
| `__syx_file_move` | `File.move(from, to)` |
| `__syx_dir_create` | `Directory.create(path)` |
| `__syx_dir_list` | `Directory.list(path)` |
| `__syx_dir_exists` | `Directory.exists(path)` |
| `__syx_http_get` | `HTTP.get(url)` |
| `__syx_http_post` | `HTTP.post(url, body)` |
| `__syx_http_response_json` | `response.json()` |

HTTP 实现基于 `reqwest`（blocking + rustls）；文件 I/O 基于 `std::fs`。

### 13.3 迭代器协议

`ITERATOR_PROTOCOL` 定义 `makeIterator` / `next`；`for-in` 在 VM 层通过 `Value::Iterator` 驱动。

---

## 14. 工具链 Crate（P2/P3）

### 14.1 syx-fmt

- 解析源码为 AST，按约定风格重新输出
- 支持函数、import、全局变量、控制流语句的格式化
- `format_project` 就地覆写入口文件

### 14.2 syx-lint

内置规则示例：

| 代码 | 规则 |
|------|------|
| `L001` | 函数名应小写开头 |
| `L002` | 重复函数名 |
| `L003` | 空函数体 |
| `L004` | public 函数应声明返回类型 |
| `L005` | async 函数命名建议 |
| `L006` | 未使用的 import |

### 14.3 syx-doc

- 从 `///` 文档注释提取说明
- 为 public 函数/结构体生成 Markdown 签名块
- 输出至 `{project}/docs/api.md`

### 14.4 syx-debug

- `DebugSession`：维护 PC、函数索引、断点表
- `step` / `continue` / `stack_summary` 基础步进
- DAP（Debug Adapter Protocol）JSON 消息处理桩

---

## 15. 错误处理与诊断

### 15.1 诊断体系

基于 `miette` 实现富文本错误报告：

```
error[E0308]: type mismatch
  --> main.sy:5:10
   |
 5 |     let x: Int = "hello"
   |                  ^^^^^^^ expected `Int`, found `String`
```

### 15.2 错误类型

| 阶段 | 典型诊断码 |
|------|-----------|
| 词法 | 无效字符、未终止字符串 |
| 语法 | 期望 Token、同步点恢复 |
| 语义 | `S001` 重复定义、`S005` 内部符号访问 |
| Meta | `E001` 纯函数含 I/O |
| Async | `A001`/`A002` await 约束 |
| 模块 | `M001` 模块未找到 |
| 运行时 | VM 栈下溢、类型转换失败 |

### 15.3 错误恢复策略

- **词法**：跳过无效字符继续扫描
- **语法**：同步到语句边界
- **语义**：收集全部错误后统一返回
- **运行时**：`try-catch` 捕获，支持 `finally`（语言层）

---

## 16. 测试体系

### 16.1 测试组织

```
crates/syx-driver/tests/
├── integration.rs           # 基础集成
├── p0_integration.rs        # P0 功能回归
├── p1_integration.rs        # P1 功能回归
├── p0_strings.sy           # 字符串测试用例
├── p0_arrays.sy
├── p0_file_io.sy
├── p0_http.sy
├── p0_modules.sy
├── p1_features.sy
├── p1_null.sy
└── p1_dataclass.sy

各 crate 内 #[cfg(test)] 单元测试
```

### 16.2 P0 集成测试覆盖

| 测试 | 验证点 |
|------|--------|
| `p0_demo_compiles` | 示例项目可编译 |
| `p0_strings_run` | 字符串方法与插值 |
| `p0_arrays_run` | 数组与 for-in |
| `p0_file_io_run` | File Host Functions |
| `p0_http_compiles` | HTTP 编译通过 |
| `p0_modules_run` | 跨模块 public 导入 |
| `p0_modules_internal_rejected` | internal 符号拒绝 |
| `p0_deps_project_*` | syx.toml 依赖项目 |

### 16.3 P1 集成测试覆盖

| 测试 | 验证点 |
|------|--------|
| `p1_features_compile` | guard let、效果标注等 |
| `p1_null_run` | Optional 空安全 |
| `p1_dataclass_run` | data class 与元数据注册 |

### 16.4 基准测试

`crates/syx-driver/benches/compile_bench.rs` 提供编译性能基准（criterion 可选）。

---

## 17. 构建与 CI

### 17.1 本地构建

```bash
cargo build              # 调试构建
cargo build --release    # 发布构建
cargo test --all         # 全部测试
cargo fmt --all          # 格式化
cargo clippy --all-targets --all-features -- -D warnings
```

### 17.2 环境变量

| 变量 | 作用 |
|------|------|
| `SYX_USE_MIR=1` | 启用 MIR 优化流水线 |

### 17.3 CI 流水线

GitHub Actions（`.github/workflows/ci.yml`）：

1. `cargo fmt --all -- --check`
2. `cargo clippy --all-targets --all-features`
3. `cargo test --all`

使用 `Swatinem/rust-cache` 加速依赖缓存。

---

## 18. 附录

### 附录 A：语言特性支持列表（Rust 实现）

| 特性 | 状态 | 说明 |
|------|------|------|
| 基本类型 | ✅ | 固定宽度数值、`Int`/`UInt`（平台）、Bool、String |
| 集合 | ✅ | `Array<T>`、`Set<T>`、`[String: V]` Dictionary |
| 变量声明 | ✅ | var、let |
| 函数定义 | ✅ | func、参数、返回类型 |
| 类/结构体/枚举 | ✅ | 基础 OOP |
| 协议/扩展 | ⚠️ | 基础实现 |
| 泛型 | ⚠️ | 基础泛型 |
| 闭包 | ⚠️ | 基础闭包 |
| 属性观察器 | ⚠️ | willSet/didSet 解析 |
| 属性包装器 | ⚠️ | @ 语法解析 |
| 访问控制 | ✅ | public/internal 等 |
| 异常处理 | ✅ | try/catch/throw |
| 可选类型 | ✅ P1 | ?、!、??、guard let |
| 数组/字典 | ✅ | 基础集合 |
| 字符串插值 | ✅ P0 | `\(expr)` |
| 正则匹配 | ✅ P0 | `matches` |
| 模块 import | ✅ P0 | public 导出 |
| 文件 I/O | ✅ P0 | Host Functions |
| HTTP | ✅ P0 | GET/POST |
| for-in 迭代 | ✅ P0 | 数组、字符串 split 结果 |
| data class | ✅ P1 | 元数据注册 |
| 效果标注 | ✅ P1 | requires Pure/IO |
| async/await | ⚠️ P2 | 语法 + 校验，运行时桩 |
| MIR 优化 | ⚠️ P2 | 环境变量启用 |
| 格式化/Lint/Doc | ⚠️ P2 | MVP |
| LLVM 原生 | ⚠️ P3 | MVP 子集 |
| WASM 后端 | ⚠️ P3 | MVP 子集 |
| JIT | ❌ P3 | 仅热路径计数桩 |
| 宏系统 | ✅ M18 | `syx-macro` 固定点展开 + Service DI v9（`@Lazy` · cycle lazy-proxy · proceed · `@Bean` · M17 inherited）；见 [19-macros.md](../../books/language-guide/19-macros.md) · [ADR-120](../adr/architecture-decisions.md) |
| IDE/LSP | ❌ | 未实现 |

### 附录 B：性能指标（目标）

| 指标 | 目标 | 当前 |
|------|------|------|
| 编译速度 | 1000+ LOC/s | 待基准验证 |
| 编译内存 | < 100 MB | Rust 实现较轻量 |
| VM 启动 | < 100 ms | 满足 |
| GC 停顿 | < 10 ms | 简化 GC，小堆可满足 |

### 附录 C：路线图摘要

**短期（P1 收尾）**

- 完善空安全与 data class 运行时
- 扩展 Lint 规则与格式化覆盖

**中期（P2）**

- MIR 默认启用与更多优化 Pass
- async/await 完整运行时
- 调试器 DAP 完善

**长期（P3）**

- LLVM 后端指令全覆盖
- WASM 后端与 WASI 集成
- 真正 JIT 编译
- LSP 与 IDE 插件

---

**文档版本：** v2.0（Rust 实现）  
**最后更新：** 2026-06-14  
**维护 crate：** `syx-driver`（编排入口）
