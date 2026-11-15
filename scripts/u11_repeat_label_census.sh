#!/usr/bin/env bash
# U-11 labeled repeat / break outer census (MIR loop_stack thin knife).
# Soft park: CST postfix async · codegen Break · while case · JIT · SyxHost · D3 · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — body must break (never unbounded loop).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== U-11 repeat-label census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "U-11 surface missing"
from pathlib import Path
ast = Path("crates/syx-ast/src/lib.rs").read_text(encoding="utf-8")
idx = ast.find("RepeatWhile {")
if idx < 0:
    raise SystemExit("RepeatWhile not found")
chunk = ast[idx:idx + 400]
if "label: Option<SmolStr>" not in chunk:
    raise SystemExit("RepeatWhile missing label field")
parser = Path("crates/syx-parser/src/lib.rs").read_text(encoding="utf-8")
if "U-11:" not in parser:
    raise SystemExit("parser missing U-11 marker")
pidx = parser.find("fn parse_repeat_while")
if pidx < 0:
    raise SystemExit("parse_repeat_while not found")
pchunk = parser[pidx:pidx + 700]
if "label," not in pchunk and "label\n" not in pchunk:
    raise SystemExit("parse_repeat_while does not pass label")
mir = Path("crates/syx-mir/src/mir_lower_stmt.rs").read_text(encoding="utf-8")
if "U-11:" not in mir or "label.clone()" not in mir:
    raise SystemExit("MIR RepeatWhile missing U-11 label.clone() push")
# Prefer the lowering arm (not the span-only match).
midx = mir.find("// U-11:")
if midx < 0:
    midx = mir.find("push((cond_block, exit_block, label.clone()))")
if midx < 0 and "label.clone()" not in mir:
    raise SystemExit("MIR RepeatWhile does not push label.clone()")
nail = Path("crates/syx-driver/tests/lang/repeat_label_break.rs")
if not nail.is_file():
    raise SystemExit("missing lang/repeat_label_break.rs")
nt = nail.read_text(encoding="utf-8")
for needle in (
    "u11_labeled_repeat_break_outer_runs",
    "u11_unlabeled_repeat_break_still_runs",
    "u11_labeled_while_break_still_runs",
):
    if needle not in nt:
        raise SystemExit(f"repeat_label_break missing {needle}")
main = Path("crates/syx-driver/tests/lang/main.rs").read_text(encoding="utf-8")
if "mod repeat_label_break;" not in main:
    raise SystemExit("lang/main.rs missing mod repeat_label_break")
print("u11_ast_parser_mir_surface=1")
print("u11_lang_nail=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/u-11-repeat-label-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/u-11-repeat-label-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("U-11", "repeat", "≠", "R-50", "JIT"):
    if needle not in ct:
        raise SystemExit(f"u-11 census missing {needle}")
print("u11_census_doc=1")
PY

SYX=./target/debug/syx
if [[ ! -x "$SYX" ]]; then
  cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
fi

LAB=$(mktemp /tmp/u11_lab_XXXX.sy)
cat >"$LAB" <<'EOF'
public func main() {
    outer: repeat {
        print(1)
        break outer
    } while true
}
EOF
out=$("$SYX" run "$LAB" 2>/tmp/u11_lab_err.txt) || {
  cat /tmp/u11_lab_err.txt
  die "labeled repeat probe run failed"
}
if [[ "$out" != "1" ]]; then
  echo "got: $out" >&2
  die "labeled repeat probe output mismatch"
fi
ok "probe_labeled_repeat_break" "1"
rm -f "$LAB"

UNLAB=$(mktemp /tmp/u11_unlab_XXXX.sy)
cat >"$UNLAB" <<'EOF'
public func main() {
    repeat { break } while false
    print(1)
}
EOF
out=$("$SYX" run "$UNLAB" 2>/tmp/u11_unlab_err.txt) || {
  cat /tmp/u11_unlab_err.txt
  die "unlabeled repeat probe run failed"
}
if [[ "$out" != "1" ]]; then
  echo "got: $out" >&2
  die "unlabeled repeat probe output mismatch"
fi
ok "probe_unlabeled_repeat" "1"
rm -f "$UNLAB"

WHILE=$(mktemp /tmp/u11_while_XXXX.sy)
cat >"$WHILE" <<'EOF'
public func main() {
    outer: while true {
        print(2)
        break outer
    }
}
EOF
out=$("$SYX" run "$WHILE" 2>/tmp/u11_while_err.txt) || {
  cat /tmp/u11_while_err.txt
  die "labeled while probe run failed"
}
if [[ "$out" != "2" ]]; then
  echo "got: $out" >&2
  die "labeled while probe output mismatch"
fi
ok "probe_labeled_while" "2"
rm -f "$WHILE"

cargo test -p syx-driver --test lang repeat_label_break -- --test-threads=4 \
  >/tmp/u11_lang_nail.txt 2>&1 || {
  tail -40 /tmp/u11_lang_nail.txt
  die "lang repeat_label_break nail failed"
}
ok "lang_nail" "repeat_label_break"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/u11_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/u11_book_out.txt
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

python3 - <<'PY' || die "roadmap U-11 exit / next park missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if "U-11" not in text:
    raise SystemExit("roadmap missing U-11")
rows = [l for l in text.splitlines() if "| **U-11** |" in l]
if not rows or not any("[x]" in r for r in rows):
    raise SystemExit(f"U-11 not [x]: {rows}")
if "默认下一" in text:
    chunk = text.split("默认下一", 1)[1][:400]
    if "R-50" in chunk and "不" not in chunk and "park" not in chunk.lower() and "未编号" not in chunk:
        raise SystemExit("roadmap must not default next to R-50")
print("roadmap_u11_exit=1")
print("roadmap_no_r50_default=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "u11_repeat_label_census_ok=0"
  exit 1
fi
echo "u11_repeat_label_census_ok=1"
echo "current=U-11-repeat-label"
echo "next=U-12-or-park"
