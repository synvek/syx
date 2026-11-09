# Stage-2 workspace slim census (S2-06)

**ADR:** [stage2-s2-06-claim](../adr/stage2-s2-06-claim.md) · **Window:** S2-I262 · **Current:** S2-I262  
**Open baseline:** [stage2-s2-06-open](../adr/stage2-s2-06-open.md) · S2-I260 · **Exclude:** [stage2-s2-06-exclude-syx-gui](../adr/stage2-s2-06-exclude-syx-gui.md) · S2-I261

**Goal (S2-06):** shrink the **default** workspace so product/optional surfaces (GUI / Farm·Dist narrative / etc.) do not dominate `cargo check --workspace` / default contributor builds.  
**Status:** S2-06 **`[x]`** scoped (S2-I262) — **≠** Stage-2 exit.

## Command

```bash
bash scripts/s2_06_workspace_census.sh
```

## S2-I260 open baseline (2026-11-07)

| Metric | Value |
|--------|------:|
| `Cargo.toml` members | **`["crates/*"]`** (glob; no `default-members` narrow) |
| Crate directories under `crates/` | **37** |
| GUI product crate in members | **`syx-gui`** (yes) |
| Named Farm / Dist / HSM / CDN crates | **0** |
| CI check/clippy excludes `syx-gui` | **yes** (partial narrow ≠ S2-06 done) |

## S2-I261 exclude (2026-11-07)

| Metric | Value |
|--------|------:|
| `Cargo.toml` members | **`["crates/*"]`** (unchanged) |
| `Cargo.toml` exclude | **`["crates/syx-gui"]`** |
| Crate directories under `crates/` | **37** (directory retained) |
| Default workspace packages (`cargo metadata --no-deps`) | **36** — **`syx-gui` absent** |
| GUI on disk + path opt-in | **yes** — `syx-vm` `gui-syx-isolated` |
| Named Farm / Dist / HSM / CDN crates | **0** |
| CI redundant `--exclude syx-gui` | **removed** (crate no longer a member) |

## S2-I262 claim (2026-11-07) — Current

| Metric | Value |
|--------|------:|
| Plan S2-06 | **`[x]`** scoped |
| Stage-2 | **`[~]`** (S2-07 remaining) |
| Default workspace | gui excluded; Farm/Dist/HSM/CDN = 0 crates |
| Residual OK | on-disk `syx-gui` + path/feature opt-in + driver tests/narrative |
| CI opt-in build | `--manifest-path crates/syx-gui/Cargo.toml` |

## Classification (honest)

| Class | Crates / notes |
|-------|----------------|
| **Core language / toolchain** | `syx-ast` `syx-lexer` `syx-parser` `syx-cst` `syx-semantic` `syx-mir` `syx-codegen` `syx-frontend` `syx-hir` `syx-driver` `syx-cli` `syx-vm` `syx-stdlib` `syx-diagnostics` `syx-config` `syx-fmt` `syx-lint` `syx-lsp` `syx-doc` `syx-macro` `syx-mono` `syx-meta` `syx-pkg` `syx-async` `syx-debug` `syx-host-plugin` … |
| **Backend / emit** | `syx-llvm` `syx-llvm-prefix` `syx-lld` `syx-rt` `syx-runtime-core` `syx-mir-run` `syx-cranelift-jit` `syx-wasm-emit` `syx-wasm-run` `syx-compiler-wasm` |
| **Opt-in product (workspace-excluded)** | **`syx-gui`** — on disk; path+feature via `syx-vm` `gui-syx-isolated`; **not** a default workspace member (S2-I261+) |
| **Already-narrowed narrative** | Farm / Dist / HSM / CDN — **no** sibling crates under `crates/`; residual = driver **test targets** / park product umbrella (≠ default lib members) |

## Gap vs plan/02 S2-06

| Claim | Today (honest) | plan/02 S2-06 target |
|-------|----------------|----------------------|
| Default workspace excludes product GUI | **Done** via `exclude = ["crates/syx-gui"]` (S2-I261) · **claimed** S2-I262 | GUI out of default build or under `experimental/` |
| Farm/Dist/HSM/CDN out of core workspace | **Already absent as crates** | Keep absent; do not re-introduce |
| Contributor `cargo check --workspace` | Glob members + exclude gui | Default path stays slim |

## Ladder

1. **S2-I260:** census + open ADR — landed.
2. **S2-I261:** exclude **`syx-gui`** from default workspace — landed.
3. **S2-I262:** dedicated **S2-06 `[x]` claim** — **this window**; ≠ Stage-2 exit.
4. Next default: **S2-I263 S2-07 open**.

## Non-claims (post-claim honesty)

- **≠** Stage-2 exit  
- **≠** delete `syx-gui` / move under `experimental/` required for scoped `[x]`  
- **≠** open S2-07 / HostFn+1 / product umbrella unpark / F-08 this window  

## References

- [stage2-s2-06-claim](../adr/stage2-s2-06-claim.md) · S2-I262
- [stage2-s2-06-exclude-syx-gui](../adr/stage2-s2-06-exclude-syx-gui.md) · S2-I261
- [stage2-s2-06-open](../adr/stage2-s2-06-open.md) · S2-I260
- [stage2-s2-05-claim](../adr/stage2-s2-05-claim.md) · S2-I259
- [plan/02](../../plan/02-improvement-roadmap.md) S2-06
