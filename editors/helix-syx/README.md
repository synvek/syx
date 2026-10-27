# SYX Helix LSP (thin)

Thin third editor client for C-07 deepen (W927 / ADR-1271): launch the same `syx lsp` stdio server used by VS Code and Neovim.

## Prerequisites

From the repository root:

```bash
cargo build -p syx-cli
```

Ensure `syx` is on `PATH`, or override `command` in [`languages.toml`](./languages.toml) to `target/debug/syx`.

## Install (drop-in)

Merge the language + language-server tables from [`languages.toml`](./languages.toml) into your Helix config, e.g.:

```text
~/.config/helix/languages.toml
```

Or during development, point Helix at this fragment and use an absolute `command`:

```toml
[language-server.syx]
command = "/path/to/synswx/target/debug/syx"
args = ["lsp"]
```

## What it does

- Registers filetype / language `syx` for `*.sy`
- Starts LSP via `language-servers = ["syx"]` with `command = "syx"` / `args = ["lsp"]`
- Relies on server capabilities (hover / completion / definition) already advertised by `syx lsp`

## Troubleshooting

1. Build the CLI: `cargo build -p syx-cli`
2. Confirm: `syx lsp` answers a Content-Length `initialize` frame (CI smoke covers this)
3. Override path: set `language-server.syx.command` to an absolute binary

## Honesty

This is a **deepen** config surface for C-07 `[~]` (after W926 first-slice). It does **not** claim C-07 `[x]`, Q-T2, marketplace, or GUI editor E2E.
