# Q-L5 / B-08 JIT Exit Charter (Wave A W367)

> **Status:** **Q-L5 / B-08 exit signed** (ADR-711 / W367) — thin-open + deepen×3 + floor locked by prior sub-charters; this window is **policy + meta** only with **explicit scope narrow**.  
> **Floor cross-index:** [`ql5-jit-floor.md`](ql5-jit-floor.md) / ADR-710.  
> **Vectors cross-index:** [`ql5-jit-vectors.md`](ql5-jit-vectors.md) / ADR-706–709.  
> Roadmap B-08 is `[x]`. Q-L5 checklist is **checked**. **Not** production-default JIT. **Not** full MirOp / microbench / full deopt table GA.

## Scope narrow (ADR-711)

Q-L5 / B-08 **complete** =
- thin-open (W362 / ADR-706) + deepen batch-1～3 (W363–365 / ADR-707–709) + floor (W366 / ADR-710)
- Cranelift tier-2 Suite-1 pilot + opt-in default-off **in scope / met**
- UnsupportedMirOp deopt 七钉 **partial met 即足**（**Not** 全量 deopt 表 GA）

Production-default JIT / full supported MirOp expand / microbench GA = **product defer**（clusters stay **unmet**; **Not** claimed **met**). Exit ≠ production-default JIT.

## Exit acceptance

| Prerequisite | Status | Evidence |
|--------------|--------|----------|
| Thin-open | **met** (W362 / ADR-706) | [`ql5-jit-vectors.md`](ql5-jit-vectors.md) + `jit_thin.rs` |
| Deepen×3 | **met** (W363–365 / ADR-707–709) | 七钉 deopt + `jit_deepen.rs` |
| Floor clusters | **met** (W366 / ADR-710) | [`ql5-jit-floor.md`](ql5-jit-floor.md) + `jit_floor_thin.rs` |
| Scope narrow exit | **signed** (W367 / ADR-711) | this document + ADR-711 |

Q-L5 / B-08 `[x]` is signed when the prerequisites above hold and this document's Status cites ADR-711. This window does **not** add new deopt nails and does **not** flip JIT to production default.

**Evidence nails:** `crates/syx-driver/tests/stdlib/jit_exit.rs` + plan/02 B-08 + Q-L5 flip.

## Frozen

- Claiming production-default JIT / full MirOp / wasm JIT GA / full deopt table GA / microbench GA
- Expanding **supported** MirOp set this window
- New deopt / HostFn behavioral nails this window
- Residual `native=partial→real` (listener / pool / row / deque)
- Website met / building website this window
- B-04 / B-05 / B-06 `[x]`; Wave B production delivery

**Next:** W405 B-05 Medium production cutover honesty **已交**（ADR-749）→ W406+ — AOT sidecar embed deepen / dual-load runtime / TLS handler / status raise / absolute thresholds / Long MIR-free；Long MIR-free **out of scope**（≠ this window）.
