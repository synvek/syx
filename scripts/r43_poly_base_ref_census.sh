#!/usr/bin/env bash
# R-43 polymorphic_base_ref / vtable census.
# Hard gates: erased Animal embed+wasm woof · lang_poly_base_ref_failed=0
# Soft: native ≠ GA. KPIs: wasm 46/46 · browser_failed=0 · Object≤180/10 · simd≥159
# Non-goals: full inheritance GA · R-44 JIT/Int8/Comparable · simd wipe · json bags
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-43 polymorphic_base_ref / vtable census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

# --- Fixture must be erased factory (no weak static Dog_speak luck) ---
python3 - <<'PY' || die "erased fixture nail missing"
from pathlib import Path
p = Path("crates/syx-driver/tests/fixtures/parity/polymorphic_base_ref.sy")
t = p.read_text(encoding="utf-8")
needles = ("func make() -> Animal", "let a: Animal = make()", "a.speak()")
for n in needles:
    if n not in t:
        raise SystemExit(f"fixture missing erased nail: {n}")
if "Dog().speak()" in t.split("func main")[-1]:
    raise SystemExit("fixture main must not be direct Dog().speak()")
print("fixture_erased_animal=1")
PY

# --- Lang poly base_ref + MIR call-site asserts ---
lang_out=$(mktemp)
cargo test -p syx-driver --test lang polymorphic_base_ref -- --test-threads=8 >"$lang_out" 2>&1 || true
result_line=$(grep -E "^test result:" "$lang_out" | tail -1 || true)
ok "lang_poly_base_ref_result_line" "$result_line"
poly_failed=$(echo "$result_line" | sed -n 's/.* \([0-9][0-9]*\) failed.*/\1/p')
poly_passed=$(echo "$result_line" | sed -n 's/.* \([0-9][0-9]*\) passed.*/\1/p')
poly_failed=${poly_failed:-999}
poly_passed=${poly_passed:-0}
ok "lang_poly_base_ref_failed" "$poly_failed"
ok "lang_poly_base_ref_passed" "$poly_passed"
if [[ "$poly_failed" -ne 0 || "$poly_passed" -lt 8 ]]; then
  die "polymorphic_base_ref must be ≥8 passed / 0 failed"
fi
rm -f "$lang_out"

mir_out=$(mktemp)
cargo test -p syx-driver --test lang poly_mir_ -- --test-threads=8 >"$mir_out" 2>&1 || true
mir_line=$(grep -E "^test result:" "$mir_out" | tail -1 || true)
ok "poly_mir_result_line" "$mir_line"
mir_failed=$(echo "$mir_line" | sed -n 's/.* \([0-9][0-9]*\) failed.*/\1/p')
mir_failed=${mir_failed:-999}
ok "poly_mir_failed" "$mir_failed"
if [[ "$mir_failed" -ne 0 ]]; then
  die "poly_mir_* CallIndirect asserts must be green"
fi
rm -f "$mir_out"

# --- Static evidence: MIR lower + wasm AOT CallIndirect emit ---
python3 - <<'PY' || die "R-43 MIR/wasm CallIndirect wiring missing"
from pathlib import Path
mir = Path("crates/syx-mir/src/mir_lower_expr_call_member.rs").read_text(encoding="utf-8")
if "R-43" not in mir or "CallIndirect" not in mir or "__vtable_" not in mir:
    raise SystemExit("mir_lower_expr_call_member missing R-43 vtable CallIndirect")
wasm = Path("crates/syx-wasm-emit/src/mir_emit.rs").read_text(encoding="utf-8")
if "MirOp::CallIndirect" not in wasm or "TakeRtI32" not in wasm:
    raise SystemExit("wasm mir_emit missing CallIndirect → TakeRtI32 table dispatch")
print("mir_vtable_callindirect=1")
print("wasm_callindirect_emit=1")
PY

# --- Census doc + R-29 still-red hygiene ---
python3 - <<'PY' || die "census / R-29 still-red hygiene failed"
from pathlib import Path
census = Path("docs/census/r-43-poly-base-ref-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/r-43-poly-base-ref-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("R-43", "woof", "CallIndirect", "__vtable_", "erase", "wasm", "≠", "R-44"):
    if needle not in ct:
        raise SystemExit(f"r-43 census missing {needle}")
r29 = Path("docs/census/r-29-lang-super-census.md").read_text(encoding="utf-8")
if "polymorphic_base_ref" in r29 and "R-43" not in r29:
    raise SystemExit("R-29 still-red must point poly base_ref → R-43")
if "JIT differential" not in r29 and "Comparable" not in r29:
    raise SystemExit("R-29 must retain R-44 still-red items")
print("r43_census_doc=1")
print("r29_still_red_updated=1")
PY

# --- Hard KPIs ---
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

# Roadmap exit pointer (checked after docs update in same window)
python3 - <<'PY' || die "roadmap R-43 exit / R-44 default missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
rows = [l for l in text.splitlines() if "| **R-43** |" in l]
if not rows or not any("`[x]`" in r or "| [x] |" in r for r in rows):
    raise SystemExit(f"R-43 not [x]: {rows}")
tail = text.split("默认下一窗", 1)
if len(tail) < 2 or "R-44" not in tail[1][:160]:
    raise SystemExit("roadmap must default next window to R-44")
print("roadmap_r43_exit=1")
print("roadmap_next_r44=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "r43_poly_base_ref_census_ok=0"
  exit 1
fi
echo "r43_poly_base_ref_census_ok=1"
echo "current=R-43-poly-base-ref"
echo "next=R-44-jit-int8-comparable"
