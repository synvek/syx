# SA-5 docs / examples / doctest census

**Status:** SA-5 exit hinge met (SA-5-01 + SA-5-03 + SA-5-05 `[x]`) · SA track exited via SA-6 · next = **post-SA Residual / 维护**  
**Date:** 2026-11-09

## Exit hinge

| ID | Claim | Evidence |
|----|-------|----------|
| SA-5-01 | en+zh `library-guide` aligned | Chapters `03-system` / `04-logging` / `05-system-networking`; overview = `importable_modules`; fences `Os.`/`OsError` = 0; `syx doctest` 7/7 |
| SA-5-03 | examples + VM allowlist | `Os.`/`Path.`/`options: Object` = 0 in `examples/**/*.sy`; `scripts/examples_vm_contract.sh` **PASS (26)** |
| SA-5-05 | `syx doctest` MVP | CLI `Doctest` subcommand; `scripts/book_pass_rate.py --library-guide --strict`; KPI `syx_doctest_subcommand_count≥1` |

## KPI snapshot

| KPI | Value |
|-----|-------|
| `syx_doctest_subcommand_count` | ≥1 (census ≈4 matches in `syx-cli`) |
| library-guide en fence pass | **7/7** |
| examples VM allowlist | **26/26 PASS** |
| stability.toml `[[package]]` rows | **20** (≥19) |

## Driver fix bundled (examples unblock)

Workspace stdlib overlay previously used `packages/.../mod.sy` as the module graph key, so MIR/codegen treated it as a **user** dep and prefixed ops (`SyxHost_<`). SA-5 uses `<stdlib>/Name.sy` as the overlay key while still reading disk bytes + prepending `SyxHost`.

## Thin follow-ups (not exit blockers)

- soga: `Http.fetch(url, opts)` → `Http.post(url:body:)` (headers/timeout still not on thin surface)
- `packages/README.md` + `stability.toml` import names aligned (≠ SA-5-06 manifest wiring)
- SA-5-02 language-guide full sweep still open
- Channel runtime residual (`trySend` / stack on `send`) noted; concurrency example slimmed to `Concurrency.*`
- Networking aggregate still ships tls/stream/grpc on disk; embedded `NET_SOURCE` remains core HTTP/TCP set

## Hand-off (done in SA-6)

- SA-6-01 census + scoped `File.stat` / `readAsync` off Object — see [sa-6-exit-census](sa-6-exit-census.md)
- SA-6-03 [sa-exit](../adr/sa-exit.md) Accepted

## Residuals (post-SA)

- Public `Object` inventory deepen / SA-4-03 deep clear
- `FileHandle.withOpen(body: Object)` unify
- Collections merge / Testing `@Test`
- `syx migrate` product CLI
- Channel Host surface repair

## Next

**post-SA Residual / 维护** — [sa-exit](../adr/sa-exit.md) residual 表.
