# C-09 fuzz entry stub + deepen + claimed + scaffold + CI (W941–W943 / W948 / W950 / ADR-1285–1287 / ADR-1292 / ADR-1294)

## Status

**C-09 fuzz entry stub first-slice met** (ADR-1285 / W941): documented future `cargo fuzz` developer entry + checklist/inventory.

**C-09 fuzz deepen met** (ADR-1286 / W942): thin product CLI `syx fuzz` delegates to `cargo fuzz run syx_smoke`.

**C-09 fuzz narrow entry claimed met** (ADR-1287 / W943): checklist + `SyxFuzzGaClaimedInventory` with **`c09_fuzz_ga_claimed=true`**.

**C-09 fuzz scaffold first-slice met** (ADR-1292 / W948): [`fuzz/`](../../fuzz/) crate + target `syx_smoke`.

**C-09 CI fuzz first-slice met** (ADR-1294 / W950): PR CI bounded `cargo fuzz run syx_smoke -- -runs=1`. HostFn count **629**; C-06～C-11 **`[x]`**（bench/coverage/fuzz/property claimed + scaffold + CI property **维持**）；Math **维护**；F-07 **`[x]`**；F-08 **仍冻**；≠ scaffold GA / ≠ `c09_fuzz_ci_ga` / ≠ 无界 fuzz / ≠ 回翻 C-09 `[~]`.

Fingerprint: `eight-hundredth` / `A-W950`（prior scaffold：`seven-hundred-and-ninety-eighth` / `A-W948`；prior claimed：`seven-hundred-and-ninety-third` / `A-W943`）。

## Checklist (narrow charter · hard)

| # | Item | Evidence |
|---|------|----------|
| 1 | Documented future fuzz entry command | Behavior below |
| 2 | Inventory signs stub | `SyxFuzzStubFirstSliceInventory` |
| 3 | Topics | [`plan/topics/syx-fuzz.md`](../../plan/topics/syx-fuzz.md) |
| 4 | UsageGuide | [`docs/9_UsageGuide.md`](../../docs/9_UsageGuide.md) |
| 5 | Coverage Index cross-link | [`c09-syx-coverage-stub-first-slice.md`](c09-syx-coverage-stub-first-slice.md) fuzz row |
| 6 | Bench Index cross-link | [`c09-syx-bench-entry-first-slice.md`](c09-syx-bench-entry-first-slice.md) fuzz row |
| 7 | Roadmap C-09 `[x]` retained | plan/02 C-09 **`[x]`** |
| 8 | `syx fuzz` thin CLI | [`crates/syx-cli/src/main.rs`](../../crates/syx-cli/src/main.rs) `Commands::Fuzz` |
| 9 | Inventory signs deepen | `SyxFuzzDeepenInventory` |
| 10 | Inventory signs claimed | `SyxFuzzGaClaimedInventory` |
| 11 | `fuzz/` scaffold + `syx_smoke` | [`fuzz/Cargo.toml`](../../fuzz/Cargo.toml) · [`fuzz/fuzz_targets/syx_smoke.rs`](../../fuzz/fuzz_targets/syx_smoke.rs) |
| 12 | Inventory signs scaffold | `SyxFuzzScaffoldFirstSliceInventory` / **`syx_fuzz_scaffold_first_slice_signed=true`** / **`c09_fuzz_scaffold_ga=false`** |
| 13 | PR CI bounded `syx_smoke` | [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml) (`-runs=1`) |
| 14 | Inventory signs CI | `SyxFuzzCiFirstSliceInventory` / `syx_fuzz_ci_first_slice_inventory` / **`syx_fuzz_ci_first_slice_signed=true`** / **`c09_fuzz_ci_ga=false`** |

## Index

| Prior | Status |
|-------|--------|
| Fuzz entry charter documented | **met**（W941 stub） |
| `syx fuzz` thin delegate | **met**（W942 deepen） |
| Fuzz narrow entry claimed | **met**（W943；`c09_fuzz_ga_claimed=true`） |
| cargo-fuzz scaffold / `fuzz/` crate | **met**（W948；`c09_fuzz_scaffold_ga=false`） |
| libFuzzer / CI fuzz job | **met**（W950；`-runs=1`；`c09_fuzz_ci_ga=false`） |
| property entry stub | **documented**（W944；narrow claimed W946；`property_smoke` W947） |
| property runner / harness | **smoke met**（W947）；**CI property met**（W949；proptestest unmet） |
| C-09 bench `[x]` | **met**（W937；本窗不改） |
| Coverage narrow claimed | **met**（W940；本窗不改） |
| Residual maintenance rollup | **met**（W951；详见 [`c09-syx-residual-maintenance-first-slice.md`](c09-syx-residual-maintenance-first-slice.md)） |

## Behavior (hard)

Product CLI 与等价 cargo（scaffold **met**；PR CI **有界钉** W950）：

```bash
syx fuzz

# Equivalent (requires local cargo-fuzz; scaffold present)
cargo fuzz run syx_smoke

# Bounded smoke (PR CI gate; W950)
cargo fuzz run syx_smoke -- -runs=1
```

PR CI（[`.github/workflows/ci.yml`](../../.github/workflows/ci.yml)）：install cargo-fuzz + clang，然后 `cargo fuzz run syx_smoke -- -runs=1`。

Narrow charter = **document + CLI + claimed + scaffold + bounded CI step**；不要求 scaffold GA / 无界 fuzz / `c09_fuzz_ci_ga=true`。

## Contract (hard)

| Item | Meaning |
|------|---------|
| Stub inventory | `SyxFuzzStubFirstSliceInventory` / `syx_fuzz_stub_first_slice_inventory` |
| Flags (stub) | `fuzz_entry_documented=true`; historical **`c09_fuzz_ga_claimed=false`**; `syx_fuzz_stub_first_slice_signed=true` |
| Deepen inventory | `SyxFuzzDeepenInventory` / `syx_fuzz_deepen_inventory` |
| Flags (deepen) | `first_slice_floor_ok=true`; `syx_fuzz_cli_present=true`; historical **`c09_fuzz_ga_claimed=false`**; `syx_fuzz_deepen_signed=true` |
| Claimed inventory | `SyxFuzzGaClaimedInventory` / `syx_fuzz_ga_claimed_inventory` |
| Flags (claimed) | `deepen_floor_ok=true`; **`c09_fuzz_ga_claimed=true`**; `syx_fuzz_ga_claimed_signed=true` |
| Scaffold inventory | `SyxFuzzScaffoldFirstSliceInventory` / `syx_fuzz_scaffold_first_slice_inventory` |
| Flags (scaffold) | `claimed_floor_ok=true`; `fuzz_scaffold_present=true`; **`c09_fuzz_scaffold_ga=false`**; **`syx_fuzz_scaffold_first_slice_signed=true`** |
| CI inventory | `SyxFuzzCiFirstSliceInventory` / `syx_fuzz_ci_first_slice_inventory` |
| Flags (CI) | `scaffold_floor_ok=true`; `fuzz_ci_present=true`; **`c09_fuzz_ci_ga=false`**; **`syx_fuzz_ci_first_slice_signed=true`** |
| Forbidden claim | scaffold GA / `c09_fuzz_ci_ga=true` / unbounded fuzz / F-08 / HostFn+1 / Math llvm GA / 回翻 C-09 bench `[~]` |
| Honesty | CI first-slice **≠** scaffold GA / **≠** unbounded fuzz GA |

## Evidence

- [`plan/topics/syx-fuzz.md`](../../plan/topics/syx-fuzz.md)
- [`crates/syx-driver/src/lib.rs`](../../crates/syx-driver/src/lib.rs)
- [`crates/syx-cli/src/main.rs`](../../crates/syx-cli/src/main.rs)
- [`fuzz/Cargo.toml`](../../fuzz/Cargo.toml) · [`fuzz/fuzz_targets/syx_smoke.rs`](../../fuzz/fuzz_targets/syx_smoke.rs)
- [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml)
- Construction first/deepen/claimed/scaffold/ci meta retired（`fuzz/fuzz_targets/syx_smoke.rs` + CI remain）

## Frozen

- Claiming `c09_fuzz_scaffold_ga=true` / `c09_fuzz_ci_ga=true` / unbounded fuzz farm this window
- Opening F-08 / Math HostFn surface / reverting C-09 bench `[x]` / claiming proptestest GA

## Next

W948 fuzz scaffold **已交**（ADR-1292）→ W949 CI property **已交**（ADR-1293）→ W950 CI fuzz **已交**（ADR-1294）→ W951 residual maintenance **已交**（ADR-1295）→ W952+ 维护；F-08 仍冻
