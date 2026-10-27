# Preface

Welcome to the ***SYX Standard Library Guide***. For language syntax, see the [Language Guide](../language-guide/SUMMARY.md); for the formal rules, see the [Language Reference](../language-reference/SUMMARY.md). This book covers only the **embeddable packages you can `import`**: their paths, their honest boundaries, and the runnable examples in this repository that you can check them against.

> **This edition covers:** SYX **117.0** / Phase **139** · HostFn ≈ **630** · embedded manifest [`packages/internal/manifest.json`](../../packages/internal/manifest.json) · package overview [`packages/README.md`](../../packages/README.md)

## How This Book Divides the Work with the Language Guide

| If you want… | Go to |
|------|------|
| `func` / `class` / `import` syntax | [Language Guide](../language-guide/01-getting-started.md) |
| Host plugins / `dlopen` | [Language Guide · Runtime Extensions](../language-guide/33-runtime-extensions.md) |
| `import Linalg` / `Os` / `Log` / `IO` | **This book** |
| CLI, `syx.toml` | [docs/9_UsageGuide.md](../../docs/9_UsageGuide.md) |

## Honesty Conventions

- **Implemented**: surfaces exercised by smoke tests or runnable through Host.
- **Partially implemented**: compiles, but the semantics are incomplete (several `Process` paths remain `partial`).
- **Not claimed**: product `Mat` GA, E-00, `A[i,j]`, faer `add`/`sub` host, P-SURF `[x]`.

The embedded package sources live in [`packages/`](../../packages/README.md). Do **not** merge `Mat` into `packages/core/math.sy`.
