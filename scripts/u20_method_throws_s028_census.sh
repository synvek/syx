#!/usr/bin/env bash
# U-20 method-throws S028 census (type-member throws flag; ≠ closure throws; ≠ S020 bag MD).
# Soft park: closure throws · mutating/await S020 retarget · comma while-let · CST · Break · JIT · SyxHost · D3 · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== U-20 method-throws S028 census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "semantic U-20 surface missing"
from pathlib import Path
tm = Path("crates/syx-semantic/src/semantic_analyze_type_member.rs").read_text(encoding="utf-8")
if "U-20" not in tm:
    raise SystemExit("missing U-20 marker on analyze_type_member")
if "current_function_is_throws" not in tm:
    raise SystemExit("type_member missing current_function_is_throws")
if "throws_type.is_some()" not in tm:
    raise SystemExit("Function arm missing throws_type.is_some()")
if "self.current_function_is_throws = *throws" not in tm:
    raise SystemExit("Init arm missing throws bool wiring")
nail = Path("crates/syx-driver/tests/lang/method_throws_s028.rs")
if not nail.is_file():
    raise SystemExit("missing lang/method_throws_s028.rs")
nt = nail.read_text(encoding="utf-8")
for needle in (
    "u20_throws_method_throw_ok",
    "u20_non_throws_method_is_s028",
    "u20_throws_method_do_catch_runs",
    "u20_toplevel_throws_still_ok",
    "u20_init_throws_ok",
):
    if needle not in nt:
        raise SystemExit(f"method_throws_s028 missing {needle}")
main = Path("crates/syx-driver/tests/lang/main.rs").read_text(encoding="utf-8")
if "mod method_throws_s028;" not in main:
    raise SystemExit("lang/main.rs missing mod method_throws_s028")
print("u20_semantic_surface=1")
print("u20_lang_nail=1")
PY

python3 - <<'PY' || die "census hygiene failed"
from pathlib import Path
census = Path("docs/census/u-20-method-throws-s028-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/u-20-method-throws-s028-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("U-20", "S028", "≠", "R-50", "JIT", "analyze_type_member"):
    if needle not in ct:
        raise SystemExit(f"u-20 census missing {needle}")
print("u20_census_doc=1")
PY

SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

THROWS=$(mktemp /tmp/u20_throws_XXXX.sy)
cat >"$THROWS" <<'EOF'
class C {
    func f() throws {
        throw "x"
    }
}
public func main() {
    do {
        try C().f()
    } catch {
        print("caught")
    }
}
EOF
if ! "$SYX" check "$THROWS" >/tmp/u20_throws_out.txt 2>&1; then
  cat /tmp/u20_throws_out.txt
  die "throws method must pass check"
fi
ok "probe_throws_method_check" "1"
out=$("$SYX" run "$THROWS" 2>/tmp/u20_throws_err.txt) || {
  cat /tmp/u20_throws_err.txt
  die "throws method run failed"
}
if [[ "$out" != "caught" ]]; then
  echo "got: $out" >&2
  die "throws method output mismatch"
fi
ok "probe_throws_method_run" "caught"
rm -f "$THROWS"

BAD=$(mktemp /tmp/u20_bad_XXXX.sy)
cat >"$BAD" <<'EOF'
class C {
    func f() {
        throw "x"
    }
}
public func main() {
    C().f()
}
EOF
if "$SYX" check "$BAD" >/tmp/u20_bad_out.txt 2>&1; then
  die "non-throws method must fail check"
fi
if ! grep -q 'S028' /tmp/u20_bad_out.txt; then
  cat /tmp/u20_bad_out.txt
  die "non-throws method must report S028"
fi
ok "probe_non_throws_s028" "1"
rm -f "$BAD"

TOP=$(mktemp /tmp/u20_top_XXXX.sy)
cat >"$TOP" <<'EOF'
func f() throws {
    throw "x"
}
public func main() {
    do {
        try f()
    } catch {
        print("caught")
    }
}
EOF
if ! "$SYX" check "$TOP" >/tmp/u20_top_out.txt 2>&1; then
  cat /tmp/u20_top_out.txt
  die "top-level throws must stay green"
fi
ok "probe_toplevel_throws" "1"
rm -f "$TOP"

cargo test -p syx-driver --test lang method_throws_s028 -- --test-threads=4 \
  >/tmp/u20_lang_nail.txt 2>&1 || {
  tail -40 /tmp/u20_lang_nail.txt
  die "lang method_throws_s028 nail failed"
}
ok "lang_nail" "method_throws_s028"

cargo test -p syx-driver --test lang throw_requires_throws -- --test-threads=4 \
  >/tmp/u20_u19_nail.txt 2>&1 || {
  tail -40 /tmp/u20_u19_nail.txt
  die "U-19 throw_requires_throws nail regress"
}
ok "u19_nail" "throw_requires_throws"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/u20_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/u20_book_out.txt
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

python3 - <<'PY' || die "roadmap U-20 exit / next park missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if "U-20" not in text:
    raise SystemExit("roadmap missing U-20")
rows = [l for l in text.splitlines() if "| **U-20** |" in l]
if not rows or not any("[x]" in r for r in rows):
    raise SystemExit(f"U-20 not [x]: {rows}")
if "默认下一" in text:
    chunk = text.split("默认下一", 1)[1][:500]
    if "R-50" in chunk and "不" not in chunk and "park" not in chunk.lower() and "未编号" not in chunk:
        raise SystemExit("roadmap must not default next to R-50")
print("roadmap_u20_exit=1")
print("roadmap_no_r50_default=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "u20_method_throws_s028_census_ok=0"
  exit 1
fi
echo "u20_method_throws_s028_census_ok=1"
echo "current=U-20-method-throws-s028"
echo "next=park"
