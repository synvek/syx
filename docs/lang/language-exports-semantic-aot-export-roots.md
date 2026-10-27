# Language exports semantic→`aot_export_roots` (W481 / ADR-825)

## Status

**Language exports semantic→`aot_export_roots` narrow slice met** (ADR-825 / W481): `lower_module` fills `MirModule.aot_export_roots` from `Decl::Export` function names. W480 Decl::Export first-slice floor still holds (inventory historical `semantic_fills_aot_export_roots=false`). Auto-discovery and exports GA remain **off** and are **honestly signed**. Live `export { … }` list landed in **W483** (see [`language-exports-export-list-first-slice.md`](language-exports-export-list-first-slice.md)); W481 inventory/history still treats list as out-of-scope for that window. HostFn count **554**; B-02 **`[x]`** (narrow); B-09 **`[~]`**; ≠ auto-discovery / ≠ exports GA / ≠ MIR blob / ≠ B-09 `[x]`.

Fingerprint: `three-hundred-and-thirty-first` / `A-W481`.

## Index

| Prior | Status |
|-------|--------|
| W460 language exports thin-open | **met** (honesty floor) |
| W462 language exports syntax-spec deepen | **met** (honesty floor) |
| W480 Decl::Export first-slice | **met** (honesty floor) |
| This window semantic→`aot_export_roots` | **met** |
| Auto-discovery | **still unmet** |
| Exports GA | **still unmet** |

## Behavior (hard)

```syx
export func greet() {
    print("hi")
}
func main() {
    greet()
}
```

After `lower_module`, `MirModule.aot_export_roots` contains `"greet"`. Empty export set ⇒ empty roots ⇒ main-only emit (W440 compatible). Non-empty roots ⇒ default multi-root emit includes named functions.

**Not** scanning all `public`/`func` declarations (auto-discovery deferred).

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `LanguageExportsSemanticAotExportRootsInventory` / `language_exports_semantic_aot_export_roots_inventory` |
| Flags | `language_exports_semantic_aot_export_roots_signed=true`; `decl_export_first_slice_floor_ok=true`; `semantic_fills_aot_export_roots=true`; `language_exports_auto_discovery_enabled=false`; `exports_ga_claimed=false` |
| Honesty floor | `LanguageExportsDeclExportFirstSliceInventory` keeps `semantic_fills_aot_export_roots=false` (historical) |
| Forbidden claim | auto-discovery / exports GA / delete compile API / B-09 `[x]` / MIR blob re-embed / HostFn+1 |
| Honesty | semantic roots fill **≠** auto-discovery / **≠** exports GA |

## Evidence

- [`crates/syx-mir/src/lib.rs`](../../crates/syx-mir/src/lib.rs) `lower_module` Export→roots + `LanguageExportsSemanticAotExportRootsInventory`
- Meta: `language_exports_semantic_aot_export_roots.rs`
- Floor: [`language-exports-decl-export-first-slice.md`](language-exports-decl-export-first-slice.md), [`language-exports-syntax-spec-deepen.md`](language-exports-syntax-spec-deepen.md), [`language-exports-aot-roots-thin.md`](language-exports-aot-roots-thin.md)

## Frozen

- Auto-discovery / `export { … }` list / exports GA
- Deleting `compile`/`compile_into*` / claiming `retired=true`
- B-09 unified heap / B-09 `[x]` / GC GA
- Changing emit / absolute thresholds / HostFn+1
- Claiming daemon GA / MIR blob re-embed / production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w480_*.plan.md`

## Next

W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
