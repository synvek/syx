# S4-01 manifest / doctest scope v0（S3-I90）

**ADR:** [residual-scope](adr/stage4-s4-01-residual-scope.md) · **Window:** S3-I90  
**Purpose:** Bound what “manifest 接线 / doctest 范围” means for S4-01 **scoped acceptance** — documentation only.  
**≠** change [`packages/internal/manifest.json`](../packages/internal/manifest.json) schema · **≠** implement `syx doctest` · **≠** driver read of [`stability.toml`](../packages/internal/stability.toml).

## In scope (Primary — already landed)

| Item | Status |
|------|--------|
| Plan ternary SSOT (`stable` \| `preview` \| `experimental`) | [I89](adr/stage4-s4-01-stability-ssot.md) · `stability.toml` |
| README → ternary mapping | [stdlib-stability-v0](stdlib-stability-v0.md) |
| Census KPIs keep manifest/doctest counts at **0** honestly | deferred zeros |

## Deferred / supersede-only（not blockers for scoped `[x]`）

| Axis | Meaning | Product work |
|------|---------|--------------|
| Manifest wiring | Optional future `stability` field on package rows in `manifest.json` | Schema + loaders |
| Driver SSOT read | Compiler/tooling consumes `stability.toml` | Integration |
| `syx doctest` | CLI subcommand for package/API doctests | CLI + runner |
| Packages doctest gate | CI gate under `packages/**` | Test harness |
| Batch annotations | Mass-edit `.sy` docs for stable APIs | Source churn |

Restoring any as a **hard claim gate** requires a **superseding ADR**.

## Out（Hard）

- S4-01 `[x]` this window · Stage-4 exit · S4-02…05 construction  
- HostFn+1 · F-08 · Wave / product-umbrella unpark  
- Language semantics changes  

## Ladder

- **S3-I90:** this charter + residual-scope ADR  
- **S3-I91:** dedicated S4-01 scoped `[x]` claim under Primary above  
