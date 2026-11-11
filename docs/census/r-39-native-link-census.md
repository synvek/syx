# R-39 full-native link fail-closed census

**Date:** 2026-11-11  
**Scope:** todo-005 C1–C3 / §4.3 — Linux cc-first link; ban `ld64.lld`; normalize `-Wl,` for bare LLD; hello runnable under `full-native`; link Err must not print Built native.  
**Script:** [`scripts/r39_native_link_census.sh`](../../scripts/r39_native_link_census.sh)

## Progress

| Track | Status |
|-------|--------|
| Cred / SA / R-0…R-38 | exited (wasm **46/46**; browser **46/0/2**; Object **186/11**) |
| **R-39** | **exited** — full-native hello link+run; Linux ld64 ban; `-Wl,` normalize; cc-first |
| Object / simd | **186/11** / unchanged |

## Root cause (closed this window)

1. Linux LLD argv still accepted `-Wl,--gc-sections` (cc form).
2. Resolvers could still pick Mach-O **`ld64.lld`** on Linux.
3. LLD noise + host `cc` Ok could print **Built native** for a dead binary (todo-005 §4.3).

## Delivery

1. Ban Linux `ld64.lld` in [`syx-rt/link.rs`](../../crates/syx-rt/src/link.rs), [`syx-lld`](../../crates/syx-lld/src/lib.rs), [`syx-llvm-prefix`](../../crates/syx-llvm-prefix/src/lib.rs).
2. `normalize_lld_link_flag` strips `-Wl,` before bare LLD argv.
3. Linux **cc-first** (`SYX_FORCE_LLD=1` to opt into LLD) in `link_executable_with` + `native_build` full-native path.
4. [`scripts/s0_backend_smoke.sh`](../../scripts/s0_backend_smoke.sh) `--native-full` requires hello build+run; rejects Built native on Err.
5. macOS TBD/`arm64e` → cc skip LLD path preserved.

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| Linux ld64 banned (source) | yes | **yes** |
| `-Wl,` normalize | yes | **yes** |
| full-native hello run | Hello | **pass** |
| browser_failed | **0** | **0** (46/0/2) |
| wasm | **46/46** | **46/46** |
| Object | **≤186 / ≤11** | **186/11** |
| native CI jobs | soft (R-37) | **remain soft** |

## Non-claims

- ≠ C4 throw IR `%b3` / `llvm-as` gate
- ≠ Reclaim hard `native-strict-main` / parity megagate
- ≠ R-40 cold-start E1 / book-pass hard
- ≠ hot_loop native run GA (still may crash after build; smoke soft for non-hello)
- ≠ Packaged `lib/sysroot` GA / Object/simd
