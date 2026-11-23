# C-1c immutable 写拒收 census

**Date:** 2026-11-23  
**Scope:** `mut_ref_cell_set_field` 对 `mutable=false` ObjectRef **不 insert**，返回原 handle。≠ let/var→mutable 产品接线 · ≠ Nil 拒写 · ≠ C-1d churn 叙事。  
**Script:** [`scripts/c1c_immutable_write_census.sh`](../../scripts/c1c_immutable_write_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-1a/b** | **exited** |
| **C-1c** immutable 写拒收 | **exited** |
| Open | 默认下一 **C-1d**；C-0a+/e park |

## Root cause

`mut_ref_cell_set_field` 用 `mutable: _` 忽略 flag，map 总 insert，返回强制 `mutable: true`。C-1b `weak_load` 固定产出 `mutable: false`，与「永远可写」撞洞。

## Delivery

1. `!mutable` → 跳过 insert；返回原 `ObjectRef`（保留 false）  
2. `mutable` 成功路径保留写；不再靠「强制 true」掩盖拒收  
3. 永久钉：`c1c_immutable_object_ref_set_field_refused` · `c1c_weak_load_result_set_field_refused` · mutable 仍可写  
4. VM/mir-run ObjectSet / Host 同 API，一处盖全

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `mutable=false` set | 字段不变 · 仍 false | **pass** (`c1c_refuse_gate=1`) |
| `weak_load` 后再 set | 字段不变 | **pass** (`c1c_nails=1`) |
| `mutable=true` / MutRefCell | 仍可写 | **pass** (phase125 + c1c mutable nail) |
| runtime-core c1c_ nails | green | **3/3** (+ weak/phase125 回归) |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c1c_immutable_write_census_ok` | **1** | **1** · next=C-1d |

## Non-claims

- ≠ let/var → `ObjectRef.mutable` 产品接线 / 扩 S060
- ≠ 以 Nil / Error Value / ObjectSet `Err` 表示拒写
- ≠ 字段 weak 自动置 nil / suite_memory 全绿
- ≠ C-1d Host/文档 churn · C-2 · R-50

## References

- [`c-1b-weak-identity-census`](c-1b-weak-identity-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-1c → C-1d
