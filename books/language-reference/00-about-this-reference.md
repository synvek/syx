# About This Reference

This reference describes the formal rules of the **SYX language** — lexical structure, grammar, types, expressions, statements, declarations, attributes, patterns, generics and protocols — for readers who need to look things up precisely (compiler contributors, SDK maintainers, advanced language users).

> **Current version:** SYX **117.0** (Phase **139**) · `syx-cache-v1` · license [Apache-2.0](../../LICENSE) · documentation last synced 2026-10-27
>
> For the full status matrix, see [shared/implementation-status.md](../shared/implementation-status.md); for architecture decisions, see [docs/adr/architecture-decisions.md](../../docs/adr/architecture-decisions.md); for the capabilities summary, see [docs/adr/capabilities-summary.md](../../docs/adr/capabilities-summary.md).

## Relationship to the *Language Guide*

| This Reference | The *Language Guide* |
|--------|--------------|
| Formal definitions, grammar tables, diagnostic codes | Narrative explanations, complete examples, learning path |
| "What is the grammar of an `if` statement?" | "How do I write branching logic with `if`?" |

The "**Usage examples**" at the end of each chapter link to the corresponding guide chapter; the "**See also**" at the end of each guide chapter links back to this reference.

## Chapter Structure

| # | Chapter | Contents |
|---|------|------|
| 1 | [Lexical Structure](01-lexical-structure.md) | Tokens, keywords, literals, comments, escapes |
| 2 | [Types](02-types.md) | Type expressions, numerics, String, collections, references, opaque types, SIMD |
| 3 | [Expressions](03-expressions.md) | Expression classification, precedence table |
| 4 | [Statements](04-statements.md) | Variable declarations, control flow, exceptions |
| 5 | [Declarations](05-declarations.md) | import / func / class / struct / enum / actor / protocol / extension |
| 6 | [Attributes](06-attributes.md) | `@attached` / `@Service` / `@Around` / `@resultBuilder` / `@Equatable` and more |
| 7 | [Patterns](07-patterns.md) | Variants, tuples, switch / if case patterns |
| 8 | [Generic Parameters and Arguments](08-generic-parameters-and-arguments.md) | `<T>` / where / protocol constraints / monomorphization |
| 9 | [Protocols and Conformances](09-protocols-and-conformances.md) | Protocol declarations, conformances, witnesses, existential types |
| 10 | [Grammar Summary](10-grammar-summary.md) | EBNF / modules / functions / types / expressions |
| A | [Appendix: Implementation Status Matrix](appendix-implementation-matrix.md) | Points at the implementation status master table |

## Notation Conventions

| Notation | Meaning |
|------|------|
| `…` | Grammar element |
| `\|` | Alternative |
| `?` | Optional |
| `*` | Zero or more times |
| `+` | One or more times |
| `[x]` | Optional group |
| `{x}` | Zero or more repetitions |
| `(x \| y)` | Grouping |
| `terminal` | Terminal (keyword / punctuation) |
| `nonterm` | Nonterminal |

## Implementation Status

Every rule or feature is marked with one of five tiers (see [shared/implementation-status.md](../shared/implementation-status.md)):

| Tier | Meaning |
|------|------|
| **Implemented** / **GA** | Verifiable with `syx run`, parity, or hard gates |
| **Partially implemented** / **partial GA** | Parses/compiles but semantics are incomplete |
| **Planned** | Recorded on the roadmap, implementation unfinished |
| **Not implemented** | No corresponding AST/semantics |
| **Rejected** | Deliberately fail-closed (the compiler reports an error) |

## Diagnostic Code Prefixes

| Prefix | Phase | Scope | Example |
|------|------|------|------|
| L | Lexical / Lint | Tokens, naming conventions | L001 unterminated string / L001 function names should be lowercase |
| P | Parsing | Syntax errors | P001 parse error |
| S | Semantic | Symbols, types, borrowing | S002 undefined identifier |
| A | Actor | Isolation violations | A002 cross-actor access |
| C | Concurrency | Cross-isolation, Sendable | C003 not Sendable |
| R | Runtime | panic / trap | R007 force unwrap nil |
| H | Host plugin | dlopen / signature verification | H001 plugin not loaded |
| E | Macro | Expansion errors | E002 macro expansion depth exceeded |
| W | Warning | Compile-time warnings | W001 #warning |

The full index lives in [Language Guide Appendix B: Diagnostic Code Index](../language-guide/appendix-b-diagnostic-index.md).

## Relationship to the Compiler Implementation

| This Reference | Compiler source |
|--------|------------|
| Lexical | `crates/syx-lexer/src/` |
| Syntax (AST) | `crates/syx-ast/src/lib.rs` |
| Parsing | `crates/syx-parser/src/` |
| Semantics | `crates/syx-semantic/src/` |
| Macro expansion | `crates/syx-macro/src/` |
| MIR | `crates/syx-mir/src/` |
| Codegen | `crates/syx-codegen/src/` / `crates/syx-llvm/src/` / `crates/syx-wasm-emit/src/` |
| VM | `crates/syx-vm/src/` |

The complete BNF is kept in sync with the parser in [docs/2_SyntaxAnalysis.md](../../docs/2_SyntaxAnalysis.md).

## Toolchain

- Installation / CLI / project configuration: [docs/9_UsageGuide.md](../../docs/9_UsageGuide.md)
- Syntax cheat sheet: [docs/8_QuickReference.md](../../docs/8_QuickReference.md)
- Compiler technical documentation: [docs/README.md](../../docs/README.md)

---

**Usage examples** → [Language Guide](../language-guide/00-preface.md)
