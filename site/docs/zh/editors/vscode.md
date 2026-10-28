# VS Code 扩展

**SYX Language** 扩展（`editors/vscode-syx`）为 `.sy` 提供语法高亮，以及基于 LSP 的辅助开发：补全、悬停、跳转定义、行内类型提示、格式化与 DAP 调试。

## 能力一览

| 能力 | 说明 |
|------|------|
| 语法高亮 | `.sy` 的 TextMate 语法 |
| 成员补全 | 在值后输入 `.`（例如 `Array` 上的 `xs.`） |
| import 补全 | `import` / `import ` → `IO`、`Net`、`Crypto` 等包 |
| 悬停与跳转定义 | 由 `syx lsp` 提供 |
| 行内提示（Inlay Hints） | 类型提示（含泛型）；`syx.inlayHints.enabled` 默认开启 |
| 保存时格式化 | `[syx]` 默认开启 |
| 语义高亮 | 函数、类型、装饰器等 token 着色 |
| 调试 | 调试类型 `syx` → `syx debug --dap` |

语言服务运行正常时，状态栏显示 **SYX LSP**。

## 安装（从本仓库）

```bash
# 1. 构建 CLI（语言服务在此二进制中）
cargo build -p syx-cli
# 可选更快的发布版：
# cargo build --release -p syx-cli

# 2. 扩展依赖
cd editors/vscode-syx
npm install

# 3a. 开发扩展：打开本目录（或仓库根）→ F5
# 3b. 或打包 VSIX 后安装：
npm run package
# 然后 VS Code → 扩展 → … → 从 VSIX 安装…
```

也可使用 **Developer: Install Extension from Location…**，选择已执行过 `npm install` 的 `editors/vscode-syx`。

请打开**仓库工作区根目录**，以便扩展找到 `target/debug/syx` 或 `target/release/syx`。

## 设置

| 设置项 | 默认 | 作用 |
|--------|------|------|
| `syx.languageServerPath` | `syx` | 支持 `syx lsp` 的 CLI 路径。可用绝对路径；相对 `syx` 会解析为工作区下的 `target/debug/syx` 或 `target/release/syx` |
| `syx.trace.server` | `off` | `messages` / `verbose` → 输出通道 **SYX LSP Trace** |
| `syx.inlayHints.enabled` | `true` | 开关行内类型提示 |

常用命令（命令面板）：

- **SYX: Check Language Server**
- **SYX: Restart Language Server**

## 试用补全

1. 打开任意 `.sy`（例如 `examples/hello.sy`）。
2. 在 `Array` 或 `String` 值后输入 `.` —— 应出现成员方法。
3. 输入 `import` 再空格 —— 应出现可导入包。

若无提示：确认存在 `target/debug/syx`，运行 **SYX: Check Language Server**，再执行 **Developer: Reload Window**。

## 调试

扩展已安装且 CLI 已构建时：

```json
{
  "type": "syx",
  "request": "launch",
  "name": "SYX: Debug File",
  "program": "${workspaceFolder}/target/debug/syx",
  "args": ["debug", "--dap", "${file}"],
  "cwd": "${workspaceFolder}"
}
```

在 `.sy` 中下断点后按 F5。更细的 DAP 说明见 [后端调试](/zh/compiler/backend-debugging)。

## 打包注意

请使用 `npm run package`（或带依赖的 `vsce package`）。**不要**加 `--no-dependencies` —— 会漏掉 `vscode-languageclient`，导致 LSP 无法启动。

## 源码

- 扩展：[`editors/vscode-syx`](https://github.com/synvek/syx/tree/main/editors/vscode-syx)
- 语言服务：`syx lsp`（经 `syx-cli` 调用 `crates/syx-lsp`）
