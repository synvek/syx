# C-0c free-function call arity census

**Date:** 2026-11-18  
**Scope:** 非泛型自由函数调用 arity → **S061**；计入 `Param.default`；标签仍 **S042**；类型错配仍 **S004**。≠ 方法/泛型/host · 按标签对齐 · C-0d `&mut` · C-0a Unknown。  
**Script:** [`scripts/c0c_call_arity_census.sh`](../../scripts/c0c_call_arity_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0b** | **exited** |
| **C-0c** | **exited** — free-function arity S061 |
| Open | 默认下一 **C-0d**；序 b→c→d→a；C-0e park |

## Root cause

Ident 调用只遍历已有 args；`function_params` 不存 default；少参/多参 check 绿、运行时错。

## Delivery

1. [`semantic_analyzer.rs`](../../crates/syx-semantic/src/semantic_analyzer.rs)：`function_param_has_default`
2. [`semantic_collect_decls.rs`](../../crates/syx-semantic/src/semantic_collect_decls.rs)：非泛型自由函数写入 default 旗
3. [`semantic_analyze_call.rs`](../../crates/syx-semantic/src/semantic_analyze_call.rs)：`required`/`max` → S061（标签检查之后）
4. appendix-B en/zh · 钉 [`type_soundness.rs`](../../crates/syx-driver/tests/lang/type_soundness.rs)
5. [`type_checker.rs`](../../crates/syx-semantic/src/type_checker.rs)：C-0b 连带 — bare Array/Dictionary ↔ typed；found-`Object` 作 host 噪声（待 C-0a）

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `sum2(1)` / `sum2(1,2,3)` | **S061** | **S061** |
| `sum2(1,2)` / default omit | check **0** | **0** |
| wrong type / missing label | **S004** / **S042** | **S004** / **S042** |
| lang nail | type_soundness | **12/12** |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c0c_call_arity_census_ok` | **1** | **1** |

## Non-claims

- ≠ 方法 / init / 泛型 / host / `print` arity
- ≠ 按标签名对齐、跳过中间 default、Same-label 空调用改 S042
- ≠ C-0d `&mut` / C-0a Unknown / C-0e narrowing
- ≠ MIR verifier / JIT / SyxHost / R-50

## References

- [`c-0b-return-type-census`](c-0b-return-type-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-0c → C-0d
