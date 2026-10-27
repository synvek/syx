# B-01 FinalClass→MIR type-shell migrate (W468 / ADR-812)

## Status

**B-01 FinalClass MIR shell migrate met**: `Decl::FinalClass` variant-init shells (`{Name}_{variant}_init`) are emitted by `lower_finalclass_shells` (DictBuild keys `value`/`variant`/`__type`; ≠ Enum `tag`/`case`) and skipped by AST `compile_type_shells`. Residual AST type-shell Decl clusters **16→15**. Roadmap B-01 **stays `[~]`** — this window does **not** claim B-01 `[x]`, migrate Class/Actor/Extension/VarGlobal shells, or delete `compile_type_shells`. HostFn count **554**; B-02 thin+deepen **met** still `[~]`; B-03～B-08 **`[x]`**; ≠ `Decl::Export` / ≠ daemon GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-eighteenth` / `A-W468`.

## Index

| Prior | Status |
|-------|--------|
| W463 B-01 MIR-consumer thin-open | **met** |
| W464 B-01 MIR-consumer deepen thin | **met** |
| W467 Struct→MIR shell migrate | **met** |
| DataClass/Enum/Struct MIR shells | **met** |
| This window FinalClass→MIR shell migrate | **met** |
| Remaining active AST shells (Class/…) | **still unmet** |
| B-01 `[x]` / AST codegen migration complete | **still unmet** |
| B-02 dual-track retired | **still unmet** (`[~]`) |

## Migrate ladder

| Step | Meaning | Status |
|------|---------|--------|
| 1. Thin-open | MIR primary + AST type-shell honesty | **met** (W463) |
| 2. Deepen thin / residual cluster index | residual table | **met** (W464) |
| 3. Migrate one type-shell cluster | **Struct→MIR** | **met** (W467 / ADR-811) |
| 4. Migrate second type-shell cluster | **FinalClass→MIR** | **met** (W468 / ADR-812) |
| 5. Migrate remaining active shells | Class / Actor / … | **unmet** |
| 6. B-01 `[x]` | axis exit | **unmet** |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| FinalClass MIR shells (`lower_finalclass_shells`) | **yes** | Variant-init DictBuild from MIR |
| FinalClass skipped in `compile_type_shells` | **yes** | No AST FinalClass shell |
| Residual AST shell cluster count | **15** | Was 16 before FinalClass |
| AST codegen migration complete | **no** | Other shells remain |
| B-01 axis exit (`[x]`) | **no** | migrate one ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB01FinalClassMirShellMigrateInventory` / `gate_b01_finalclass_mir_shell_migrate_inventory` |
| Flags | `b01_finalclass_mir_shell_migrate_signed=true`; `struct_migrate_floor_ok=true`; `finalclass_mir_shell_path_present=true`; `finalclass_ast_shell_skipped=true`; `residual_ast_shell_cluster_count=15`; `ast_codegen_migration_complete=false`; `b01_axis_exit_enabled=false`; `type_shell_delete_enabled=false` |
| Forbidden claim | B-01 `[x]` / clear all shells / `Decl::Export` / B-02 retired / daemon GA / MIR blob re-embed / HostFn+1 |
| Honesty | FinalClass migrate **≠** B-01 `[x]` / **≠** residual zero |

## Evidence

- [`crates/syx-mir/src/lib.rs`](../../crates/syx-mir/src/lib.rs) `lower_finalclass_shells`
- [`crates/syx-codegen/src/lib.rs`](../../crates/syx-codegen/src/lib.rs) `compile_with_mir` merge + `compile_type_shells` skip FinalClass; `B01_RESIDUAL_AST_SHELL_CLUSTER_COUNT=15`
- Meta: `gate_b01_finalclass_mir_shell_migrate.rs`
- Floor: [`gate-b01-struct-mir-shell-migrate.md`](gate-b01-struct-mir-shell-migrate.md) / [`gate-b01-mir-consumer-deepen-thin.md`](gate-b01-mir-consumer-deepen-thin.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-01, [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Migrating Class / Extension / Actor / VarGlobal this window
- Claiming B-01 `[x]` / residual zero / deleting `compile_type_shells`
- Folding FinalClass into `lower_enum_shells` (different DictBuild keys)
- Import/TypeAlias no-op “migrate”
- `TokenKind::Export` / `Decl::Export` / parser `export`
- B-02 retire / delete `compile` / B-02 `[x]`
- Claiming daemon GA / MIR blob re-embed / HostFn+1
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w467_*.plan.md`

## Next

W468 B-01 FinalClass MIR shell migrate **已交**（ADR-812）→ W469 B-01 VarGlobal MIR shell migrate **已交**（ADR-813）→ W470 B-01 Extension MIR shell migrate **已交**（ADR-814）→ W471 B-01 Class MIR shell migrate **已交**（ADR-815）→ W472 B-01 Actor MIR shell migrate **已交**（ADR-816）→ W473 B-01 no-op residual honesty **已交**（ADR-817）→ W474 B-01 MIR-consumer narrow `[x]` exit **已交**（ADR-818）→ W475 B-02 retire-readiness thin **已交**（ADR-819）→ W476 B-09 GC narrative thin **已交**（ADR-820）→ W477 B-02 compile_into 首片 **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
