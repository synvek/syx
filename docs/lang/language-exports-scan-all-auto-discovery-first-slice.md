# Language exports scan-all auto-discovery first-slice (W923 / ADR-1267)

## Status

**Language exports scan-all auto-discovery first-slice met** (ADR-1267 / W923): `lower_module` fills `MirModule.aot_export_roots` from top-level **`public func`** (alongside existing `export func` / `export {…}`). W486 trailing-comma floor still holds (inventory historical `language_exports_auto_discovery_enabled=false`). **Exports GA** and **F-07 `[x]`** remain **off** and are **honestly signed**. HostFn count **629**; C-06 **`[x]`**; F-07 **`[~]`**（开窗，≠ `[x]`）；≠ wasm-gc / ≠ F-08 / ≠ Track L / ≠ Math llvm GA.

Fingerprint: `seven-hundred-and-seventy-third` / `A-W923`.

## Index

| Prior | Status |
|-------|--------|
| W460–W486 language exports ladder | **met** (thin → Decl::Export → semantic → list → trailing) |
| This window scan-all auto-discovery | **met** (`public func` → roots) |
| Exports GA | **still unmet** |
| F-07 `[x]` | **still unmet**（本窗仅 `[~]`） |

## Behavior (hard)

```syx
public func greet() {
    print("hi")
}
func helper() {}
func main() {
    greet()
}
```

After `lower_module`, `MirModule.aot_export_roots` contains `"greet"`. Bare/`internal` `func helper` and `main` are **not** added. Empty public set and no `export` ⇒ empty roots ⇒ main-only emit (W440 compatible). `export func` / `export {…}` paths unchanged (dedupe).

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `LanguageExportsScanAllAutoDiscoveryFirstSliceInventory` / `language_exports_scan_all_auto_discovery_first_slice_inventory` |
| Flags | `language_exports_scan_all_auto_discovery_first_slice_signed=true`; `export_list_trailing_comma_floor_ok=true`; **`language_exports_auto_discovery_enabled=true`**; **`exports_ga_claimed=false`**; **`f07_ga_claimed=false`** |
| Honesty floor | `LanguageExportsExportListTrailingCommaInventory` keeps `language_exports_auto_discovery_enabled=false` (historical) |
| Forbidden claim | exports GA / F-07 `[x]` / wasm-gc / F-08 RT ABI / Track L / HostFn+1 / Math llvm GA |
| Honesty | scan-all auto **≠** exports GA / **≠** F-07 complete |

## Evidence

- [`crates/syx-mir/src/lib.rs`](../../crates/syx-mir/src/lib.rs) `lower_module` Public→roots + inventory
- Construction meta retired（`syx-mir` unit: `lower_module_public_func_fills_aot_export_roots_scan_all`）
- Floor: [`language-exports-export-list-trailing-comma.md`](language-exports-export-list-trailing-comma.md), [`language-exports-semantic-aot-export-roots.md`](language-exports-semantic-aot-export-roots.md)
- Cross: [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Claiming `exports_ga_claimed=true` / F-07 `[x]` / wasm-gc / F-08 RT ABI full clear
- Track L (multiline / tuple / …) as this window’s charter
- Changing emit thresholds / HostFn+1 / Math HostFn surface
- Editing historical language-exports inventories to flip auto=true

## Next

W923 scan-all auto-discovery first-slice **已交**（ADR-1267）→ W924 exports GA evidence **已交**（ADR-1268）→ W925 exports GA claimed + F-07 `[x]` **已交**（ADR-1269）→ W926+（F-08 仍冻 / 或 C-07）；扫全 deepen（class/method / generics）另钉
