# Language exports exports-GA (narrow charter) (W925 / ADR-1269)

## Status

**Language exports exports-GA narrow charter met** (ADR-1269 / W925): checklist SSOT below is complete; product inventory signs **`exports_ga_claimed=true`** and **`f07_ga_claimed=true`**; roadmap F-07 **`[x]`**. Historical W923/W924 inventories keep `claimed=false`. HostFn count **629**; C-06 **`[x]`**; Math 增强轨 **维护**；≠ scan-all class/method/generics deepen / ≠ compact product GA / ≠ Track L / ≠ wasm-gc / ≠ F-08 / ≠ Math llvm GA.

Fingerprint: `seven-hundred-and-seventy-fifth` / `A-W925`.

## Checklist (narrow charter · hard)

| # | Item | Evidence |
|---|------|----------|
| 1 | `export func` / `export {…}` / trailing → `aot_export_roots` | W480–W486 |
| 2 | scan-all：`public func` → roots | W923 / ADR-1267 |
| 3 | 空 roots ⇒ main-only emit | W440 |
| 4 | 非空语言面 roots ⇒ default `llvm_emit_function_names` 含该 root | W924 / ADR-1268 |

## Index

| Prior | Status |
|-------|--------|
| W923 scan-all auto-discovery | **met** |
| W924 exports GA evidence (emit) | **met** |
| This window exports GA **claimed** (narrow) | **met** |
| F-07 `[x]` | **met** |
| Scan-all class/method/generics deepen | **still unmet**（honest） |
| Compact product GA | **still unmet**（honest） |

## Behavior (hard)

Narrow charter = checklist above. Example orphan public root still drives default emit (W924 nail); empty roots remain main-only (W440).

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `LanguageExportsExportsGaClaimedInventory` / `language_exports_exports_ga_claimed_inventory` |
| Flags | `language_exports_exports_ga_claimed_signed=true`; `exports_ga_evidence_floor_ok=true`; **`exports_ga_claimed=true`**; **`f07_ga_claimed=true`** |
| Honesty floor | `LanguageExportsExportsGaEvidenceFirstSliceInventory` keeps `exports_ga_claimed=false` / `f07_ga_claimed=false`（historical） |
| Forbidden claim | scan-all class/method/generics deepen / compact product GA / Track L / wasm-gc / F-08 RT ABI / HostFn+1 / Math llvm GA / C-07 |
| Honesty | narrow exports GA **≠** widen deepen / **≠** F-08 / **≠** wasm-gc |

## Evidence

- [`crates/syx-mir/src/lib.rs`](../../crates/syx-mir/src/lib.rs) claimed inventory
- Construction claimed meta retired（inventory + `syx-mir` emit units remain）
- Floor: [`language-exports-exports-ga-evidence-first-slice.md`](language-exports-exports-ga-evidence-first-slice.md), [`language-exports-scan-all-auto-discovery-first-slice.md`](language-exports-scan-all-auto-discovery-first-slice.md)
- Cross: [`plan/topics/gc-parity.md`](../../plan/topics/gc-parity.md), [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Claiming scan-all class/method/generics deepen as this window’s charter
- Claiming compact-blob product GA / Track L / wasm-gc / F-08 RT ABI full clear
- Editing historical language-exports inventories to flip their `exports_ga_claimed=true`
- Changing emit thresholds / HostFn+1 / Math HostFn surface / opening C-07

## Next

W925 exports GA claimed + F-07 `[x]` **已交**（ADR-1269）→ W926–W928 C-07 LSP interop **已交**（ADR-1270–1272；C-07 `[x]`）→ W929–W931 C-11 first→deepen→claimed **已交**（ADR-1273–1275；C-11 `[x]`）→ W932–W934 C-08 first→deepen→claimed **已交**（ADR-1276–1278；C-08 `[x]`）→ W935 C-09 bench entry **已交**（ADR-1279；C-09 `[~]`）→ W936+；扫全 deepen / compact / F-08 另钉
