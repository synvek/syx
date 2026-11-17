#!/usr/bin/env bash
# U-28 let-enum MemberAssign S060 census (enum_cases root + dataclass→enum hop).
# Soft park: self.x= · Ch12 MD · enum type/name gate · comma while-let · closure throws · CST · Break · JIT · SyxHost · D3 · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== U-28 let-enum MemberAssign S060 census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "semantic U-28 surface missing"
from pathlib import Path
call = Path("crates/syx-semantic/src/semantic_analyze_call.rs").read_text(encoding="utf-8")
if "enum_cases" not in call or "U-28" not in call:
    raise SystemExit("missing enum_cases / U-28 in value_member_assign_immutable_root")
if "value_member_assign_immutable_root" not in call:
    raise SystemExit("missing value_member_assign_immutable_root")
expr = Path("crates/syx-semantic/src/semantic_analyze_expr.rs").read_text(encoding="utf-8")
if "U-28" not in expr:
    raise SystemExit("MemberAssign missing U-28 note")
nail = Path("crates/syx-driver/tests/lang/let_enum_member_assign_s060.rs")
if not nail.is_file():
    raise SystemExit("missing lang/let_enum_member_assign_s060.rs")
nt = nail.read_text(encoding="utf-8")
for needle in (
    "u28_let_enum_payload_is_s060",
    "u28_let_nested_enum_is_s060",
    "u28_var_enum_ok",
    "u28_let_tuple_still_s060",
    "u28_let_struct_still_s060",
    "u28_var_enum_runs",
):
    if needle not in nt:
        raise SystemExit(f"let_enum_member_assign_s060 missing {needle}")
main = Path("crates/syx-driver/tests/lang/main.rs").read_text(encoding="utf-8")
if "mod let_enum_member_assign_s060;" not in main:
    raise SystemExit("lang/main.rs missing mod let_enum_member_assign_s060")
for rel in (
    "books/language-guide/09-enumerations.md",
    "books/zh-cn/language-guide/09-enumerations.md",
    "books/language-guide/appendix-b-diagnostic-index.md",
    "books/zh-cn/language-guide/appendix-b-diagnostic-index.md",
):
    t = Path(rel).read_text(encoding="utf-8")
    if "S060" not in t:
        raise SystemExit(f"{rel} missing S060")
    if "09-enumerations" in rel or "appendix-b" in rel:
        if "enum" not in t.lower() and "枚举" not in t and "关联" not in t:
            raise SystemExit(f"{rel} missing enum/关联 S060 note")
print("u28_semantic_surface=1")
print("u28_lang_nail=1")
print("u28_books_thin=1")
PY

python3 - <<'PY' || die "census doc missing"
from pathlib import Path
census = Path("docs/census/u-28-let-enum-member-assign-s060-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/u-28-let-enum-member-assign-s060-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("U-28", "S060", "enum", "≠", "R-50", "JIT"):
    if needle not in ct:
        raise SystemExit(f"u-28 census missing {needle}")
print("u28_census_doc=1")
PY

SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

LET=$(mktemp /tmp/u28_let_XXXX.sy)
cat >"$LET" <<'EOF'
enum Box { case wrap(payload: Int) }
public func main() {
    let e = Box.wrap(payload: 1)
    e.payload = 9
}
EOF
if "$SYX" check "$LET" >/tmp/u28_let_out.txt 2>&1; then
  die "let enum payload must fail check"
fi
if ! grep -q 'S060' /tmp/u28_let_out.txt; then
  cat /tmp/u28_let_out.txt
  die "let enum payload must report S060"
fi
ok "probe_let_enum_s060" "1"
rm -f "$LET"

NEST=$(mktemp /tmp/u28_nest_XXXX.sy)
cat >"$NEST" <<'EOF'
enum E { case wrap(payload: Int) }
struct W { var e: E }
public func main() {
    let w = W(e: E.wrap(payload: 1))
    w.e.payload = 9
}
EOF
if "$SYX" check "$NEST" >/tmp/u28_nest_out.txt 2>&1; then
  die "let nested enum must fail check"
fi
if ! grep -q 'S060' /tmp/u28_nest_out.txt; then
  cat /tmp/u28_nest_out.txt
  die "let nested enum must report S060"
fi
ok "probe_let_nested_enum_s060" "1"
rm -f "$NEST"

VAR=$(mktemp /tmp/u28_var_XXXX.sy)
cat >"$VAR" <<'EOF'
enum Box { case wrap(payload: Int) }
public func main() {
    var e = Box.wrap(payload: 1)
    e.payload = 9
    print(e.payload)
}
EOF
out=$("$SYX" run "$VAR" 2>/tmp/u28_var_err.txt) || {
  cat /tmp/u28_var_err.txt
  die "var enum run failed"
}
if [[ "$out" != "9" ]]; then
  echo "got: $out" >&2
  die "var enum output mismatch"
fi
ok "probe_var_enum" "9"
rm -f "$VAR"

TUP=$(mktemp /tmp/u28_tup_XXXX.sy)
cat >"$TUP" <<'EOF'
public func main() {
    let t = (1, 2)
    t.0 = 3
}
EOF
if "$SYX" check "$TUP" >/tmp/u28_tup_out.txt 2>&1; then
  die "let tuple must still fail check"
fi
if ! grep -q 'S060' /tmp/u28_tup_out.txt; then
  cat /tmp/u28_tup_out.txt
  die "let tuple must still report S060"
fi
ok "probe_let_tuple_s060" "1"
rm -f "$TUP"

cargo test -p syx-driver --test lang let_enum_member_assign_s060 -- --test-threads=4 \
  >/tmp/u28_lang_nail.txt 2>&1 || {
  tail -40 /tmp/u28_lang_nail.txt
  die "lang let_enum_member_assign_s060 nail failed"
}
ok "lang_nail" "let_enum_member_assign_s060"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/u28_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/u28_book_out.txt
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

python3 - <<'PY' || die "roadmap U-28 exit / next park missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if "U-28" not in text:
    raise SystemExit("roadmap missing U-28")
rows = [l for l in text.splitlines() if "| **U-28** |" in l]
if not rows or not any("[x]" in r for r in rows):
    raise SystemExit(f"U-28 not [x]: {rows}")
if "默认下一" in text:
    chunk = text.split("默认下一", 1)[1][:500]
    if "R-50" in chunk and "不" not in chunk and "park" not in chunk.lower() and "未编号" not in chunk:
        raise SystemExit("roadmap must not default next to R-50")
print("roadmap_u28_exit=1")
print("roadmap_no_r50_default=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "u28_let_enum_member_assign_s060_census_ok=0"
  exit 1
fi
echo "u28_let_enum_member_assign_s060_census_ok=1"
echo "current=U-28-let-enum-member-assign-s060"
echo "next=park"
