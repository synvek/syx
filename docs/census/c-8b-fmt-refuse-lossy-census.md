# C-8b formatter refuse-lossy census

**Date:** 2026-11-25  
**Scope:** format 默认拒 CST001→AST 丢注释写盘（`F003`）；显式 `--allow-lossy` 才放行；LSP 硬拒 lossy。**不**宣称 C-8 `[x]`；默认下一 **C-8c**。  
**Script:** [`scripts/c8b_fmt_refuse_lossy_census.sh`](../../scripts/c8b_fmt_refuse_lossy_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0 / C-1 / C-2 / C-4 / C-6** | **exited** |
| **C-3 / C-5 / C-7** | **`[~]`** |
| **C-8a** Ch15 MD | **exited** |
| **C-8b** refuse-lossy | **exited** |
| **C-8** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-8c**；真 CallSuper · CST 扩面 · C-7c+ · ModuleId · C-5c+ · tls.server · macOS · R-50 park |

## Root cause

`format_source_auto` / 默认 `format_dry_run` 在 `CST001` 时静默 AST fallback；write/`--check`/LSP 共用该路径 → 含注释的 CST 外语法可成功丢注释落盘。

## Delivery

1. `FormatPolicy { allow_lossy }`；默认 CST001 → `F003`；`allow_lossy` → `format_source_auto`
2. CLI `--allow-lossy`；LSP `allow_lossy=false`
3. 契约钉 + UsageGuide en/zh
4. SSOT / `02`：C-8b `[x]`；C-8 `[~]`；下一 **C-8c**
5. census + 硬 KPI 持有

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| default path CST001 not silent Ok | **pass** | **1** (`c8b_src_policy_ok` / default refuse write) |
| CLI `--allow-lossy` | **pass** | **1** |
| LSP no allow_lossy=true | **pass** | **1** |
| refuse + allow_lossy behavioral nails | **pass** | **1** (credibility 2/2 + fmt nail) |
| `native=real` count | **16** | **16** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c8b_fmt_refuse_lossy_census_ok` | **1** | **1** |
| `c8_exited` | **0** | **0** |

## Results (2026-11-25)

```
c8b_src_policy_ok=1
c8b_cli_allow_lossy=1
c8b_lsp_hard_refuse=1
c8b_usageguide_ok=1
c8b_refuse_lossy_nails=1
c8b_fmt_dry_run_nail=1
c8b_default_refuse_write=1
c8b_default_msg_or_unchanged=1
c8b_allow_lossy_write=1
c8b_native_real_count=16
c8b_tls_server_partial=1
c8b_census_doc=1
c8b_ssot_updated=1
c8b_roadmap_ok=1
c8_exited=0
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c8b_fmt_refuse_lossy_census_ok=1
current=C-8b
next=C-8c
c8_exited=0
```

## Non-claims

- ≠ C-8 `[x]` · CST 子集扩面 / trivia GA · format lossless GA
- ≠ LSP 偷偷开 lossy · book/parser 分叉清单（C-8c）
- ≠ 真 CallSuper · C-7c+ · ModuleId · C-5c+ · tls.server · macOS · R-50

## Soft park

C-8c · 真 CallSuper · CST 扩面 · C-7c+ · ModuleId · C-5c+ · tls.server · macOS · R-50
