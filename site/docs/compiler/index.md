# Compiler Internals

Technical documentation for the Rust SYX compiler toolchain (`crates/`).

```
Source (.sy)
  → Lexer → Parser → Semantic → MIR
                               ├→ VM
                               ├→ LLVM native
                               └→ Wasm
```

| Document | Description |
|----------|-------------|
| [Lexical Analysis](./lexical-analysis) | `syx-lexer` |
| [Syntax Analysis](./syntax-analysis) | `syx-parser` + `syx-ast` |
| [Semantic Analysis](./semantic-analysis) | `syx-semantic` + `syx-stdlib` |
| [Instruction Set](./instruction-set) | `syx-codegen` + `syx-vm` |
| [Technical Overview](./technical-overview) | Crate overview and pipelines |
| [Backend Parity](./backend-parity) | VM / MIR / LLVM / Wasm |
| [Network Stdlib](./network-stdlib) | Networking packages |
| [Backend Debugging](./backend-debugging) | Debugging backends |
| [Env Registry](./env-registry) | Environment variable registry |

For day-to-day usage see the [Quick Guide](/quick-guide/). Architecture decisions: [About](/about/architecture).
