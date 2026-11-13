---
title: SYX vs Swift
description: SYX is not Swift — it borrows familiar Swift-like syntax but has its own semantics, standard library and Rust toolchain. Neither a subset nor a superset of Swift.
lastUpdated: 2026-11-13
---
# SYX vs Swift

**Short answer:** SYX is **not** Swift. It borrows a Swift-like surface (`func`, `let` / `var`, protocols, generics, `async` / `await`) so the language feels familiar, but it is an **independent** language with its own semantics, standard library, and a **Rust-implemented** toolchain.

| | Swift | SYX |
|---|---|---|
| Who defines it | Apple / Swift.org | SYX Language Project |
| Compiler | Swift compiler (C++/Swift) | Rust workspace (`syx-*` crates) |
| Source extension | `.swift` | `.sy` |
| Project file | `Package.swift` (SPM) | `syx.toml` |
| Primary runtimes | Swift runtime / Darwin & more | Stack VM, LLVM native AOT, Wasm |
| Relationship | — | Inspired by Swift syntax; **not** a dialect |

## Where behaviour commonly diverges

Names often match while meaning differs — especially:

- `String` / character indexing and Unicode handling
- Memory model and ownership expectations
- Macros and result-builder style features
- Concurrency and cross-module actors

For a chapter-level tour of differences, start with the [Language Guide preface](/guide/00-preface). For “does SYX support *X* today?”, use [Capabilities](./capabilities).

## Related

- [What is SYX?](./what-is-syx)
- [Backends (VM / LLVM / Wasm)](./backends)
- [Install SYX](./install)
