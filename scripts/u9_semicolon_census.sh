#!/usr/bin/env bash
# U-9 statement semicolon tolerance census (parser thin knife).
# Soft park: while let · codegen Break · CST postfix async · JIT · SyxHost · D3 · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== U-9 semicolon census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "parser U-9 surface missing"
from pathlib import Path
lib = Path("crates/syx-parser/src/lib.rs").read_text(encoding="utf-8")
if "U-9:" not in lib:
    raise SystemExit("parse_block_stmts missing U-9 marker")
if "optional statement separators" not in lib:
    raise SystemExit("missing U-9 comment on parse_block_stmts")
# Require semicolon eat inside parse_block_stmts (not only class/actor members).
idx = lib.find("fn parse_block_stmts")
if idx < 0:
    raise SystemExit("parse_block_stmts not found")
chunk = lib[idx:idx + 800]
if "TokenKind::Semicolon" not in chunk:
    raise SystemExit("parse_block_stmts does not eat Semicolon")
nail = Path("crates/syx-driver/tests/lang/stmt_semicolon.rs")
if not nail.is_file():
    raise SystemExit("missing lang/stmt_semicolon.rs")
nt = nail.read_text(encoding="utf-8")
for needle in (
    "u9_if_block_then_semicolon_stmt_runs",
    "u9_double_semicolon_tolerated",
    "u9_no_semicolon_if_still_runs",
    "u9_let_semicolon_then_stmt_runs",
):
    if needle not in nt:
        raise SystemExit(f"stmt_semicolon missing {needle}")
main = Path("crates/syx-driver/tests/lang/main.rs").read_text(encoding="utf-8")
if "mod stmt_semicolon;" not in main:
    raise SystemExit("lang/main.rs missing mod stmt_semicolon")
print("u9_parser_surface=1")
print("u9_lang_nail=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/u-9-semicolon-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/u-9-semicolon-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("U-9", "semicolon", "≠", "R-50", "JIT"):
    if needle not in ct:
        raise SystemExit(f"u-9 census missing {needle}")
print("u9_census_doc=1")
PY

SYX=./target/debug/syx
if [[ ! -x "$SYX" ]]; then
  cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
fi

IFSEMI=$(mktemp /tmp/u9_ifsemi_XXXX.sy)
cat >"$IFSEMI" <<'EOF'
public func main() {
    if true { }; print(1)
}
EOF
out=$("$SYX" run "$IFSEMI" 2>/tmp/u9_ifsemi_err.txt) || {
  cat /tmp/u9_ifsemi_err.txt
  die "if-semicolon probe run failed"
}
if [[ "$out" != "1" ]]; then
  echo "got: $out" >&2
  die "if-semicolon probe output mismatch"
fi
ok "probe_if_semicolon" "1"
rm -f "$IFSEMI"

DBL=$(mktemp /tmp/u9_dbl_XXXX.sy)
cat >"$DBL" <<'EOF'
public func main() {
    if true { } ;; print(2)
}
EOF
out=$("$SYX" run "$DBL" 2>/tmp/u9_dbl_err.txt) || {
  cat /tmp/u9_dbl_err.txt
  die "double-semicolon probe run failed"
}
if [[ "$out" != "2" ]]; then
  echo "got: $out" >&2
  die "double-semicolon probe output mismatch"
fi
ok "probe_double_semicolon" "2"
rm -f "$DBL"

LETSEMI=$(mktemp /tmp/u9_letsemi_XXXX.sy)
cat >"$LETSEMI" <<'EOF'
public func main() {
    let x = 1; print(x)
}
EOF
out=$("$SYX" run "$LETSEMI" 2>/tmp/u9_letsemi_err.txt) || {
  cat /tmp/u9_letsemi_err.txt
  die "let-semicolon probe run failed"
}
if [[ "$out" != "1" ]]; then
  echo "got: $out" >&2
  die "let-semicolon probe output mismatch"
fi
ok "probe_let_semicolon" "1"
rm -f "$LETSEMI"

NOSEMI=$(mktemp /tmp/u9_nosemi_XXXX.sy)
cat >"$NOSEMI" <<'EOF'
public func main() {
    if true { print(1) }
}
EOF
out=$("$SYX" run "$NOSEMI" 2>/tmp/u9_nosemi_err.txt) || {
  cat /tmp/u9_nosemi_err.txt
  die "no-semicolon probe run failed"
}
if [[ "$out" != "1" ]]; then
  echo "got: $out" >&2
  die "no-semicolon probe output mismatch"
fi
ok "probe_no_semicolon" "1"
rm -f "$NOSEMI"

cargo test -p syx-driver --test lang stmt_semicolon -- --test-threads=4 \
  >/tmp/u9_lang_nail.txt 2>&1 || {
  tail -40 /tmp/u9_lang_nail.txt
  die "lang stmt_semicolon nail failed"
}
ok "lang_nail" "stmt_semicolon"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/u9_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/u9_book_out.txt
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

python3 - <<'PY' || die "roadmap U-9 exit / next park missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if "U-9" not in text:
    raise SystemExit("roadmap missing U-9")
rows = [l for l in text.splitlines() if "| **U-9** |" in l]
if not rows or not any("[x]" in r for r in rows):
    raise SystemExit(f"U-9 not [x]: {rows}")
if "默认下一" in text:
    chunk = text.split("默认下一", 1)[1][:400]
    if "R-50" in chunk and "不" not in chunk and "park" not in chunk.lower() and "未编号" not in chunk:
        raise SystemExit("roadmap must not default next to R-50")
print("roadmap_u9_exit=1")
print("roadmap_no_r50_default=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "u9_semicolon_census_ok=0"
  exit 1
fi
echo "u9_semicolon_census_ok=1"
echo "current=U-9-semicolon"
echo "next=U-10-or-park"
