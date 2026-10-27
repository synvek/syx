# syx-driver integration tests

User-facing domain and surface harnesses. Run a whole domain with:

```bash
cargo test -p syx-driver --test <harness> -- --test-threads=1
```

## Domain harnesses (product nails)

| Target | Path | What to look for |
|--------|------|------------------|
| `evergreen` | `evergreen/` | Smoke, parity, registry, `build_syx*`, fixtures, plugin load |
| `lang` | `lang/` | Borrow / move, protocols, inheritance, operators, model v1, MIR pilot |
| `concurrency` | `concurrency/` | Tasks, actors, channels, pools, mutex/semaphore, race/chaos |
| `stdlib` | `stdlib/` | Collections, string/time/JSON/HTTP/SQL/TCP tools, leak live-counts, error facades |
| `observability` | `observability/` | Structured logging, trace/span, metrics, profiler |
| `security` | `security/` | Permission model/grants/FS, HostFn capability gate |
| `backends` | `backends/` | Native/WASM hostfn availability, matrix/GA tips, microbench/criterion gates |

## Surface harnesses (backends / platforms)

| Target | Path | Scope |
|--------|------|--------|
| `gui` | `gui/` | guidemo parity, winit, SynUI tip |
| `wasm` | `wasm/` | AOT / mir_exec·run_op / hybrid AOT / runtime smoke |
| `browser` | `browser/` | Playwright tier-2\* + compile/e2e |
| `native` | `native/` | LLVM stack, link, strict parity, build profile/LTO |
| `data` | `data/` | Codable / ORM / SQL / XML |
| `farm` | `farm/` | Lease tip + Raft / TCP / billing |
| `runtime` | `runtime/` | async, GC, net, HOF, reflect, select |
| `platform` | `platform/` | Driver gate, Service DI, cache, dist, plugins |
| `differential` | `differential/` | Four-backend oracle |

## Hygiene

- Prefer product-semantic file names (`leak_file_handle.rs`, `exclusive_borrow.rs`). Historical window ids live in a one-line `// was: …` marker.
- Shared `.sy` sources for crate unit tests live under `fixtures/lang/` and `fixtures/parity/` (not Phase-window names at `tests/` root).
- Do **not** add new top-level `m2_*` / `m3_*` / `m4_*` / `ec_m*` crates.
- Do **not** reintroduce `*_closeout.rs` or numbered ADR charter pins.
- Architecture SSOT: [`docs/adr/architecture-decisions.md`](../../../docs/adr/architecture-decisions.md).
- Behavioral count floor: [`scripts/audit_behavioral_tests.sh`](../../../scripts/audit_behavioral_tests.sh).

## Old name → harness (short)

| Old top-level | Now |
|---------------|-----|
| `m2_*` / `ec_m2_1_*` / `ec_m2_3_*` | `--test stdlib` |
| `ec_m2_4_*` | `--test backends` |
| `m3_a*` / `m3_b*` / `m3_c*` / `concurrency.rs` | `--test concurrency` |
| `m3_d*` | `--test observability` |
| `m3_e*` | `--test security` |
| `m4_*` / `model_v1_nails` / `ec_m4_b_named_move` (behavior) | `--test lang` |
| `smoke` / `parity_*` / `build_syx*` / `registry` / … | `--test evergreen` |
| `wasm_*.rs` / `native_*.rs` / `browser_*.rs` | `--test wasm` / `native` / `browser` |

Meta inventory tests (`ec_m3_*`, thin `ec_m4_l/m/p/x`) were removed; use the domain harnesses above.
