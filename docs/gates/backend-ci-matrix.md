# Backend / CI support matrix (C-2a SSOT)

**Role:** Single source of truth for which backends are **default-hard** vs **soft / experimental**.  
**Live backlog:** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-2  
**Status snapshot:** [`plan/01-project-status.md`](../../plan/01-project-status.md) · Native 2.0 / CI soft  
**Census:** [`docs/census/c-2a-backend-ci-matrix-census.md`](../census/c-2a-backend-ci-matrix-census.md)

**Date:** 2026-11-23 · **C-2a…C-2b-5**（`-N` = N×历史 `+`）

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
| **Native paid floor** | required `full-native-linux` hello + path_file + tcp + sql GA | `.github/workflows/ci.yml` · LLVM 21.1.6 · `--features full-native` · **no** `continue-on-error` (C-2b-2…5) |

These are the **default support claims** for SYX as research beta. Do not advertise product-wide Native AOT GA or JIT as default-supported on soft CI alone; paid raises ≠ bulk `native=real` (**C-2b-6**).

---

## soft_non_default

| Job / path | CI / code | Why soft |
|------------|-----------|----------|
| `native-linux` | `.github/workflows/ci.yml` · `continue-on-error: true` · L006 → **step fail** (C-2b+; ≠ soft-skip exit 0) | Default CLI lacks `--features full-native` |
| `native-macos` | same · L006 → **exit 1** (C-2b+) | same |
| `browser-e2e` | `continue-on-error: true` | Soft e2e wall; hard browser floor is census `browser_failed=0` |
| Extended / differential soft walls | `continue-on-error` | Unpaid vs default_hard |
| `scripts/s0_backend_smoke.sh` | L006 **FAIL-CLOSED** counts as pass on required lint | Toolchain honesty, not native GA |
| ~~`run_native_strict_or_mirror_vm`~~ | **C-2b exited** — helper **removed**; GA uses `try_run_native_strict` + **skip** (≠ VM mirror) | Forbidden: mirror VM stdout as native GA |

**Capability:** C-2b-1 demoted unpaid → `partial`. Raised: path×2 (C-2b-3) · tcp×2 (C-2b-4) · `data.sqlite` (C-2b-5). Remaining product rows stay `partial` until **C-2b-6** (incl. sqlite pool/row).

---

## experimental

| Surface | Label | Notes |
|---------|-------|-------|
| **JIT** (Suite-1 / Cranelift) | experimental / unpaid | CI job `jit` · `continue-on-error`; **non_default_kpi** |
| SyxHost / CST deep / true implicit `CallSuper` | park | Not default next |

C-2c will unify public “Stub vs experimental-real-subset” wording; this SSOT already marks JIT **experimental**.

---

## Non-claims

### C-2a (done)
- Documented hard/soft/experimental split; L006 skip ≠ native supported

### C-2b (done · mirror honesty)
- **Removed** `run_native_strict_or_mirror_vm`; GA nails skip without full-native AOT
- **Hard rule:** mirror VM stdout as native parity = **forbidden**

### C-2b+ (done · native honesty thin)
- **Demoted** unpaid product `native="real"` → `"partial"` (38 rows)
- **Banned** L006 soft-skip → step success (`return`/`exit` 0); jobs stay `continue-on-error`
- **Hard rule:** unpaid `native=real` = **forbidden** until paid full-native evidence

### C-2b++ / C-2b-2 (done · paid full-native floor)
- **Required** job `full-native-linux`: LLVM 21.1.6 → `cargo build -p syx-cli --features full-native` → `syx build --native examples/hello.sy`
- **No** `continue-on-error`; **no** `full-native || release` fallback on that job

### C-2b-3 (done · path_file paid raise)
- `full-native-linux` also runs `path_file_native_wasm_ga` with `--features full-native`
- **Raised** `io.path` + `io.file.read_all_bytes` → `native=real` (notes cite C-2b-3)

### C-2b-4 (done · tcp paid raise)
- `full-native-linux` also runs `tcp_native_wasm_ga` with `--features full-native`
- **Raised** `net.tcp` + `net.tcp.listener` → `native=real` (notes cite C-2b-4)

### C-2b-5 (done · sql paid raise)
- `full-native-linux` also runs `sql_native_wasm_ga` with `--features full-native`
- **Raised** `data.sqlite` → `native=real` (notes cite C-2b-5；≠ pool/row)
- Product `native=real` total = **5**

### Still park → **C-2b-6** / C-2c / C-2d
- ≠ more selective `native=real` (http / sqlite pool·row / other io.file.*) (**C-2b-6**)
- ≠ macOS required full-native
- ≠ full FeaturePriority JIT Stub scrub (**C-2c**)
- ≠ rename every capability `ci.job.*` to match yaml (**C-2d**)
- ≠ R-50

---

## Next

Default next after C-2b-5 exit: **C-2b-6** — next selective evidence-backed `native=real` batch (http or pool/row or remaining io.file.*).
