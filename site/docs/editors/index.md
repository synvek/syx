# Editors

Develop `.sy` with syntax highlighting and language-server assistance.

| Editor | Status | Entry |
|--------|--------|-------|
| **VS Code** | Full: TextMate grammar + LSP + DAP | [VS Code extension](./vscode) |
| Neovim | Thin LSP client (`syx lsp`) | [neovim-syx](https://github.com/synvek/syx/tree/main/editors/neovim-syx) |
| Helix | Language config | [helix-syx](https://github.com/synvek/syx/tree/main/editors/helix-syx) |

The language server is the same binary used by the CLI: `syx lsp` (built with `cargo build -p syx-cli`).
