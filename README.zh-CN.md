# SYX 编译器

[English](README.md)

**SYX**（读作 *sy-ex*，`/saɪˈɛks/`）是一门受 Swift 启发的编程语言，配套由 Rust 实现的编译器工具链。

| | |
|---|---|
| 源文件 | `.sy` |
| 项目清单 | `syx.toml` |
| 工具链 | `syx` CLI（`syx-cli`） |

> **现行公开能力：** [docs/adr/capabilities-summary.md](docs/adr/capabilities-summary.md)  
> **使用指南：** [docs/9_UsageGuide.md](docs/9_UsageGuide.md)  
> **语言书籍：** [books/](books/) · **教程：** [tutorial/](tutorial/)

## 快速开始

**环境要求：** Rust **1.85+**（见工作区 `rust-version`）。

```bash
# 构建编译器
cargo build --release -p syx-cli

# 运行程序（VM，完整标准库）
cargo run -p syx-cli -- run examples/hello.sy

# 仅类型检查
cargo run -p syx-cli -- check examples/hello.sy

# 构建项目（含 syx.toml 的目录）
cargo run -p syx-cli -- build examples/

# 生成 / 运行 Wasm
cargo run -p syx-cli -- build --wasm examples/hello.sy
cargo run -p syx-cli -- run --wasm examples/hello.sy
```

### 原生（LLVM）

LLVM 是**唯一**的原生 AOT 后端。完整原生工具链构建（Inkwell + 自带 LLD）需开启 `full-native`，并按使用指南配置本地 LLVM：

```bash
cargo build -p syx-cli --features full-native --release

./target/release/syx build --native --release ./examples/basic_demo.sy
./examples/target/release/basic_demo
```

可选 PATH（release 构建之后）：

```bash
export PATH="$PWD/target/release:$PATH"
# 或: ./scripts/syx run ./examples/basic_demo.sy
syx run ./examples/basic_demo.sy
```

## Hello, World

```sy
func main() {
    print("Hello, World")
}
```

## 仓库布局

```
crates/              编译器 crate（词法 → 驱动 → CLI、VM、MIR、LLVM、LSP 等）
packages/            嵌入 / 可导入标准库（Core builtins、Net、Text 等）
examples/            示例程序
tutorial/            引导式 `.sy` 课程
books/               语言指南与参考
docs/                编译器文档、使用指南、ADR
editors/             VS Code / Neovim / Helix 扩展
```

主要 crate：

| Crate | 作用 |
|-------|------|
| `syx-driver` | 模块图与编译流水线 |
| `syx-cli` | `syx` 命令行工具 |
| `syx-vm` / `syx-codegen` | 栈式 VM + 字节码 |
| `syx-mir` / `syx-mir-run` | 中层 IR + 共享解释器 |
| `syx-llvm` | 原生 AOT（LLVM） |
| `syx-stdlib` | 内置类型 / Host 注册表 |
| `syx-lsp` | 语言服务器 |

Core 内置类型权威源在 [`packages/core/builtins/`](packages/core/builtins/)（数值类型由 [`_templates/`](packages/core/builtins/_templates/) 生成）。

## 编译流水线

```
源码 (.sy) → Lexer → Parser → Semantic → MIR
                                         ├→ VM（syx run）
                                         ├→ LLVM 原生（syx build --native）
                                         └→ Wasm（syx build --wasm / syx run --wasm）
```

## CLI 命令

| 命令 | 说明 |
|------|------|
| `syx run [path]` | 编译并在 VM 上执行（完整标准库） |
| `syx check [path]` | 仅类型检查 |
| `syx build [path]` | 编译文件或项目 |
| `syx build --native [path]` | MIR → 原生可执行文件（**LLVM**） |
| `syx build --wasm [path]` | 生成 `main.wasm` |
| `syx run --wasm [path]` | 编译并运行 Wasm（wasmtime 宿主） |
| `syx test [path]` | 运行 `tests/` 下的测试 |

原生 / Wasm 细节见 [docs/9_UsageGuide.md](docs/9_UsageGuide.md)。

## 项目清单

在项目根目录创建 `syx.toml`：

```toml
name = "my-app"
version = "0.1.0"
main = "main.sy"
```

## 标准库要点

可导入模块位于 [`packages/`](packages/)（`import Text`、`import Net` 等）。始终在作用域内的内置包括数值类型、`String` / `Array` / `Dictionary`，以及 `print` / `readLine` 等 Host 全局（见 [`packages/core/builtins/special/globals.sy`](packages/core/builtins/special/globals.sy)）。

常见 Host 面：

- `File.read` / `File.write` / `File.exists`
- `HTTP.get` / `HTTP.post`（以及更丰富的 `Net` API）

完整模块列表：[`packages/internal/manifest.json`](packages/internal/manifest.json) · 概览：[`packages/README.md`](packages/README.md)。

## 测试

行为测试套件位于 `crates/syx-driver/tests`：

```bash
cargo test -p syx-driver --test lang --release -- --test-threads=1
cargo test -p syx-driver --test stdlib --release -- --test-threads=1
cargo test -p syx-driver --test native --release -- --test-threads=1
cargo test -p syx-driver --test concurrency --release -- --test-threads=1
```

CI 还会跑能力表新鲜度检查（`scripts/gen_ir_capabilities.py --check`）以及公开 tip 泄漏门禁。

## 贡献

- 功能与行为测试相关 PR 请提交到本公开仓库。
- 在 [docs/adr/capabilities-summary.md](docs/adr/capabilities-summary.md) 中如实记录能力边界；不要为 tip / 序词指纹增加公开 CI 钉子。

## 文档

| 文档 | 说明 |
|------|------|
| [现行能力摘要](docs/adr/capabilities-summary.md) | 已实现 / 诚实 OUT（公开 ADR 入口） |
| [完整使用指南](docs/9_UsageGuide.md) | 安装、CLI、项目、标准库、工具链 |
| [ADR 双轨说明](docs/adr/README.md) | 摘要 vs 历史全量 |
| [技术文档索引](docs/README.md) | 编译器实现与其它文档 |
| [电子书](books/README.md) | 语言指南与参考 |
| [教程](tutorial/README.md) | 动手课程 |

## 许可证

[Apache License 2.0](LICENSE)
