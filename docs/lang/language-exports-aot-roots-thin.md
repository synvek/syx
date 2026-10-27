# Language exports → `aot_export_roots` thin-open (W460 / ADR-804)

## Status

**Language exports thin-open met** (policy+inventory): MIR `MirModule.aot_export_roots` exists and W440 default multi-root emit reads it (empty roots still main-only); frontend `Module.exports` syntax, semantic→roots fill, and language auto-discovery remain **off** and are **honestly signed** — **not** delivered this window. HostFn count **554**; B-04 **`[x]`** (narrow) unchanged; ≠ exports GA / ≠ frontend parser / ≠ daemon GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-tenth` / `A-W460`.

## Index

| Prior | Status |
|-------|--------|
| W326–W332 export-root readiness | **met** |
| W440 default multi-root via `aot_export_roots` | **met** |
| W446 B-04 Short `[x]` exit (narrow) | **met** |
| This window language exports thin-open | **met** |
| Frontend syntax-spec deepen (W462) | **met** (docs; see [`language-exports-syntax-spec-deepen.md`](language-exports-syntax-spec-deepen.md)) |
| Frontend lexer/AST/parser Export (W480) | **met** (see [`language-exports-decl-export-first-slice.md`](language-exports-decl-export-first-slice.md); thin inventory `frontend_export_syntax_present` still **false**) |
| Semantic fills `aot_export_roots` | **still unmet** |
| Language exports auto-discovery | **still unmet** |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| MIR `aot_export_roots` field | **yes** | W440 field + serde default |
| Default emit reads roots | **yes** | empty ⇒ main-only |
| Frontend `Module.exports` | **no** | no AST/parser Decl |
| Semantic → roots fill | **no** | no automatic discovery |
| Auto-discovery enabled | **no** | Frozen this window |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `LanguageExportsAotRootsThinInventory` / `language_exports_aot_roots_thin_inventory` |
| Flags | `language_exports_thin_signed=true`; `mir_aot_export_roots_field_present=true`; `default_multi_root_emit_wired=true`; `frontend_export_syntax_present=false`; `semantic_fills_aot_export_roots=false`; `language_exports_auto_discovery_enabled=false` |
| Forbidden claim | Auto-discovery / frontend first-slice / exports GA / daemon GA / MIR blob re-embed / HostFn+1 |
| Honesty | thin-open **≠** auto-discovery / **≠** B-04 reopen / **≠** frontend met |

## Evidence

- [`crates/syx-mir/src/lib.rs`](../../crates/syx-mir/src/lib.rs) `LanguageExportsAotRootsThinInventory`
- Meta: `language_exports_aot_roots_thin.rs`
- Cross: [`b04-default-multi-root.md`](b04-default-multi-root.md), [`b04-short-exit.md`](b04-short-exit.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Claiming thin inventory `frontend_export_syntax_present` as delivered / semantic→`aot_export_roots` automatic fill
- Claiming language exports auto-discovery / exports GA
- Changing emit / compact / absolute thresholds this window
- Deleting `syx_rt_load_mir_blob` ABI / re-embedding blob/sidecar
- Claiming daemon GA / Array alpn / adding HostFn (keep 554)
- Claiming production-default JIT

## Next

W460 language exports thin-open **已交**（ADR-804）→ W461 B-03 FrontendPipeline entry matrix **已交**（ADR-805）→ W462 language exports syntax-spec deepen **已交**（ADR-806）→ W463 B-01 MIR-consumer thin-open **已交**（ADR-807）→ W464 B-01 MIR-consumer deepen thin **已交**（ADR-808）→ W465 B-02 dual-track thin-open **已交**（ADR-809）→ W466 B-02 dual-track deepen thin **已交**（ADR-810）→ W467 B-01 Struct MIR shell migrate **已交**（ADR-811）→ W468 B-01 FinalClass MIR shell migrate **已交**（ADR-812）→ W469 B-01 VarGlobal MIR shell migrate **已交**（ADR-813）→ W470 B-01 Extension MIR shell migrate **已交**（ADR-814）→ W471 B-01 Class MIR shell migrate **已交**（ADR-815）→ W472 B-01 Actor MIR shell migrate **已交**（ADR-816）→ W473 B-01 no-op residual honesty **已交**（ADR-817）→ W474 B-01 MIR-consumer narrow `[x]` exit **已交**（ADR-818）→ W475 B-02 retire-readiness thin **已交**（ADR-819）→ W476 B-09 GC narrative thin **已交**（ADR-820）→ W477 B-02 compile_into 首片 **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487 B-09 MirRuntime 根集首片 **已交**（ADR-831）→ W488+ — 可选 VM `GcCollect` 收敛 / mailbox Value / enable / 扫全 auto / exports GA / RT ABI 全清（慎）
