# Stage-2 governance census (S2-I1 baseline)

**Date:** 2026-11-01  
**Window:** S2-I1  
**ADR:** [stage2-open](adr/stage2-open.md)  
**Goal (S2-01):** process-governance inventory / fingerprint / gate narrative **out of product `src/`** → target **0** `*_inventory` linked into product binaries (migration/deletion in later windows; **this file is baseline only**).

## Command

```bash
bash scripts/s2_governance_census.sh
```

## Baseline counts (crates/, excluding target)

| Metric | Count |
|--------|------:|
| `fn *_inventory` matches | 237 |
| `_FINGERPRINT` / `FINGERPRINT:` const-ish matches | 241 |
| `fn *_fingerprint` matches | 655 |
| `crates/syx-driver/src/*_report.rs` files | 26 |
| `crates/syx-driver/src/lang_props_v*.rs` files | 5 |
| `Always \`true\`` / `Always \`false\`` comment matches | 2162 |

## Non-claims

- **≠** S2-01 `[x]` / Stage-2 exit.
- Does **not** authorize mass delete in the same window as open.
- Re-run the script after each S2-01 migration slice; update this table when materially changed.
