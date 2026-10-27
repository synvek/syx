# Q-L5 / B-08 JIT Floor (Wave A W366; exit W367)

> **Status:** **Q-L5 / B-08 floor signed** (ADR-710 / W366) — Cranelift tier-2 Suite-1 pilot + opt-in default-off **met**; deopt UnsupportedMirOp（七钉）**partial met**; more MirOp / microbench / production-default **unmet**.  
> **Exit cross-index:** **Q-L5 / B-08 exit signed** ([`ql5-jit-exit.md`](ql5-jit-exit.md) / ADR-711 / W367) via scope narrow (production-default / full MirOp / microbench deferred unmet).  
> **W362–W365 cross-index:** thin-open ([`ql5-jit-vectors.md`](ql5-jit-vectors.md) / ADR-706) + deepen batch-1～3（ADR-707–709）.  
> Roadmap B-08 is `[x]` (W367). Q-L5 checklist is **checked** (W367). **Not** production-default JIT. Floor ≠ exit (exit signed separately).

## Floor clusters

| Cluster | Status | Evidence |
|---------|--------|----------|
| **Cranelift tier-2 Suite-1 pilot** | **met** | `crates/syx-cranelift-jit`；`packages/internal/capabilities.toml` `runtime.jit.tier2`；`crates/syx-driver/tests/lang/jit_suite1_differential.rs`；ADR-140 |
| **Opt-in 默认关** | **met** | `SYX_JIT_TIER2` / `--jit`；[`plan/topics/jit.md`](../../plan/topics/jit.md) |
| **Deopt UnsupportedMirOp（七钉）** | **partial met** | `jit_print_deopts`；`jit_callhost_deopts`；`jit_conststr_deopts`；`jit_arraybuild_deopts`；`jit_dictbuild_deopts`；`jit_setbuild_deopts`；`jit_loadglobal_deopts` |
| **更多 MirOp / 微基准 / 生产默认** | **unmet** | B-08 正文债；exit via scope narrow **defers** production-default / full MirOp / microbench |

## Floor charter definition

Q-L5 / B-08 **floor** is signed when the clusters above are indexed and this document's Status cites ADR-710. **Exit** (`[x]`) is signed separately by ADR-711 / [`ql5-jit-exit.md`](ql5-jit-exit.md) via explicit scope narrow. Floor ≠ exit.

**Evidence nails:** `crates/syx-driver/tests/stdlib/jit_floor_thin.rs` + `jit_thin.rs` + `jit_deepen.rs` + [`ql5-jit-vectors.md`](ql5-jit-vectors.md).

## Frozen

- Claiming production-default JIT / full MirOp / wasm JIT GA / full deopt table GA / microbench GA
- Expanding **supported** MirOp set this window
- New deopt / HostFn behavioral nails this window
- Residual `native=partial→real` (listener / pool / row / deque)
- Website met / building website this window
- B-04 / B-05 / B-06 `[x]`; Wave B production delivery

**Next:** W405 B-05 Medium production cutover honesty **已交**（ADR-749）→ W406+ — AOT sidecar embed deepen / dual-load runtime / TLS handler / status raise / absolute thresholds / Long MIR-free；Long MIR-free **out of scope**（≠ this window）.
