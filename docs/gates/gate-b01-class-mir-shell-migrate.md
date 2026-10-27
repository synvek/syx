# B-01 Class→MIR shell migrate (W471 / ADR-815)

## Status

**B-01 Class MIR shell migrate met**: top-level `Decl::Class` init/method/deinit bodies are authoritative from MIR `lower_module` / `lower_type_members` / `lower_class_init`, and AST `compile_type_shells` skips Class. `collect_type_decls` still indexes `class_types` / `class_vtables` / `class_superclasses`. Actor migrated later (W472). Residual AST type-shell Decl clusters **13→12** at charter (live **11** after Actor). Roadmap B-01 **stays `[~]`** — this window does **not** claim B-01 `[x]` or delete `compile_type_shells`. HostFn count **554**; B-02 thin+deepen **met** still `[~]`; B-03～B-08 **`[x]`**; ≠ `Decl::Export` / ≠ daemon GA / ≠ MIR blob delete.

Fingerprint: `three-hundred-and-twenty-first` / `A-W471`.

## Index

| Prior | Status |
|-------|--------|
| W463 B-01 MIR-consumer thin-open | **met** |
| W464 B-01 MIR-consumer deepen thin | **met** |
| W467 Struct→MIR shell migrate | **met** |
| W468 FinalClass→MIR shell migrate | **met** |
| W469 VarGlobal→MIR `__init_global` migrate | **met** |
| W470 Extension→MIR shell migrate | **met** |
| This window Class→MIR shell migrate | **met** |
| Remaining active AST shell (Actor) | **met** (W472；see [`gate-b01-actor-mir-shell-migrate.md`](gate-b01-actor-mir-shell-migrate.md)) |
| B-01 `[x]` / AST codegen migration complete | **still unmet** |
| B-02 dual-track retired | **still unmet** (`[~]`) |

## Migrate ladder

| Step | Meaning | Status |
|------|---------|--------|
| 1. Thin-open | MIR primary + AST type-shell honesty | **met** (W463) |
| 2. Deepen thin / residual cluster index | residual table | **met** (W464) |
| 3. Migrate Struct | Struct→MIR | **met** (W467) |
| 4. Migrate FinalClass | FinalClass→MIR | **met** (W468) |
| 5. Migrate VarGlobal | VarGlobal→MIR combined `__init_global` | **met** (W469) |
| 6. Migrate Extension | Extension→MIR AST-skip | **met** (W470) |
| 7. Migrate Class | **Class→MIR AST-skip** | **met** (W471 / ADR-815) |
| 8. Migrate Actor | Actor→MIR | **met** (W472 / ADR-816) |
| 9. B-01 `[x]` | axis exit | **unmet** |

## Gap table (hard)

| Gap | Present? | Meaning |
|-----|----------|---------|
| Class members via MIR `lower_type_members` / `lower_class_init` | **yes** | No new `lower_class_shells` API |
| Class skipped in `compile_type_shells` | **yes** | No duplicate AST class shells |
| Class index preserved | **yes** | `class_types` / `class_vtables` / `class_superclasses` |
| Actor still AST-shelled | **no** | Actor migrated W472 |
| Residual AST shell cluster count | **11** (live) | Was 12 at Class charter; live after Actor |
| AST codegen migration complete | **no** | 11 no-op clusters remain |
| B-01 axis exit (`[x]`) | **no** | migrate one ≠ exit |

## Contract (hard)

| Item | Meaning |
|------|---------|
| Inventory | `GateB01ClassMirShellMigrateInventory` / `gate_b01_class_mir_shell_migrate_inventory` |
| Flags | `b01_class_mir_shell_migrate_signed=true`; `extension_migrate_floor_ok=true`; `class_mir_path_present=true`; `class_ast_shell_skipped=true`; `class_index_preserved=true`; `residual_ast_shell_cluster_count=12`; `ast_codegen_migration_complete=false`; `b01_axis_exit_enabled=false`; `type_shell_delete_enabled=false` |
| Forbidden claim | B-01 `[x]` / clear all shells / Actor migrate / `Decl::Export` / B-02 retired / daemon GA / MIR blob re-embed / HostFn+1 |
| Honesty | Class migrate **≠** B-01 `[x]` / **≠** residual zero / **≠** Actor migrate |

## Evidence

- [`crates/syx-mir/src/lib.rs`](../../crates/syx-mir/src/lib.rs) `lower_module` → `Decl::Class` → `lower_type_members` / `lower_class_init`
- [`crates/syx-codegen/src/lib.rs`](../../crates/syx-codegen/src/lib.rs) `compile_type_shells` skip Class; `B01_RESIDUAL_AST_SHELL_CLUSTER_COUNT=12`
- Meta: `gate_b01_class_mir_shell_migrate.rs`
- Floor: [`gate-b01-extension-mir-shell-migrate.md`](gate-b01-extension-mir-shell-migrate.md) / [`gate-b01-mir-consumer-deepen-thin.md`](gate-b01-mir-consumer-deepen-thin.md)
- Cross: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) B-01, [`plan/topics/mir-aot.md`](../../plan/topics/mir-aot.md)

## Frozen

- Migrating Actor this window
- Claiming B-01 `[x]` / residual zero / deleting `compile_type_shells`
- Import/TypeAlias/Protocol no-op “migrate”
- `TokenKind::Export` / `Decl::Export` / parser `export`
- B-02 retire / delete `compile` / B-02 `[x]`
- Claiming daemon GA / MIR blob re-embed / HostFn+1
- Editing `.cursor/plans/wave_a_w449_*.plan.md` … `wave_a_w470_*.plan.md`

## Next

W471 B-01 Class MIR shell migrate **已交**（ADR-815）→ W472 B-01 Actor MIR shell migrate **已交**（ADR-816）→ W473 B-01 no-op residual honesty **已交**（ADR-817）→ W474 B-01 MIR-consumer narrow `[x]` exit **已交**（ADR-818）→ W475 B-02 retire-readiness thin **已交**（ADR-819）→ W476 B-09 GC narrative thin **已交**（ADR-820）→ W477 B-02 compile_into 首片 **已交**（ADR-821）→ W478 B-02 严 deprecate gate **已交**（ADR-822）→ W479 B-02 窄 `[x]` exit **已交**（ADR-823）→ W480 Decl::Export 首片 **已交**（ADR-824）→ W481 semantic→`aot_export_roots` **已交**（ADR-825）→ W482 删 `compile`/`compile_into*` **已交**（ADR-826）→ W483 `export {…}` 列表首片 **已交**（ADR-827）→ W484 residual ABI 窄清理 **已交**（ADR-828）→ W485 B-09 GC narrative deepen **已交**（ADR-829）→ W486 `export {…}` 尾逗号 **已交**（ADR-830）→ W487+ — 可选 B-09 统一堆真施工；或另钉扫全 auto；或 exports GA
