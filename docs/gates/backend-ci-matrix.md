# Backend / CI support matrix (C-2a SSOT)

**Role:** Single source of truth for which backends are **default-hard** vs **soft / experimental**.  
**Live backlog:** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-2  
**Status snapshot:** [`plan/01-project-status.md`](../../plan/01-project-status.md) · Native 2.0 / CI soft  
**Census:** [`docs/census/c-2a-backend-ci-matrix-census.md`](../census/c-2a-backend-ci-matrix-census.md)

**Date:** 2026-11-24 · **C-2a…C-2c**（`-N` = N×历史 `+`）

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
| **Native paid floor** | required `full-native-linux` hello + path_file + tcp + sql + http GA + pool/row llvm + http.types/server/stream/timeout llvm + file raii/stream/watch llvm + tls.client aot | `.github/workflows/ci.yml` · LLVM 21.1.6 · `--features full-native` · **no** `continue-on-error` (C-2b-2…13) |

These are the **default support claims** for SYX as research beta. Do not advertise product-wide Native AOT GA or JIT as default-supported on soft CI alone; paid raises ≠ bulk `native=real`; JIT remains experimental (**C-2c**).

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

**Capability:** C-2b-1 demoted unpaid → `partial`. Raised: path×2 (C-2b-3) · tcp×2 (C-2b-4) · `data.sqlite` (C-2b-5) · `net.http` (C-2b-6) · pool+row (C-2b-7) · `net.http.types` (C-2b-8) · `net.http.server` (C-2b-9) · `net.http.stream` (C-2b-10) · `net.http.timeout_tls` (C-2b-11) · io.file.raii·stream·watch (C-2b-12) · `net.tls.client` (C-2b-13). Remaining product rows stay `partial` (incl. `net.tls.server`). JIT public labels scrubbed (**C-2c**); next axis **C-2d**.

---

## experimental

| Surface | Label | Notes |
|---------|-------|-------|
| **JIT** (Suite-1 / Cranelift) | experimental / unpaid | CI job `jit` · `continue-on-error`; **non_default_kpi** |
| SyxHost / CST deep / true implicit `CallSuper` | park | Not default next |

**C-2c (done):** public FeaturePriority / env-registry / AnalysisReport wording unified to **experimental-real-subset · default off · non_default_kpi** (≠ Stub / ≠ default-on).

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

### C-2b-6 (done · http paid raise)
- `full-native-linux` also runs `http_native_wasm_ga` with `--features full-native`
- **Raised** `net.http` → `native=real` (notes cite C-2b-6；≠ server / types / stream / timeout_tls)
- Product `native=real` total = **6**

### C-2b-7 (done · pool/row paid raise)
- `full-native-linux` also runs `hostfn_sql_pool_lifecycle_native_llvm` + `hostfn_sql_row_accessors_native_llvm` (`--test stdlib`)
- **Raised** `data.sqlite.pool` + `data.sqlite.row` → `native=real` (notes cite C-2b-7)
- Product `native=real` total = **8**

### C-2b-8 (done · http.types paid raise)
- `full-native-linux` also runs `m2_s4_01_response_accessors_native_llvm` (`--test stdlib`)
- **Raised** `net.http.types` → `native=real` (notes cite C-2b-8；≠ server / stream / timeout_tls)
- Product `native=real` total = **9**

### C-2b-9 (done · http.server paid raise)
- `full-native-linux` also runs `m2_s4_03b_graceful_shutdown_native_llvm` (`--test stdlib`)
- **Raised** `net.http.server` → `native=real` (notes cite C-2b-9；≠ stream / timeout_tls)
- Product `native=real` total = **10**

### C-2b-10 (done · http.stream paid raise)
- `full-native-linux` also runs `m2_s4_04_http_body_stream_native_llvm` (`--test stdlib`)
- **Raised** `net.http.stream` → `native=real` (notes cite C-2b-10；≠ timeout_tls)
- Product `native=real` total = **11**

### C-2b-11 (done · timeout_tls paid raise)
- `full-native-linux` also runs `m2_s4_02b_timeout_hang_loopback_native_aot` (`--test stdlib`)
- **Raised** `net.http.timeout_tls` → `native=real` (notes cite C-2b-11；≠ net.tls.client / skip-verify / server TLS)
- Product `native=real` total = **12**

### C-2b-12 (done · io.file.raii·stream·watch paid raise)
- `full-native-linux` also runs `m2_s3_02b_with_open_native_llvm` + `m2_s3_03_file_stream_read_chunk_native_llvm` + `m2_s3_03_file_watcher_native_llvm` (`--test stdlib`)
- **Raised** `io.file.raii` + `io.file.stream` + `io.file.watch` → `native=real` (notes cite C-2b-12；≠ net.tls.* / recursive watch GA / File.watch one-shot)
- Product `native=real` total = **15**

### C-2b-13 (done · tls.client paid raise)
- `full-native-linux` also runs `m2_s4_05_https_loopback_ok_with_extra_root_native_aot` + `m2_s4_05_self_signed_fail_closed_native_aot` (`--test stdlib`)
- **Raised** `net.tls.client` → `native=real` (notes cite C-2b-13；≠ skip-verify / server TLS / daemon / `net.tls.server`)
- Product `native=real` total = **16**

### C-2c (done · JIT experimental label scrub)
- Public FeaturePriority §23 / env-registry `SYX_JIT_TIER2` / AnalysisReport JIT rows → **experimental-real-subset · default off · non_default_kpi**
- CI job `jit` stays `continue-on-error`; product `native=real` count unchanged (**16**)
- ≠ JIT Suite-1 expand / ≠ default-on JIT / ≠ hard KPI

### Still park → **C-2d**
- ≠ `net.tls.server` paid raise (park；≠ daemon GA)
- ≠ macOS required full-native
- ≠ rename every capability `ci.job.*` to match yaml (**C-2d**)
- ≠ R-50

---

## Next

Default next after C-2c exit: **C-2d** — align `ci.job.*` capability surface names with yaml job ids.
