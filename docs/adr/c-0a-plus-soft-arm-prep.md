# C-0a+ / C-0v — Unknown / ErrorType / InferenceVar + Object soft arms

**Status:** **Implemented** (2026-11-27).  
**Parent:** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md).  
**Census:** [`c-0a-plus-soft-arm-census`](../../plan/harness/census-docs/c-0a-plus-soft-arm-census.md) · [`c-0v-hostfn-object-facade-census`](../../plan/harness/census-docs/c-0v-hostfn-object-facade-census.md).

## Landed

1. **`TypeId::{ErrorType,InferenceVar}`** + hole/error predicates.
2. **C-0a+3:** deleted `(_, Unknown)` soft arm; kept `(_, InferenceVar)`.
3. **C-0v3:** deleted global `(_, Named("Object"))` soft arm; Object→ground → **S004**.
4. **C-0v residual:** narrow `Dictionary ← Object` carve-out (SA-6-01 façades) until FileStat/FileReadAsync/TcpReadAsync can RetypeGround without MIR016.
5. **Permanent:** expected-`Any`; expected-`Object` ← Dictionary; Array/Set inference-hole elems.

## Non-claims

- ≠ FileStat host_fn_return_type → Dictionary (MIR016 / C-3p park)  
- ≠ C-0e / C-3 `[x]` / C-5 `[x]`  
- ≠ maturity composite bump beyond cautious semantic 2.2
