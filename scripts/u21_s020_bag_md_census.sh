#!/usr/bin/env bash
# U-21 S020 bag MD census (mutating→S060 / await→A002; ≠ closure throws / comma while-let / R-50).
# Soft park: closure throws · Ch15/Ch28 S020 · comma while-let · CST · Break · JIT · SyxHost · D3 · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== U-21 S020-bag MD census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "U-21 nail / books surface missing"
from pathlib import Path
nail = Path("crates/syx-driver/tests/lang/s020_bag_honesty.rs")
if not nail.is_file():
    raise SystemExit("missing lang/s020_bag_honesty.rs")
nt = nail.read_text(encoding="utf-8")
for needle in (
    "u21_mutating_let_is_s060",
    "u21_await_outside_async_is_a002",
    "u21_mutating_not_s020_via_check",
):
    if needle not in nt:
        raise SystemExit(f"s020_bag_honesty missing {needle}")
main = Path("crates/syx-driver/tests/lang/main.rs").read_text(encoding="utf-8")
if "mod s020_bag_honesty;" not in main:
    raise SystemExit("lang/main.rs missing mod s020_bag_honesty")
# mutating books
for rel in (
    "books/language-guide/10-structures-and-classes.md",
    "books/zh-cn/language-guide/10-structures-and-classes.md",
    "books/language-guide/12-methods.md",
    "books/zh-cn/language-guide/12-methods.md",
):
    t = Path(rel).read_text(encoding="utf-8")
    if "S060" not in t:
        raise SystemExit(f"{rel} missing S060")
    if "S020: cannot mutate" in t or "S020：cannot mutate" in t:
        raise SystemExit(f"{rel} still S020 mutate comment")
    if "| **S020** | `mutating`" in t or "| **S020** | `mutating` 调用于" in t:
        raise SystemExit(f"{rel} still S020 mutating diag")
# await books
for rel in (
    "books/language-guide/01-getting-started.md",
    "books/zh-cn/language-guide/01-getting-started.md",
    "books/language-guide/07-functions.md",
    "books/zh-cn/language-guide/07-functions.md",
    "books/language-guide/18-concurrency.md",
    "books/zh-cn/language-guide/18-concurrency.md",
    "books/language-reference/03-expressions.md",
    "books/zh-cn/language-reference/03-expressions.md",
):
    t = Path(rel).read_text(encoding="utf-8")
    if "A002" not in t:
        raise SystemExit(f"{rel} missing A002")
    if "non-`async` function → compile error **S020**" in t:
        raise SystemExit(f"{rel} still await S020")
    if "非 `async` 函数使用 `await` → 编译错误 **S020**" in t:
        raise SystemExit(f"{rel} still await S020 zh")
    if "| **S020** | Using `await`" in t or "| **S020** | 非 `async` 函数使用 `await`" in t:
        raise SystemExit(f"{rel} still S020 await diag row")
for rel in (
    "books/language-guide/appendix-b-diagnostic-index.md",
    "books/zh-cn/language-guide/appendix-b-diagnostic-index.md",
):
    t = Path(rel).read_text(encoding="utf-8")
    if "S060" not in t:
        raise SystemExit(f"{rel} missing S060")
    if "non-`async` context" in t or "非 `async` 内 `await`" in t.split("| S020 |")[1][:80]:
        # S020 row must not still bag non-async await
        for line in t.splitlines():
            if line.startswith("| S020 |") and ("non-`async`" in line or "非 `async`" in line):
                raise SystemExit(f"{rel} S020 still bags non-async await")
print("u21_lang_nail=1")
print("u21_books_retarget=1")
PY

python3 - <<'PY' || die "census doc missing"
from pathlib import Path
census = Path("docs/census/u-21-s020-bag-md-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/u-21-s020-bag-md-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("U-21", "S060", "A002", "≠", "R-50", "JIT"):
    if needle not in ct:
        raise SystemExit(f"u-21 census missing {needle}")
print("u21_census_doc=1")
PY

SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

MUT=$(mktemp /tmp/u21_mut_XXXX.sy)
cat >"$MUT" <<'EOF'
public func main() {
    let a: Array<Int> = []
    a.append(1)
}
EOF
if "$SYX" check "$MUT" >/tmp/u21_mut_out.txt 2>&1; then
  die "mutating let must fail check"
fi
if ! grep -q 'S060' /tmp/u21_mut_out.txt; then
  cat /tmp/u21_mut_out.txt
  die "mutating let must report S060"
fi
if grep -q 'S020' /tmp/u21_mut_out.txt; then
  cat /tmp/u21_mut_out.txt
  die "mutating let must not report S020"
fi
ok "probe_mutating_s060" "1"
rm -f "$MUT"

AWAIT=$(mktemp /tmp/u21_await_XXXX.sy)
cat >"$AWAIT" <<'EOF'
func f() -> Int {
    return await g()
}
async func g() -> Int { return 1 }
public func main() { print(1) }
EOF
if "$SYX" check "$AWAIT" >/tmp/u21_await_out.txt 2>&1; then
  die "await outside async must fail check"
fi
if ! grep -q 'A002' /tmp/u21_await_out.txt; then
  cat /tmp/u21_await_out.txt
  die "await outside async must report A002"
fi
if grep -q 'S020' /tmp/u21_await_out.txt; then
  cat /tmp/u21_await_out.txt
  die "await outside async must not report S020"
fi
ok "probe_await_a002" "1"
rm -f "$AWAIT"

cargo test -p syx-driver --test lang s020_bag_honesty -- --test-threads=4 \
  >/tmp/u21_lang_nail.txt 2>&1 || {
  tail -40 /tmp/u21_lang_nail.txt
  die "lang s020_bag_honesty nail failed"
}
ok "lang_nail" "s020_bag_honesty"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/u21_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/u21_book_out.txt
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

python3 - <<'PY' || die "roadmap U-21 exit / next park missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if "U-21" not in text:
    raise SystemExit("roadmap missing U-21")
rows = [l for l in text.splitlines() if "| **U-21** |" in l]
if not rows or not any("[x]" in r for r in rows):
    raise SystemExit(f"U-21 not [x]: {rows}")
if "默认下一" in text:
    chunk = text.split("默认下一", 1)[1][:500]
    if "R-50" in chunk and "不" not in chunk and "park" not in chunk.lower() and "未编号" not in chunk:
        raise SystemExit("roadmap must not default next to R-50")
print("roadmap_u21_exit=1")
print("roadmap_no_r50_default=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "u21_s020_bag_md_census_ok=0"
  exit 1
fi
echo "u21_s020_bag_md_census_ok=1"
echo "current=U-21-s020-bag-md"
echo "next=park"
