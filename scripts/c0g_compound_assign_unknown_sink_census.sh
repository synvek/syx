#!/usr/bin/env bash
# C-0g CompoundAssign Unknown→ground sink census (S004).
# Soft park: C-0a+ 三分 · close soft arm · C-0e · C-3/C-5 [x] · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-0g compound-assign-unknown-sink census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "C-0g surface missing"
from pathlib import Path

expr = Path("crates/syx-semantic/src/semantic_analyze_expr.rs").read_text(encoding="utf-8")
if "C-0g" not in expr or "check_unknown_to_ground_sink" not in expr:
    raise SystemExit("CompoundAssign must host C-0g sink")
idx_ca = expr.find("Expr::CompoundAssign")
idx_ma = expr.find("Expr::MemberAssign", idx_ca if idx_ca >= 0 else 0)
region = expr[idx_ca:idx_ma] if idx_ca >= 0 and idx_ma > idx_ca else ""
if "C-0g: compound-assign" not in region or "check_unknown_to_ground_sink" not in region:
    raise SystemExit("C-0g sink must be inside CompoundAssign arm")

tc = Path("crates/syx-semantic/src/type_checker.rs").read_text(encoding="utf-8")
if "(_, TypeId::Unknown) => true" not in tc:
    raise SystemExit("must not remove (_, Unknown) soft arm")

nail = Path("crates/syx-driver/tests/lang/type_soundness.rs").read_text(encoding="utf-8")
for needle in (
    "c0g_untyped_compound_assign_int_s004",
    "c0g_literal_compound_still_ok",
    "c0g_untyped_compound_assign_any_still_ok",
    "c0f_untyped_arg_to_int_param_s004",
):
    if needle not in nail:
        raise SystemExit(f"lang nail missing {needle}")

print("c0g_surface=1")
print("c0g_lang_nail=1")
print("c0g_soft_arms=1")
PY

python3 - <<'PY' || die "ledger / census hygiene failed"
from pathlib import Path

census = Path("docs/census/c-0g-compound-assign-unknown-sink-census.md")
if not census.is_file():
    raise SystemExit("missing c-0g census")
ct = census.read_text(encoding="utf-8")
for needle in ("C-0g", "S004", "跨轴", "check_unknown_to_ground_sink", "CompoundAssign"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")

rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("**C-0g**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-0g [x]")
if not any("**C-0a+**" in l and "park" in l for l in rm.splitlines()):
    raise SystemExit("02 must keep C-0a+ park")

st = Path("plan/01-project-status.md").read_text(encoding="utf-8")
if "C-0g" not in st:
    raise SystemExit("01 must cite C-0g")

ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
next_sec = ssot.split("## Next", 1)[-1]
if "跨轴" not in next_sec or "C-0g" not in next_sec:
    raise SystemExit("SSOT Next must cite post-C-0g 跨轴")

cl = Path("CHANGELOG.md").read_text(encoding="utf-8")
head = cl.split("### SYX 0.4.0", 1)[-1].split("###", 1)[0]
if "C-0g" not in head:
    raise SystemExit("CHANGELOG must mention C-0g")

print("c0g_census_doc=1")
print("c0g_roadmap_ok=1")
print("c0g_ssot_updated=1")
PY

echo "== nails cargo =="
cargo test -p syx-driver --test lang -- c0g_ --nocapture >/tmp/c0g_lang.txt 2>&1 \
  || die "c0g lang nails failed"
if ! grep -q "c0g_untyped_compound_assign_int_s004 ... ok" /tmp/c0g_lang.txt; then
  die "c0g S004 nail did not pass"
fi
ok "c0g_lang" "1"
tail -20 /tmp/c0g_lang.txt

cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
SYX=./target/debug/syx

PROBE=$(mktemp /tmp/c0g_probe_XXXX.sy)
cat >"$PROBE" <<'EOF'
func bump(_ x) {
    var n: Int = 0
    n += x
}
func main() { bump(1) }
EOF
set +e
out=$("$SYX" check "$PROBE" --enable-std=false 2>&1)
ec=$?
set -e
if [[ "$ec" -eq 0 ]] || ! echo "$out" | grep -q '\[S004\]'; then
  echo "$out" >&2
  die "untyped compound-assign to Int must S004"
fi
if echo "$out" | grep -q 'Unknown'; then
  echo "$out" >&2
  die "diag must not print literal Unknown"
fi
ok "probe_compound_assign" "S004"
rm -f "$PROBE"

LIT=$(mktemp /tmp/c0g_lit_XXXX.sy)
cat >"$LIT" <<'EOF'
func main() {
    var n: Int = 0
    n += 1
}
EOF
"$SYX" check "$LIT" --enable-std=false >/tmp/c0g_lit_check.txt 2>&1 || {
  cat /tmp/c0g_lit_check.txt
  die "literal compound-assign must still ok"
}
ok "probe_literal" "0"
rm -f "$LIT"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c0g_book_out.txt 2>&1 || die "opt-in output pins failed"
python3 - <<PY || die "sy,output must be 12/12 (en+zh)"
import json, sys
from pathlib import Path
d = json.loads(Path("$out_json").read_text())
ok_n = d.get("output_run_ok", 0)
tot = d.get("output_run_total", 0)
en = [r for r in d["results"] if "zh-cn" not in r["file"]]
zh = [r for r in d["results"] if "zh-cn" in r["file"]]
print(f"output_run_ok={ok_n}")
print(f"output_run_total={tot}")
sys.exit(0 if ok_n == 12 and tot == 12 and len(en) == 6 and len(zh) == 6 else 1)
PY
rm -f "$out_json"

browser_out=$(mktemp)
cargo test -p syx-driver --test browser -- --test-threads=8 >"$browser_out" 2>&1 || true
b_line=$(grep -E "^test result:" "$browser_out" | tail -1 || true)
ok "browser_result_line" "$b_line"
browser_failed=$(echo "$b_line" | sed -n 's/.* \([0-9][0-9]*\) failed.*/\1/p')
browser_failed=${browser_failed:-999}
ok "browser_failed" "$browser_failed"
[[ "$browser_failed" -eq 0 ]] || die "browser_failed must be 0"
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
[[ "$wasm_failed" -eq 0 && "$wasm_passed" -eq 46 ]] || die "wasm must be 46/46"
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
print(f"simd_object_residual={len(obj_re.findall(code))}")
sys.exit(0 if len(obj_re.findall(code)) >= 159 else 1)
PY

if [[ "$fail" -ne 0 ]]; then
  echo "c0g_compound_assign_unknown_sink_census_ok=0"
  exit 1
fi
echo "c0g_compound_assign_unknown_sink_census_ok=1"
echo "current=C-0g-compound-assign-unknown-sink"
echo "next=cross-axis"
