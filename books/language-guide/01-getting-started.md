# Getting Started

**Implementation status:** **Implemented** (including the `async` entry point and the full paths of `syx run` / `syx build` / `syx run --wasm` / `syx build --native`)

This chapter starts from zero: write your first SYX program, run it with the CLI, read errors, and grow it into a multi-file project.

## Hello, World

The smallest runnable program (from `examples/hello.sy`):

```syx
func main() {
    print("Hello, World")
}
```

| Element | Description |
|------|------|
| `func main()` | The **only** program entry point; it may be omitted (`syx run` inserts `main` automatically), but a project root entry must exist |
| `print(…)` | The language-level global Host output (written like a function; the authoritative signature is in [`packages/core/builtins/special/globals.sy`](../../packages/core/builtins/special/globals.sy)) |
| `{ … }` | Statement block |

SYX does **not** allow top-level executable statements; all logic must live inside a `func` (`main` or another function). This is the main difference from Python and Ruby.

## Installation and Building

### Option A: Build from Source (Recommended for Daily Use)

```bash
git clone <repo-url> synswx
cd synswx

# Debug build (slow startup, about 5–10 seconds)
cargo build

# Release build (startup < 0.3 seconds; recommended for daily use)
cargo build --release
```

The release binary is at `target/release/syx`. Add it to your `PATH`:

```bash
export PATH="$PWD/target/release:$PATH"
syx --version
```

### Option B: One-Off Run (No Build Step)

```bash
cargo run -p syx-cli -- run examples/hello.sy
```

Cargo compiles `syx-cli` and runs it immediately. Common in CI / smoke tests; Option A is recommended for daily use.

### Option C: `cargo install` (Into `~/.cargo/bin`)

```bash
cargo install --path crates/syx-cli
syx run examples/hello.sy
```

### System Requirements

| Component | Version |
|------|------|
| Rust | 1.80+ |
| Cargo | Installed with Rust |
| clang / system linker | Required by `syx build --native` |
| LLVM 18+ (optional) | `--backend=llvm` emits LLVM IR directly (the default `--backend=llvm` still depends on the `syx-rt` static library) |

## Three Ways to Run

| Scenario | Command | Output | Best for |
|------|------|------|------|
| **Daily development / debugging** | `syx run program.sy` | No file, executed directly | Full P0 capabilities (File, HTTP, modules, macros) |
| **Native compilation** | `syx build --native program.sy` | An `./program` executable | Production deployment, benchmarks, C ABI integration |
| **WASM** | `syx run --wasm program.sy` | Runs in memory | Cross-platform distribution, embedding in a host |

See [docs/9_UsageGuide.md § Compilation and Back Ends](../../docs/9_UsageGuide.md) and the WASM guest path notes in [Chapter 31, SIMD](31-simd.md).

## Your First Program: Variables and String Interpolation

```syx
func main() {
    let name = "SYX"
    var count = 0
    count = count + 1
    print("Hello, \(name)! count=\(count)")
}
```

Output:

```
Hello, SYX! count=1
```

| Keyword | Meaning | Reassignment |
|--------|------|----------|
| `let` | Immutable binding | ❌ Compile error **S003** |
| `var` | Mutable binding | ✅ |

String interpolation `"\(expr)"` is parsed at compile time into a `StringConcat` expression; any expression is legal inside `\(name)` (including function calls). See [Chapter 4, Strings and Characters](04-strings-and-characters.md).

## Multi-File Projects

SYX encourages putting multiple modules into a `syx.toml` project:

```
my-app/
├── syx.toml
├── main.sy
└── lib/
    └── math.sy
```

`syx.toml`:

```toml
name = "my-app"
version = "0.1.0"
main = "main.sy"

[dependencies]
# Path dependency (points at lib/math.sy)
math = "lib/math.sy"
```

`lib/math.sy`:

```syx
public func add(a: Int, b: Int) -> Int {
    return a + b
}
```

`main.sy`:

```syx
import math

func main() {
    print("3 + 4 = \(math.add(3, 4))")
}
```

Run it:

```bash
cd my-app
syx run .
# → 3 + 4 = 7
```

Only `public` symbols are visible across modules (see [Chapter 28, Access Control](28-access-control.md) and [Chapter 29, Modules and Packages](29-modules-and-packages.md)).

## The async Entry Point

`func main` may be declared `async` and call `await`:

```syx
import Net

async func main() {
    let response = await Net.get("https://example.com")
    print(response.status)
}
```

> **Note:** `await` is legal only inside an `async` function body (compile error S020). Using `await` in a non-`async` function → compile error.

## Common Errors and Troubleshooting

| Symptom | Cause | Fix |
|------|------|------|
| `syx: command not found` | Not built / not on PATH | `cargo build --release` + `export PATH=$PWD/target/release:$PATH` |
| `P001: parse error at line N` | Syntax error | Look at the `^^^^^` highlight on the line above |
| `S002: undefined identifier 'x'` | An undeclared identifier was used | Check imports / spelling / scope |
| `S003: cannot assign to let` | Reassigning a `let` binding | Change it to `var` |
| `S004: type mismatch` | Type mismatch | Add an explicit annotation or insert a conversion |
| `R007: force unwrap of nil` | `!` unwrapped an `Optional` | Use `if let` / `guard let` / `??` |
| Network test `Http.get` times out | Sandboxed network isolation | Go offline or set `SYX_NET_ALLOW=*` (enterprise edition) |

For the complete diagnostic codes, see [Appendix B: Diagnostic Code Index](appendix-b-diagnostic-index.md) and [docs/9_UsageGuide.md § Diagnostic Codes](../../docs/9_UsageGuide.md).

## Compile-Time vs. Runtime Checks

`syx run` runs the full pipeline by default: lex → parse → semantic → macro → MIR → VM. Common flags:

| Flag | Behavior |
|------|------|
| `syx check program.sy` | Type checking only, no execution |
| `syx emit program.sy --format syxbc` | Emits stack VM bytecode text (human readable) |
| `syx emit program.sy --format mir` | Emits unified MIR text |
| `syx emit program.sy --format wasm -o out.wasm` | Emits a WASM binary (differs from `--wasm`; see [docs/9_UsageGuide.md](../../docs/9_UsageGuide.md)) |
| `syx emit . --format all -o emit/` | Exports six IRs at once (`syxbc` / `mir` / `wasm` / `wat` / `llvm` / `clif`) |
| `syx doc .` | Generates `docs/api.md` from `///` documentation comments |
| `syx test .` | Runs `tests/**/*.sy` (see [Chapter 34, Unit Testing](34-testing.md)) |
| `syx lint .` | Static checking (see [docs/9_UsageGuide.md § Static Checking](../../docs/9_UsageGuide.md)) |
| `syx format .` | Formats the entry file |
| `syx repl` | Interactive REPL (Ctrl-D to exit) |

## Next Steps

| If you want to… | Jump to |
|------|------|
| Write more complex data | [Chapter 5, Collection Types](05-collection-types.md) |
| Write your own types | [Chapter 10, Structures and Classes](10-structures-and-classes.md) |
| Write async / concurrent code | [Chapter 18, Concurrency](18-concurrency.md) |
| Write macros / DI | [Chapter 19, Macros](19-macros.md) |
| Look up precise grammar | [Language Reference](../language-reference/00-about-this-reference.md) |
| Check whether a feature is GA | [Implementation Status Master Table](../shared/implementation-status.md) |

## Differences from Swift

| Point | SYX |
|----|-----|
| Entry point | `func main()` is required (sync or `async`) |
| `print` | Host global (signature in `globals.sy`; written `print(x)`) |
| String interpolation | `"\(expr)"`, same as Swift |
| Top-level code | Forbidden; logic must live inside a `func` |

---

**Next chapter** → [The Basics](02-the-basics.md)

**See also** → [Complete Usage Guide](../../docs/9_UsageGuide.md) · [Language Reference: Declarations](../language-reference/05-declarations.md) · [Appendix A: Implementation Status Matrix](appendix-a-implementation-matrix.md)
