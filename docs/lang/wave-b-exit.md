# Wave B Structural Exit (Wave A W340)

> **Status:** **Wave B exit signed** (ADR-684 / W340) — plan/02 Wave B exit triad (B-04 + B-05 + B-06 production delivery **or** honest defer) locked by prior sub-charters and top-level policy+meta this window.  
> Progress may claim **Wave B 退出条件已满足**. B-04/B-05/B-06 remain thin `[~]` (honest defer ≠ axis `[x]`).

## Exit triad (plan/02)

| Condition | Status | Sub-charter evidence |
|-----------|--------|----------------------|
| **B-04 Short 生产交付或诚实延后签收** | **诚实延后签收 met** | ADR-682 / [`wave-b-exit-triad-b04-defer.md`](wave-b-exit-triad-b04-defer.md) + `aot_short_horizon_honest_defer_inventory`（~~`gate_b04_short_honest_defer_thin.rs`~~ F-10 W589） |
| **B-05 Medium 生产交付或诚实延后签收** | **诚实延后签收 met** | ADR-683 / [`wave-b-exit-triad-b05-defer.md`](wave-b-exit-triad-b05-defer.md) + `mir_module_medium_horizon_honest_defer_inventory`（~~`gate_b05_medium_honest_defer_thin.rs`~~ F-10 W587） |
| **B-06 Missing 清零或诚实延后签收** | **诚实延后签收 met** | ADR-681 / [`wave-b-exit-triad-b06-defer.md`](wave-b-exit-triad-b06-defer.md) + `llvm_stub_missing_clear_honest_defer_inventory`（~~`gate_b06_missing_clear_honest_defer_thin.rs`~~ F-10 W585） |

Readiness floor (not the charter itself): ADR-680 / [`wave-b-exit-readiness.md`](wave-b-exit-readiness.md) + `WaveBStructuralExitReadinessInventory` (`wave_b_exit_enabled=false`; readiness ≠ charter).

## Exit charter definition

Wave B structural exit = the three conditions above are signed via production delivery **or** honest defer. This window adds **policy + meta** only (`wave_b_exit_thin.rs`); it does **not** flip B-04/B-05/B-06 to `[x]`, enable production multi-root / true size CI / real codec / dual-blob production / Missing stub clear, or raise remaining `native=partial` product rows.

**Evidence nails:** `crates/syx-driver/tests/native/wave_b_exit_thin.rs` + plan/02 progress summary.

## Frozen

- B-04 / B-05 / B-06 `[x]` (axis production delivery — separate deepen/exit)
- Production multi-root cutover; true size golden CI / absolute thresholds
- Real codec; dual-blob production embed/load; encode/load/AOT semantic change
- Missing 清零 (stub rewrite); absolute HostFn↔stub matrix exit; B-07
- A-06 / Q-S2 `[x]`; `native=partial→real` batch (timeout_tls / tls.client / …)
- Claiming readiness inventory flags enable production exit machinery (`wave_b_exit_enabled` stays false on readiness inventory)

**Next:** W342+ — native hang/HTTPS AOT nails → A-06/Q-S2 exit charter.
