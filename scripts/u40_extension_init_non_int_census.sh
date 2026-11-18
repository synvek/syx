#!/usr/bin/env bash
# U-40 extension self.init non-Int FieldDefaultLit census.
# Soft park: CallSuper · Ch15 MD · ((7)) · !true · Call defaults · CST · Break · JIT · SyxHost · D3 · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12 · Dog anon/lab · Integer Point 0/0
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== U-40 extension-init-non-int census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "MIR U-40 surface missing"
from pathlib import Path
idx = Path("crates/syx-mir/src/mir_module_index.rs").read_text(encoding="utf-8")
if "FieldDefaultLit>>" not in idx and "FieldDefaultLit)>" not in idx:
    # signature uses FieldDefaultLit in return
    if "Option<Vec<(SmolStr, FieldDefaultLit)>>" not in idx:
        raise SystemExit("extension_init_literal_fields not widened to FieldDefaultLit")
if "field_default_from_expr(&arg.value)" not in idx and "field_default_from_expr" not in idx:
    raise SystemExit("collector missing field_default_from_expr")
shell = Path("crates/syx-mir/src/mir_lower_type_members.rs").read_text(encoding="utf-8")
if "FieldDefaultLit::Num" not in shell:
    raise SystemExit("shell missing Num arm")
if "dest: Some" not in shell:
    raise SystemExit("shell Num missing dest: Some")
unify = Path("crates/syx-driver/src/mir_unify.rs").read_text(encoding="utf-8")
if "ConstBool" not in unify or "ConstStr" not in unify:
    raise SystemExit("mir_unify missing ConstBool/ConstStr for 0-arg replace")
nail = Path("crates/syx-driver/tests/lang/field_default_literals.rs")
nt = nail.read_text(encoding="utf-8")
for needle in (
    "u40_extension_double_self_init_runs",
    "u40_extension_bool_string_self_init_runs",
    "u39_struct_group_peel_defaults_run",
    "u38_inherited_field_default_on_subclass_init",
    "u33_struct_bool_string_int_defaults_run",
):
    if needle not in nt:
        raise SystemExit(f"field_default_literals missing {needle}")
main = Path("crates/syx-driver/tests/lang/main.rs").read_text(encoding="utf-8")
if "mod field_default_literals;" not in main:
    raise SystemExit("lang/main.rs missing mod field_default_literals")
print("u40_mir_surface=1")
print("u40_lang_nail=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/u-40-extension-init-non-int-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/u-40-extension-init-non-int-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("U-40", "extension", "≠", "R-50", "JIT"):
    if needle not in ct:
        raise SystemExit(f"u-40 census missing {needle}")
print("u40_census_doc=1")
PY

SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

FLOAT=$(mktemp /tmp/u40_float_XXXX.sy)
cat >"$FLOAT" <<'EOF'
struct Point {
    var x: Double
    var y: Double
}
extension Point {
    init() { self.init(x: 1.5, y: 0.0) }
}
public func main() {
    let p = Point()
    print(p.x)
    print(p.y)
}
EOF
out=$("$SYX" run "$FLOAT" 2>/tmp/u40_float_err.txt) || {
  cat /tmp/u40_float_err.txt
  die "extension Double probe run failed"
}
expected=$'1.5\n0'
if [[ "$out" != "$expected" ]]; then
  echo "got: $out" >&2
  die "extension Double probe output mismatch"
fi
ok "probe_extension_double" "1.5_0"
rm -f "$FLOAT"

BOOL=$(mktemp /tmp/u40_bool_XXXX.sy)
cat >"$BOOL" <<'EOF'
struct Flag {
    var on: Bool
    var name: String
}
extension Flag {
    init() { self.init(on: true, name: "ok") }
}
public func main() {
    let f = Flag()
    print(f.on)
    print(f.name)
}
EOF
out=$("$SYX" run "$BOOL" 2>/tmp/u40_bool_err.txt) || {
  cat /tmp/u40_bool_err.txt
  die "extension Bool/String probe run failed"
}
expected=$'true\nok'
if [[ "$out" != "$expected" ]]; then
  echo "got: $out" >&2
  die "extension Bool/String probe output mismatch"
fi
ok "probe_extension_bool_string" "true_ok"
rm -f "$BOOL"

INT=$(mktemp /tmp/u40_int_XXXX.sy)
cat >"$INT" <<'EOF'
struct Point {
    var x: Int
    var y: Int
}
extension Point {
    init() { self.init(x: 0, y: 0) }
}
public func main() {
    let p = Point()
    print(p.x)
    print(p.y)
}
EOF
out=$("$SYX" run "$INT" 2>/tmp/u40_int_err.txt) || {
  cat /tmp/u40_int_err.txt
  die "Integer S1-I7 probe run failed"
}
expected=$'0\n0'
if [[ "$out" != "$expected" ]]; then
  echo "got: $out" >&2
  die "Integer S1-I7 probe output mismatch"
fi
ok "probe_integer_s1_i7" "0_0"
rm -f "$INT"

DOG=$(mktemp /tmp/u40_dog_XXXX.sy)
cat >"$DOG" <<'EOF'
class Animal {
    var name: String = "anon"
}
class Dog: Animal {
    var breed: String
    init(breed: String) {
        self.breed = breed
    }
}
public func main() {
    let d = Dog(breed: "lab")
    print(d.name)
    print(d.breed)
}
EOF
out=$("$SYX" run "$DOG" 2>/tmp/u40_dog_err.txt) || {
  cat /tmp/u40_dog_err.txt
  die "Dog inherited default regression failed"
}
expected=$'anon\nlab'
if [[ "$out" != "$expected" ]]; then
  echo "got: $out" >&2
  die "Dog inherited default regression mismatch"
fi
ok "probe_dog_inherited" "anon_lab"
rm -f "$DOG"

cargo test -p syx-driver --test lang field_default_literals -- --test-threads=4 \
  >/tmp/u40_lang_nail.txt 2>&1 || {
  tail -40 /tmp/u40_lang_nail.txt
  die "lang field_default_literals nail failed"
}
ok "lang_nail" "field_default_literals"

cargo test -p syx-mir s1_i7_extension_point_convenience_init_lowers -- --exact \
  >/tmp/u40_s1i7.txt 2>&1 || {
  tail -40 /tmp/u40_s1i7.txt
  die "s1_i7 unit nail failed"
}
ok "s1_i7_unit" "ok"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/u40_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/u40_book_out.txt
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

python3 - <<'PY' || die "roadmap U-40 exit / next park missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if "U-40" not in text:
    raise SystemExit("roadmap missing U-40")
rows = [l for l in text.splitlines() if "| **U-40** |" in l]
if not rows or not any("[x]" in r for r in rows):
    raise SystemExit(f"U-40 not [x]: {rows}")
if "默认下一" in text:
    chunk = text.split("默认下一", 1)[1][:500]
    if "R-50" in chunk and "不" not in chunk and "park" not in chunk.lower() and "未编号" not in chunk:
        raise SystemExit("roadmap must not default next to R-50")
print("roadmap_u40_exit=1")
print("roadmap_no_r50_default=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "u40_extension_init_non_int_census_ok=0"
  exit 1
fi
echo "u40_extension_init_non_int_census_ok=1"
echo "current=U-40-extension-init-non-int"
echo "next=park"
