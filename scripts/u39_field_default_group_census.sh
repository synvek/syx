#!/usr/bin/env bash
# U-39 field-default one-layer Group peel census.
# Soft park: ((7)) · --1 · !true · extension float · CallSuper · CST · Break · JIT · SyxHost · D3 · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12 · Dog anon/lab
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== U-39 field-default-group census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "MIR U-39 surface missing"
from pathlib import Path
idx = Path("crates/syx-mir/src/mir_module_index.rs").read_text(encoding="utf-8")
if "Expr::Group" not in idx:
    raise SystemExit("missing Expr::Group peel")
if "negate_int_or_num" not in idx:
    raise SystemExit("missing negate_int_or_num")
if "neg_operand_is_nested_neg" not in idx:
    raise SystemExit("missing neg_operand_is_nested_neg")
nail = Path("crates/syx-driver/tests/lang/field_default_literals.rs")
if not nail.is_file():
    raise SystemExit("missing lang/field_default_literals.rs")
nt = nail.read_text(encoding="utf-8")
for needle in (
    "u39_struct_group_peel_defaults_run",
    "u39_class_group_peel_empty_init_runs",
    "u38_inherited_field_default_on_subclass_init",
    "u37_struct_int_to_double_default_runs",
    "u33_struct_bool_string_int_defaults_run",
):
    if needle not in nt:
        raise SystemExit(f"field_default_literals missing {needle}")
main = Path("crates/syx-driver/tests/lang/main.rs").read_text(encoding="utf-8")
if "mod field_default_literals;" not in main:
    raise SystemExit("lang/main.rs missing mod field_default_literals")
print("u39_mir_surface=1")
print("u39_lang_nail=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/u-39-field-default-group-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/u-39-field-default-group-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("U-39", "Group", "≠", "R-50", "JIT"):
    if needle not in ct:
        raise SystemExit(f"u-39 census missing {needle}")
print("u39_census_doc=1")
PY

SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

GROUP=$(mktemp /tmp/u39_group_XXXX.sy)
cat >"$GROUP" <<'EOF'
struct S {
    var n: Int = (7)
    var x: Double = (1.5)
    var a: Int = -(1)
    var b: Double = -(1.5)
    var c: Int = (-1)
    var z: Int = ((7))
}
public func main() {
    let s = S()
    print(s.n)
    print(s.x)
    print(s.a)
    print(s.b)
    print(s.c)
    print(s.z)
}
EOF
out=$("$SYX" run "$GROUP" 2>/tmp/u39_group_err.txt) || {
  cat /tmp/u39_group_err.txt
  die "struct Group peel probe run failed"
}
expected=$'7\n1.5\n-1\n-1.5\n-1\nnil'
if [[ "$out" != "$expected" ]]; then
  echo "got: $out" >&2
  die "struct Group peel probe output mismatch"
fi
ok "probe_struct_group" "7_1.5_-1_-1.5_-1_nil"
rm -f "$GROUP"

CLASS=$(mktemp /tmp/u39_class_XXXX.sy)
cat >"$CLASS" <<'EOF'
class C {
    var n: Int = (7)
    var x: Double = -(1.5)
    init() {}
}
public func main() {
    let c = C()
    print(c.n)
    print(c.x)
}
EOF
out=$("$SYX" run "$CLASS" 2>/tmp/u39_class_err.txt) || {
  cat /tmp/u39_class_err.txt
  die "class Group peel probe run failed"
}
expected=$'7\n-1.5'
if [[ "$out" != "$expected" ]]; then
  echo "got: $out" >&2
  die "class Group peel probe output mismatch"
fi
ok "probe_class_group" "7_-1.5"
rm -f "$CLASS"

DOG=$(mktemp /tmp/u39_dog_XXXX.sy)
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
out=$("$SYX" run "$DOG" 2>/tmp/u39_dog_err.txt) || {
  cat /tmp/u39_dog_err.txt
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
  >/tmp/u39_lang_nail.txt 2>&1 || {
  tail -40 /tmp/u39_lang_nail.txt
  die "lang field_default_literals nail failed"
}
ok "lang_nail" "field_default_literals"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/u39_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/u39_book_out.txt
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

python3 - <<'PY' || die "roadmap U-39 exit / next park missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if "U-39" not in text:
    raise SystemExit("roadmap missing U-39")
rows = [l for l in text.splitlines() if "| **U-39** |" in l]
if not rows or not any("[x]" in r for r in rows):
    raise SystemExit(f"U-39 not [x]: {rows}")
if "默认下一" in text:
    chunk = text.split("默认下一", 1)[1][:500]
    if "R-50" in chunk and "不" not in chunk and "park" not in chunk.lower() and "未编号" not in chunk:
        raise SystemExit("roadmap must not default next to R-50")
print("roadmap_u39_exit=1")
print("roadmap_no_r50_default=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "u39_field_default_group_census_ok=0"
  exit 1
fi
echo "u39_field_default_group_census_ok=1"
echo "current=U-39-field-default-group"
echo "next=park"
