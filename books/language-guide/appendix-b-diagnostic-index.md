# Appendix B: Diagnostic Code Index

A user-facing cheat sheet of diagnostic codes. For compiler implementation details, see [docs/3_SemanticAnalysis.md](../../docs/3_SemanticAnalysis.md).

> **Current version:** SYX **117.0** · Phase **139** · documentation fingerprint `diag-139` (compiler source is authoritative)

## Semantics (S series)

| Code | Meaning |
|------|------|
| S001 | Duplicate symbol definition |
| S002 | Undefined identifier |
| S003 | Assignment to an immutable variable |
| S004 | Type mismatch |
| S005 | Cross-module access to an internal / private / fileprivate symbol |
| S006 | `Iterator` protocol missing `next()` |
| S007 | Sealed / exhaustiveness related (switch missing a case) |
| S008 | Type narrowing (e.g. `Int64 → Int8`) |
| S009 | Signed/unsigned mixing (partial) |
| S010 | Does not conform to the protocol / generic where constraint not satisfied |
| S011 | Borrow / mutation violation |
| S012 | Associated type error |
| S013 | Collection type consistency |
| S014 | Signed / unsigned mixing |
| S015 | Does not conform to the `Hashable` protocol |
| S016 | Does not conform to the `Equatable` protocol |
| S017 | Lifetime violation |
| S019 | Sendable channel |
| S020 | MainActor violation after await / `guard let` `else` does not exit / `await` in a non-`async` context |
| S022 | Using SIMD while it is disabled |
| S024 | Collection hosted ops forbid the legacy `MethodCall` (Phase 48) |
| S025 | Semantic-layer hosted methods must go through an extension / `CallHost` (Phase 50) |
| S046 | `is` with a numeric target forbidden (`1 is Int` and similar) |
| S050 | Inheritance override validation related (missing `override` / not declared in the superclass / signature mismatch) |
| S051 | `override` keyword missing |
| S052 | `override` of something not declared in the superclass |
| S053 | Inheriting a `final` class |
| S054 | `override` signature mismatch |
| S055 | `override` of a `final` superclass method |
| S056 | Computed property observers (rejected) |

## Host Plugins (H series)

| Code | Meaning |
|------|------|
| H001 | Plugin not loaded or plugin Host does not exist |
| H002 | Compile-time plugin manifest inconsistent with runtime / lock file |
| H003 | Plugin ABI version mismatch |
| H004 | Plugin `min_syx_version` not satisfied |

## Lexical (L series)

| Code | Meaning |
|------|------|
| L001 | Unterminated string / invalid lexical element (lexer) |
| L002–L012 | Lint rules (see [9_UsageGuide](../../docs/9_UsageGuide.md)) |

Lint excerpts:

| Code | Meaning |
|------|------|
| L001 | Function names should start lowercase (lint) |
| L002 | Duplicate function name |
| L003 | Empty function body |
| L004 | A public function should declare a return type |
| L005 | An async function should use await |
| L006 | Illegal character (variable name) |
| L007 | Global variable missing initialization |
| L008 | Empty import name |
| L009 | Bare return |
| L010 | Line longer than 120 characters |
| L011 | Contains TODO/FIXME |
| L012 | Prefer structured logging over print |

## Syntax (P series)

| Code | Meaning |
|------|------|
| P001 | Parse error (illegal syntax) |
| P001 | `syx.lock` missing at compile time (manifest not installed) |

## Runtime (R series)

| Code | Meaning |
|------|------|
| R003 | `as!` failure; Array / String subscript out of bounds; integer narrowing out of range |
| R007 | Force-unwrapping `nil` with `!` |
| R008 | `readLine` failure |
| R011 | Integer overflow Display (**≠ GA**; current arithmetic always wraps). The same code may also be used by other domains (such as the HTTP client) |
| R012 | Integer division by zero |
| R013 | URL parsing failure |
| R014 | Recursion too deep (stack overflow) |
| R016 | Required field not initialized |

## Concurrency (A / C series)

| Code | Meaning |
|------|------|
| A002 | Actor isolation violation |
| A003 | GlobalActor related |
| C001 | Actor isolation violation (finer-grained) |
| C002 | distributed actor boundary |
| C003 | Passing a non-`Sendable` type across isolation |
| C004 | Task cancellation related |
| C005 | GlobalActor violation after await |
| C006 | Structured concurrency parent task cancellation propagation |
| C009 | Actor synchronous same-instance reentrancy rejected (serial-no-reenter) |
| C010 | Channel closed (post-close send) |

## Macros (E / W series)

| Code | Meaning |
|------|------|
| W001 | `#warning` triggered |
| E001 | `#error` triggered |
| E002 | Macro expansion depth exceeded or cyclic expansion |
| E003 | Role misplacement (e.g. `@freestanding(expression)` used in statement position) |
| E004 | external macro during WASM compilation (`SYX_TARGET_WASM=1`) |
| E006 | `@Service` / `@Component` used on a non-type |
| E007 | Dependency cycle without `@Lazy` |
| E008 | `@Inject` missing dependency |
| E010 | DI typed resolve with a dynamic variable |

## Borrow Machine Proofs

| Phase | Trigger |
|------|------|
| Phase 32+ | `SYX_BORROW_MACHINE_PROOF=1` enables proof obligation export |
| Phase 33+ | `SYX_BORROW_COQ_LEAN_EXPORT=1` enables Coq/Lean export |
| Phase 34+ | `SYX_BORROW_COQ_LEAN_PROOF_PROD=1` prod machine checking |

## Full Toolchain Diagnostics

See [docs/9_UsageGuide.md — Diagnostic Codes](../../docs/9_UsageGuide.md#diagnostic-codes).

---

**See also** → [Language Reference: About This Reference](../language-reference/00-about-this-reference.md) · [Implementation Status Master Table](../shared/implementation-status.md) · [docs/3_SemanticAnalysis.md](../../docs/3_SemanticAnalysis.md)
