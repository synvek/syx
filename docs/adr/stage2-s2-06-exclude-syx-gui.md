# ADR: Stage-2 S2-06 exclude syx-gui from default workspace (S2-I261)

**Status:** Accepted (S2-I261) · S2-06 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I260 S2-06 open](stage2-s2-06-open.md), the remaining default-member product face was [`syx-gui`](../../crates/syx-gui) under `members = ["crates/*"]`. This window removes it from the **default** workspace via `[workspace] exclude`, keeps the on-disk crate + `syx-vm` path/feature opt-in, and does **not** claim S2-06 `[x]` or exit Stage-2.

## Decision

1. Root [`Cargo.toml`](../../Cargo.toml): keep `members = ["crates/*"]`; add `exclude = ["crates/syx-gui"]`.
2. Rewrite [`crates/syx-gui/Cargo.toml`](../../crates/syx-gui/Cargo.toml) without workspace inheritance (excluded packages cannot inherit); pin version `0.1.3` / edition / path deps to `syx-diagnostics` + `syx-runtime-core` + explicit third-party versions.
3. Keep [`syx-vm`](../../crates/syx-vm/Cargo.toml) `syx-gui = { path = "../syx-gui", optional = true }` + feature `gui-syx-isolated`.
4. Thin script/CI: census emits `syx_gui_workspace_excluded`; `test_p105_gates.sh` uses `--manifest-path crates/syx-gui/Cargo.toml`; CI drops redundant `--exclude syx-gui`.
5. **Non-claims (hard):** ≠ S2-06 `[x]` / ≠ Stage-2 exit / ≠ delete `syx-gui` / ≠ move under `experimental/` this window / ≠ open S2-07 / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ change GUI semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Root exclude | `exclude = ["crates/syx-gui"]` in root `Cargo.toml` |
| Metadata | `cargo metadata --no-deps` packages **do not** include `syx-gui` |
| Directory + opt-in | `crates/syx-gui/` still present; vm path + `gui-syx-isolated` intact |
| Census / contract | [stage2-workspace-census](../stage2-workspace-census.md) · `s2_06_exclude_syx_gui_contract` |
| Plan status | S2-06 remains `[~]` |

## Consequences

- Default workspace no longer builds `syx-gui`; opt-in path remains.
- S2-06 stays `[~]`; Stage-2 stays `[~]`.
- Default next = **S2-I262 S2-06 `[x]` claim**; **landed** as [s2-06-claim](stage2-s2-06-claim.md)（S2-I262）；next = **S2-I263 S2-07 open**.
- Prior: [s2-06-open](stage2-s2-06-open.md) · S2-I260.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-06-open](stage2-s2-06-open.md) · S2-I260
- [stage2-workspace-census](../stage2-workspace-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-06
