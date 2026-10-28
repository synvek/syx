# 编辑器

为 `.sy` 提供语法高亮与语言服务辅助开发。

| 编辑器 | 状态 | 入口 |
|--------|------|------|
| **VS Code** | 完整：TextMate 语法 + LSP + DAP | [VS Code 扩展](./vscode) |
| Neovim | 轻量 LSP 客户端（`syx lsp`） | [neovim-syx](https://github.com/synvek/syx/tree/main/editors/neovim-syx) |
| Helix | 语言配置 | [helix-syx](https://github.com/synvek/syx/tree/main/editors/helix-syx) |

语言服务与 CLI 共用同一二进制：`syx lsp`（通过 `cargo build -p syx-cli` 构建）。
