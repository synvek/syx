# R-35 writing blockers census

**Date:** 2026-11-11  
**Scope:** todo-005 B7 writing blockers — implicit `self` compound assign / closure mut capture / `async let` false S020.  
**Script:** [`scripts/r35_writing_blockers_census.sh`](../../scripts/r35_writing_blockers_census.sh)

## Progress

| Track | Status |
|-------|--------|
| Cred / SA / R-0…R-34 | exited (wasm **46/46**; browser **46/0/2**; Object **186/11**) |
| **R-35** | writing blockers cleared (file-path fixtures) |
| Object / simd | **186/11** / unchanged |

## Root cause

1. **Compound assign / implicit self:** bare `count += 1` missed enclosing-field path when Core globals/`Object.count` shadowed; `self.x +=` was not parsed as MemberAssign; class MutRefCell field load used register-dest LoadLocal (stack miss); Array method-name fallback stole `.count`.
2. **Closure mut capture:** free-var walk skipped Assign/CompoundAssign LHS; captures were by-value copies (no write-back); MIR001 lacked span.
3. **async let S020:** multi-await hoist rewrote `let` as mutable placeholders → MainActor freeze set → false S020; await operands also needed exemption. Detach targets named only via `ConstStr` were trimmed from the program (C017).

## Delivery

1. Semantic: Assign/CompoundAssign prefer enclosing `var` field over immutable globals; Ident read order locals → field → globals; hoist `var` with no init excluded from MainActor freeze; await-operand depth skips S020.
2. Parser: `self.x +=` / `obj.x +=` desugar to MemberAssign.
3. MIR: enclosing field load/store unwrap MutRefCell; member get skips inherited/Array `count` when type has stored field; free-var LHS + MutRefCell mut captures; program trim keeps `ConstStr` function names; Ident `Type()` pads memberwise defaults even under ArgsEarly; dataclass copy rebuilds bag without calling 0-arg init.
4. Fixtures + file-path tests: [`writing_blockers.rs`](../../crates/syx-driver/tests/lang/writing_blockers.rs).

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `writing_blockers::*` (3) | green | **pass** |
| browser_failed | **0** | **0** (46/0/2) |
| wasm | **46/46** | **46/46** |
| Object | **≤186 / ≤11** | **186/11** |

## Non-claims

- ≠ Index `xs[i] +=`
- ≠ capture-list syntax / `@escaping` GA
- ≠ deleting all S020 / true MainActor cross-await shared-state diagnostics
- ≠ R-36 Unknown leakage / Span-everywhere
- ≠ R-37 CI
- ≠ `Array.first/last`, `for (k,v)`, `catch E.bad` (still on attached table)
