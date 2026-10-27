# Preface

Welcome to the ***SYX Language Guide***. SYX (pronounced **sy-ex**, `/saɪˈɛks/`) is a programming language inspired by Swift syntax, backed by a complete Rust-implemented compiler toolchain (default **VM** + **LLVM** native AOT + **WASM**; experimental JIT tier-2 opt-in). Source files use the **`.sy`** extension and project manifests are **`syx.toml`**. This guide helps you learn SYX semantics and idioms systematically, and build intuition through runnable examples.

> **This edition covers:** SYX **117.0** · Phase **139** · `syx-cache-v1` · audit 2600/3050 · license [Apache-2.0](../../LICENSE) · documentation last synced 2026-10-27
>
> For the full status matrix, see [shared/implementation-status.md](../shared/implementation-status.md); for architecture decisions, see [docs/adr/architecture-decisions.md](../../docs/adr/architecture-decisions.md); for the capabilities summary, see [docs/adr/capabilities-summary.md](../../docs/adr/capabilities-summary.md).

## What Is SYX

SYX provides:

- **Syntax:** Swift-style `func` / `let` / `var`, protocols, generics, `async` / `await`, property wrappers, result builders
- **Types:** statically and strongly typed, including `Int8..Int64/128`, `UInt8..UInt64/128`, `Float32/64`, `SIMD2/4/8/16<T>`, `BigInt`, and reference types `&T` / `&mut T`
- **Back ends:** default stack VM (`syx run`) + Native LLVM (`syx build --native`) + WASM (`syx run --wasm`); Cranelift **AOT removed** (experimental JIT tier-2 opt-in)
- **Modules:** a `syx.toml` project + `import` path dependencies + git/registry dependencies + Host plugins (`dlopen`)
- **Standard library:** embedded `Math` / `IO` / `Net` / `Crypto` / `Di` / `Os` / `Sync` / `UI` and more; Core builtins in [`packages/core/builtins/`](../../packages/core/builtins/)

> **Important:** SYX is **neither** a subset nor a superset of Swift. Many names match, but semantics may differ substantially (especially String character semantics, the memory model, macros, concurrency, and cross-module actors).

## The Compilation Pipeline

```
Source (.sy, UTF-8)
  → syx-lexer (Token stream)
  → syx-parser (AST, recursive descent)
  → syx-semantic (type checking + symbol table + borrowing)
  → syx-macro (fixed-point expansion)
  → syx-mir (unified IR)
  → syx-vm | syx-llvm | syx-wasm-emit
```

See [docs/5_CompleteTechnicalDocumentation.md](../../docs/5_CompleteTechnicalDocumentation.md) · [docs/3_SemanticAnalysis.md](../../docs/3_SemanticAnalysis.md).

## Conventions Used in This Book

### Implementation Status Markers

An **Implementation status** line appears at the top of each section:

| Marker | Meaning |
|------|------|
| **Implemented** / **GA** | Verifiable with `syx run`, parity, or hard gates |
| **Partially implemented** | Parses/compiles but semantics are incomplete |
| **Planned** | Recorded on the roadmap, implementation unfinished |
| **Not implemented** | No corresponding AST/semantics |

The single master table: [shared/implementation-status.md](../shared/implementation-status.md).

### Code Examples

Unless stated otherwise, examples can be run from the repository root with:

```bash
cargo run -p syx-cli -- run path/to/example.sy
```

### Relationship to Other Reference Documents

| Need | Where to go |
|------|------|
| Narrative learning | This book (language-guide) |
| Formal rules | [language-reference](../language-reference/00-about-this-reference.md) |
| Embedded package APIs | [library-guide](../library-guide/00-preface.md) |
| Installation / CLI | [docs/9_UsageGuide.md](../../docs/9_UsageGuide.md) |

For the CLI, Native/WASM compilation, the Embed API and diagnostic codes, see:

- [docs/9_UsageGuide.md](../../docs/9_UsageGuide.md) — the complete usage guide
- [docs/8_QuickReference.md](../../docs/8_QuickReference.md) — syntax cheat sheet
- [docs/12_BackendDebuggingGuide.md](../../docs/12_BackendDebuggingGuide.md) — DAP debugging and back-end debugging

## Differences from Swift at a Glance

| Topic | Swift 6 | SYX 117.0 |
|------|---------|-----------|
| String characters | Extended grapheme clusters | **Grapheme clusters** (`unicode-segmentation`); earlier versions used Unicode scalars |
| Memory | ARC (uniform) | **ARC (class) + ManagedHeap** (VM / WASM / Native) |
| Entry point | `@main` / free function | `func main()` (synchronous or `async`) |
| `switch` | Implicit break | `case { }` blocks; no fallthrough |
| Macros | Built-in + full Swift macros + external macro | freestanding + user `#macro` MVP + attached / `@resultBuilder` / DI pointcut MVP |
| Runtime extensions | Static stdlib + ABI plugins | Static Host + opt-in dlopen plugins + Embed API |
| Generics | Full + specialization | Monomorphization (no full runtime specialization reflection) |
| Concurrency | Structured + actors + distributed | `async`/`await` + `actor` + `Task`/`TaskGroup` + `Channel` + `GlobalActor` MVP |
| Native back end | — | **LLVM only** (Cranelift AOT removed) |

## Recommended Learning Path

| Stage | Chapters | Goal |
|------|------|------|
| **Getting started** | 1–8 | Able to read and write ordinary SYX programs |
| **Type system** | 9–16 | Master enum/class/struct/protocol/generics/properties/initialization/deinitialization |
| **Advanced features** | 17–25 | Error handling, concurrency, macros, protocols, opaque types |
| **Runtime and extensions** | 26–33 | Memory, borrowing, macros/AOP in practice, Host plugins, SIMD |
| **Appendices** | A, B | Master status table + diagnostic code cheat sheet |

At any stage you can jump to the corresponding language reference chapter to look things up precisely.

## Version History (Recent Phases)

| Phase | Marketing version | Main changes |
|------|--------|---------|
| **139** | **117.0** | multiline / tuple / nested path / range / Character / call labels; SIMD16 all/any; `syx-cache-v1`; audit 2600/3050; SynUI deferred; Farm/Dist freeze v19 |
| 138 | 116.0 | Class Properties v6, Macro M28, Service DI v19, Actor shared pool, SIMD16 bitmask, Os primaryIpv4 |
| 137 | 115.0 | Class Properties v5, Macro M27, Service DI v18, Task DiscardOldest, SIMD16 popcnt |
| 136 | 114.0 | Class Properties v4, Macro M26, Service DI v17, Os hostname/uptime |
| 135 | 113.0 | Class Properties v3, Macro M25, SynUI v12, Task CallerRuns |

For the full history, see [shared/implementation-status.md](../shared/implementation-status.md) and the ADR series.

---

**Next chapter** → [Getting Started](01-getting-started.md)

**See also** → [Language Reference: About This Reference](../language-reference/00-about-this-reference.md) · [Implementation Status Master Table](../shared/implementation-status.md) · [Complete Usage Guide](../../docs/9_UsageGuide.md)
