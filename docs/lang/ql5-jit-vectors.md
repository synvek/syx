# Q-L5 / B-08 JIT Vectors (Wave A W362 thin + W363–W365 deepen + W366 floor + W367 exit)

> **Status:** **Q-L5 / B-08 thin floor signed** (ADR-706 / W362) + **deepen batch-1** (ADR-707 / W363) + **deepen batch-2** (ADR-708 / W364) + **deepen batch-3** (ADR-709 / W365) + **floor signed** (ADR-710 / W366) + **exit signed** (ADR-711 / W367) — Cranelift tier-2 Suite-1 pilot + opt-in default-off indexed; deopt reasons **partial met** (Print + CallHost + ConstStr + ArrayBuild + DictBuild + SetBuild + LoadGlobal exact); more MirOp / microbench / production-default **unmet** (deferred under exit scope narrow).  
> Roadmap B-08 is `[x]`. Q-L5 checklist is **checked**. **Not** production-default JIT. Exit ≠ production-default JIT.  
> **W361 cross-index:** A-10 / M0-C05 exit signed ([`a10-capability-exit.md`](a10-capability-exit.md) / ADR-705); Wave A axes A-01～A-10 all `[x]`.  
> **Floor / exit cross-index:** [`ql5-jit-floor.md`](ql5-jit-floor.md) / ADR-710；[`ql5-jit-exit.md`](ql5-jit-exit.md) / ADR-711.

## Scope

| Cluster | Status | Evidence |
|---------|--------|----------|
| **Cranelift tier-2 Suite-1 pilot** | **in scope** | `crates/syx-cranelift-jit`；`packages/internal/capabilities.toml` `runtime.jit.tier2`；`crates/syx-driver/tests/lang/jit_suite1_differential.rs`；ADR-140 |
| **Opt-in 默认关** | **in scope** | `SYX_JIT_TIER2` / `--jit`；[`plan/topics/jit.md`](../../plan/topics/jit.md) |
| **Deopt 原因（UnsupportedMirOp）** | **partial met** | `jit_print_deopts` + **W363** `jit_callhost_deopts` + `jit_conststr_deopts` + **W364** `jit_arraybuild_deopts` + `jit_dictbuild_deopts` + **W365** `jit_setbuild_deopts` + `jit_loadglobal_deopts`；crate 对称；**Not** 全量 MirOp deopt 表 GA |
| **更多 MirOp / 微基准 / 生产默认** | **unmet** | B-08 正文债；exit scope narrow defers |

## Thin-open / deepen / floor / exit definition

Q-L5 / B-08 **thin floor** indexes Suite-1 evidence (`jit_thin.rs`). **W363–W365 deepen** adds CallHost/ConstStr/ArrayBuild/DictBuild/SetBuild/LoadGlobal deopt exact. **W366 floor** indexes thin+deepen. **W367 exit** signs Q-L5/B-08 `[x]` under scope narrow ([`ql5-jit-exit.md`](ql5-jit-exit.md)). Exit ≠ production-default JIT GA.

**Evidence nails:** `jit_thin.rs` + `jit_deepen.rs` + `jit_floor_thin.rs` + `jit_exit.rs` + roadmap B-08 `[x]` + Q-L5 checked.

## Frozen

- Claiming production-default JIT / full MirOp / wasm JIT GA / full deopt table GA / microbench GA
- Expanding **supported** MirOp set this window
- Residual `native=partial→real` (listener / pool / row / deque)
- Website met / building website this window
- B-04 / B-05 / B-06 `[x]`; Wave B production delivery

**Next:** W405 B-05 Medium production cutover honesty **已交**（ADR-749）→ W406+ — AOT sidecar embed deepen / dual-load runtime / TLS handler / status raise / absolute thresholds / Long MIR-free；Long MIR-free **out of scope**（≠ this window）.
