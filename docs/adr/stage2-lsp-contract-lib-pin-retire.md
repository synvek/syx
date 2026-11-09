# ADR: Stage-2 S2-02 LSP contract lib-pin retire (S2-I132)

**Status:** Accepted (S2-I132) · S2-02 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I131 cross-crate residual soft](stage2-cross-crate-residual-soft-retire.md), claim-inventory soft OUT seas are cleared except LSP non-KEEP ≈6 — nested `include_str!(syx-driver/src/lib.rs).contains("lsp-semantic-v*")` pins inside KEEP semantic `*_contract` tests ([claim inventory](stage2-claim-readiness-inventory.md)). Driver hasher already covers the same tags. This window strips those redundant pins without deleting contracts. **≠** S2-02 `[x]` / **≠** claim-readiness complete / **≠** S2-01 gate peel.

## Decision

1. **Strip** from `p115`–`p120` and `p107` semantic `*_contract` tests the block:
   - `let lib = include_str!("../../syx-driver/src/lib.rs");`
   - `assert!(lib.contains("lsp-semantic-v*…"));`
2. **Keep** all remaining contract matrix / `tests>=N` / pad assertions; DI×6; v13×4 stubs; hasher bytes in driver `lib.rs`; floors 450/350; stdlib/platform KEEP tip.
3. **Do not** peel `syx-rt` `gate_b09_gc_narrative` / `llvm_stub_audit`.
4. **Expected:** claim inventory LSP OUT soft → **0**; residual OUT = S2-01 gate modules; inventory still **incomplete**.
5. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ claim-readiness **complete** / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- Soft OUT seas for claim inventory are cleared; gate peel remains the blocker for S2-01满勾.
- Open: [stage2-open](stage2-open.md) · [stage2-claim-readiness-inventory](stage2-claim-readiness-inventory.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-02-open](stage2-s2-02-open.md)
- [stage2-lsp-hybrid-fingerprint-stub-retire](stage2-lsp-hybrid-fingerprint-stub-retire.md) · S2-I123
- [stage2-cross-crate-residual-soft-retire](stage2-cross-crate-residual-soft-retire.md) · S2-I131
- [stage2-claim-readiness-inventory](stage2-claim-readiness-inventory.md)
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-02
