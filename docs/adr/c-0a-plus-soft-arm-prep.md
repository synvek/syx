# C-0a+ — Unknown / ErrorType / InferenceVar trichotomy

**Status:** **Implemented** (2026-11-27) · windows **C-0a+1…+3**.  
**Parent:** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-0a+ `[x]`.  
**Census:** [`plan/harness/census-docs/c-0a-plus-soft-arm-census.md`](../../plan/harness/census-docs/c-0a-plus-soft-arm-census.md).

## What landed

1. **`TypeId::ErrorType`** + **`TypeId::InferenceVar`** (+ predicates `is_error_type` / `is_inference_hole`).
2. **Producers:** untyped params / empty collection unify / bare let / missing member → `InferenceVar`; invalid `super` / macro invoke → `ErrorType`.
3. **Soft arm:** deleted `(_, Unknown) => true`; kept `(_, InferenceVar) => true`.
4. **Still open:** `(_, Named("Object"))` HostFn façade soft arm (separate honesty window).
5. **Permanent:** expected-`Any` top; Array/Set inference-hole elem carve-outs.

## Non-claims

- ≠ HostFn Object façade rewrite  
- ≠ C-0e / C-3 `[x]` / C-5 `[x]`  
- ≠ automatic maturity semantic-dim bump (Object escape remains)
