# ADR: Stage-2 S2-01 gate_b09 narrative peel (S2-I133)

**Status:** Accepted (S2-I133) · S2-01 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I132 LSP lib-pin](stage2-lsp-contract-lib-pin-retire.md), claim soft OUT seas are cleared. Remaining S2-01 blockers are product-path governance modules in `syx-rt` ([claim inventory](stage2-claim-readiness-inventory.md)). This window peels **`gate_b09_gc_narrative` only** — five policy-fixed constant-`true` flags with zero Rust call sites after [S2-I18 product-flag decouple](stage2-b09-product-flag-decouple.md). **Does not** peel `llvm_stub_audit`.

## Decision

1. **Delete** [`crates/syx-rt/src/gate_b09_gc_narrative.rs`](../../crates/syx-rt/src/gate_b09_gc_narrative.rs) and drop its `mod` / `pub use` from [`lib.rs`](../../crates/syx-rt/src/lib.rs).
2. **Removed product API names** (semantics archived here as historically policy-fixed `true`; emit/VM/LLVM never read these gates):
   - `objectref_rc_migrate_enabled`
   - `storage_cleanup_enabled`
   - `unified_mir_runtime_heap_enabled`
   - `b09_axis_exit_enabled`
   - `gc_ga_claimed`
3. **Supersede** the product-path reading of [stage2-b09-product-flag-decouple](stage2-b09-product-flag-decouple.md) “MUST KEEP flag API names” — policy is now ADR text, not a live `syx-rt` export surface.
4. **Keep** `llvm_stub_audit` (+ native / wave_b evidence); floors 450/350; `soft_debt_zero`; hasher; KEEP tip.
5. **Expected:** claim inventory gate blockers **2 → 1** (`llvm_stub_audit` only); census KPIs remain **0**; S2-01/S2-02 stay `[~]`; claim-readiness still **incomplete**.
6. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ claim-readiness **complete** / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ rewrite B-09 product semantics.

## Consequences

- Product `syx-rt` no longer exports B-09 narrative flags; governance surface shrinks.
- Next: `llvm_stub_audit` peel **or** ADR acceptance as product capability before S2-01满勾.
- Open: [stage2-open](stage2-open.md) · [stage2-claim-readiness-inventory](stage2-claim-readiness-inventory.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-b09-product-flag-decouple](stage2-b09-product-flag-decouple.md) · S2-I18
- [stage2-claim-readiness-inventory](stage2-claim-readiness-inventory.md)
- [stage2-lsp-contract-lib-pin-retire](stage2-lsp-contract-lib-pin-retire.md) · S2-I132
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01
