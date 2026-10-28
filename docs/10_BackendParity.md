# SYX backend parity (VM · MIR/WASM · LLVM)

[简体中文](./zh/10_BackendParity.zh-CN.md)

Documentation entry for backend feature parity (behavior is defined by `crates/syx-driver/tests` and CI).

Main CI (`.github/workflows/ci.yml`) roughly runs:

```bash
cargo test -p syx-driver \
  --test evergreen --test lang --test concurrency --test stdlib \
  --test observability --test security --test backends \
  -- --test-threads=1
cargo test -p syx-lsp
# + capability freshness / tip-leak gates (see workflow)
```

Evergreen includes `parity_native` and related checks; historical standalone `parity_vm` / `parity_wasm` suite names follow the current `tests/` layout.

## Three-backend matrix

| Dimension | VM (`syx-vm`) | WASM (`syx-wasm-run`) | Native LLVM (`syx-llvm` + `syx-rt`) |
|-----------|---------------|------------------------|-------------------------------------|
| Entry | `syx run` | `syx run --wasm` / `syx build --wasm` | `syx build --native` |
| Role | Semantic baseline / day-to-day | Sandbox deploy (CLI near-full stdlib) | **Sole** native AOT |
| Mid-end | Unified MIR | Unified MIR | Unified MIR |

## Native backend (LLVM only)

- `syx build --native` → LLVM AOT; **no** Cranelift AOT (`syx-cranelift` retired).
- Experimental JIT: `syx-cranelift-jit` + `SYX_JIT_TIER2=1` (Suite-1; off by default; ≠ production default).
- Object / link flags: `--object`, `--release`, `full-native` — see [9_UsageGuide.md](9_UsageGuide.md).
- ObjectRef / host / plugin boundaries: [`docs/adr/architecture-decisions.md`](adr/architecture-decisions.md).

## Audit and fixtures

- Behavioral suites: `crates/syx-driver/tests/{lang,stdlib,native,concurrency,evergreen,backends,…}`
- Shared fixtures: `crates/syx-driver/tests/fixtures/` (when present)
- Capability table: [`books/shared/capabilities-generated.md`](../books/shared/capabilities-generated.md)

## Related docs

- [Architecture decisions](adr/architecture-decisions.md)
- [Backend debugging guide](12_BackendDebuggingGuide.md)
- [Complete technical documentation](5_CompleteTechnicalDocumentation.md)
- [Usage guide — compile & backends](9_UsageGuide.md)
