# A-08 DateTime Vectors (Wave A W352 thin-open; W353–W355 deepen; W356 floor; W357 exit)

> **Status:** **A-08 thin floor signed** (ADR-696 / W352) + **W353–W355 DateTime deepen** (ADR-697 / ADR-698 / ADR-699) + **Q-S4 floor signed** (ADR-700 / W356；[`gate-a08-qs4-floor.md`](../../plan/archive/gates/gate-a08-qs4-floor.md)；F-10 W583 archived) + **Q-S4 / A-08 exit signed** (ADR-701 / W357；[`gate-a08-qs4-exit.md`](../../plan/archive/gates/gate-a08-qs4-exit.md)；F-10 W582 archived) — inventory indexed; six exact nails landed; floor clusters met; exit charter signed; IANA deepen + calendar deepen **partial met** (not full IANA / leap-second GA).  
> Roadmap A-08 is `[x]`; Q-S4 is checked. **Not** all A-* `[x]` (A-10 remains open).  
> **Exit cross-index:** [`gate-a08-qs4-exit.md`](../../plan/archive/gates/gate-a08-qs4-exit.md) / ADR-701（F-10 W582 archived）.  
> **Floor cross-index:** [`gate-a08-qs4-floor.md`](../../plan/archive/gates/gate-a08-qs4-floor.md) / ADR-700（F-10 W583 archived）.  
> **W351 cross-index:** A-07 / Q-S3 exit signed ([`gate-a07-qs3-exit.md`](../../plan/archive/gates/gate-a07-qs3-exit.md) / ADR-695；F-10 archived).

## Scope

| Cluster | Status | Evidence (existing nails) |
|---------|--------|---------------------------|
| **Gregorian / RFC3339 calendar** | **in scope** | `crates/syx-driver/tests/platform/datetime_gregorian.rs`; `crates/syx-driver/tests/stdlib/iso_rfc3339_roundtrip.rs`; `packages/internal/capabilities.toml` `datetime.calendar` |
| **Instant / Duration / Clock** | **in scope** | `crates/syx-driver/tests/stdlib/instant_duration.rs`; `datetime.clock` caps |
| **IANA / DST narrow（NY）** | **in scope** | `crates/syx-driver/tests/stdlib/iana_tzdb_dst.rs`; W221–W223 HostFn (`DateTimeTimezoneOffset` / `DateTimeFormatWithTimezone`); `datetime.timezone` caps |
| **Package surface** | **in scope** | `packages/datetime/mod.sy`; fixtures `datetime_iso.sy` / `parse_format.sy` |
| **IANA deepen（第二+第三 zone）** | **partial met** (W353–W355 / ADR-697 / ADR-698 / ADR-699) | `datetime_iana_asia_tokyo_offset`; `datetime_iana_format_with_timezone_tokyo`; `datetime_iana_europe_london_winter_offset`; `datetime_iana_europe_london_summer_offset` — **Not** full IANA / leap-second / full tz GA |
| **历法集加深 / 非 Gregorian** | **partial met** (W353–W354 / ADR-697 / ADR-698) | `gate_a08_calendar_reject_2023_feb29`; `gate_a08_calendar_reject_2023_apr31` — invalid Gregorian fail-closed；**Not** 非 Gregorian calendar GA |
| **全量 IANA / leap second / full tz GA** | **unmet** | caps notes already mark ≠ |

## Thin-open + deepen + floor + exit definition

Q-S4 **thin floor** = W352 indexes pre-existing nails（thin meta retired F-10 W583）. W353–W355 add **exact HostFn behavioral nails** (6 cumulative). W356 signs **Q-S4 floor**（[`gate-a08-qs4-floor.md`](../../plan/archive/gates/gate-a08-qs4-floor.md)；F-10 W583 archived）. W357 signs **exit** (`[x]`；[`gate-a08-qs4-exit.md`](../../plan/archive/gates/gate-a08-qs4-exit.md)；F-10 W582 archived). Exit ≠ full IANA / leap-second GA.

**Evidence nails:** `crates/syx-driver/tests/stdlib/datetime_deepen.rs` + platform nails（datetime_gregorian / iso_rfc3339 / iana_tzdb / instant_duration / packages/datetime） + roadmap A-08 `[x]`（F-10 W582–W583：qs4_exit + thin/floor meta retired）.

## Frozen

- Claiming all A-* rows `[x]` (A-10 still open)
- Leap-second / full IANA tz GA / 非 Gregorian calendar GA claim
- Residual `native=partial→real` (listener / pool / row / deque)
- more RFC honesty HostFn nails this window
- New HostFn behavioral nails this window
- B-04 / B-05 / B-06 `[x]`; Wave B production delivery

**Next:** W405 B-05 Medium production cutover honesty **已交**（ADR-749）→ W406+ — AOT sidecar embed deepen / dual-load runtime / TLS handler / status raise / absolute thresholds / Long MIR-free；Long MIR-free **out of scope**（≠ this window）.
