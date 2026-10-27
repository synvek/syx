# SYX Language eBooks (English)

This directory contains three English Markdown eBooks for SYX language users and implementers:

| Book | Directory | Description |
|------|------|------|
| **SYX Language Guide** | [language-guide/](language-guide/) | A narrative tutorial, from the basics to advanced features, with complete examples and differences from Swift |
| **SYX Language Reference** | [language-reference/](language-reference/) | Formal reference for lexical structure, syntax, types, declarations, patterns and attributes |
| **SYX Standard Library Guide** | [library-guide/](library-guide/) | The embedded-package user surface: `import Linalg` / `Os` / `Log` / `IO`, …; honest boundaries and repository examples |

Shared resource: [shared/implementation-status.md](shared/implementation-status.md) — the **current** master feature implementation status table (SYX **117.0** / Phase **139**).

**Version alignment:** SYX 117.0 (Phase 139) · cache `syx-cache-v1` · audit 2600/3050 · architecture decisions in [docs/adr/architecture-decisions.md](../docs/adr/architecture-decisions.md)

## Reading Recommendations

| If you are… | Suggested path |
|------|---------|
| New to SYX | [Preface](language-guide/00-preface.md) → [Getting Started](language-guide/01-getting-started.md) → read on in `language-guide/SUMMARY.md` order |
| Looking up an embedded package (Linalg / Os / Log) | [Standard Library Guide](library-guide/00-preface.md) → [Package Overview](library-guide/01-overview.md) |
| Looking up a specific syntax / type | [Language Reference · Contents](language-reference/00-about-this-reference.md) → the corresponding chapter |
| Checking whether a feature is GA/partial | [Implementation Status Master Table](shared/implementation-status.md) (the **single** ground truth) |
| Writing a compiler or SDK | [docs/README.md](../docs/README.md) → [Architecture Decisions](../docs/adr/architecture-decisions.md) |
| Installing the CLI, running programs, debugging | [docs/9_UsageGuide.md](../docs/9_UsageGuide.md) |

## Building HTML / EPUB / PDF

### mdBook (Recommended)

```bash
# Language Guide
cd books/language-guide && mdbook build

# Language Reference
cd books/language-reference && mdbook build

# Standard Library Guide
cd books/library-guide && mdbook build
```

Each book ships a `book.toml`; `SUMMARY.md` is the table of contents. `mdbook serve` starts a local hot-reload preview.

### Pandoc One-Step EPUB

```bash
pandoc books/language-guide/SUMMARY.md \
  $(ls books/language-guide/*.md | grep -v SUMMARY) \
  -o syx-language-guide.epub \
  --toc --metadata title="SYX Language Guide"

pandoc books/language-reference/SUMMARY.md \
  $(ls books/language-reference/*.md | grep -v SUMMARY) \
  -o syx-language-reference.epub \
  --toc --metadata title="SYX Language Reference"
```

### VS Code

Install a Markdown preview enhancement extension and open any chapter; `SUMMARY.md` provides navigation.

## Implementation Status Markers

All three books use the same four tiers (see [Implementation Status Master Table](shared/implementation-status.md)):

| Marker | Meaning |
|------|------|
| **Implemented** / **GA** | Verifiable with `syx run`, or parity / hard gates |
| **Partially implemented** | Parses/compiles but semantics are incomplete |
| **Planned** | Recorded on the roadmap, implementation unfinished |
| **Not implemented** | No corresponding AST/semantics |

Items prefixed with "**partial GA**" delivered a minimal working subset in that Phase; the remaining full set advances with later Phases. Items marked "**Rejected**" are deliberately fail-closed (for example computed-property observers, S056).

## Relationship to the Existing docs

| Document | Role |
|------|------|
| `books/` (this directory) | Language tutorial + formal reference + [Standard Library Guide](library-guide/00-preface.md) |
| `docs/8_QuickReference.md` | Syntax cheat sheet (a short version strictly aligned with the parser) |
| `docs/9_UsageGuide.md` | Toolchain, CLI, standard library and Native/WASM compilation |
| [`docs/adr/architecture-decisions.md`](../docs/adr/architecture-decisions.md) | Architecture decisions SSOT |
| `plan/topics/hostfn-extension.md` | Runtime extension layering and the maintainer checklist |
| `docs/1–5_*.md` | Compiler implementation technical documentation (in 1→2→3→4→5 order) |
| `packages/README.md` | Embedded standard library (`import Di` / `import Os` / `import UI`, …) |

## Current Phase 139 Capability Summary

- **Gate0:** audit **2600/3050**; manifest **117.0** / phase **139**; EXCLUDE historical behavioral fill
- **Language surface:** multiline / tuple / nested path / closed range / Character / call labels (see status for details)
- **GUI:** SynUI deferred (not GA in this phase)
- **Farm/Dist:** frozen at v19
- **Wasm SIMD16:** i8x16 all_true/any_true guest-v22 MVP plus the existing guest surface
- **Details:** [shared/implementation-status.md](shared/implementation-status.md)
- **Farm / Dist:** **frozen v19** (hsm-key-id / peer-announce-hold); no new surface in this phase
- **Os (partial GA):** `primaryIpv4()` MVP; disk/load / hostname/uptime / cpu/mem/user inherited; **not** a complete sysinfo → **139+**
- **DX:** `driver_gate` / LSP audit in status; cache **`syx-cache-v1`**; license **Apache-2.0**
- **Native AOT:** **LLVM sole** (Cranelift AOT removed; JIT tier-2 opt-in)
- **Core builtins:** [`packages/core/builtins/`](../packages/core/builtins/) (numeric templates [`_templates/`](../packages/core/builtins/_templates/))
