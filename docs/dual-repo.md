# Dual-repository policy (public + internal private)

SYX open-source cleanup (**plan-001**) uses two sibling checkouts:

| Role | Contents |
|------|----------|
| **This repository (public)** | Language, runtime, behavioral tests, capability tables, public ADR summary |
| **Internal private sibling** | Window ledger, tip matrix overlay, `qs1_*` tip nails, `inventory/qs1_ledger.rs`, ADR window Decision stream, Cursor plan archives |

## Rules

1. **Feature and behavioral-test PRs land only in the public repository.** Do not change language/runtime semantics only in the private tree.
2. **Public CI must not nail tip** (`A-W*`) or HostFn matrix ordinal fingerprints. Tip lives in the private overlay + harness.
3. **Public docs** describe capabilities via [adr/capabilities-summary.md](adr/capabilities-summary.md). Window Decision streams are **internal** and are not a build dependency.
4. **Public must not path-depend** on the private checkout for default `cargo test` / `cargo build`. Optional private harnesses may depend on a sibling public path when run by maintainers.
5. Contributors do **not** need the private repo to build, test, or contribute features.

## Maintainer pointers

- Cleanup program: [plan/plan-001-cleanup.md](../plan/plan-001-cleanup.md)
- Phase 6 scan notes: [plan/cleanup-manifest/phase6-scan.md](../plan/cleanup-manifest/phase6-scan.md)
- ADR dual-track: [adr/README.md](adr/README.md)

Phase **6b** (plans untrack, ADR peel, `qs1_ledger` overlay, plan tip compress) and **Phase 7** (ship commits, residual W354 peel, formal unfreeze) are **done** — see [plan/plan-001-cleanup.md](../plan/plan-001-cleanup.md) §9. Feature windows may resume with explicit ADR/window plans; tip nails stay in the private overlay.
