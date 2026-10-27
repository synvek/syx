# Appendix A: Implementation Status Matrix

This appendix points at the unified master feature-status table, to avoid duplicating [shared/implementation-status.md](../shared/implementation-status.md).

**Complete matrix:** [books/shared/implementation-status.md](../shared/implementation-status.md)

## Quick Summary (SYX 117.0 · Phase 139 · Truth M0)

| Domain | Implemented / GA | Partially implemented | Not implemented / debt |
|------|-------------|----------|---------------|
| Basic syntax and types | Numerics, collections, control flow, `Array<T>`, `Dictionary`, tuple / `"""` / `...` / call labels (P139); Iterator for-in (ADR-223); collection mutation write-back (ADR-224); Deque&lt;Int&gt; MVP (ADR-248) | `Sequence.makeIterator` / full Collection Index / Dict subscript insertion | Heap/lazy sequence |
| String / Character | UTF-8, Character distinct scalar MVP (P139) | Full Unicode Character set, Index model | — |
| OOP / class system | class/struct/enum, Properties v6, final (sealed docs removed) | Full deinit surface | — |
| Concurrency | async/Task/actor, Actor shared pool | Full ThreadPoolExecutor set | Swift 6 strict concurrency |
| Memory | ARC + ManagedHeap | Complete borrow | — |
| Macros / DI | M28 + DI v19 inheritance | Full token / AspectJ / Spring | **140+** |
| SIMD | guest-v22 all/any true + inherited bitmask/… | Full SIMD16 set | — |
| Net / Time | HTTP client/server VM; TCP **integer handle** (M0); Clock.nowMs; DateTime narrow calendar core **implemented** (ADR-222) | IANA / `formatWithTimezone` **unsupported** (U001) | Real WS/gRPC protocols, IANA DateTime GA |
| GUI / Farm / Dist | SynUI v12 / Farm·Dist v19 **frozen** | — | SynUI v13 / Farm v20 |
| Toolchain | LLVM sole native; `syx-cache-v1`; **fmt fail-closed** (M0-A02); JIT **experimental-real-subset** opt-in (`SYX_JIT_TIER2` / Suite-1 Cranelift; M0-A01) | lossless CST formatter | Cranelift (AOT removed; JIT tier-2 still opt-in) |

> **Truth M0 W1–W2 (ADR-132):** stop fake successes (fmt/DateTime/TCP/Dict.remove/PBKDF2/JIT); see [`plan/archive/roadmap-m0-m4-task-list.md`](../../plan/archive/roadmap-m0-m4-task-list.md) (frozen) and [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md).

> **Truth M0 W2–W4 (ADR-133):** capability manifest + four-back-end differential; the generated matrix is in [capabilities-generated.md](../shared/capabilities-generated.md).

> **Key changes in this Phase 139:** multiline / tuple / nested path / range / Character / call labels / sealed docs; SIMD16 all/any; audit 2600/3050; `syx-cache-v1`.

## Three-Back-End Parity Overview

| Back end | Scope | Status |
|------|------|------|
| **VM** (default) | Interpretation + GC | **GA** |
| **Native (LLVM)** | MIR → LLVM IR → static linking | **GA** (Phase 77 sole backend) |
| **WASM** | Direct MIR compilation + wasmtime host running the SYX VM | **GA** |
| Cranelift | MIR → Cranelift IR | **Removed** (Phase 77) |

## Status Tiers

| Tier | Meaning |
|------|------|
| **Implemented** / **GA** | Verifiable with `syx run`, or parity / hard gates |
| **Partially implemented** / **partial GA** | Compiles but semantics are incomplete |
| **Planned** | Recorded on the roadmap |
| **Not implemented** | No corresponding AST/semantics |

> **Update policy:** please modify only `shared/implementation-status.md`; this appendix is a quick summary only.

---

**See also** → [Language Reference Appendix](../language-reference/appendix-implementation-matrix.md) · [Implementation Status Master Table](../shared/implementation-status.md) · [architecture-decisions.md](../../docs/adr/architecture-decisions.md)
