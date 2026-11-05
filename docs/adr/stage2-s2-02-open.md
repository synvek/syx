# ADR: Stage-2 S2-02 open — behavior-driven tests (S2-I108)

**Status:** Accepted (S2-I108) · S2-02 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I107 lang_props retire](stage2-lang-props-retire.md), S2-01 census KPIs are all **0**. Remaining Stage-2 default work is [plan/02 S2-02](../../plan/02-improvement-roadmap.md): retire `include_str!(product src)+contains` soft-nail as the primary test path in favor of `.sy` / behavioral asserts. S2-01 stays `[~]` (hasher / gate narrative / remaining soft-nail sea).

## Decision

1. **Open S2-02** as an active Stage-2 workstream without claiming S2-02 `[x]` or S2-01 `[x]`.
2. **S2-I108 first knife** — delete redundant props fingerprint soft-nail tests that only `include_str!(lib.rs).contains("lang-props-v*:…")` (duplicated by `cache_v108`–`v111` props nails); **keep** all props behavior tests and docs honesty markers.
3. **Do not** delete `cache_v*` / `driver_gate` / LSP phase stubs / hasher bytes / `lang_surface_v1` in this window (deferred).
4. Later S2-02 knives under sibling notes without claiming S2-02 `[x]` until soft-nail is no longer the main path — **S2-I109…I125** as prior; **S2-I126:** [stdlib M2 cap/ci_wires soft×79](stage2-stdlib-m2-cap-ci-wires-soft-retire.md).
5. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- S2-02 moves `[ ]` → `[~]`; props product-src fingerprint soft-nails shrink by five tests.
- **S2-I109:** historical `cache_v108`–`v110` soft-nail files retired; `cache_v111` tip kept.
- **S2-I110:** `driver_gate` pure exists proxies + empty charters retired (≈−32).
- **S2-I111:** `driver_gate` heap/cache invalidation-exact + tag soft retired (≈−11).
- **S2-I112:** `driver_gate` `host_registry_*` registry soft-nails retired (≈−5).
- **S2-I113:** meta_allowable×2 + `native_llvm_locals_v7` retired (≈−3); `soft_debt_zero` kept.
- **S2-I114:** include_str import/surface soft×25 retired; `#[test]` floor 500→450; behavioral≥350 kept.
- **S2-I115:** manifest/audit/codable residual soft×7 retired; floor stays 450; behavioral≥350 kept.
- **S2-I116:** backends honesty_fences + colocated inventory_meta/ci_wires soft×28 retired; docs_honesty/LSP deferred.
- **S2-I117:** backends criterion+microbench soft×24 retired; keep smoke_ok/instant; availability/docs/LSP deferred.
- **S2-I118:** hostfn_availability modules×4 + ga6/ga7 inventory_meta×2 retired (≈−18); docs_honesty/LSP deferred.
- **S2-I119:** LSP `*_fingerprint_stub`×45 retired; phase134/135/138/139 `tests>=N` relaxed; docs_honesty deferred.
- **S2-I120:** LSP pure `*_stub`×136 + `*_soft_pin`×13 retired; phase129–139 `tests>=N` relaxed; hybrid×36 kept; claim-readiness deferred.
- **S2-I121:** LSP historical pure soft×117 retired; phase129–131 `tests>=N` relaxed; hybrid×36 kept; backends column pins deferred.
- **S2-I122:** backends wasm column/doc pins×47 retired; keep `run_*`; hybrid×36 kept; claim-readiness deferred.
- **S2-I123:** LSP hybrid fingerprint stubs×26 retired; phase128–135 floors relaxed; KEEP DI×6+v13×4; claim-readiness deferred.
- **S2-I124:** platform fingerprint/surface soft×31 retired; KEEP cache_v111 tip; claim-readiness deferred.
- **S2-I125:** cross-crate surface/fp/playwright soft×43 retired (gui/runtime/data/native/browser); KEEP cache_v111 tip; soft residual (stdlib) → claim-readiness still deferred.
- **S2-I126:** stdlib M2 host_fn_cap/ci_wires/registry/surface soft×79 retired; KEEP inventory/docs_and_capability; residual hostfn_sql/residual_thin/product_caps → claim-readiness still deferred.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-lang-props-retire](stage2-lang-props-retire.md) · S2-I107
- [stage2-cache-soft-nail-retire](stage2-cache-soft-nail-retire.md) · S2-I109
- [stage2-driver-gate-exists-soft-nail-retire](stage2-driver-gate-exists-soft-nail-retire.md) · S2-I110
- [stage2-driver-gate-invalidation-exact-retire](stage2-driver-gate-invalidation-exact-retire.md) · S2-I111
- [stage2-driver-gate-host-registry-soft-nail-retire](stage2-driver-gate-host-registry-soft-nail-retire.md) · S2-I112
- [stage2-driver-gate-meta-allowable-native-locals-retire](stage2-driver-gate-meta-allowable-native-locals-retire.md) · S2-I113
- [stage2-driver-gate-include-str-surface-retire](stage2-driver-gate-include-str-surface-retire.md) · S2-I114
- [stage2-driver-gate-manifest-audit-codable-residual-retire](stage2-driver-gate-manifest-audit-codable-residual-retire.md) · S2-I115
- [stage2-backends-honesty-fences-retire](stage2-backends-honesty-fences-retire.md) · S2-I116
- [stage2-backends-criterion-microbench-soft-retire](stage2-backends-criterion-microbench-soft-retire.md) · S2-I117
- [stage2-backends-hostfn-availability-ga67-meta-retire](stage2-backends-hostfn-availability-ga67-meta-retire.md) · S2-I118
- [stage2-lsp-fingerprint-stub-retire](stage2-lsp-fingerprint-stub-retire.md) · S2-I119
- [stage2-lsp-pure-stub-retire](stage2-lsp-pure-stub-retire.md) · S2-I120
- [stage2-lsp-historical-pure-soft-retire](stage2-lsp-historical-pure-soft-retire.md) · S2-I121
- [stage2-backends-wasm-column-doc-pin-retire](stage2-backends-wasm-column-doc-pin-retire.md) · S2-I122
- [stage2-lsp-hybrid-fingerprint-stub-retire](stage2-lsp-hybrid-fingerprint-stub-retire.md) · S2-I123
- [stage2-platform-fingerprint-surface-soft-retire](stage2-platform-fingerprint-surface-soft-retire.md) · S2-I124
- [stage2-cross-crate-surface-soft-retire](stage2-cross-crate-surface-soft-retire.md) · S2-I125
- [stage2-stdlib-m2-cap-ci-wires-soft-retire](stage2-stdlib-m2-cap-ci-wires-soft-retire.md) · S2-I126
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-02






