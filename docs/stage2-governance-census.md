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

## S2-I3 after second knife (2026-11-01)

**Slice:** deleted `syx-driver` W955–W965 `syx_math_llvm_*` inventory chain; removed companion test `math_llvm_ga_exit_first_slice` (+ orphaned I2 `mod` lines for residual maintenance tests). **Kept** `tests/stdlib/math.rs` behavior smokes; `*_report.rs` / `lang_props` / `gate_b09` / `gate_a06`.

| Metric | Count | Δ vs S2-I2 |
|--------|------:|-----------:|
| `fn *_inventory` matches | **203** | **−11** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 2018 | −55 |

## S2-I4 after third knife (2026-11-01)

**Slice:** truncated `syx-codegen` gate_b01+gate_b02 inventory tail (~L5580→EOF); removed 10 companion native tests. **≠** S2-03 / B-01·B-02 product exit. **Kept** `gate_b09` / mir horizon / `*_report` / `lang_props`.

| Metric | Count | Δ vs S2-I3 |
|--------|------:|-----------:|
| `fn *_inventory` matches | **186** | **−17** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 1884 | −134 |

## S2-I5 after fourth knife (2026-11-01)

**Slice:** deleted unused small packs — `gate_a06` + `profiles_sdk` (link) + `lsp_editor_interop_*` + `frontend_pipeline_entry_matrix` (+ companion driver test). **Kept** `gate_b09` / llvm_stub / wave_b / mir / `*_report`.

| Metric | Count | Δ vs S2-I4 |
|--------|------:|-----------:|
| `fn *_inventory` matches | **178** | **−8** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 1854 | −30 |

## S2-I6 after fifth knife (2026-11-01)

**Slice:** deleted `syx-mir` short post-cutover tail (`honest_defer`→`exit`) + full `language_exports_*` inventory chain; removed 4 `gate_b04_*` companions (kept `absolute_thresholds`) + 5 language_exports signing companions; kept trailing-comma **parser** behavior test. **Kept** `aot_short_horizon_cutover_readiness` (wave_b product read) / absolute_threshold / b09 / llvm_stub / medium·long.

| Metric | Count | Δ vs S2-I5 |
|--------|------:|-----------:|
| `fn *_inventory` matches | **160** | **−18** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 1802 | −52 |

## S2-I7 after sixth knife (2026-11-01)

**Slice:** deleted full `mir_module_long_horizon_*` inventory chain (25); removed 25 `long_mir_free_*` companions + 3 wave_b long-match unit tests. **Kept** short/medium `cutover_readiness`, llvm `clear_align`, wave_b hardcoded long flags, `gate_b09`.

| Metric | Count | Δ vs S2-I6 |
|--------|------:|-----------:|
| `fn *_inventory` matches | **132** | **−28** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 1152 | −650 |

## Non-claims

- **≠** S2-01 `[x]` / Stage-2 exit.
- Re-run the script after each S2-01 migration slice; update this table when materially changed.
