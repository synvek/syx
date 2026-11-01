# Stage-2 governance census (S2-01)

**ADR:** [stage2-open](adr/stage2-open.md)  
**Goal (S2-01):** process-governance inventory / fingerprint / gate narrative **out of product `src/`** → target **0** `*_inventory` linked into product binaries.

## Command

```bash
bash scripts/s2_governance_census.sh
```

## S2-I1 baseline (2026-11-01)

| Metric | Count |
|--------|------:|
| `fn *_inventory` matches | 237 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 |
| `fn *_fingerprint` matches | 655 |
| `crates/syx-driver/src/*_report.rs` files | 26 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 |
| `Always \`true\`` / `Always \`false\`` comment matches | 2162 |

## S2-I2 after first knife (2026-11-01)

**Slice:** deleted `syx-driver` W932–W954 C-08/C-09 tooling + residual/maintenance inventory chain (~L9861–10754 historical); removed companion `include_str`/inventory tests `platform_residual_maintenance_first_slice` / `math_residual_maintenance_first_slice`. **Kept** W955+ `syx_math_llvm_*` and all `*_report.rs` / `lang_props` / `gate_b09` / `gate_a06`.

| Metric | Count | Δ vs S2-I1 |
|--------|------:|-----------:|
| `fn *_inventory` matches | **214** | **−23** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 2073 | −89 |

## Non-claims

- **≠** S2-01 `[x]` / Stage-2 exit.
- Re-run the script after each S2-01 migration slice; update this table when materially changed.
