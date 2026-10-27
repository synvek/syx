# Language exports `export {…}` trailing-comma (W486 / ADR-830)

> **Status:** **`export { Name, …, }` trailing-comma signed** (ADR-830 / W486) — parser accepts optional trailing comma in `Decl::ExportList`; empty list still rejected; lower→`aot_export_roots` unchanged.  
> Cross-index: [`language-exports-export-list-first-slice.md`](language-exports-export-list-first-slice.md) / ADR-827；[`language-exports-syntax-spec-deepen.md`](language-exports-syntax-spec-deepen.md) / ADR-806.  
> Roadmap B-02 **`[x]`**；B-09 **`[~]`**. **Not** scan-all auto. **Not** `language_exports_auto_discovery_enabled=true`. **Not** exports GA. **Not** B-09 enable/`[x]`. **Not** MIR blob delete.

Fingerprint: `three-hundred-and-thirty-sixth` / `A-W486`.

## Scope narrow (ADR-830)

Trailing-comma **complete** (narrow) =
- W483 ExportList without trailing comma
- This window: `export { Ident ("," Ident)* ","? }` (non-empty)

Scan-all auto / exports GA / B-09 unified heap enable = **unmet**（本窗 W486 历史口径；scan-all 窄片见 W923 / [`language-exports-scan-all-auto-discovery-first-slice.md`](language-exports-scan-all-auto-discovery-first-slice.md)）。

## Acceptance

| Prerequisite | Status | Evidence |
|--------------|--------|----------|
| ExportList first-slice | **met** (W483 / ADR-827) | [`language-exports-export-list-first-slice.md`](language-exports-export-list-first-slice.md) |
| Trailing comma | **signed** (W486 / ADR-830) | this document + `LanguageExportsExportListTrailingCommaInventory` |

**Evidence nails:** `crates/syx-driver/tests/native/language_exports_export_list_trailing_comma.rs` + `language_exports_export_list_trailing_comma_inventory`.

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `LanguageExportsExportListTrailingCommaInventory` / `language_exports_export_list_trailing_comma_inventory` |
| Flags | `language_exports_export_list_trailing_comma_signed=true`; `export_list_first_slice_floor_ok=true`; `export_list_trailing_comma_enabled=true`; `language_exports_auto_discovery_enabled=false`; `exports_ga_claimed=false` |
| Honesty floor | W483 list inventory keeps historical fields; auto/GA false |
| Forbidden claim | scan-all auto / exports GA / B-09 `[x]` / unified heap enable / RT ABI delete / HostFn+1 / MIR distance change |
| Honesty | trailing comma **≠** scan-all auto-discovery / **≠** exports GA |

## Frozen

- Claiming `language_exports_auto_discovery_enabled=true` / scan-all `public`/`func`
- Exports GA / B-09 `[x]` / unified heap enable / GC GA
- RT ABI delete / claiming production blob delete this window
- Adding HostFn (keep 554) / claiming production-default JIT
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w485_*.plan.md`

## Next

W486 trailing-comma **已交**（ADR-830）→ … → **W923 scan-all auto-discovery first-slice 已交**（ADR-1267；`language_exports_auto_discovery_enabled=true`；exports GA 仍 unmet；F-07 `[~]`）。
