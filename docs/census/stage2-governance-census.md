# Stage-2 governance census (S2-01)

**ADR:** [stage2-open](../adr/stage2-open.md)  
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

**Slice:** cutover product decouple ([ADR](../adr/stage2-cutover-decouple.md)): `wave_b` hard-codes `relational_b04/b05/b06`; deleted 3 rollups (`aot_short_horizon_cutover_readiness` / `mir_module_medium_horizon_cutover_readiness` / `llvm_stub_clear_align_horizon_readiness`). **Kept** feed chains, HostFn×8, `capability_align`, `gate_b09`, `absolute_threshold`, `wave_b_tri_axis_*` / structural.

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

**Slice:** retired wave_b tri+structural readiness inventories ([stage2-wave-b-inventory-retire](../adr/stage2-wave-b-inventory-retire.md)) + companion thin tests. **Kept** `gate_b09` / HostFn×8 / `capability_align`; W340 exit charter evidence.

| Metric | Count | Δ vs S2-I15 |
|--------|------:|------------:|
| `fn *_inventory` matches | **83** | **−2** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 1013 | −17 |

## S2-I17 after sixteenth knife (2026-11-02)

**Slice:** HostFn←capability_align decouple ([stage2-hostfn-align-decouple](../adr/stage2-hostfn-align-decouple.md)); deleted align inventory + W314 companion. **Kept** HostFn×8 / `audit_report` / `gate_b09`.

| Metric | Count | Δ vs S2-I16 |
|--------|------:|------------:|
| `fn *_inventory` matches | **82** | **−1** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 1013 | 0 |

## S2-I18 after seventeenth knife (2026-11-02)

**Slice:** b09 product-flag tip decouple ([stage2-b09-product-flag-decouple](../adr/stage2-b09-product-flag-decouple.md)); five flags policy-fixed `true`; deleted tip product inventories×4 + companions. **Kept** b09 construction chain / HostFn×8 / flag API names.

| Metric | Count | Δ vs S2-I17 |
|--------|------:|------------:|
| `fn *_inventory` matches | **78** | **−4** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 953 | −60 |

## S2-I19 after eighteenth knife (2026-11-02)

**Slice:** b09 construction tip peel 两联 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted mutrefcell+reentry peel inventories×6 + companions. **Kept** dual_track 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I18 |
|--------|------:|------------:|
| `fn *_inventory` matches | **72** | **−6** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 863 | −90 |

## S2-I20 after nineteenth knife (2026-11-02)

**Slice:** b09 construction tip dual_track+mirop 两联 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted dual_track collapse + mirop residual peel inventories×6 + companions. **Kept** objectref peel 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I19 |
|--------|------:|------------:|
| `fn *_inventory` matches | **66** | **−6** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 769 | −94 |

## S2-I21 after twentieth knife (2026-11-02)

**Slice:** b09 construction tip objectref peel 三联 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted peel readiness/delete/post_peel_refresh inventories×3 + companion. **Kept** deep_field 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I20 |
|--------|------:|------------:|
| `fn *_inventory` matches | **63** | **−3** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 714 | −55 |

## S2-I22 after twenty-first knife (2026-11-02)

**Slice:** b09 construction tip deep_field+strip_rc 两联 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted deep_field + strip_rc inventories×4 + companions. **Kept** handle 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I21 |
|--------|------:|------------:|
| `fn *_inventory` matches | **59** | **−4** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 653 | −61 |

## S2-I23 after twenty-second knife (2026-11-02)

**Slice:** b09 construction tip handle 三联 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted post_handle_refresh + handle delete + handle retire inventories×3 + companion. **Kept** nested_field 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I22 |
|--------|------:|------------:|
| `fn *_inventory` matches | **56** | **−3** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 604 | −49 |

## S2-I24 after twenty-third knife (2026-11-02)

**Slice:** b09 construction tip nested_field ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted nested_field_dual_track_first_slice inventory×1 + companion. **Kept** refresh_thin 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I23 |
|--------|------:|------------:|
| `fn *_inventory` matches | **55** | **−1** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 589 | −15 |

## S2-I25 after twenty-fourth knife (2026-11-02)

**Slice:** b09 construction tip refresh+wrap+value 三联 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted refresh_thin + wrap delete + value_rc_api delete inventories×3 + companions. **Kept** storelocal_deinits 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I24 |
|--------|------:|------------:|
| `fn *_inventory` matches | **52** | **−3** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 550 | −39 |

## S2-I26 after twenty-fifth knife (2026-11-02)

**Slice:** b09 construction tip death-path 四联 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted storelocal_deinits + gc_type_deinit + deinits/storelocal heap_unroot inventories×4 + companions. **Kept** rc_release 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I25 |
|--------|------:|------------:|
| `fn *_inventory` matches | **48** | **−4** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 496 | −54 |

## S2-I27 after twenty-sixth knife (2026-11-02)

**Slice:** b09 construction tip opcode retire 三联 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted rc_release_opcode + rc_dup_loadlocal + rc_retain_opcode retire first-slice inventories×3 + companions. **Kept** rc_retire_readiness 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I26 |
|--------|------:|------------:|
| `fn *_inventory` matches | **45** | **−3** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 455 | −41 |

## S2-I28 after twenty-seventh knife (2026-11-02)

**Slice:** b09 construction tip retire-readiness 三联 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted rc_retire_readiness + rt_abi retain_release/wrap retire readiness inventories×3. **Kept** rc_migrate_readiness 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I27 |
|--------|------:|------------:|
| `fn *_inventory` matches | **42** | **−3** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 417 | −38 |

## S2-I29 after twenty-eighth knife (2026-11-02)

**Slice:** b09 construction tip migrate_readiness 单枚 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted rc_migrate_readiness_thin inventory×1. **Kept** ssa_loadlocal heap_slots 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I28 |
|--------|------:|------------:|
| `fn *_inventory` matches | **41** | **−1** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 403 | −14 |

## S2-I30 after twenty-ninth knife (2026-11-02)

**Slice:** b09 construction tip heap_slots 三联 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted ssa_loadlocal + loadglobal + objectset heap_slots inventories×3 + companions. **Kept** llvm wrap emit 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I29 |
|--------|------:|------------:|
| `fn *_inventory` matches | **38** | **−3** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 364 | −39 |

## S2-I31 after thirtieth knife (2026-11-02)

**Slice:** b09 construction tip llvm wrap emit 单枚 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted llvm_objectref_wrap_emit_retire inventory×1 + companion. **Kept** arrayget heap_slots 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I30 |
|--------|------:|------------:|
| `fn *_inventory` matches | **37** | **−1** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 350 | −14 |

## S2-I32 after thirty-first knife (2026-11-02)

**Slice:** b09 construction tip arrayget+residual_birth heap_slots 两联 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted arrayget + residual_birth inventories×2 + companions. **Kept** llvm_wasm emit retire 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I31 |
|--------|------:|------------:|
| `fn *_inventory` matches | **35** | **−2** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 322 | −28 |

## S2-I33 after thirty-second knife (2026-11-02)

**Slice:** b09 construction tip llvm_wasm emit-retire 单枚 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted llvm_wasm_objectref_rc_emit_retire inventory×1 + companion. **Kept** retain_catch_all heap_slots 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I32 |
|--------|------:|------------:|
| `fn *_inventory` matches | **34** | **−1** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 308 | −14 |

## S2-I34 after thirty-third knife (2026-11-02)

**Slice:** b09 construction tip retain/method/actor heap_slots 三联 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted retain_catch_all + method_call + actor_call inventories×3 + companions. **Kept** mir_run_call_return 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I33 |
|--------|------:|------------:|
| `fn *_inventory` matches | **31** | **−3** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 263 | −45 |

## S2-I35 after thirty-fourth knife (2026-11-02)

**Slice:** b09 construction tip call-return 两联 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted mir_run + vm call_return inventories×2 + companions. **Kept** mutrefcell CallHost 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I34 |
|--------|------:|------------:|
| `fn *_inventory` matches | **29** | **−2** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 236 | −27 |

## S2-I36 after thirty-fifth knife (2026-11-02)

**Slice:** b09 construction tip mutrefcell CallHost 两联 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted mir_run + vm CallHost inventories×2 + companions. **Kept** mutrefcell DictBuild 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I35 |
|--------|------:|------------:|
| `fn *_inventory` matches | **27** | **−2** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 212 | −24 |

## S2-I37 after thirty-sixth knife (2026-11-02)

**Slice:** b09 construction tip mutrefcell DictBuild 两联 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted mir_run_dictbuild_align + vm_dictbuild heap_slots inventories×2 + companions. **Kept** objectref CallHost 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I36 |
|--------|------:|------------:|
| `fn *_inventory` matches | **25** | **−2** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 188 | −24 |

## S2-I38 after thirty-seventh knife (2026-11-02)

**Slice:** b09 construction tip objectref CallHost 两联 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted mir_run + vm CallHost inventories×2 + companions. **Kept** objectref DictBuild 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I37 |
|--------|------:|------------:|
| `fn *_inventory` matches | **23** | **−2** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 164 | −24 |

## S2-I39 after thirty-eighth knife (2026-11-02)

**Slice:** b09 construction tip objectref DictBuild 单枚 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted mir_run_dictbuild_align inventory×1 + companion. **Kept** objectref vm_heap_slots 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I38 |
|--------|------:|------------:|
| `fn *_inventory` matches | **22** | **−1** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 152 | −12 |

## S2-I40 after thirty-ninth knife (2026-11-02)

**Slice:** b09 construction tip objectref vm_heap_slots 单枚 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted vm_heap_slots inventory×1 + companion. **Kept** objectref vm_managed_heap_probe 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I39 |
|--------|------:|------------:|
| `fn *_inventory` matches | **21** | **−1** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 139 | −13 |

## S2-I41 after fortieth knife (2026-11-02)

**Slice:** b09 construction tip objectref vm_managed_heap_probe 单枚 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted managed_heap_probe inventory×1 + companion. **Kept** objectref_rc_migrate 及以下 / `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag API.

| Metric | Count | Δ vs S2-I40 |
|--------|------:|------------:|
| `fn *_inventory` matches | **20** | **−1** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 129 | −10 |

## S2-I42 after forty-first knife (2026-11-02)

**Slice:** b09 construction tip objectref_rc_migrate 单枚 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted rc_migrate inventory×1 + companion. **Kept** memory_pool_dead_shell 及以下 / 五枚 policy-fixed flag（含 `objectref_rc_migrate_enabled`） / `gc_narrative_thin` / HostFn×8.

| Metric | Count | Δ vs S2-I41 |
|--------|------:|------------:|
| `fn *_inventory` matches | **19** | **−1** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 119 | −10 |

## S2-I43 after forty-second knife (2026-11-02)

**Slice:** b09 construction tip memory_pool_dead_shell 单枚 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted dead_shell inventory×1 + companion. **Kept** storage_cleanup 及以下 / 五枚 policy-fixed flag / `gc_narrative_thin` / HostFn×8.

| Metric | Count | Δ vs S2-I42 |
|--------|------:|------------:|
| `fn *_inventory` matches | **18** | **−1** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 110 | −9 |

## S2-I44 after forty-third knife (2026-11-02)

**Slice:** b09 construction tip storage_cleanup 单枚 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted storage_cleanup inventory×1 + companion. **Kept** enable_readiness 及以下 / 五枚 policy-fixed flag（含 `storage_cleanup_enabled`） / `gc_narrative_thin` / HostFn×8.

| Metric | Count | Δ vs S2-I43 |
|--------|------:|------------:|
| `fn *_inventory` matches | **17** | **−1** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 101 | −9 |

## S2-I45 after forty-fourth knife (2026-11-02)

**Slice:** b09 construction tip 四联 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted enable_readiness + storage_honesty + fully_converged + dual_heap inventories×4 + companions. **Kept** actor_mailbox 及以下 / 五枚 policy-fixed flag / `gc_narrative_thin` / HostFn×8. **Default going forward:** tip packs Δ3–4.

| Metric | Count | Δ vs S2-I44 |
|--------|------:|------------:|
| `fn *_inventory` matches | **13** | **−4** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 67 | −34 |

## S2-I46 after forty-fifth knife (2026-11-03)

**Slice:** b09 construction tip 四联 ([stage2-b09-construction-tip-knife](../adr/stage2-b09-construction-tip-knife.md)); deleted actor_mailbox + vm_gccollect + mir_runtime_root_set + gc_narrative_deepen inventories×4 + companions. **Kept** `gc_narrative_thin` / HostFn×8 / 五枚 policy-fixed flag.

| Metric | Count | Δ vs S2-I45 |
|--------|------:|------------:|
| `fn *_inventory` matches | **9** | **−4** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 34 | −33 |

## S2-I47 after forty-sixth knife (2026-11-03)

**Slice:** HostFn construction tip + orphan thin ([stage2-hostfn-construction-tip-knife](../adr/stage2-hostfn-construction-tip-knife.md)); deleted `gc_narrative_thin` + HostFn exit + fifth + fourth inventories×4 + companions (`tcp_handle_release_wire`, `tcp_write_wire`). **Kept** HostFn×5 (third→thin) / 五枚 policy-fixed flag / `llvm_stub_audit_report`.

| Metric | Count | Δ vs S2-I46 |
|--------|------:|------------:|
| `fn *_inventory` matches | **5** | **−4** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 16 | −18 |

## S2-I48 after forty-seventh knife (2026-11-03)

**Slice:** HostFn construction tip 四联 ([stage2-hostfn-construction-tip-knife](../adr/stage2-hostfn-construction-tip-knife.md)); deleted third + second + first + deepen inventories×4 + companions (`tcp_read_wire`, `file_read_wire`, `gate_b07_…_wire`). **Kept** HostFn `thin` / 五枚 policy-fixed flag / `llvm_stub_audit_report`.

| Metric | Count | Δ vs S2-I47 |
|--------|------:|------------:|
| `fn *_inventory` matches | **1** | **−4** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 2 | −14 |

## S2-I49 after forty-eighth knife (2026-11-03)

**Slice:** HostFn thin 末刀 ([stage2-hostfn-construction-tip-knife](../adr/stage2-hostfn-construction-tip-knife.md)); deleted `gate_hostfn_real_lowering_horizon_thin_inventory`×1. **Kept** `llvm_stub_audit_report` / `classify_llvm_stub_block` / 五枚 policy-fixed flag. Census script hardened for zero-match `rg` (pipefail). **inventory KPI = 0 ≠ S2-01 `[x]`** (fingerprint/report remain).

| Metric | Count | Δ vs S2-I48 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | **−1** |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | 26 | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | −2 |

## S2-I50 after forty-ninth knife (2026-11-03)

**Slice:** doctor-report tip 四联 ([stage2-doctor-report-tip-knife](../adr/stage2-doctor-report-tip-knife.md)); deleted actor_ipc + dict_runtime + dist_failover + rt_v7 reports×4 + CLI flags. **Kept** `capabilities_report` / remaining reports / fingerprint / 五枚 flag / `llvm_stub_audit_report`. inventory still **0**.

| Metric | Count | Δ vs S2-I49 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | **22** | **−4** |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I51 after fiftieth knife (2026-11-03)

**Slice:** doctor-report tip 四联 ([stage2-doctor-report-tip-knife](../adr/stage2-doctor-report-tip-knife.md)); deleted dist_actor_cluster + heap_bridge_prod + heapslot_parity + native_sdk reports×4 + CLI flags. **Kept** `capabilities_report` / remaining reports / fingerprint / 五枚 flag. inventory still **0**.

| Metric | Count | Δ vs S2-I50 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | **18** | **−4** |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I52 after fifty-first knife (2026-11-03)

**Slice:** doctor-report tip 四联 ([stage2-doctor-report-tip-knife](../adr/stage2-doctor-report-tip-knife.md)); deleted gc + native_exec_op + inkwell + gc_generational reports×4 + CLI flags (`--gc-gen-report` alias) + LSP phase73/74/75/80 nails + driver `p98_gc_generational_doctor_alias`. **Kept** `capabilities_report` / `heap_bridge_report` / remaining coupled reports. inventory still **0**.

| Metric | Count | Δ vs S2-I51 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | **14** | **−4** |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I53 after fifty-second knife (2026-11-03)

**Slice:** doctor-report LSP-nail 四联 ([stage2-doctor-report-lsp-nail-knife](../adr/stage2-doctor-report-lsp-nail-knife.md)); deleted cache_remote + rt_exec + heap_host + farm reports×4 + CLI flags + LSP phase76/77 module/CLI nails. **Kept** `capabilities_report` / `heap_bridge_report` / remaining coupled reports. inventory still **0**.

| Metric | Count | Δ vs S2-I52 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | **10** | **−4** |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I54 after fifty-third knife (2026-11-03)

**Slice:** tip 末枚 + doctor-report driver-src-nail 三联 ([stage2-doctor-report-driver-src-nail-knife](../adr/stage2-doctor-report-driver-src-nail-knife.md)); deleted heap_bridge + native_profile + farm_billing + dist_cdn reports×4 + CLI flags + LSP phase80/81 nails + driver include_str nails. **Kept** `capabilities_report` / `rt_runtime_report` / remaining `run_*` coupled reports. inventory still **0**.

| Metric | Count | Δ vs S2-I53 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | **6** | **−4** |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I55 after fifty-fourth knife (2026-11-03)

**Slice:** LSP 末枚 + doctor-report run-test-nail 三联 ([stage2-doctor-report-run-test-nail-knife](../adr/stage2-doctor-report-run-test-nail-knife.md)); deleted rt_runtime + gc_relocate + reflect + hsm_rotation reports×4 + CLI flags + LSP phase77/78/79 nails + driver `run_*` nails retargeted. **Kept** `capabilities_report` / `gui_backend_report`. inventory still **0**.

| Metric | Count | Δ vs S2-I54 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | **2** | **−4** |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I56 after fifty-fifth knife (2026-11-03)

**Slice:** doctor-report run-test-nail 末枚 ([stage2-doctor-report-run-test-nail-knife](../adr/stage2-doctor-report-run-test-nail-knife.md)); deleted `gui_backend_report` + CLI flag + GUI/driver nails retargeted to `syx_vm::gui_*`. **Kept** `capabilities_report`. inventory still **0**.

| Metric | Count | Δ vs S2-I55 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | 655 | 0 |
| `crates/syx-driver/src/*_report.rs` files | **1** | **−1** |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I57 after fifty-sixth knife (2026-11-03)

**Slice:** fingerprint orphan tip 四联 ([stage2-fingerprint-tip-knife](../adr/stage2-fingerprint-tip-knife.md)); deleted `heap_v90_phase118` track a–d `*_fingerprint`×4 (zero readers; not on cache aggregator). **Kept** `heap_v90_phase118_dx_fingerprint` / aggregator chain / `capabilities_report`. inventory still **0**.

| Metric | Count | Δ vs S2-I56 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **651** | **−4** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I58 after fifty-seventh knife (2026-11-03)

**Slice:** fingerprint tip 四联 + nail co-delete ([stage2-fingerprint-tip-knife](../adr/stage2-fingerprint-tip-knife.md)); deleted `heap_v76_phase104` track a–d `*_fingerprint`×4; narrowed `p104_driver_heap_v76_fingerprint` to `dx` only. **Kept** `heap_v76_phase104_dx_fingerprint` / `heap_v90_phase118_dx_fingerprint` / aggregator chain / `capabilities_report`. inventory still **0**.

| Metric | Count | Δ vs S2-I57 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **647** | **−4** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I59 after fifty-eighth knife (2026-11-03)

**Slice:** fingerprint census test-noise tip×31 ([stage2-fingerprint-tip-knife](../adr/stage2-fingerprint-tip-knife.md)); deleted `#[test] fn *_fingerprint` in `syx-lsp` `lsp_phase86_real`…`lsp_phase90_real` (Δ31; default test-noise pack Δ16–32). **Kept** cache aggregator / `capabilities_report` / driver product fingerprints. inventory still **0**.

| Metric | Count | Δ vs S2-I58 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **616** | **−31** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I60 after fifty-ninth knife (2026-11-03)

**Slice:** fingerprint census test-noise tip×34 ([stage2-fingerprint-tip-knife](../adr/stage2-fingerprint-tip-knife.md)); deleted `#[test] fn *_fingerprint` in `syx-lsp` `lsp_phase91_real`…`lsp_phase95_real`. **Kept** cache aggregator / `capabilities_report` / driver product fingerprints. inventory still **0**.

| Metric | Count | Δ vs S2-I59 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **582** | **−34** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I61 after sixtieth knife (2026-11-03)

**Slice:** fingerprint census test-noise tip×29 ([stage2-fingerprint-tip-knife](../adr/stage2-fingerprint-tip-knife.md)); deleted `#[test] fn *_fingerprint` in `syx-lsp` `lsp_phase96_real`…`lsp_phase100_real`. **Kept** cache aggregator / `capabilities_report` / phase101+ leftovers. inventory still **0**.

| Metric | Count | Δ vs S2-I60 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **553** | **−29** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I62 after sixty-first knife (2026-11-03)

**Slice:** fingerprint census test-noise tip×24 ([stage2-fingerprint-tip-knife](../adr/stage2-fingerprint-tip-knife.md)); deleted remaining `#[test] fn *_fingerprint` in `syx-lsp` phase79–85 / 101–103 / 131–139. **LSP `lsp_phase*_real.rs` exact `*_fingerprint` test-noise axis closed.** **Kept** driver `tests/**` fingerprint tests / cache aggregator / `capabilities_report`. inventory still **0**.

| Metric | Count | Δ vs S2-I61 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **529** | **−24** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I63 after sixty-second knife (2026-11-03)

**Slice:** fingerprint census test-noise tip×29 ([stage2-fingerprint-tip-knife](../adr/stage2-fingerprint-tip-knife.md)); deleted `#[test] fn *_fingerprint` in driver `driver_gate.rs`×16 + `property_observers.rs`×8 + `native_gui_parity.rs`×5. **Driver tests fingerprint test-noise axis opened.** **Kept** remaining driver test nails / cache aggregator / `capabilities_report`. inventory still **0**.

| Metric | Count | Δ vs S2-I62 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **500** | **−29** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I64 after sixty-third knife (2026-11-03)

**Slice:** fingerprint census test-noise tip×30 ([stage2-fingerprint-tip-knife](../adr/stage2-fingerprint-tip-knife.md)); deleted `#[test] fn *_fingerprint` in driver `tests/farm`×18 + `tests/gui`×7 + `tests/wasm`×5. **Kept** platform/runtime/data/native/concurrency nails / cache aggregator / `capabilities_report` / product `farm_lease_*`. inventory still **0**.

| Metric | Count | Δ vs S2-I63 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **470** | **−30** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I65 after sixty-fourth knife (2026-11-03)

**Slice:** fingerprint census test-noise tip×29 ([stage2-fingerprint-tip-knife](../adr/stage2-fingerprint-tip-knife.md)); deleted `#[test] fn *_fingerprint` in driver `tests/runtime`×15 + `tests/data`×14. **Kept** platform/native/concurrency nails / cache aggregator / `capabilities_report` / product `farm_lease_*`. inventory still **0**.

| Metric | Count | Δ vs S2-I64 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **441** | **−29** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I66 after sixty-fifth knife (2026-11-03)

**Slice:** fingerprint census test-noise tip×36 ([stage2-fingerprint-tip-knife](../adr/stage2-fingerprint-tip-knife.md)); deleted `#[test] fn *_fingerprint` in driver `tests/platform`×36. **Kept** native/concurrency nails / cache aggregator / `capabilities_report` / product `farm_lease_*`. inventory still **0**.

| Metric | Count | Δ vs S2-I65 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **405** | **−36** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I67 after sixty-sixth knife (2026-11-03)

**Slice:** fingerprint census test-noise tip×22 ([stage2-fingerprint-tip-knife](../adr/stage2-fingerprint-tip-knife.md)); deleted `#[test] fn *_fingerprint` in driver `tests/native`×13 + `tests/concurrency`×9. **Driver `tests/**` fingerprint test-noise axis closed.** **Kept** cache aggregator / `capabilities_report` / product `farm_lease_*` / non-driver nails. inventory still **0**.

| Metric | Count | Δ vs S2-I66 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **383** | **−22** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I68 after sixty-seventh knife (2026-11-03)

**Slice:** fingerprint tip orphan+sparse×5 ([stage2-fingerprint-tip-knife](../adr/stage2-fingerprint-tip-knife.md)); deleted product orphans×3 (`heap_v75_phase103_track_a` / `heap_v76_phase104_dx` / `heap_v90_phase118_dx`) + non-driver `#[test] *_fingerprint`×2 (`pipeline_order` / `p120_macros_exports_schema`). **Tip-knife orphan axis closed.** Aggregator unchanged (fold deferred). inventory still **0**.

| Metric | Count | Δ vs S2-I67 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **378** | **−5** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I69 after sixty-eighth knife (2026-11-03)

**Slice:** fingerprint aggregator fold Δ6 ([stage2-fingerprint-aggregator-fold](../adr/stage2-fingerprint-aggregator-fold.md)); merged `heap_v3`/`v4`/`v48`–`v51` hasher bytes into `heap_v52_phase80_fingerprint`; deleted ×6 defs + aligned `compute_cache_version_tag` `format!`. **Kept** soft-nail band (v58+) / `heap_v111_*` / `farm_lease_*` / `capabilities_report`. inventory still **0**.

| Metric | Count | Δ vs S2-I68 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **372** | **−6** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I70 after sixty-ninth knife (2026-11-03)

**Slice:** fingerprint aggregator fold Δ16 ([stage2-fingerprint-aggregator-fold](../adr/stage2-fingerprint-aggregator-fold.md)); merged `heap_v53`…`v68` hasher statements into `heap_v69_phase97_fingerprint`; deleted ×16 defs + aligned `format!`; retargeted `driver_gate` soft-nail v58→v69. **Kept** `heap_v52` / `heap_v70+` / `heap_v111_*` / `farm_lease_*` / `capabilities_report`. inventory still **0**.

| Metric | Count | Δ vs S2-I69 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **356** | **−16** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I71 after seventieth knife (2026-11-03)

**Slice:** fingerprint aggregator fold Δ16 ([stage2-fingerprint-aggregator-fold](../adr/stage2-fingerprint-aggregator-fold.md)); merged `heap_v70`…`v75` + `v77`/`v78` tracks+dx into `heap_v79_phase107_track_a_gui_fingerprint`; deleted ×16 + aligned `format!`; retargeted soft-nails (driver_gate×5 + lsp×1). **Kept** `heap_v52`/`v69` / `heap_v79` other tracks+dx / `heap_v111_*` / `farm_lease_*` defs / `capabilities_report`. inventory still **0**.

| Metric | Count | Δ vs S2-I70 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **340** | **−16** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I72 after seventy-first knife (2026-11-03)

**Slice:** fingerprint aggregator fold Δ16 ([stage2-fingerprint-aggregator-fold](../adr/stage2-fingerprint-aggregator-fold.md)); merged `heap_v79` b/c/d/dx + `v80`/`v81` a…dx + `v82` a/b into `heap_v82_phase110_track_c_farm_fingerprint`; deleted ×16 + aligned `format!`. Soft-nail-free band. **Kept** `heap_v52`/`v69`/`v79_track_a` / `v82` track_d+dx / `v83+` / `heap_v111_*` / `farm_lease_*` / `capabilities_report`. inventory still **0**.

| Metric | Count | Δ vs S2-I71 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **324** | **−16** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I73 after seventy-second knife (2026-11-04)

**Slice:** fingerprint aggregator fold Δ16 ([stage2-fingerprint-aggregator-fold](../adr/stage2-fingerprint-aggregator-fold.md)); merged `heap_v82` d+dx + `v83`/`v84` a…dx + `v85` a–d into `heap_v85_phase113_dx_fingerprint`; deleted ×16 + aligned `format!`. Soft-nail-free band. **Kept** `heap_v52`/`v69`/`v79_track_a`/`v82_track_c` / `v86+` / `heap_v111_*` / `farm_lease_*` / `capabilities_report`. inventory still **0**.

| Metric | Count | Δ vs S2-I72 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **308** | **−16** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I74 after seventy-third knife (2026-11-04)

**Slice:** fingerprint aggregator fold Δ16 ([stage2-fingerprint-aggregator-fold](../adr/stage2-fingerprint-aggregator-fold.md)); merged `heap_v86`/`v87`/`v88` a…dx + `v89_track_a` into `heap_v89_phase117_track_b_wasm_fingerprint`; deleted ×16 + aligned `format!`. Soft-nail-free band. **Kept** `heap_v52`/`v69`/`v79_track_a`/`v82_track_c`/`v85_dx` / `v89` c/d/dx / `v90+` / `heap_v111_*` / `farm_lease_*` / `capabilities_report`. inventory still **0**.

| Metric | Count | Δ vs S2-I73 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **292** | **−16** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I75 after seventy-fourth knife (2026-11-04)

**Slice:** fingerprint aggregator fold Δ16 ([stage2-fingerprint-aggregator-fold](../adr/stage2-fingerprint-aggregator-fold.md)); merged `heap_v89` c/d/dx + `v91`/`v92` a…dx + `v93` a/b/c into `heap_v93_phase121_track_d_farm_dist_fingerprint`; deleted ×16 + aligned `format!`; soft-nail retarget (`evergreen/smoke` v91_b → merge target; `service_di_mvp` v92 prefix → merge target). **Kept** `heap_v52`/`v69`/`v79_track_a`/`v82_track_c`/`v85_dx`/`v89_track_b` / `v93_dx` / `v94+` / `heap_v111_*` / `farm_lease_*` / `capabilities_report`. inventory still **0**.

| Metric | Count | Δ vs S2-I74 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **276** | **−16** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I76 after seventy-fifth knife (2026-11-04)

**Slice:** fingerprint aggregator fold Δ16 ([stage2-fingerprint-aggregator-fold](../adr/stage2-fingerprint-aggregator-fold.md)); merged `heap_v93_dx` + `v94`/`v95`/`v96` a…dx into `heap_v97_phase125_track_a_macro_service_fingerprint`; deleted ×16 + aligned `format!`. Soft-nail-free band. **Kept** `heap_v52`/`v69`/`v79_track_a`/`v82_track_c`/`v85_dx`/`v89_track_b`/`v93_track_d` / `v97` b/c/d/dx / `v98+` / `heap_v111_*` / `farm_lease_*` / `capabilities_report`. inventory still **0**.

| Metric | Count | Δ vs S2-I75 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **260** | **−16** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I77 after seventy-sixth knife (2026-11-04)

**Slice:** fingerprint aggregator fold Δ16 ([stage2-fingerprint-aggregator-fold](../adr/stage2-fingerprint-aggregator-fold.md)); merged `heap_v97` b…dx + `v98`/`v99` a…dx + `v100` a/b into `heap_v100_phase128_track_c_wasm_fingerprint`; deleted ×16 + aligned `format!`. Soft-nail-free band. **Kept** `heap_v52`/`v69`/`v79_track_a`/`v82_track_c`/`v85_dx`/`v89_track_b`/`v93_track_d`/`v97_track_a` / `v100` d/dx / `v101+` / `heap_v111_*` / `farm_lease_*` / `capabilities_report`. inventory still **0**.

| Metric | Count | Δ vs S2-I76 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **244** | **−16** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I78 after seventy-seventh knife (2026-11-04)

**Slice:** fingerprint aggregator fold Δ16 ([stage2-fingerprint-aggregator-fold](../adr/stage2-fingerprint-aggregator-fold.md)); merged `heap_v100` d/dx + `v101`/`v102` a…dx + `v104` a–d into `heap_v104_phase132_dx_fingerprint` (agg order); deleted ×16 + aligned `format!`; soft-nail retarget (`codable_associated_enum_e2e` v101_a → merge target; `token_expr_mvp` v102_a → merge target). **Kept** `heap_v52`/`v69`/`v79_track_a`/`v82_track_c`/`v85_dx`/`v89_track_b`/`v93_track_d`/`v97_track_a`/`v100_track_c` / `v103` / `v105+` / `heap_v111_*` / `farm_lease_*` / `capabilities_report`. inventory still **0**.

| Metric | Count | Δ vs S2-I77 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **228** | **−16** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I79 after seventy-eighth knife (2026-11-04)

**Slice:** fingerprint aggregator fold Δ16 ([stage2-fingerprint-aggregator-fold](../adr/stage2-fingerprint-aggregator-fold.md)); merged `heap_v103` a…dx + `v105` a…e/dx + `v106` a…e into `heap_v106_phase134_dx_fingerprint` (agg order); deleted ×16 + aligned `format!`. Soft-nail-free band. **Kept** `heap_v52`/`v69`/`v79_track_a`/`v82_track_c`/`v85_dx`/`v89_track_b`/`v93_track_d`/`v97_track_a`/`v100_track_c`/`v104_dx` / `v107+` / `heap_v111_*` / `farm_lease_*` / `capabilities_report`. inventory still **0**.

| Metric | Count | Δ vs S2-I78 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **212** | **−16** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I80 after seventy-ninth knife (2026-11-04)

**Slice:** fingerprint aggregator fold Δ16 ([stage2-fingerprint-aggregator-fold](../adr/stage2-fingerprint-aggregator-fold.md)); merged `heap_v107` a…dx + `v108` a/c/d/e/dx + `v109` a/c/d/e/dx into `heap_v110_phase138_track_a_props_fingerprint`; deleted ×16 + aligned `format!`; soft-nail retarget (`cache_v108`/`cache_v109` tag_active → merge target). **Kept** prior tips / `v110` c/d/e/dx / **`heap_v111_*`** / `farm_lease_*` / `capabilities_report`. inventory still **0**.

| Metric | Count | Δ vs S2-I79 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **196** | **−16** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I81 after eightieth knife (2026-11-04)

**Slice:** fingerprint aggregator fold Δ4 ([stage2-fingerprint-aggregator-fold](../adr/stage2-fingerprint-aggregator-fold.md)); merged `heap_v110` c/d/e/dx into adjacent kept `heap_v110_phase138_track_a_props_fingerprint`; deleted ×4 + aligned `format!`; soft-nail narrowed (`cache_v110_tag_active` keep track_a only). **Kept** prior tips / **全部 `heap_v111_*`** / `farm_lease_*` / `capabilities_report`. Contiguous foldable tips before `v111` **exhausted**. inventory still **0**.

| Metric | Count | Δ vs S2-I80 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **192** | **−4** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I82 after eighty-first knife (2026-11-04)

**Slice:** fingerprint aggregator earlier kept-tip re-fold Δ8 ([stage2-fingerprint-aggregator-fold](../adr/stage2-fingerprint-aggregator-fold.md)); merged `heap_v52`/`v69`/`v79_track_a`/`v82_track_c`/`v85_dx`/`v89_track_b`/`v93_track_d`/`v97_track_a` into `heap_v100_phase128_track_c_wasm_fingerprint`; deleted ×8 + aligned `format!`; soft-nail retarget (driver_gate v69/v79_a; lsp v79_a; smoke/service_di v93_d → merge target). **Kept** foundation×12 / `v100_c` / `v104_dx` / `v106_dx` / `v110_a` / **`heap_v111_*`** / `farm_lease_*` / `capabilities_report`. inventory still **0**.

| Metric | Count | Δ vs S2-I81 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **184** | **−8** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I83 after eighty-second knife (2026-11-04)

**Slice:** fingerprint aggregator earlier kept-tip re-fold Δ3 ([stage2-fingerprint-aggregator-fold](../adr/stage2-fingerprint-aggregator-fold.md)); merged `heap_v104_dx` / `v106_dx` / `v110_a` into `heap_v100_phase128_track_c_wasm_fingerprint`; deleted ×3 + aligned `format!` (23→20); soft-nail retarget (codable/token_expr v104; driver_gate + cache_v108–110 v110_a → merge target). **Kept** foundation×12 / `v100_c` / **`heap_v111_*`** / `farm_lease_*` / `capabilities_report`. inventory still **0**. Pre-`v111` earlier-tip re-fold axis **exhausted**.

| Metric | Count | Δ vs S2-I82 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **181** | **−3** |
| `crates/syx-driver/src/*_report.rs` files | **1** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I84 after eighty-third knife (2026-11-04)

**Slice:** capabilities product surface 另策 ([stage2-capabilities-report-surface](../adr/stage2-capabilities-report-surface.md)); renamed `capabilities_report.rs` → `capabilities.rs` + `pub mod capabilities`; retargeted CLI/LSP/website·hosted·marketing imports + capability_thin/floor path nails + A-10/website doc paths. **Kept** `run_capabilities_report` / doctor flags / LSP / fingerprint tower / foundation / `heap_v111_*` / `farm_lease_*`. inventory still **0**. Report KPI axis **exhausted** (`*_report.rs` = 0).

| Metric | Count | Δ vs S2-I83 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **181** | 0 |
| `crates/syx-driver/src/*_report.rs` files | **0** | **−1** |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I85 after eighty-fourth knife (2026-11-04)

**Slice:** fingerprint v111 aggregator fold Δ4 ([stage2-fingerprint-v111-fold](../adr/stage2-fingerprint-v111-fold.md)); merged `heap_v111` c/d/inherit/dx into `heap_v111_phase139_track_l_language_fingerprint`; deleted ×4 + aligned `format!` (20→16); soft-nail narrowed (`cache_v111_tag_active` keep track_l only). **Kept** foundation×12 / `v100_c` / `track_l` / `farm_lease_*` / capabilities API / `lang_props_v*`. inventory still **0**; report still **0**.

| Metric | Count | Δ vs S2-I84 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **177** | **−4** |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I86 after eighty-fifth knife (2026-11-04)

**Slice:** fingerprint foundation aggregator fold Δ12 ([stage2-fingerprint-foundation-fold](../adr/stage2-fingerprint-foundation-fold.md)); merged foundation×12 into `heap_v100_phase128_track_c_wasm_fingerprint`; deleted ×12 `*_fingerprint` + aligned `format!` (16→4); renamed `module_hash` helpers to `*_digest` / `host_feature_tag`; soft-nail retarget (reflect_witness + inheritance → merge target). **Kept** `v100_c` / `track_l` / `farm_lease_*` / capabilities API / `lang_props_v*`. inventory still **0**; report still **0**.

| Metric | Count | Δ vs S2-I85 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **165** | **−12** |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I87 after eighty-sixth knife (2026-11-04)

**Slice:** fingerprint track_l aggregator fold Δ1 ([stage2-fingerprint-track-l-fold](../adr/stage2-fingerprint-track-l-fold.md)); merged `heap_v111_phase139_track_l_language_fingerprint` into `heap_v100_phase128_track_c_wasm_fingerprint`; deleted ×1 + aligned `format!` (4→3); soft-nail retarget (`cache_v111_tag_active` + `cache_v111_lang_surface_fingerprints` → merge target). **Kept** `v100_c` / `farm_lease_*` / capabilities API / `lang_props_v*` / product sea. On-agg single tip. inventory still **0**; report still **0**.

| Metric | Count | Δ vs S2-I86 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **164** | **−1** |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I88 after eighty-seventh knife (2026-11-04)

**Slice:** product-sea cdn_bgp sticky/peer-announce rename ×11 ([stage2-fingerprint-product-sea-cdn-bgp-rename](../adr/stage2-fingerprint-product-sea-cdn-bgp-rename.md)); `*_fingerprint` → `*_digest` in `syx-async` `dist_transport` + lib re-exports + same-crate call sites. **Kept** `v100_c` / `farm_lease_*` / capabilities API / `lang_props_v*` / other unlisted cdn_bgp / gui C-ABI. inventory still **0**; report still **0**.

| Metric | Count | Δ vs S2-I87 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **153** | **−11** |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I89 after eighty-eighth knife (2026-11-04)

**Slice:** product-sea async dist/cdn gate rename ×8 ([stage2-fingerprint-product-sea-async-dist-rename](../adr/stage2-fingerprint-product-sea-async-dist-rename.md)); `*_fingerprint` → `*_digest` in `syx-async` `dist_transport` + lib re-exports + same-crate call sites + driver platform soft-nails (`dist_bgp_anycast_pop`, `dist_bgp_peer_exchange`). **Kept** `v100_c` / `farm_lease_*` / capabilities API / `lang_props_v*` / gui C-ABI / `select_*`. `syx-async` product-sea fingerprint surface **0**. inventory still **0**; report still **0**.

| Metric | Count | Δ vs S2-I88 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **145** | **−8** |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I90 after eighty-ninth knife (2026-11-04)

**Slice:** product-sea select channel rename ×14 ([stage2-fingerprint-product-sea-select-channel-rename](../adr/stage2-fingerprint-product-sea-select-channel-rename.md)); 8 symbols `*_fingerprint` → `*_digest` in `syx-parser` (×7) + `syx-vm` `channel_hosts` (×7) + vm lib re-exports + driver soft-nails (`channel_fairness`, `select_mixed_send_recv`, `select_deadline_strict`) + LSP `lsp_phase109_real` string probe. **Kept** `v100_c` / `farm_lease_*` / capabilities API / `lang_props_v*` / gui C-ABI / `SELECT_*_FINGERPRINT` consts. inventory still **0**; report still **0**.

| Metric | Count | Δ vs S2-I89 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **131** | **−14** |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I91 after ninetieth knife (2026-11-04)

**Slice:** product-sea gui shape-core rename ×9 ([stage2-fingerprint-product-sea-gui-shape-rename](../adr/stage2-fingerprint-product-sea-gui-shape-rename.md)); `*_fingerprint` → `*_digest` in `syx-gui` `shape.rs` + `lib.rs` `shape::` call sites only. **Kept** `gui_*_fingerprint` façades / `syx_gui_dylib_*` C-ABI / dlopen / leaf / `farm_lease_*` / `v100_c` / `lang_props_v*`. inventory still **0**; report still **0**.

| Metric | Count | Δ vs S2-I90 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **122** | **−9** |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I92 after ninety-first knife (2026-11-04)

**Slice:** product-sea gui shape façade rename ×9 ([stage2-fingerprint-product-sea-gui-shape-facade-rename](../adr/stage2-fingerprint-product-sea-gui-shape-facade-rename.md)); `gui_*_fingerprint` → `gui_*_digest` in `syx-gui` `lib.rs` defs + same-file tests. **Kept** `syx_gui_dylib_*` C-ABI / dlopen / VM `stdlib_phase97` hosts / leaf / `farm_lease_*` / `v100_c` / `lang_props_v*`. inventory still **0**; report still **0**.

| Metric | Count | Δ vs S2-I91 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **113** | **−9** |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I93 after ninety-second knife (2026-11-04)

**Slice:** product-sea gui render leaf rename ×14 ([stage2-fingerprint-product-sea-gui-leaf-rename](../adr/stage2-fingerprint-product-sea-gui-leaf-rename.md)); leaf defs×7 (`cjk_font`/`emoji`/`skia_*`/`wgpu_present`) + façades×7 census rows in `lib.rs` (`*_fingerprint` → `*_digest`). **Kept** `syx_gui_dylib_*` C-ABI / dlopen / VM hosts / non-leaf façades / `farm_lease_*` / `v100_c` / `lang_props_v*`. inventory still **0**; report still **0**.

| Metric | Count | Δ vs S2-I92 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **99** | **−14** |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I94 after ninety-third knife (2026-11-04)

**Slice:** product-sea VM stdlib_phase97 gui host rename ×9 ([stage2-fingerprint-product-sea-vm-stdlib97-host-rename](../adr/stage2-fingerprint-product-sea-vm-stdlib97-host-rename.md)); `gui_*_fingerprint` → `gui_*_digest` in `stdlib_phase97_hosts` + vm lib re-exports + driver gui soft-nails. Bodies still call `dlopen_*_fingerprint`. **Kept** C-ABI / dlopen wrappers / syx-gui 余 façade / `farm_lease_*` / `v100_c` / `lang_props_v*`. inventory still **0**; report still **0**.

| Metric | Count | Δ vs S2-I93 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **90** | **−9** |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I95 after ninety-fourth knife (2026-11-04)

**Slice:** product-sea VM `gui_dlopen` Rust wrapper rename ×10 ([stage2-fingerprint-product-sea-vm-gui-dlopen-wrapper-rename](../adr/stage2-fingerprint-product-sea-vm-gui-dlopen-wrapper-rename.md)); 9× `dlopen_*_fingerprint` → `dlopen_*_digest` + `read_fingerprint` → `read_digest`; soft-nail `stdlib_phase97_hosts` call sites. **Kept** dlsym `b"syx_gui_dylib_*_fingerprint"` / C-ABI / syx-gui 余 façade / `farm_lease_*` / `v100_c` / `lang_props_v*`. inventory still **0**; report still **0**.

| Metric | Count | Δ vs S2-I94 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **80** | **−10** |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I96 after ninety-fifth knife (2026-11-04)

**Slice:** product-sea syx-gui remaining façade rename ×6 ([stage2-fingerprint-product-sea-gui-remaining-facade-rename](../adr/stage2-fingerprint-product-sea-gui-remaining-facade-rename.md)); `gui_{ime_preedit,egui_mvp,egui_wgpu,vertical_rl,pump_reentrancy,doctor_coverage}_fingerprint` → `*_digest` in `lib.rs` + same-file tests. **Kept** C-ABI×19 / dlsym / synui / `farm_lease_*` / `v100_c` / `lang_props_v*`. inventory still **0**; report still **0**.

| Metric | Count | Δ vs S2-I95 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **74** | **−6** |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I97 after ninety-sixth knife (2026-11-04)

**Slice:** product-sea VM synui rename ×7 ([stage2-fingerprint-product-sea-vm-synui-rename](../adr/stage2-fingerprint-product-sea-vm-synui-rename.md)); 5 unique Rust symbols → `*_digest` (real+stub = Δ7); soft-nail hosts/lib. **Kept** `__syx_synui_*` HostFn strings / `HostFnId` / `SYNUI_*_FINGERPRINT` const / C-ABI / `farm_lease_*` / `v100_c` / `lang_props_v*`. inventory still **0**; report still **0**.

| Metric | Count | Δ vs S2-I96 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **67** | **−7** |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I98 after ninety-seventh knife (2026-11-04)

**Slice:** product-sea driver generate helper rename ×3 ([stage2-fingerprint-product-sea-generate-rename](../adr/stage2-fingerprint-product-sea-generate-rename.md)); `native_input` / `unified_mir_fn_count` / `dependency_tree` `*_fingerprint` → `*_digest` in `lib.rs` + `query.rs` soft-nail. **Kept** C-ABI×19 / `farm_lease_*` / `heap_v100_*` / orphan×2 / test-noise / `lang_props_v*`. inventory still **0**; report still **0**.

| Metric | Count | Δ vs S2-I97 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **64** | **−3** |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I99 after ninety-eighth knife (2026-11-04)

**Slice:** product-sea orphan tip rename ×2 ([stage2-fingerprint-product-sea-orphan-rename](../adr/stage2-fingerprint-product-sea-orphan-rename.md)); `reflect_witness_honest` / `macro_external_plugin_prod` `*_fingerprint` → `*_digest`. **Kept** hasher bytes / C-ABI×19 / `farm_lease_*` / `heap_v100_*` / test-noise / `lang_props_v*`. inventory still **0**; report still **0**.

| Metric | Count | Δ vs S2-I98 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **62** | **−2** |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I100 after ninety-ninth knife (2026-11-04)

**Slice:** test-noise ×6 ([stage2-fingerprint-test-noise-rename](../adr/stage2-fingerprint-test-noise-rename.md)); matrix helpers → `*_digest` + driver_gate probe needles retarget to `*_count`. **Kept** C-ABI×19 / `farm_lease_*`×36 / `heap_v100_*`×1 / `lang_props_v*`. inventory still **0**; report still **0**. Remaining fingerprint_fn = MUST STAY floor.

| Metric | Count | Δ vs S2-I99 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **56** | **−6** |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I101 after hundredth knife (2026-11-04)

**Slice:** farm_lease digest rename ×36 ([stage2-fingerprint-farm-lease-digest-rename](../adr/stage2-fingerprint-farm-lease-digest-rename.md)); all `farm_lease_*_fingerprint` → `farm_lease_*_digest` in `farm_lease_auth.rs` + lib/tests soft-nail; gate return literals kept. **Kept** C-ABI×19 / `heap_v100_*`×1 / `lang_props_v*`. inventory still **0**; report still **0**. Remaining fingerprint_fn = C-ABI + heap.

| Metric | Count | Δ vs S2-I100 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 | 0 |
| `fn *_fingerprint` matches | **20** | **−36** |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I102 after hundred-first knife (2026-11-04)

**Slice:** fingerprint_const gui leaf rename ([stage2-fingerprint-const-gui-leaf-rename](../adr/stage2-fingerprint-const-gui-leaf-rename.md)); syx-gui leaf `*_FINGERPRINT` → `*_DIGEST` (shape/egui/wgpu/skia/emoji + lib refs); string literals kept. **Kept** C-ABI×19 / heap_v100 / VM·parser const / `lang_props_v*`. inventory still **0**; report still **0**; `fingerprint_fn` still **20**.

| Metric | Count | Δ vs S2-I101 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | **201** | **−40** |
| `fn *_fingerprint` matches | **20** | 0 |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I103 after hundred-second knife (2026-11-04)

**Slice:** fingerprint_const VM synui rename ([stage2-fingerprint-const-vm-synui-rename](../adr/stage2-fingerprint-const-vm-synui-rename.md)); 36× `SYNUI_*_FINGERPRINT` → `SYNUI_*_DIGEST` in real+stub+lib + `synui_surface` soft-nail; literals kept. **Kept** HostFn `__syx_synui_*` / `HostFnId` / SELECT const / C-ABI / heap. inventory still **0**; report still **0**; `fingerprint_fn` still **20**.

| Metric | Count | Δ vs S2-I102 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | **55** | **−146** |
| `fn *_fingerprint` matches | **20** | 0 |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I104 after hundred-third knife (2026-11-05)

**Slice:** fingerprint_const SELECT rename ([stage2-fingerprint-const-select-rename](../adr/stage2-fingerprint-const-select-rename.md)); 7× `SELECT_*_FINGERPRINT` → `SELECT_*_DIGEST` in parser + `channel_hosts` + soft-nail (lsp 109–116 + `select_deadline_strict`); literals kept. **Kept** C-ABI×19 / heap_v100 / HostFn / `lang_props_v*`. inventory still **0**; report still **0**; `fingerprint_fn` still **20**. **const axis cleared.**

| Metric | Count | Δ vs S2-I103 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | **0** | **−55** |
| `fn *_fingerprint` matches | **20** | 0 |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I105 after hundred-fourth knife (2026-11-05)

**Slice:** fingerprint_fn heap tip digest rename ([stage2-fingerprint-heap-tip-digest-rename](../adr/stage2-fingerprint-heap-tip-digest-rename.md)); `heap_v100_phase128_track_c_wasm_fingerprint` → `*_digest` (def + caller + soft-nail); hasher bytes / tip body kept. **Kept** C-ABI×19 / `lang_props_v*`. inventory still **0**; report still **0**; `fingerprint_const` still **0**.

| Metric | Count | Δ vs S2-I104 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | **0** | 0 |
| `fn *_fingerprint` matches | **19** | **−1** |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I106 after hundred-fifth knife (2026-11-05)

**Slice:** fingerprint_fn C-ABI/dlsym digest rename ([stage2-fingerprint-cabi-dylib-digest-rename](../adr/stage2-fingerprint-cabi-dylib-digest-rename.md)); 19× `syx_gui_dylib_*_fingerprint` → `*_digest` + 9× dlsym/fields in `gui_dlopen`; `c"…"` literals kept. **Kept** `lang_props_v*` / HostFn strings. inventory still **0**; report still **0**; `fingerprint_const` still **0**. **fn axis cleared.**

| Metric | Count | Δ vs S2-I105 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | **0** | 0 |
| `fn *_fingerprint` matches | **0** | **−19** |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I107 after hundred-sixth knife (2026-11-05)

**Slice:** lang_props product retire ([stage2-lang-props-retire](../adr/stage2-lang-props-retire.md)); deleted `lang_props_v2…v6` + `pub mod`; soft-nail retarget to `lib.rs` literals; hasher `b"lang-props-v*:…"` **kept**. inventory/report/fingerprint still **0**. **lang_props KPI cleared.**

| Metric | Count | Δ vs S2-I106 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | **0** | 0 |
| `fn *_fingerprint` matches | **0** | 0 |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | **0** | **−5** |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I108 after hundred-seventh knife (2026-11-05)

**Slice:** S2-02 open — props fingerprint soft-nail retire ([stage2-s2-02-open](../adr/stage2-s2-02-open.md)); deleted 5× `props_*` `include_str!(lib.rs)+contains(lang-props…)` tests; behavior + docs honesty + `cache_v*` kept. **Census KPI unchanged (all 0).** ≠ S2-01 `[x]` / ≠ S2-02 `[x]`.

| Metric | Count | Δ vs S2-I107 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | **0** | 0 |
| `fn *_fingerprint` matches | **0** | 0 |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | **0** | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I109 after hundred-eighth knife (2026-11-05)

**Slice:** S2-02 cache soft-nail retire ([stage2-cache-soft-nail-retire](../adr/stage2-cache-soft-nail-retire.md)); deleted `cache_v108`–`v110_invalidation.rs` (≈−21 pure fingerprint soft-nails); retargeted `driver_gate` fixtures → `cache_v111_invalidation.rs`; tip kept. **Census KPI unchanged (all 0).** ≠ S2-01 `[x]` / ≠ S2-02 `[x]`.

| Metric | Count | Δ vs S2-I108 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | **0** | 0 |
| `fn *_fingerprint` matches | **0** | 0 |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | **0** | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I110 after hundred-ninth knife (2026-11-05)

**Slice:** S2-02 driver_gate exists soft-nail retire ([stage2-driver-gate-exists-soft-nail-retire](../adr/stage2-driver-gate-exists-soft-nail-retire.md)); deleted ≈28 `*_gate_exists` / files_exist / crate_exists + 3 empty charters + dead `browser_tier2u_meta_allowable` (≈−32); `cache_v111` + behavioral kept. **Census KPI unchanged (all 0).** ≠ S2-01 `[x]` / ≠ S2-02 `[x]`.

| Metric | Count | Δ vs S2-I109 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | **0** | 0 |
| `fn *_fingerprint` matches | **0** | 0 |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | **0** | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I111 after hundred-tenth knife (2026-11-05)

**Slice:** S2-02 driver_gate invalidation-exact soft-nail retire ([stage2-driver-gate-invalidation-exact-retire](../adr/stage2-driver-gate-invalidation-exact-retire.md)); deleted heap/cache `*_invalidation_exact`×8 + cache tag soft×3 (≈−11); `cache_v111` + `native_llvm_locals_v7` + behavioral kept. **Census KPI unchanged (all 0).** ≠ S2-01 `[x]` / ≠ S2-02 `[x]`.

| Metric | Count | Δ vs S2-I110 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | **0** | 0 |
| `fn *_fingerprint` matches | **0** | 0 |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | **0** | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I112 after hundred-eleventh knife (2026-11-05)

**Slice:** S2-02 driver_gate host_registry soft-nail retire ([stage2-driver-gate-host-registry-soft-nail-retire](../adr/stage2-driver-gate-host-registry-soft-nail-retire.md)); deleted `host_registry_*`×5 (`include_str!(registry)+contains`); `*_host` / parity behavioral kept. **Census KPI unchanged (all 0).** ≠ S2-01 `[x]` / ≠ S2-02 `[x]`.

| Metric | Count | Δ vs S2-I111 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | **0** | 0 |
| `fn *_fingerprint` matches | **0** | 0 |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | **0** | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I113 after hundred-twelfth knife (2026-11-05)

**Slice:** S2-02 driver_gate meta-allowable + native_locals soft-nail retire ([stage2-driver-gate-meta-allowable-native-locals-retire](../adr/stage2-driver-gate-meta-allowable-native-locals-retire.md)); deleted `dist_prod_ga` / `borrow_mandatory` meta×2 + `native_llvm_locals_v7` (≈−3); `soft_debt_zero` kept. **Census KPI unchanged (all 0).** ≠ S2-01 `[x]` / ≠ S2-02 `[x]`.

| Metric | Count | Δ vs S2-I112 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | **0** | 0 |
| `fn *_fingerprint` matches | **0** | 0 |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | **0** | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## S2-I114 after hundred-thirteenth knife (2026-11-05)

**Slice:** S2-02 driver_gate include_str surface retire + floor relax ([stage2-driver-gate-include-str-surface-retire](../adr/stage2-driver-gate-include-str-surface-retire.md)); `#[test]` floor **500→450** (behavioral≥350 kept); deleted import/surface/abi include_str soft×25; `soft_debt_zero` kept. **Census KPI unchanged (all 0).** ≠ S2-01 `[x]` / ≠ S2-02 `[x]`.

| Metric | Count | Δ vs S2-I113 |
|--------|------:|------------:|
| `fn *_inventory` matches | **0** | 0 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | **0** | 0 |
| `fn *_fingerprint` matches | **0** | 0 |
| `crates/syx-driver/src/*_report.rs` files | **0** | 0 |
| `crates/syx-driver/src/lang_props_v*.rs` files | **0** | 0 |
| `Always \`true\`` / `Always \`false\`` comment matches | 0 | 0 |

## Non-claims

- **≠** S2-01 `[x]` / Stage-2 exit.
- Re-run the script after each S2-01 migration slice; update this table when materially changed.
