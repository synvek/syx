#!/usr/bin/env bash
# U-29 nested enum/tuple MemberAssign S060 census (enum↔enum / enum→tuple / tuple→enum).
# Soft park: self.x= · Ch12 MD · enum type/name gate · comma while-let · closure throws · CST · Break · JIT · SyxHost · D3 · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== U-29 nested enum/tuple MemberAssign S060 census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "semantic U-29 surface missing"
from pathlib import Path
call = Path("crates/syx-semantic/src/semantic_analyze_call.rs").read_text(encoding="utf-8")
if "value_product_field_ty" not in call or "U-29" not in call:
    raise SystemExit("missing value_product_field_ty / U-29 in value_member_assign_immutable_root")
if "enum_case_field_types" not in call:
    raise SystemExit("missing enum_case_field_types hop")
if "value_member_assign_immutable_root" not in call:
    raise SystemExit("missing value_member_assign_immutable_root")
expr = Path("crates/syx-semantic/src/semantic_analyze_expr.rs").read_text(encoding="utf-8")
if "U-29" not in expr:
    raise SystemExit("MemberAssign missing U-29 note")
nail = Path("crates/syx-driver/tests/lang/let_nested_enum_tuple_s060.rs")
if not nail.is_file():
    raise SystemExit("missing lang/let_nested_enum_tuple_s060.rs")
nt = nail.read_text(encoding="utf-8")
for needle in (
    "u29_let_enum_to_enum_is_s060",
    "u29_let_enum_to_tuple_is_s060",
    "u29_let_tuple_to_enum_is_s060",
    "u29_let_ident_enum_still_s060",
    "u29_let_dataclass_to_enum_still_s060",
    "u29_var_enum_to_enum_ok",
    "u29_var_enum_to_enum_runs",
):
    if needle not in nt:
        raise SystemExit(f"let_nested_enum_tuple_s060 missing {needle}")
main = Path("crates/syx-driver/tests/lang/main.rs").read_text(encoding="utf-8")
if "mod let_nested_enum_tuple_s060;" not in main:
    raise SystemExit("lang/main.rs missing mod let_nested_enum_tuple_s060")
for rel in (
    "books/language-guide/09-enumerations.md",
    "books/zh-cn/language-guide/09-enumerations.md",
    "books/language-guide/appendix-b-diagnostic-index.md",
    "books/zh-cn/language-guide/appendix-b-diagnostic-index.md",
):
    t = Path(rel).read_text(encoding="utf-8")
    if "S060" not in t:
        raise SystemExit(f"{rel} missing S060")
    if "nested" not in t.lower() and "嵌套" not in t:
        raise SystemExit(f"{rel} missing nested/嵌套 S060 note")
print("u29_semantic_surface=1")
print("u29_lang_nail=1")
print("u29_books_thin=1")
PY

python3 - <<'PY' || die "census doc missing"
from pathlib import Path
census = Path("docs/census/u-29-let-nested-enum-tuple-s060-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/u-29-let-nested-enum-tuple-s060-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("U-29", "S060", "enum", "tuple", "≠", "R-50", "JIT"):
    if needle not in ct:
        raise SystemExit(f"u-29 census missing {needle}")
print("u29_census_doc=1")
PY

SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

EE=$(mktemp /tmp/u29_ee_XXXX.sy)
cat >"$EE" <<'EOF'
enum Inner { case wrap(payload: Int) }
enum Outer { case box(inner: Inner) }
public func main() {
    let o = Outer.box(inner: Inner.wrap(payload: 1))
    o.inner.payload = 9
}
EOF
if "$SYX" check "$EE" >/tmp/u29_ee_out.txt 2>&1; then
  die "let enum→enum must fail check"
fi
if ! grep -q 'S060' /tmp/u29_ee_out.txt; then
  cat /tmp/u29_ee_out.txt
  die "let enum→enum must report S060"
fi
ok "probe_let_enum_to_enum_s060" "1"
rm -f "$EE"

ET=$(mktemp /tmp/u29_et_XXXX.sy)
cat >"$ET" <<'EOF'
enum E { case wrap(t: (Int, Int)) }
public func main() {
    let e = E.wrap(t: (1, 2))
    e.t.0 = 9
}
EOF
if "$SYX" check "$ET" >/tmp/u29_et_out.txt 2>&1; then
  die "let enum→tuple must fail check"
fi
if ! grep -q 'S060' /tmp/u29_et_out.txt; then
  cat /tmp/u29_et_out.txt
  die "let enum→tuple must report S060"
fi
ok "probe_let_enum_to_tuple_s060" "1"
rm -f "$ET"

TE=$(mktemp /tmp/u29_te_XXXX.sy)
cat >"$TE" <<'EOF'
enum E { case wrap(payload: Int) }
public func main() {
    let t = (E.wrap(payload: 1), 0)
    t.0.payload = 9
}
EOF
if "$SYX" check "$TE" >/tmp/u29_te_out.txt 2>&1; then
  die "let tuple→enum must fail check"
fi
if ! grep -q 'S060' /tmp/u29_te_out.txt; then
  cat /tmp/u29_te_out.txt
  die "let tuple→enum must report S060"
fi
ok "probe_let_tuple_to_enum_s060" "1"
rm -f "$TE"

IDENT=$(mktemp /tmp/u29_ident_XXXX.sy)
cat >"$IDENT" <<'EOF'
enum Box { case wrap(payload: Int) }
public func main() {
    let e = Box.wrap(payload: 1)
    e.payload = 9
}
EOF
if "$SYX" check "$IDENT" >/tmp/u29_ident_out.txt 2>&1; then
  die "let ident enum must still fail check"
fi
if ! grep -q 'S060' /tmp/u29_ident_out.txt; then
  cat /tmp/u29_ident_out.txt
  die "let ident enum must still report S060"
fi
ok "probe_let_ident_enum_s060" "1"
rm -f "$IDENT"

VAR=$(mktemp /tmp/u29_var_XXXX.sy)
cat >"$VAR" <<'EOF'
enum Inner { case wrap(payload: Int) }
enum Outer { case box(inner: Inner) }
public func main() {
    var o = Outer.box(inner: Inner.wrap(payload: 1))
    o.inner.payload = 9
    print(o.inner.payload)
}
EOF
out=$("$SYX" run "$VAR" 2>/tmp/u29_var_err.txt) || {
  cat /tmp/u29_var_err.txt
  die "var enum→enum run failed"
}
if [[ "$out" != "9" ]]; then
  echo "got: $out" >&2
  die "var enum→enum output mismatch"
fi
ok "probe_var_enum_to_enum" "9"
rm -f "$VAR"

cargo test -p syx-driver --test lang let_nested_enum_tuple_s060 -- --test-threads=4 \
  >/tmp/u29_lang_nail.txt 2>&1 || {
  tail -40 /tmp/u29_lang_nail.txt
  die "lang let_nested_enum_tuple_s060 nail failed"
}
ok "lang_nail" "let_nested_enum_tuple_s060"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/u29_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/u29_book_out.txt
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

python3 - <<'PY' || die "roadmap U-29 exit / next park missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if "U-29" not in text:
    raise SystemExit("roadmap missing U-29")
rows = [l for l in text.splitlines() if "| **U-29** |" in l]
if not rows or not any("[x]" in r for r in rows):
    raise SystemExit(f"U-29 not [x]: {rows}")
if "默认下一" in text:
    chunk = text.split("默认下一", 1)[1][:500]
    if "R-50" in chunk and "不" not in chunk and "park" not in chunk.lower() and "未编号" not in chunk:
        raise SystemExit("roadmap must not default next to R-50")
print("roadmap_u29_exit=1")
print("roadmap_no_r50_default=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "u29_let_nested_enum_tuple_s060_census_ok=0"
  exit 1
fi
echo "u29_let_nested_enum_tuple_s060_census_ok=1"
echo "current=U-29-let-nested-enum-tuple-s060"
echo "next=park"
