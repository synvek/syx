# C-8c book↔parser fork inventory census

**Date:** 2026-11-25  
**Scope:** 语言书 ↔ parser/语义 **活分叉清单开账**（≥20 种子行）。**不**改 parser 语法面；**不**宣称分叉已收敛；**不**宣称 C-8 `[x]`；默认下一 **C-8d**。  
**Script:** [`scripts/c8c_book_parser_fork_census.sh`](../../scripts/c8c_book_parser_fork_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Book SSOT ADR:** [`docs/adr/language-book-ssot.md`](../adr/language-book-ssot.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0 / C-1 / C-2 / C-4 / C-6** | **exited** |
| **C-3 / C-5 / C-7** | **`[~]`** |
| **C-8a / C-8b** | **exited** |
| **C-8c** fork inventory | **exited** |
| **C-8** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-8f**（C-8d/C-8e exited）；CST lossless · 真 CallSuper · C-7c+ · ModuleId · C-5c+ · tls.server · macOS · R-50 park |

## Root cause

书为语言面 SSOT（language-book-ssot），但无「书宣称 / toolchain 事实 / 分类」活 ledger。`dual_grammar` / stage2-parser-census 管 AST↔CST，**不是** book↔parser。`book_pass_rate` 只报绿/红，不分类 fork。散落 `Not implemented` / Honesty 旁注无法驱动 C-8d。

## Schema

| Field | Meaning |
|-------|---------|
| `id` | Stable row id (`F01`…) |
| `book_locus` | Chapter + locus (en path; zh mirror implied under `books/zh-cn/…`) |
| `book_claim` | What the guide currently says |
| `toolchain_fact` | Parser/semantic/runtime fact (honest, short) |
| `class` | Closed enum below |
| `evidence` | Book path and/or code path |
| `next` | `park` / `C-8d` / `C-8d+` |

**class（封闭枚举）：**

- `parked_honest` — 书已标 Not implemented / park，与 toolchain 一致
- `book_ahead` — 书偏 GA/「全 Implemented」，toolchain 残或缺
- `legacy_accept` — parser 仍接受非书形（language-book-ssot 兼容）
- `doc_debt` — 书内措辞矛盾/含糊，待 C-8d 改 MD（本窗不修）

## Live inventory

| id | book_locus | book_claim | toolchain_fact | class | evidence | next |
|----|------------|------------|----------------|-------|----------|------|
| F01 | Ch15 omit `super.init` | omit path works; honesty: ≠ CallSuper; true CallSuper Not implemented | U-38 field-default pad only; no compiler-inserted CallSuper | parked_honest | `books/language-guide/15-initialization.md`; C-8a nails | park |
| F02 | Ch15 `convenience init` | Swift-diff table: **Not implemented** | no `convenience` keyword surface as Swift | parked_honest | `15-initialization.md` Swift diff; parser lacks convenience keyword GA | park |
| F03 | Ch15 `required init` | **Not implemented** | required-init surface not GA | parked_honest | `15-initialization.md` | park |
| F04 | Ch15 `init!` | **Not implemented** | no IUO failable init | parked_honest | `15-initialization.md` | park |
| F05 | Ch15 `defer` in init | **Not implemented** | defer-in-init not shipped | parked_honest | `15-initialization.md` | park |
| F06 | Ch14 multiple inheritance | **Not implemented** (use protocols) | single inheritance only | parked_honest | `14-inheritance.md` | park |
| F07 | Ch14 actor inheritance | actor cannot be inherited (**Not implemented**) | actor inherit rejected | parked_honest | `14-inheritance.md` | park |
| F08 | Ch09 `indirect enum` | **Not implemented** | recursive enum needs class wrap | parked_honest | `09-enumerations.md` | park |
| F09 | Ch09 `: String` raw value | **Not implemented** | Int raw only partial | parked_honest | `09-enumerations.md` | park |
| F10 | Ch12 `inout` | **Not implemented**; use `&mut T` | no `inout` keyword | parked_honest | `12-methods.md` | park |
| F11 | Ch12 `@objc` / KVO | **Not implemented** | ObjC bridge absent | parked_honest | `12-methods.md` | park |
| F12 | Ch12 `final func` forbid override | **Not implemented** (use `final class`) | per-method final not GA | parked_honest | `12-methods.md` | park |
| F13 | Ch06 `where case` | **Not implemented** | use `if case` chaining | parked_honest | `06-control-flow.md` | park |
| F14 | Ch06 `if #available` | **Not implemented** | availability queries absent | parked_honest | `06-control-flow.md` | park |
| F15 | Ch06 fallthrough | **Rejected** | intentionally unsupported | parked_honest | `06-control-flow.md` | park |
| F16 | Ch04 `String(format:)` | **Not implemented** | use interpolation / `toString` | parked_honest | `04-strings-and-characters.md` | park |
| F17 | Ch17 `rethrows` | **Not implemented** | rethrows absent | parked_honest | `17-error-handling.md` | park |
| F18 | ADR struct bare fields | Ch10 Compatibility (C-8e): book `var`/`let`; legacy bare still accepted one cycle | parser still accepts legacy `x: Int` | parked_honest | `10-structures-and-classes.md` (C-8e); `docs/adr/language-book-ssot.md`; `crates/syx-parser/src/lib.rs` (~Legacy bare field) | park |
| F19 | ADR unmarked mutating | Ch10 Compatibility (C-8e): book `mutating func`; unmarked legacy / S060 U-30 | unmarked mutating still accepted (compat cycle) | parked_honest | `10-structures-and-classes.md` (C-8e); `docs/adr/language-book-ssot.md` | park |
| F20 | Ch06 status vs labeled break | status + mid-table + Swift-diff: labeled break/continue **Partially**; Honesty (C-8d) ≠ full Implemented | nested OK; cross-fn/closure evolving | parked_honest | `06-control-flow.md` (C-8d) | park |
| F21 | Ch15/Ch14 convenience | Ch15 status + Ch14 Swift-diff: `convenience`/`required` **Not implemented** | no convenience keyword GA | parked_honest | `15-initialization.md`; `14-inheritance.md` (C-8d) | park |
| F22 | Ch10 `mutating func` | feature + Swift-diff: **Partially**; Honesty (C-8d) ≠ GA | Phase 60+ usable; deep-copy edges evolve | parked_honest | `10-structures-and-classes.md` (C-8d) | park |
| F23 | Ch17 typed throws | Partial + Honesty (C-8d): residual ≠ GA; rethrows Not implemented | typed-throws residual; rethrows park | parked_honest | `17-error-handling.md` (C-8d) | park |
| F24 | `02` §3 true CallSuper | park: thick semantics / separate ADR | no CallSuper impl (same as F01 toolchain) | parked_honest | `plan/02-improvement-roadmap.md` §3 | park |

**Row count:** 24 (≥20). **Classes present (post C-8e):** `parked_honest` only as live class（`book_ahead`/`doc_debt` 由 C-8d 清零；`legacy_accept` 由 C-8e 书侧注后 → `parked_honest`；开账时四 class 齐全）。

## Delivery

1. Live inventory table (this file) ≥20 rows + closed class enum  
2. Lang thin nails + census script  
3. SSOT / `02`：C-8c `[x]`；C-8 `[~]`；引入 C-8d；下一 **C-8d**  
4. language-book-ssot References pointer only  
5. 硬 KPI 持有

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| inventory rows ≥20 | **≥20** | **24** |
| all four classes present | **pass** | **1** |
| CallSuper row `parked_honest` | **pass** | **1** (F01) |
| convenience init row present | **pass** | **1** (F02) |
| legacy bare-field `legacy_accept` | **pass** | **1** (F18) |
| lang inventory nails | **pass** | **1** (3/3) |
| `native=real` count | **16** | **16** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c8c_book_parser_fork_census_ok` | **1** | **1** |
| `c8_exited` | **0** | **0** |

## Results (2026-11-25)

```
c8c_inventory_rows=24
c8c_classes_all=1
c8c_callsuper_parked=1
c8c_legacy_bare=1
c8c_adr_pointer=1
c8c_inventory_nails=1
c8c_native_real_count=16
c8c_tls_server_partial=1
c8c_census_doc=1
c8c_ssot_updated=1
c8c_roadmap_ok=1
c8_exited=0
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c8c_book_parser_fork_census_ok=1
current=C-8c
next=C-8d
c8_exited=0
```

## Non-claims

- ≠ C-8 `[x]` · fork 已收敛 · 全量 fence 修绿
- ≠ 改 parser/semantic 语法面 · CST lossless GA
- ≠ dual_grammar / SA-2 surface 冒充本窗
- ≠ 真 CallSuper · C-7c+ · ModuleId · C-5c+ · tls.server · macOS · R-50
- ≠ 重写 `implementation-status.md`

## Soft park

C-8d · CST lossless/扩面 · 真 CallSuper · C-7c+ · ModuleId · C-5c+ · tls.server · macOS · R-50 · 重写 implementation-status · 统一 dual_grammar 与 fork ledger
