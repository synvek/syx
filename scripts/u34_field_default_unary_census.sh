#!/usr/bin/env bash
# U-34 field-default unary Neg Int census (`var n: Int = -1`).
# Soft park: Float · Call defaults · deep -(1) · CST · Break · JIT · SyxHost · D3 · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== U-34 field-default-unary census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "MIR U-34 surface missing"
from pathlib import Path
idx = Path("crates/syx-mir/src/mir_module_index.rs").read_text(encoding="utf-8")
if "field_default_from_expr" not in idx:
    raise SystemExit("missing field_default_from_expr")
if "UnaryOp::Neg" not in idx:
    raise SystemExit("missing UnaryOp::Neg arm")
if "wrapping_neg" not in idx:
    raise SystemExit("missing wrapping_neg")
# collectors must call from_expr
if idx.count("field_default_from_expr(") < 3:
    raise SystemExit("expected from_expr helper + two collector call sites")
nail = Path("crates/syx-driver/tests/lang/field_default_literals.rs")
if not nail.is_file():
    raise SystemExit("missing lang/field_default_literals.rs")
nt = nail.read_text(encoding="utf-8")
for needle in (
    "u34_struct_unary_neg_int_default_runs",
    "u34_class_unary_neg_int_default_runs",
    "u33_struct_bool_string_int_defaults_run",
    "u33_counter_int_default_still_runs",
):
    if needle not in nt:
        raise SystemExit(f"field_default_literals missing {needle}")
main = Path("crates/syx-driver/tests/lang/main.rs").read_text(encoding="utf-8")
if "mod field_default_literals;" not in main:
    raise SystemExit("lang/main.rs missing mod field_default_literals")
print("u34_mir_surface=1")
print("u34_lang_nail=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/u-34-field-default-unary-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/u-34-field-default-unary-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("U-34", "unary", "≠", "R-50", "JIT"):
    if needle not in ct:
        raise SystemExit(f"u-34 census missing {needle}")
print("u34_census_doc=1")
PY

SYX=./target/debug/syx
# Always rebuild so MIR U-34 collector is not shadowed by a stale CLI binary.
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

STRUCT=$(mktemp /tmp/u34_struct_XXXX.sy)
cat >"$STRUCT" <<'EOF'
struct S {
    var n: Int = -1
}
public func main() {
    let x = S()
    print(x.n)
}
EOF
out=$("$SYX" run "$STRUCT" 2>/tmp/u34_struct_err.txt) || {
  cat /tmp/u34_struct_err.txt
  die "struct unary -1 probe run failed"
}
if [[ "$out" != "-1" ]]; then
  echo "got: $out" >&2
  die "struct unary -1 probe output mismatch"
fi
ok "probe_struct_unary_neg" "-1"
rm -f "$STRUCT"

CLASS=$(mktemp /tmp/u34_class_XXXX.sy)
cat >"$CLASS" <<'EOF'
class C {
    var n: Int = -1
    init() {}
}
public func main() {
    let c = C()
    print(c.n)
}
EOF
out=$("$SYX" run "$CLASS" 2>/tmp/u34_class_err.txt) || {
  cat /tmp/u34_class_err.txt
  die "class unary -1 probe run failed"
}
if [[ "$out" != "-1" ]]; then
  echo "got: $out" >&2
  die "class unary -1 probe output mismatch"
fi
ok "probe_class_unary_neg" "-1"
rm -f "$CLASS"

POS=$(mktemp /tmp/u34_pos_XXXX.sy)
cat >"$POS" <<'EOF'
struct S {
    var b: Bool = true
    var s: String = "hi"
    var n: Int = 7
}
public func main() {
    let x = S()
    print(x.b)
    print(x.s)
    print(x.n)
}
EOF
out=$("$SYX" run "$POS" 2>/tmp/u34_pos_err.txt) || {
  cat /tmp/u34_pos_err.txt
  die "U-33 positive literals regression failed"
}
expected=$'true\nhi\n7'
if [[ "$out" != "$expected" ]]; then
  echo "got: $out" >&2
  die "U-33 positive literals output mismatch"
fi
ok "probe_u33_positive" "true_hi_7"
rm -f "$POS"

cargo test -p syx-driver --test lang field_default_literals -- --test-threads=4 \
  >/tmp/u34_lang_nail.txt 2>&1 || {
  tail -40 /tmp/u34_lang_nail.txt
  die "lang field_default_literals nail failed"
}
ok "lang_nail" "field_default_literals"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/u34_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/u34_book_out.txt
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

python3 - <<'PY' || die "roadmap U-34 exit / next park missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if "U-34" not in text:
    raise SystemExit("roadmap missing U-34")
rows = [l for l in text.splitlines() if "| **U-34** |" in l]
if not rows or not any("[x]" in r for r in rows):
    raise SystemExit(f"U-34 not [x]: {rows}")
if "默认下一" in text:
    chunk = text.split("默认下一", 1)[1][:500]
    if "R-50" in chunk and "不" not in chunk and "park" not in chunk.lower() and "未编号" not in chunk:
        raise SystemExit("roadmap must not default next to R-50")
print("roadmap_u34_exit=1")
print("roadmap_no_r50_default=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "u34_field_default_unary_census_ok=0"
  exit 1
fi
echo "u34_field_default_unary_census_ok=1"
echo "current=U-34-field-default-unary"
echo "next=park"
