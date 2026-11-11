# R-36 diagnostics quality census

**Date:** 2026-11-11  
**Scope:** todo-005 B8 / §4.7 — ban Unknown leak / MIR001·R003·C020 Display spans / Error print `E.bad`.  
**Script:** [`scripts/r36_diagnostics_census.sh`](../../scripts/r36_diagnostics_census.sh)

## Progress

| Track | Status |
|-------|--------|
| Cred / SA / R-0…R-35 | exited (wasm **46/46**; browser **46/0/2**; Object **186/11**) |
| **R-36** | diagnostics quality cleared (file-path fixtures) |
| Object / simd | **186/11** / unchanged |

## Root cause

1. **Unknown leak:** `type_id_to_name(Unknown)` → `"Unknown"` in S004/S014; `type_name_to_id("Unknown")` fell through to Named.
2. **Span-less MIR001/C020/R003:** MIR/codegen errors omitted SourceFile; C020 used `Span::DUMMY`; R003 host path (`StringCharAt`) ignored `instruction_spans` / `debug_source` (serde-skipped on cache).
3. **Error bag print:** enum shells `{__type, case, tag}` rendered via Object/ObjectRef field dump as `{case: bad, tag: 0}`.

## Delivery

1. Semantic: `type_id_to_diag_name` (`Unknown`→`_`); dual-Unknown S004 suppressed; `type_name_to_id("Unknown")` → sentinel.
2. MIR/codegen/driver: LowerCtx SourceFile; C020 uses instr span + `debug_source`; rehydrate `debug_source` after cache.
3. VM: `current_instr_span` + `enrich_runtime_diag` for host R003.
4. runtime-core: enum case Display `E.bad` / `NetworkError.failed(...)`.
5. Fixtures + file-path tests: [`diagnostics_quality.rs`](../../crates/syx-driver/tests/lang/diagnostics_quality.rs).

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `diagnostics_quality::*` (5) | green | **pass** |
| browser_failed | **0** | **0** (46/0/2) |
| wasm | **46/46** | **46/46** |
| Object | **≤186 / ≤11** | **186/11** |

## Non-claims

- ≠ R-37 CI
- ≠ `catch E.bad` pattern syntax
- ≠ full-repo zero `Span::DUMMY` / every R003
- ≠ `Array.first/last`, `for (k,v)` (still on attached table)
