#!/usr/bin/env bash
# R-44 Int8 contextual wrap census.
# Hard gates: int8 wrap trio green · contextual_int_literal int8 failed=0
# Soft park: Comparable sort · JIT Suite-1. KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-44 Int8 contextual wrap census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "R-44 MIR/semantic Int8 wrap wiring missing"
from pathlib import Path
binary = Path("crates/syx-mir/src/mir_lower_expr_binary.rs").read_text(encoding="utf-8")
if "R-44" not in binary or "BinOp::Shl" not in binary or "unary_numeric_kind" not in binary:
    raise SystemExit("mir_lower_expr_binary missing R-44 shift left-kind")
compound = Path("crates/syx-mir/src/mir_lower_expr_compound_assign.rs").read_text(encoding="utf-8")
if "R-44" not in compound or "fixed-width" not in compound:
    raise SystemExit("mir_lower_expr_compound_assign missing R-44 fixed-width path")
sem = Path("crates/syx-semantic/src/semantic_analyze_expr.rs").read_text(encoding="utf-8")
if "R-44" not in sem or "BinOp::Shl | BinOp::Shr" not in sem:
    raise SystemExit("semantic_analyze_expr missing R-44 shift left-type")
print("mir_shift_left_kind=1")
print("compound_fixed_width=1")
print("semantic_shift_left_type=1")
PY

lang_out=$(mktemp)
cargo test -p syx-driver --test lang int8_ -- --test-threads=8 >"$lang_out" 2>&1 || true
result_line=$(grep -E "^test result:" "$lang_out" | tail -1 || true)
ok "lang_int8_result_line" "$result_line"
int8_failed=$(echo "$result_line" | sed -n 's/.* \([0-9][0-9]*\) failed.*/\1/p')
int8_passed=$(echo "$result_line" | sed -n 's/.* \([0-9][0-9]*\) passed.*/\1/p')
int8_failed=${int8_failed:-999}
int8_passed=${int8_passed:-0}
ok "lang_int8_failed" "$int8_failed"
ok "lang_int8_passed" "$int8_passed"
if [[ "$int8_failed" -ne 0 || "$int8_passed" -lt 3 ]]; then
  die "contextual Int8 filter must be ≥3 passed / 0 failed"
fi
rm -f "$lang_out"

# Explicit trio nails
for nail in binary_int8_shl_wraps compound_assign_int8_plus_wraps compound_assign_int8_shl_wraps; do
  nail_out=$(mktemp)
  cargo test -p syx-driver --test lang "$nail" -- --exact --test-threads=1 >"$nail_out" 2>&1 || true
  n_line=$(grep -E "^test result:" "$nail_out" | tail -1 || true)
  n_failed=$(echo "$n_line" | sed -n 's/.* \([0-9][0-9]*\) failed.*/\1/p')
  n_failed=${n_failed:-999}
  ok "nail_${nail}_failed" "$n_failed"
  if [[ "$n_failed" -ne 0 ]]; then
    die "nail $nail must pass"
  fi
  rm -f "$nail_out"
done

python3 - <<'PY' || die "census / still-red hygiene failed"
from pathlib import Path
census = Path("docs/census/r-44-int8-wrap-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/r-44-int8-wrap-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("R-44", "Int8", "-128", "-126", "Comparable", "JIT", "≠"):
    if needle not in ct:
        raise SystemExit(f"r-44 census missing {needle}")
r29 = Path("docs/census/r-29-lang-super-census.md").read_text(encoding="utf-8")
if "Int8" in r29 and "R-44" not in r29:
    raise SystemExit("R-29 still-red must point Int8 wrap → R-44")
if "Comparable" not in r29 and "JIT" not in r29:
    raise SystemExit("R-29 must retain Comparable/JIT still-red")
print("r44_census_doc=1")
print("r29_still_red_updated=1")
PY

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

python3 - <<'PY' || die "object/simd KPI failed"
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
if total > 180 or files > 10:
    sys.exit(1)
simd = (root / "packages/core/simd.sy").read_text(encoding="utf-8")
code = "\n".join(ln for ln in simd.splitlines() if not ln.strip().startswith("//"))
simd_n = len(obj_re.findall(code))
print(f"simd_object_residual={simd_n}")
sys.exit(0 if simd_n >= 159 else 1)
PY

python3 - <<'PY' || die "roadmap R-44 exit / Comparable default missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
rows = [l for l in text.splitlines() if "| **R-44** |" in l]
if not rows or not any("`[x]`" in r or "| [x] |" in r for r in rows):
    raise SystemExit(f"R-44 not [x]: {rows}")
tail = text.split("默认下一窗", 1)
if len(tail) < 2 or "Comparable" not in tail[1][:200]:
    raise SystemExit("roadmap must default next window to Comparable residual")
print("roadmap_r44_exit=1")
print("roadmap_next_comparable=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "r44_int8_wrap_census_ok=0"
  exit 1
fi
echo "r44_int8_wrap_census_ok=1"
echo "current=R-44-int8-wrap"
echo "next=Comparable-sort-residual"
