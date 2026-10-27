# LSP editor interop (W926–W928 / ADR-1270–1272)

## Status

**C-07 LSP editor interop narrow charter met** (ADR-1272 / W928): checklist SSOT below is complete; product inventory signs **`c07_ga_claimed=true`**; roadmap C-07 **`[x]`**. Prior floors: W926 first-slice（ADR-1270）+ W927 deepen Helix + `initialize` smoke（ADR-1271）. Historical W926/W927 inventories keep `c07_ga_claimed=false`. HostFn count **629**; C-06 **`[x]`**；Math 增强轨 **维护**；F-07 **`[x]`**；F-08 **仍冻**；≠ Q-T2 全绿 / ≠ marketplace / ≠ GUI E2E / ≠ LSP phase audit 抬楼.

Fingerprint: `seven-hundred-and-seventy-eighth` / `A-W928`（prior deepen：`seven-hundred-and-seventy-seventh` / `A-W927`；first-slice：`seven-hundred-and-seventy-sixth` / `A-W926`）。

## Checklist (narrow charter · hard)

| # | Item | Evidence |
|---|------|----------|
| 1 | VS Code client launches `syx lsp` | `editors/vscode-syx` |
| 2 | Second client (Neovim) configures `syx lsp` | `editors/neovim-syx` |
| 3 | Inventory signs interop first-slice | `LspEditorInteropFirstSliceInventory` / `lsp_editor_interop_first_slice_inventory` |
| 4 | Third client (Helix) configures `syx lsp` | `editors/helix-syx` |
| 5 | stdio `initialize` Content-Length smoke | `lsp_stdio_initialize_smoke` |
| 6 | Inventory signs interop deepen | `LspEditorInteropDeepenInventory` / `lsp_editor_interop_deepen_inventory` |
| 7 | Product claimed + roadmap C-07 `[x]` | `LspEditorInteropGaClaimedInventory` / `lsp_editor_interop_ga_claimed_inventory` |

## Index

| Prior | Status |
|-------|--------|
| VS Code extension | **met**（既有） |
| Neovim thin config | **met**（W926 / ADR-1270） |
| Helix thin config | **met**（W927 / ADR-1271） |
| stdio `initialize` smoke | **met**（W927 / ADR-1271） |
| This window C-07 **claimed** (narrow) | **met**（W928 / ADR-1272） |
| C-07 `[x]` | **met**（narrow） |
| Q-T2 / marketplace / GUI E2E | **still unmet**（honest） |

## Behavior (hard)

Clients speak LSP over stdio with command `syx lsp`（或 workspace `target/debug/syx lsp`）. Narrow charter = **≥2 editors（含 Helix）+ initialize protocol smoke + claimed inventory**；不要求 Q-T2 全绿、marketplace 或 GUI editor E2E。

## Contract (hard)

| Item | Meaning |
|------|---------|
| First-slice inventory | `LspEditorInteropFirstSliceInventory` / `lsp_editor_interop_first_slice_inventory` — `lsp_editor_interop_first_slice_signed=true`；historical **`c07_ga_claimed=false`** |
| Deepen inventory | `LspEditorInteropDeepenInventory` / `lsp_editor_interop_deepen_inventory` — `initialize_smoke_signed=true`；`helix_client_config_present=true`；historical **`c07_ga_claimed=false`** |
| Claimed inventory | `LspEditorInteropGaClaimedInventory` / `lsp_editor_interop_ga_claimed_inventory` |
| Flags | `lsp_editor_interop_ga_claimed_signed=true`; `deepen_floor_ok=true`; **`c07_ga_claimed=true`** |
| Forbidden claim | Q-T2 met / marketplace publish KPI / GUI E2E / HostFn+1 / Math llvm GA / F-08 / C-08 |
| Honesty | claimed **≠** Q-T2 / **≠** marketplace / **≠** GUI E2E |

## Evidence

- [`editors/vscode-syx`](../../editors/vscode-syx)、[`editors/neovim-syx`](../../editors/neovim-syx)、[`editors/helix-syx`](../../editors/helix-syx)
- [`crates/syx-lsp/src/lib.rs`](../../crates/syx-lsp/src/lib.rs) inventories
- Behavioral floor: `lsp_stdio_initialize_smoke.rs`（construction first/deepen/claimed meta retired）

## Frozen

- Claiming Q-T2 full green / marketplace / GUI editor E2E this window
- Opening F-08 / C-08 / C-11 / Math HostFn surface
- Mutating historical W926/W927 `c07_ga_claimed=false` nails

## Next

W928 C-07 LSP interop claimed + `[x]` **已交**（ADR-1272）→ W929–W931 C-11 profiles/sdk **已交**（ADR-1273–1275；C-11 `[x]`）→ W932–W934 C-08 `syx test` **已交**（ADR-1276–1278；C-08 `[x]`）→ W935 C-09 bench entry **已交**（ADR-1279；C-09 `[~]`）→ W936+；F-08 仍冻
