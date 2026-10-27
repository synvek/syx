# C-09 `syx` bench entry + deepen + claimed (W935–W937 / ADR-1279–1281)

## Status

**C-09 bench entry claimed met** (ADR-1281 / W937；floor ADR-1280 / W936 + ADR-1279 / W935): Criterion 四切片 + `compile_bench` 入口文档 + CI 引用 + **`syx bench` 薄 allowlist 委派** + inventory 链。HostFn count **629**; C-06/C-07/C-08/C-11 **`[x]`**；C-09 **`[x]`**（窄；≠ llvm-cov / ≠ fuzz / ≠ property / ≠ Criterion ms SLA）；Math **维护**；F-07 **`[x]`**；F-08 **仍冻**。

Fingerprint: `seven-hundred-and-eighty-seventh` / `A-W937`（floor W936：`seven-hundred-and-eighty-sixth` / `A-W936`；W935：`seven-hundred-and-eighty-fifth` / `A-W935`）。

## Checklist (narrow charter · hard)

| # | Item | Evidence |
|---|------|----------|
| 1 | `path_join_criterion` | [`crates/syx-driver/benches/path_join_criterion.rs`](../../crates/syx-driver/benches/path_join_criterion.rs) |
| 2 | `http_response_criterion` | [`crates/syx-driver/benches/http_response_criterion.rs`](../../crates/syx-driver/benches/http_response_criterion.rs) |
| 3 | `sql_exec_criterion` | [`crates/syx-driver/benches/sql_exec_criterion.rs`](../../crates/syx-driver/benches/sql_exec_criterion.rs) |
| 4 | `tcp_connect_criterion` | [`crates/syx-driver/benches/tcp_connect_criterion.rs`](../../crates/syx-driver/benches/tcp_connect_criterion.rs) |
| 5 | `compile_bench` | [`crates/syx-driver/benches/compile_bench.rs`](../../crates/syx-driver/benches/compile_bench.rs) |
| 6 | CI short sample referenced | [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml) + [`nightly.yml`](../../.github/workflows/nightly.yml) |
| 7 | First-slice inventory | `SyxBenchEntryFirstSliceInventory`（W935；historical `c09_ga_claimed=false`） |
| 8 | `syx bench` allowlist CLI | [`crates/syx-cli/src/main.rs`](../../crates/syx-cli/src/main.rs) `Commands::Bench`（W936） |
| 9 | Deepen inventory | `SyxBenchDeepenInventory`（W936；historical `c09_ga_claimed=false`） |
| 10 | Claimed inventory + roadmap `[x]` | `SyxBenchGaClaimedInventory`（W937；**`c09_ga_claimed=true`**）；plan/02 C-09 **`[x]`** |

## Index

| Prior | Status |
|-------|--------|
| Four Criterion benches | **met**（既有） |
| `compile_bench` Instant | **met**（既有） |
| PR CI short sample | **met**（既有；文档化） |
| Nightly `compile_bench` | **met**（既有；文档化） |
| C-09 `[~]` open | **met**（W935；supplanted by `[x]`） |
| `syx bench` thin allowlist | **met**（W936 deepen） |
| C-09 `[x]` claimed | **met**（W937 窄） |
| coverage entry stub | **documented**（W938；gate unmet） |
| fuzz entry stub | **documented**（W941；narrow claimed W943） |
| property entry stub | **documented**（W944；narrow claimed W946；`property_smoke` W947） |
| fuzz harness / property runner / SLA | **fuzz scaffold + CI `-runs=1` met**（W948/W950）；property smoke + CI **met**（W947/W949；proptestest unmet）；SLA unmet |
| Residual maintenance rollup | **met**（W951；详见 [`c09-syx-residual-maintenance-first-slice.md`](c09-syx-residual-maintenance-first-slice.md)） |

## Behavior (hard)

Documented commands:

```bash
# Product CLI (W936 deepen) — allowlist delegate
syx bench --list
syx bench path_join_criterion

# Equivalent cargo (W935 first-slice; still valid)
cargo bench -p syx-driver --bench path_join_criterion
cargo bench -p syx-driver --bench http_response_criterion
cargo bench -p syx-driver --bench sql_exec_criterion
cargo bench -p syx-driver --bench tcp_connect_criterion
cargo bench -p syx-driver --bench compile_bench
```

PR CI uses short Criterion sampling (`--sample-size 10`); nightly runs `compile_bench`. **No** absolute ms SLA fail.

Narrow charter = **document existing bench entry + thin `syx bench` allowlist**；不要求 llvm-cov / fuzz / property / SLA gate。

## Contract (hard)

| Item | Meaning |
|------|---------|
| First-slice inventory | `SyxBenchEntryFirstSliceInventory` / `syx_bench_entry_first_slice_inventory`（historical `c09_ga_claimed=false`） |
| Deepen inventory | `SyxBenchDeepenInventory` / `syx_bench_deepen_inventory`（historical `c09_ga_claimed=false`） |
| Claimed inventory | `SyxBenchGaClaimedInventory` / `syx_bench_ga_claimed_inventory` |
| Flags | `syx_bench_entry_first_slice_signed=true`; `syx_bench_deepen_signed=true`; `syx_bench_ga_claimed_signed=true`; `deepen_floor_ok=true`; **`c09_ga_claimed=true`**（claimed only） |
| Forbidden claim | llvm-cov CI gate / fuzz harness / property runner / Criterion ms SLA / F-08 / HostFn+1 / Math llvm GA |
| Honesty | narrow `[x]` **≠** coverage / fuzz / property / SLA complete |

## Evidence

- [`plan/topics/syx-bench.md`](../../plan/topics/syx-bench.md)
- [`crates/syx-driver/src/lib.rs`](../../crates/syx-driver/src/lib.rs)
- [`crates/syx-cli/src/main.rs`](../../crates/syx-cli/src/main.rs)
- Behavioral floor: `syx_bench_cli_allowlist.rs`（construction first/deepen/claimed meta retired）

## Frozen

- Claiming coverage / fuzz / property / Criterion ms SLA this window
- Adding llvm-cov gate / new CI job / absolute SLA
- Opening F-08 / Math HostFn surface

## Next

W937 C-09 bench entry claimed + `[x]` **已交**（ADR-1281）→ W938 coverage stub first-slice **已交**（ADR-1282）→ W939 coverage deepen **已交**（ADR-1283）→ W940 coverage claimed **已交**（ADR-1284）→ W941 fuzz stub **已交**（ADR-1285）→ W942 fuzz deepen **已交**（ADR-1286）→ W943 fuzz claimed **已交**（ADR-1287）→ W944 property stub **已交**（ADR-1288）→ W945 property deepen **已交**（ADR-1289）→ W946 property claimed **已交**（ADR-1290）→ W947 property_smoke harness **已交**（ADR-1291）→ W948 fuzz scaffold **已交**（ADR-1292）→ W949 CI property **已交**（ADR-1293）→ W950 CI fuzz **已交**（ADR-1294）→ W951 residual maintenance **已交**（ADR-1295）→ W952+ 维护；F-08 仍冻
