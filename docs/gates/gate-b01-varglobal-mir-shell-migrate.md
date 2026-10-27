# B-01 VarGlobal→MIR `__init_global` migrate (W469 / ADR-813)

## Status

**B-01 VarGlobal MIR shell migrate met**: top-level `Decl::VarGlobal` initializers are emitted as **one** combined MIR `__init_global` via `lower_varglobal_inits` (declaration-order slots; survives `compile_from_mir` name-dedupe), `program.globals` is registered on the MIR path, and AST `compile_type_shells` skips VarGlobal. Residual AST type-shell Decl clusters **15→14**. Roadmap B-01 **stays `[~]`** — this window does **not** claim B-01 `[x]`, migrate Class/Actor/Extension shells, or delete `compile_type_shells`. HostFn count **554**; B-02 thin+deepen **met** still `[~]`; B-03～B-08 **`[x]`**; ≠ `Decl::Export` / ≠ daemon GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-nineteenth` / `A-W469`.

## Index

| Prior | Status |
|-------|--------|
| W463 B-01 MIR-consumer thin-open | **met** |
| W464 B-01 MIR-consumer deepen thin | **met** |
| W467 Struct→MIR shell migrate | **met** |
| W468 FinalClass→MIR shell migrate | **met** |
| This window VarGlobal→MIR `__init_global` migrate | **met** |
| Remaining active AST shells (Class/…) | **still unmet** |
| B-01 `[x]` / AST codegen migration complete | **still unmet** |
| B-02 dual-track retired | **still unmet** (`[~]`) |

## Migrate ladder

| Step | Meaning | Status |
|------|---------|--------|
| 1. Thin-open | MIR primary + AST type-shell honesty | **met** (W463) |
| 2. Deepen thin / residual cluster index | residual table | **met** (W464) |
| 3. Migrate Struct | Struct→MIR | **met** (W467) |
| 4. Migrate FinalClass | FinalClass→MIR | **met** (W468) |
| 5. Migrate VarGlobal | **VarGlobal→MIR combined `__init_global`** | **met** (W469 / ADR-813) |
| 6. Migrate remaining active shells | Class / Actor / Extension | **unmet** |
| 7. B-01 `[x]` | axis exit | **unmet** |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| Combined `__init_global` (`lower_varglobal_inits`) | **yes** | One MIR fn; declaration-order slots |
| `program.globals` registered on MIR path | **yes** | Skip AST arm must not drop names |
| VarGlobal skipped in `compile_type_shells` | **yes** | No AST top-level VarGlobal shell |
| Residual AST shell cluster count | **14** | Was 15 before VarGlobal |
| AST codegen migration complete | **no** | Other shells remain |
| B-01 axis exit (`[x]`) | **no** | migrate one ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB01VarGlobalMirShellMigrateInventory` / `gate_b01_varglobal_mir_shell_migrate_inventory` |
| Flags | `b01_varglobal_mir_shell_migrate_signed=true`; `finalclass_migrate_floor_ok=true`; `varglobal_mir_init_path_present=true`; `varglobal_ast_shell_skipped=true`; `program_globals_registered_on_mir_path=true`; `residual_ast_shell_cluster_count=14`; `ast_codegen_migration_complete=false`; `b01_axis_exit_enabled=false`; `type_shell_delete_enabled=false` |
| Forbidden claim | B-01 `[x]` / clear all shells / `Decl::Export` / B-02 retired / daemon GA / MIR blob re-embed / HostFn+1 / `__init_global_{name}` / VM init rewrite |
| Honesty | VarGlobal migrate **≠** B-01 `[x]` / **≠** residual zero |

## Evidence

- [`crates/syx-mir/src/lib.rs`](../../crates/syx-mir/src/lib.rs) `lower_varglobal_inits`
- [`crates/syx-codegen/src/lib.rs`](../../crates/syx-codegen/src/lib.rs) `compile_with_mir` globals register + merge; `compile_type_shells` skip VarGlobal; `B01_RESIDUAL_AST_SHELL_CLUSTER_COUNT=14`
- Meta: `gate_b01_varglobal_mir_shell_migrate.rs`
- Floor: [`gate-b01-finalclass-mir-shell-migrate.md`](gate-b01-finalclass-mir-shell-migrate.md) / [`gate-b01-mir-consumer-deepen-thin.md`](gate-b01-mir-consumer-deepen-thin.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-01, [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Migrating Class / Extension / Actor this window
- Claiming B-01 `[x]` / residual zero / deleting `compile_type_shells`
- Unique names `__init_global_{name}` / changing VM/async init loops
- Import/TypeAlias/Protocol no-op “migrate”
- `TokenKind::Export` / `Decl::Export` / parser `export`
- B-02 retire / delete `compile` / B-02 `[x]`
- Claiming daemon GA / MIR blob re-embed / HostFn+1
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w468_*.plan.md`

## Next

W469 B-01 VarGlobal MIR shell migrate **已交**（ADR-813）→ W470 B-01 Extension MIR shell migrate **已交**（ADR-814）→ W471 B-01 Class MIR shell migrate **已交**（ADR-815）→ W472 B-01 Actor MIR shell migrate **已交**（ADR-816）→ W473 B-01 no-op residual honesty **已交**（ADR-817）→ W474 B-01 MIR-consumer narrow `[x]` exit **已交**（ADR-818）→ W475 B-02 retire-readiness thin **已交**（ADR-819）→ W476 B-09 GC narrative thin **已交**（ADR-820）→ W477 B-02 compile_into 首片 **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
