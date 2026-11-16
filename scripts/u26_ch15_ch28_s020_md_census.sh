#!/usr/bin/env bash
# U-26 Ch15/Ch28 S020 MD census (book soften; ≠ tree init/access diags; ≠ S060 retarget; ≠ R-50).
# Soft park: tuple .0= S060 · self.x= · comma while-let · closure throws · CST · Break · JIT · SyxHost · D3.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== U-26 Ch15/Ch28 S020 MD census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "books/nail U-26 surface missing"
from pathlib import Path
for rel in (
    "books/language-guide/15-initialization.md",
    "books/zh-cn/language-guide/15-initialization.md",
    "books/language-guide/28-access-control.md",
    "books/zh-cn/language-guide/28-access-control.md",
):
    t = Path(rel).read_text(encoding="utf-8")
    if "not yet diagnosed" not in t and "尚未诊断" not in t:
        raise SystemExit(f"{rel} missing soften phrase")
    # Must not claim these rules as a hard S020 trigger row/callout.
    for bad in (
        "→ **S020** compile error",
        "→ **S020** 编译错误",
        "| **S020** | Reading `self` before `super.init`",
        "| **S020** | `super.init` 之前读 `self`",
        "| **S020** | A member's access level higher than the type's",
        "| **S020** | 成员访问级别高于类型",
    ):
        if bad in t:
            raise SystemExit(f"{rel} still has false S020 claim: {bad!r}")
    if "≠ **S020**" not in t and "≠ **S020**" not in t.replace(" ", ""):
        if "≠ **S020**" not in t:
            raise SystemExit(f"{rel} missing ≠ S020 honesty")
app = Path("books/language-guide/appendix-b-diagnostic-index.md").read_text(encoding="utf-8")
if "MainActor" not in app or "S020" not in app:
    raise SystemExit("appendix-B must keep S020=MainActor")
nail = Path("crates/syx-driver/tests/lang/ch15_ch28_s020_honesty.rs")
if not nail.is_file():
    raise SystemExit("missing lang/ch15_ch28_s020_honesty.rs")
nt = nail.read_text(encoding="utf-8")
for needle in (
    "u26_init_order_not_s020",
    "u26_access_level_not_s020",
    "u26_mutating_still_s060",
    "u26_await_still_a002",
):
    if needle not in nt:
        raise SystemExit(f"ch15_ch28_s020_honesty missing {needle}")
main = Path("crates/syx-driver/tests/lang/main.rs").read_text(encoding="utf-8")
if "mod ch15_ch28_s020_honesty;" not in main:
    raise SystemExit("lang/main.rs missing mod ch15_ch28_s020_honesty")
print("u26_books_soften=1")
print("u26_lang_nail=1")
PY

python3 - <<'PY' || die "census doc missing"
from pathlib import Path
census = Path("docs/census/u-26-ch15-ch28-s020-md-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/u-26-ch15-ch28-s020-md-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("U-26", "S020", "Ch15", "Ch28", "≠", "R-50", "JIT"):
    if needle not in ct:
        raise SystemExit(f"u-26 census missing {needle}")
print("u26_census_doc=1")
PY

SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

INIT=$(mktemp /tmp/u26_init_XXXX.sy)
cat >"$INIT" <<'EOF'
class Base {
    var n: Int
    init(n: Int) { self.n = n }
}
class Child: Base {
    var m: Int
    init(n: Int, m: Int) {
        print(self.n)
        self.m = m
        super.init(n: n)
    }
}
public func main() {
    let _ = Child(n: 1, m: 2)
}
EOF
if ! "$SYX" check "$INIT" >/tmp/u26_init_out.txt 2>&1; then
  cat /tmp/u26_init_out.txt
  die "init-order probe must check (no diag yet)"
fi
if grep -q 'S020' /tmp/u26_init_out.txt; then
  cat /tmp/u26_init_out.txt
  die "init-order probe must not emit S020"
fi
ok "probe_init_no_s020" "1"
rm -f "$INIT"

ACC=$(mktemp /tmp/u26_acc_XXXX.sy)
cat >"$ACC" <<'EOF'
internal class Box {
    public var n: Int
    init(n: Int) { self.n = n }
}
public func main() {
    let b = Box(n: 1)
    print(b.n)
}
EOF
if ! "$SYX" check "$ACC" >/tmp/u26_acc_out.txt 2>&1; then
  cat /tmp/u26_acc_out.txt
  die "access-level probe must check (no diag yet)"
fi
if grep -q 'S020' /tmp/u26_acc_out.txt; then
  cat /tmp/u26_acc_out.txt
  die "access-level probe must not emit S020"
fi
ok "probe_access_no_s020" "1"
rm -f "$ACC"

MUT=$(mktemp /tmp/u26_mut_XXXX.sy)
cat >"$MUT" <<'EOF'
public func main() {
    let a: Array<Int> = []
    a.append(1)
}
EOF
if "$SYX" check "$MUT" >/tmp/u26_mut_out.txt 2>&1; then
  die "mutating let must fail check"
fi
if ! grep -q 'S060' /tmp/u26_mut_out.txt; then
  cat /tmp/u26_mut_out.txt
  die "mutating let must still be S060"
fi
ok "probe_mutating_s060" "1"
rm -f "$MUT"

cargo test -p syx-driver --test lang ch15_ch28_s020_honesty -- --test-threads=4 \
  >/tmp/u26_lang_nail.txt 2>&1 || {
  tail -40 /tmp/u26_lang_nail.txt
  die "lang ch15_ch28_s020_honesty nail failed"
}
ok "lang_nail" "ch15_ch28_s020_honesty"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/u26_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/u26_book_out.txt
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

python3 - <<'PY' || die "roadmap U-26 exit / next park missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if "U-26" not in text:
    raise SystemExit("roadmap missing U-26")
rows = [l for l in text.splitlines() if "| **U-26** |" in l]
if not rows or not any("[x]" in r for r in rows):
    raise SystemExit(f"U-26 not [x]: {rows}")
if "默认下一" in text:
    chunk = text.split("默认下一", 1)[1][:500]
    if "R-50" in chunk and "不" not in chunk and "park" not in chunk.lower() and "未编号" not in chunk:
        raise SystemExit("roadmap must not default next to R-50")
print("roadmap_u26_exit=1")
print("roadmap_no_r50_default=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "u26_ch15_ch28_s020_md_census_ok=0"
  exit 1
fi
echo "u26_ch15_ch28_s020_md_census_ok=1"
echo "current=U-26-ch15-ch28-s020-md"
echo "next=park"
