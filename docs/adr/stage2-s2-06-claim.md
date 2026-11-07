# ADR: Stage-2 S2-06 plan claim (S2-I262)

**Status:** Accepted (S2-I262) · S2-06 `[x]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I260 census open](stage2-s2-06-open.md) and [S2-I261 exclude syx-gui](stage2-s2-06-exclude-syx-gui.md), the S2-06 construction ladder meets the scoped acceptance bar: default workspace no longer includes the GUI product crate; Farm/Dist/HSM/CDN remain absent as crates. This dedicated window flips the plan checkbox only — aligned with [S2-I259 S2-05 claim](stage2-s2-05-claim.md) / [S2-I164 S2-04 claim](stage2-s2-04-claim.md). It does **not** exit Stage-2, delete or relocate `syx-gui`, or open S2-07.

## Evidence

| Axis | Evidence |
|------|----------|
| Open baseline | [stage2-s2-06-open](stage2-s2-06-open.md) · [stage2-workspace-census](../stage2-workspace-census.md) · S2-I260 |
| GUI out of default workspace | Root `exclude = ["crates/syx-gui"]` · S2-I261 · metadata packages exclude `syx-gui` |
| Farm/Dist/HSM/CDN | **0** named crates under `crates/` |
| Opt-in residual OK | `crates/syx-gui/` on disk; `syx-vm` path + `gui-syx-isolated`; driver gui tests / narrative residual |
| Contract chain | `s2_06_workspace_census` · `s2_06_exclude_syx_gui` · this claim |
| Honesty OUT | **≠** delete gui · **≠** move `experimental/` · **≠** Stage-2 exit |

## Decision

1. Flip [`plan/02`](../../plan/02-improvement-roadmap.md) **S2-06 → `[x]`**.
2. Keep **Stage-2 → `[~]`** until S2-07 acceptance (or explicit Stage-2 exit ADR).
3. **Scoped `[x]` meaning:** default `cargo check --workspace` does not build `syx-gui`; Farm/Dist/HSM/CDN have **no** sibling crates; on-disk GUI + path/feature opt-in + driver tests/narrative residual **OK** — **not** 「gui deleted」 / **not** 「zero product residual」 / **not** 「migrated under `experimental/` required」.
4. Update [stage2-s2-06-open](stage2-s2-06-open.md) / [stage2-s2-06-exclude-syx-gui](stage2-s2-06-exclude-syx-gui.md) / [stage2-open](stage2-open.md) Consequences; census Current=I262; next default = **S2-07 open**.
5. **Non-claims (hard):** ≠ Stage-2 exit / ≠ open or complete S2-07 construction this window / ≠ delete `syx-gui` / ≠ move under `experimental/` / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ Frontend CST migrate / ≠ change GUI or language semantics.

## Consequences

- S2-06 plan-claimed; Stage-2 stays `[~]` (S2-01…06 `[x]`; S2-07 open).
- Decision §4 locked next = **S2-07 open**; default next window = **S2-I263 S2-07 open** — not Stage-2 exit.
- Prior: [s2-06-exclude-syx-gui](stage2-s2-06-exclude-syx-gui.md) · S2-I261.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-06-open](stage2-s2-06-open.md) · S2-I260
- [stage2-s2-06-exclude-syx-gui](stage2-s2-06-exclude-syx-gui.md) · S2-I261
- [stage2-s2-05-claim](stage2-s2-05-claim.md) · S2-I259 (claim pattern)
- [stage2-workspace-census](../stage2-workspace-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-06
