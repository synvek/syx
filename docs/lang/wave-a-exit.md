# Wave A Structural Exit (Wave A W298)

> **Status:** **Wave A exit signed** (ADR-642 / W298) — plan/02 exit triad (Q-L2 + Q-S1 + CI differential) locked by prior sub-charters and top-level policy+meta this window.  
> Progress may claim **Wave A 退出条件已满足**. A-06～A-10 deepen exits were signed later (W345–W361); **A-01～A-10 are all `[x]`** (W361 / ADR-705). **Q-L5** (= B-08) is **checked** (W367 / ADR-711 exit charter; scope narrow ≠ production-default JIT).

## Exit triad (plan/02)

| Condition | Status | Sub-charter evidence |
|-----------|--------|----------------------|
| **Q-L2** claimed-supported no silent skip | **met** | ADR-641 / Suite-1 **47/47** + Suite-2 File + MirOp/capabilities pilot floor (`tests/differential/suite1.rs`); F-10 retired meta → [`plan/archive/gates/gate-a02-ql2-floor.md`](../../plan/archive/gates/gate-a02-ql2-floor.md) (W575 / ADR-919) |
| **Q-S1** core HostFn success+negatives+close matrix | **met** | ADR-640 / HostFn 五域 (`hostfn_{file,http,tcp,process,sql}.rs`) + [`hostfn_qs1_matrix.toml`](../../packages/internal/hostfn_qs1_matrix.toml)；F-10 retired meta → [`plan/archive/gates/gate-a05-qs1-floor.md`](../../plan/archive/gates/gate-a05-qs1-floor.md) (W576 / ADR-920) |
| **CI differential** stable complete | **met** | A-09 `[x]` + differential **16/16** green; Q-L4 floor via ADR-632 / [`gate-a04-edge-semantics.md`](../gates/gate-a04-edge-semantics.md) |

## Exit charter definition

Wave A structural exit = the three conditions above are signed. This window adds **policy + meta** only (`wave_a_exit.rs`); it does **not** clear Suite-2/3/4 wasm skips or raise remaining `native=partial` product rows (timeout_tls/tls.client raised separately in W343 / ADR-687). Axis deepen exits (A-06～A-10) are later windows; by W361 all A-* rows are `[x]`. Q-L5 / B-08 is a cross-Wave language threshold (thin→floor→exit W362–W367); structural exit (W298) did not require it; Q-L5 is now checked under scope narrow.

**Evidence nails:** `crates/syx-driver/tests/stdlib/wave_a_exit.rs` + plan/02 progress summary.

## Frozen

- Claiming Suite-2/3/4 wasm skip full clear as Wave A structural requirement
- Remaining `native=partial→real` batch (listener / pool/row / deque / synui / farm)
- Claiming production-default JIT / full MirOp / microbench GA as Q-L5 exit meaning
- B-04 / B-05 / B-06 `[x]`; Wave B production delivery

**Next:** W405 B-05 Medium production cutover honesty **已交**（ADR-749）→ W406+ — AOT sidecar embed deepen / dual-load runtime / TLS handler / status raise / absolute thresholds / Long MIR-free；Long MIR-free **out of scope**（≠ this window）.
