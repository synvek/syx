> **现状 SSOT：** [books/shared/implementation-status.md](../../books/shared/implementation-status.md)（SYX **117.0** / Phase **139** · `syx-cache-v1`）。下文优先级矩阵可能滞后；以 status 与 `architecture-decisions.md` 为准。  
> **许可证：** [Apache-2.0](../../LICENSE) · **原生 AOT：** 仅 LLVM（Cranelift AOT 已移除；JIT tier-2 为 opt-in 实验）。

# SYX 语言功能实现优先级清单

[English](../7_FeatureImplementationPriority.md)


本文档基于 **Rust 实现的 SYX 工具链**（Cargo workspace）现状与市场需求，提供功能实现优先级建议。源码扩展名为 `.sy`，示例均使用 SYX 语法。

> **编译器架构：** `Lexer → Parser → Semantic → MIR → CodeGen → VM`；`--native` 走 **MIR → LLVM → syx-rt → `app`**；`--wasm` 走 MIR → WASM emit。

---

## 实现状态图例

| 标记 | 含义 |
|------|------|
| ✅ | 已完成，有集成测试或示例可运行 |
| ⚠️ MVP | 语法/基础能力可用，功能或性能未完整 |
| ❌ | 未实现或仅预留词法/AST |

**主要 Crate：** `syx-lexer` · `syx-parser` · `syx-ast` · `syx-semantic` · `syx-codegen` · `syx-vm` · `syx-runtime-core` · `syx-stdlib` · `syx-driver` · `syx-mir` · `syx-mir-run` · `syx-rt` · `syx-llvm` · `syx-wasm-emit` · `syx-fmt` · `syx-lint` · `syx-doc` · `syx-async` · `syx-meta` · `syx-cli` · `syx-macro` · `syx-host-plugin`

---

## 优先级定义

| 级别 | 名称 | 定义 | 时间框架 |
|------|------|------|----------|
| P0 | 关键 | 缺少此功能语言无法使用 | 1-3个月 |
| P1 | 高 | 显著提升语言可用性 | 3-6个月 |
| P2 | 中 | 提升开发体验 | 6-12个月 |
| P3 | 低 | 锦上添花的功能 | 12个月+ |

---

## P0 - 关键功能

### 1. 模块化系统 ✅

**现状：** 已实现。支持 `import`、模块路径解析、`public` / `internal` / `private` 可见性、跨模块 `public` 符号导出；`internal` 跨模块访问报 **S005**。多模块项目通过 `syx.toml` + `examples/deps/` 验证。

**示例：**
```syx
// math.sy
public func add(a: Int, b: Int) -> Int {
    return a + b
}

internal func secret() -> Int {
    return 0
}

// main.sy
import math

func main() {
    print(math.add(1, 2))
}
```

**子任务：**
- [x] 模块声明语法（`import moduleName`）
- [x] 可见性控制（public / internal / private）
- [x] 模块搜索路径（同目录、`syx.toml` 依赖、相对路径）
- [x] 命名空间隔离（`module.symbol` 调用）

**实现位置：** `syx-driver`（`compile_module_recursive`、`resolve_module_path`）、`syx-semantic`（S005）

---

### 2. 字符串处理增强 ✅

**现状：** 已实现。支持插值、`split`、`trim`、`hasPrefix`、`lowercased` / `uppercased`、`replacingOccurrences`、`substring`、`length`、`matches`（字符串正则）；正则字面量 `/pattern/` 可解析。

**示例：**
```syx
func main() {
    let name = "World"
    print("Hello, \(name)!")

    let parts = "a,b,c".split(",")
    for item in parts {
        print(item)
    }

    print("  Hello  ".trim())
    print("abc123".matches("\\d+"))
}
```

**子任务：**
- [x] 字符串插值
- [x] 正则匹配（`matches` + host `__syx_regex_match`）
- [x] 常用字符串方法（见 `syx-stdlib` `STRING_METHODS`）

**实现位置：** `syx-stdlib/registry.rs`、`syx-vm/hosts.rs`

---

### 3. 文件 I/O ✅

**现状：** 已实现。`File` / `Directory` 静态方法经 Host Functions 提供完整 P0 API；配合 `try/catch` 捕获 I/O 错误。

**示例：**
```syx
func main() {
    let path = "/tmp/data.txt"
    try {
        File.write(path, "line1\nline2")
        let content = File.read(path)
        print(content)

        let lines = File.readLines(path)
        for line in lines {
            print(line)
        }

        print(File.exists(path))
        Directory.create("/tmp/mydir")
    } catch (err) {
        print("file error")
    }
}
```

**子任务：**
- [x] 读/写/追加
- [x] 存在性检查、删除、复制、移动
- [x] 目录创建、列表、存在性

**实现位置：** `syx-stdlib`（`HostFnId::File*` / `Dir*`）、`syx-vm/hosts.rs`

---

### 4. 基础网络库 ✅

**现状：** 已实现。`HTTP.get` / `HTTP.post` 经 host 调用；响应对象支持 `.json()`。网络测试默认 `#[ignore]`（需外网）。

**示例：**
```syx
func main() {
    try {
        let response = HTTP.get("https://httpbin.org/get")
        let json = response.json()
        print(json)
    } catch (err) {
        print("http error")
    }
}
```

**子任务：**
- [x] HTTP GET / POST
- [x] JSON 响应解析
- [ ] 超时、重试、自定义 Header（待扩展）

**实现位置：** `syx-stdlib`、`syx-vm/hosts.rs`

---

### 5. 迭代器协议 ⚠️ MVP

**现状：** `for-in` 已支持 `Array`、按字符遍历 `String`、以及 `File.readLines` 返回的数组。`protocol Iterator { func next() }` 可声明并通过 **S006** 校验；用户自定义 `Sequence` / 迭代器实现尚未完整。

**示例：**
```syx
func main() {
    for n in [1, 2, 3] {
        print(n)
    }
    for ch in "hi" {
        print(ch)
    }
}
```

**子任务：**
- [x] Array / String / readLines 结果遍历
- [x] Iterator 协议声明与语义检查
- [ ] 用户自定义迭代器与 `makeIterator`
- [ ] 文件按行流式迭代（当前基于 readLines 数组）

**实现位置：** `syx-codegen`（MakeIterator）、`syx-semantic`（S006）

---

## P1 - 高优先级功能

### 6. 结构化并发 ⚠️ MVP

**现状：** `async func` / `await` 语法已解析；`syx-async` 校验 async 函数内 await 使用；当前为 **单 await 点、阻塞 poll**，无 TaskGroup / 多线程调度器。

**示例：**
```syx
async func fetchLabel() -> String {
    let resp = await HTTP.get("https://example.com")
    return "done"
}
```

**子任务：**
- [x] async/await 语法解析
- [x] 非 async 函数内 await 编译错误
- [ ] Task / TaskGroup
- [ ] 多线程调度器与状态机完整 lowering

**实现位置：** `syx-async`、`syx-codegen`

---

### 7. 空安全系统 ✅

**现状：** 已实现。`T?`、`nil`、`??` 合并、`!` 强制解包（nil → **R007**）、`?.` 可选链、`if let` / `guard let` 绑定。

**示例：**
```syx
func main() {
    let a: Int? = nil
    print(a ?? 99)

    let b: Int? = 7
    print(b!)

    let s: String? = "hello"
    print(s?.hasPrefix("he"))

    if let y = b {
        print(y)
    }
}
```

**子任务：**
- [x] 可空类型与 nil
- [x] `??` / `!` / `?.`
- [x] `if let` / `guard let`

**实现位置：** `syx-semantic`、`syx-codegen`、`syx-vm`（R007）

---

### 8. 数据类（Data Class）⚠️ MVP

**现状：** `data class Point(x: Int, y: Int)` 可编译运行；生成构造逻辑、字段访问（`p.x`）及 **`.description()`** 自动描述字符串（如 `Point(x: 3, y: 4)`）。尚未自动生成 `copy()`、`==`、`hashCode`。

**示例：**
```syx
data class Point(x: Int, y: Int)

func main() {
    let p = Point(3, 4)
    print(p.x)
    print(p.y)
}
```

**子任务：**
- [x] 声明语法与构造器
- [x] 字段读写
- [ ] 自动生成 copy / equals / hashCode / description

**实现位置：** `syx-codegen`、`syx-meta`（`register_data_classes`）

---

### 9. 扩展函数 ⚠️ MVP

**现状：** `extension Type { func ... }` 已实现；实例调用 desugar 为模块级函数。当前以函数扩展为主，计算属性支持有限。

**示例：**
```syx
extension Int {
    func doubled() -> Int {
        return self + self
    }
}

func main() {
    print(5.doubled())
}
```

**子任务：**
- [x] extension 声明与 desugar
- [ ] 计算属性、泛型 extension

**实现位置：** `syx-codegen`（`extension_methods`）

---

### 10. 模式匹配增强 ⚠️ 部分

**现状：** `switch` 支持整型 case、半开范围 `0..<13`、密封类 variant 模式 `.ok(let x)`。匹配守卫（`where`）、复杂元组模式尚未完整。

**示例：**
```syx
func main() {
    let age = 25
    switch age {
    case 0..<13:
        print("child")
    case 13..<20:
        print("teen")
    default:
        print("adult")
    }
}
```

**子任务：**
- [x] 整型 / 范围 case
- [x] sealed variant case
- [ ] `where` 守卫
- [ ] 元组 / 嵌套模式

**实现位置：** `syx-parser`（`parse_switch_pattern`）、`syx-codegen`

---

### 11. 密封类（Sealed Class）⚠️ MVP

**现状：** `sealed class` 语法、variant 定义、switch 模式匹配已实现；穷尽性检查为 MVP 级别（复杂泛型场景待加强）。

**示例：**
```syx
sealed class Result {
    case ok(value: Int)
    case err(message: String)
}

func main() {
    let r = Result.ok(42)
    switch r {
    case .ok(let x):
        print(x)
    case .err(let msg):
        print(msg)
    }
}
```

**子任务：**
- [x] sealed class 语法与 codegen
- [x] variant 构造与 switch
- [ ] 编译期穷尽性证明（全场景）

**实现位置：** `syx-codegen`（`sealed_classes`）

---

### 12. 包管理器 ✅

**现状：** `syx-pkg` crate 提供 path / git / HTTP registry 三种依赖源；`syx install` 写入结构化 `syx.lock`；编译时由 lock 驱动模块解析（**P001** 缺失 lock）。

**示例：**
```toml
[dependencies.local_math]
version = "1.0.0"
path = "../deps_math"
```

**子任务：**
- [x] 本地路径依赖
- [x] lockfile 与 `syx install` / `syx add` / `syx update`
- [x] SemVer 约束解析与校验
- [x] Git URL 依赖拉取（clone 到 `.sy/vendor/`）
- [x] HTTP / file registry 客户端与 `syx publish`

**实现位置：** `syx-pkg`（解析/安装/registry）；`syx-driver`（lock 驱动 `resolve_module`）

---

## P2 - 中优先级功能

### 13. 宏系统 ✅ M15（诚实分层）

**现状（Phase 125）：** `syx-macro::expand_module` 固定点展开 freestanding / attached / external / resultBuilder / service DI；Service DI v6：**基类型/协议型多态 inject**、字段 `@Qualifier` 元数据、**`@Around` 窄拦截 MVP**（**仅** `@Service`/`@Component` 实例方法，**非** AspectJ）；M14 **`Di.withRequest`** / `@Qualifier` / `@Primary` / `@Profile` 继承。文档指纹 `macro-docs-v7:m15-service-v6-honesty`。Token 宏 / AspectJ AOP / 完整 Spring → **128+**。详见 [19-macros.md](../../books/language-guide/19-macros.md) · [ADR-119](../adr/architecture-decisions.md)。

**实现位置：** `syx-macro`（engine）、`syx-driver`（`expand_module` 薄包装）、`syx-parser`（`Stmt::MacroInvoke`）

---

### 14. Actor 模型 ✅

**现状：** `actor` 声明、mailbox 隔离、`ActorCall`/`ActorSend`（VM / MIR / WASM / LLVM）已实现；parity 见行为套件。WASM 可选 `SYX_WASM_MIR_FALLBACK=1`。

**实现位置：** `syx-codegen`、`syx-mir-run`、`syx-cranelift`、`syx-rt/bridge.rs`

---

### 15. 借用检查器（简化）⚠️ MVP

**现状（Phase 7）：** `&T` 只读借用、`&mut T` 可变借用（单活跃规则 **S011**）、move；fixture：`p6_borrow_*`、`p7_borrow_mut*.sy`。

**实现位置：** `syx-semantic`（`TypeId::Reference`、`moved_bindings`）、`syx-parser`（`&` 类型/表达式）

---

### 16. 编译器优化 ⚠️ 部分

**现状：** `syx-mir` 提供 lowering 与基础 `optimize` pass；`SYX_USE_MIR=1` 可启用 MIR→CodeGen 路径。常量折叠、死代码消除、内联等为初步实现，非生产级。

**子任务：**
- [x] MIR 中间表示
- [x] 基础 optimize pass
- [ ] 完整优化 pipeline 与 benchmark 驱动

**实现位置：** `syx-mir`、`syx-driver`（`SYX_USE_MIR`；stdlib 模块仍走 codegen，见 `docs/12_BackendDebuggingGuide.md`）

---

### 17. 增量编译 ⚠️ MVP

**现状：** 项目级 `.sy-cache/` 按模块 SHA256 + **v8** 指纹（rustc 版本、依赖树 hash、MIR op 版本）缓存 `ModuleUnit`（bincode）；`purge_stale_incremental_caches` 在依赖变更时失效。

**实现位置：** `syx-driver`（`compile_with_cache`、`incremental_cache_path`）

---

### 18. 调试支持 ✅

**现状：** `syx debug` / `syx debug --dap` 提供行号断点、`continue`/`stepIn`/`stepOut`、结构化 `variables`（`value_description`）、`stackTrace`。见 [docs/12](./12_BackendDebuggingGuide.zh-CN.md)。

**实现位置：** `syx-debug`、`syx-cli`

---

### 19. 标准库扩展 ⚠️ 部分

**现状：** Array 已实现 **`length`**、**`contains(item)`**、**`sorted()`**、**`reverse()`**、**`joined(separator)`**；**`map`/`filter`/`reduce`** 走闭包 HOF（VM/MIR/native parity，`p9_hof_closure.sy`）。新增全局 **`readLine()`** 从 stdin 读一行；范围字面量 **`(1..<10).contains(n)`** 可用。无 DateTime 等。

**示例：**
```syx
let nums = [3, 1, 2]
print(nums.contains(2))
print(nums.sorted().length)
print(nums.reverse().joined(","))
print((1..<10).contains(5))

let name = readLine()
print(name)
```

**实现位置：** `syx-stdlib`（`ARRAY_METHODS`、`HostFnId::ReadLine`）、`syx-vm/hosts.rs`

---

### 20. 工具链完善 ✅

**现状：** CLI 子命令齐全：`run` · `build` · `check` · `test` · `format` · `lint` · `doc` · `debug` · `repl` · `add` · `update`。

```bash
cargo build --release          # 构建 syx CLI
syx run examples/p0_demo.sy
syx format .
syx lint .
syx doc .                      # → docs/api.md
syx repl
syx test .
```

**子任务：**
- [x] format（`syx-fmt`）
- [x] lint（`syx-lint`，L001–L012）
- [x] doc（`syx-doc`）
- [x] test / check / repl
- [x] LSP 语言服务（SemanticAnalyzer：hover 类型、completion、definition、rename、references）

**实现位置：** `syx-cli`、`syx-driver`

---

## P3 - 低优先级功能

### 21. Native 后端（LLVM sole） ✅ / 持续加深

**价值：** 原生性能，完整 host 能力（print / HTTP / task 等）  
**现状：** `syx build --native` → MIR → **`syx-cranelift`**（默认）或 **`syx-llvm`**（`--backend=llvm`）→ `user.o` → 链接 **`syx-rt`** → **`app`**。`--object` 仅生成 `.o`。MIR 语义与 VM 对齐（`ValueEq`、`ToDescription`、浮点运算、try/catch）。Release 二进制体积随 `syx-rt` feature 变化（默认 `async-runtime`；`full` 含 HTTP）。  
**工作量：** 持续迭代 · **难度：** ⭐⭐⭐⭐

**LLVM：** 通过 `--backend=llvm` 或 `SYX_BACKEND=llvm` 启用；parity 子集，非默认 release blocker。

---

### 22. WASM 目标 ⚠️ MVP

**价值：** 浏览器 / 内置 wasmtime 运行  
**现状：** `syx build --wasm` 将链接后的 MIR **直编译**为 WASM（guest 调 `syx.mir_exec`）；`syx run --wasm` 经 wasmtime + `MirRuntime` 运行。已移除 SYXP 容器路径。  
**工作量：** 1-2个月（完善） · **难度：** ⭐⭐⭐

---

### 23. JIT 编译 ❌ / ⚠️ Stub

**现状：** `syx-vm` 中 `JitCompiler` 为热路径计数 stub，默认未启用 tiered compilation。

---

### 24. 类型级编程 ❌

**现状：** 无条件类型、映射类型等 TypeScript 风格类型运算。

---

### 25. 效果系统 ⚠️ MVP

**现状：** `requires Pure` / `requires IO` 函数注解已实现；`syx-meta` 在编译期禁止 Pure 函数内 I/O（如 `print`、File 调用）。

**示例：**
```syx
func compute(x: Int) -> Int requires Pure {
    return x * 2
}

func load() -> String requires IO {
    return File.read("data.txt")
}
```

**实现位置：** `syx-meta`（`check_effects`、`validate_pure_functions`）

---

### 26. 依赖注入框架 ⚠️ MVP

**现状（Phase 125）：** `@Service` / `@Component` 多包扫描 + registry；`@Inject` ctor + 字段 `@Autowired`；**基类型/协议型多态 inject GA**；字段 `@Qualifier` 元数据；**`@Around` 方法拦截 MVP**（**仅** `@Service`/`@Component` 实例方法，**非** AspectJ）；prototype + **request scope** + Qualifier/Primary/Profile + `Di.withRequest`。E2E：`examples/service_di/` · `p125_service_di_v6.rs` · `lsp_phase125_real`。Token 宏 / 完整 AOP → **128+**。见 [19-macros.md](../../books/language-guide/19-macros.md) · [ADR-119](../adr/architecture-decisions.md)。

---

### 27. 反射系统 ⚠️ MVP

**现状：** Rust 侧 `syx_driver::reflect_type_info` / `syx-meta::reflect_type` 提供类型元数据查询 stub；源码内 `reflect(T.self)` 为脚手架。

---

### 28. 元编程 ❌ / ⚠️ 脚手架

**现状：** `#compileTime { }` 块可解析，无编译期代码生成执行。

---

## 功能依赖关系图

```
核心语言功能（P0  largely ✅）
├── 模块化系统 ✅ ──────┬──→ 包管理器 ⚠️
├── 字符串增强 ✅ ──────┤
├── 文件 I/O ✅ ────────┼──→ 标准库扩展 ⚠️
├── 网络库 ✅ ──────────┤
└── 迭代器协议 ⚠️ ──────┘

类型系统增强（P1 部分 ✅）
├── 空安全 ✅ ──────────┬──→ 数据类 ⚠️
├── 数据类 ⚠️ ──────────┤
├── 密封类 ⚠️ ──────────┤
└── 扩展函数 ⚠️ ────────┘

高级特性
├── 结构化并发 ✅ ──────┬──→ Actor ✅
├── 借用检查 ❌ ────────┤
└── 宏系统 ❌ ──────────┘

工具链与后端
├── 包管理器 ✅ ────────┬──→ LSP ✅
├── fmt/lint/doc ✅ ────┤
├── 增量编译 ✅ ────────┤
├── MIR/优化 ✅ ────────┤
├── Cranelift/WASM ✅ ──┤
├── unified MIR ✅ ─────┤
└── 调试 DAP ✅ ────────┘
```

---

## 当前进度与后续计划

### 已完成里程碑（Rust 工具链）

| 领域 | 状态 |
|------|------|
| P0 模块 / 字符串 / File / HTTP / URL | ✅ |
| P0 脚本 stdlib（Env/Path/Time） | ✅ |
| P1 空安全 / try-catch | ✅ |
| P1 data class / extension / sealed | ⚠️ MVP（copy/eq/hash 合成） |
| P1 async/await + Task 语法 | ⚠️ MVP（状态机挂起） |
| P1 包管理（path + git + registry） | ✅ MVP |
| P2 增量缓存（v8 依赖树失效） | ✅ |
| P2 LSP / 调试 DAP | ✅ GA（Phase 11：≥45 LSP 测试、code actions/semantic tokens、DAP async 栈帧） |
| P3 native/wasm 后端 | ✅ parity-gate |
| P3 unified MIR + registry fixture CI | ✅ |
| P3 effects Pure/IO | ⚠️ MVP |

### 下一阶段建议

**短期（Phase 6+）：**
- 借用检查（简化版）
- 宏展开与元编程
- LLVM 作为默认 native 后端
- GC intrinsics 三后端

**已完成（Phase 5 / SYX 1.0）：**
- Actor / Hasher / try-catch 三后端无默认 MIR 回退
- enum 关联值 + dataclass eq/copy/hash parity
- async 公平调度 fixture（`p10_async_interleave`）
- SemanticAnalyzer 驱动 LSP；DAP stepOut + 结构化 locals
- `build_unified_mir` 收敛 + HTTP mock / registry PR CI

**长期：**
- 借用检查（简化版）
- 宏展开与元编程
- JIT / 完整优化 pipeline

---

## 资源需求估算

### 人力需求（剩余工作）

| 阶段 | 重点 | 预估 |
|------|------|------|
| MVP 完善 | data class、Array HOF、async | 2-3 人月 |
| 生产就绪 | Actor、LSP、后端 host 桥接 | 6-9 人月 |
| 高级特性 | 借用、宏、JIT | 12+ 人月 |

### 技能要求

- 编译器前端（Rust、类型系统、MIR）
- 运行时（VM、Host Functions、async）
- 工具链（CLI、LSP、调试协议）
- 后端（LLVM、WASM emit）

---

## Phase 33 → Phase 34 scaffold vs 生产

| 领域 | Phase 33 | Phase 34 目标 | 环境门控 |
|------|----------|---------------|----------|
| LLVM / inkwell | ADR-042 默认 GA；text IR bytes | **ADR-044 inkwell Prod GA** | `SYX_LLVM_STRICT_GREEN_WEEKS≥91` |
| LLVM AOT | Phase 8 VM/native GC | Phase 9 **统一** VM/WASM/native GC **直出 IR** | `SYX_LLVM_AOT_PHASE9` |
| 统一 GC | VM/native 原子计数 scaffold | **ADR-045 三后端统一 generational GC** | `SYX_UNIFIED_GC_LTS=1` |
| LLVM 性能 | `p33_llvm_perf_gate` parity-gate 正式 gate | **`p34_llvm_perf_gate` perf release GA** | `llvm_perf_green_seasons()≥5` |
| dist Win pipe | WinAPI scaffold | **Named Pipe prod default** | `SYX_DIST_ACTOR_WIN_PIPE_PROD=1` |
| dist TLS | 前缀/链长校验 | **rustls crate prod default on** | `SYX_DIST_ACTOR_RUSTLS_CRATE_PROD=1` |
| Registry CDN | Phase 14 边缘读 opt-in | Phase 15 **全球 CDN 多活写** E2E | `registry_cdn_phase15_enabled()` |
| GlobalActor | v13 跨 org 联邦 LB scaffold | v14 **全球多活 CDN 路由** | `syx_rt_global_actor_v14` |
| Net | setsockopt unix stub | **全平台 setsockopt prod** | ADR-032 Phase 34 |
| borrow | Coq/Lean 字符串导出 scaffold | **机器检验证明 prod** | `borrow_regions_coq_lean_proof_prod_enabled()` |
| DateTime | MVP 3 fn | **DateTime v2** ISO8601 + 时区 | `lib/datetime.sy` |
| typealias | 未实现 | **typealias MVP**（可选） | `p34_typealias_mvp.sy` |
| IDE 路由图 | multi-org 联邦 live v4 | **multi-region 联邦 live v5** | VS Code 2.7.0 |

## Phase 32 → Phase 33 scaffold vs 生产

| 领域 | Phase 32 | Phase 33 目标 | 环境门控 |
|------|----------|---------------|----------|
| LLVM / inkwell | inkwell candidate opt-in | **ADR-042 inkwell 默认 GA** | `SYX_LLVM_STRICT_GREEN_WEEKS≥78` |
| LLVM AOT | Phase 7 guest/WASM GC | Phase 8 VM/native GC **直出 IR** | `SYX_LLVM_AOT_PHASE8` |
| Cranelift GC | **⚡** | **GC intrinsics ✅** | `syx_rt_cranelift_gc_parity` |
| LLVM 性能 | `p32_llvm_perf_gate` CI gate 候选 | **`p33_llvm_perf_gate` parity-gate 正式 gate** | `llvm_perf_green_seasons()≥4` |
| WASI GC | remembered set guest prod opt-in | **guest prod 默认 on** + VM `--run` 联动 | `SYX_WASI_GC_PHASE33=1`（ADR-043） |
| VM/native GC | **无** | **remembered set prod** | `SYX_VM_NATIVE_GC_PHASE33=1` |
| Net stream | socket window 内存计数 | **setsockopt 真路径** prod | ADR-032 Phase 33 |
| Registry CDN | Phase 13 offline install scaffold | Phase 14 **CDN 边缘缓存** E2E | `registry_cdn_phase14_enabled()` |
| GlobalActor | v12 动态权重 scaffold | v13 **跨 org 联邦 LB** + endpoint 摘除 | `syx_rt_global_actor_v13` |
| borrow | 机器证明 harness | **Coq/Lean 外部证明 scaffold** | `borrow_regions_coq_lean_export_enabled()` |
| dist Actor | 文件 pipe + pin 前缀 | **真 Named Pipe API** + **rustls crate** E2E | `SYX_DIST_ACTOR_WIN_PIPE_PROD=1` |
| IDE 路由图 | multi-repo 联邦 live v3 | **multi-org 联邦 live v4** | VS Code 2.6.0 |

## Phase 31 → Phase 32 scaffold vs 生产

| 领域 | Phase 31 | Phase 32 目标 | 环境门控 |
|------|----------|---------------|----------|
| LLVM release | ADR-038 perf 候选 | **ADR-040 release blocker GA** | `SYX_LLVM_STRICT_GREEN_WEEKS≥65` |
| LLVM AOT | Phase 6 + exec_op **→ 0%** | Phase 7 GC intrinsics 全路径 | `SYX_LLVM_AOT_PHASE7` |
| LLVM 性能 | nightly bench 非 gate | **`p32_llvm_perf_gate` CI gate 候选** | `llvm_perf_green_seasons()≥3` |
| WASI GC | generational relocating | **remembered set full generational** | `SYX_WASI_GC_PHASE32=1`（ADR-041） |
| Net stream | flow control 计数 | **true socket window** prod | ADR-032 Phase 32 |
| Registry CDN | Phase 12 install 签名 | Phase 13 **offline install** E2E | `registry_cdn_phase13_enabled()` |
| GlobalActor | v11 加权 LB | v12 **动态权重** + 跨 region 探针 | `syx_rt_global_actor_v12` |
| borrow | 形式化验证 scaffold | **机器证明 harness** | `borrow_regions_machine_proof_enabled()` |
| dist Actor | 文件 pipe + pin 前缀 | **真 Named Pipe API** + **rustls crate** E2E | `SYX_DIST_ACTOR_WIN_PIPE_PROD=1` |
| IDE 路由图 | 跨 workspace 联邦 v2 | **多 repo 联邦 live v3** | VS Code 2.5.0 |

## Phase 30 → Phase 31 scaffold vs 生产

| 领域 | Phase 30 | Phase 31 目标 | 环境门控 |
|------|----------|---------------|----------|
| LLVM AOT | Phase 5；GC intrinsics ❌ | Phase 6 + exec_op **→ 0%** | `SYX_LLVM_AOT_PHASE6` |
| LLVM 性能 | nightly bench 非 gate | ADR-038 **perf release 候选** | `SYX_LLVM_STRICT_GREEN_WEEKS≥52` |
| WASI GC | relocating prod | **generational relocating** | `SYX_WASI_GC_PHASE31=1`（ADR-039） |
| Net stream | ring buffer 计数 | socket flow control prod | ADR-032 Phase 31 |
| Registry CDN | Phase 11 自动 URL | Phase 12 install 签名 E2E | `registry_cdn_phase12_enabled()` |
| GlobalActor | v10 三区域 | v11 加权 LB + failover | `syx_rt_global_actor_v11` |
| borrow | 语言完整 | 形式化验证 scaffold | `borrow_regions_formal_verify_enabled()` |
| IDE 路由图 | 多进程 live | 跨 workspace 联邦 | VS Code 2.4.0 |

## Phase 29 → Phase 30 scaffold vs 生产

| 领域 | Phase 29 | Phase 30 目标 | 环境门控 |
|------|----------|---------------|----------|
| LLVM AOT | Phase 4；Host I/O/JSON ❌ | Phase 5 + exec_op **< 1%** | `SYX_LLVM_AOT_PHASE5` |
| LLVM CLI 默认 | ADR-034 parity-gate only | ADR-036 CLI **默认 llvm** | `SYX_LLVM_STRICT_GREEN_WEEKS≥39` |
| WASI GC | relocating MVP | **relocating prod** | `SYX_WASI_GC_PHASE30=1`（ADR-037） |
| Net stream | handle + 字符串 backpressure | ring buffer 真背压 prod | ADR-032 Phase 30 |
| Registry CDN | Phase 10 签名 URL | Phase 11 自动 failover | `registry_cdn_phase11_enabled()` |
| GlobalActor | v9 双区域 | v10 全球多活 LB | `syx_rt_global_actor_v10` |
| borrow | SE-0461 100% | 语言完整 | — |
| IDE 路由图 | 双向 sync | 多进程 live | VS Code 2.3.0 |

## Phase 28 → Phase 29 scaffold vs 生产

| 领域 | Phase 28 | Phase 29 目标 | 环境门控 |
|------|----------|---------------|----------|
| LLVM AOT | Phase 3；enum/witness ❌ | Phase 4 + exec_op **< 5%** | `SYX_LLVM_AOT_PHASE4` |
| LLVM 默认后端 | ADR-033 候选 | ADR-034 parity-gate **正式 on** | `SYX_LLVM_STRICT_GREEN_WEEKS≥26` |
| WASI GC | 增量 sweep | **relocating MVP** | `SYX_WASI_GC_PHASE29=1` |
| Net stream | 背压/handle scaffold | handle + 真 backpressure prod | ADR-032 Phase 29 |
| Registry CDN | Phase 9 failover | Phase 10 签名 E2E | `registry_cdn_phase10_enabled()` |
| GlobalActor | v8 单区域 | v9 跨区域多活 | `syx_rt_global_actor_v9` |
| Result | 泛型 MVP | 全量 mono | — |
| borrow | SE-0461 子集 | SE-0461 100% | — |
| IDE 路由图 | LSP live | 双向 sync | VS Code 2.2.0 |

## Phase 27 → Phase 28 scaffold vs 生产

| 领域 | Phase 27 | Phase 28 目标 | 环境门控 |
|------|----------|---------------|----------|
| dist Actor | UDS gate + TLS 内存队列 | UDS prod + TLS 真连接 | `SYX_DIST_ACTOR_UDS=1` / `SYX_DIST_ACTOR_TLS=1` |
| WASI GC | Phase 27 评估 gate | guest **默认 on** prod | `SYX_WASI_GC_PHASE28=1` |
| Registry CDN | Phase 8 静态 URL 列表 | Phase 9 failover + health probe | `registry_cdn_phase9_enabled()` |
| LLVM AOT | AOT Phase 2 + exec_op 混合 | AOT Phase 3 + ADR-033 默认后端 | `SYX_LLVM_AOT_PHASE3` |
| Net/WSS | stream smoke + WSS mock | 背压 + TLS + WSS prod | `SYX_NET_STREAM_TLS=1` |
| Result | ADR-031 文档 + LSP | parser + mono 真泛型 | — |
| IDE 路由图 | mock webview | LSP live + navigate | `syx/actorRoutes` |

## Phase 26 → Phase 27 scaffold vs 生产

| 领域 | Phase 26 | Phase 27 目标 | 环境门控 |
|------|----------|---------------|----------|
| dist Actor | 进程内 `DIST_PROCESS_ROUTES` scaffold | 真 UDS/pipe + TLS MVP | `dist_actor_uds_enabled()` / `dist_actor_tls_enabled()` |
| WASI GC | Phase 26 guest 默认 off | 默认 on **评估** | `SYX_WASI_GC_PHASE27=1` |
| Registry CDN | Phase 7 单区域 | Phase 8 多区域 failover | `SYX_REGISTRY_CDN_PHASE8=1` |
| LLVM AOT | ADR-028 MVP | AOT Phase 2 + ADR-029 release 默认 on | `SYX_LLVM_AOT_PHASE2` |
| Net | read/write smoke | stream MVP（ADR-030） | `SYX_NET_TCP_MVP=1` |
| Result | wire API | 泛型语法 MVP 或 ADR-031 文档 | — |
| IDE 路由图 | 无 webview | VS Code panel spike | `editors/vscode-syx` |

---

## 风险评估

| 风险 | 概率 | 影响 | 缓解措施 |
|------|------|------|----------|
| MVP 与文档不同步 | 中 | 中 | 以 `crates/syx-driver/tests/*.sy` 与集成测试为准 |
| Native/WASM 与 VM 能力分裂 | 高 | 中 | 文档明确 `--native`/`--wasm` 子集；日常用 `syx run` |
| async 语义过于简化 | 中 | 高 | 先文档标注 MVP，再迭代调度器 |
| 借用检查过于复杂 | 高 | 高 | 参考 Swift 所有权模型，非完整 Rust borrow |
| 社区参与度低 | 中 | 中 | 完善 UsageGuide / QuickReference，降低贡献门槛 |

---

**文档版本：** 2.8（Phase 34 Unified GC LTS + Inkwell Prod GA）  
**最后更新：** 2026-06-22  
**验证方式：** `cargo test -p syx-driver`（P0/P1 集成测试 + `p34_*` parity-gate）
