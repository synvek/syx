# VS Code extension

The **SYX Language** extension (`editors/vscode-syx`) gives `.sy` files syntax highlighting and LSP-powered assistance: completion, hover, go-to-definition, inlay hints, formatting, and DAP debugging.

## What you get

| Feature | Notes |
|---------|--------|
| Syntax highlighting | TextMate grammar for `.sy` |
| Member completion | Type `.` after values (e.g. `xs.` on an `Array`) |
| Import completion | `import` / `import ` → packages such as `IO`, `Net`, `Crypto` |
| Hover & go-to-definition | Via `syx lsp` |
| Inlay hints | Type hints (including generics); `syx.inlayHints.enabled` default **on** |
| Format on save | Enabled for `[syx]` by default |
| Semantic highlighting | Token colors for functions, types, decorators, … |
| Debugging | Debug type `syx` → `syx debug --dap` |

Status bar shows **SYX LSP** when the language server is running.

## Install (from this repository)

```bash
# 1. Build the CLI (language server lives here)
cargo build -p syx-cli
# optional faster binary:
# cargo build --release -p syx-cli

# 2. Extension dependencies
cd editors/vscode-syx
npm install

# 3a. Develop / debug the extension: open this folder (or repo root) → F5
# 3b. Or package a VSIX and install it:
npm run package
# then VS Code → Extensions → … → Install from VSIX…
```

You can also use **Developer: Install Extension from Location…** and select `editors/vscode-syx` (after `npm install`).

Open the **repository workspace root** so the extension can find `target/debug/syx` or `target/release/syx`.

## Settings

| Setting | Default | Purpose |
|---------|---------|---------|
| `syx.languageServerPath` | `syx` | Path to the CLI that supports `syx lsp`. Absolute paths work; relative `syx` resolves to `target/debug/syx` or `target/release/syx` under the workspace |
| `syx.trace.server` | `off` | `messages` / `verbose` → Output channel **SYX LSP Trace** |
| `syx.inlayHints.enabled` | `true` | Toggle inlay hints |

Useful commands (Command Palette):

- **SYX: Check Language Server**
- **SYX: Restart Language Server**

## Try completion

1. Open any `.sy` file (for example `examples/hello.sy`).
2. After an `Array` or `String` value, type `.` — member methods should appear.
3. Type `import` followed by a space — importable packages should appear.

If nothing shows up: confirm `target/debug/syx` exists, run **SYX: Check Language Server**, then **Developer: Reload Window**.

## Debugging

With the extension installed and the CLI built:

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

Set breakpoints in `.sy` and press F5. Deeper DAP notes: [Backend Debugging](/compiler/backend-debugging).

## Packaging note

Use `npm run package` (or `vsce package` **with** dependencies). Do **not** pass `--no-dependencies` — that omits `vscode-languageclient` and the LSP will not start.

## Source

- Extension: [`editors/vscode-syx`](https://github.com/synvek/syx/tree/main/editors/vscode-syx)
- Language server: `syx lsp` (`crates/syx-lsp` via `syx-cli`)
