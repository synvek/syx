#!/usr/bin/env bash
# R-29 lang honesty + super.method census.
# KPIs: lang_super_failed=0 · lang_phase_pin_failed=0 · browser_failed=0 · object ≤186/11
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-29 lang super census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

lang_out=$(mktemp)
cargo test -p syx-driver --test lang -- \
  super_call poly_super nested_super_call manifest_phase int_extension_declares \
  array_element_check set_element_check \
  >"$lang_out" 2>&1 || true
result_line=$(grep -E "^test result:" "$lang_out" | tail -1 || true)
ok "lang_filter_result_line" "$result_line"
lang_failed=$(echo "$result_line" | sed -n 's/.* \([0-9][0-9]*\) failed.*/\1/p')
lang_failed=${lang_failed:-999}
ok "lang_super_failed" "$lang_failed"
# phase + int pins are included in the filter via manifest_phase / int_extension
ok "lang_phase_pin_failed" "$lang_failed"
if [[ "$lang_failed" -ne 0 ]]; then
  die "lang honesty/super/element filter must be 0 failed (got $lang_failed)"
  grep "FAILED" "$lang_out" >&2 || true
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
  die "object residual exceeded ≤186/11"
fi

if [[ "$fail" -ne 0 ]]; then
  exit 1
fi
echo "r29_lang_super_census_ok=1"
echo "current=R-29-lang-super"
echo "next=R-30-plus-residual"
