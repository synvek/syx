# R-33 Iterator for-in witness census

**Date:** 2026-11-11  
**Scope:** custom `class: Iterator` + `for-in` under file-path / prelude trim; cancel SEM-ITER R003 index 928.  
**Script:** [`scripts/r33_iterator_witness_census.sh`](../../scripts/r33_iterator_witness_census.sh)

## Progress

| Track | Status |
|-------|--------|
| Cred / SA / R-0…R-32 | exited (wasm **46/46**; browser **46/0/2**; Object **186/11**) |
| **R-33** | SEM-ITER cleared (file-path Counter sum `6`) |
| Object / simd | **186/11** / **159** — unchanged |

## Root cause

`run_file_printed` calls [`trim_program_to_entry_reachability`](../../crates/syx-codegen/src/program_trim.rs) after compile. Protocol witness slots were lowered as **`ConstI32(fnIdx)`**, so trim:

1. Did **not** treat the index as a call edge → dropped `Counter_next`
2. Did **not** remap the baked index → runtime `functions.get(928)` → **R003**

Embedded tests skipped trim, so S4-04 hinge looked green while book/file-path stayed red.

## Delivery

1. [`Instruction::ConstFnIndex(u32)`](../../crates/syx-codegen/src/lib.rs) — distinct from `ConstI32`; trim keeps + remaps.
2. Codegen / wasm: resolve by name; missing prelude shells (e.g. `Int8_equals`) → `ConstI32(-1)` (MakeIterator ignores); never silent `0`.
3. VM: execute `ConstFnIndex` as platform Int; MakeIterator accepts `I32`/`I64` with `>= 0`; OOB R003 includes `functions.len`.
4. Unit: `trim_keeps_const_fn_index_witness_and_remaps`; lang [`protocol_iterator_forin`](../../crates/syx-driver/tests/fixtures/lang/protocol_iterator_forin.sy).

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `language_deepen::protocol_iterator_forin` | print `6` | **pass** |
| `protocol_iterator::*` | green | **24/24** |
| browser_failed | **0** | **0** (46/0/2) |
| wasm | **46/46** | **46/46** |
| Object | **≤186 / ≤11** | **186/11** |

## Non-claims

- ≠ `Sequence.makeIterator` GA
- ≠ user Sequence map/filter / Collection Index
- ≠ `next() -> String?` / generic `Element?` surface (stable subset **`Int?`**)
- ≠ S4-04 credibility Markdown strings as SEM-ITER evidence (executable fixture only)
- ≠ unblocking hung `m2_s1_01_custom_iterator_for_in_native` (wasm + VM file-path covered)
