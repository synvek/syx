# ADR: Stage-2 S2-05 open — file split (S2-I165)

**Status:** Accepted (S2-I165) · S2-05 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I164 S2-04 claim](stage2-s2-04-claim.md), Stage-2 default work shifts to [plan/02 S2-05](../../plan/02-improvement-roadmap.md): split giant files in [`syx-mir`](../../crates/syx-mir), [`syx-driver`](../../crates/syx-driver), and [`syx-semantic`](../../crates/syx-semantic). Today those crates still concentrate most logic in multi-thousand-line `lib.rs` faces (combined **31222** LOC at open). This window **opens** the workstream with a file-split **census baseline** — it does **not** claim S2-05 `[x]`, migrate Frontend onto CST, or exit Stage-2.

## Gap: monolith vs plan S2-05

| Claim | Today (honest) | plan/02 S2-05 target |
|-------|----------------|----------------------|
| Maintainable module layout | Three giant `lib.rs` (+ uneven sidecars) | Modules cut by ownership / layer |
| `syx-mir` | ~13k in `lib.rs`; only `mir_text` sidecar | Extract coherent MIR faces |
| `syx-driver` | ~9k `lib.rs` + many sidecars (incl. large `farm_lease_auth`) | Shrink residual `lib.rs` orchestration |
| `syx-semantic` | ~9k `lib.rs`; thin helper modules | Extract checker / lower clusters |

Baseline numbers: [stage2-file-split-census](../stage2-file-split-census.md).

## Decision

1. **Open S2-05** as an active Stage-2 workstream; flip plan S2-05 `[ ]` → `[~]`.
2. **S2-I165 first slice** — publish file-split census (+ script); add a thin contract under `syx-driver/tests/` (no product split this window).
3. **Ladder:** **S2-I166** = first extract knife from **`syx-mir`** (largest face; one boundary; zero semantics) → further knives → dedicated **S2-05 `[x]` claim**.
4. **Non-claims (hard):** ≠ S2-05 `[x]` / ≠ Stage-2 exit / ≠ Frontend CST migrate / ≠ delete `syx-cst` / ≠ dual-grammar retire / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-06…07 this window / ≠ mechanical mega-split of all three crates this window.

## Evidence (this window)

| Check | Evidence |
|-------|----------|
| File-split census | [`docs/stage2-file-split-census.md`](../stage2-file-split-census.md) · `scripts/s2_05_file_split_census.sh` |
| Contract nail | `s2_05_file_split_census_contract` |
| Plan status | S2-05 `[~]` under this ADR |

## Consequences

- S2-05 moved `[ ]` → `[~]` at open; **S2-I259** claimed S2-05 `[x]` scoped ([stage2-s2-05-claim](stage2-s2-05-claim.md)); Stage-2 stays `[~]`.
- Construction ladder completed through I258 peels + I259 claim; default next = **S2-06 open** — not Stage-2 exit.
- Historical next at open = **S2-I166 `syx-mir` first extract** — landed; see census ladder.
- **S2-I166:** [mir-types-extract](stage2-s2-05-mir-types-extract.md) landed — `mir_types.rs`; next = **S2-I167 codec/blob**（≠ S2-05 `[x]` / ≠ Stage-2 exit）.
- **S2-I167:** [mir-codec-extract](stage2-s2-05-mir-codec-extract.md) landed — `mir_codec.rs`; next = **S2-I168 AOT shaken/compact**（≠ S2-05 `[x]` / ≠ Stage-2 exit）.
- **S2-I168:** [mir-aot-extract](stage2-s2-05-mir-aot-extract.md) landed — `mir_aot.rs`; next = **S2-I169 mir_opt**（≠ S2-05 `[x]` / ≠ Stage-2 exit）.
- **S2-I169:** [mir-opt-extract](stage2-s2-05-mir-opt-extract.md) landed — `mir_opt.rs`; next = **S2-I170 mir_lower 首抽**（≠ S2-05 `[x]` / ≠ Stage-2 exit）.
- **S2-I170:** [mir-module-index-extract](stage2-s2-05-mir-module-index-extract.md) landed — `mir_module_index.rs`; next = **S2-I171 LowerCtx**（≠ S2-05 `[x]` / ≠ Stage-2 exit）.
- Prior: [s2-04-claim](stage2-s2-04-claim.md) · S2-I164.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-04-claim](stage2-s2-04-claim.md) · S2-I164
- [stage2-file-split-census](../stage2-file-split-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-05
