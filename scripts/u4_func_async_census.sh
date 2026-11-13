#!/usr/bin/env bash
# U-4 func…async dual-accept census (parser thin knife).
# Soft park: credibility 4 MD · JIT · SyxHost · D3 · R-50 · book rewrite.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== U-4 func async dual census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "parser dual-accept surface missing"
from pathlib import Path
lib = Path("crates/syx-parser/src/lib.rs").read_text(encoding="utf-8")
if "U-4: Swift-style postfix" not in lib:
    raise SystemExit("parse_function_with_name_opts missing U-4 postfix async")
if "match_kind(TokenKind::Async)" not in lib:
    raise SystemExit("parser missing Async match after params")
nail = Path("crates/syx-driver/tests/lang/func_async_dual.rs")
if not nail.is_file():
    raise SystemExit("missing lang/func_async_dual.rs")
nt = nail.read_text(encoding="utf-8")
for needle in ("func() async", "async func", "async -> Int"):
    if needle not in nt and "async ->" not in nt and "postfix" not in nt:
        pass
if "u4_postfix_func_async" not in nt or "u4_prefix_async_func" not in nt:
    raise SystemExit("func_async_dual missing postfix/prefix tests")
main = Path("crates/syx-driver/tests/lang/main.rs").read_text(encoding="utf-8")
if "mod func_async_dual;" not in main:
    raise SystemExit("lang/main.rs missing mod func_async_dual")
print("u4_parser_surface=1")
print("u4_lang_nail=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/u-4-func-async-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/u-4-func-async-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("U-4", "func…async", "async func", "≠", "R-50", "JIT"):
    if needle not in ct:
        raise SystemExit(f"u-4 census missing {needle}")
print("u4_census_doc=1")
PY

SYX=./target/debug/syx
if [[ ! -x "$SYX" ]]; then
  cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
fi

PROBE=$(mktemp /tmp/u4_probe_XXXX.sy)
cat >"$PROBE" <<'EOF'
func f() async -> Int {
    return 7
}
public func main() {
    print(f())
}
EOF
"$SYX" check "$PROBE" >/tmp/u4_probe_check.txt 2>&1 || {
  tail -20 /tmp/u4_probe_check.txt
  die "postfix probe check failed"
}
out=$("$SYX" run "$PROBE" 2>/tmp/u4_probe_run_err.txt) || {
  cat /tmp/u4_probe_run_err.txt
  die "postfix probe run failed"
}
if [[ "$out" != "7" ]]; then
  echo "got: $out" >&2
  die "postfix probe output mismatch"
fi
ok "probe_postfix" "async->Int=7"
rm -f "$PROBE"

PREFIX=$(mktemp /tmp/u4_prefix_XXXX.sy)
cat >"$PREFIX" <<'EOF'
async func f() {
    print(9)
}
public func main() {
    f()
}
EOF
out=$("$SYX" run "$PREFIX" 2>/tmp/u4_prefix_err.txt) || {
  cat /tmp/u4_prefix_err.txt
  die "prefix probe run failed"
}
if [[ "$out" != "9" ]]; then
  echo "got: $out" >&2
  die "prefix probe output mismatch"
fi
ok "probe_prefix" "async_func=9"
rm -f "$PREFIX"

cargo test -p syx-driver --test lang func_async_dual -- --test-threads=4 \
  >/tmp/u4_lang_nail.txt 2>&1 || {
  tail -40 /tmp/u4_lang_nail.txt
  die "lang func_async_dual nail failed"
}
ok "lang_nail" "func_async_dual"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/u4_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/u4_book_out.txt
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

python3 - <<'PY' || die "roadmap U-4 exit / next park missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if "U-4" not in text:
    raise SystemExit("roadmap missing U-4")
rows = [l for l in text.splitlines() if "| **U-4** |" in l]
if not rows or not any("[x]" in r for r in rows):
    raise SystemExit(f"U-4 not [x]: {rows}")
if "默认下一" in text:
    chunk = text.split("默认下一", 1)[1][:400]
    if "R-50" in chunk and "不" not in chunk and "park" not in chunk.lower() and "未编号" not in chunk:
        raise SystemExit("roadmap must not default next to R-50")
print("roadmap_u4_exit=1")
print("roadmap_no_r50_default=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "u4_func_async_census_ok=0"
  exit 1
fi
echo "u4_func_async_census_ok=1"
echo "current=U-4-func-async"
echo "next=U-5-or-park"
