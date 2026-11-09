# ADR: SA (stdlib Swift alignment) exit

**Status:** Accepted  
**Date:** 2026-11-09  
**Context:** SA-0…SA-5 scoped exits are landed (`plan/01` · `plan/02`). SA-6 exit hinge is census KPI green ([`sa-6-exit-census.md`](../census/sa-6-exit-census.md) · [`scripts/sa_6_exit_census.sh`](../../scripts/sa_6_exit_census.sh)) plus this ADR. Aligned with [stage4-exit](stage4-exit.md) honesty (scoped claim + residual list; ≠ boiling the ocean).

## Decision

1. **SA alignment period exits.** `plan/01` and `plan/02` mark **SA-6** and the SA track as **`[x]` 退出** as of this ADR.
2. **Evidence (In):**
   - SA-0…SA-5 exit hinges `[x]` (charter · prelude · language · module remap · System API/host · docs/doctest)
   - **SA-6-01** `[x]` scoped — [`sa-6-exit-census.md`](../census/sa-6-exit-census.md):
     - prelude / importable SSOT match
     - old import lines = 0 (packages / examples / books)
     - public `__syx_*` (non-internal) = 0
     - System/IO **`File.stat` / `File.readAsync`** cleared off `Object` (→ `Dictionary`)
   - **SA-6-03** `[x]` — this ADR
3. **Object KPI honesty (supersedes literal “public Object signatures = 0”):**
   - Hard green = named System/IO slice above
   - Remaining public `: Object` / `-> Object` counted as `object_sig_residual_count` → **post-SA Residual** (≠ SA-4-03 full clear this flip)
4. **Residual (post-exit; does not block this flip):**
   - Public Object residual inventory (simd / Channel / JSON / Networking / …)
   - `FileHandle.withOpen(body: Object)` closure/host unify
   - Channel Host surface / trySend runtime
   - Collections merge (OrderedSet/Deque) / Testing `@Test`
   - `syx migrate` product CLI; SA-5-06 manifest↔stability wiring
   - SA-5-02 language-guide full sweep; SA-5-04 tools deepen
   - SA-4-03…07 deep package Swiftify / async productization
   - Parked: Foundation naming · F-08 · product umbrella · HostFn+1 as KPI
5. **Non-claims:**
   - ≠ SA-7 / new SA plan row
   - ≠ full-library Object=0
   - ≠ Foundation / F-08 / product-umbrella unpark
   - ≠ remove user-package `SyxHost.*` calls

## Consequences

- Default next = **post-SA Residual / 维护** (same semantic as credibility post-exit Residual / 维护).
- Marketing / capability text may say **SA alignment exited** only with the residual list kept honest.
- Census Current → SA-6 exited; next maintenance tips at residual backlog, not a new SA stage.

## References

- Charter: [stdlib-swift-alignment](stdlib-swift-alignment.md)
- Census: [sa-6-exit-census](../census/sa-6-exit-census.md) · [sa-5-docs-tools](../census/sa-5-docs-tools.md)
- Precedent: [stage4-exit](stage4-exit.md)
