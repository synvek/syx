#!/usr/bin/env bash
# R-35 writing blockers census.
# KPIs: writing_blockers green · browser_failed=0 · wasm 46/46 · object ≤186/11
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-35 writing blockers census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

# Fixture-local caches (project_root_for = parent of .sy) can stale-green.
rm -rf crates/syx-driver/tests/fixtures/lang/target target/cache /tmp/target/cache 2>/dev/null || true

lang_out=$(mktemp)
cargo test -p syx-driver --test lang writing_blockers -- --test-threads=1 >"$lang_out" 2>&1 || true
result_line=$(grep -E "^test result:" "$lang_out" | tail -1 || true)
ok "writing_blockers_result_line" "$result_line"
wb_failed=$(echo "$result_line" | sed -n 's/.* \([0-9][0-9]*\) failed.*/\1/p')
wb_passed=$(echo "$result_line" | sed -n 's/.* \([0-9][0-9]*\) passed.*/\1/p')
wb_failed=${wb_failed:-999}
wb_passed=${wb_passed:-0}
ok "writing_blockers_failed" "$wb_failed"
ok "writing_blockers_passed" "$wb_passed"
if [[ "$wb_failed" -ne 0 || "$wb_passed" -lt 3 ]]; then
  die "writing_blockers must be ≥3 passed / 0 failed"
fi
rm -f "$lang_out"

browser_out=$(mktemp)
cargo test -p syx-driver --test browser -- --test-threads=8 >"$browser_out" 2>&1 || true
b_line=$(grep -E "^test result:" "$browser_out" | tail -1 || true)
ok "browser_result_line" "$b_line"
browser_failed=$(echo "$b_line" | sed -n 's/.* \([0-9][0-9]*\) failed.*/\1/p')
browser_failed=${browser_failed:-999}
ok "browser_failed" "$browser_failed"
if [[ "$browser_failed" -ne 0 ]]; then
  die "browser_failed must be 0"
fi
rm -f "$browser_out"

wasm_out=$(mktemp)
cargo test -p syx-driver --test wasm -- --test-threads=8 >"$wasm_out" 2>&1 || true
w_line=$(grep -E "^test result:" "$wasm_out" | tail -1 || true)
ok "wasm_result_line" "$w_line"
wasm_failed=$(echo "$w_line" | sed -n 's/.* \([0-9][0-9]*\) failed.*/\1/p')
wasm_passed=$(echo "$w_line" | sed -n 's/.* \([0-9][0-9]*\) passed.*/\1/p')
wasm_failed=${wasm_failed:-999}
wasm_passed=${wasm_passed:-0}
ok "wasm_passed" "$wasm_passed"
ok "wasm_failed" "$wasm_failed"
if [[ "$wasm_failed" -ne 0 || "$wasm_passed" -ne 46 ]]; then
  die "wasm must be 46/46"
fi
rm -f "$wasm_out"

python3 - <<'PY'
import re, sys
from pathlib import Path
root = Path(".")
obj_re = re.compile(r"(?::\s*Object\b|->\s*Object\b|Array\s*<\s*Object\s*>)")
total = files = 0
for path in sorted((root / "packages").rglob("*.sy")):
    rel = path.relative_to(root).as_posix()
    if rel.startswith("packages/internal/"):
        continue
    code = "\n".join(
        ln for ln in path.read_text(encoding="utf-8").splitlines()
        if not ln.strip().startswith("//")
    )
    n = len(obj_re.findall(code))
    if n:
        files += 1
        total += n
print(f"object_sig_residual_count={total}")
print(f"object_sig_residual_files={files}")
sys.exit(0 if total <= 186 and files <= 11 else 1)
PY
if [[ $? -ne 0 ]]; then
  die "object residual exceeded 186/11"
fi

ok "object_track" "parked_le_186_11"
ok "r35_writing_blockers_census_ok" 1

if [[ "$fail" -ne 0 ]]; then
  exit 1
fi
echo "current=R-35-writing-blockers"
echo "next=R-36-diagnostics"
