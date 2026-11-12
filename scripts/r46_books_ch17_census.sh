#!/usr/bin/env bash
# R-46 books Ch17 Error surface census.
# Hard gates: Ch17 anti-pattern residual 0 · en↔zh fence sync · sy,output 12/12
# Soft park: func…async parser · JIT. KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-46 books Ch17 Error surface census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 scripts/r46_ch17_fence_sync.py || die "Ch17 fence sync / anti-pattern residual"

# SEM-THROW pin still present
python3 - <<'PY' || die "SEM-THROW output pin missing"
from pathlib import Path
for p in (
    Path("books/language-guide/17-error-handling.md"),
    Path("books/zh-cn/language-guide/17-error-handling.md"),
):
    t = p.read_text(encoding="utf-8")
    if "syx,output" not in t or "prints: caught" not in t:
        raise SystemExit(f"missing SEM-THROW pin in {p}")
print("sem_throw_pin=1")
PY

SYX=./target/debug/syx
if [[ ! -x "$SYX" ]]; then
  cargo build -p syx-driver --bin syx >/dev/null 2>&1 || die "build syx failed"
fi

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/r46_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -25 /tmp/r46_book_out.txt
python3 - <<PY || die "sy,output must be 12/12 (en+zh)"
import json, sys
from pathlib import Path
d = json.loads(Path("$out_json").read_text())
ok_n = d.get("output_run_ok", 0)
tot = d.get("output_run_total", 0)
print(f"output_run_ok={ok_n}")
print(f"output_run_total={tot}")
en = [r for r in d["results"] if "zh-cn" not in r["file"]]
zh = [r for r in d["results"] if "zh-cn" in r["file"]]
print(f"output_en={len(en)} output_zh={len(zh)}")
sys.exit(0 if ok_n == 12 and tot == 12 and len(en) == 6 and len(zh) == 6 else 1)
PY
rm -f "$out_json"

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/r-46-books-ch17-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/r-46-books-ch17-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("R-46", "Ch17", "Error", "0", "12/12", "≠", "R-47"):
    if needle not in ct:
        raise SystemExit(f"r-46 census missing {needle}")
print("r46_census_doc=1")
PY

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

python3 - <<'PY' || die "roadmap R-46 exit / R-47 default missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
rows = [l for l in text.splitlines() if "| **R-46** |" in l]
if not rows or not any("`[x]`" in r or "| [x] |" in r for r in rows):
    raise SystemExit(f"R-46 not [x]: {rows}")
tail = text.split("默认下一窗", 1)
if len(tail) < 2 or "R-47" not in tail[1][:200]:
    raise SystemExit("roadmap must default next window to R-47")
print("roadmap_r46_exit=1")
print("roadmap_next_r47=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "r46_books_ch17_census_ok=0"
  exit 1
fi
echo "r46_books_ch17_census_ok=1"
echo "current=R-46-books-ch17"
echo "next=R-47-package-api"
