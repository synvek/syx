# C-5c-1 embed/macro timeout hard wire census

**Date:** 2026-11-25  
**Scope:** opt-in hard timeout — embed env∪CLI、wasmtime kill、`SYX_MACRO_TIMEOUT_MS`→M040。**不**宣称 C-5 `[x]`；默认下一 **C-5c-2**。  
**Script:** [`scripts/c5c1_embed_macro_timeout_census.sh`](../../scripts/c5c1_embed_macro_timeout_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-5a / C-5b** | **exited** |
| **C-5c-1** timeout | **exited** |
| **C-5** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-5c-2**；RSS 硬杀 · permission-sandbox default-on · tls.server · macOS · R-50 park |

## Root cause

wasmtime 路径丢弃 `timeout_secs`；CLI `--embed-timeout-ms` 写入 config 但 embed 只读 env；macro 子进程无墙钟；公开面仍写 soft timeout。

## Delivery

1. `embed_timeout_ms()`：env 优先 ∪ `runtime_config().embed.timeout_ms`  
2. wasmtime：`wait_command_output_deadline` kill → None  
3. macro：`SYX_MACRO_TIMEOUT_MS` → kill + M040  
4. evergreen `c5c1_*` + macro unit nails  
5. ADR / env-registry / UsageGuide 诚实  
6. SSOT / `02`：C-5c-1 `[x]`；下一 **C-5c-2**；C-5 `[~]`  
7. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| timeout API + hooks | **1** | **1** (`c5c1_api` · `c5c1_hooks`) |
| evergreen / macro nails | **pass** | **1** (`c5c1_evergreen` · `c5c1_macro_unit`) |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c5c1_embed_macro_timeout_census_ok` | **1** | **1** |

## Results (2026-11-25)

```
c5c1_api=1
c5c1_hooks=1
c5c1_neg_pin=1
c5c1_native_real_count=16
c5c1_tls_server_partial=1
c5c1_census_doc=1
c5c1_ssot_updated=1
c5c1_roadmap_ok=1
c5c1_evergreen=1
c5c1_macro_unit=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c5c1_embed_macro_timeout_census_ok=1
current=C-5c-1-embed-macro-timeout
next=C-5c-2
```

## Non-claims

- ≠ C-5 `[x]` · seccomp BPF · Job Object · RSS 硬杀  
- ≠ permission-sandbox default-on · in-process orphan kill  
- ≠ tls.server · macOS · R-50 · 改硬 KPI

## Soft park

**C-5c-2** seccomp · RSS/memory 硬杀 · permission-sandbox default-on · 全量 MirOp · CST lossless · CallSuper · C-7c+ · tls.server · macOS · R-50
