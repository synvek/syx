# ADR: R-42 simd Object honesty charter

**Status:** Accepted (R-42) · charter only · residual **≥159** retained  
**Date:** 2026-11-12  
**Context:** After R-41 OrderedSet (−6 → inventory **180/10**), the dominant public Object surface is [`packages/core/simd.sy`](../../packages/core/simd.sy) (**159** tokens across `SIMD2`/`SIMD4`/`SIMD8`/`SIMD16`). Runtime already has `TypeId::Simd` / `Value::Simd` / MIR simd ops / `SyxHost.simd_*`, but the public façade and many host decls still speak `Object` (host often lies with `Float64`/`String` placeholders). R-27/R-29 parked simd with **禁 wipe**. This window locks the knife policy — it does **not** rewrite signatures.

## Decision

### 1. 禁止 wipe（hard）

- **禁止** 无契约地把公开 `Object` 改成 `Sendable`、删签名、或改 census 期望来「降」`simd_object_residual`。
- R-42 exit **requires** `simd_object_residual ≥ 159` (measured on `simd.sy`).

### 2. 允许动刀（全部满足才可开后续切片窗）

同时满足：

1. 公开参数/返回可表达为 `TypeId::Simd { lanes, element }`（或具体 lane 标量），**不是**裸 `Object`/`Sendable` 伪装。
2. 对应 `SyxHost.simd_*` / TYPE_METHODS / MIR 路径以真实向量值传递，**不**依赖 Object bag 或 `Float64` 占位撒谎。
3. 有失败测试驱动（fail-closed）；禁止 `#[ignore]` 伪装绿。
4. 每切片单独开窗，并显式下调 Object 天花板（写 census）。

### 3. 建议后续切片序（≠ 本窗施工）

1. `SIMD2` F64 二元族（`addF64`/`subF64`/`mulF64`/`divF64`）+ `fromArray` 对齐。  
2. `SIMD2` load/store / `reduceAdd`。  
3. 更宽 lane（`SIMD4` → `SIMD8` → `SIMD16`）按族推进。  

### 4. Scoped `[x]` meaning

R-42 `[x]` = **charter + cluster census landed** — **not** 「simd Object 清零」 / **not** 「inventory −159」。

## Non-claims

- ≠ simd 产品泛型 GA  
- ≠ 本窗改 [`simd.sy`](../../packages/core/simd.sy) Object 签名  
- ≠ json Object bags / OrderedSet`<T>` / LANG-POLY / JIT / 冷启动 E1  

## Consequences

- Census: [`r-42-simd-object-census`](../census/r-42-simd-object-census.md).  
- Default next = **R-43** `polymorphic_base_ref`（ledger）.  
- Future simd façade work must cite this ADR’s §允许动刀.

## References

- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) R-42  
- [`r-27-collections-object-census`](../census/r-27-collections-object-census.md) · simd parked  
- [`r-41-orderedset-census`](../census/r-41-orderedset-census.md) · explicit slice left simd to R-42  
