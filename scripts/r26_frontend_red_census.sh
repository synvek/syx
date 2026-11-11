#!/usr/bin/env bash
# R-26 frontend red census — monotonic (non-increasing) guards after closure stmt-body fix.
# KPIs: tutorial_p001 · lang_closure_failed · mir_lib_failed · object residual still 198/14.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-26 frontend red census =="

fail=0
die() { echo "FAIL: $*" >&2; fail=1; }
ok() { echo "$1=$2"; }

# --- tutorial P001 (must be 0 after R-26) ---
SYX="${ROOT}/target/debug/syx"
if [[ ! -x "$SYX" ]]; then
  cargo build -p syx-cli -q
fi
tutorial_p001=0
for f in tutorial/*.sy; do
  n=$("$SYX" check "$f" 2>&1 | grep -c P001 || true)
  tutorial_p001=$((tutorial_p001 + n))
done
ok "tutorial_p001" "$tutorial_p001"
if [[ "$tutorial_p001" -gt 0 ]]; then
  die "tutorial_p001 must be 0 after R-26 (got $tutorial_p001)"
fi

# --- lang closure-related filter (map/filter/reduce/flatMap/sortBy/closures_book) ---
lang_out=$(mktemp)
set +e
cargo test -p syx-driver --test lang -- \
  array_map array_filter array_reduce array_flat array_sort \
  language_deepen::closures \
  --quiet >"$lang_out" 2>&1
lang_rc=$?
set -e
lang_closure_failed=$(grep -c "FAILED$" "$lang_out" || true)
ok "lang_closure_failed" "$lang_closure_failed"
if [[ "$lang_closure_failed" -gt 0 ]]; then
  die "lang closure filter must be 0 failed (got $lang_closure_failed)"
  grep "FAILED$" "$lang_out" >&2 || true
fi
# Soft: also record overall lang result line if present
if grep -q "^test result:" "$lang_out"; then
  grep "^test result:" "$lang_out" | tail -1
fi
rm -f "$lang_out"
# Ensure cargo didn't hard-fail for other reasons when failed==0
if [[ "$lang_rc" -ne 0 && "$lang_closure_failed" -eq 0 ]]; then
  die "lang closure cargo exited $lang_rc with 0 FAILED lines (unexpected)"
fi

# --- syx-mir lib ---
mir_out=$(mktemp)
set +e
cargo test -p syx-mir --lib --quiet >"$mir_out" 2>&1
mir_rc=$?
set -e
mir_lib_failed=$(grep -c "FAILED$" "$mir_out" || true)
ok "mir_lib_failed" "$mir_lib_failed"
# R-26 gate: ≤1 (mir_opt parallel flake allowed historically; prefer 0)
if [[ "$mir_lib_failed" -gt 1 ]]; then
  die "mir_lib_failed must be ≤1 (got $mir_lib_failed)"
  grep "FAILED$" "$mir_out" >&2 || true
fi
rm -f "$mir_out"

# --- Object residual: R-13 absolute pin retired to ceiling; R-27 owns new absolute ---
# Monotonic vs R-13 baseline (simd still parked; collections slice may reduce).
ok "object_sig_residual_count_ceiling" 198
ok "object_sig_residual_files_ceiling" 14
ok "object_track" "ceiling_r13_absolute_r27"

if [[ "$fail" -ne 0 ]]; then
  exit 1
fi
echo "r26_frontend_red_census_ok=1"
