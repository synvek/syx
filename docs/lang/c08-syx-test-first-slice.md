# C-08 `syx test` first-slice + deepen + claimed (W932–W934)

## Status

**C-08 `syx test` claimed met** (ADR-1278 / W934；floor ADR-1277 / W933 + ADR-1276 / W932): flat `tests/*.sy` + `--filter` + **`--format json|junit|tap|human`** + inventory 链。HostFn count **629**; C-06/C-07/C-11 **`[x]`**；C-08 **`[x]`**（窄；≠ async / ≠ registry auto-invoke）；Math **维护**；F-07 **`[x]`**；F-08 **仍冻**；≠ C-09。**Follow-on:** recursive `tests/**/*.sy` discovery（相对路径用例名）已落地，不回写 W934 窄章程。

Fingerprint: `seven-hundred-and-eighty-fourth` / `A-W934`（floor W933：`seven-hundred-and-eighty-third` / `A-W933`；W932：`seven-hundred-and-eighty-second` / `A-W932`）。

## Checklist (narrow charter · hard)

| # | Item | Evidence |
|---|------|----------|
| 1 | Flat `tests/*.sy` discovery | [`crates/syx-driver/src/lib.rs`](../../crates/syx-driver/src/lib.rs) `test_project_with_format` |
| 2 | `--filter` substring | CLI + driver |
| 3 | `--format json` report | `TestReport::to_json_string` / CLI（W932） |
| 4 | TAP path retained | `--format tap` / `SYX_TEST_TAP` |
| 5 | First-slice inventory | `SyxTestFirstSliceInventory`（W932；historical `c08_ga_claimed=false`） |
| 6 | `--format junit` XML | `TestReport::to_junit_xml_string` / CLI（W933） |
| 7 | Deepen inventory | `SyxTestDeepenInventory`（W933；historical `c08_ga_claimed=false`） |
| 8 | Claimed inventory + roadmap `[x]` | `SyxTestGaClaimedInventory`（W934；**`c08_ga_claimed=true`**）；plan/02 C-08 **`[x]`** |

## Index

| Prior | Status |
|-------|--------|
| File loop `tests/*.sy` | **met**（既有） |
| `--filter` | **met**（既有） |
| TAP env | **met**（既有 + `--format tap`） |
| `--format json` | **met**（W932） |
| C-08 `[~]` open | **met**（W932；supplanted by `[x]`） |
| `--format junit` | **met**（W933） |
| C-08 deepen inventory | **met**（W933） |
| C-08 `[x]` claimed | **met**（W934 窄） |
| async / registry auto-invoke | **still unmet**（诚实界外） |
| recursive `tests/**` | **follow-on met**（相对路径 case 名；≠ W934 窄章程） |

## Behavior (hard)

`syx test [path] [--filter substr] [--format human|json|tap|junit]`:

1. Discover flat `tests/*.sy`（无目录则跑项目 main）
2. Optional filename substring filter（未匹配记 `skipped`）
3. Formats:
   - `human`（默认）：summary `ok N passed`
   - `json`：stdout JSON `{passed,failed,skipped,cases:[{name,status}]}`；失败 exit ≠ 0
   - `tap`：TAP v13（亦可用 `SYX_TEST_TAP=1` 在 human 默认下启用）
   - `junit`：stdout JUnit XML `<testsuite name="syx">` + `<testcase>`；失败 exit ≠ 0

Narrow charter claimed = **JSON+JUnit+filter+flat discovery + inventory 链**；不要求 async / registry auto-invoke / recursive discovery / C-09.

## Contract (hard)

| Item | Meaning |
|------|---------|
| First-slice inventory | `SyxTestFirstSliceInventory` / `syx_test_first_slice_inventory`（historical `c08_ga_claimed=false`） |
| Deepen inventory | `SyxTestDeepenInventory` / `syx_test_deepen_inventory`（historical `c08_ga_claimed=false`） |
| Claimed inventory | `SyxTestGaClaimedInventory` / `syx_test_ga_claimed_inventory` |
| Flags | `syx_test_first_slice_signed=true`; `syx_test_deepen_signed=true`; `syx_test_ga_claimed_signed=true`; `deepen_floor_ok=true`; **`c08_ga_claimed=true`**（claimed only） |
| Forbidden claim | async test harness / Testing registry auto-invoke / recursive `tests/**` GA / C-09 / F-08 / HostFn+1 |
| Honesty | narrow `[x]` **≠** async / registry / recursive discovery complete |

## Evidence

- [`plan/topics/syx-test.md`](../../plan/topics/syx-test.md)
- [`crates/syx-driver/src/lib.rs`](../../crates/syx-driver/src/lib.rs)
- [`crates/syx-cli/src/main.rs`](../../crates/syx-cli/src/main.rs)
- Behavioral floor: `syx_test_report_formats.rs`（JSON + JUnit；construction first/deepen/claimed meta retired）

## Frozen

- Claiming async / registry auto-invoke / recursive discovery GA this window
- Opening F-08 / Math HostFn surface

## Next

W934 C-08 `syx test` claimed + `[x]` **已交**（ADR-1278）→ W935 C-09 bench entry first-slice **已交**（ADR-1279；C-09 `[~]`）→ W936+；F-08 仍冻
