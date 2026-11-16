#!/usr/bin/env bash
# U-17 bare guard census (parser dispatch; If desugar).
# Soft park: S020 · comma while-let · guard case · CST · Break · JIT · SyxHost · D3 · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== U-17 guard-bare census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "parser U-17 surface missing"
from pathlib import Path
lib = Path("crates/syx-parser/src/lib.rs").read_text(encoding="utf-8")
if "fn parse_guard(" not in lib and "fn parse_guard(&" not in lib:
    # rustc: fn parse_guard(&mut self)
    if "fn parse_guard(" not in lib:
        raise SystemExit("parse_guard not found")
if "U-17" not in lib:
    raise SystemExit("missing U-17 marker")
idx = lib.find("fn parse_guard(")
chunk = lib[idx:idx + 2500]
if "parse_guard_let" not in chunk:
    raise SystemExit("parse_guard must dispatch to parse_guard_let")
if "Stmt::If" not in chunk:
    raise SystemExit("bare guard missing Stmt::If desugar")
nail = Path("crates/syx-driver/tests/lang/guard_bare.rs")
if not nail.is_file():
    raise SystemExit("missing lang/guard_bare.rs")
nt = nail.read_text(encoding="utf-8")
for needle in (
    "u17_guard_true_continues",
    "u17_guard_false_takes_else",
    "u17_guard_expr_cond_continues",
    "u17_single_guard_let_still_runs",
    "u17_comma_guard_let_still_runs",
):
    if needle not in nt:
        raise SystemExit(f"guard_bare missing {needle}")
main = Path("crates/syx-driver/tests/lang/main.rs").read_text(encoding="utf-8")
if "mod guard_bare;" not in main:
    raise SystemExit("lang/main.rs missing mod guard_bare")
print("u17_parser_surface=1")
print("u17_lang_nail=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/u-17-guard-bare-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/u-17-guard-bare-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("U-17", "guard", "≠", "R-50", "JIT", "S020"):
    if needle not in ct:
        raise SystemExit(f"u-17 census missing {needle}")
print("u17_census_doc=1")
PY

SYX=./target/debug/syx
# Always rebuild so parser U-17 arm is not shadowed by a stale CLI binary.
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

TRUE=$(mktemp /tmp/u17_true_XXXX.sy)
cat >"$TRUE" <<'EOF'
public func main() {
    guard true else {
        print("else")
        return
    }
    print(1)
}
EOF
out=$("$SYX" run "$TRUE" 2>/tmp/u17_true_err.txt) || {
  cat /tmp/u17_true_err.txt
  die "guard-true probe run failed"
}
if [[ "$out" != "1" ]]; then
  echo "got: $out" >&2
  die "guard-true probe output mismatch"
fi
ok "probe_guard_true" "1"
rm -f "$TRUE"

FALSE=$(mktemp /tmp/u17_false_XXXX.sy)
cat >"$FALSE" <<'EOF'
public func main() {
    guard false else {
        print("else")
        return
    }
    print("no")
}
EOF
out=$("$SYX" run "$FALSE" 2>/tmp/u17_false_err.txt) || {
  cat /tmp/u17_false_err.txt
  die "guard-false probe run failed"
}
if [[ "$out" != "else" ]]; then
  echo "got: $out" >&2
  die "guard-false probe output mismatch"
fi
ok "probe_guard_false" "else"
rm -f "$FALSE"

EXPR=$(mktemp /tmp/u17_expr_XXXX.sy)
cat >"$EXPR" <<'EOF'
public func main() {
    let n = 3
    guard n > 0 else {
        print("else")
        return
    }
    print(n)
}
EOF
out=$("$SYX" run "$EXPR" 2>/tmp/u17_expr_err.txt) || {
  cat /tmp/u17_expr_err.txt
  die "guard-expr probe run failed"
}
if [[ "$out" != "3" ]]; then
  echo "got: $out" >&2
  die "guard-expr probe output mismatch"
fi
ok "probe_guard_expr" "3"
rm -f "$EXPR"

GLET=$(mktemp /tmp/u17_glet_XXXX.sy)
cat >"$GLET" <<'EOF'
public func main() {
    let a: Int? = 7
    guard let x = a else { return }
    print(x)
}
EOF
out=$("$SYX" run "$GLET" 2>/tmp/u17_glet_err.txt) || {
  cat /tmp/u17_glet_err.txt
  die "guard-let probe run failed"
}
if [[ "$out" != "7" ]]; then
  echo "got: $out" >&2
  die "guard-let probe output mismatch"
fi
ok "probe_guard_let" "7"
rm -f "$GLET"

GCOMMA=$(mktemp /tmp/u17_gcomma_XXXX.sy)
cat >"$GCOMMA" <<'EOF'
public func main() {
    let a: Int? = 1
    let b: Int? = 2
    guard let x = a, let y = b else {
        print("else")
        return
    }
    print(x)
    print(y)
}
EOF
out=$("$SYX" run "$GCOMMA" 2>/tmp/u17_gcomma_err.txt) || {
  cat /tmp/u17_gcomma_err.txt
  die "comma guard-let probe run failed"
}
expected=$'1\n2'
if [[ "$out" != "$expected" ]]; then
  echo "got: $out" >&2
  die "comma guard-let probe output mismatch"
fi
ok "probe_comma_guard_let" "1_2"
rm -f "$GCOMMA"

cargo test -p syx-driver --test lang guard_bare -- --test-threads=4 \
  >/tmp/u17_lang_nail.txt 2>&1 || {
  tail -40 /tmp/u17_lang_nail.txt
  die "lang guard_bare nail failed"
}
ok "lang_nail" "guard_bare"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/u17_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/u17_book_out.txt
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

python3 - <<'PY' || die "roadmap U-17 exit / next park missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if "U-17" not in text:
    raise SystemExit("roadmap missing U-17")
rows = [l for l in text.splitlines() if "| **U-17** |" in l]
if not rows or not any("[x]" in r for r in rows):
    raise SystemExit(f"U-17 not [x]: {rows}")
if "默认下一" in text:
    chunk = text.split("默认下一", 1)[1][:500]
    if "R-50" in chunk and "不" not in chunk and "park" not in chunk.lower() and "未编号" not in chunk:
        raise SystemExit("roadmap must not default next to R-50")
print("roadmap_u17_exit=1")
print("roadmap_no_r50_default=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "u17_guard_bare_census_ok=0"
  exit 1
fi
echo "u17_guard_bare_census_ok=1"
echo "current=U-17-guard-bare"
echo "next=park"
