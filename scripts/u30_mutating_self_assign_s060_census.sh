#!/usr/bin/env bash
# U-30 non-mutating self write → S060 census (struct/enum method body gate).
# Soft park: comma while-let · field-default nil · closure throws · CST · Break · JIT · SyxHost · D3 · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== U-30 mutating self-assign S060 census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "semantic U-30 surface missing"
from pathlib import Path
ana = Path("crates/syx-semantic/src/semantic_analyzer.rs").read_text(encoding="utf-8")
if "current_function_is_mutating" not in ana:
    raise SystemExit("missing current_function_is_mutating")
tm = Path("crates/syx-semantic/src/semantic_analyze_type_member.rs").read_text(encoding="utf-8")
if "self_mutable_for_method" not in tm or "U-30" not in tm:
    raise SystemExit("missing self_mutable_for_method / U-30")
call = Path("crates/syx-semantic/src/semantic_analyze_call.rs").read_text(encoding="utf-8")
if "enclosing_value_field_assign_blocked" not in call:
    raise SystemExit("missing enclosing_value_field_assign_blocked")
nail = Path("crates/syx-driver/tests/lang/mutating_self_assign_s060.rs")
if not nail.is_file():
    raise SystemExit("missing lang/mutating_self_assign_s060.rs")
nt = nail.read_text(encoding="utf-8")
for needle in (
    "u30_non_mutating_self_member_is_s060",
    "u30_non_mutating_implicit_field_is_s060",
    "u30_mutating_self_member_ok",
    "u30_class_self_member_ok",
    "u30_init_self_member_ok",
    "u30_let_mutating_call_still_s060",
    "u30_var_mutating_runs",
):
    if needle not in nt:
        raise SystemExit(f"mutating_self_assign_s060 missing {needle}")
main = Path("crates/syx-driver/tests/lang/main.rs").read_text(encoding="utf-8")
if "mod mutating_self_assign_s060;" not in main:
    raise SystemExit("lang/main.rs missing mod mutating_self_assign_s060")
for rel in (
    "books/language-guide/12-methods.md",
    "books/zh-cn/language-guide/12-methods.md",
    "books/language-guide/appendix-b-diagnostic-index.md",
    "books/zh-cn/language-guide/appendix-b-diagnostic-index.md",
):
    t = Path(rel).read_text(encoding="utf-8")
    if "S060" not in t:
        raise SystemExit(f"{rel} missing S060")
print("u30_semantic_surface=1")
print("u30_lang_nail=1")
print("u30_books_thin=1")
PY

python3 - <<'PY' || die "census doc missing"
from pathlib import Path
census = Path("docs/census/u-30-mutating-self-assign-s060-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/u-30-mutating-self-assign-s060-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("U-30", "S060", "mutating", "≠", "R-50", "JIT"):
    if needle not in ct:
        raise SystemExit(f"u-30 census missing {needle}")
print("u30_census_doc=1")
PY

SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

SELF=$(mktemp /tmp/u30_self_XXXX.sy)
cat >"$SELF" <<'EOF'
struct S {
    var x: Int = 0
    func f() { self.x = 1 }
}
public func main() {
    var s = S()
    s.f()
}
EOF
if "$SYX" check "$SELF" >/tmp/u30_self_out.txt 2>&1; then
  die "non-mutating self.x= must fail check"
fi
if ! grep -q 'S060' /tmp/u30_self_out.txt; then
  cat /tmp/u30_self_out.txt
  die "non-mutating self.x= must report S060"
fi
ok "probe_non_mutating_self_s060" "1"
rm -f "$SELF"

IMPL=$(mktemp /tmp/u30_impl_XXXX.sy)
cat >"$IMPL" <<'EOF'
struct S {
    var x: Int = 0
    func f() { x = 1 }
}
public func main() {
    var s = S()
    s.f()
}
EOF
if "$SYX" check "$IMPL" >/tmp/u30_impl_out.txt 2>&1; then
  die "non-mutating implicit field must fail check"
fi
if ! grep -q 'S060' /tmp/u30_impl_out.txt; then
  cat /tmp/u30_impl_out.txt
  die "non-mutating implicit field must report S060"
fi
ok "probe_non_mutating_implicit_s060" "1"
rm -f "$IMPL"

MUT=$(mktemp /tmp/u30_mut_XXXX.sy)
cat >"$MUT" <<'EOF'
struct S {
    var x: Int = 0
    mutating func f() { self.x = 1 }
}
public func main() {
    var s = S()
    s.f()
    print(s.x)
}
EOF
out=$("$SYX" run "$MUT" 2>/tmp/u30_mut_err.txt) || {
  cat /tmp/u30_mut_err.txt
  die "mutating self run failed"
}
if [[ "$out" != "1" ]]; then
  echo "got: $out" >&2
  die "mutating self output mismatch"
fi
ok "probe_mutating_run" "1"
rm -f "$MUT"

CLS=$(mktemp /tmp/u30_cls_XXXX.sy)
cat >"$CLS" <<'EOF'
class C {
    var x: Int = 0
    func f() { self.x = 1 }
}
public func main() {
    let c = C()
    c.f()
    print(c.x)
}
EOF
out=$("$SYX" run "$CLS" 2>/tmp/u30_cls_err.txt) || {
  cat /tmp/u30_cls_err.txt
  die "class self run failed"
}
if [[ "$out" != "1" ]]; then
  echo "got: $out" >&2
  die "class self output mismatch"
fi
ok "probe_class_run" "1"
rm -f "$CLS"

LET=$(mktemp /tmp/u30_let_XXXX.sy)
cat >"$LET" <<'EOF'
struct S {
    var x: Int = 0
    mutating func f() { self.x = 1 }
}
public func main() {
    let s = S()
    s.f()
}
EOF
if "$SYX" check "$LET" >/tmp/u30_let_out.txt 2>&1; then
  die "let mutating call must still fail check"
fi
if ! grep -q 'S060' /tmp/u30_let_out.txt; then
  cat /tmp/u30_let_out.txt
  die "let mutating call must still report S060"
fi
ok "probe_let_mutating_call_s060" "1"
rm -f "$LET"

cargo test -p syx-driver --test lang mutating_self_assign_s060 -- --test-threads=4 \
  >/tmp/u30_lang_nail.txt 2>&1 || {
  tail -40 /tmp/u30_lang_nail.txt
  die "lang mutating_self_assign_s060 nail failed"
}
ok "lang_nail" "mutating_self_assign_s060"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/u30_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/u30_book_out.txt
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

python3 - <<'PY' || die "roadmap U-30 exit / next park missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if "U-30" not in text:
    raise SystemExit("roadmap missing U-30")
rows = [l for l in text.splitlines() if "| **U-30** |" in l]
if not rows or not any("[x]" in r for r in rows):
    raise SystemExit(f"U-30 not [x]: {rows}")
if "默认下一" in text:
    chunk = text.split("默认下一", 1)[1][:500]
    if "R-50" in chunk and "不" not in chunk and "park" not in chunk.lower() and "未编号" not in chunk:
        raise SystemExit("roadmap must not default next to R-50")
print("roadmap_u30_exit=1")
print("roadmap_no_r50_default=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "u30_mutating_self_assign_s060_census_ok=0"
  exit 1
fi
echo "u30_mutating_self_assign_s060_census_ok=1"
echo "current=U-30-mutating-self-assign-s060"
echo "next=park"
