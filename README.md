# SYX Compiler

[简体中文](README.zh-CN.md)

**SYX** (pronounced *sy-ex*, `/saɪˈɛks/`) is a Swift-inspired programming language with a Rust-implemented compiler toolchain.

| | |
|---|---|
| Source files | `.sy` |
| Project manifest | `syx.toml` |
| Toolchain | `syx` CLI (`syx-cli`) |

> **Public capabilities:** [docs/adr/capabilities-summary.md](docs/adr/capabilities-summary.md)  
> **Usage guide:** [docs/9_UsageGuide.md](docs/9_UsageGuide.md)  
> **Language books:** [books/](books/) · **Tutorial:** [tutorial/](tutorial/)

## Quick start

**Requirements:** Rust **1.85+** (see workspace `rust-version`).

```bash
# Build the compiler
cargo build --release -p syx-cli

# Run a program (VM, full stdlib)
cargo run -p syx-cli -- run examples/hello.sy

# Type-check only
cargo run -p syx-cli -- check examples/hello.sy

# Build a project (directory with syx.toml)
cargo run -p syx-cli -- build examples/

# Emit / run Wasm
cargo run -p syx-cli -- build --wasm examples/hello.sy
cargo run -p syx-cli -- run --wasm examples/hello.sy
```

### Native (LLVM)

LLVM is the **sole** native AOT backend. For a full native toolchain build (Inkwell + vendored LLD), enable `full-native` and point at a local LLVM install as documented in the usage guide:

```bash
cargo build -p syx-cli --features full-native --release

./target/release/syx build --native --release ./examples/basic_demo.sy
./examples/target/release/basic_demo
```

Optional PATH helper (after a release build):

```bash
export PATH="$PWD/target/release:$PATH"
# or: ./scripts/syx run ./examples/basic_demo.sy
syx run ./examples/basic_demo.sy
```

## Hello, World

```sy
func main() {
    print("Hello, World")
}
```

## Project layout

```
crates/              Compiler crates (lexer → driver → CLI, VM, MIR, LLVM, LSP, …)
packages/            Embedded / importable standard library (Core builtins, Net, Text, …)
examples/            Sample programs
tutorial/            Guided `.sy` lessons
books/               Language guide & reference
docs/                Compiler docs, usage guide, ADRs
editors/             VS Code / Neovim / Helix extensions
```

Notable crates:

| Crate | Role |
|-------|------|
| `syx-driver` | Module graph & compile pipeline |
| `syx-cli` | `syx` command-line tool |
| `syx-vm` / `syx-codegen` | Stack VM + bytecode |
| `syx-mir` / `syx-mir-run` | Mid-level IR + shared interpreter |
| `syx-llvm` | Native AOT (LLVM) |
| `syx-stdlib` | Builtin type / host registry |
| `syx-lsp` | Language server |

Core builtin type authority lives under [`packages/core/builtins/`](packages/core/builtins/) (numeric bodies are generated from [`_templates/`](packages/core/builtins/_templates/)).

## Compiler pipeline

```
Source (.sy) → Lexer → Parser → Semantic → MIR
                                         ├→ VM (syx run)
                                         ├→ LLVM native (syx build --native)
                                         └→ Wasm (syx build --wasm / syx run --wasm)
```

## CLI commands

| Command | Description |
|---------|-------------|
| `syx run [path]` | Compile and execute on the VM (full stdlib) |
| `syx check [path]` | Type-check without running |
| `syx build [path]` | Compile a file or project |
| `syx build --native [path]` | MIR → native executable (**LLVM**) |
| `syx build --wasm [path]` | Emit `main.wasm` |
| `syx run --wasm [path]` | Compile and run Wasm (wasmtime host) |
| `syx test [path]` | Run tests under `tests/` |

See [docs/9_UsageGuide.md](docs/9_UsageGuide.md) for native / Wasm setup details.

## Project manifest

Create `syx.toml` at the project root:

```toml
name = "my-app"
version = "0.1.0"
main = "main.sy"
```

## Standard library highlights

Importable modules live under [`packages/`](packages/) (`import Text`, `import Net`, …). Always-in-scope builtins include numerics, `String` / `Array` / `Dictionary`, and host globals such as `print` / `readLine` (see [`packages/core/builtins/special/globals.sy`](packages/core/builtins/special/globals.sy)).

Common host surfaces:

- `File.read` / `File.write` / `File.exists`
- `HTTP.get` / `HTTP.post` (and richer `Net` APIs)

Full module list: [`packages/internal/manifest.json`](packages/internal/manifest.json) · overview: [`packages/README.md`](packages/README.md).

## Testing

Behavioral suites live under `crates/syx-driver/tests`:

```bash
cargo test -p syx-driver --test lang --release -- --test-threads=1
cargo test -p syx-driver --test stdlib --release -- --test-threads=1
cargo test -p syx-driver --test native --release -- --test-threads=1
cargo test -p syx-driver --test concurrency --release -- --test-threads=1
```

CI also runs capability freshness checks (`scripts/gen_ir_capabilities.py --check`) and a public tip-leak gate.

## Contributing

- Feature and behavioral-test PRs belong in this public repository.
- Document capability boundaries in [docs/adr/capabilities-summary.md](docs/adr/capabilities-summary.md); do not add public CI nails for tip / ordinal fingerprints.

## Documentation

| Doc | Description |
|-----|-------------|
| [Capabilities summary](docs/adr/capabilities-summary.md) | What works / honest OUT (public ADR entry) |
| [Usage guide](docs/9_UsageGuide.md) | Install, CLI, projects, stdlib, toolchains |
| [ADR dual-track](docs/adr/README.md) | Summary vs historical dump |
| [Docs index](docs/README.md) | Compiler internals & other docs |
| [Books](books/README.md) | Language guide & reference |
| [Tutorial](tutorial/README.md) | Hands-on lessons |

## License

[Apache License 2.0](LICENSE)
