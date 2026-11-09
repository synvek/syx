# ADR: SA-2-01 — Swift `do/catch` language surface

**Status:** Accepted (SA-2-01)  
**Date:** 2026-11-09  
**Context:** SA-0-01 locked Swift `do { try … } catch` as the language default; the implementation still used statement-level `try { … } catch (e)`. SA-1 exited. Living backlog: [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md).

## Decision

1. **Statement surface is `do { … } catch …`.** Maps to existing `Stmt::Try` / `BeginTry`/`Catch` MIR (no new error-stack model).
2. **`try` is expression-only** (`try` / `try?` / `try!`). Statement-level `try { }` is a hard parse error (no compatibility window).
3. **Catch forms (MVP):**
   - `catch { }` — binds as `error` (Swift bare catch)
   - `catch (name[: Type]) { }` — kept for existing call sites
   - `catch name[: Type] { }` — paren-less binding
4. **Books / QuickReference / language-book-ssot** teach `do/catch` only.
5. **Non-claims:** ≠ typed multi-catch patterns (SA-2-02 deepens `Error`); ≠ SA-3 rename; ≠ package API Swiftify.

## Consequences

- Full-tree destructive rewrite of `try {` → `do {` in packages, examples, tools, tutorial, driver fixtures, and books.
- Formatter emits `do { } catch`.
- Next windows: SA-2-05 `throws -> T` order; SA-2-02 `enum: Error`.

## References

- [stdlib-swift-alignment.md](stdlib-swift-alignment.md)
- [language-book-ssot.md](language-book-ssot.md)
- [`books/language-guide/17-error-handling.md`](../../books/language-guide/17-error-handling.md)
