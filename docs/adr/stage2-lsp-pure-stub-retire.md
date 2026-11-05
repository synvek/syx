# ADR: Stage-2 S2-02 LSP pure stub + soft_pin retire + phase floor relax (S2-I120)

**Status:** Accepted (S2-I120) · S2-02 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I119 fingerprint_stub retire](stage2-lsp-fingerprint-stub-retire.md), remaining LSP soft-nail mass is pure `include_str!(self).contains(…)` `*_stub` tests plus colocated `*_soft_pin` (readme/manifest/adr). docs_honesty soft DELETE-safe Δ is only **3** (props×3 remain I108 KEEP) and is **not** the primary knife. Hybrid DI/SynUI stubs that exercise `pN_doc` / semantic probes stay.

## Decision

1. **Authorize deletion** of **136** pure `*_stub` `#[test]` functions under [`crates/syx-lsp/src/lsp_phase{125..139}_real.rs`](../../crates/syx-lsp/src/) (audit_align / stdlib_parity_pin / track_fingerprints / macro_docs honesty / farm·dist·synui·simd·macro·pointcut·props·lang feature-header soft, etc.).
2. **Authorize deletion** of **13** `*_soft_pin` tests: `p135_readme_banner_soft_pin`; p136–p139 each `manifest_soft_pin` + `adr*_charter_soft_pin` + `readme_banner_soft_pin`.
3. **Relax** `src.matches("#[test]").count()` floors (after pure+pins):
   - p129: **38 → 37**
   - p130: **40 → 37**
   - p131: **42 → 37**
   - p132: **44 → 37**
   - p133: **46 → 37**
   - p134: **46 → 36**
   - p135: **47 → 36**
   - p136: **52 → 40**
   - p137: **54 → 40**
   - p138: **54 → 40**
   - p139: **55 → 41**
4. **Do not** change floors for p125–p128 (headroom remains); **do not** rename hasher / `cache_v111` historical tags.
5. **Do not** change `driver_gate` floors (`#[test]≥450`, `behavioral≥350`) or `soft_debt_zero_meta_allowable_charter`.
6. **Keep** hybrid stubs ×36 (DI `pN_doc` probes + attr/SynUI semantic stubs); non-stub behavioral / contract / matrix; file-header fingerprint comments; props docs_honesty×3; product `*_docs_and_capability` / inventory caps; `cache_v111`; hasher.
7. **Expected:** S2-01 census KPIs remain **0**; ≈ −149 soft-nail tests.
8. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- LSP pure soft stub + soft_pin sea closes; follow-on historical pure soft retire: [S2-I121](stage2-lsp-historical-pure-soft-retire.md).
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-02-open](stage2-s2-02-open.md) · S2-I108
- [stage2-lsp-fingerprint-stub-retire](stage2-lsp-fingerprint-stub-retire.md) · S2-I119
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-02
