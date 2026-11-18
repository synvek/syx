#!/usr/bin/env bash
# U-33 field-default Bool/String/Int literals census (MIR pad; ≠ Float / Call / codegen map).
# Soft park: Float · nested ctor · closure throws syntax · CST · Break · JIT · SyxHost · D3 · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== U-33 field-default-literals census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "MIR U-33 surface missing"
from pathlib import Path
idx = Path("crates/syx-mir/src/mir_module_index.rs").read_text(encoding="utf-8")
if "enum FieldDefaultLit" not in idx:
    raise SystemExit("missing FieldDefaultLit")
if "dataclass_field_defaults" not in idx:
    raise SystemExit("missing dataclass_field_defaults rename")
if "dataclass_field_default_i64" in idx:
    raise SystemExit("stale dataclass_field_default_i64 still present")
if "field_default_from_literal" not in idx:
    raise SystemExit("missing field_default_from_literal")
ident = Path("crates/syx-mir/src/mir_lower_expr_call_ident.rs").read_text(encoding="utf-8")
if "FieldDefaultLit::Bool" not in ident or "FieldDefaultLit::Str" not in ident:
    raise SystemExit("call-ident pad missing Bool/Str emit")
clinit = Path("crates/syx-mir/src/mir_lower_class_init.rs").read_text(encoding="utf-8")
if "FieldDefaultLit::Bool" not in clinit or "FieldDefaultLit::Str" not in clinit:
    raise SystemExit("class-init pad missing Bool/Str emit")
# codegen map must stay untouched (off syx-run path)
cg = Path("crates/syx-codegen/src/lib.rs").read_text(encoding="utf-8")
if "dataclass_field_defaults: indexmap::IndexMap<(SmolStr, SmolStr), i32>" not in cg and \
   "dataclass_field_defaults: IndexMap<(SmolStr, SmolStr), i32>" not in cg:
    # tolerate either form; require i32 map still exists as non-goal
    if "dataclass_field_defaults" not in cg:
        raise SystemExit("unexpected: codegen map vanished (should remain parked)")
nail = Path("crates/syx-driver/tests/lang/field_default_literals.rs")
if not nail.is_file():
    raise SystemExit("missing lang/field_default_literals.rs")
nt = nail.read_text(encoding="utf-8")
for needle in (
    "u33_struct_bool_string_int_defaults_run",
    "u33_class_empty_init_applies_defaults",
    "u33_explicit_label_overrides_default",
    "u33_missing_default_still_nil",
    "u33_counter_int_default_still_runs",
):
    if needle not in nt:
        raise SystemExit(f"field_default_literals missing {needle}")
main = Path("crates/syx-driver/tests/lang/main.rs").read_text(encoding="utf-8")
if "mod field_default_literals;" not in main:
    raise SystemExit("lang/main.rs missing mod field_default_literals")
print("u33_mir_surface=1")
print("u33_lang_nail=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/u-33-field-default-literals-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/u-33-field-default-literals-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("U-33", "FieldDefaultLit", "≠", "R-50", "JIT"):
    if needle not in ct:
        raise SystemExit(f"u-33 census missing {needle}")
print("u33_census_doc=1")
PY

SYX=./target/debug/syx
# Always rebuild so MIR U-33 pads are not shadowed by a stale CLI binary.
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

STRUCT=$(mktemp /tmp/u33_struct_XXXX.sy)
cat >"$STRUCT" <<'EOF'
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
out=$("$SYX" run "$STRUCT" 2>/tmp/u33_struct_err.txt) || {
  cat /tmp/u33_struct_err.txt
  die "struct defaults probe run failed"
}
expected=$'true\nhi\n7'
if [[ "$out" != "$expected" ]]; then
  echo "got: $out" >&2
  die "struct defaults probe output mismatch"
fi
ok "probe_struct_defaults" "true_hi_7"
rm -f "$STRUCT"

CLASS=$(mktemp /tmp/u33_class_XXXX.sy)
cat >"$CLASS" <<'EOF'
class Animal {
    var name: String = "anon"
    var ok: Bool = false
    var age: Int = 7
    init() {}
}
public func main() {
    let a = Animal()
    print(a.name)
    print(a.ok)
    print(a.age)
}
EOF
out=$("$SYX" run "$CLASS" 2>/tmp/u33_class_err.txt) || {
  cat /tmp/u33_class_err.txt
  die "class defaults probe run failed"
}
expected=$'anon\nfalse\n7'
if [[ "$out" != "$expected" ]]; then
  echo "got: $out" >&2
  die "class defaults probe output mismatch"
fi
ok "probe_class_defaults" "anon_false_7"
rm -f "$CLASS"

OVERRIDE=$(mktemp /tmp/u33_ov_XXXX.sy)
cat >"$OVERRIDE" <<'EOF'
struct Box {
    var n: Int = 1
}
public func main() {
    let x = Box(n: 3)
    print(x.n)
}
EOF
out=$("$SYX" run "$OVERRIDE" 2>/tmp/u33_ov_err.txt) || {
  cat /tmp/u33_ov_err.txt
  die "override probe run failed"
}
if [[ "$out" != "3" ]]; then
  echo "got: $out" >&2
  die "override probe output mismatch"
fi
ok "probe_override" "3"
rm -f "$OVERRIDE"

HOLE=$(mktemp /tmp/u33_hole_XXXX.sy)
cat >"$HOLE" <<'EOF'
struct Hole {
    var s: String
}
public func main() {
    let x = Hole()
    print(x.s)
}
EOF
out=$("$SYX" run "$HOLE" 2>/tmp/u33_hole_err.txt) || {
  cat /tmp/u33_hole_err.txt
  die "missing-default probe run failed"
}
if [[ "$out" != "nil" ]]; then
  echo "got: $out" >&2
  die "missing-default probe output mismatch"
fi
ok "probe_missing_nil" "nil"
rm -f "$HOLE"

COUNTER=$(mktemp /tmp/u33_ctr_XXXX.sy)
cat >"$COUNTER" <<'EOF'
struct Counter {
    var value: Int = 0
    mutating func increment() {
        self.value = self.value + 1
    }
}
public func main() {
    var c = Counter()
    c.increment()
    print(c.value)
}
EOF
out=$("$SYX" run "$COUNTER" 2>/tmp/u33_ctr_err.txt) || {
  cat /tmp/u33_ctr_err.txt
  die "Counter probe run failed"
}
if [[ "$out" != "1" ]]; then
  echo "got: $out" >&2
  die "Counter probe output mismatch"
fi
ok "probe_counter" "1"
rm -f "$COUNTER"

cargo test -p syx-driver --test lang field_default_literals -- --test-threads=4 \
  >/tmp/u33_lang_nail.txt 2>&1 || {
  tail -40 /tmp/u33_lang_nail.txt
  die "lang field_default_literals nail failed"
}
ok "lang_nail" "field_default_literals"

cargo test -p syx-mir counter_book_fields_and_defaults -- --test-threads=4 \
  >/tmp/u33_mir_counter.txt 2>&1 || {
  tail -40 /tmp/u33_mir_counter.txt
  die "mir Counter default assert failed"
}
ok "mir_counter_assert" "1"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/u33_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/u33_book_out.txt
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

python3 - <<'PY' || die "roadmap U-33 exit / next park missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if "U-33" not in text:
    raise SystemExit("roadmap missing U-33")
rows = [l for l in text.splitlines() if "| **U-33** |" in l]
if not rows or not any("[x]" in r for r in rows):
    raise SystemExit(f"U-33 not [x]: {rows}")
if "默认下一" in text:
    chunk = text.split("默认下一", 1)[1][:500]
    if "R-50" in chunk and "不" not in chunk and "park" not in chunk.lower() and "未编号" not in chunk:
        raise SystemExit("roadmap must not default next to R-50")
print("roadmap_u33_exit=1")
print("roadmap_no_r50_default=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "u33_field_default_literals_census_ok=0"
  exit 1
fi
echo "u33_field_default_literals_census_ok=1"
echo "current=U-33-field-default-literals"
echo "next=park"
