#!/usr/bin/env bash
# U-23 let-member S060 census (MemberAssign on immutable struct Ident; class/actor OK).
# Soft park: Ch15/Ch28 S020 · comma while-let · closure throws · CST · Break · JIT · SyxHost · D3 · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== U-23 let-member S060 census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "semantic U-23 surface missing"
from pathlib import Path
call = Path("crates/syx-semantic/src/semantic_analyze_call.rs").read_text(encoding="utf-8")
if "check_member_assign_on_immutable" not in call or "U-23" not in call:
    raise SystemExit("missing check_member_assign_on_immutable / U-23")
expr = Path("crates/syx-semantic/src/semantic_analyze_expr.rs").read_text(encoding="utf-8")
if "check_member_assign_on_immutable" not in expr or "U-23" not in expr:
    raise SystemExit("MemberAssign missing U-23 call")
nail = Path("crates/syx-driver/tests/lang/let_member_assign_s060.rs")
if not nail.is_file():
    raise SystemExit("missing lang/let_member_assign_s060.rs")
nt = nail.read_text(encoding="utf-8")
for needle in (
    "u23_let_struct_member_is_s060",
    "u23_let_class_member_ok",
    "u23_var_struct_member_ok",
    "u23_let_subscript_still_s060",
    "u23_let_append_still_s060",
    "u23_let_class_member_runs",
    "u23_var_struct_member_runs",
):
    if needle not in nt:
        raise SystemExit(f"let_member_assign_s060 missing {needle}")
main = Path("crates/syx-driver/tests/lang/main.rs").read_text(encoding="utf-8")
if "mod let_member_assign_s060;" not in main:
    raise SystemExit("lang/main.rs missing mod let_member_assign_s060")
# books thin
for rel in (
    "books/language-guide/10-structures-and-classes.md",
    "books/zh-cn/language-guide/10-structures-and-classes.md",
    "books/language-guide/appendix-b-diagnostic-index.md",
    "books/zh-cn/language-guide/appendix-b-diagnostic-index.md",
):
    t = Path(rel).read_text(encoding="utf-8")
    if "S060" not in t:
        raise SystemExit(f"{rel} missing S060")
    if "10-structures" in rel:
        if "struct" not in t.lower() and "成员" not in t:
            raise SystemExit(f"{rel} missing struct member S060 note")
    if "appendix-b" in rel:
        if "member" not in t.lower() and "成员" not in t:
            raise SystemExit(f"{rel} missing member/成员 S060 note")
print("u23_semantic_surface=1")
print("u23_lang_nail=1")
print("u23_books_thin=1")
PY

python3 - <<'PY' || die "census doc missing"
from pathlib import Path
census = Path("docs/census/u-23-let-member-assign-s060-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/u-23-let-member-assign-s060-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("U-23", "S060", "MemberAssign", "≠", "R-50", "JIT"):
    if needle not in ct:
        raise SystemExit(f"u-23 census missing {needle}")
print("u23_census_doc=1")
PY

SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

STRUCT=$(mktemp /tmp/u23_struct_XXXX.sy)
cat >"$STRUCT" <<'EOF'
struct Point {
    var x: Int
    var y: Int
}
public func main() {
    let p = Point(x: 1, y: 2)
    p.x = 3
}
EOF
if "$SYX" check "$STRUCT" >/tmp/u23_struct_out.txt 2>&1; then
  die "let struct member must fail check"
fi
if ! grep -q 'S060' /tmp/u23_struct_out.txt; then
  cat /tmp/u23_struct_out.txt
  die "let struct member must report S060"
fi
ok "probe_let_struct_s060" "1"
rm -f "$STRUCT"

CLASS=$(mktemp /tmp/u23_class_XXXX.sy)
cat >"$CLASS" <<'EOF'
class Box {
    var n: Int
    init(n: Int) { self.n = n }
}
public func main() {
    let b = Box(n: 1)
    b.n = 9
    print(b.n)
}
EOF
out=$("$SYX" run "$CLASS" 2>/tmp/u23_class_err.txt) || {
  cat /tmp/u23_class_err.txt
  die "let class member run failed"
}
if [[ "$out" != "9" ]]; then
  echo "got: $out" >&2
  die "let class member output mismatch"
fi
ok "probe_let_class" "9"
rm -f "$CLASS"

VAR=$(mktemp /tmp/u23_var_XXXX.sy)
cat >"$VAR" <<'EOF'
struct Point {
    var x: Int
    var y: Int
}
public func main() {
    var p = Point(x: 1, y: 2)
    p.x = 3
    print(p.x)
}
EOF
out=$("$SYX" run "$VAR" 2>/tmp/u23_var_err.txt) || {
  cat /tmp/u23_var_err.txt
  die "var struct member run failed"
}
if [[ "$out" != "3" ]]; then
  echo "got: $out" >&2
  die "var struct member output mismatch"
fi
ok "probe_var_struct" "3"
rm -f "$VAR"

ARR=$(mktemp /tmp/u23_arr_XXXX.sy)
cat >"$ARR" <<'EOF'
public func main() {
    let a: Array<Int> = [1]
    a[0] = 2
}
EOF
if "$SYX" check "$ARR" >/tmp/u23_arr_out.txt 2>&1; then
  die "let Array subscript must still fail check"
fi
if ! grep -q 'S060' /tmp/u23_arr_out.txt; then
  cat /tmp/u23_arr_out.txt
  die "let Array subscript must still report S060"
fi
ok "probe_let_subscript_s060" "1"
rm -f "$ARR"

cargo test -p syx-driver --test lang let_member_assign_s060 -- --test-threads=4 \
  >/tmp/u23_lang_nail.txt 2>&1 || {
  tail -40 /tmp/u23_lang_nail.txt
  die "lang let_member_assign_s060 nail failed"
}
ok "lang_nail" "let_member_assign_s060"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/u23_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/u23_book_out.txt
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

python3 - <<'PY' || die "roadmap U-23 exit / next park missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if "U-23" not in text:
    raise SystemExit("roadmap missing U-23")
rows = [l for l in text.splitlines() if "| **U-23** |" in l]
if not rows or not any("[x]" in r for r in rows):
    raise SystemExit(f"U-23 not [x]: {rows}")
if "默认下一" in text:
    chunk = text.split("默认下一", 1)[1][:500]
    if "R-50" in chunk and "不" not in chunk and "park" not in chunk.lower() and "未编号" not in chunk:
        raise SystemExit("roadmap must not default next to R-50")
print("roadmap_u23_exit=1")
print("roadmap_no_r50_default=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "u23_let_member_assign_s060_census_ok=0"
  exit 1
fi
echo "u23_let_member_assign_s060_census_ok=1"
echo "current=U-23-let-member-assign-s060"
echo "next=park"
