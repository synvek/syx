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

## S2-I8 after seventh knife (2026-11-01)

**Slice:** deleted mir medium post-cutover tail (`honest_defer`→`exit`, 11); removed 7 `gate_b05_*` companions. **Kept** `mir_module_medium_horizon_cutover_readiness` (wave_b product), short cutover, llvm `clear_align`, `gate_b09`.

| Metric | Count | Δ vs S2-I7 |
|--------|------:|-----------:|
| `fn *_inventory` matches | **121** | **−11** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 1075 | −77 |

## S2-I9 after eighth knife (2026-11-01)

**Slice:** deleted mir bypass pack (`layout` / `codec_partition` / `debug_strip` / `native_omit`, 4) + llvm post-clear tail (`honest_defer` / `align_horizon_exit`, 2). **Kept** production helpers `module_with_debug_locals_stripped` / `module_with_native_instructions_omitted`, short/medium `cutover_readiness`, llvm `clear_align`, HostFn×8, `gate_b09`.

| Metric | Count | Δ vs S2-I8 |
|--------|------:|-----------:|
| `fn *_inventory` matches | **115** | **−6** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 1068 | −7 |

## S2-I10 after ninth knife (2026-11-01)

**Slice:** deleted wave_b residual ABI pair (`mir_residual_honesty` + `mir_residual_abi_narrow_cleanup`, 2) + companion native tests. **Kept** `wave_b_tri_axis_*` / `wave_b_structural_exit_*`, short/medium `cutover_readiness`, llvm `clear_align`, HostFn×8, `gate_b09`, `absolute_threshold`.

| Metric | Count | Δ vs S2-I9 |
|--------|------:|-----------:|
| `fn *_inventory` matches | **113** | **−2** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 1053 | −15 |

## S2-I11 after tenth knife (2026-11-01)

**Slice:** cutover product decouple ([ADR](adr/stage2-cutover-decouple.md)): `wave_b` hard-codes `relational_b04/b05/b06`; deleted 3 rollups (`aot_short_horizon_cutover_readiness` / `mir_module_medium_horizon_cutover_readiness` / `llvm_stub_clear_align_horizon_readiness`). **Kept** feed chains, HostFn×8, `capability_align`, `gate_b09`, `absolute_threshold`, `wave_b_tri_axis_*` / structural.

| Metric | Count | Δ vs S2-I10 |
|--------|------:|------------:|
| `fn *_inventory` matches | **110** | **−3** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 1045 | −8 |

## S2-I12 after eleventh knife (2026-11-01)

**Slice:** deleted short export/blob/absolute feed×6 + inventory-only helper `encode_module_compact_only_for_inventory` + companion `gate_b04_absolute_thresholds` (`b04_absolute_thresholds_live_inventory`). **Kept** `aot_blob_size_report` / size fixtures / emit·compact production APIs; medium dual_blob feed; llvm clear upstream; HostFn/`capability_align`; `gate_b09`; wave_b tri/structural.

| Metric | Count | Δ vs S2-I11 |
|--------|------:|------------:|
| `fn *_inventory` matches | **102** | **−8** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 1045 | 0 |

## S2-I13 after twelfth knife (2026-11-01)

**Slice:** deleted medium metadata/dual_blob feed×5 (`metadata_table` / `dual_blob_shape` / `codec_readiness` / `aot_embed` / `cutover_readiness`) + encode/decode_probe helpers. **Kept** sidecar/dual_blob/AOT embed production APIs + wire tests; llvm clear upstream; HostFn/`capability_align`; `gate_b09`; wave_b tri/structural.

| Metric | Count | Δ vs S2-I12 |
|--------|------:|------------:|
| `fn *_inventory` matches | **97** | **−5** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 1032 | −13 |

## S2-I14 after thirteenth knife (2026-11-01)

**Slice:** deleted llvm clear upstream `declare_diff` + `missing_clear_readiness` (+ collect helpers / unit / driver W315). **Kept** `capability_align` / HostFn×8 / audit_report; wave_b tri/structural; `gate_b09`; product LLVM emit.

| Metric | Count | Δ vs S2-I13 |
|--------|------:|------------:|
| `fn *_inventory` matches | **95** | **−2** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 1030 | −2 |

## S2-I15 after fourteenth knife (2026-11-02)

**Slice:** renamed 10 driver test-side false-positive `fn *_inventory` (b07 `*_live_nails`×5, m2 `*_host_caps_nails`×3, m3/m4 `*_ci_and_signed`×2). **No product `src/` inventory deleted.** Remaining ≈85 are product-surface (`gate_b09`≈74 + HostFn/align≈9 + wave_b×2).

| Metric | Count | Δ vs S2-I14 |
|--------|------:|------------:|
| `fn *_inventory` matches | **85** | **−10** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 1030 | 0 |

## S2-I16 after fifteenth knife (2026-11-02)

**Slice:** retired wave_b tri+structural readiness inventories ([stage2-wave-b-inventory-retire](adr/stage2-wave-b-inventory-retire.md)) + companion thin tests. **Kept** `gate_b09` / HostFn×8 / `capability_align`; W340 exit charter evidence.

| Metric | Count | Δ vs S2-I15 |
|--------|------:|------------:|
| `fn *_inventory` matches | **83** | **−2** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 1013 | −17 |

## Non-claims

- **≠** S2-01 `[x]` / Stage-2 exit.
- Re-run the script after each S2-01 migration slice; update this table when materially changed.
