#!/usr/bin/env bash
# U-37 field-default Integer→Float/Double promote-at-collect census.
# Soft park: inherited pad walk · CallSuper · extension float · Group peel · CST · Break · JIT · SyxHost · D3 · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== U-37 field-default-int-float census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "MIR U-37 surface missing"
from pathlib import Path
idx = Path("crates/syx-mir/src/mir_module_index.rs").read_text(encoding="utf-8")
if "promote_int_default_for_float_field" not in idx:
    raise SystemExit("missing promote_int_default_for_float_field")
if "is_float" not in idx:
    raise SystemExit("missing is_float in collect promote")
if "from_type_name" not in idx:
    raise SystemExit("missing from_type_name")
nail = Path("crates/syx-driver/tests/lang/field_default_literals.rs")
if not nail.is_file():
    raise SystemExit("missing lang/field_default_literals.rs")
nt = nail.read_text(encoding="utf-8")
for needle in (
    "u37_struct_int_to_double_default_runs",
    "u37_class_int_to_double_default_runs",
    "u37_struct_neg_int_to_double_default_runs",
    "u36_struct_unary_neg_double_default_runs",
    "u35_struct_double_default_runs",
    "u33_struct_bool_string_int_defaults_run",
):
    if needle not in nt:
        raise SystemExit(f"field_default_literals missing {needle}")
main = Path("crates/syx-driver/tests/lang/main.rs").read_text(encoding="utf-8")
if "mod field_default_literals;" not in main:
    raise SystemExit("lang/main.rs missing mod field_default_literals")
print("u37_mir_surface=1")
print("u37_lang_nail=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/u-37-field-default-int-float-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/u-37-field-default-int-float-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("U-37", "Integer", "≠", "R-50", "JIT"):
    if needle not in ct:
        raise SystemExit(f"u-37 census missing {needle}")
print("u37_census_doc=1")
PY

SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

# / 2 distinguishes I32 (0) from F64 (0.5)
STRUCT=$(mktemp /tmp/u37_struct_XXXX.sy)
cat >"$STRUCT" <<'EOF'
struct S {
    var x: Double = 1
}
public func main() {
    let s = S()
    print(s.x / 2)
}
EOF
out=$("$SYX" run "$STRUCT" 2>/tmp/u37_struct_err.txt) || {
  cat /tmp/u37_struct_err.txt
  die "struct Int→Double probe run failed"
}
if [[ "$out" != "0.5" ]]; then
  echo "got: $out" >&2
  die "struct Int→Double probe output mismatch (want 0.5; I32 path yields 0)"
fi
ok "probe_struct_int_double" "0.5"
rm -f "$STRUCT"

CLASS=$(mktemp /tmp/u37_class_XXXX.sy)
cat >"$CLASS" <<'EOF'
class C {
    var x: Double = 1
    init() {}
}
public func main() {
    let c = C()
    print(c.x / 2)
}
EOF
out=$("$SYX" run "$CLASS" 2>/tmp/u37_class_err.txt) || {
  cat /tmp/u37_class_err.txt
  die "class Int→Double probe run failed"
}
if [[ "$out" != "0.5" ]]; then
  echo "got: $out" >&2
  die "class Int→Double probe output mismatch"
fi
ok "probe_class_int_double" "0.5"
rm -f "$CLASS"

NEG=$(mktemp /tmp/u37_neg_XXXX.sy)
cat >"$NEG" <<'EOF'
struct S {
    var x: Double = -1
}
public func main() {
    let s = S()
    print(s.x / 2)
}
EOF
out=$("$SYX" run "$NEG" 2>/tmp/u37_neg_err.txt) || {
  cat /tmp/u37_neg_err.txt
  die "struct Neg Int→Double probe run failed"
}
if [[ "$out" != "-0.5" ]]; then
  echo "got: $out" >&2
  die "struct Neg Int→Double probe output mismatch"
fi
ok "probe_struct_neg_int_double" "-0.5"
rm -f "$NEG"

REG=$(mktemp /tmp/u37_reg_XXXX.sy)
cat >"$REG" <<'EOF'
struct S {
    var a: Double = 1.5
    var b: Double = -1.5
    var n: Int = -1
}
public func main() {
    let s = S()
    print(s.a)
    print(s.b)
    print(s.n)
}
EOF
out=$("$SYX" run "$REG" 2>/tmp/u37_reg_err.txt) || {
  cat /tmp/u37_reg_err.txt
  die "U-33…U-36 regression probe failed"
}
expected=$'1.5\n-1.5\n-1'
if [[ "$out" != "$expected" ]]; then
  echo "got: $out" >&2
  die "U-33…U-36 regression output mismatch"
fi
ok "probe_u33_u36_regression" "1.5_-1.5_-1"
rm -f "$REG"

cargo test -p syx-driver --test lang field_default_literals -- --test-threads=4 \
  >/tmp/u37_lang_nail.txt 2>&1 || {
  tail -40 /tmp/u37_lang_nail.txt
  die "lang field_default_literals nail failed"
}
ok "lang_nail" "field_default_literals"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/u37_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/u37_book_out.txt
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

python3 - <<'PY' || die "roadmap U-37 exit / next park missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if "U-37" not in text:
    raise SystemExit("roadmap missing U-37")
rows = [l for l in text.splitlines() if "| **U-37** |" in l]
if not rows or not any("[x]" in r for r in rows):
    raise SystemExit(f"U-37 not [x]: {rows}")
if "默认下一" in text:
    chunk = text.split("默认下一", 1)[1][:500]
    if "R-50" in chunk and "不" not in chunk and "park" not in chunk.lower() and "未编号" not in chunk:
        raise SystemExit("roadmap must not default next to R-50")
print("roadmap_u37_exit=1")
print("roadmap_no_r50_default=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "u37_field_default_int_float_census_ok=0"
  exit 1
fi
echo "u37_field_default_int_float_census_ok=1"
echo "current=U-37-field-default-int-float"
echo "next=park"
