# 快速入门

**实现状态：** **已实现**（含 `async` 入口、`syx run` / `syx build` / `syx run --wasm` / `syx build --native` 全路径）

本章从零开始：编写第一个 SYX 程序、用 CLI 跑起来、读懂错误、扩展成多文件项目。

## Hello, World

最小可运行程序（来源 `examples/hello.sy`）：

```syx
func main() {
    print("Hello, World")
}
```

| 元素 | 说明 |
|------|------|
| `func main()` | **唯一** 程序入口；可省略（`syx run` 会自动插入 `main`），但项目根入口必须存在 |
| `print(…)` | 语言全局 Host 输出（写法像函数；权威签名见 [`packages/core/builtins/special/globals.sy`](../../packages/core/builtins/special/globals.sy)） |
| `{ … }` | 语句块 |

SYX **不允许** 顶层可执行语句；所有逻辑都必须在 `func` 内（`main` 或其他函数）。这是和 Python、Ruby 的主要区别。

## 安装与构建

### 方式 A：从源码构建（推荐日常）

```bash
git clone <repo-url> synswx
cd synswx

# 调试版（启动慢，约 5–10 秒）
cargo build

# 发布版（启动 < 0.3 秒；推荐日常）
cargo build --release
```

发布版二进制在 `target/release/syx`。把它加入 `PATH`：

```bash
export PATH="$PWD/target/release:$PATH"
syx --version
```

### 方式 B：单次运行（无需构建）

```bash
cargo run -p syx-cli -- run examples/hello.sy
```

Cargo 会编译 `syx-cli` 并立刻执行。CI / smoke test 常用；日常推荐方式 A。

### 方式 C：`cargo install`（安装到 `~/.cargo/bin`）

```bash
cargo install --path crates/syx-cli
syx run examples/hello.sy
```

### 系统要求

| 组件 | 版本 |
|------|------|
| Rust | 1.80+ |
| Cargo | 随 Rust 安装 |
| clang / 系统链接器 | `syx build --native` 需要 |
| LLVM 18+（可选） | `--backend=llvm` 直出 LLVM IR（默认 `--backend=llvm` 仍依赖 `syx-rt` 静态库） |

## 三种运行方式

| 场景 | 命令 | 产出 | 适用 |
|------|------|------|------|
| **日常开发 / 调试** | `syx run program.sy` | 无文件，直接执行 | 完整 P0 能力（File、HTTP、模块、宏） |
| **Native 编译** | `syx build --native program.sy` | `./program` 可执行文件 | 生产部署、benchmark、C ABI 集成 |
| **WASM** | `syx run --wasm program.sy` | 内存中运行 | 跨平台分发、嵌入宿主 |

详见 [docs/9_UsageGuide.md § 编译与后端](../../docs/9_UsageGuide.md#编译与后端) 与 [第 31 章 SIMD](../../docs/adr/architecture-decisions.md) 中 WASM guest 路径说明。

## 你的第一个程序：变量与字符串插值

```syx
func main() {
    let name = "SYX"
    var count = 0
    count = count + 1
    print("Hello, \(name)! count=\(count)")
}
```

输出：

```
Hello, SYX! count=1
```

| 关键字 | 含义 | 重新赋值 |
|--------|------|----------|
| `let` | 不可变绑定 | ❌ 编译错误 **S003** |
| `var` | 可变绑定 | ✅ |

字符串插值 `"\(expr)"` 在编译期被解析为 `StringConcat` 表达式；`\(name)` 内任意表达式合法（包括函数调用）。详见 [第 4 章 字符串和字符](../../docs/adr/architecture-decisions.md)。

## 多文件项目

SYX 鼓励把多模块放进 `syx.toml` 项目：

```
my-app/
├── syx.toml
├── main.sy
└── lib/
    └── math.sy
```

`syx.toml`：

```toml
name = "my-app"
version = "0.1.0"
main = "main.sy"

[dependencies]
# 路径依赖（指向 lib/math.sy）
math = "lib/math.sy"
```

`lib/math.sy`：

```syx
public func add(a: Int, b: Int) -> Int {
    return a + b
}
```

`main.sy`：

```syx
import math

func main() {
    print("3 + 4 = \(math.add(3, 4))")
}
```

运行：

```bash
cd my-app
syx run .
# → 3 + 4 = 7
```

跨模块仅 `public` 符号可见（详见 [第 28 章 访问控制](../../docs/adr/architecture-decisions.md) 与 [第 29 章 模块与包](../../docs/adr/architecture-decisions.md)）。

## async 入口

`func main` 可声明 `async`，调用 `await`：

```syx
import Net

async func main() {
    let response = await Net.get("https://example.com")
    print(response.status)
}
```

> **注意：** `await` 仅在 `async` 函数体内合法（编译错误 S020）。非 `async` 函数使用 `await` → 编译错误。

## 常见错误与排查

| 现象 | 原因 | 解决 |
|------|------|------|
| `syx: command not found` | 未构建/未加入 PATH | `cargo build --release` + `export PATH=$PWD/target/release:$PATH` |
| `P001: parse error at line N` | 语法错误 | 查看上面一行的 `^^^^^` 高亮 |
| `S002: undefined identifier 'x'` | 用了未声明的标识符 | 检查 import / 拼写 / 作用域 |
| `S003: cannot assign to let` | 给 `let` 绑定重新赋值 | 改 `var` |
| `S004: type mismatch` | 类型不匹配 | 显式标注或插入转换 |
| `R007: force unwrap of nil` | `!` 解包了一个 `Optional` | 用 `if let` / `guard let` / `??` |
| 网络测试 `Http.get` 超时 | 沙箱网络隔离 | 离线或设置 `SYX_NET_ALLOW=*`（企业版） |

完整诊断码见 [附录 B 诊断代码索引](appendix-b-diagnostic-index.md) 与 [docs/9_UsageGuide.md § 诊断代码](../../docs/9_UsageGuide.md#诊断代码)。

## 编译期与运行期检查

`syx run` 默认走完整流水线：lex → parse → semantic → macro → MIR → VM。常用 flag：

| Flag | 行为 |
|------|------|
| `syx check program.sy` | 仅类型检查，不执行 |
| `syx emit program.sy --format syxbc` | 输出栈 VM 字节码文本（人类可读） |
| `syx emit program.sy --format mir` | 输出统一 MIR 文本 |
| `syx emit program.sy --format wasm -o out.wasm` | 输出 WASM 二进制（与 `--wasm` 行为不同；详见 [docs/9_UsageGuide.md](../../docs/9_UsageGuide.md#字节码与-ir-导出--syx-emit)） |
| `syx emit . --format all -o emit/` | 一次导出六种 IR（`syxbc` / `mir` / `wasm` / `wat` / `llvm` / `clif`） |
| `syx doc .` | 从 `///` 文档注释生成 `docs/api.md` |
| `syx test .` | 运行 `tests/**/*.sy`（见 [第 34 章 单元测试](34-testing.md)） |
| `syx lint .` | 静态检查（见 [docs/9_UsageGuide.md § 静态检查](../../docs/9_UsageGuide.md#静态检查--syx-lint)） |
| `syx format .` | 格式化入口文件 |
| `syx repl` | 交互式 REPL（Ctrl-D 退出） |

## 下一步

| 你想… | 跳到 |
|------|------|
| 写更复杂的数据 | [第 5 章 集合类型](../../docs/adr/architecture-decisions.md) |
| 写自己的类型 | [第 10 章 结构体和类](../../docs/adr/architecture-decisions.md) |
| 写 async / 并发 | [第 18 章 并发](../../docs/adr/architecture-decisions.md) |
| 写宏 / DI | [第 19 章 宏](../../docs/adr/architecture-decisions.md) |
| 查精确语法 | [语言参考](../language-reference/00-about-this-reference.md) |
| 查某个特性是否 GA | [实现状态总表](../shared/implementation-status.md) |

## 与 Swift 的差异

| 点 | SYX |
|----|-----|
| 入口 | 必须 `func main()`（sync 或 `async`） |
| `print` | Host 全局（签名见 `globals.sy`；写法 `print(x)`） |
| 字符串插值 | `"\(expr)"` 一致 |
| 顶层代码 | 禁止；逻辑必须在 `func` 内 |

---

**下一章** → [基础](../../docs/adr/architecture-decisions.md)

**参见** → [完整使用指南](../../docs/9_UsageGuide.md) · [语言参考：声明](../language-reference/05-declarations.md) · [附录 A 实现状态矩阵](appendix-a-implementation-matrix.md)
