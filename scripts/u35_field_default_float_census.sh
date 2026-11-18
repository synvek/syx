#!/usr/bin/env bash
# U-35 field-default Double literals census (ConstNumeric dest:Some).
# Soft park: unary -1.5 · extension float · super.init · CST · Break · JIT · SyxHost · D3 · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== U-35 field-default-float census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "MIR U-35 surface missing"
from pathlib import Path
idx = Path("crates/syx-mir/src/mir_module_index.rs").read_text(encoding="utf-8")
if "Num { kind: u8, bits: u64 }" not in idx and "Num {" not in idx:
    raise SystemExit("missing FieldDefaultLit::Num")
if "Literal::Double" not in idx:
    raise SystemExit("missing Double collect arm")
if "float_literal_bits" not in idx:
    raise SystemExit("missing float_literal_bits")
ident = Path("crates/syx-mir/src/mir_lower_expr_call_ident.rs").read_text(encoding="utf-8")
clinit = Path("crates/syx-mir/src/mir_lower_class_init.rs").read_text(encoding="utf-8")
for label, text in (("call-ident", ident), ("class-init", clinit)):
    if "FieldDefaultLit::Num" not in text:
        raise SystemExit(f"{label} missing Num emit")
    if "ConstNumeric" not in text:
        raise SystemExit(f"{label} missing ConstNumeric")
    if "dest: Some" not in text:
        raise SystemExit(f"{label} missing dest: Some for Num")
nail = Path("crates/syx-driver/tests/lang/field_default_literals.rs")
if not nail.is_file():
    raise SystemExit("missing lang/field_default_literals.rs")
nt = nail.read_text(encoding="utf-8")
for needle in (
    "u35_struct_double_default_runs",
    "u35_class_double_default_runs",
    "u34_struct_unary_neg_int_default_runs",
    "u33_struct_bool_string_int_defaults_run",
):
    if needle not in nt:
        raise SystemExit(f"field_default_literals missing {needle}")
main = Path("crates/syx-driver/tests/lang/main.rs").read_text(encoding="utf-8")
if "mod field_default_literals;" not in main:
    raise SystemExit("lang/main.rs missing mod field_default_literals")
print("u35_mir_surface=1")
print("u35_lang_nail=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/u-35-field-default-float-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/u-35-field-default-float-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("U-35", "Double", "≠", "R-50", "JIT"):
    if needle not in ct:
        raise SystemExit(f"u-35 census missing {needle}")
print("u35_census_doc=1")
PY

SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

STRUCT=$(mktemp /tmp/u35_struct_XXXX.sy)
cat >"$STRUCT" <<'EOF'
struct S {
    var x: Double = 1.5
}
public func main() {
    let s = S()
    print(s.x)
}
EOF
out=$("$SYX" run "$STRUCT" 2>/tmp/u35_struct_err.txt) || {
  cat /tmp/u35_struct_err.txt
  die "struct Double probe run failed"
}
if [[ "$out" != "1.5" ]]; then
  echo "got: $out" >&2
  die "struct Double probe output mismatch"
fi
ok "probe_struct_double" "1.5"
rm -f "$STRUCT"

CLASS=$(mktemp /tmp/u35_class_XXXX.sy)
cat >"$CLASS" <<'EOF'
class C {
    var x: Double = 1.5
    init() {}
}
public func main() {
    let c = C()
    print(c.x)
}
EOF
out=$("$SYX" run "$CLASS" 2>/tmp/u35_class_err.txt) || {
  cat /tmp/u35_class_err.txt
  die "class Double probe run failed"
}
if [[ "$out" != "1.5" ]]; then
  echo "got: $out" >&2
  die "class Double probe output mismatch"
fi
ok "probe_class_double" "1.5"
rm -f "$CLASS"

REG=$(mktemp /tmp/u35_reg_XXXX.sy)
cat >"$REG" <<'EOF'
struct S {
    var n: Int = -1
    var b: Bool = true
    var s: String = "hi"
}
public func main() {
    let x = S()
    print(x.n)
    print(x.b)
    print(x.s)
}
EOF
out=$("$SYX" run "$REG" 2>/tmp/u35_reg_err.txt) || {
  cat /tmp/u35_reg_err.txt
  die "U-33/U-34 regression probe failed"
}
expected=$'-1\ntrue\nhi'
if [[ "$out" != "$expected" ]]; then
  echo "got: $out" >&2
  die "U-33/U-34 regression output mismatch"
fi
ok "probe_u33_u34_regression" "-1_true_hi"
rm -f "$REG"

cargo test -p syx-driver --test lang field_default_literals -- --test-threads=4 \
  >/tmp/u35_lang_nail.txt 2>&1 || {
  tail -40 /tmp/u35_lang_nail.txt
  die "lang field_default_literals nail failed"
}
ok "lang_nail" "field_default_literals"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/u35_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/u35_book_out.txt
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

python3 - <<'PY' || die "roadmap U-35 exit / next park missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if "U-35" not in text:
    raise SystemExit("roadmap missing U-35")
rows = [l for l in text.splitlines() if "| **U-35** |" in l]
if not rows or not any("[x]" in r for r in rows):
    raise SystemExit(f"U-35 not [x]: {rows}")
if "默认下一" in text:
    chunk = text.split("默认下一", 1)[1][:500]
    if "R-50" in chunk and "不" not in chunk and "park" not in chunk.lower() and "未编号" not in chunk:
        raise SystemExit("roadmap must not default next to R-50")
print("roadmap_u35_exit=1")
print("roadmap_no_r50_default=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "u35_field_default_float_census_ok=0"
  exit 1
fi
echo "u35_field_default_float_census_ok=1"
echo "current=U-35-field-default-float"
echo "next=park"
