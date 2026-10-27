# Language exports exports-GA evidence first-slice (W924 / ADR-1268)

## Status

**Language exports exports-GA evidence first-slice met** (ADR-1268 / W924): language-face roots from scan-all / `export` drive W440 default [`llvm_emit_function_names`](../../crates/syx-mir/src/lib.rs). W923 scan-all floor still holds. **Full `exports_ga_claimed`** and **F-07 `[x]`** remain **off** and are **honestly signed**. HostFn count **629**; C-06 **`[x]`**; F-07 **`[~]`**（证据窄片，≠ `[x]`）；≠ wasm-gc / ≠ F-08 / ≠ Track L / ≠ Math llvm GA / ≠ compact GA.

Fingerprint: `seven-hundred-and-seventy-fourth` / `A-W924`.

## Index

| Prior | Status |
|-------|--------|
| W923 scan-all auto-discovery | **met** (`public func` → roots) |
| W440 default multi-root emit | **met** (`llvm_emit_function_names` reads roots) |
| This window exports GA **evidence** | **met** (public roots → default emit) |
| Full exports GA (`exports_ga_claimed`) | **still unmet** |
| F-07 `[x]` | **still unmet**（本窗仍 `[~]`） |

## Behavior (hard)

```syx
public func greet() {
    print("hi")
}
func helper() {}
func main() {
    print("main")
}
```

After `lower_module`, `MirModule.aot_export_roots` contains `"greet"`. Default `llvm_emit_function_names` includes `greet` (and reachable names from that root). A clone with **cleared** `aot_export_roots` remains **main-only** and does **not** include orphan `greet`. Compact GA is **not** claimed this window.

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `LanguageExportsExportsGaEvidenceFirstSliceInventory` / `language_exports_exports_ga_evidence_first_slice_inventory` |
| Flags | `language_exports_exports_ga_evidence_first_slice_signed=true`; `scan_all_auto_discovery_floor_ok=true`; `default_multi_root_emit_floor_ok=true`; **`public_func_roots_drive_default_emit=true`**; **`exports_ga_claimed=false`**; **`f07_ga_claimed=false`** |
| Honesty floor | `LanguageExportsScanAllAutoDiscoveryFirstSliceInventory` keeps `exports_ga_claimed=false` / `f07_ga_claimed=false` |
| Forbidden claim | full exports GA / F-07 `[x]` / wasm-gc / F-08 RT ABI / Track L / HostFn+1 / Math llvm GA / compact product GA |
| Honesty | evidence first-slice **≠** `exports_ga_claimed=true` / **≠** F-07 complete |

## Evidence

- [`crates/syx-mir/src/lib.rs`](../../crates/syx-mir/src/lib.rs) inventory + unit (`public_func_roots_drive_llvm_emit_function_names_exports_ga_evidence`)
- Construction meta retired（`syx-mir` unit: `public_func_roots_drive_llvm_emit_function_names_exports_ga_evidence`）
- Floor: [`language-exports-scan-all-auto-discovery-first-slice.md`](language-exports-scan-all-auto-discovery-first-slice.md), W440 default multi-root
- Cross: [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Claiming `exports_ga_claimed=true` / F-07 `[x]` / wasm-gc / F-08 RT ABI full clear
- Track L (multiline / tuple / …) as this window’s charter
- Changing emit thresholds / HostFn+1 / Math HostFn surface
- Editing historical language-exports inventories to flip `exports_ga_claimed=true`
- Claiming compact-blob product GA from this evidence nail

## Next

W924 exports GA evidence first-slice **已交**（ADR-1268）→ W925 exports GA claimed + F-07 `[x]` **已交**（ADR-1269）→ W926+（F-08 仍冻 / 或 C-07）；扫全 deepen / compact 另钉
