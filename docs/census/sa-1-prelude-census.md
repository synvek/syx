# SA-1 prelude shrink census

**Current:** SA-1 exit  
**Next:** SA-2-01 (`do/catch`)  
**Charter:** [stdlib-swift-alignment.md](../adr/stdlib-swift-alignment.md)

## Outcome

Injected prelude modules = [`packages/internal/prelude_core.toml`](../../packages/internal/prelude_core.toml) `[target]` / `[current_injected]` (**26** modules including **`Concurrency`** as core-layer narrative — injection unchanged).

Left prelude → **IMPORTABLE** (same names until SA-3): Math, Random, JSON, Time, Env, Path, Fmt, OrderedSet, Deque, SIMD4.

## Swift-shaped core surfaces

- `Double.pi` / `Double.sin` / `Double.sqrt` (and `Float64` equivalents)
- `Int.random(in:)` → half-open `[0, upperBound)`

## Script

`bash scripts/sa_1_prelude_census.sh` → `sa1_prelude_helper=1` · `OK`
