# ADR: Stage-3 S3-02 open — hide internal monomorph names (S3-I42)

**Status:** Accepted (S3-I42) · S3-02 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I41 S3-01 claim](stage3-s3-01-claim.md), Stage-3 default work shifts to [plan/02 S3-02](../../plan/02-improvement-roadmap.md): user-facing surfaces show generic forms, not internal monomorph mangled names (`Foo_Int`). Honest inventory shows `mangle_name` in mono/codegen and `type_id_to_name(Named)` passthrough feeding diagnostics/LSP; debug may expose MIR mangled fn names. This window **opens** the workstream with a mono-display **census baseline** — it does **not** claim S3-02 `[x]`, demangle product paths, or exit Stage-3.

## Gap: user display vs plan S3-02

| Claim | Today (honest) | plan/02 S3-02 target |
|-------|----------------|----------------------|
| User diags / type names | `type_id_to_name(Named)` clones raw name (may be `Foo_Int`) | Generic form e.g. `Foo<Int>` |
| LSP / IDE hover | Shares `type_id_to_name` / related paths | Same user-facing form |
| Debug stack fn names | MIR / mangled (e.g. `Greeter_hello`) | Readable generic/method form |
| Runtime `__type` | `object_ref_type_name` may carry internal names | Consistent user surface |
| Internal mono / codegen | `mangle_name` **OK** as IR identity | **Out of S3-02 user-face scope** |

Baseline: [stage3-mono-display-census](../census/stage3-mono-display-census.md).

## Decision

1. **Open S3-02** as an active Stage-3 workstream; flip plan S3-02 `[ ]` → `[~]`.
2. **S3-I42 first slice** — publish mono-display census (+ script); add a thin contract under `syx-driver/tests/` (**no** product display-source edit this window).
3. **Ladder:** **S3-I43** = first construct knife on user-facing display (priority: semantic `type_id_to_name` / `user_facing_type_name` demangle path for diagnostics) → residual LSP/debug/runtime faces → dedicated **S3-02 `[x]` claim**.
4. **Non-claims (hard):** ≠ S3-02 `[x]` / ≠ Stage-3 exit / ≠ demangle or rewrite `type_id_to_name` / LSP / debug / runtime this window / ≠ open S3-03…07 construction / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ change diagnostics `primary_span` defaults / ≠ change language semantics / ≠ force mono/codegen to stop mangling.

## Evidence (this window)

| Check | Evidence |
|-------|----------|
| Mono-display census | [`docs/census/stage3-mono-display-census.md`](../census/stage3-mono-display-census.md) · `scripts/s3_02_mono_display_census.sh` |
| Contract nail | `s3_02_mono_display_census_contract` |
| Plan status | S3-02 `[~]` under this ADR · Stage-3 `[~]` · S3-01 remains `[x]` |

## Consequences

- S3-02 moves `[ ]` → `[~]`; Stage-3 stays `[~]`; S3-01 stays `[x]`.
- **S3-I43 landed** as [stage3-s3-02-demangle-semantic](stage3-s3-02-demangle-semantic.md)（Named demangle hinge；`named_passthrough=0`）.
- **S3-I44 landed** as [stage3-s3-02-debug-stack-display](stage3-s3-02-debug-stack-display.md)（DAP/summary 用户面名）.
- **S3-I45 landed** as [stage3-s3-02-runtime-type-display](stage3-s3-02-runtime-type-display.md)（runtime `__type` 用户面）.
- **S3-I46 landed** as [stage3-s3-02-meta-type-display](stage3-s3-02-meta-type-display.md)（meta 用户面）.
- **S3-I47 landed** as [stage3-s3-02-claim](stage3-s3-02-claim.md)（S3-02 `[x]` scoped claim）.
- **S3-I48 landed** as [stage3-s3-03-open](stage3-s3-03-open.md)（S3-03 `[~]` open + census）.
- Default next construction window = **S3-I49 first construct** — not Stage-3 exit.
- Prior: [stage3-s3-01-claim](stage3-s3-01-claim.md) · S3-I41.
- Mono-display census Current → **S3-I47**.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-claim](stage3-s3-01-claim.md) · S3-I41
- [stage3-mono-display-census](../census/stage3-mono-display-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-02
