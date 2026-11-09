# ADR: Stage-3 S3-07 default-vm construct（S3-I83）

**Status:** Accepted (S3-I83) · S3-07 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I82 S3-07 open](stage3-s3-07-open.md), census baseline showed `default_value = "native"`, `init_vm_named_template_count=0`, and no init→run integration nail — while `syx run` already takes the **VM** path. This window is the first **construct knife**: align `syx init` default with the trusted backend. It does **not** claim S3-07 `[x]` or exit Stage-3.

## Decision

1. Change `Init` `--template` **`default_value` → `"vm"`**.
2. Add explicit **`"vm"`** match arm (`print("vm")` + plain `build.syb`) and explicit **`"native"`** arm (former default scaffold).
3. Keep `wasm` / `plugin` / `lib`; unknown names fall back to native-shaped scaffold (`_` → `print("native")`).
4. Nail with contract + census KPI flip; ladder next = **S3-I84** residual-or-claim.
5. **Non-claims (hard):** ≠ S3-07 `[x]` / ≠ Stage-3 exit / ≠ UsageGuide 大改 / ≠ 抽 Init crate / ≠ HostFn+1 / ≠ Wave·F-08 / ≠ reopen S3-06 LSP / S3-05 lint / S3-04 fib&lt;0.5 / ≠ change `build --native` semantics / ≠ language semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| CLI hinge | [`crates/syx-cli/src/main.rs`](../../crates/syx-cli/src/main.rs) `default_value = "vm"` · `"vm"` / `"native"` arms |
| Census | `init_default_template_native=0` · `init_default_template_vm≥1` · `init_vm_named_template_count≥1` · `init_integration_test_count≥1` |
| Contract | `s3_07_default_vm_contract` |
| Prior | [S3-I82 open](stage3-s3-07-open.md) |

## Consequences

- Default init scaffold aligns with VM / `syx run`; S3-07 was `[~]` under this construct window; Stage-3 stays `[~]`（later **7/7** under S3-I85）。
- **S3-I84 landed** as [stage3-s3-07-residual-scope](stage3-s3-07-residual-scope.md)（scoped acceptance · deferred UsageGuide/crate）。
- **S3-I85 landed** as [stage3-s3-07-claim](stage3-s3-07-claim.md)（S3-07 `[x]` scoped claim）。
- **S3-I86 landed** as [stage3-exit-open](stage3-exit-open.md)（Stage-3 exit open）。
- Default next = **S3-I87**（write `stage3-exit`）— not Stage-3 exit itself.
- Prior: [stage3-s3-07-open](stage3-s3-07-open.md) · S3-I82.
- Init-template census Current → **S3-I85** · Claimed scoped `[x]`；exit census Current → **S3-I86**.

## References

- [stage3-s3-07-open](stage3-s3-07-open.md) · S3-I82
- [stage3-init-template-census](../census/stage3-init-template-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-07
- [`crates/syx-cli/src/main.rs`](../../crates/syx-cli/src/main.rs)
