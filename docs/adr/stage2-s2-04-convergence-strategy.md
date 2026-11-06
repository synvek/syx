# ADR: Stage-2 S2-04 convergence strategy (S2-I144)

**Status:** Accepted (S2-I144) · S2-04 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I143 S2-04 open](stage2-s2-04-open.md) published the dual-track [parser census](../stage2-parser-census.md), the next ladder step is to **lock a convergence strategy** before migration knives. The primary honesty debt is fmt **write** (`format_file` → AST `format_source`) vs **`--check`** (`format_check_path` → `format_dry_run` → `format_source_auto` CST-first). Today's CST path concatenates lexemes and **does not reindent**; flipping write to CST-first would break user expectations for `syx format` layout. This window decides SSOT order only — it does **not** change `format_file`.

## Decision

1. **First convergence axis = fmt tool surface:** unify **write / `--check`** semantics and entry points **before** compile CST→AST lower, AST+trivia, or deleting `syx-cst`.
2. **Target SSOT entry** for tools = lossless-preferring path (today's implementation face = `format_source_auto` family). **CST reindent GA** landed in **S2-I150**; **write migration** landed in **S2-I151** (`format_file` → `format_dry_run` / auto). `SYX_FMT_AST=1` remains the AST escape hatch.
3. **Hard prerequisite (met by S2-I150; write migrated in S2-I151):** switching `format_file` onto auto required CST **reindent** while preserving comments/trivia — both now true for the fmt tool surface.
4. **Later axes (subsequent windows):** deepen CST coverage → migrate write onto SSOT → LSP/lint share tree → stop dual-maintaining `syx-cst` grammar → dedicated **S2-04 `[x]`** claim.
5. **Non-claims (hard):** ≠ S2-04 `[x]` / ≠ change `format_file` this window / ≠ delete `syx-cst` / ≠ change `FrontendPipeline` / ≠ land rowan / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open S2-05…07 construction.

## Evidence

| Check | Evidence |
|-------|----------|
| Strategy locked | this ADR |
| Write migrated (I151) | `s2_04_fmt_write_check_strategy` (driver `format_file` → `format_dry_run`) |
| Check still auto | same test (`format_check_path` → `format_dry_run` → `format_source_auto`) |
| Census updated | [stage2-parser-census](../stage2-parser-census.md) I144 footnote |

## Consequences

- S2-04 stays `[~]`.
- **S2-I145:** [cst-reindent-func-body](stage2-s2-04-cst-reindent-func-body.md) — opt-in `format_cst_reindent` FunctionDecl depth-1 first knife; default `format_cst` unchanged (**≠** reindent GA).
- **S2-I146:** [cst-reindent-func-nested](stage2-s2-04-cst-reindent-func-nested.md) — nested relative depth + stack (**≠** reindent GA / ≠ 2→4).
- **S2-I147:** [cst-reindent-func-2to4](stage2-s2-04-cst-reindent-func-2to4.md) — FunctionDecl 2→4 normalize (**≠** reindent GA / ≠ class).
- **S2-I148:** [cst-reindent-class-stored](stage2-s2-04-cst-reindent-class-stored.md) — class stored-member track (**≠** reindent GA / ≠ env gate).
- **S2-I149:** [cst-reindent-env-gate](stage2-s2-04-cst-reindent-env-gate.md) — `SYX_FMT_CST_REINDENT` experiment gate (**≠** reindent GA).
- **S2-I150:** [cst-reindent-ga](stage2-s2-04-cst-reindent-ga.md) — `format_source_cst` → `format_cst_reindent` (**≠** write 迁入).
- **S2-I151:** [fmt-write-migration](stage2-s2-04-fmt-write-migration.md) — `format_file` → `format_dry_run` / auto (**≠** S2-04 `[x]`).
- **S2-I152:** [lsp-document-formatting](stage2-s2-04-lsp-document-formatting.md) — LSP format → dry_run/auto (**≠** lint 共树 / ≠ S2-04 `[x]`).
- **S2-I153:** [lint-cst-first](stage2-s2-04-lint-cst-first.md) — `lint_source_auto` (**≠** lint-on-CST / ≠ S2-04 `[x]`).
- **S2-I154:** [cst-subset-if-let](stage2-s2-04-cst-subset-if-let.md) — `take_if_let_simple` (**≠** if case / ≠ Frontend / ≠ S2-04 `[x]`).
- **S2-I155:** [cst-subset-guard-let](stage2-s2-04-cst-subset-guard-let.md) — `take_guard_let_simple` (**≠** if case / ≠ Frontend / ≠ S2-04 `[x]`).
- Next default = more CST deepen (if-case / range) or Frontend share tree → retire dual `syx-cst` → **S2-04 `[x]`** claim — **not** Stage-2 exit.
- Open: [stage2-s2-04-open](stage2-s2-04-open.md) · [stage2-open](stage2-open.md).

## References

- [stage2-s2-04-open](stage2-s2-04-open.md) · S2-I143
- [stage2-parser-census](../stage2-parser-census.md)
- [stage2-s2-04-cst-reindent-func-body](stage2-s2-04-cst-reindent-func-body.md) · S2-I145
- [stage2-s2-04-cst-reindent-func-nested](stage2-s2-04-cst-reindent-func-nested.md) · S2-I146
- [stage2-s2-04-cst-reindent-func-2to4](stage2-s2-04-cst-reindent-func-2to4.md) · S2-I147
- [stage2-s2-04-cst-reindent-class-stored](stage2-s2-04-cst-reindent-class-stored.md) · S2-I148
- [stage2-s2-04-cst-reindent-env-gate](stage2-s2-04-cst-reindent-env-gate.md) · S2-I149
- [stage2-s2-04-cst-reindent-ga](stage2-s2-04-cst-reindent-ga.md) · S2-I150
- [stage2-s2-04-fmt-write-migration](stage2-s2-04-fmt-write-migration.md) · S2-I151
- [stage2-s2-04-lsp-document-formatting](stage2-s2-04-lsp-document-formatting.md) · S2-I152
- [stage2-s2-04-lint-cst-first](stage2-s2-04-lint-cst-first.md) · S2-I153
- [stage2-s2-04-cst-subset-if-let](stage2-s2-04-cst-subset-if-let.md) · S2-I154
- [stage2-s2-04-cst-subset-guard-let](stage2-s2-04-cst-subset-guard-let.md) · S2-I155
- [plan/02](../../plan/02-improvement-roadmap.md) S2-04
- W873 / ADR-1217 (`format_source_auto`); driver `format_file` comments
