# Stage-2 file-split census (S2-05)

**ADR:** [stage2-s2-05-open](adr/stage2-s2-05-open.md) · **Window:** S2-I165（baseline）· **Current:** S2-I235  

 
 

 
**Goal (S2-05):** split giant `lib.rs` faces in `syx-mir` / `syx-driver` / `syx-semantic` into maintainable modules.  
**This census:** honest LOC + sidecar inventory — **≠** split complete / **≠** S2-05 `[x]`.

## Command

```bash
bash scripts/s2_05_file_split_census.sh
```

## S2-I165 baseline (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **13216** |
| `crates/syx-driver/src/lib.rs` LOC | **9117** |
| `crates/syx-semantic/src/lib.rs` LOC | **8889** |
| Combined three `lib.rs` LOC | **31222** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **1** (`mir_text.rs`) |
| `syx-driver` sidecar `.rs` (excl. lib/bin) | **17** |
| `syx-semantic` sidecar `.rs` (excl. lib/bin) | **3** |

## S2-I166 after mir_types extract (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **12941** (−275 vs I165) |
| `crates/syx-mir/src/mir_types.rs` LOC | **284** |
| Combined three `lib.rs` LOC | **30947** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **2** |

ADR: [stage2-s2-05-mir-types-extract](adr/stage2-s2-05-mir-types-extract.md).

## S2-I167 after mir_codec extract (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **12698** (−243 vs I166) |
| `crates/syx-mir/src/mir_codec.rs` LOC | **264** |
| Combined three `lib.rs` LOC | **30704** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **3** |

ADR: [stage2-s2-05-mir-codec-extract](adr/stage2-s2-05-mir-codec-extract.md).

## S2-I168 after mir_aot extract (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **12241** (−457 vs I167) |
| `crates/syx-mir/src/mir_aot.rs` LOC | **484** |
| Combined three `lib.rs` LOC | **30247** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **4** |

ADR: [stage2-s2-05-mir-aot-extract](adr/stage2-s2-05-mir-aot-extract.md).

## S2-I169 after mir_opt extract (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **11353** (−888 vs I168) |
| `crates/syx-mir/src/mir_opt.rs` LOC | **913** |
| Combined three `lib.rs` LOC | **29359** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **5** (`mir_text`, `mir_types`, `mir_codec`, `mir_aot`, `mir_opt`) |

ADR: [stage2-s2-05-mir-opt-extract](adr/stage2-s2-05-mir-opt-extract.md).

## S2-I170 after mir_module_index extract (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **10742** (−611 vs I169) |
| `crates/syx-mir/src/mir_module_index.rs` LOC | **632** |
| Combined three `lib.rs` LOC | **28748** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **6** (+ `mir_module_index`) |

ADR: [stage2-s2-05-mir-module-index-extract](adr/stage2-s2-05-mir-module-index-extract.md).

## S2-I171 after mir_lower_ctx extract (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **10015** (−727 vs I170) |
| `crates/syx-mir/src/mir_lower_ctx.rs` LOC | **744** |
| Combined three `lib.rs` LOC | **28021** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **7** (+ `mir_lower_ctx`) |

ADR: [stage2-s2-05-mir-lower-ctx-extract](adr/stage2-s2-05-mir-lower-ctx-extract.md).

## S2-I172 after mir_lower_expr_literal extract (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **9804** (−211 vs I171) |
| `crates/syx-mir/src/mir_lower_expr_literal.rs` LOC | **233** |
| Combined three `lib.rs` LOC | **27810** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **8** (+ `mir_lower_expr_literal`) |

ADR: [stage2-s2-05-mir-lower-expr-literal-extract](adr/stage2-s2-05-mir-lower-expr-literal-extract.md).

## S2-I173 after mir_lower_expr_call_ident extract (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **9120** (−684 vs I172) |
| `crates/syx-mir/src/mir_lower_expr_call_ident.rs` LOC | **723** |
| Combined three `lib.rs` LOC | **27126** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **9** (+ `mir_lower_expr_call_ident`) |

ADR: [stage2-s2-05-mir-lower-expr-call-ident-extract](adr/stage2-s2-05-mir-lower-expr-call-ident-extract.md).

## S2-I174 after mir_lower_expr_call_member extract (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **8165** (−955 vs I173) |
| `crates/syx-mir/src/mir_lower_expr_call_member.rs` LOC | **1001** |
| Combined three `lib.rs` LOC | **26171** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **10** (+ `mir_lower_expr_call_member`) |

ADR: [stage2-s2-05-mir-lower-expr-call-member-extract](adr/stage2-s2-05-mir-lower-expr-call-member-extract.md).

## S2-I175 after mir_lower_expr_member extract (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **7801** (−364 vs I174) |
| `crates/syx-mir/src/mir_lower_expr_member.rs` LOC | **393** |
| Combined three `lib.rs` LOC | **25807** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **11** (+ `mir_lower_expr_member`) |

ADR: [stage2-s2-05-mir-lower-expr-member-extract](adr/stage2-s2-05-mir-lower-expr-member-extract.md).

## S2-I176 after mir_lower_expr_member_assign extract (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **7443** (−358 vs I175) |
| `crates/syx-mir/src/mir_lower_expr_member_assign.rs` LOC | **379** |
| Combined three `lib.rs` LOC | **25449** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **12** (+ `mir_lower_expr_member_assign`) |

ADR: [stage2-s2-05-mir-lower-expr-member-assign-extract](adr/stage2-s2-05-mir-lower-expr-member-assign-extract.md).

## S2-I177 after mir_lower_expr_index_assign extract (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **7303** (−140 vs I176) |
| `crates/syx-mir/src/mir_lower_expr_index_assign.rs` LOC | **163** |
| Combined three `lib.rs` LOC | **25309** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **13** (+ `mir_lower_expr_index_assign`) |

ADR: [stage2-s2-05-mir-lower-expr-index-assign-extract](adr/stage2-s2-05-mir-lower-expr-index-assign-extract.md).

## S2-I178 after mir_lower_expr_binary extract (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **7175** (−128 vs I177) |
| `crates/syx-mir/src/mir_lower_expr_binary.rs` LOC | **158** |
| Combined three `lib.rs` LOC | **25181** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **14** (+ `mir_lower_expr_binary`) |

ADR: [stage2-s2-05-mir-lower-expr-binary-extract](adr/stage2-s2-05-mir-lower-expr-binary-extract.md).

## S2-I179 after mir_lower_expr_unary extract (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **7068** (−107 vs I178) |
| `crates/syx-mir/src/mir_lower_expr_unary.rs` LOC | **130** |
| Combined three `lib.rs` LOC | **25074** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **15** (+ `mir_lower_expr_unary`) |

ADR: [stage2-s2-05-mir-lower-expr-unary-extract](adr/stage2-s2-05-mir-lower-expr-unary-extract.md).

## S2-I180 after mir_lower_expr_closure extract (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **7003** (−65 vs I179) |
| `crates/syx-mir/src/mir_lower_expr_closure.rs` LOC | **88** |
| Combined three `lib.rs` LOC | **25009** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **16** (+ `mir_lower_expr_closure`) |

ADR: [stage2-s2-05-mir-lower-expr-closure-extract](adr/stage2-s2-05-mir-lower-expr-closure-extract.md).

## S2-I181 after mir_lower_expr_index extract (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **6941** (−62 vs I180) |
| `crates/syx-mir/src/mir_lower_expr_index.rs` LOC | **87** |
| Combined three `lib.rs` LOC | **24947** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **17** (+ `mir_lower_expr_index`) |

ADR: [stage2-s2-05-mir-lower-expr-index-extract](adr/stage2-s2-05-mir-lower-expr-index-extract.md).

## S2-I182 after mir_lower_expr_task_block extract (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **6886** (−55 vs I181) |
| `crates/syx-mir/src/mir_lower_expr_task_block.rs` LOC | **78** |
| Combined three `lib.rs` LOC | **24892** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **18** (+ `mir_lower_expr_task_block`) |

ADR: [stage2-s2-05-mir-lower-expr-task-block-extract](adr/stage2-s2-05-mir-lower-expr-task-block-extract.md).

## S2-I183 after mir_lower_expr_compound_assign extract (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **6833** (−53 vs I182) |
| `crates/syx-mir/src/mir_lower_expr_compound_assign.rs` LOC | **77** |
| Combined three `lib.rs` LOC | **24839** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **19** (+ `mir_lower_expr_compound_assign`) |

ADR: [stage2-s2-05-mir-lower-expr-compound-assign-extract](adr/stage2-s2-05-mir-lower-expr-compound-assign-extract.md).

## S2-I184–I188 after Ternary→Is batch extract (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **6681** (−152 vs I183) |
| Batch sidecars LOC | ternary **61** · tuple **64** · cast **57** · optional_cast **40** · is **39** |
| Combined three `lib.rs` LOC | **24687** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **24** (+5: ternary/tuple/cast/optional_cast/is) |

ADR: [stage2-s2-05-mir-lower-expr-ternary-to-is-batch](adr/stage2-s2-05-mir-lower-expr-ternary-to-is-batch.md).

## S2-I189–I194 after Call→MacroInvoke batch extract (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **6501** (−180 vs I188) |
| Batch sidecars LOC | call **156** · array **31** · await **30** · assign **31** · type_self **31** · macro_invoke **25** |
| Combined three `lib.rs` LOC | **24507** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **30** (+6: call/array/await/assign/type_self/macro_invoke) |
| Honesty floor (`syx-mir` lib) | **6000** (was 6500) |

ADR: [stage2-s2-05-mir-lower-expr-call-to-macro-batch](adr/stage2-s2-05-mir-lower-expr-call-to-macro-batch.md).

## S2-I195–I200 after Range→Super batch extract (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **6297** (−204 vs I194) |
| Batch sidecars LOC | range **263** · ident **14** · group **15** · borrow **15** · keypath **11** · super **12** |
| Combined three `lib.rs` LOC | **24303** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **36** (+6: range/ident/group/borrow/keypath/super) |
| Honesty floor (`syx-mir` lib) | **6000** (unchanged) |

ADR: [stage2-s2-05-mir-lower-expr-range-to-super-batch](adr/stage2-s2-05-mir-lower-expr-range-to-super-batch.md).

## S2-I201–I204 after free_vars→shells batch extract (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **4541** (−1756 vs I200) |
| Batch sidecars LOC | free_vars **115** · switch **429** · stmt **884** · shells **396** |
| Combined three `lib.rs` LOC | **22547** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **40** (+4: collect_closure_free_vars/switch_pattern/stmt/shells) |
| Honesty floor (`syx-mir` lib) | **4000** (was 6000) |

ADR: [stage2-s2-05-mir-free-vars-to-shells-batch](adr/stage2-s2-05-mir-free-vars-to-shells-batch.md).

## S2-I205–I208 after type_members→infer batch extract (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **2960** (−1581 vs I204) |
| Batch sidecars LOC | type_members **414** · class_init **358** · host_cast **272** · type_infer **650** |
| Combined three `lib.rs` LOC | **20966** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **44** (+4: type_members/class_init/host_cast/type_infer) |
| Honesty floor (`syx-mir` lib) | **2900** (was 4000; plan mid-target 3000, landed 2960) |

ADR: [stage2-s2-05-mir-type-members-to-infer-batch](adr/stage2-s2-05-mir-type-members-to-infer-batch.md).

## S2-I209–I212 after SIMD→support batch extract (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **2366** (−596 vs I208) |
| Batch sidecars LOC | simd **355** · lower_fn **145** · member_dispatch **78** · support **97** |
| Combined three `lib.rs` LOC | **20372** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **48** (+4: simd/lower_function/member_dispatch/support) |
| Honesty floor (`syx-mir` lib) | **2000** (was 2900) |

ADR: [stage2-s2-05-mir-simd-to-support-batch](adr/stage2-s2-05-mir-simd-to-support-batch.md).

## S2-I213–I214 after lower_expr→orchestrator close-out (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-mir/src/lib.rs` LOC | **2015** (−351 vs I212) |
| Batch sidecars LOC | lower_expr **130** · lower_module **213** |
| Combined three `lib.rs` LOC | **20021** |
| `syx-mir` sidecar `.rs` (excl. lib/bin) | **50** (+2: lower_expr/lower_module) |
| Honesty floor (`syx-mir` lib) | **2000** (unchanged; tests still dominate) |

ADR: [stage2-s2-05-mir-lower-expr-to-module-batch](adr/stage2-s2-05-mir-lower-expr-to-module-batch.md). **MIR production peel exhausted.**

## S2-I215–I217 after driver fingerprint→lsp batch extract (2026-11-06)

| Metric | Value |
|--------|------:|
| `crates/syx-driver/src/lib.rs` LOC | **5766** (−3351 vs I214) |
| Batch sidecars LOC | fingerprint **2021** · farm_tcp **406** · lsp_ide **1073** |
| Combined three `lib.rs` LOC | **16670** |
| `syx-driver` sidecar `.rs` (excl. lib/bin) | **20** (+3: cache_fingerprint/farm_tcp/lsp_ide) |
| Honesty floor (`syx-driver` lib) | **5500** (was 8000) |

ADR: [stage2-s2-05-driver-fingerprint-to-lsp-batch](adr/stage2-s2-05-driver-fingerprint-to-lsp-batch.md).

## S2-I218–I220 after driver mir_unify→test→compile batch extract (2026-11-07)

| Metric | Value |
|--------|------:|
| `crates/syx-driver/src/lib.rs` LOC | **4141** (−1625 vs I217) |
| Batch sidecars LOC | mir_unify **529** · test_runner **715** · compile_linked **455** |
| Combined three `lib.rs` LOC | **15045** |
| `syx-driver` sidecar `.rs` (excl. lib/bin) | **23** (+3: mir_unify/test_runner/compile_linked) |
| Honesty floor (`syx-driver` lib) | **4000** (was 5500) |

ADR: [stage2-s2-05-driver-mir-unify-to-compile-batch](adr/stage2-s2-05-driver-mir-unify-to-compile-batch.md).

## S2-I221–I223 after driver compile_ctx→prelude→native batch extract (2026-11-07)

| Metric | Value |
|--------|------:|
| `crates/syx-driver/src/lib.rs` LOC | **2716** (−1425 vs I220) |
| Batch sidecars LOC | compile_file_context **286** · prelude_glue **840** · native_build **380** |
| Combined three `lib.rs` LOC | **13620** |
| `syx-driver` sidecar `.rs` (excl. lib/bin) | **26** (+3: compile_file_context/prelude_glue/native_build) |
| Honesty floor (`syx-driver` lib) | **2500** (was 4000) |

ADR: [stage2-s2-05-driver-compile-ctx-to-native-batch](adr/stage2-s2-05-driver-compile-ctx-to-native-batch.md).

## S2-I224–I226 after driver options→check→run batch extract (2026-11-07)

| Metric | Value |
|--------|------:|
| `crates/syx-driver/src/lib.rs` LOC | **2119** (−597 vs I223) |
| Batch sidecars LOC | driver_options **267** · check_project **132** · run_exec **255** |
| Combined three `lib.rs` LOC | **13023** |
| `syx-driver` sidecar `.rs` (excl. lib/bin) | **29** (+3: driver_options/check_project/run_exec) |
| Honesty floor (`syx-driver` lib) | **2000** (was 2500) |

ADR: [stage2-s2-05-driver-options-to-run-batch](adr/stage2-s2-05-driver-options-to-run-batch.md).

## S2-I227–I229 after driver format→cache→wasm batch extract (2026-11-07)

| Metric | Value |
|--------|------:|
| `crates/syx-driver/src/lib.rs` LOC | **1613** (−506 vs I226) |
| Batch sidecars LOC | format_lint **187** · compile_cache **184** · run_wasm **196** |
| Combined three `lib.rs` LOC | **12517** |
| `syx-driver` sidecar `.rs` (excl. lib/bin) | **32** (+3: format_lint/compile_cache/run_wasm) |
| Honesty floor (`syx-driver` lib) | **1500** (was 2000) |

ADR: [stage2-s2-05-driver-format-to-wasm-batch](adr/stage2-s2-05-driver-format-to-wasm-batch.md).

## S2-I230–I232 after driver compile→gates→debug batch extract (2026-11-07)

| Metric | Value |
|--------|------:|
| `crates/syx-driver/src/lib.rs` LOC | **1282** (−331 vs I229) |
| Batch sidecars LOC | compile_to_program **176** · feature_gates **138** · debug_repl **94** |
| Combined three `lib.rs` LOC | **12186** |
| `syx-driver` sidecar `.rs` (excl. lib/bin) | **35** (+3: compile_to_program/feature_gates/debug_repl) |
| Honesty floor (`syx-driver` lib) | **1200** (was 1500) |

ADR: [stage2-s2-05-driver-compile-to-debug-batch](adr/stage2-s2-05-driver-compile-to-debug-batch.md).

## S2-I233–I235 after driver production close-out batch extract (2026-11-07)

| Metric | Value |
|--------|------:|
| `crates/syx-driver/src/lib.rs` LOC | **1170** (−112 vs I232) |
| Batch sidecars LOC | doc_project **28** · cranelift_stubs **40** · thin_host **91** |
| Combined three `lib.rs` LOC | **12074** |
| `syx-driver` sidecar `.rs` (excl. lib/bin) | **38** (+3: doc_project/cranelift_stubs/thin_host) |
| Honesty floor (`syx-driver` lib) | **1100** (was 1200) |

ADR: [stage2-s2-05-driver-production-closeout-batch](adr/stage2-s2-05-driver-production-closeout-batch.md).

## Honesty notes

| Crate | Already extracted (examples) | Remaining debt |
|-------|------------------------------|----------------|
| `syx-mir` | …/lower_expr+module | **生产尾面耗尽；tests 仍堆 lib** |
| `syx-driver` | …/compile·gates·debug/**close-out** | **可搬生产 fn/const 簇已清空；hub + `#[cfg(test)]` + host_rt stubs 常驻** — **next knife = I236 syx-semantic 首抽**（≠ lib peel 全面耗尽宣称 / ≠ S2-05 `[x]`） |
| `syx-semantic` | thin helpers | Core checker / lowers still in `lib.rs` |

## Ladder (locked at open)

1. **S2-I165:** census + open ADR — **landed**.
2. **S2-I166:** first knife from **`syx-mir`** (`mir_types`) — **landed**.
3. **S2-I167:** `mir_codec` — **landed**.
4. **S2-I168:** `mir_aot` — **landed**.
5. **S2-I169:** `mir_opt` — **landed**.
6. **S2-I170:** `mir_module_index` (mir_lower 首抽) — **landed**.
7. **S2-I171:** LowerCtx extract — **landed**.
8. **S2-I172:** `lower_expr` Literal first extract — **landed**.
9. **S2-I173:** Call Ident-callee extract — **landed**.
10. **S2-I174:** Call Member sub-face — **landed**.
11. **S2-I175:** Expr::Member arm — **landed**.
12. **S2-I176:** Expr::MemberAssign arm — **landed**.
13. **S2-I177:** Expr::IndexAssign arm — **landed**.
14. **S2-I178:** Expr::Binary arm — **landed**.
15. **S2-I179:** Expr::Unary arm — **landed**.
16. **S2-I180:** Expr::Closure arm — **landed**.
17. **S2-I181:** Expr::Index arm — **landed**.
18. **S2-I182:** Expr::TaskBlock arm — **landed**.
19. **S2-I183:** Expr::CompoundAssign arm — **landed**.
20. **S2-I184:** Expr::Ternary arm — **landed** (batch).
21. **S2-I185:** Expr::Tuple arm — **landed** (batch).
22. **S2-I186:** Expr::Cast arm (+3 helpers `pub(crate)`) — **landed** (batch).
23. **S2-I187:** Expr::OptionalCast arm — **landed** (batch).
24. **S2-I188:** Expr::Is arm — **landed** (batch).
25. **S2-I189:** Call shell — **landed** (batch).
26. **S2-I190:** Expr::Array arm — **landed** (batch).
27. **S2-I191:** Expr::Await arm — **landed** (batch).
28. **S2-I192:** Expr::Assign arm (+narrow-cast `pub(crate)`) — **landed** (batch).
29. **S2-I193:** Expr::TypeSelf arm — **landed** (batch).
30. **S2-I194:** Expr::MacroInvoke arm — **landed** (batch).
31. **S2-I195:** Range family (dict*/bounds/contains + 3 arms) — **landed** (batch).
32. **S2-I196:** Expr::Ident arm — **landed** (batch).
33. **S2-I197:** Expr::Group arm — **landed** (batch).
34. **S2-I198:** Expr::Borrow arm — **landed** (batch).
35. **S2-I199:** Expr::KeyPath arm — **landed** (batch).
36. **S2-I200:** Expr::Super arm — **landed** (batch).
37. **S2-I201:** `collect_closure_free_vars` — **landed** (batch).
38. **S2-I202:** switch/pattern bind — **landed** (batch).
39. **S2-I203:** `lower_stmt` face — **landed** (batch).
40. **S2-I204:** shells + `mir_shell_fn` — **landed** (batch).
41. **S2-I205:** `lower_type_members` — **landed** (batch).
42. **S2-I206:** `lower_class_init` / vtable / mut_ref — **landed** (batch).
43. **S2-I207:** host/cast helpers — **landed** (batch).
44. **S2-I208:** type-infer helpers — **landed** (batch).
45. **S2-I209:** SIMD helpers — **landed** (batch).
46. **S2-I210:** `lower_function_with_params` — **landed** (batch).
47. **S2-I211:** member-call dispatch — **landed** (batch).
48. **S2-I212:** lower support (flatten/locals/keypath) — **landed** (batch).
49. **S2-I213:** `lower_expr` shell — **landed** (batch).
50. **S2-I214:** `lower_module*` orchestrator — **landed** (batch); **mir production peel exhausted**.
51. **S2-I215:** driver `cache_fingerprint` — **landed** (batch).
52. **S2-I216:** driver `farm_tcp` — **landed** (batch).
53. **S2-I217:** driver `lsp_ide` — **landed** (batch).
54. **S2-I218:** driver `mir_unify` — **landed** (batch).
55. **S2-I219:** driver `test_runner` — **landed** (batch).
56. **S2-I220:** driver `compile_linked` — **landed** (batch).
57. **S2-I221:** driver `compile_file_context` — **landed** (batch).
58. **S2-I222:** driver `prelude_glue` — **landed** (batch).
59. **S2-I223:** driver `native_build` — **landed** (batch).
60. **S2-I224:** driver `driver_options` hub — **landed** (batch).
61. **S2-I225:** driver `check_project` — **landed** (batch).
62. **S2-I226:** driver `run_exec` — **landed** (batch).
63. **S2-I227:** driver `format_lint` — **landed** (batch).
64. **S2-I228:** driver `compile_cache` — **landed** (batch).
65. **S2-I229:** driver `run_wasm` — **landed** (batch).
66. **S2-I230:** driver `compile_to_program` — **landed** (batch).
67. **S2-I231:** driver `feature_gates` — **landed** (batch).
68. **S2-I232:** driver `debug_repl` — **landed** (batch).
69. **S2-I233:** driver `doc_project` — **landed** (batch).
70. **S2-I234:** driver `cranelift_stubs` — **landed** (batch).
71. **S2-I235:** driver `thin_host` — **landed** (batch); movable production fn clusters cleared.
72. **S2-I236:** syx-semantic 首抽 → further semantic knives → dedicated **S2-05 `[x]` claim**.

## References

- [stage2-s2-05-open](adr/stage2-s2-05-open.md) · S2-I165
- [stage2-s2-05-mir-types-extract](adr/stage2-s2-05-mir-types-extract.md) · S2-I166
- [stage2-s2-05-mir-codec-extract](adr/stage2-s2-05-mir-codec-extract.md) · S2-I167
- [stage2-s2-05-mir-aot-extract](adr/stage2-s2-05-mir-aot-extract.md) · S2-I168
- [stage2-s2-05-mir-opt-extract](adr/stage2-s2-05-mir-opt-extract.md) · S2-I169
- [stage2-s2-05-mir-module-index-extract](adr/stage2-s2-05-mir-module-index-extract.md) · S2-I170
- [stage2-s2-05-mir-lower-ctx-extract](adr/stage2-s2-05-mir-lower-ctx-extract.md) · S2-I171
- [stage2-s2-05-mir-lower-expr-literal-extract](adr/stage2-s2-05-mir-lower-expr-literal-extract.md) · S2-I172
- [stage2-s2-05-mir-lower-expr-call-ident-extract](adr/stage2-s2-05-mir-lower-expr-call-ident-extract.md) · S2-I173
- [stage2-s2-05-mir-lower-expr-call-member-extract](adr/stage2-s2-05-mir-lower-expr-call-member-extract.md) · S2-I174
- [stage2-s2-05-mir-lower-expr-member-extract](adr/stage2-s2-05-mir-lower-expr-member-extract.md) · S2-I175
- [stage2-s2-05-mir-lower-expr-member-assign-extract](adr/stage2-s2-05-mir-lower-expr-member-assign-extract.md) · S2-I176
- [stage2-s2-05-mir-lower-expr-index-assign-extract](adr/stage2-s2-05-mir-lower-expr-index-assign-extract.md) · S2-I177
- [stage2-s2-05-mir-lower-expr-binary-extract](adr/stage2-s2-05-mir-lower-expr-binary-extract.md) · S2-I178
- [stage2-s2-05-mir-lower-expr-unary-extract](adr/stage2-s2-05-mir-lower-expr-unary-extract.md) · S2-I179
- [stage2-s2-05-mir-lower-expr-closure-extract](adr/stage2-s2-05-mir-lower-expr-closure-extract.md) · S2-I180
- [stage2-s2-05-mir-lower-expr-index-extract](adr/stage2-s2-05-mir-lower-expr-index-extract.md) · S2-I181
- [stage2-s2-05-mir-lower-expr-task-block-extract](adr/stage2-s2-05-mir-lower-expr-task-block-extract.md) · S2-I182
- [stage2-s2-05-mir-lower-expr-compound-assign-extract](adr/stage2-s2-05-mir-lower-expr-compound-assign-extract.md) · S2-I183
- [stage2-s2-05-mir-lower-expr-ternary-to-is-batch](adr/stage2-s2-05-mir-lower-expr-ternary-to-is-batch.md) · S2-I184–I188
- [stage2-s2-05-mir-lower-expr-call-to-macro-batch](adr/stage2-s2-05-mir-lower-expr-call-to-macro-batch.md) · S2-I189–I194
- [stage2-s2-05-mir-lower-expr-range-to-super-batch](adr/stage2-s2-05-mir-lower-expr-range-to-super-batch.md) · S2-I195–I200
- [stage2-s2-05-mir-free-vars-to-shells-batch](adr/stage2-s2-05-mir-free-vars-to-shells-batch.md) · S2-I201–I204
- [stage2-s2-05-mir-type-members-to-infer-batch](adr/stage2-s2-05-mir-type-members-to-infer-batch.md) · S2-I205–I208
- [stage2-s2-05-mir-simd-to-support-batch](adr/stage2-s2-05-mir-simd-to-support-batch.md) · S2-I209–I212
- [stage2-s2-05-mir-lower-expr-to-module-batch](adr/stage2-s2-05-mir-lower-expr-to-module-batch.md) · S2-I213–I214
- [stage2-s2-05-driver-fingerprint-to-lsp-batch](adr/stage2-s2-05-driver-fingerprint-to-lsp-batch.md) · S2-I215–I217
- [stage2-s2-05-driver-mir-unify-to-compile-batch](adr/stage2-s2-05-driver-mir-unify-to-compile-batch.md) · S2-I218–I220
- [stage2-s2-05-driver-compile-ctx-to-native-batch](adr/stage2-s2-05-driver-compile-ctx-to-native-batch.md) · S2-I221–I223
- [stage2-s2-05-driver-options-to-run-batch](adr/stage2-s2-05-driver-options-to-run-batch.md) · S2-I224–I226
- [stage2-s2-05-driver-format-to-wasm-batch](adr/stage2-s2-05-driver-format-to-wasm-batch.md) · S2-I227–I229
- [stage2-s2-05-driver-compile-to-debug-batch](adr/stage2-s2-05-driver-compile-to-debug-batch.md) · S2-I230–I232
- [stage2-s2-05-driver-production-closeout-batch](adr/stage2-s2-05-driver-production-closeout-batch.md) · S2-I233–I235
- [plan/02](../plan/02-improvement-roadmap.md) S2-05

