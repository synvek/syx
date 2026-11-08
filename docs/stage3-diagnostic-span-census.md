# Stage-3 diagnostic Span census (S3-01)

**ADR:** … · [s3-01-claim](adr/stage3-s3-01-claim.md) · [s3-02-open](adr/stage3-s3-02-open.md) · [demangle-semantic](adr/stage3-s3-02-demangle-semantic.md) · [debug-stack-display](adr/stage3-s3-02-debug-stack-display.md) · [runtime-type-display](adr/stage3-s3-02-runtime-type-display.md) · [meta-type-display](adr/stage3-s3-02-meta-type-display.md) · [s3-02-claim](adr/stage3-s3-02-claim.md) · **Window:** S3-I47 · **Current:** S3-I47

**Goal (S3-01):** every **user-facing** diagnostic carries `Span` / `primary_span`; positionless = CI defect; miette label/help.  
**This census:** prior face gates + filtered `positionless_helper_calls=0` — **S3-01 `[x]` claimed**（S3-I41）；**S3-02 `[x]` claimed**（S3-I47）；**≠** Stage-3 exit.

## Command

```bash
bash scripts/s3_01_span_census.sh
```

## Classification (hard)

| Class | Counts as S3-01 debt? | Notes |
|-------|----------------------|-------|
| **User diagnostic** | **Yes** | bare `SyxDiagnostic::error\|warning(` |
| **IR / MIR `span: None`** | **No** | `syx-mir` metadata |
| **Positionless helper** | **Yes（已清）** | 真实便利 `*_error(` → DUMMY |
| **Census noise / policy** | **No（I40 过滤）** | whitelist in I40 ADR |

## Noise / policy whitelist（I40）

**Noise：** `set_error` · `from_error` · `last_os_error` · `host_log_error` · `registered_body_error` · `diagnostics_on_parse_error` · `*_is/has/no_parse/non_error` · `p[0-9]+_*_error`

**Policy carve-out：** `lockfile_error` · `reflect_specialization_miss_error`

## Gated KPIs（bare = 0）

| KPI | Value |
|-----|------:|
| all prior `semantic_*` / `host_*` / `host_residual_bare` | **0** |
| `positionless_parser_driver_bare` | **0**（I26） |
| `positionless_driver_compile_bare` | **0**（I27） |
| `positionless_driver_emit_test_native_bare` | **0**（I28） |
| `positionless_vm_phase84_bare` | **0**（I29） |
| `positionless_vm_hosts_bare` | **0**（I30） |
| `positionless_vm_phase89_bare` | **0**（I31） |
| `positionless_vm_phase120_bare` | **0**（I32） |
| `positionless_vm_http_bare` | **0**（I32） |
| `positionless_vm_os_bare` | **0**（I32） |
| `positionless_vm_lib_bare` | **0**（I33） |
| `positionless_vm_phase83_bare` | **0**（I33） |
| `positionless_vm_channel_bare` | **0**（I33） |
| `positionless_vm_phase_tail_bare` | **0**（I33） |
| `positionless_vm_mid_bare` | **0**（I34） |
| `positionless_wasm_run_bare` | **0**（I34） |
| `positionless_pkg_bare` | **0**（I35） |
| `positionless_macro_bare` | **0**（I36） |
| `positionless_async_bare` | **0**（I37） |
| `positionless_llvm_bare` | **0**（I37） |
| `positionless_rt_bare` | **0**（I38） |
| `positionless_jit_bare` | **0**（I38） |
| `positionless_debug_bare` | **0**（I38） |
| `positionless_codegen_bare` | **0**（I38） |
| `positionless_mir_bare` | **0**（I39） |
| `positionless_mir_run_bare` | **0**（I39） |
| `positionless_meta_bare` | **0**（I39） |
| `positionless_hir_bare` | **0**（I39） |
| `positionless_cst_bare` | **0**（I39） |
| `positionless_wasm_emit_bare` | **0**（I39） |
| `positionless_fmt_bare` | **0**（I39） |
| `positionless_compiler_wasm_bare` | **0**（I39） |
| `positionless_lexer_bare` | **0**（I39） |
| `positionless_lint_bare` | **0**（I39） |
| `positionless_helper_calls`（filtered） | **0**（I40） |

## Claim-prep quality KPIs

| KPI | Value | Notes |
|-----|------:|-------|
| `with_span_calls` | ~48+ | |
| `primary_span_none_literal` | ~48+ | |
| `positionless_helper_calls` | **0** | I40 filtered |
| `positionless_helper_noise` | **~26** | 透明 |
| `positionless_helper_policy` | **~5** | 透明 |
| `miette_help_impl` | **1**（I25） | |
| `with_span_label_wired` | **1**（I25） | |

## S3-I41 S3-01 claim (2026-11-08)

| Metric | Value |
|--------|------:|
| S3-01 plan | **`[x]`**（scoped） |
| Stage-3 | **`[~]`** |
| Claim ①②③④ | **aligned / claimed** |

## Ladder (locked)

1. S3-I1…I40 — done.
2. **S3-I41:** S3-01 `[x]` claim — landed.
3. **S3-I42:** **S3-02 open** — landed；see [mono-display census](stage3-mono-display-census.md).
4. **S3-I43:** S3-02 demangle-semantic hinge — landed.
5. **S3-I44:** debug stack display — **landed**.
6. **S3-I45:** runtime `__type` user-facing — **landed**.
7. **S3-I46:** meta type user-facing — **landed**.
8. **S3-I47:** S3-02 scoped claim — **landed**.
9. **Next:** **S3-03 open** — still ≠ Stage-3 exit.
7. Later: S3-02 claim / S3-03…07 / Stage-3 exit.

## Non-claims

- **≠** Stage-3 exit  
- **≠** S3-02 open/complete this window  
- **≠** HostFn+1 / F-08 / Wave  

## References

- [stage3-s3-01-claim](adr/stage3-s3-01-claim.md) · S3-I41
- [stage3-s3-01-noise-claim-prep](adr/stage3-s3-01-noise-claim-prep.md) · S3-I40
- [plan/02](../plan/02-improvement-roadmap.md) S3-01
