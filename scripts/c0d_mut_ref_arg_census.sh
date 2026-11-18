#!/usr/bin/env bash
# C-0d &mut bare-value arg census (S004).
# Soft park: shared & bare · methods · delete is_compatible arm · C-0a · C-0e · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-0d mut-ref-arg census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "C-0d surface missing"
from pathlib import Path
call = Path("crates/syx-semantic/src/semantic_analyze_call.rs").read_text(encoding="utf-8")
if "is_explicit_mut_borrow" not in call or "peel_expr_groups" not in call:
    raise SystemExit("call path missing mut-borrow AST helpers")
if "C-0d" not in call:
    raise SystemExit("call path missing C-0d gate comment")
# Must NOT have removed the is_compatible MutReference loose arm
tc = Path("crates/syx-semantic/src/type_checker.rs").read_text(encoding="utf-8")
if "TypeId::MutReference(exp), found) if is_compatible(exp, found)" not in tc.replace(" ", ""):
    # tolerate formatting
    if "(TypeId::MutReference(exp), found) if is_compatible(exp, found) => true" not in tc:
        raise SystemExit("do not delete is_compatible MutReference bare-value arm this window")
ana = Path("crates/syx-semantic/src/semantic_analyzer.rs").read_text(encoding="utf-8")
if "fn deref_type" not in ana:
    raise SystemExit("deref_type must remain")
nail = Path("crates/syx-driver/tests/lang/type_soundness.rs").read_text(encoding="utf-8")
for needle in (
    "c0d_bare_value_to_mut_ref_s004",
    "c0d_explicit_mut_borrow_ok",
    "c0d_shared_borrow_to_mut_ref_s004",
    "c0d_shared_ref_bare_value_still_ok",
    "c0d_grouped_mut_borrow_ok",
):
    if needle not in nail:
        raise SystemExit(f"type_soundness missing {needle}")
main = Path("crates/syx-driver/tests/lang/main.rs").read_text(encoding="utf-8")
if "mod type_soundness;" not in main:
    raise SystemExit("lang/main.rs missing mod type_soundness")
print("c0d_surface=1")
print("c0d_lang_nail=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/c-0d-mut-ref-arg-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/c-0d-mut-ref-arg-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("C-0d", "S004", "≠", "C-0a", "deref_type"):
    if needle not in ct:
        raise SystemExit(f"c-0d census missing {needle}")
print("c0d_census_doc=1")
PY

SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

BARE=$(mktemp /tmp/c0d_bare_XXXX.sy)
cat >"$BARE" <<'EOF'
func bump(_ x: &mut Int) { x = x + 1 }
func main() {
    var n = 0
    bump(n)
}
EOF
set +e
out=$("$SYX" check "$BARE" --enable-std=false 2>&1)
ec=$?
set -e
if [[ "$ec" -eq 0 ]] || ! echo "$out" | grep -q '\[S004\]'; then
  echo "$out" >&2
  die "bare value must S004"
fi
if ! echo "$out" | grep -q 'expected &mut Int'; then
  echo "$out" >&2
  die "diag must mention expected &mut Int"
fi
ok "probe_bare" "S004"
rm -f "$BARE"

OKF=$(mktemp /tmp/c0d_ok_XXXX.sy)
cat >"$OKF" <<'EOF'
func bump(_ x: &mut Int) { x = x + 1 }
func main() {
    var n = 0
    bump(&mut n)
}
EOF
"$SYX" check "$OKF" --enable-std=false >/tmp/c0d_ok_check.txt 2>&1 || {
  cat /tmp/c0d_ok_check.txt
  die "explicit &mut must check green"
}
ok "probe_mut_borrow" "0"
rm -f "$OKF"

GRP=$(mktemp /tmp/c0d_grp_XXXX.sy)
cat >"$GRP" <<'EOF'
func bump(_ x: &mut Int) { x = x + 1 }
func main() {
    var n = 0
    bump((&mut n))
}
EOF
"$SYX" check "$GRP" --enable-std=false >/tmp/c0d_grp_check.txt 2>&1 || {
  cat /tmp/c0d_grp_check.txt
  die "grouped &mut must check green"
}
ok "probe_grouped" "0"
rm -f "$GRP"

SHR=$(mktemp /tmp/c0d_shr_XXXX.sy)
cat >"$SHR" <<'EOF'
func id(_ x: &Int) -> Int { return 1 }
func main() {
    var n = 0
    let _ = id(n)
}
EOF
"$SYX" check "$SHR" --enable-std=false >/tmp/c0d_shr_check.txt 2>&1 || {
  cat /tmp/c0d_shr_check.txt
  die "shared & bare value must stay green"
}
ok "probe_shared_bare" "0"
rm -f "$SHR"

cargo test -p syx-driver --test lang type_soundness -- --test-threads=4 \
  >/tmp/c0d_lang_nail.txt 2>&1 || {
  tail -40 /tmp/c0d_lang_nail.txt
  die "lang type_soundness nail failed"
}
cargo test -p syx-driver --test lang exclusive_borrow -- --test-threads=4 \
  >/tmp/c0d_excl_nail.txt 2>&1 || {
  tail -40 /tmp/c0d_excl_nail.txt
  die "lang exclusive_borrow nail failed"
}
ok "lang_nail" "type_soundness+exclusive_borrow"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c0d_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c0d_book_out.txt
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
print(f"output_en={len(en)} output_zh={len(zh)}")
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

python3 - <<'PY' || die "roadmap C-0d exit / next C-0a missing"
from pathlib import Path
import re
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
rows = [l for l in text.splitlines() if "C-0d" in l and "[x]" in l]
if not rows:
    raise SystemExit("C-0d not [x]")
header = "\n".join(text.splitlines()[:12])
if "C-0a" not in header and not any("C-0a" in l and "默认下一" in l for l in text.splitlines()):
    raise SystemExit("roadmap default next must be C-0a")
if re.search(r"默认下一\s*=\s*\*?\*?R-50", text) or re.search(r"默认下一\s*=\s*\*?\*?U-41", text):
    raise SystemExit("roadmap must not default next to R-50/U-41")
print("roadmap_c0d_exit=1")
print("roadmap_next_c0a=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "c0d_mut_ref_arg_census_ok=0"
  exit 1
fi
echo "c0d_mut_ref_arg_census_ok=1"
echo "current=C-0d-mut-ref-arg"
echo "next=C-0a"
