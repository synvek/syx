# C-11 profiles / syx-sdk (W929–W931 · claimed + `[x]`)

## Status

**C-11 profiles / syx-sdk claimed met** (ADR-1275 / W931；floor ADR-1273 / W929 + ADR-1274 / W930): profile matrix documented；flat install + **`SYX_RT_STATIC_LIB`**；**`SYX_RT_PROFILE`** multi-profile lookup；薄 packaging 布局；claimed inventory。HostFn count **629**; C-06 **`[x]`**；C-07 **`[x]`**；C-11 **`[x]`**（窄）；Math 增强轨 **维护**；F-07 **`[x]`**；F-08 **仍冻**；≠ Release 全 profile 矩阵 CI 上传 / ≠ marketplace SDK KPI / ≠ C-08.

Fingerprint: `seven-hundred-and-eighty-first` / `A-W931`（floor W930：`seven-hundred-and-eightieth` / `A-W930`；W929：`seven-hundred-and-seventy-ninth` / `A-W929`）。

## Checklist (narrow charter · hard)

| # | Item | Evidence |
|---|------|----------|
| 1 | Profile matrix documented | [`plan/topics/profiles-sdk.md`](../../plan/topics/profiles-sdk.md) |
| 2 | Install layout lookup (`../lib`) | [`crates/syx-rt/src/link.rs`](../../crates/syx-rt/src/link.rs) |
| 3 | Runtime `SYX_RT_STATIC_LIB` override | `runtime_env_rt_archive` in link.rs |
| 4 | Inventory signs first-slice | `ProfilesSdkFirstSliceInventory` |
| 5 | Multi-profile install lookup | `runtime_feature_profile_rt_archive` / `SYX_RT_PROFILE` |
| 6 | Packaging layout documented | [`packaging/syx-sdk/README.md`](../../packaging/syx-sdk/README.md) |
| 7 | Inventory signs deepen | `ProfilesSdkDeepenInventory` |
| 8 | Claimed + roadmap `[x]` | `ProfilesSdkGaClaimedInventory`；plan/02 C-11 **`[x]`** |

## Index

| Prior | Status |
|-------|--------|
| Compile-time embed `env!("SYX_RT_STATIC_LIB")` | **met**（既有） |
| Install `bin/../lib` lookup | **met**（W929） |
| Runtime env override | **met**（W929） |
| Multi-profile `SYX_RT_PROFILE` lookup | **met**（W930） |
| Packaging layout README | **met**（W930） |
| C-11 `[~]` open / deepen | **met**（W929–W930） |
| C-11 `[x]` narrow claimed | **met**（W931） |
| Release full-matrix CI / marketplace | **still unmet** |

## Behavior (hard)

`rt_archive_path` / `rt_archive_path_for_profile` resolve order：

1. Runtime env `SYX_RT_STATIC_LIB`（若指向存在的 `.a`/`.lib` 或 deps 目录）
2. Runtime env `SYX_RT_PROFILE=<feature-id>` → `../lib/profiles/<id>/libsyx_rt.a`，回退 `../lib/<id>/libsyx_rt.a`（及 `.lib`）
3. Compile-time embed path
4. Install layout relative to `syx` exe (`../lib` flat)
5. Cargo `target/{debug,release}` / deps search

Narrow claimed = **checklist + claimed inventory + roadmap `[x]`** after W929–W930 floors；不要求本窗发布多 profile Release 资产包 / CI 上传。

## Contract (hard)

| Item | Meaning |
|------|---------|
| First-slice inventory | `ProfilesSdkFirstSliceInventory` / `profiles_sdk_first_slice_inventory` |
| Deepen inventory | `ProfilesSdkDeepenInventory` / `profiles_sdk_deepen_inventory` |
| Claimed inventory | `ProfilesSdkGaClaimedInventory` / `profiles_sdk_ga_claimed_inventory` |
| Flags (first) | `profiles_sdk_first_slice_signed=true`; …; **`c11_ga_claimed=false`** |
| Flags (deepen) | `profiles_sdk_deepen_signed=true`; …; **`c11_ga_claimed=false`** |
| Flags (claimed) | `deepen_floor_ok=true`; **`c11_ga_claimed=true`**; `profiles_sdk_ga_claimed_signed=true` |
| Forbidden claim | Release full profile matrix CI upload / marketplace SDK KPI / C-08 / F-08 / HostFn+1 / Math llvm GA |
| Honesty | narrow `[x]` **≠** Release 资产已发货 / **≠** multi-platform CI matrix |

## Evidence

- [`plan/topics/profiles-sdk.md`](../../plan/topics/profiles-sdk.md)
- [`packaging/syx-sdk/README.md`](../../packaging/syx-sdk/README.md)
- [`crates/syx-rt/src/link.rs`](../../crates/syx-rt/src/link.rs)
- Construction first/deepen/claimed meta retired（`syx-rt` link profile unit tests + inventories remain）

## Frozen

- Claiming Release full-matrix CI upload / marketplace SDK this window
- Opening C-08 / F-08 / Math HostFn surface
- Rewriting historical W929/W930 inventory `c11_ga_claimed` to true

## Next

W931 C-11 profiles/sdk claimed + `[x]` **已交**（ADR-1275）→ W932–W934 C-08 `syx test` first→deepen→claimed **已交**（ADR-1276–1278；C-08 `[x]`）→ W935 C-09 bench entry first-slice **已交**（ADR-1279；C-09 `[~]`）→ W936+；F-08 仍冻
