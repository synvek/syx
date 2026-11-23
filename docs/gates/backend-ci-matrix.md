# Backend / CI support matrix (C-2a SSOT)

**Role:** Single source of truth for which backends are **default-hard** vs **soft / experimental**.  
**Live backlog:** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-2  
**Status snapshot:** [`plan/01-project-status.md`](../../plan/01-project-status.md) · Native 2.0 / CI soft  
**Census:** [`docs/census/c-2a-backend-ci-matrix-census.md`](../census/c-2a-backend-ci-matrix-census.md)

**Date:** 2026-11-23 · **C-2a**

## Contract

| Tier | Meaning |
|------|---------|
| **default_hard** | Merge / census KPI floor. Soft-skip must **not** count as “supported”. |
| **soft_non_default** | May run with `continue-on-error` or L006 skip; **≠ product green / ≠ default KPI**. |
| **experimental** | Opt-in pilot only; never default KPI. |

### Hard rule (C-2a)

**L006 skip / soft native job green ≠ native supported.**  
LLVM refuse-without-`full-native` uses diagnostic code **L006** (toolchain gate; ≠ book “illegal char” L006).

---

## default_hard

| Surface | Floor | Evidence (today) |
|---------|-------|------------------|
| **VM** | runnable / check smoke | required `lint` + driver / census |
| **MIR** | runnable path | required CI + driver |
| **Wasm** | **46/46** | `--test wasm` (census); required `compiler-wasm` / `wasm-aot-strict` / `wasm-aot-full` |
| **Browser** | **0** failed | `--test browser` (census); note: CI job `browser-e2e` is still **soft** |
| **Book output** | `sy,output` **12/12** | `scripts/book_pass_rate.py` (census) |
| **Object / simd residual** | **≤180/≤10** · **≥159** | packages scan (census) |

These are the **default support claims** for SYX as research beta. Do not advertise Native AOT or JIT as default-supported on the strength of soft CI alone.

---

## soft_non_default

| Job / path | CI / code | Why soft |
|------------|-----------|----------|
| `native-linux` | `.github/workflows/ci.yml` · `continue-on-error: true` · `soft_native` → L006 **return 0** | Default CLI lacks `--features full-native` |
| `native-macos` | same pattern | same |
| `browser-e2e` | `continue-on-error: true` | Soft e2e wall; hard browser floor is census `browser_failed=0` |
| Extended / differential soft walls | `continue-on-error` | Unpaid vs default_hard |
| `scripts/s0_backend_smoke.sh` | L006 **FAIL-CLOSED** counts as pass on required lint | Toolchain honesty, not native GA |
| `run_native_strict_or_mirror_vm` | [`native_strict.rs`](../../crates/syx-driver/tests/backends/native_strict.rs) | No AOT → **mirrors VM** (still green) — **C-2b** to hard-fail |

**Capability split (registered, not fixed here):** many product rows `backends.native = "real"` while `ci.job.native-*` notes unpaid / ≠ full-native GA. Demoting `native=real` en masse = **C-2b**.

---

## experimental

| Surface | Label | Notes |
|---------|-------|-------|
| **JIT** (Suite-1 / Cranelift) | experimental / unpaid | CI job `jit` · `continue-on-error`; **non_default_kpi** |
| SyxHost / CST deep / true implicit `CallSuper` | park | Not default next |

C-2c will unify public “Stub vs experimental-real-subset” wording; this SSOT already marks JIT **experimental**.

---

## Non-claims (C-2a)

- ≠ CI install LLVM / required full-native / ban `soft_native` exit 0 (**C-2b**)
- ≠ delete `run_native_strict_or_mirror_vm` or fix GA mirror (**C-2b**)
- ≠ bulk demote capability `native=real` (**C-2b**)
- ≠ full FeaturePriority JIT Stub scrub (**C-2c**)
- ≠ rename every capability `ci.job.*` to match yaml (**C-2d**)
- ≠ R-50

---

## Next

Default next after C-2a exit: **C-2b** — `native=real` must share full-native toolchain; failure = red; ban L006 soft-skip masquerading as native green.
