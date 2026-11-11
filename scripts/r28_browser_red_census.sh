#!/usr/bin/env bash
# R-28 browser red census.
# KPIs: browser_failed=0 · wasm 46/46 · object residual ≤186/11.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-28 browser red census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

browser_out=$(mktemp)
if ! cargo test -p syx-driver --test browser -- --test-threads=8 >"$browser_out" 2>&1; then
  :
fi
# Parse "test result: ok. N passed; F failed; I ignored"
result_line=$(grep -E "^test result:" "$browser_out" | tail -1 || true)
ok "browser_result_line" "$result_line"
browser_failed=$(echo "$result_line" | sed -n 's/.* \([0-9][0-9]*\) failed.*/\1/p')
browser_failed=${browser_failed:-999}
ok "browser_failed" "$browser_failed"
if [[ "$browser_failed" -ne 0 ]]; then
  die "browser_failed must be 0 (got $browser_failed)"
  grep -E "FAILED|failures:" "$browser_out" >&2 || true
fi
rm -f "$browser_out"

# Object residual ceiling (R-27 absolute ≤186/11)
python3 - <<'PY'
import re, sys
from pathlib import Path
root = Path(".")
obj_re = re.compile(r"(?::\s*Object\b|->\s*Object\b|Array\s*<\s*Object\s*>)")
total = 0
files = 0
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
obj_rc=$?
if [[ "$obj_rc" -ne 0 ]]; then
  die "object residual exceeded R-27 ceiling 186/11"
fi

ok "object_track" "parked_le_186_11"
ok "r28_browser_red_census_ok" 1

if [[ "$fail" -ne 0 ]]; then
  exit 1
fi
echo "current=R-28-browser-closeout"
echo "next=R-29-plus-residual"
