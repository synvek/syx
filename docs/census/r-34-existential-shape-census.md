# R-34 Array\<Shape\> protocol dispatch census

**Date:** 2026-11-11  
**Scope:** book ch23-shaped `Array<Shape>` + `s.area()` / `s.describe()` via protocol witnesses; cancel SEM-EXIST silent `0` / false S014.  
**Script:** [`scripts/r34_existential_shape_census.sh`](../../scripts/r34_existential_shape_census.sh)

## Progress

| Track | Status |
|-------|--------|
| Cred / SA / R-0…R-33 | exited (wasm **46/46**; browser **46/0/2**; Object **186/11**) |
| **R-34** | SEM-EXIST cleared (file-path total≈`21.56636`) |
| Object / simd | **186/11** / **159** — unchanged |

## Root cause

1. MIR protocol instance calls emitted **`Call(Shape_area)`** (no body) or fell through to **`ObjectGet("area")`** → Nil / silent `0`.
2. `for-in` did not bind element type from `Array<Shape>` → receiver typed unknown → ObjectGet path.
3. AST `emit_witness_protocol_call` used an extra **Dup** that left `recv` under CallIndirect results → `sum + s.area()` discarded prior sum.
4. Call-site array literals analyzed without param expected type → homogeneous unify → false **S014** for `[Circle, Square]` under `Array<Shape>`.

## Delivery

1. MIR: protocol receiver → `ObjectGet("__witness_{P}_{m}")` + `CallIndirect(1+nargs)` (no Dup).
2. For-in: `infer_for_in_elem_type` binds `s: Shape` from `Array<Shape>` / `Set<T>`.
3. Semantic: call args use `analyze_expr_with_expected(param)` (skip Unknown untyped params); annotated `Array<Protocol>` already OK.
4. AST witness helper: remove Dup (stack-correct with binary uses).
5. Fixture + file-path tests: [`protocol_shape_array.sy`](../../crates/syx-driver/tests/fixtures/lang/protocol_shape_array.sy).

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `protocol_shape_array::*` | green (describe + total*100≈2156) | **pass** |
| `protocol_shape_array_nonconform_s014` | S014 | **pass** |
| browser_failed | **0** | **0** (46/0/2) |
| wasm | **46/46** | **46/46** |
| Object | **≤186 / ≤11** | **186/11** |

## Non-claims

- ≠ `any Shape` / `some` opaque return GA
- ≠ inheritance `class Shape` polymorphism (book ch14)
- ≠ Sequence/map; HostFn+1
- ≠ fixing inline `[Circle(...), Square(...)]` ArrayBuild stack when untyped (use locals / annotated `Array<Shape>`)
- ≠ `Int(Double)` print (still `"Int {}"`; fixture prints `t * 100.0`)
