#!/usr/bin/env bash
# C-0c free-function call arity census (S061).
# Soft park: methods · generics · host · label-slot · C-0d &mut · C-0a Unknown · C-0e · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-0c call-arity census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "C-0c surface missing"
from pathlib import Path
ana = Path("crates/syx-semantic/src/semantic_analyzer.rs").read_text(encoding="utf-8")
if "function_param_has_default" not in ana:
    raise SystemExit("missing function_param_has_default field")
col = Path("crates/syx-semantic/src/semantic_collect_decls.rs").read_text(encoding="utf-8")
if "function_param_has_default" not in col or "p.default.is_some()" not in col:
    raise SystemExit("collect_decls missing default flags")
call = Path("crates/syx-semantic/src/semantic_analyze_call.rs").read_text(encoding="utf-8")
if '"S061"' not in call:
    raise SystemExit("call path missing S061")
if "function_param_has_default" not in call:
    raise SystemExit("call path missing default lookup")
nail = Path("crates/syx-driver/tests/lang/type_soundness.rs").read_text(encoding="utf-8")
for needle in (
    "c0c_too_few_args_s061",
    "c0c_too_many_args_s061",
    "c0c_exact_arity_ok",
    "c0c_trailing_default_omit_ok",
    "c0c_wrong_arg_type_still_s004",
    "c0c_missing_label_still_s042",
):
    if needle not in nail:
        raise SystemExit(f"type_soundness missing {needle}")
en = Path("books/language-guide/appendix-b-diagnostic-index.md").read_text(encoding="utf-8")
zh = Path("books/zh-cn/language-guide/appendix-b-diagnostic-index.md").read_text(encoding="utf-8")
if "S061" not in en or "S061" not in zh:
    raise SystemExit("appendix-B missing S061")
print("c0c_surface=1")
print("c0c_lang_nail=1")
print("c0c_appendix=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/c-0c-call-arity-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/c-0c-call-arity-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("C-0c", "S061", "≠", "C-0d", "default"):
    if needle not in ct:
        raise SystemExit(f"c-0c census missing {needle}")
print("c0c_census_doc=1")
PY

SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

FEW=$(mktemp /tmp/c0c_few_XXXX.sy)
cat >"$FEW" <<'EOF'
func sum2(_ a: Int, _ b: Int) -> Int { return a }
func main() { sum2(1) }
EOF
set +e
out=$("$SYX" check "$FEW" 2>&1)
ec=$?
set -e
if [[ "$ec" -eq 0 ]] || ! echo "$out" | grep -q '\[S061\]'; then
  echo "$out" >&2
  die "too-few must S061"
fi
ok "probe_too_few" "S061"
rm -f "$FEW"

MANY=$(mktemp /tmp/c0c_many_XXXX.sy)
cat >"$MANY" <<'EOF'
func sum2(_ a: Int, _ b: Int) -> Int { return a }
func main() { sum2(1, 2, 3) }
EOF
set +e
out=$("$SYX" check "$MANY" 2>&1)
ec=$?
set -e
if [[ "$ec" -eq 0 ]] || ! echo "$out" | grep -q '\[S061\]'; then
  echo "$out" >&2
  die "too-many must S061"
fi
ok "probe_too_many" "S061"
rm -f "$MANY"

OKF=$(mktemp /tmp/c0c_ok_XXXX.sy)
cat >"$OKF" <<'EOF'
func sum2(_ a: Int, _ b: Int) -> Int { return a }
func main() { sum2(1, 2) }
EOF
"$SYX" check "$OKF" >/tmp/c0c_ok_check.txt 2>&1 || {
  cat /tmp/c0c_ok_check.txt
  die "exact arity must check green"
}
ok "probe_exact" "0"
rm -f "$OKF"

DEF=$(mktemp /tmp/c0c_def_XXXX.sy)
cat >"$DEF" <<'EOF'
func f(_ x: Int = 1) -> Int { return x }
func main() { print(f()); print(f(2)) }
EOF
"$SYX" check "$DEF" >/tmp/c0c_def_check.txt 2>&1 || {
  cat /tmp/c0c_def_check.txt
  die "trailing default omit must check green"
}
ok "probe_default_omit" "0"
rm -f "$DEF"

cargo test -p syx-driver --test lang type_soundness -- --test-threads=4 \
  >/tmp/c0c_lang_nail.txt 2>&1 || {
  tail -40 /tmp/c0c_lang_nail.txt
  die "lang type_soundness nail failed"
}
ok "lang_nail" "type_soundness"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c0c_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c0c_book_out.txt
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

python3 - <<'PY' || die "roadmap C-0c exit / next C-0d missing"
from pathlib import Path
import re
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
rows = [l for l in text.splitlines() if "C-0c" in l and "[x]" in l]
if not rows:
    raise SystemExit("C-0c not [x]")
header = "\n".join(text.splitlines()[:12])
if "C-0d" not in header and not any("C-0d" in l and "默认下一" in l for l in text.splitlines()):
    raise SystemExit("roadmap default next must be C-0d")
if re.search(r"默认下一\s*=\s*\*?\*?R-50", text) or re.search(r"默认下一\s*=\s*\*?\*?U-41", text):
    raise SystemExit("roadmap must not default next to R-50/U-41")
print("roadmap_c0c_exit=1")
print("roadmap_next_c0d=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "c0c_call_arity_census_ok=0"
  exit 1
fi
echo "c0c_call_arity_census_ok=1"
echo "current=C-0c-call-arity"
echo "next=C-0d"
