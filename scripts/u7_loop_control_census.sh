#!/usr/bin/env bash
# U-7 MIR loop control census (repeat / break / continue).
# Soft park: if-let trailing · semicolon · grammar MD · codegen Break · JIT · SyxHost · D3 · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== U-7 loop control MIR census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "MIR loop-control surface missing"
from pathlib import Path
ctx = Path("crates/syx-mir/src/mir_lower_ctx.rs").read_text(encoding="utf-8")
if "loop_stack" not in ctx or "resolve_loop_target" not in ctx:
    raise SystemExit("LowerCtx missing loop_stack / resolve_loop_target")
stmt = Path("crates/syx-mir/src/mir_lower_stmt.rs").read_text(encoding="utf-8")
if "U-7" not in stmt:
    raise SystemExit("mir_lower_stmt missing U-7 markers")
if "Stmt::RepeatWhile" not in stmt or "Stmt::Break" not in stmt:
    raise SystemExit("mir_lower_stmt missing RepeatWhile/Break arms")
nail = Path("crates/syx-driver/tests/lang/loop_control_mir.rs")
if not nail.is_file():
    raise SystemExit("missing lang/loop_control_mir.rs")
nt = nail.read_text(encoding="utf-8")
for needle in ("u7_while_break_runs", "u7_for_continue_runs", "u7_repeat_while_runs"):
    if needle not in nt:
        raise SystemExit(f"loop_control_mir missing {needle}")
main = Path("crates/syx-driver/tests/lang/main.rs").read_text(encoding="utf-8")
if "mod loop_control_mir;" not in main:
    raise SystemExit("lang/main.rs missing mod loop_control_mir")
print("u7_mir_surface=1")
print("u7_lang_nail=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/u-7-loop-control-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/u-7-loop-control-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("U-7", "break", "repeat", "≠", "R-50", "JIT"):
    if needle not in ct:
        raise SystemExit(f"u-7 census missing {needle}")
print("u7_census_doc=1")
PY

SYX=./target/debug/syx
if [[ ! -x "$SYX" ]]; then
  cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
fi

BR=$(mktemp /tmp/u7_br_XXXX.sy)
cat >"$BR" <<'EOF'
public func main() {
    var i = 0
    while i < 5 {
        i = i + 1
        if i == 2 { break }
    }
    print(i)
}
EOF
out=$("$SYX" run "$BR" 2>/tmp/u7_br_err.txt) || {
  cat /tmp/u7_br_err.txt
  die "break probe run failed"
}
if [[ "$out" != "2" ]]; then
  echo "got: $out" >&2
  die "break probe output mismatch"
fi
ok "probe_break" "2"
rm -f "$BR"

CONT=$(mktemp /tmp/u7_cont_XXXX.sy)
cat >"$CONT" <<'EOF'
public func main() {
    var s = 0
    for i in 0..<5 {
        if i == 2 { continue }
        s = s + i
    }
    print(s)
}
EOF
out=$("$SYX" run "$CONT" 2>/tmp/u7_cont_err.txt) || {
  cat /tmp/u7_cont_err.txt
  die "continue probe run failed"
}
if [[ "$out" != "8" ]]; then
  echo "got: $out" >&2
  die "continue probe output mismatch"
fi
ok "probe_continue" "8"
rm -f "$CONT"

RW=$(mktemp /tmp/u7_rw_XXXX.sy)
cat >"$RW" <<'EOF'
public func main() {
    var n = 0
    repeat {
        n = n + 1
    } while n < 3
    print(n)
}
EOF
out=$("$SYX" run "$RW" 2>/tmp/u7_rw_err.txt) || {
  cat /tmp/u7_rw_err.txt
  die "repeat probe run failed"
}
if [[ "$out" != "3" ]]; then
  echo "got: $out" >&2
  die "repeat probe output mismatch"
fi
ok "probe_repeat" "3"
rm -f "$RW"

cargo test -p syx-driver --test lang loop_control_mir -- --test-threads=4 \
  >/tmp/u7_lang_nail.txt 2>&1 || {
  tail -40 /tmp/u7_lang_nail.txt
  die "lang loop_control_mir nail failed"
}
ok "lang_nail" "loop_control_mir"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/u7_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/u7_book_out.txt
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

python3 - <<'PY' || die "roadmap U-7 exit / next park missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if "U-7" not in text:
    raise SystemExit("roadmap missing U-7")
rows = [l for l in text.splitlines() if "| **U-7** |" in l]
if not rows or not any("[x]" in r for r in rows):
    raise SystemExit(f"U-7 not [x]: {rows}")
if "默认下一" in text:
    chunk = text.split("默认下一", 1)[1][:400]
    if "R-50" in chunk and "不" not in chunk and "park" not in chunk.lower() and "未编号" not in chunk:
        raise SystemExit("roadmap must not default next to R-50")
print("roadmap_u7_exit=1")
print("roadmap_no_r50_default=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "u7_loop_control_census_ok=0"
  exit 1
fi
echo "u7_loop_control_census_ok=1"
echo "current=U-7-loop-control"
echo "next=U-8-or-park"
