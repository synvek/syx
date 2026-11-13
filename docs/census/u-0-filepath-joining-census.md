# U-0 FilePath joining 返回类型 census

**Date:** 2026-11-13  
**Scope:** R-47 非声明残余 — `FilePath.joining` / `normalized` 链式与再赋值保持 `FilePath`（≠ 产品 GA 全路径 API）。  
**Script:** [`scripts/u0_filepath_joining_census.sh`](../../scripts/u0_filepath_joining_census.sh)

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 | **收口**（无 R-50） |
| **U-0** | **exited** — joining chain + reassign 绿 |
| Open | park：JIT Suite-1、`func…async`、simd 159；候选 U-1 |

## Root cause

1. MIR `ModuleIndex` 的 `Decl::Struct` 只登记字段，未把实例方法写入 `extension_method_returns` / `Type_method` → `function_returns`；链式后 recv 丢类型，方法降为 `ObjectGet`。
2. 导入的 `FilePath` Struct 在语义分析**之后**才 inject → `class_methods` 无 `joining` → 再赋值 S004 `expected FilePath, found _`。
3. recv 一旦类型正确，零参字段调用 `rawValue()` 误降为 `Call FilePath_rawValue`（C020）。

## Delivery

1. [`mir_module_index.rs`](../../crates/syx-mir/src/mir_module_index.rs)：Struct 实例方法登记返回类型。
2. [`prelude_glue.rs`](../../crates/syx-driver/src/prelude_glue.rs) `inject_imported_public_structs` + compile/mir_unify：**仅** public Struct 语义前 inject（全量 Class inject 会踩 SIMD `S042`）。
3. [`semantic_analyzer.rs`](../../crates/syx-semantic/src/semantic_analyzer.rs)：`{Type}_{method}` function_return_types 兜底。
4. [`mir_lower_expr_call_member.rs`](../../crates/syx-mir/src/mir_lower_expr_call_member.rs)：零参 stored field → `ObjectGet`。
5. 薄钉 [`filepath_joining_chain.rs`](../../crates/syx-driver/tests/lang/filepath_joining_chain.rs)。

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| chain `joining`→`normalized` + `rawValue` | 非 nil | **pass** |
| `p = p.joining(...)` | 无 S004 | **pass** |
| lang nail | pass | **pass** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** (46 passed) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |

## Non-claims

- ≠ 新开 R-50
- ≠ JIT Suite-1 unpark / `func…async` 产品化
- ≠ SyxHost 全树隐藏 / `Http`→`HTTPClient`
- ≠ D3 census 合并 / simd wipe / 全量 credibility 删除
- ≠ 改写 [`packages/core/path.sy`](../../packages/core/path.sy) API 表面（本窗根因在驱动/语义/MIR）

## References

- [`r-47-package-api-census`](r-47-package-api-census.md)（非声明 residual → U-0 消化）
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-0 → 下一 U / park
