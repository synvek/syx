# R-0 residual census (post-SA)

**Status:** R-0 exit hinge met (R-0-01 + R-0-02 `[x]`) · R-1 also met · next = **R-2 Channel**  
**Date:** 2026-11-09  
**Script:** [`scripts/r0_residual_census.sh`](../../scripts/r0_residual_census.sh) · SA exit: [`docs/adr/sa-exit.md`](../adr/sa-exit.md)

## Exit hinge (R-0)

| ID | Claim | Evidence |
|----|-------|----------|
| R-0-01 | Residual backlog SSOT + census | This doc + script; `plan/02` 阶段 R |
| R-0-02 | Networking `TcpStream.readAsync` off Object | KPI below |

## KPI snapshot

| KPI | Value | Notes |
|-----|-------|-------|
| `sa6_hinge_still_green` | **1** | Rollup of [`sa_6_exit_census.sh`](../../scripts/sa_6_exit_census.sh) |
| `networking_tcp_readasync_object_cleared` | **1** | `TcpStream.readAsync` / host `tcp_read_async` ≠ Object |
| `filehandle_withopen_object_residual` | **0** | Cleared in R-1 (`body: Function`) |
| `object_sig_residual_count` | **>0** (via sa_6) | Honest inventory; ≠ full-library Object=0 |

### Networking Object slice (R-0-02)

| API | Was | Now |
|-----|-----|-----|
| `TcpStream.readAsync` | `throws -> Object` | `throws -> Dictionary` **cleared** |
| `SyxHost.tcp_read_async` | `-> Object` | `-> Dictionary` **cleared** |

Sources: [`packages/networking/mod.sy`](../../packages/networking/mod.sy) · [`packages/net/tcp.sy`](../../packages/net/tcp.sy) (embedded `NET_SOURCE`) · [`packages/internal/system_host.sy`](../../packages/internal/system_host.sy).

## Scoped honesty

- Hard green: named Networking tcp readAsync slice above.
- Remaining Object (simd / Channel / WS / withOpen / …) stays inventory → **R-1+**.
- **≠ SA-7** — R-* is post-SA maintenance ledger only.

## How to run

```bash
bash scripts/r0_residual_census.sh
```

## Next

**R-2** — Channel `send`/`trySend` Object 切片（见 `plan/02` 阶段 R）。
