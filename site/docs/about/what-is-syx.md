---
title: What is SYX?
description: SYX is a statically typed, Swift-inspired programming language with a Rust compiler toolchain — .sy sources, syx.toml projects, and VM / LLVM / Wasm backends from one MIR pipeline.
lastUpdated: 2026-11-13
---
# What is SYX?

**SYX** (pronounced *sy-ex*, `/saɪˈɛks/`) is a **statically typed, Swift-inspired programming language** whose compiler toolchain is implemented in Rust. It targets people who like Swift-style `func` / `let` / `var`, protocols, generics and `async` / `await`, but want one open toolchain that can run on a bytecode VM, compile to native code, and emit WebAssembly.

| Fact | Value |
|------|-------|
| Kind | Programming language + compiler toolchain |
| Syntax lineage | Swift-inspired surface; **independent** semantics |
| Implementation | Rust (`syx-lexer`, `syx-parser`, `syx-semantic`, `syx-mir`, …) |
| Source files | `.sy` (UTF-8) |
| Project manifest | `syx.toml` |
| CLI | `syx` (`syx-cli`) |
| Backends | Stack VM, LLVM native AOT, WebAssembly |
| License | Apache-2.0 |

SYX is **neither a subset nor a superset of Swift**. Shared names do not imply shared behaviour — see [SYX vs Swift](./syx-vs-swift).

## One-minute mental model

```text
.sy source → lexer → parser → semantic → MIR → VM | LLVM | Wasm
```

- **Learn the language:** [Language Guide](/guide/00-preface)
- **Install and run:** [Install SYX](./install)
- **Honest feature boundaries:** [Capabilities](./capabilities)
