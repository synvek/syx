#!/usr/bin/env bash
# R-48 credibility MD-only claim diet census.
# Soft park: D3 census merge · R-49 docs · JIT · func…async.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-48 credibility diet census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

CRED=crates/syx-driver/tests/credibility
MAIN="$CRED/main.rs"

python3 - <<'PY' || die "knife modules / mod residual / delete count failed"
from pathlib import Path

cred = Path("crates/syx-driver/tests/credibility")
main = (cred / "main.rs").read_text(encoding="utf-8")

removed = [
    "sequence_collection_census_contract",
    "sequence_collection_claim_contract",
    "sequence_collection_residual_scope_contract",
    "stdlib_cache_census_contract",
    "stdlib_cache_claim_contract",
    "structured_concurrency_census_contract",
    "structured_concurrency_claim_contract",
    "structured_concurrency_residual_scope_contract",
]
for name in removed:
    path = cred / f"{name}.rs"
    if path.exists():
        raise SystemExit(f"still present: {path}")
    if f"mod {name};" in main:
        raise SystemExit(f"main.rs still mods {name}")
print(f"md_only_modules_deleted={len(removed)}")
if len(removed) < 8:
    raise SystemExit("delete count must be ≥8")
print("knife_mod_residual=0")

# D2 executable nails exist
for p in (
    Path("crates/syx-driver/tests/lang/protocol_iterator.rs"),
    Path("docs/census/r-33-iterator-witness-census.md"),
    Path("scripts/r40_coldstart_census.sh"),
    Path("docs/census/r-40-coldstart-census.md"),
):
    if not p.is_file():
        raise SystemExit(f"D2 nail missing: {p}")
print("d2_s4_04_nail=protocol_iterator+r33")
print("d2_s3_03_nail=r40_coldstart")
PY

python3 - <<'PY' || die "census doc hygiene / D2 bind failed"
from pathlib import Path
census = Path("docs/census/r-48-credibility-diet-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/r-48-credibility-diet-census.md")
ct = census.read_text(encoding="utf-8")
for needle in (
    "R-48",
    "S4-04",
    "S3-03",
    "protocol_iterator",
    "r-40-coldstart",
    "r-33",
    "≠",
    "R-49",
    "8",
):
    if needle not in ct:
        raise SystemExit(f"r-48 census missing {needle}")
print("r48_census_doc=1")
print("d2_bind_documented=1")
PY

# credibility harness still builds (filter a cheap kept module if possible)
cargo test -p syx-driver --test credibility system_import_smoke -- --test-threads=8 >/tmp/r48_cred.txt 2>&1 || {
  tail -40 /tmp/r48_cred.txt
  die "credibility test binary failed"
}
ok "credibility_smoke" "system_import_smoke"
grep -E "^test result:" /tmp/r48_cred.txt | tail -1 || true

SYX=./target/debug/syx
if [[ ! -x "$SYX" ]]; then
  cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
fi

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/r48_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/r48_book_out.txt
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

python3 - <<'PY' || die "roadmap R-48 exit / R-49 default missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
rows = [l for l in text.splitlines() if "| **R-48** |" in l]
if not rows or not any("`[x]`" in r or "| [x] |" in r for r in rows):
    raise SystemExit(f"R-48 not [x]: {rows}")
tail = text.split("默认下一窗", 1)
if len(tail) < 2 or "R-49" not in tail[1][:240]:
    raise SystemExit("roadmap must default next window to R-49")
print("roadmap_r48_exit=1")
print("roadmap_next_r49=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "r48_credibility_diet_census_ok=0"
  exit 1
fi
echo "r48_credibility_diet_census_ok=1"
echo "current=R-48-credibility-diet"
echo "next=R-49-docs-adr"
