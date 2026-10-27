# C-09 coverage entry stub + deepen + claimed (W938–W940 / ADR-1282–1284)

## Status

**C-09 coverage entry stub first-slice met** (ADR-1282 / W938): documented local `cargo llvm-cov` developer entry + checklist/inventory.

**C-09 coverage deepen met** (ADR-1283 / W939): thin product CLI `syx cov` delegates to `cargo llvm-cov -p syx-driver --test native --no-report`.

**C-09 coverage narrow entry claimed met** (ADR-1284 / W940): checklist + `SyxCoverageGaClaimedInventory` with **`c09_coverage_ga_claimed=true`**. HostFn count **629**; C-06～C-11 **`[x]`**（C-09 bench 窄 `[x]` **维持**）；Math **维护**；F-07 **`[x]`**；F-08 **仍冻**；≠ llvm-cov % CI gate / ≠ fuzz / ≠ property / ≠ Criterion ms SLA / ≠ 回翻 C-09 `[~]`.

Fingerprint: `seven-hundred-and-ninetieth` / `A-W940`（prior deepen：`seven-hundred-and-eighty-ninth` / `A-W939`；prior stub：`seven-hundred-and-eighty-eighth` / `A-W938`）。

## Checklist (narrow charter · hard)

| # | Item | Evidence |
|---|------|----------|
| 1 | Documented llvm-cov entry command | Behavior below |
| 2 | Inventory signs stub | `SyxCoverageStubFirstSliceInventory` |
| 3 | Topics | [`plan/topics/syx-coverage.md`](../../plan/topics/syx-coverage.md) |
| 4 | UsageGuide | [`docs/9_UsageGuide.md`](../../docs/9_UsageGuide.md) |
| 5 | Bench Index cross-link | [`c09-syx-bench-entry-first-slice.md`](c09-syx-bench-entry-first-slice.md) coverage row |
| 6 | Roadmap C-09 bench `[x]` retained | plan/02 C-09 **`[x]`** |
| 7 | `syx cov` thin CLI | [`crates/syx-cli/src/main.rs`](../../crates/syx-cli/src/main.rs) `Commands::Cov` |
| 8 | Inventory signs deepen | `SyxCoverageDeepenInventory` |
| 9 | Inventory signs claimed | `SyxCoverageGaClaimedInventory` |

## Index

| Prior | Status |
|-------|--------|
| Coverage entry documented | **met**（W938 stub） |
| `syx cov` thin delegate | **met**（W939 deepen） |
| Coverage narrow entry claimed | **met**（W940；`c09_coverage_ga_claimed=true`） |
| llvm-cov CI % gate | **still unmet** |
| PR fail-on-coverage | **still unmet** |
| fuzz entry stub | **documented**（W941；narrow claimed W943） |
| property entry stub | **documented**（W944；narrow claimed W946；`property_smoke` W947） |
| fuzz harness / property runner | **fuzz scaffold + CI `-runs=1` met**（W948/W950）；property smoke + CI **met**（W947/W949；proptestest unmet） |
| C-09 bench `[x]` | **met**（W937；本窗不改） |
| Residual maintenance rollup | **met**（W951；详见 [`c09-syx-residual-maintenance-first-slice.md`](c09-syx-residual-maintenance-first-slice.md)） |

## Behavior (hard)

Product CLI（W939 deepen）与等价 cargo（W938 stub）：

```bash
syx cov

# Equivalent (requires local cargo-llvm-cov)
cargo llvm-cov -p syx-driver --test native --no-report
```

`--no-report` keeps the stub cheap for smoke; full HTML/LCOV reports are out of charter. **No** absolute coverage % fail. Missing `cargo-llvm-cov` → non-zero + install hint.

Narrow charter = **document coverage entry + thin `syx cov` delegate + claimed inventory**；不要求 CI job / % gate.

## Contract (hard)

| Item | Meaning |
|------|---------|
| Stub inventory | `SyxCoverageStubFirstSliceInventory` / `syx_coverage_stub_first_slice_inventory` |
| Flags (stub) | `coverage_entry_documented=true`; historical **`c09_coverage_ga_claimed=false`**; `syx_coverage_stub_first_slice_signed=true` |
| Deepen inventory | `SyxCoverageDeepenInventory` / `syx_coverage_deepen_inventory` |
| Flags (deepen) | `first_slice_floor_ok=true`; `syx_cov_cli_present=true`; historical **`c09_coverage_ga_claimed=false`**; `syx_coverage_deepen_signed=true` |
| Claimed inventory | `SyxCoverageGaClaimedInventory` / `syx_coverage_ga_claimed_inventory` |
| Flags (claimed) | `deepen_floor_ok=true`; **`c09_coverage_ga_claimed=true`**; `syx_coverage_ga_claimed_signed=true` |
| Forbidden claim | llvm-cov % CI gate / PR fail-on-coverage / fuzz / property / F-08 / HostFn+1 / Math llvm GA / 回翻 C-09 bench `[~]` |
| Honesty | 窄 claimed **≠** coverage % GA / **≠** rewrite C-09 bench `[x]` boundary |

## Evidence

- [`plan/topics/syx-coverage.md`](../../plan/topics/syx-coverage.md)
- [`crates/syx-driver/src/lib.rs`](../../crates/syx-driver/src/lib.rs)
- [`crates/syx-cli/src/main.rs`](../../crates/syx-cli/src/main.rs)
- Construction first/deepen/claimed meta retired（CLI `syx cov` + inventories remain in product code）

## Frozen

- Claiming coverage % GA / CI fail-on-coverage / fuzz / property this window
- Adding llvm-cov PR gate / absolute % SLA
- Opening F-08 / Math HostFn surface / reverting C-09 bench `[x]`

## Next

W940 C-09 coverage narrow claimed **已交**（ADR-1284）→ W941 fuzz stub first-slice **已交**（ADR-1285）→ W942 fuzz deepen **已交**（ADR-1286）→ W943 fuzz claimed **已交**（ADR-1287）→ W944 property stub **已交**（ADR-1288）→ W945 property deepen **已交**（ADR-1289）→ W946 property claimed **已交**（ADR-1290）→ W947 property_smoke harness **已交**（ADR-1291）→ W948 fuzz scaffold **已交**（ADR-1292）→ W949 CI property **已交**（ADR-1293）→ W950 CI fuzz **已交**（ADR-1294）→ W951 residual maintenance **已交**（ADR-1295）→ W952+ 维护；F-08 仍冻
