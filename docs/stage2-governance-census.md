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

## S2-I17 after sixteenth knife (2026-11-02)

**Slice:** HostFn←capability_align decouple ([stage2-hostfn-align-decouple](adr/stage2-hostfn-align-decouple.md)); deleted align inventory + W314 companion. **Kept** HostFn×8 / `audit_report` / `gate_b09`.

| Metric | Count | Δ vs S2-I16 |
|--------|------:|------------:|
| `fn *_inventory` matches | **82** | **−1** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 1013 | 0 |

## S2-I18 after seventeenth knife (2026-11-02)

**Slice:** b09 product-flag tip decouple ([stage2-b09-product-flag-decouple](adr/stage2-b09-product-flag-decouple.md)); five flags policy-fixed `true`; deleted tip product inventories×4 + companions. **Kept** b09 construction chain / HostFn×8 / flag API names.

| Metric | Count | Δ vs S2-I17 |
|--------|------:|------------:|
| `fn *_inventory` matches | **78** | **−4** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 953 | −60 |

## S2-I19 after eighteenth knife (2026-11-02)

**Slice:** b09 construction tip peel 两联 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted mutrefcell+reentry peel inventories×6 + companions. **Kept** dual_track 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I18 |
|--------|------:|------------:|
| `fn *_inventory` matches | **72** | **−6** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 863 | −90 |

## S2-I20 after nineteenth knife (2026-11-02)

**Slice:** b09 construction tip dual_track+mirop 两联 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted dual_track collapse + mirop residual peel inventories×6 + companions. **Kept** objectref peel 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I19 |
|--------|------:|------------:|
| `fn *_inventory` matches | **66** | **−6** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 769 | −94 |

## S2-I21 after twentieth knife (2026-11-02)

**Slice:** b09 construction tip objectref peel 三联 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted peel readiness/delete/post_peel_refresh inventories×3 + companion. **Kept** deep_field 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I20 |
|--------|------:|------------:|
| `fn *_inventory` matches | **63** | **−3** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 714 | −55 |

## S2-I22 after twenty-first knife (2026-11-02)

**Slice:** b09 construction tip deep_field+strip_rc 两联 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted deep_field + strip_rc inventories×4 + companions. **Kept** handle 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I21 |
|--------|------:|------------:|
| `fn *_inventory` matches | **59** | **−4** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 653 | −61 |

## S2-I23 after twenty-second knife (2026-11-02)

**Slice:** b09 construction tip handle 三联 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted post_handle_refresh + handle delete + handle retire inventories×3 + companion. **Kept** nested_field 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I22 |
|--------|------:|------------:|
| `fn *_inventory` matches | **56** | **−3** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 604 | −49 |

## S2-I24 after twenty-third knife (2026-11-02)

**Slice:** b09 construction tip nested_field ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted nested_field_dual_track_first_slice inventory×1 + companion. **Kept** refresh_thin 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I23 |
|--------|------:|------------:|
| `fn *_inventory` matches | **55** | **−1** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 589 | −15 |

## S2-I25 after twenty-fourth knife (2026-11-02)

**Slice:** b09 construction tip refresh+wrap+value 三联 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted refresh_thin + wrap delete + value_rc_api delete inventories×3 + companions. **Kept** storelocal_deinits 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I24 |
|--------|------:|------------:|
| `fn *_inventory` matches | **52** | **−3** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 550 | −39 |

## S2-I26 after twenty-fifth knife (2026-11-02)

**Slice:** b09 construction tip death-path 四联 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted storelocal_deinits + gc_type_deinit + deinits/storelocal heap_unroot inventories×4 + companions. **Kept** rc_release 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I25 |
|--------|------:|------------:|
| `fn *_inventory` matches | **48** | **−4** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 496 | −54 |

## S2-I27 after twenty-sixth knife (2026-11-02)

**Slice:** b09 construction tip opcode retire 三联 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted rc_release_opcode + rc_dup_loadlocal + rc_retain_opcode retire first-slice inventories×3 + companions. **Kept** rc_retire_readiness 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I26 |
|--------|------:|------------:|
| `fn *_inventory` matches | **45** | **−3** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 455 | −41 |

## S2-I28 after twenty-seventh knife (2026-11-02)

**Slice:** b09 construction tip retire-readiness 三联 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted rc_retire_readiness + rt_abi retain_release/wrap retire readiness inventories×3. **Kept** rc_migrate_readiness 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I27 |
|--------|------:|------------:|
| `fn *_inventory` matches | **42** | **−3** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 417 | −38 |

## S2-I29 after twenty-eighth knife (2026-11-02)

**Slice:** b09 construction tip migrate_readiness 单枚 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted rc_migrate_readiness_thin inventory×1. **Kept** ssa_loadlocal heap_slots 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I28 |
|--------|------:|------------:|
| `fn *_inventory` matches | **41** | **−1** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 403 | −14 |

## S2-I30 after twenty-ninth knife (2026-11-02)

**Slice:** b09 construction tip heap_slots 三联 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted ssa_loadlocal + loadglobal + objectset heap_slots inventories×3 + companions. **Kept** llvm wrap emit 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I29 |
|--------|------:|------------:|
| `fn *_inventory` matches | **38** | **−3** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 364 | −39 |

## S2-I31 after thirtieth knife (2026-11-02)

**Slice:** b09 construction tip llvm wrap emit 单枚 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted llvm_objectref_wrap_emit_retire inventory×1 + companion. **Kept** arrayget heap_slots 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I30 |
|--------|------:|------------:|
| `fn *_inventory` matches | **37** | **−1** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 350 | −14 |

## S2-I32 after thirty-first knife (2026-11-02)

**Slice:** b09 construction tip arrayget+residual_birth heap_slots 两联 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted arrayget + residual_birth inventories×2 + companions. **Kept** llvm_wasm emit retire 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I31 |
|--------|------:|------------:|
| `fn *_inventory` matches | **35** | **−2** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 322 | −28 |

## S2-I33 after thirty-second knife (2026-11-02)

**Slice:** b09 construction tip llvm_wasm emit-retire 单枚 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted llvm_wasm_objectref_rc_emit_retire inventory×1 + companion. **Kept** retain_catch_all heap_slots 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I32 |
|--------|------:|------------:|
| `fn *_inventory` matches | **34** | **−1** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 308 | −14 |

## S2-I34 after thirty-third knife (2026-11-02)

**Slice:** b09 construction tip retain/method/actor heap_slots 三联 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted retain_catch_all + method_call + actor_call inventories×3 + companions. **Kept** mir_run_call_return 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I33 |
|--------|------:|------------:|
| `fn *_inventory` matches | **31** | **−3** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 263 | −45 |

## S2-I35 after thirty-fourth knife (2026-11-02)

**Slice:** b09 construction tip call-return 两联 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted mir_run + vm call_return inventories×2 + companions. **Kept** mutrefcell CallHost 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I34 |
|--------|------:|------------:|
| `fn *_inventory` matches | **29** | **−2** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 236 | −27 |

## S2-I36 after thirty-fifth knife (2026-11-02)

**Slice:** b09 construction tip mutrefcell CallHost 两联 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted mir_run + vm CallHost inventories×2 + companions. **Kept** mutrefcell DictBuild 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I35 |
|--------|------:|------------:|
| `fn *_inventory` matches | **27** | **−2** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 212 | −24 |

## S2-I37 after thirty-sixth knife (2026-11-02)

**Slice:** b09 construction tip mutrefcell DictBuild 两联 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted mir_run_dictbuild_align + vm_dictbuild heap_slots inventories×2 + companions. **Kept** objectref CallHost 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I36 |
|--------|------:|------------:|
| `fn *_inventory` matches | **25** | **−2** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 188 | −24 |

## S2-I38 after thirty-seventh knife (2026-11-02)

**Slice:** b09 construction tip objectref CallHost 两联 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted mir_run + vm CallHost inventories×2 + companions. **Kept** objectref DictBuild 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I37 |
|--------|------:|------------:|
| `fn *_inventory` matches | **23** | **−2** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 164 | −24 |

## S2-I39 after thirty-eighth knife (2026-11-02)

**Slice:** b09 construction tip objectref DictBuild 单枚 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted mir_run_dictbuild_align inventory×1 + companion. **Kept** objectref vm_heap_slots 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I38 |
|--------|------:|------------:|
| `fn *_inventory` matches | **22** | **−1** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 152 | −12 |

## S2-I40 after thirty-ninth knife (2026-11-02)

**Slice:** b09 construction tip objectref vm_heap_slots 单枚 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted vm_heap_slots inventory×1 + companion. **Kept** objectref vm_managed_heap_probe 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I39 |
|--------|------:|------------:|
| `fn *_inventory` matches | **21** | **−1** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 139 | −13 |

## S2-I41 after fortieth knife (2026-11-02)

**Slice:** b09 construction tip objectref vm_managed_heap_probe 单枚 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted managed_heap_probe inventory×1 + companion. **Kept** objectref_rc_migrate 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I40 |
|--------|------:|------------:|
| `fn *_inventory` matches | **20** | **−1** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 129 | −10 |

## S2-I42 after forty-first knife (2026-11-02)

**Slice:** b09 construction tip objectref_rc_migrate 单枚 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted rc_migrate inventory×1 + companion. **Kept** memory_pool_dead_shell 及以下 / 五枚 policy-fixed flag（含 `objectref_rc_migrate_enabled`） / `gc_narrative_thin` / HostFn×8.

| Metric | Count | Δ vs S2-I41 |
|--------|------:|------------:|
| `fn *_inventory` matches | **19** | **−1** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 119 | −10 |

## S2-I43 after forty-second knife (2026-11-02)

**Slice:** b09 construction tip memory_pool_dead_shell 单枚 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted dead_shell inventory×1 + companion. **Kept** storage_cleanup 及以下 / 五枚 policy-fixed flag / `gc_narrative_thin` / HostFn×8.

| Metric | Count | Δ vs S2-I42 |
|--------|------:|------------:|
| `fn *_inventory` matches | **18** | **−1** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 110 | −9 |

## S2-I44 after forty-third knife (2026-11-02)

**Slice:** b09 construction tip storage_cleanup 单枚 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted storage_cleanup inventory×1 + companion. **Kept** enable_readiness 及以下 / 五枚 policy-fixed flag（含 `storage_cleanup_enabled`） / `gc_narrative_thin` / HostFn×8.

| Metric | Count | Δ vs S2-I43 |
|--------|------:|------------:|
| `fn *_inventory` matches | **17** | **−1** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 101 | −9 |

## S2-I45 after forty-fourth knife (2026-11-02)

**Slice:** b09 construction tip 四联 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted enable_readiness + storage_honesty + fully_converged + dual_heap inventories×4 + companions. **Kept** actor_mailbox 及以下 / 五枚 policy-fixed flag / `gc_narrative_thin` / HostFn×8. **Default going forward:** tip packs Δ3–4.

| Metric | Count | Δ vs S2-I44 |
|--------|------:|------------:|
| `fn *_inventory` matches | **13** | **−4** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 67 | −34 |

## S2-I46 after forty-fifth knife (2026-11-03)

**Slice:** b09 construction tip 四联 ([stage2-b09-construction-tip-knife](adr/stage2-b09-construction-tip-knife.md)); deleted actor_mailbox + vm_gccollect + mir_runtime_root_set + gc_narrative_deepen inventories×4 + companions. **Kept** `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag.

| Metric | Count | Δ vs S2-I45 |
|--------|------:|------------:|
| `fn *_inventory` matches | **9** | **−4** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 34 | −33 |

## S2-I47 after forty-sixth knife (2026-11-03)

**Slice:** HostFn construction tip + orphan thin ([stage2-hostfn-construction-tip-knife](adr/stage2-hostfn-construction-tip-knife.md)); deleted `gc_narrative_thin` + HostFn exit + fifth + fourth inventories×4 + companions (`tcp_handle_release_wire`, `tcp_write_wire`). **Kept** HostFn×5 (third→thin) / 五枚 policy-fixed flag / `llvm_stub_audit_report`.

| Metric | Count | Δ vs S2-I46 |
|--------|------:|------------:|
| `fn *_inventory` matches | **5** | **−4** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 16 | −18 |

## S2-I48 after forty-seventh knife (2026-11-03)

**Slice:** HostFn construction tip 四联 ([stage2-hostfn-construction-tip-knife](adr/stage2-hostfn-construction-tip-knife.md)); deleted third + second + first + deepen inventories×4 + companions (`tcp_read_wire`, `file_read_wire`, `gate_b07_…_wire`). **Kept** HostFn `thin` / 五枚 policy-fixed flag / `llvm_stub_audit_report`.

| Metric | Count | Δ vs S2-I47 |
|--------|------:|------------:|
| `fn *_inventory` matches | **1** | **−4** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 2 | −14 |

## S2-I49 after forty-eighth knife (2026-11-03)

**Slice:** HostFn thin 末刀 ([stage2-hostfn-construction-tip-knife](adr/stage2-hostfn-construction-tip-knife.md)); deleted `gate_hostfn_real_lowering_horizon_thin_inventory`×1. **Kept** `llvm_stub_audit_report` / `classify_llvm_stub_block` / 五枚 policy-fixed flag. Census script hardened for zero-match `rg` (pipefail). **inventory KPI = 0 ≠ S2-01 `[x]`** (fingerprint/report remain).

| Metric | Count | Δ vs S2-I48 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | **−1** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | −2 |

## S2-I50 after forty-ninth knife (2026-11-03)

**Slice:** doctor-report tip 四联 ([stage2-doctor-report-tip-knife](adr/stage2-doctor-report-tip-knife.md)); deleted actor_ipc + dict_runtime + dist_failover + rt_v7 reports×4 + CLI flags. **Kept** `capabilities_report` / remaining reports / fingerprint / 五枚 flag / `llvm_stub_audit_report`. inventory still **0**.

| Metric | Count | Δ vs S2-I49 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | **22** | **−4** |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I51 after fiftieth knife (2026-11-03)

**Slice:** doctor-report tip 四联 ([stage2-doctor-report-tip-knife](adr/stage2-doctor-report-tip-knife.md)); deleted dist_actor_cluster + heap_bridge_prod + heapslot_parity + native_sdk reports×4 + CLI flags. **Kept** `capabilities_report` / remaining reports / fingerprint / 五枚 flag. inventory still **0**.

| Metric | Count | Δ vs S2-I50 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | **18** | **−4** |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I52 after fifty-first knife (2026-11-03)

**Slice:** doctor-report tip 四联 ([stage2-doctor-report-tip-knife](adr/stage2-doctor-report-tip-knife.md)); deleted gc + native_exec_op + inkwell + gc_generational reports×4 + CLI flags (`--gc-gen-report` alias) + LSP phase73/74/75/80 nails + driver `p98_gc_generational_doctor_alias`. **Kept** `capabilities_report` / `heap_bridge_report` / remaining coupled reports. inventory still **0**.

| Metric | Count | Δ vs S2-I51 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | **14** | **−4** |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I53 after fifty-second knife (2026-11-03)

**Slice:** doctor-report LSP-nail 四联 ([stage2-doctor-report-lsp-nail-knife](adr/stage2-doctor-report-lsp-nail-knife.md)); deleted cache_remote + rt_exec + heap_host + farm reports×4 + CLI flags + LSP phase76/77 module/CLI nails. **Kept** `capabilities_report` / `heap_bridge_report` / remaining coupled reports. inventory still **0**.

| Metric | Count | Δ vs S2-I52 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | **10** | **−4** |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I54 after fifty-third knife (2026-11-03)

**Slice:** tip 末枚 + doctor-report driver-src-nail 三联 ([stage2-doctor-report-driver-src-nail-knife](adr/stage2-doctor-report-driver-src-nail-knife.md)); deleted heap_bridge + native_profile + farm_billing + dist_cdn reports×4 + CLI flags + LSP phase80/81 nails + driver include_str nails. **Kept** `capabilities_report` / `rt_runtime_report` / remaining `run_*` coupled reports. inventory still **0**.

| Metric | Count | Δ vs S2-I53 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | **6** | **−4** |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I55 after fifty-fourth knife (2026-11-03)

**Slice:** LSP 末枚 + doctor-report run-test-nail 三联 ([stage2-doctor-report-run-test-nail-knife](adr/stage2-doctor-report-run-test-nail-knife.md)); deleted rt_runtime + gc_relocate + reflect + hsm_rotation reports×4 + CLI flags + LSP phase77/78/79 nails + driver `run_*` nails retargeted. **Kept** `capabilities_report` / `gui_backend_report`. inventory still **0**.

| Metric | Count | Δ vs S2-I54 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | **2** | **−4** |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I56 after fifty-fifth knife (2026-11-03)

**Slice:** doctor-report run-test-nail 末枚 ([stage2-doctor-report-run-test-nail-knife](adr/stage2-doctor-report-run-test-nail-knife.md)); deleted `gui_backend_report` + CLI flag + GUI/driver nails retargeted to `syx_vm::gui_*`. **Kept** `capabilities_report`. inventory still **0**.

| Metric | Count | Δ vs S2-I55 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | **1** | **−1** |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I57 after fifty-sixth knife (2026-11-03)

**Slice:** fingerprint orphan tip 四联 ([stage2-fingerprint-tip-knife](adr/stage2-fingerprint-tip-knife.md)); deleted `heap_v90_phase118` track a–d `*_fingerprint`×4 (zero readers; not on cache aggregator). **Kept** `heap_v90_phase118_dx_fingerprint` / aggregator chain / `capabilities_report`. inventory still **0**.

| Metric | Count | Δ vs S2-I56 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **651** | **−4** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I58 after fifty-seventh knife (2026-11-03)

**Slice:** fingerprint tip 四联 + nail co-delete ([stage2-fingerprint-tip-knife](adr/stage2-fingerprint-tip-knife.md)); deleted `heap_v76_phase104` track a–d `*_fingerprint`×4; narrowed `p104_driver_heap_v76_fingerprint` to `dx` only. **Kept** `heap_v76_phase104_dx_fingerprint` / `heap_v90_phase118_dx_fingerprint` / aggregator chain / `capabilities_report`. inventory still **0**.

| Metric | Count | Δ vs S2-I57 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **647** | **−4** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## Non-claims

- **≠** S2-01 `[x]` / Stage-2 exit.
- Re-run the script after each S2-01 migration slice; update this table when materially changed.
