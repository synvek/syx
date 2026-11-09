# SA-2 language surface census

**Current:** SA-2 exit (01…04 `[x]`; 05 done this stage)  
**Next:** SA-3-01 (`System` merge)  
**Charter:** [stdlib-swift-alignment.md](../adr/stdlib-swift-alignment.md) · [sa-2-01-do-catch.md](../adr/sa-2-01-do-catch.md)

## Outcome

| Hinge | Status |
|-------|--------|
| Swift `do { try … } catch` | Parser + MIR surface; statement `try {` removed from packages |
| `throws -> T` | packages trailing `-> T throws` = 0 |
| `enum …: Error` | Semantic throw gate; OsError / NetworkError / IOError / … pilots |
| Caseless enum namespace | `ProcessInfo.processorCount` in `packages/os` |
| `public data class` | Parser reject; packages/examples decls = 0 |

## Residual (not blocking SA-2 exit)

- Non-public `data class` still parses for fmt/CST fixtures + `@Codable` (tutorial/books → SA-5)
- SA-2-06 bare `Object` / SA-2-07 broader computed props → may bind with SA-4
- Module rename Os→System → **SA-3-01**

## Script

`bash scripts/sa_2_language_surface_census.sh` → `sa2_language_surface_helper=1` · `OK`
