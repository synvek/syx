---
layout: home
title: SYX Language
hero:
  name: SYX
  text: Swift-inspired. Rust-powered.
  tagline: A modern programming language with VM, LLVM native AOT, and Wasm — source files end in .sy
  actions:
    - theme: brand
      text: Get Started
      link: /guide/01-getting-started
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

> Documentation is synced from the repository `books/` and `docs/` trees. Switch to **简体中文** via the locale picker in the nav.
