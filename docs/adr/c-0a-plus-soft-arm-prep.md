# C-0a+ prep — Unknown / ErrorType / InferenceVar trichotomy (design only)

**Status:** Accepted as **prep** (2026-11-27). **Does not change runtime semantics.**  
**Parent:** [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-0a+ still **park** pending an explicit open-knife window.  
**Related:** C-0a…u Unknown→ground sinks (thin); [`type_checker.rs`](../../crates/syx-semantic/src/type_checker.rs) soft arms.

## Context

C-0 thin sinks reject many Unknown→ground concrete flows via `check_unknown_to_ground_sink` → **S004**, but `is_compatible` still has:

```rust
(_, TypeId::Unknown) => true,           // soft / gradual
(_, TypeId::Named(n)) if n == "Object"  // C-0b HostFn fallout
```

`TypeId` today has a single `Unknown` bucket — **no** distinct `ErrorType` / `InferenceVar`. Closing the soft arm without trichotomy would break HostFn façades, empty `[]` / `Set()` init, JSON/`Object` bags, and many books examples.

## Inventory (call sites / risk)

| Bucket | Where | Close now? | Notes |
|--------|-------|------------|-------|
| Soft arm `(_, Unknown)` | `type_checker.rs` L17 | **分期** | Core gradual typing escape |
| Found-`Object` soft | `type_checker.rs` L20 | **分期** | HostFn table vs façade mismatch |
| Expected-`Any` | L22 | **永久保留** | unconstrained top |
| Array/Set Unknown-elem | L27–37 | **分期** | empty literal init |
| Bare `Set` / `Array` / `Dictionary` names | L40–46 | **分期** | host return vs façade |
| Optional / Reference peel | L47–63 | **可关子集** | after trichotomy |
| C-0* sinks | `semantic_analyze_*.rs` | **已闸** | keep; do not weaken |

**ErrorType / InferenceVar:** not present in [`semantic_types.rs`](../../crates/syx-semantic/src/semantic_types.rs) `TypeId` enum. Prep requires adding variants **before** deleting `(_, Unknown)`.

## Staging proposal (future knife, not this window)

1. **Add** `TypeId::ErrorType` + `TypeId::InferenceVar` (or equivalent names); migrate parse/check failures → ErrorType; unconstrained holes → InferenceVar; keep Unknown only as transitional alias if needed.
2. **Narrow** soft arm to `(ground, InferenceVar)` only; ErrorType never assignable to ground.
3. **HostFn / Object** façade honesty window (separate) before removing found-Object soft.
4. **Regression minimum (draft):** lang nails for C-0a…u green set + HostFn smoke (`jStr(v: Any)`, empty `Array<Int> = []`, `Object` JSON bag) must stay green.

## Exit criteria for this prep window

- This ADR exists and is linked from `plan/02` C-0a+ row.
- C-0a+ marked **prep ready / still park** (not open knife).
- **C-0e** remains park (actor S008 conflict unresolved).
- **No** change to `is_compatible` soft arms in this window.

## Non-claims

- ≠ C-0a+ implemented / soft arm closed  
- ≠ C-0e / C-3 `[x]` / C-5 `[x]`  
- ≠ HostFn Object façade rewrite  
