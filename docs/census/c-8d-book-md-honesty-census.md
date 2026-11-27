# C-8d book MD honesty census

**Date:** 2026-11-25  
**Scope:** 清 C-8c 清单 F20–F23（`doc_debt`/`book_ahead`）en+zh MD 诚实对齐。**不**改 parser；**不**宣称 C-8 `[x]`；默认下一 **C-8e**。  
**Script:** [`scripts/c8d_book_md_honesty_census.sh`](../../scripts/c8d_book_md_honesty_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Inventory:** [`c-8c-book-parser-fork-census.md`](c-8c-book-parser-fork-census.md)（F20–F23 → `parked_honest`）  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0 / C-1 / C-2 / C-4 / C-6** | **exited** |
| **C-3 / C-5 / C-7** | **`[~]`** |
| **C-8a…c** | **exited** |
| **C-8d** MD honesty | **exited** |
| **C-8** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-8e**；CST lossless · 真 CallSuper · C-7c+ · ModuleId · C-5c+ · tls.server · macOS · R-50 park |

## Root cause

Fork ledger F20–F23：同章 status / mid-table / Swift-diff 状态词矛盾，或 Partial 面缺「≠ GA」Honesty。

## Delivery

1. F20 Ch06 labeled break → Partially + Honesty（en+zh）  
2. F21 Ch15/Ch14 convenience/required → Not implemented（en+zh）  
3. F22 Ch10 mutating Swift-diff → Partially + Honesty（en+zh）  
4. F23 Ch17 typed throws Honesty ≠ GA（en+zh）  
5. inventory 行 class 更新；薄钉 + census；硬 KPI 持有

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| F20–F23 en+zh hinges | **pass** | **1** |
| inventory no live doc_debt/book_ahead | **pass** | **1** |
| lang c8d nails | **pass** | **1** (4/4) |
| `native=real` count | **16** | **16** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c8d_book_md_honesty_census_ok` | **1** | **1** |
| `c8_exited` | **0** | **0** |

## Results (2026-11-25)

```
c8d_md_f20_ok=1
c8d_md_f21_ok=1
c8d_md_f22_ok=1
c8d_md_f23_ok=1
c8d_inventory_cleared=1
c8d_honesty_nails=1
c8c_inventory_nails_post=1
c8d_native_real_count=16
c8d_tls_server_partial=1
c8d_census_doc=1
c8d_ssot_updated=1
c8d_roadmap_ok=1
c8_exited=0
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c8d_book_md_honesty_census_ok=1
current=C-8d
next=C-8e
c8_exited=0
```

## Non-claims

- ≠ C-8 `[x]` · CST lossless GA · 改 parser 去 F18/F19
- ≠ 真 CallSuper · C-7c+ · ModuleId · C-5c+ · tls.server · macOS · R-50
- ≠ 分叉已收敛 / 重写 implementation-status

## Soft park

C-8e · CST lossless · 真 CallSuper · C-7c+ · ModuleId · C-5c+ · tls.server · macOS · R-50 · 改 parser 去 legacy
