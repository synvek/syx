# SYX compiler technical documentation

[简体中文](./zh/README.zh-CN.md)

Docs for the **Rust SYX compiler toolchain** (`crates/`). For install / CLI / stdlib, prefer [9_UsageGuide.md](./9_UsageGuide.md) and [books/](../books/).

> **Current version:** SYX **117.0** · Phase **139** · cache `syx-cache-v1` · audit driver ≥2600 / LSP ≥3050 · license [Apache-2.0](../LICENSE)  
> **Native AOT:** LLVM sole · **Implementation status SSOT:** [books/shared/implementation-status.md](../books/shared/implementation-status.md)  
> **Architecture SSOT:** [adr/architecture-decisions.md](./adr/architecture-decisions.md)

Numbered pipeline docs below are **English by default**; Chinese copies live under [`zh/`](./zh/) as `*.zh-CN.md`.

## Doc map

### Users

| Doc | Description |
|-----|-------------|
| [9_UsageGuide.md](./9_UsageGuide.md) | Install, CLI, emit, projects, native/WASM, concurrency |
| [8_QuickReference.md](./8_QuickReference.md) | Syntax cheat sheet |
| [books/language-guide/](../books/language-guide/) | Narrative language guide |
| [books/language-reference/](../books/language-reference/) | Formal language reference |
| [books/README.md](../books/README.md) | Books index and build |

### Compiler pipeline

| # | Doc | Description |
|---|-----|-------------|
| 1 | [1_LexicalAnalysis.md](./1_LexicalAnalysis.md) | `syx-lexer` |
| 2 | [2_SyntaxAnalysis.md](./2_SyntaxAnalysis.md) | `syx-parser` + `syx-ast` |
| 3 | [3_SemanticAnalysis.md](./3_SemanticAnalysis.md) | `syx-semantic` + `syx-stdlib` |
| 4 | [4_InstructionSet.md](./4_InstructionSet.md) | `syx-codegen` + `syx-vm` |
| 5 | [5_CompleteTechnicalDocumentation.md](./5_CompleteTechnicalDocumentation.md) | Crate overview and pipelines |
| 10 | [10_BackendParity.md](./10_BackendParity.md) | VM / MIR / LLVM / WASM |
| 11 | [11_NetworkStdlib.md](./11_NetworkStdlib.md) | Networking stdlib |
| 12 | [12_BackendDebuggingGuide.md](./12_BackendDebuggingGuide.md) | Backend debugging |

### Reference

| Doc | Description |
|-----|-------------|
| [env-registry.md](./env-registry.md) | Environment variable registry |
| [syx-mir-text-v1.md](./syx-mir-text-v1.md) | MIR text format |
| [syxbc-text-v1.md](./syxbc-text-v1.md) | SYXBC text format |
| [wasm_native_opcode_matrix.md](./wasm_native_opcode_matrix.md) | WASM/native opcode matrix |
| [capability-manifest-v0.md](./capability-manifest-v0.md) · [ir-capability-manifest-v0.md](./ir-capability-manifest-v0.md) · [capability-sandbox-policy-v0.md](./capability-sandbox-policy-v0.md) | Capability |
| [adr/README.md](./adr/README.md) | ADR entry (`architecture-decisions.md`) |
| [differential/](./differential/) | Oracle / differential notes |
| [test-buckets.md](./test-buckets.md) | Test buckets |

### Analysis (historical; trust status for current truth)

| Doc | Description |
|-----|-------------|
| [6_ComprehensiveAnalysisReport.md](./6_ComprehensiveAnalysisReport.md) | Competitiveness analysis (banners may lag) |
| [7_FeatureImplementationPriority.md](./7_FeatureImplementationPriority.md) | Feature priority (cross-check status) |

Concurrency content lives in the UsageGuide and [18-concurrency](../books/language-guide/18-concurrency.md); [9_ConcurrencyMVP.md](./9_ConcurrencyMVP.md) is a redirect stub.

---

## Reading paths

- **Learn the language:** books/language-guide → UsageGuide → QuickReference  
- **Read the compiler:** 1 → 2 → 3 → 4 → 5 → BackendParity  
- **Decisions / status:** architecture-decisions · implementation-status  

## Repo layout (summary)

```
crates/              # compiler / runtime / LSP / tools
packages/            # stdlib and internal manifests
docs/                # this directory
books/               # user-facing books
plan/                # maintainer status and roadmap (SSOT)
examples/            # sample programs
```

## Tools

- Rust 1.85+ · Cargo workspace · macOS / Linux / Windows

## Current version (SYX 117.0)

- **Language / phase:** 117.0 / **139** · MIR cache `syx-cache-v1`
- **Architecture decisions:** [adr/architecture-decisions.md](./adr/architecture-decisions.md)
- **Native:** LLVM only
- **Audit:** driver ≥2600 · LSP ≥3050 (see `scripts/audit_behavioral_tests.sh`)
- **Implementation status:** [books/shared/implementation-status.md](../books/shared/implementation-status.md)
