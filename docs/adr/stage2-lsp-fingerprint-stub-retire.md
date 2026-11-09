# ADR: Stage-2 S2-02 LSP fingerprint_stub soft retire + phase floor relax (S2-I119)

**Status:** Accepted (S2-I119) · S2-02 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I118 backends availability close](stage2-backends-hostfn-availability-ga67-meta-retire.md), remaining large soft-nail seas include `syx-lsp` `*_fingerprint_stub` tests (`include_str!(self).contains(…)`) across `lsp_phase124`–`139`. Several late-phase `tests >= N` contracts sit on the floor and would fail after stub deletion without coordinated relax.

## Decision

1. **Authorize deletion** of all **45** `*_fingerprint_stub` `#[test]` functions under [`crates/syx-lsp/src/lsp_phase{124..139}_real.rs`](../../crates/syx-lsp/src/).
2. **Relax** `src.matches("#[test]").count()` floors in four contracts only:
   - `p134_lsp_semantic_v36_contract`: **48 → 46**
   - `p135_lsp_semantic_v37_contract`: **48 → 47**
   - `p138_lsp_semantic_v40_contract`: **56 → 54**
   - `p139_lsp_semantic_v41_contract`: **58 → 55** (also repairs pre-existing 57&lt;58)
3. **Do not** change other phase floors; **do not** rename hasher / `cache_v111` historical tags.
4. **Do not** change `driver_gate` floors (`#[test]≥450`, `behavioral≥350`) or `soft_debt_zero_meta_allowable_charter`.
5. **Keep** non-stub LSP behavioral / contract tests; file-header fingerprint comments; docs_honesty; `cache_v111`; hasher.
6. **Expected:** S2-01 census KPIs remain **0** (`_fingerprint_stub` ≠ census `fn *_fingerprint\b`); ≈ −45 soft-nail tests.
7. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- LSP fingerprint_stub sea closes; follow-on pure stub/soft_pin retire: [S2-I120](stage2-lsp-pure-stub-retire.md).
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-02-open](stage2-s2-02-open.md) · S2-I108
- [stage2-backends-hostfn-availability-ga67-meta-retire](stage2-backends-hostfn-availability-ga67-meta-retire.md) · S2-I118
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-02
