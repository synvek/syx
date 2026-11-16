#!/usr/bin/env bash
# U-18 guard-let else-must-exit census (S027; ≠ MainActor S020).
# Soft park: bare-guard If exit · fatalError · comma while-let · CST · Break · JIT · SyxHost · D3 · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== U-18 guard-else-exit census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "semantic U-18 surface missing"
from pathlib import Path
helpers = Path("crates/syx-semantic/src/semantic_helpers.rs").read_text(encoding="utf-8")
if "fn block_definitely_exits" not in helpers:
    raise SystemExit("missing block_definitely_exits")
stmt = Path("crates/syx-semantic/src/semantic_analyze_stmt.rs").read_text(encoding="utf-8")
if "S027" not in stmt or "block_definitely_exits" not in stmt:
    raise SystemExit("GuardLet arm missing S027 / block_definitely_exits")
if "U-18" not in stmt:
    raise SystemExit("missing U-18 marker on GuardLet")
nail = Path("crates/syx-driver/tests/lang/guard_else_exit.rs")
if not nail.is_file():
    raise SystemExit("missing lang/guard_else_exit.rs")
nt = nail.read_text(encoding="utf-8")
for needle in (
    "u18_print_only_else_is_s027",
    "u18_empty_else_is_s027",
    "u18_return_else_ok_and_binds",
    "u18_throw_else_ok",
    "u18_break_else_ok",
    "u18_continue_else_ok",
    "u18_comma_guard_let_else_must_exit",
    "u18_comma_guard_let_return_ok",
    "u18_bare_guard_empty_else_not_s027",
):
    if needle not in nt:
        raise SystemExit(f"guard_else_exit missing {needle}")
main = Path("crates/syx-driver/tests/lang/main.rs").read_text(encoding="utf-8")
if "mod guard_else_exit;" not in main:
    raise SystemExit("lang/main.rs missing mod guard_else_exit")
print("u18_semantic_surface=1")
print("u18_lang_nail=1")
PY

python3 - <<'PY' || die "census/books hygiene failed"
from pathlib import Path
census = Path("docs/census/u-18-guard-else-exit-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/u-18-guard-else-exit-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("U-18", "S027", "≠", "R-50", "JIT", "MainActor"):
    if needle not in ct:
        raise SystemExit(f"u-18 census missing {needle}")
# Books: guard-else must say S027, not S020 in Ch06 diagnostic row.
for rel in (
    "books/language-guide/06-control-flow.md",
    "books/zh-cn/language-guide/06-control-flow.md",
    "books/language-guide/30-advanced-operators.md",
    "books/zh-cn/language-guide/30-advanced-operators.md",
    "books/language-reference/04-statements.md",
    "books/zh-cn/language-reference/04-statements.md",
):
    t = Path(rel).read_text(encoding="utf-8")
    if "S027" not in t:
        raise SystemExit(f"{rel} missing S027")
    # Ch06 diagnostic table row for guard else must not still say S020
    if "06-control-flow" in rel:
        if "| **S020** | `guard let`" in t or "| **S020** | `guard let` `else`" in t:
            raise SystemExit(f"{rel} still has S020 guard-let diag row")
        if "compile error **S020**" in t and "guard" in t.lower():
            # tolerate other S020 mentions only if not guard-else callout
            pass
        if "compile error **S027**" not in t and "编译错误 **S027**" not in t:
            raise SystemExit(f"{rel} missing S027 callout")
print("u18_census_doc=1")
print("u18_books_s027=1")
PY

SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

BAD=$(mktemp /tmp/u18_bad_XXXX.sy)
cat >"$BAD" <<'EOF'
public func main() {
    let a: Int? = nil
    guard let x = a else {
        print("e")
    }
    print("after")
}
EOF
if "$SYX" check "$BAD" >/tmp/u18_bad_out.txt 2>&1; then
  die "print-only else must fail check"
fi
if ! grep -q 'S027' /tmp/u18_bad_out.txt; then
  cat /tmp/u18_bad_out.txt
  die "print-only else must report S027"
fi
ok "probe_print_else_s027" "1"
rm -f "$BAD"

EMPTY=$(mktemp /tmp/u18_empty_XXXX.sy)
cat >"$EMPTY" <<'EOF'
public func main() {
    let a: Int? = nil
    guard let x = a else {
    }
    print("after")
}
EOF
if "$SYX" check "$EMPTY" >/tmp/u18_empty_out.txt 2>&1; then
  die "empty else must fail check"
fi
if ! grep -q 'S027' /tmp/u18_empty_out.txt; then
  cat /tmp/u18_empty_out.txt
  die "empty else must report S027"
fi
ok "probe_empty_else_s027" "1"
rm -f "$EMPTY"

OK=$(mktemp /tmp/u18_ok_XXXX.sy)
cat >"$OK" <<'EOF'
public func main() {
    let a: Int? = 7
    guard let x = a else {
        return
    }
    print(x)
}
EOF
out=$("$SYX" run "$OK" 2>/tmp/u18_ok_err.txt) || {
  cat /tmp/u18_ok_err.txt
  die "return else probe run failed"
}
if [[ "$out" != "7" ]]; then
  echo "got: $out" >&2
  die "return else probe output mismatch"
fi
ok "probe_return_else" "7"
rm -f "$OK"

BARE=$(mktemp /tmp/u18_bare_XXXX.sy)
cat >"$BARE" <<'EOF'
public func main() {
    guard false else {
    }
    print("after")
}
EOF
if ! "$SYX" check "$BARE" >/tmp/u18_bare_out.txt 2>&1; then
  if grep -q 'S027' /tmp/u18_bare_out.txt; then
    cat /tmp/u18_bare_out.txt
    die "bare guard must not be S027"
  fi
fi
ok "probe_bare_not_s027" "1"
rm -f "$BARE"

cargo test -p syx-driver --test lang guard_else_exit -- --test-threads=4 \
  >/tmp/u18_lang_nail.txt 2>&1 || {
  tail -40 /tmp/u18_lang_nail.txt
  die "lang guard_else_exit nail failed"
}
ok "lang_nail" "guard_else_exit"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/u18_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/u18_book_out.txt
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

python3 - <<'PY' || die "roadmap U-18 exit / next park missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if "U-18" not in text:
    raise SystemExit("roadmap missing U-18")
rows = [l for l in text.splitlines() if "| **U-18** |" in l]
if not rows or not any("[x]" in r for r in rows):
    raise SystemExit(f"U-18 not [x]: {rows}")
if "默认下一" in text:
    chunk = text.split("默认下一", 1)[1][:500]
    if "R-50" in chunk and "不" not in chunk and "park" not in chunk.lower() and "未编号" not in chunk:
        raise SystemExit("roadmap must not default next to R-50")
print("roadmap_u18_exit=1")
print("roadmap_no_r50_default=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "u18_guard_else_exit_census_ok=0"
  exit 1
fi
echo "u18_guard_else_exit_census_ok=1"
echo "current=U-18-guard-else-exit"
echo "next=park"
