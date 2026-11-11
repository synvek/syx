# R-37 CI restore census

**Date:** 2026-11-11  
**Scope:** todo-005 A1–A5 — `cargo fmt` 过关；主干不全红；不可修 job 诚实 soft-fail + 能力表。  
**Script:** [`scripts/r37_ci_census.sh`](../../scripts/r37_ci_census.sh) · fmt gate [`scripts/r37_fmt_check.sh`](../../scripts/r37_fmt_check.sh)

## Progress

| Track | Status |
|-------|--------|
| Cred / SA / R-0…R-36 | exited (wasm **46/46**; browser **46/0/2**; Object **186/11**) |
| **R-37** | CI restore: Format green; soft-fail unpaid matrix; capability honesty |
| Object / simd | **186/11** / unchanged |

## Root cause

1. **Format:** workspace unformatted; rustfmt blocked on missing gitignored `qs1_ledger.rs` mod.
2. **All-red optics:** zero `continue-on-error`; native/parity/jit/gui/macos hard-fail (native link → R-39).
3. **book-pass / windows-smoke:** check 100% residual / evergreen smoke unpaid — soft until R-38 / thin fix.
4. **A4:** branch protection can only require **job** names (`build`, optionally hard `book-pass`); `s0_backend_smoke` is a step under `build`, not a checkable job.

## Delivery

1. `cargo fmt --all` + public stub [`qs1_ledger.rs`](../../crates/syx-driver/src/qs1_ledger.rs); thin [`r37_fmt_check.sh`](../../scripts/r37_fmt_check.sh).
2. [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml): job-level `continue-on-error` on `native-strict-main`, `native-macos`, `parity-gate`, `jit-tier2-default`, `phase125-gui-xvfb`, `phase125-driver-gates`, `windows-smoke`, `book-pass`.
3. Hard retain: `build`, `build-compiler-wasm32`, `wasm-aot-matrix-strict`, `wasm-aot-matrix-full`.
4. Capability honesty rows `ci.job.*` in [`capabilities.toml`](../../packages/internal/capabilities.toml).

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `cargo fmt --all -- --check` | green | **pass** |
| browser_failed | **0** | **0** (46/0/2) |
| wasm | **46/46** | **46/46** |
| Object | **≤186 / ≤11** | **186/11** |

## A4 branch protection (operator checklist)

Required checks (when soft jobs are non-blocking for merge optics):

1. **`build`** (includes Format + `s0_backend_smoke` step)
2. Prefer also: `build-compiler-wasm32`, `wasm-aot-matrix-strict` (currently hard)

`book-pass` is **soft** this window (check residual → R-38); do **not** claim it as a hard required check until rate_pct==100 without soft-fail.

```bash
# Requires repo admin; gh may be unavailable in agent env — run locally:
gh api repos/synvek/syx/branches/main/protection \
  -X PUT --input - <<'EOF'
{
  "required_status_checks": {
    "strict": true,
    "contexts": ["build", "build-compiler-wasm32", "wasm-aot-matrix-strict"]
  },
  "enforce_admins": true,
  "required_pull_request_reviews": null,
  "restrictions": null
}
EOF
```

If API/auth unavailable: leave this checklist in census; **do not** claim protection already configured.

## Non-claims

- ≠ R-38 book `run`+stdout GA / ≠ book-pass 100% hard again
- ≠ R-39 full-native link fail-closed (soft native jobs unpaid)
- ≠ extracting Playwright as its own job
- ≠ Object/simd / LANG-POLY
