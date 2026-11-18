#!/usr/bin/env bash
# C-0b return type assignability census.
# Soft park: C-0c arity · C-0d &mut · C-0a Unknown · C-0e narrowing · CallSuper · JIT · SyxHost · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-0b return-type census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "C-0b surface missing"
from pathlib import Path
stmt = Path("crates/syx-semantic/src/semantic_analyze_stmt.rs").read_text(encoding="utf-8")
if "check_assignable_with_literal" not in stmt or '"return"' not in stmt:
    raise SystemExit("Stmt::Return missing check_assignable_with_literal for return")
if "function_return_types.get(&fn_name)" not in stmt:
    raise SystemExit("Stmt::Return missing function_return_types lookup")
member = Path("crates/syx-semantic/src/semantic_analyze_type_member.rs").read_text(encoding="utf-8")
if "{type_name}_{method}" not in member:
    raise SystemExit("method path missing mangled current_function")
nail = Path("crates/syx-driver/tests/lang/type_soundness.rs").read_text(encoding="utf-8")
for needle in (
    "c0b_wrong_return_string_to_int_s004",
    "c0b_correct_return_int_ok",
    "c0b_method_wrong_return_s004",
):
    if needle not in nail:
        raise SystemExit(f"type_soundness missing {needle}")
main = Path("crates/syx-driver/tests/lang/main.rs").read_text(encoding="utf-8")
if "mod type_soundness;" not in main:
    raise SystemExit("lang/main.rs missing mod type_soundness")
print("c0b_surface=1")
print("c0b_lang_nail=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/c-0b-return-type-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/c-0b-return-type-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("C-0b", "S004", "≠", "C-0c", "Unknown"):
    if needle not in ct:
        raise SystemExit(f"c-0b census missing {needle}")
print("c0b_census_doc=1")
PY

SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

WRONG=$(mktemp /tmp/c0b_wrong_XXXX.sy)
cat >"$WRONG" <<'EOF'
func f() -> Int { return "wrong" }
func main() { }
EOF
set +e
out=$("$SYX" check "$WRONG" 2>&1)
ec=$?
set -e
if [[ "$ec" -eq 0 ]]; then
  die "wrong return must fail check"
fi
if ! echo "$out" | grep -q '\[S004\]'; then
  echo "$out" >&2
  die "wrong return must emit S004"
fi
if echo "$out" | grep -q 'Unknown'; then
  echo "$out" >&2
  die "diag must not print literal Unknown"
fi
ok "probe_wrong_return" "S004"
rm -f "$WRONG"

OKF=$(mktemp /tmp/c0b_ok_XXXX.sy)
cat >"$OKF" <<'EOF'
func f() -> Int { return 1 }
func main() { }
EOF
"$SYX" check "$OKF" >/tmp/c0b_ok_check.txt 2>&1 || {
  cat /tmp/c0b_ok_check.txt
  die "correct return must check green"
}
ok "probe_ok_return" "0"
rm -f "$OKF"

METH=$(mktemp /tmp/c0b_meth_XXXX.sy)
cat >"$METH" <<'EOF'
struct Box {
    func value() -> Int { return "no" }
}
func main() { }
EOF
set +e
mout=$("$SYX" check "$METH" 2>&1)
mec=$?
set -e
if [[ "$mec" -eq 0 ]] || ! echo "$mout" | grep -q '\[S004\]'; then
  echo "$mout" >&2
  die "method wrong return must S004"
fi
ok "probe_method_wrong_return" "S004"
rm -f "$METH"

cargo test -p syx-driver --test lang type_soundness -- --test-threads=4 \
  >/tmp/c0b_lang_nail.txt 2>&1 || {
  tail -40 /tmp/c0b_lang_nail.txt
  die "lang type_soundness nail failed"
}
ok "lang_nail" "type_soundness"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c0b_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c0b_book_out.txt
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

python3 - <<'PY' || die "roadmap C-0b exit / next C-0c missing"
from pathlib import Path
import re
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
rows = [l for l in text.splitlines() if "C-0b" in l and "[x]" in l]
if not rows:
    raise SystemExit("C-0b not [x]: scan failed")
if "C-0c" not in text:
    raise SystemExit("roadmap missing C-0c")
header = "\n".join(text.splitlines()[:12])
if "C-0c" not in header and not any("C-0c" in l and "默认下一" in l for l in text.splitlines()):
    raise SystemExit("roadmap default next must be C-0c")
# Fail only on affirmative "next = R-50/U-41", not "无 R-50" park notes.
if re.search(r"默认下一\s*=\s*\*?\*?R-50", text) or re.search(r"默认下一\s*=\s*\*?\*?U-41", text):
    raise SystemExit("roadmap must not default next to R-50/U-41")
print("roadmap_c0b_exit=1")
print("roadmap_next_c0c=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "c0b_return_type_census_ok=0"
  exit 1
fi
echo "c0b_return_type_census_ok=1"
echo "current=C-0b-return-type"
echo "next=C-0c"
