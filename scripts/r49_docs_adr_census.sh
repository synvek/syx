#!/usr/bin/env bash
# R-49 docs/ADR hygiene census.
# Soft park: full ADR merge · 724-link wipe · JIT · func…async.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-49 docs ADR census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "version / shortlist gate failed"
from pathlib import Path
import re

for p in (Path("docs/README.md"), Path("docs/zh/README.zh-CN.md"), Path("books/README.md")):
    t = p.read_text(encoding="utf-8")
    if "0.2.0" not in t:
        raise SystemExit(f"{p}: missing 0.2.0")
    # forbid current-banner lies (allow quoting obsolete in architecture-decisions only)
    if "117.0" in t or "Phase 139" in t or "Phase **139**" in t:
        raise SystemExit(f"{p}: still contains 117.0 / Phase 139")
    print(f"version_ok={p.as_posix()}")

cur = Path("docs/adr/current.md")
if not cur.is_file():
    raise SystemExit("missing docs/adr/current.md")
ct = cur.read_text(encoding="utf-8")
rows = re.findall(r"^\| \d+ \|", ct, re.M)
print(f"current_shortlist_rows={len(rows)}")
if not (10 <= len(rows) <= 25):
    raise SystemExit("current shortlist must be 10–25 rows")
if "architecture-decisions.md" not in ct:
    raise SystemExit("current.md must point at historical ledger")
adr_readme = Path("docs/adr/README.md").read_text(encoding="utf-8")
if "current.md" not in adr_readme:
    raise SystemExit("adr/README must link current.md")
print("adr_shortlist_ok=1")

mega = Path("docs/adr/architecture-decisions.md").read_text(encoding="utf-8")
if "Historical ledger" not in mega and "R-49" not in mega[:800]:
    raise SystemExit("architecture-decisions.md missing R-49 historical honesty banner")
print("mega_adr_honesty=1")
PY

python3 scripts/r49_docs_link_check.py || die "scoped link check failed"

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/r-49-docs-adr-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/r-49-docs-adr-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("R-49", "0.2.0", "current.md", "scoped", "≠", "15", "JIT"):
    if needle not in ct:
        raise SystemExit(f"r-49 census missing {needle}")
print("r49_census_doc=1")
PY

SYX=./target/debug/syx
if [[ ! -x "$SYX" ]]; then
  cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
fi

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/r49_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/r49_book_out.txt
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

python3 - <<'PY' || die "roadmap R-49 exit / ledger close missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
rows = [l for l in text.splitlines() if "| **R-49** |" in l]
if not rows or not any("`[x]`" in r for r in rows):
    raise SystemExit(f"R-49 not [x]: {rows}")
if "编号账本已收口" not in text and "编号账本（SA-0…R-49）收口" not in text:
    raise SystemExit("roadmap must state numbered ledger closed")
if "**默认下一窗：`R-50`" in text or "默认下一窗：`R-50`" in text:
    raise SystemExit("must not open R-50 as default next window")
print("roadmap_r49_exit=1")
print("roadmap_ledger_closed=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "r49_docs_adr_census_ok=0"
  exit 1
fi
echo "r49_docs_adr_census_ok=1"
echo "current=R-49-docs-adr"
echo "next=numbered-ledger-closed"
