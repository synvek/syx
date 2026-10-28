# SYX Backend Debugging Guide

[简体中文](./zh/12_BackendDebuggingGuide.zh-CN.md)

SYX can run the same source on multiple execution paths. This guide explains when to use each path, common differences, and troubleshooting steps.

## Backend selection

| Path | How to enable | When to use |
|------|---------------|-------------|
| **VM** | Default `syx run` (MIR → bytecode) | Daily development, full stdlib, fastest iteration |
| **Native (LLVM)** | `syx build --native` | Release builds, performance, no VM dependency |
| **WASM** | `syx run --wasm` / `syx build --wasm` | Sandbox / browser deployment (CLI ≠ `compiler.wasm` subset) |
| **JIT (experimental)** | `SYX_JIT_TIER2=1` + feature (see status) | Suite-1 subset; **not** AOT; off by default |

Unified MIR build: `build_unified_mir` is the single mid-end entry for native / WASM (see [docs/10](10_BackendParity.md)). **By default** public stdlib symbols are merged into MIR; disable with `SYX_STDLIB_MIR=0`.

## VS Code extension setup

1. Build the CLI: `cargo build -p syx-cli`
2. Enter `editors/vscode-syx/` and install extension dependencies: **`npm install`** (required; otherwise activation fails and debugger `type: syx` is unavailable)
3. In VS Code: **Extensions → … → Install from VSIX…** (install the `.vsix` from `npm run package`) or **Developer: Install Extension from Location…** (select the `editors/vscode-syx` directory)
4. After changing extension source or repackaging, run **Developer: Reload Window**
5. (Optional) Set `syx.languageServerPath` to the absolute path `${workspaceFolder}/target/debug/syx`; if unset, the extension defaults to `<workspace>/target/debug/syx`

The LSP **starts automatically** when you open a `.sy` file; you do not need a separate debug configuration to run `syx lsp`.

### Dot member completion

Typing `.` in a `.sy` file triggers member completion (requires a healthy LSP):

- Built-in collections: methods on `String` / `Array` / `Set` / `Dictionary` (e.g. `xs.map`, `s.split`)
- Literals and call results: `[1, 2].`, `getItems().`
- Stdlib modules: `HTTP.get(...).` → `status` / `json`; after `import Concurrency`, `Concurrency.` → `join` / `sleep`
- Member completion **does not** mix in unrelated globals (such as `print`)

Verification:

1. `cargo build -p syx-cli` and `npm install` under `editors/vscode-syx`
2. F5 → Extension Development Host, open `examples/task_demo.sy`
3. After an array variable, type `.`; expect candidates like `count`, `map` (`len` is retired; see S004)
4. If nothing appears: check **Output → SYX LSP Trace** (set `syx.trace.server` to `messages` or `verbose`)

The repo root `.vscode/launch.json` includes SYX debug configurations; **after installing the extension you can debug graphically**. The extension also provides **SYX: Debug File** as an equivalent entry.

## VS Code graphical debugging (DAP)

### Prerequisites

- Open VS Code at a **directory that contains `target/debug/syx`** (e.g. this repo root), not a single `.sy` file alone
- SYX Language extension installed (previous section)
- `cargo build -p syx-cli` succeeded and `target/debug/syx` exists

### Steps

1. Open the target `.sy` file (e.g. `examples/main.sy`)
2. Click the gutter to set breakpoints (can be set before launch; they become verified after the session starts)
3. Open **Run and Debug**, choose one of:
   - **SYX: Debug Current File** (repo `.vscode/launch.json`)
   - **SYX: Debug File** (extension `initialConfigurations`)
4. Press **F5**; first run runs `preLaunchTask` to build `syx-cli`
5. Debug toolbar: **Continue (F5)**, **Step Over (F10)**, **Step Into (F11)**, **Step Out (Shift+F11)**; use Variables / Call Stack for locals and frames

Equivalent command:

```bash
./target/debug/syx debug --dap path/to/file.sy
```

The extension spawns that binary via the `type: "syx"` debugger and speaks DAP over stdin/stdout.

### launch.json example

**You must use `type: "syx"`** so the SYX-registered debugger launches the Rust binary.  
**Do not** use `"type": "node"` — the Node debugger will treat `syx` as JavaScript and fail with `SyntaxError: Invalid or unexpected token`.

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "SYX: Debug Current File",
      "type": "syx",
      "request": "launch",
      "program": "${workspaceFolder}/target/debug/syx",
      "args": ["debug", "--dap", "${file}"],
      "cwd": "${workspaceFolder}",
      "preLaunchTask": "cargo build syx"
    },
    {
      "name": "SYX: Debug Project (examples)",
      "type": "syx",
      "request": "launch",
      "program": "${workspaceFolder}/target/debug/syx",
      "args": ["debug", "--dap", "${workspaceFolder}/examples"],
      "cwd": "${workspaceFolder}",
      "preLaunchTask": "cargo build syx"
    }
  ]
}
```

| Field | Meaning |
|------|---------|
| `type` | Must be `"syx"` (debugger contributed by the extension) |
| `program` | Path to the `syx` CLI binary |
| `args` | `["debug", "--dap", "<file or directory>"]`; `${file}` is the active editor file |
| `preLaunchTask` | Optional; maps to `cargo build syx` in `.vscode/tasks.json` |

### Common troubleshooting

| Symptom | Cause | Fix |
|------|------|------|
| Console shows `node ... ./target/debug/syx lsp` and `SyntaxError: Invalid or unexpected token` | `launch.json` uses `"type": "node"`, or an old Node debug config was selected | Use `"type": "syx"` and **SYX: Debug Current File** / **SYX: Debug File**; **do not** start LSP with a Node config |
| `Couldn't find a debug adapter descriptor for debug type 'syx'` | SYX extension failed to activate (common: VSIX without bundled deps, missing `npm install`, `require` failure on load) | Run `npm install` in `editors/vscode-syx` → **Developer: Reload Window**; or **Install Extension from Location** again; check **Help → Toggle Developer Tools** for `SYX Language` activation errors |
| Run/Debug does nothing | Workspace is not the project root; `target/debug/syx` missing | Open the repo root with `Cargo.toml` and `target/debug/syx` |
| Debug Console only shows `cargo build` warnings and `Finished dev profile` | That is **preLaunchTask** output, not an error; DAP uses stdio so **Debug Console is usually quiet** | After a successful build, confirm the **orange debug toolbar** and **CALL STACK** / **VARIABLES**; breakpoints should be solid red and verified |
| Breakpoints stay gray / never hit | Debug session did not start or DAP handshake incomplete | Confirm Run and Debug toolbar after F5; rebuild CLI and reload window |
| No LSP hover/diagnostics | Wrong `syx` path or not built | Set `syx.languageServerPath` or run `cargo build -p syx-cli` |
| Extension changes have no effect | Extension not reloaded | **Developer: Reload Window** or Install from Location again |

Bad example (**do not** use):

```json
{
  "name": "SYX Debug (DAP)",
  "type": "node",
  "program": "${workspaceFolder}/target/debug/syx",
  "args": ["debug", "--dap", "${file}"]
}
```

That makes VS Code run the Rust binary under Node.js and will always fail.

### Current capabilities and limits

SYX VS Code debugging is **VM bytecode-level DAP** (Phases 5–13, evolving), not a full Swift/LLVM debugger.

| Capability | Status |
|------|------|
| Start/stop (including Stop) | Supported (`terminate` + `terminated` event) |
| Breakpoints (source line → PC) | Supported; debug builds **force the AST path** to keep `instruction_spans` (default MIR compile has no line mapping) |
| Step Over / Into / Out | Supported (single VM instruction steps; editor line updates only where `instruction_spans` cover) |
| Variables panel | Supports `local_N` slots; values appear after stepping past the assigning instruction |
| Call Stack highlights current line | Supported (`stackTrace` returns `source.path` + line) |
| Program stdout | Not shown in Debug Console yet (may add `output` events later) |
| async/await complex control flow | Line mapping may drift; prefer breakpoints on simple statements |

If a breakpoint stays a gray hollow circle, that line has no mappable bytecode (blank line, type-only line, etc.); set breakpoints on **executable** lines such as `print(...)`, `let` assignments.

Rebuild to pick up CLI changes:

```bash
cargo build -p syx-cli
```

Then **Developer: Reload Window** in VS Code and try F5 + breakpoints on `examples/main.sy` or `crates/syx-driver/tests/p4_hof.sy`.

### Quick CLI verification

```bash
syx debug examples/main.sy          # interactive terminal debug
syx debug --dap examples/main.sy    # DAP mode (used by VS Code)
```

Without VS Code, you can verify debugging alone:

```bash
# Interactive terminal debug (bytecode level)
syx debug examples/main.sy

# DAP mode (used by VS Code)
syx debug --dap examples/main.sy
```

In DAP mode, write JSON requests with `Content-Length` headers to stdin; stdout returns the same framing. After `initialize` you should get an `initialized` event; after `configurationDone`, `stopped` with `reason: "entry"`.

## VS Code Marketplace publish draft (Phase 10)

> Draft process only; **no requirement to actually publish**. Version aligns with `syx-cli` (currently `0.4.0` / SYX 4.0).

1. **Prerequisites**
   - `cargo build --release -p syx-cli` (user machines need `syx lsp` / `syx debug --dap`)
   - Node.js 18+, `npm install` in `editors/vscode-syx/`
2. **Package**
   ```bash
   cd editors/vscode-syx
   npm run package    # vsce package → syx-language-0.4.0.vsix
   ```
3. **Local verify**
   - VS Code: **Install from VSIX…** with the generated `.vsix`
   - Open `.sy`: verify hover (types), rename, Find All References
   - F5 debug: breakpoint lines match `instruction_spans`
4. **Publish (optional)**
   ```bash
   # Requires publisher account and Personal Access Token
   npm run publish:marketplace
   ```
   - Marketplace entry: `displayName` = SYX Language, `publisher` = syx
   - README points to docs/9 usage guide and docs/12 debugging guide
5. **CI alignment**
   - `cargo test -p syx-lsp` ≥ 35
   - `cargo test -p syx-debug pc_to_line_parity`
   - Release workflow requires wasmtime + `SYX_WASI=1` E2E

## VS Code E2E (Phase 8)

1. Open the repo root; `.vscode/launch.json` has **SYX: Debug Current File** and **SYX: Debug Project (examples)** (`type: "syx"`).
2. `tasks.json` `cargo build syx` is the Debug config `preLaunchTask`.
3. Set breakpoints in a `.sy` file → F5 **SYX: Debug Current File** → verify `stackTrace` shows source lines (`instruction_spans`).
4. Watch: DAP `evaluate` with `watch:local_0` prefix.

## DAP 3.0 (Phase 7)

- **Watch:** `evaluate` expressions with a `watch:` prefix (e.g. `watch:local_0`)
- **Async frames:** `stackTrace` appends `async::<label>` frames when async is suspended
- **Source map:** `DebugSession.source_map` maps PC ↔ source line (VM path)
- Conditional breakpoints and multi-frame features: see Phase 6 below

## DAP 2.0 (Phase 6)

- Conditional breakpoints: `setBreakpoints` supports `condition` (e.g. `local_0 == "7"`, `false` to disable)
- `evaluate`: read `local_N` slots
- Multi-frame `stackTrace`: `stack_summary` includes fn/pc and stack-top values
- Launch: `.vscode/launch.json` → `"type": "syx"` + `syx debug --dap ${file}` (see “VS Code graphical debugging”)

## MIR path notes

- MIR is enabled for user modules only; embedded stdlib (`<stdlib>/…`) still uses codegen to avoid stdlib wrapper recursion.
- `compile_with_mir`: type shells (dataclass init/hash, extensions) from codegen; function bodies overridden by MIR.
- If MIR and VM output diverge, compare `crates/syx-driver/tests/p5_mir_parity.rs` first.

## Native linking

1. After changing `syx-rt` / `syx-mir-run`, rebuild the runtime:
   ```bash
   cargo build --release -p syx-rt -p syx-cli
   ```
2. Size optimization: `syx build --native --minimal` rebuilds `syx-rt --no-default-features` then links.
3. Link error `undefined symbol _syx_rt_*`: remove stale `target/*/deps/libsyx_rt-*.a`, rebuild `syx-rt` (driver picks the newest archive by mtime).

## Incremental compile cache

`.sy-cache/` under single-file test dirs may cache old bytecode. If behavior is wrong after stdlib changes:

```bash
rm -rf path/to/tests/.sy-cache
```

## Network tests

HTTP regression uses in-tree TCP mocks (`p16_arch_eco::http_fetch_e2e_vm_and_native`, `p14_native_http`); no public internet required. Optional local checks:

```bash
cargo test -p syx-driver --test p16_arch_eco http_fetch_e2e -- --test-threads=1
cargo test -p syx-driver --test p14_native_http -- --test-threads=1
```

## DAP / breakpoints

`syx debug` starts **interactive terminal debugging** (bytecode level, not IDE UI):

```bash
# Single file
syx debug examples/main.sy

# Project directory (reads syx.toml)
syx debug examples
```

At the `(debug)` prompt:

| Command | Meaning |
|------|------|
| `l` / `list [n]` | List first n VM bytecode instructions in main |
| `s` / `step` | Show next instruction without executing |
| `b <pc>` | Set breakpoint at instruction index pc (prompted during step) |
| `r` / `run` | Run to completion on the VM and print stdout |
| `q` | Quit |

IDE integration: `syx debug --dap` uses **standard DAP framing** (stdin/stdout, `Content-Length` + JSON). Implementation in `crates/syx-debug`; VS Code extension `type: "syx"` spawns the binary via `DebugAdapterExecutable`. Protocol flow:

1. Client → `initialize` → Server responds + `initialized` event
2. Client → `launch` → Server loads program
3. Client → `setBreakpoints` (source line → PC, `instruction_spans` + `line_to_pc`)
4. Client → `configurationDone` → Server sends `stopped` (`reason: "entry"`)
5. While running, `continue` / `next` / `stepIn` / `stepOut` pause with further `stopped` events

Supported commands:

| DAP command | Meaning |
|----------|------|
| `setBreakpoints` | Source line → PC mapping (`instruction_spans` + `line_to_pc`) |
| `continue` | Run to next breakpoint |
| `next` / `stepIn` | Execute one VM instruction |
| `stepOut` | Run until current function `Return` |
| `scopes` / `variables` | Structured locals (`value_description` + type names) |
| `stackTrace` | Current function name + source line |
| `configurationDone` | Configuration complete; triggers entry `stopped` |

Full `launch.json` example: see “VS Code graphical debugging” above.

For day-to-day triage:

- `syx run file.sy` + `print(...)` to inspect output
- `SYX_USE_MIR=1 syx run` to compare MIR vs VM
- `cargo test -p syx-driver --test p5_mir_parity` for parity regression
