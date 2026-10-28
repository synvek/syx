# SYX Language Tutorial — Comprehensive Examples

> **Directory**: `tutorial/`
> **Corresponding docs**: [`books/language-guide/`](../books/language-guide/) (33 chapters + 2 appendices)
> **Goal**: Provide one runnable `.sy` example per chapter of the *SYX Language Guide*, covering the language features that are available on the current implementation surface.
> **Runnable**: Every file contains a `func main()` (or a synchronous entry point) and can be verified with `syx run tutorial/XX-....sy`; unsupported features are marked with `// note:` instead of incorrect Swift pseudo-syntax.

---

## Chapter Index

| # | Chapter | Example file | Core topics |
|---|---------|--------------|-------------|
| 00 | Preface | — | (introductory chapter, no standalone example) |
| 01 | Getting Started | [01-getting-started.sy](./01-getting-started.sy) | `func main()`, async entry, `import`, CLI, troubleshooting |
| 02 | The Basics | [02-the-basics.sy](./02-the-basics.sy) | `let`/`var`, type annotation, numeric literals, literal suffixes, `typealias`, nesting, tuples, raw strings |
| 03 | Basic Operators | [03-basic-operators.sy](./03-basic-operators.sy) | arithmetic / comparison / logical / nil-coalescing / ranges / compound assignment / precedence |
| 04 | Strings and Characters | [04-strings-and-characters.sy](./04-strings-and-characters.sy) | string literals, `Character`, grapheme clusters, escapes, interpolation, String methods, regex, String↔Array |
| 05 | Collection Types | [05-collection-types.sy](./05-collection-types.sy) | `Array`/`Set`/`Dictionary`/`Object`, HOF, chaining, for-in |
| 06 | Control Flow | [06-control-flow.sy](./06-control-flow.sy) | `if`/`while`/`for-in`/`switch`/`where` guards / `if let` / `if case` / `for await in` |
| 07 | Functions | [07-functions.sy](./07-functions.sy) | named parameters, default parameters, multiple returns, nested functions, effect annotations, async/throws, overloading, HOF |
| 08 | Closures | [08-closures.sy](./08-closures.sy) | closure expressions, capture, trailing closures, HOF chains, reference semantics |
| 09 | Enumerations | [09-enumerations.sy](./09-enumerations.sy) | simple / associated-value / generic enums, exhaustive switch, `Result` |
| 10 | Structures and Classes | [10-structures-and-classes.sy](./10-structures-and-classes.sy) | `struct`/`class`/`data class`/`actor`, ARC, `mutating`, `final` |
| 11 | Properties | [11-properties.sy](./11-properties.sy) | stored / static / computed properties, `willSet`/`didSet`, `lazy`, `@Published` |
| 12 | Methods | [12-methods.sy](./12-methods.sy) | instance / static methods, `mutating`, `override`, protocol default implementations, self |
| 13 | Subscripts | [13-subscripts.sy](./13-subscripts.sy) | built-in subscripts, custom `subscript`, multi-argument subscripts, KeyPath |
| 14 | Inheritance | [14-inheritance.sy](./14-inheritance.sy) | single inheritance, required `override`, `super`, `final class`, vtable |
| 15 | Initialization | [15-initialization.sy](./15-initialization.sy) | designated init, two-phase initialization, `init?`, delegated initialization, `required` |
| 16 | Deinitialization | [16-deinitialization.sy](./16-deinitialization.sy) | `deinit`, ARC deallocation, deinit chain, actor deinit |
| 17 | Error Handling | [17-error-handling.sy](./17-error-handling.sy) | `try`/`catch`/`throw`/`try?`/`try!`/`Result<T,E>` |
| 18 | Concurrency | [18-concurrency.sy](./18-concurrency.sy) | `async`/`await`, `Task`, `TaskGroup`, Actor, Channel, GlobalActor, configuration |
| 19 | Macros | [19-macros.sy](./19-macros.sy) | `#macro`, `@attached`, `@Codable`, `@resultBuilder`, Service DI, AOP |
| 20 | Type Casting | [20-type-casting.sy](./20-type-casting.sy) | numeric promotion, `is`/`as?`/`as!`, protocol-existential cast, literal inference |
| 21 | Nested Types | [21-nested-types.sy](./21-nested-types.sy) | `Outer.Inner` paths, deep nesting, flattening recommended, nested enums |
| 22 | Extensions | [22-extensions.sy](./22-extensions.sy) | `extension` methods / computed properties / initializers / protocols / where constraints |
| 23 | Protocols | [23-protocols.sy](./23-protocols.sy) | protocol declaration, witness table, associated types, default implementations, `any P`, Self |
| 24 | Generics | [24-generics.sy](./24-generics.sy) | generic functions / types / where / monomorphization, protocol constraints, associated types |
| 25 | Opaque Types | [25-opaque-types.sy](./25-opaque-types.sy) | `some P` compile-time hiding, `any P` existential, performance trade-offs |
| 26 | Memory Management | [26-memory-management.sy](./26-memory-management.sy) | ARC + ManagedHeap GC, three back ends, refcount, deinit, retain cycles |
| 27 | Memory Safety | [27-memory-safety.sy](./27-memory-safety.sy) | borrow checker, NLL, S011 conflict matrix, Sendable |
| 28 | Access Control | [28-access-control.sy](./28-access-control.sy) | `public`/`internal`/`private`/`fileprivate`, cross-module S005 |
| 29 | Modules and Packages | [29-modules-and-packages.sy](./29-modules-and-packages.sy) | single-file / multi-file, `import` paths, git/registry dependencies, `syx.toml`, locks |
| 30 | Advanced Operators | [30-advanced-operators.sy](./30-advanced-operators.sy) | bitwise, custom infix/prefix/postfix, precedence / associativity |
| 31 | SIMD | [31-simd.sy](./31-simd.sy) | `SIMD2/4/8/16<T>`, per-lane operations, reductions, cross-back-end |
| 32 | Operators vs Core Host | [32-operators-vs-host.sy](./32-operators-vs-host.sy) | Intrinsic vs Host, MIR opcodes, LLVM/Cranelift/WASM mapping |
| 33 | Runtime Extensions | [33-runtime-extensions.sy](./33-runtime-extensions.sy) | dlopen plugins, `SYX_HOST_PLUGIN_PATH`, ed25519 signature verification, Embed API |
| A | Appendix A: Implementation Status Matrix | — | (status table, no standalone example) |
| B | Appendix B: Diagnostic Code Index | — | (diagnostic cheat sheet, no standalone example) |

---

## How to Use

### 1) Read alongside the Language Guide

Each file's top comment lists "Language features covered in this chapter"; reading it against `books/language-guide/<chapter>.md` works best.

### 2) Want to run them

From the repository root:

```bash
cargo run -p syx-cli -- run tutorial/01-getting-started.sy
# or with a prebuilt binary:
./target/debug/syx run tutorial/25-opaque-types.sy
```

All 33 files can be `syx run`. Some chapters deliberately avoid surfaces that are not yet stable (e.g. custom `subscript`, `super.init`, unreliable `throw`→`catch`, associated-value `switch` that hangs, missing plugin host); see the `// note:` comments inside each file.

### 3) Want more real runnable examples

- `crates/syx-driver/tests/fixtures/` — compiler integration-test corpus
- `examples/` — user-facing examples
- `books/language-guide/<chapter>.md` — examples embedded in each chapter (the narrative may run ahead of this directory's runnable subset)

### 4) Want to write the same code against Swift

`books/language-guide/<chapter>.md` has a "Differences from Swift" table at the end of every chapter. Common pitfalls: SYX enum **does not** use the `case` keyword; use `class`/`data class` for records, not the Swift-style `struct { var x: Int }`; Dictionary literals use `{ "k": v }`; closure parameters must be written as `{ (x) in ... }`.

---

## Implementation Status Summary (as of SYX 117.0 / Phase 139)

| Category | Implementation status |
|----------|----------------------|
| **Implemented (GA)** | type-system basics, strings/collections, control flow, functions, closures, class/struct/enum/actor, init/deinit, error handling try/catch, concurrency (async/await/Task/Actor/Channel), extensions, protocol witnesses, monomorphized generics, where constraints, access control, modules/packages/locks, bitwise, custom operators, SIMD2/4/8/16, ARC + ManagedHeap GC, Host plugins |
| **Partial GA** | opaque types `some P` (subset), `throws T` typed throws, rethrows, `@objc`/KVO, `inout`, full `weak`/`unowned`, Self constraints, full protocol extension where, `@escaping`/`@autoclosure`, full generic reflection, AspectJ aspect classes, full JSON paths, CompileTime evaluation |
| **Planned** | full Swift properties, AspectJ aspect classes, full token-stream API, full Result generics set |
| **Not implemented** | `@available`, `@objc` bridging, Objective-C interop, Swift-Package compatibility |

---

## Contributing

Found an example bug / want to complete a feature?

1. Edit the corresponding `XX-<chapter>.sy` directly.
2. Keep the top comment's "Language features covered in this chapter" list in sync.
3. Reference the corresponding chapter in the PR description.

---

**Last updated:** 2026-10-28
**Maintainer:** SYX core team
**Version:** matches SYX 117.0 / Phase 139 (tutorial fully `syx run`-nable)
