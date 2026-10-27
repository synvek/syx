# C-09 property entry stub + deepen + claimed + harness + CI (W944–W947 / W949 / ADR-1288–1291 / ADR-1293)

## Status

**C-09 property entry stub first-slice met** (ADR-1288 / W944): documented future property runner developer entry + checklist/inventory.

**C-09 property deepen met** (ADR-1289 / W945): thin product CLI `syx prop` delegates to `cargo test -p syx-driver --test property_smoke`.

**C-09 property narrow entry claimed met** (ADR-1290 / W946): checklist + `SyxPropertyGaClaimedInventory` with **`c09_property_ga_claimed=true`**.

**C-09 property_smoke harness first-slice met** (ADR-1291 / W947): minimal identity smoke at [`tests/property_smoke.rs`](../../crates/syx-driver/tests/property_smoke.rs).

**C-09 CI property first-slice met** (ADR-1293 / W949): PR CI explicit step runs `property_smoke`. HostFn count **629**; C-06～C-11 **`[x]`**（bench/coverage/fuzz claimed + fuzz scaffold + `property_smoke` **维持**）；Math **维护**；F-07 **`[x]`**；F-08 **仍冻**；≠ proptestest GA / ≠ `c09_property_ci_ga` / ≠ CI fuzz / ≠ 回翻 C-09 `[~]`.

Fingerprint: `seven-hundred-and-ninety-ninth` / `A-W949`（prior harness：`seven-hundred-and-ninety-seventh` / `A-W947`；prior claimed：`seven-hundred-and-ninety-sixth` / `A-W946`；prior deepen：`seven-hundred-and-ninety-fifth` / `A-W945`；prior stub：`seven-hundred-and-ninety-fourth` / `A-W944`）。

## Checklist (narrow charter · hard)

| # | Item | Evidence |
|---|------|----------|
| 1 | Documented future property entry command | Behavior below |
| 2 | Inventory signs stub | `SyxPropertyStubFirstSliceInventory` |
| 3 | Topics | [`plan/topics/syx-property.md`](../../plan/topics/syx-property.md) |
| 4 | UsageGuide | [`docs/9_UsageGuide.md`](../../docs/9_UsageGuide.md) |
| 5 | Fuzz Index cross-link | [`c09-syx-fuzz-stub-first-slice.md`](c09-syx-fuzz-stub-first-slice.md) property row |
| 6 | Coverage Index cross-link | [`c09-syx-coverage-stub-first-slice.md`](c09-syx-coverage-stub-first-slice.md) property row |
| 7 | Bench Index cross-link | [`c09-syx-bench-entry-first-slice.md`](c09-syx-bench-entry-first-slice.md) property row |
| 8 | Roadmap C-09 `[x]` retained | plan/02 C-09 **`[x]`** |
| 9 | `syx prop` thin CLI | [`crates/syx-cli/src/main.rs`](../../crates/syx-cli/src/main.rs) `Commands::Prop` |
| 10 | Inventory signs deepen | `SyxPropertyDeepenInventory` |
| 11 | Inventory signs claimed | `SyxPropertyGaClaimedInventory` |
| 12 | `property_smoke` harness | [`crates/syx-driver/tests/property_smoke.rs`](../../crates/syx-driver/tests/property_smoke.rs) |
| 13 | Inventory signs harness | `SyxPropertyHarnessFirstSliceInventory` |
| 14 | PR CI explicit `property_smoke` | [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml) |
| 15 | Inventory signs CI | `SyxPropertyCiFirstSliceInventory` / `syx_property_ci_first_slice_inventory` / **`syx_property_ci_first_slice_signed=true`** / **`c09_property_ci_ga=false`** |

## Index

| Prior | Status |
|-------|--------|
| Property entry charter documented | **met**（W944 stub） |
| `syx prop` thin delegate | **met**（W945 deepen） |
| Property narrow entry claimed | **met**（W946；`c09_property_ga_claimed=true`） |
| Property runner / `property_smoke` harness | **met**（W947 first-slice smoke） |
| CI property job | **met**（W949；`c09_property_ci_ga=false`） |
| proptestest / full property GA | **still unmet** |
| cargo-fuzz scaffold / `fuzz/` crate | **met**（W948；本窗 property 不改） |
| CI fuzz | **met**（W950；`-runs=1`；本窗 property 不改） |
| C-09 bench `[x]` | **met**（W937；本窗不改） |
| Coverage narrow claimed | **met**（W940；本窗不改） |
| Fuzz narrow claimed | **met**（W943；本窗不改） |
| Residual maintenance rollup | **met**（W951；详见 [`c09-syx-residual-maintenance-first-slice.md`](c09-syx-residual-maintenance-first-slice.md)） |

## Behavior (hard)

Product CLI 与等价 cargo（harness **已交**；PR CI **显式钉** W949）：

```bash
syx prop

# Equivalent
cargo test -p syx-driver --test property_smoke
```

PR CI（[`.github/workflows/ci.yml`](../../.github/workflows/ci.yml)）：

```yaml
- name: C-09 property_smoke CI first-slice (W949 / ADR-1293)
  run: cargo test -p syx-driver --test property_smoke -- --nocapture
```

Narrow charter = **document + CLI + claimed + smoke + explicit CI step**；不要求 proptestest GA / CI fuzz / `c09_property_ci_ga=true`。

## Contract (hard)

| Item | Meaning |
|------|---------|
| Stub inventory | `SyxPropertyStubFirstSliceInventory` / `syx_property_stub_first_slice_inventory` |
| Flags (stub) | `property_entry_documented=true`; historical **`c09_property_ga_claimed=false`**; `syx_property_stub_first_slice_signed=true` |
| Deepen inventory | `SyxPropertyDeepenInventory` / `syx_property_deepen_inventory` |
| Flags (deepen) | `first_slice_floor_ok=true`; `syx_prop_cli_present=true`; historical **`c09_property_ga_claimed=false`**; `syx_property_deepen_signed=true` |
| Claimed inventory | `SyxPropertyGaClaimedInventory` / `syx_property_ga_claimed_inventory` |
| Flags (claimed) | `deepen_floor_ok=true`; **`c09_property_ga_claimed=true`**; `syx_property_ga_claimed_signed=true` |
| Harness inventory | `SyxPropertyHarnessFirstSliceInventory` / `syx_property_harness_first_slice_inventory` |
| Flags (harness) | `claimed_floor_ok=true`; `property_smoke_present=true`; **`c09_property_harness_ga=false`**; `syx_property_harness_first_slice_signed=true` |
| CI inventory | `SyxPropertyCiFirstSliceInventory` / `syx_property_ci_first_slice_inventory` |
| Flags (CI) | `harness_floor_ok=true`; `property_ci_present=true`; **`c09_property_ci_ga=false`**; **`syx_property_ci_first_slice_signed=true`** |
| Forbidden claim | proptestest GA / CI fuzz / `c09_property_ci_ga=true` / F-08 / HostFn+1 / Math llvm GA / 回翻 C-09 bench `[~]` |
| Honesty | CI first-slice **≠** proptestest GA / **≠** full property CI GA |

## Evidence

- [`plan/topics/syx-property.md`](../../plan/topics/syx-property.md)
- [`crates/syx-driver/tests/property_smoke.rs`](../../crates/syx-driver/tests/property_smoke.rs)
- [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml)
- [`crates/syx-driver/src/lib.rs`](../../crates/syx-driver/src/lib.rs)
- [`crates/syx-cli/src/main.rs`](../../crates/syx-cli/src/main.rs)
- Construction first/deepen/claimed/harness/ci meta retired（`tests/property_smoke.rs` + CI remain）

## Frozen

- Claiming `c09_property_ci_ga=true` / adding proptestest / full property GA this window
- Adding CI fuzz / opening F-08 / Math HostFn surface / reverting C-09 bench `[x]`

## Next

W947 property_smoke harness **已交**（ADR-1291）→ W948 fuzz scaffold **已交**（ADR-1292）→ W949 CI property **已交**（ADR-1293）→ W950 CI fuzz **已交**（ADR-1294）→ W951 residual maintenance **已交**（ADR-1295）→ W952+ 维护；F-08 仍冻
