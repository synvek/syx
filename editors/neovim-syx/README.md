# SYX Neovim LSP (thin)

Thin second editor client for C-07 (W926 / ADR-1270): launch the same `syx lsp` stdio server used by VS Code.

## Prerequisites

From the repository root:

```bash
cargo build -p syx-cli
```

Ensure `syx` is on `PATH`, or point `cmd` at `target/debug/syx` below.

## Install (drop-in)

Copy or symlink this folder’s Lua into your Neovim config, e.g.:

```text
~/.config/nvim/lua/syx.lua
```

Then in `init.lua`:

```lua
require("syx").setup()
```

Or source [`syx.lua`](./syx.lua) from this tree during development:

```lua
vim.opt.runtimepath:append("/path/to/synswx/editors/neovim-syx")
require("syx").setup({
  -- Prefer workspace debug binary when developing the language:
  cmd = { vim.fn.getcwd() .. "/target/debug/syx", "lsp" },
})
```

## What it does

- Registers filetype `syx` for `*.sy`
- Starts LSP via `vim.lsp.start` with `cmd = { "syx", "lsp" }` (override with `setup({ cmd = … })`)
- Enables basic completion / hover / definition capabilities from the server

## Troubleshooting

1. Build the CLI: `cargo build -p syx-cli`
2. Confirm: `syx lsp` stays running when fed an LSP `initialize` (stdio)
3. Override path: `require("syx").setup({ cmd = { "/abs/path/syx", "lsp" } })`

## Honesty

This is a **first-slice** config surface for C-07 `[~]` (W926). Helix + initialize smoke landed in W927 deepen. It does **not** claim C-07 `[x]`, Q-T2, or marketplace.
