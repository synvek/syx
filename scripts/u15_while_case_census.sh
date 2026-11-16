#!/usr/bin/env bash
# U-15 restore while-case census (parser desugar; re-land paper U-12).
# Soft park: comma while-let · guard let · CST · Break · JIT · SyxHost · D3 · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== U-15 while-case census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "parser U-15 surface missing"
from pathlib import Path
lib = Path("crates/syx-parser/src/lib.rs").read_text(encoding="utf-8")
if "U-15:" not in lib:
    raise SystemExit("parse_while missing U-15 marker")
idx = lib.find("fn parse_while")
if idx < 0:
    raise SystemExit("parse_while not found")
chunk = lib[idx:idx + 3500]
if "TokenKind::Case" not in chunk:
    raise SystemExit("parse_while missing Case arm")
if "Stmt::IfCase" not in chunk:
    raise SystemExit("parse_while Case arm missing IfCase")
if "Stmt::Break" not in chunk:
    raise SystemExit("parse_while Case arm missing Break")
nail = Path("crates/syx-driver/tests/lang/while_case.rs")
if not nail.is_file():
    raise SystemExit("missing lang/while_case.rs")
nt = nail.read_text(encoding="utf-8")
for needle in (
    "u15_while_case_bounded_sequence_runs",
    "u15_while_case_immediate_mismatch_skips_body",
    "u15_while_case_multi_match_then_exit",
    "u15_plain_while_still_runs",
    "u15_if_case_still_runs",
    "u15_while_let_still_runs",
):
    if needle not in nt:
        raise SystemExit(f"while_case missing {needle}")
main = Path("crates/syx-driver/tests/lang/main.rs").read_text(encoding="utf-8")
if "mod while_case;" not in main:
    raise SystemExit("lang/main.rs missing mod while_case")
print("u15_parser_surface=1")
print("u15_lang_nail=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/u-15-while-case-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/u-15-while-case-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("U-15", "while case", "≠", "R-50", "JIT", "纸面"):
    if needle not in ct:
        raise SystemExit(f"u-15 census missing {needle}")
print("u15_census_doc=1")
PY

SYX=./target/debug/syx
# Always rebuild so parser Case arm is not shadowed by a stale CLI binary.
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

BOUND=$(mktemp /tmp/u15_bound_XXXX.sy)
cat >"$BOUND" <<'EOF'
var n = 0
public func next() -> Int {
    let v = n
    n = n + 1
    return v
}
public func main() {
    while case 0 = next() {
        print("hit")
    }
    print("done")
}
EOF
out=$("$SYX" run "$BOUND" 2>/tmp/u15_bound_err.txt) || {
  cat /tmp/u15_bound_err.txt
  die "bounded probe run failed"
}
expected=$'hit\ndone'
if [[ "$out" != "$expected" ]]; then
  echo "got: $out" >&2
  die "bounded probe output mismatch"
fi
ok "probe_bounded" "hit_done"
rm -f "$BOUND"

MIS=$(mktemp /tmp/u15_mis_XXXX.sy)
cat >"$MIS" <<'EOF'
public func always() -> Int { return 9 }
public func main() {
    while case 0 = always() { print("no") }
    print("done")
}
EOF
out=$("$SYX" run "$MIS" 2>/tmp/u15_mis_err.txt) || {
  cat /tmp/u15_mis_err.txt
  die "mismatch probe run failed"
}
if [[ "$out" != "done" ]]; then
  echo "got: $out" >&2
  die "mismatch probe output mismatch"
fi
ok "probe_mismatch" "done"
rm -f "$MIS"

MULTI=$(mktemp /tmp/u15_multi_XXXX.sy)
cat >"$MULTI" <<'EOF'
var n = 0
public func next() -> Int {
    if n >= 3 { return 0 }
    n = n + 1
    return 1
}
public func main() {
    while case 1 = next() { print(n) }
    print("done")
}
EOF
out=$("$SYX" run "$MULTI" 2>/tmp/u15_multi_err.txt) || {
  cat /tmp/u15_multi_err.txt
  die "multi-match probe run failed"
}
expected=$'1\n2\n3\ndone'
if [[ "$out" != "$expected" ]]; then
  echo "got: $out" >&2
  die "multi-match probe output mismatch"
fi
ok "probe_multi" "1_2_3_done"
rm -f "$MULTI"

PLAIN=$(mktemp /tmp/u15_plain_XXXX.sy)
cat >"$PLAIN" <<'EOF'
public func main() {
    var i = 0
    while i < 2 {
        print(i)
        i = i + 1
    }
}
EOF
out=$("$SYX" run "$PLAIN" 2>/tmp/u15_plain_err.txt) || {
  cat /tmp/u15_plain_err.txt
  die "plain while probe run failed"
}
expected=$'0\n1'
if [[ "$out" != "$expected" ]]; then
  echo "got: $out" >&2
  die "plain while probe output mismatch"
fi
ok "probe_plain_while" "0_1"
rm -f "$PLAIN"

IFCASE=$(mktemp /tmp/u15_ifcase_XXXX.sy)
cat >"$IFCASE" <<'EOF'
public func f() -> Int { return 7 }
public func main() {
    if case 7 = f() { print("ok") }
}
EOF
out=$("$SYX" run "$IFCASE" 2>/tmp/u15_ifcase_err.txt) || {
  cat /tmp/u15_ifcase_err.txt
  die "if case probe run failed"
}
if [[ "$out" != "ok" ]]; then
  echo "got: $out" >&2
  die "if case probe output mismatch"
fi
ok "probe_if_case" "ok"
rm -f "$IFCASE"

WLET=$(mktemp /tmp/u15_wlet_XXXX.sy)
cat >"$WLET" <<'EOF'
var n = 0
public func next() -> Int? {
    if n >= 2 { return nil }
    let v = n
    n = n + 1
    return v
}
public func main() {
    while let v = next() { print(v) }
}
EOF
out=$("$SYX" run "$WLET" 2>/tmp/u15_wlet_err.txt) || {
  cat /tmp/u15_wlet_err.txt
  die "while let probe run failed"
}
expected=$'0\n1'
if [[ "$out" != "$expected" ]]; then
  echo "got: $out" >&2
  die "while let probe output mismatch"
fi
ok "probe_while_let" "0_1"
rm -f "$WLET"

cargo test -p syx-driver --test lang while_case -- --test-threads=4 \
  >/tmp/u15_lang_nail.txt 2>&1 || {
  tail -40 /tmp/u15_lang_nail.txt
  die "lang while_case nail failed"
}
ok "lang_nail" "while_case"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/u15_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/u15_book_out.txt
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

python3 - <<'PY' || die "roadmap U-15 exit / next park missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if "U-15" not in text:
    raise SystemExit("roadmap missing U-15")
rows = [l for l in text.splitlines() if "| **U-15** |" in l]
if not rows or not any("[x]" in r for r in rows):
    raise SystemExit(f"U-15 not [x]: {rows}")
if "默认下一" in text:
    chunk = text.split("默认下一", 1)[1][:500]
    if "R-50" in chunk and "不" not in chunk and "park" not in chunk.lower() and "未编号" not in chunk:
        raise SystemExit("roadmap must not default next to R-50")
print("roadmap_u15_exit=1")
print("roadmap_no_r50_default=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "u15_while_case_census_ok=0"
  exit 1
fi
echo "u15_while_case_census_ok=1"
echo "current=U-15-while-case"
echo "next=park"
