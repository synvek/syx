# A-06 Exit Readiness (Wave A W341–W345)

> **Status:** **A-06 / Q-S2 exit signed** (ADR-685–689 / W341–W345) — exit readiness triad 3/3 floor met (W295/W342/W344) + native raise (W343); public `GateA06ExitReadinessInventory` / `gate_a06_exit_readiness_inventory` with `a06_exit_enabled=true` / `qs2_complete_enabled=true` / `native_raise_enabled=true`.  
> **Floor (archived F-10 W579):** [`gate-a06-qs2-floor.md`](../../plan/archive/gates/gate-a06-qs2-floor.md) / ADR-688.  
> **Thin negatives (archived F-10 W579):** [`gate-a06-network-negatives.md`](../../plan/archive/gates/gate-a06-network-negatives.md) / ADR-639.  
> **Exit charter (archived F-10 W578):** [`gate-a06-qs2-exit.md`](../../plan/archive/gates/gate-a06-qs2-exit.md) / ADR-689.  
> Roadmap A-06 is `[x]`; Q-S2 is checked. **Not** all A-* `[x]` (A-07/A-08/A-10 open).

## Exit readiness triad (floor met; exit signed W345)

| Condition | Status | Evidence |
|-----------|--------|----------|
| **Thin floor (timeout/TLS/refuse/Display)** | **met** (W295 / ADR-639) | product nails in `http_client_timeout_tls.rs` / `http_client_tls.rs` / `hostfn_http.rs` / `hostfn_tcp.rs` / error_tcp*（thin meta retired W579） |
| **Native hang + HTTPS/EXTRA_ROOT AOT nails** | **met** (W342 / ADR-686) | `m2_s4_02b_timeout_hang_loopback_native_aot`; `m2_s4_05_https_loopback_ok_with_extra_root_native_aot`; `m2_s4_05_self_signed_fail_closed_native_aot` |
| **Full loopback + malformed GA floor** | **met** (W344 / ADR-688) | `m2_s4_a06_malformed_truncated_response_loopback`; `m2_s4_a06_malformed_bad_status_line_loopback` |

## Scope

| Item | Status | Notes |
|------|--------|-------|
| Thin-floor inventory | **in scope (W295)** | five clusters indexed |
| Exit readiness inventory | **in scope (W341–W345)** | `GateA06ExitReadinessInventory` |
| Native hang/HTTPS AOT nails | **in scope (W342)** | native strict exe |
| `timeout_tls` / `tls.client` native raise | **in scope (W343)** | product `native=real` |
| Malformed / full-loopback floor | **in scope (W344)** | wire R003 + loopback index |
| A-06 / Q-S2 exit met claim | **in scope (W345)** | policy+meta charter |

## Exit readiness inventory deepen (W341–W345)

| Item | Delivered | Not delivered |
|------|-----------|---------------|
| `GateA06ExitReadinessInventory` | five preds + honesty flags | all A-* `[x]` |
| `gate_a06_exit_readiness_inventory()` | thin + AOT + raise + malformed + **exit on** | A-07/A-08/A-10 |
| syx-rt tests | five preds true; exit/qs2 on; raise on | new behavior nails |

Honesty: exit signed — `a06_exit_enabled=true`; `qs2_complete_enabled=true`; `native_raise_enabled=true`. Floor triad **3/3 met**. **Not** all A-* `[x]`.

**Evidence nails:** `crates/syx-rt/src/gate_a06_exit_readiness.rs` + six-domain product nails + plan/02 A-06 progress（F-10 W578–W579：qs2_exit + thin/floor meta retired；Inventory 仍活）.

## Frozen

- Claiming all A-* rows `[x]` / Wave C open as this charter
- New wire / TLS / timeout behavioral nails this window
- TLS server GA; websocket; listener accept; pool/row
- B-04 / B-05 / B-06 `[x]`; production multi-root; true codec; Missing 清零

**Next:** W346+ — A-07 Crypto thin-open or residual.
