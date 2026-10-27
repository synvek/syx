# Test buckets (M0-E01 / ADR-133 · Wave A W1 / ADR-345)

| Bucket | Meaning | GA gate? |
|--------|---------|----------|
| **semantic** | Language / type / MIR semantics | Yes (behavioral) |
| **golden** | Fixed stdout / exit fixtures | Yes |
| **differential** | Exact VM/MIR/WASM/native compare (`--test differential`) | Yes (blocking CI) |
| **fingerprint** | Count floors, source `contains`, matrix self-cert | **No** — smoke only |

## GA evidence rules (A-03)

**New** GA / Truth nails must prove behavior with **exact** printed lines and/or exit codes (or differential oracle). Forbidden as sole GA proof:

- `stdout.contains` / `printed.join("\n").contains(...)`
- Source-tree `include_str!(...).contains(...)` standing in for runtime output
- `scripts/audit_behavioral_tests.sh` count floors (C-10: **smoke / anti-delete only**, not release quality KPI)

Legacy `contains` / fingerprint nails are audited in [`plan/topics/ga-evidence-audit.md`](../plan/topics/ga-evidence-audit.md); cleanup is Wave A W2+.

See [`scripts/audit_behavioral_tests.sh`](../scripts/audit_behavioral_tests.sh) and [`docs/differential/oracle-v0.md`](differential/oracle-v0.md).
