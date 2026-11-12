---
layout: home
title: SYX Language
titleTemplate: false
description: SYX is a Swift-inspired programming language with a Rust-implemented compiler toolchain, shipping a stack VM, LLVM native AOT and a Wasm backend from one MIR pipeline.
hero:
  name: SYX
  text: Swift-inspired. Rust-powered.
  tagline: A modern programming language with VM, LLVM native AOT, and Wasm — source files end in .sy
  image:
    src: /syx-lang.png
    alt: SYX language sample — Greeter class, extension, and main
  actions:
    - theme: brand
      text: Get Started
      link: /guide/01-getting-started
    - theme: brand
      text: Download
      link: https://github.com/synvek/syx/releases/latest
    - theme: alt
      text: Language Guide
      link: /guide/00-preface
    - theme: alt
      text: Quick Reference
      link: /quick-reference/
    - theme: alt
      text: VS Code
      link: /editors/vscode
features:
  - title: Familiar syntax
    details: Swift-style func / let / var, protocols, generics, async/await — with honest semantic differences from Swift.
  - title: Three backends
    details: Stack VM for fast iteration (syx run), LLVM for native AOT, and Wasm emit/run — one MIR pipeline.
  - title: VS Code + LSP
    details: Syntax highlighting, member/import completion, hover, inlay hints, format on save, and DAP debugging via syx lsp.
  - title: Embedded stdlib
    details: Math, IO, Net, Crypto, Os, Sync, Linalg and more as importable packages with documented boundaries.
---

## What is SYX?

**SYX** (pronounced *sy-ex*, `/saɪˈɛks/`) is a statically typed, Swift-inspired programming language whose compiler toolchain is implemented in Rust. It is designed for people who like Swift's `func` / `let` / `var` style, protocols, generics and `async` / `await`, but want a single toolchain that targets a bytecode VM, native code and WebAssembly.

| Fact | Value |
|------|-------|
| Paradigm | Statically and strongly typed, multi-paradigm |
| Inspired by | Swift syntax; independent semantics |
| Implementation | Rust workspace (`syx-lexer`, `syx-parser`, `syx-semantic`, `syx-mir`, …) |
| Source file | `.sy` (UTF-8) |
| Project manifest | `syx.toml` |
| Toolchain binary | `syx` (`syx-cli`) |
| Backends | Stack VM (`syx run`), LLVM native AOT (`syx build --native`), Wasm (`syx build --wasm`) |
| Language server | `syx lsp` |
| License | Apache-2.0 |

SYX is **neither a subset nor a superset of Swift**. Names often match, but semantics differ — notably for `String` character handling, the memory model, macros, concurrency and cross-module actors.

## Hello, World

```sy
func main() {
    print("Hello, World")
}
```

```bash
cargo build --release -p syx-cli
./target/release/syx run examples/hello.sy
```

## Where to go next

| Path | Audience |
|------|----------|
| [Quick Guide](/quick-guide/) | Install, CLI, projects, Native / Wasm |
| [Quick Reference](/quick-reference/) | Syntax and stdlib cheat sheet |
| [Editors · VS Code](/editors/vscode) | Syntax highlighting, completion, LSP, DAP |
| [Language Guide](/guide/00-preface) | Learn SYX from the basics to advanced features |
| [Language Reference](/reference/00-about-this-reference) | Formal lexical, syntax, and type reference |
| [Standard Library](/library/00-preface) | Embedded packages: Linalg, Os, Log, IO, Net… |
| [Compiler Internals](/compiler/) | Lexer → parser → semantic → MIR → backends |
| [Capabilities](/about/capabilities) | What is implemented today (honest boundaries) |

## Frequently asked questions

### Is SYX the same as Swift?

No. SYX borrows Swift's surface syntax but is an independent language with its own semantics, standard library and toolchain. It is explicitly neither a subset nor a superset of Swift; the [Language Guide preface](/guide/00-preface) lists where behaviour diverges.

### What runtime backends does SYX have?

Three, all fed by one MIR pipeline: a stack bytecode VM (`syx run`), LLVM-based native AOT compilation (`syx build --native`), and WebAssembly (`syx build --wasm` / `syx run --wasm`). LLVM is the only native AOT backend.

### What file extensions and project layout does SYX use?

Source files end in `.sy`. A project declares itself with a `syx.toml` manifest containing fields such as `name`, `version` and `main`, and modules are pulled in with `import`.

### Do I need Rust to use SYX?

Not necessarily to *run* it — releases ship prebuilt binaries. Building the toolchain from source requires Rust 1.85 or newer. Building the full native toolchain adds a local LLVM toolchain.

### How do I get editor support?

Install the **SYX Language** VS Code extension for TextMate highlighting plus LSP features — completion, hover, go-to-definition, inlay hints, format on save, and DAP debugging through `syx lsp`. Neovim and Helix configuration are also provided; see [Editors](/editors/).

> Documentation is synced from the repository `books/` and `docs/` trees. Switch to **简体中文** via the locale picker in the nav.

