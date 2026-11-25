# C-8e legacy compat notes census

**Date:** 2026-11-25  
**Scope:** F18/F19 `legacy_accept` → Ch10 en+zh **Compatibility (C-8e)** 书侧注；inventory → `parked_honest`。**不**改 parser；**不**宣称 C-8 `[x]`；默认下一 **C-8f**。  
**Script:** [`scripts/c8e_legacy_compat_notes_census.sh`](../../scripts/c8e_legacy_compat_notes_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Inventory:** [`c-8c-book-parser-fork-census.md`](c-8c-book-parser-fork-census.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0 / C-1 / C-2 / C-4 / C-6** | **exited** |
| **C-3 / C-5 / C-7** | **`[~]`** |
| **C-8a…d** | **exited** |
| **C-8e** legacy notes | **exited** |
| **C-8** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-8f**；CST lossless 实现 · 真 CallSuper · 去 parser legacy · C-7c+ · ModuleId · C-5c+ · tls.server · macOS · R-50 park |

## Root cause

ADR Compatibility 表已锁 bare fields / unmarked mutating one-cycle；Ch10 只教书形，未注 legacy 仍接受 — inventory F18/F19 为 `legacy_accept`。

## Delivery

1. Ch10 en+zh Compatibility (C-8e) ×2（F18 bare · F19 mutating）  
2. inventory F18/F19 → `parked_honest`；无活 `legacy_accept`  
3. 薄钉 + census；硬 KPI 持有  
4. SSOT / `02`：C-8e `[x]`；C-8 `[~]`；下一 **C-8f**

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| Ch10 en+zh Compatibility ×2 | **pass** | **1** |
| inventory no live legacy_accept | **pass** | **1** |
| lang c8e nails | **pass** | **1** (3/3) |
| `native=real` count | **16** | **16** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c8e_legacy_compat_notes_census_ok` | **1** | **1** |
| `c8_exited` | **0** | **0** |

## Results (2026-11-25)

```
c8e_ch10_en_ok=1
c8e_ch10_zh_ok=1
c8e_inventory_cleared=1
c8e_adr_pointer=1
c8e_compat_nails=1
c8c_inventory_nails_post=1
c8e_native_real_count=16
c8e_tls_server_partial=1
c8e_census_doc=1
c8e_ssot_updated=1
c8e_roadmap_ok=1
c8_exited=0
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c8e_legacy_compat_notes_census_ok=1
current=C-8e
next=C-8f
c8_exited=0
```

## Non-claims

- ≠ C-8 `[x]` · CST lossless GA · 去 parser legacy accept
- ≠ 实现 deprecation warn · 真 CallSuper · C-7c+ · ModuleId · C-5c+ · tls.server · macOS · R-50

## Soft park

C-8f · CST lossless 实现 · 真 CallSuper · 改 parser 去 F18/F19 · C-7c+ · ModuleId · C-5c+ · tls.server · macOS · R-50
