#!/usr/bin/env bash
# U-2 credibility Stage-3 pure-MD claim diet census (U-1 follow-on).
# Soft park: D3 · JIT · func…async · SyxHost · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== U-2 credibility diet census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "knife modules / mod residual / retain failed"
from pathlib import Path

cred = Path("crates/syx-driver/tests/credibility")
main = (cred / "main.rs").read_text(encoding="utf-8")

removed = [
    "init_template_census_contract",
    "init_template_claim_contract",
    "init_template_residual_scope_contract",
    "lint_ast_census_contract",
    "lint_ast_claim_contract",
    "lint_ast_residual_scope_contract",
    "vm_hotspot_census_contract",
    "vm_hotspot_claim_contract",
    "vm_hotspot_residual_scope_contract",
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

for name in (
    "lint_ast_claim_prep_contract",
    "vm_hotspot_claim_prep_contract",
    "cst_lower_probe_contract",
):
    path = cred / f"{name}.rs"
    if not path.is_file():
        raise SystemExit(f"must retain: {path}")
    if f"mod {name};" not in main:
        raise SystemExit(f"main.rs must still mod {name}")
print("prep_and_cst_lower_retained=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/u-2-credibility-diet-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/u-2-credibility-diet-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("U-2", "9", "init_template", "lint_ast", "vm_hotspot", "≠", "R-50", "JIT", "D3"):
    if needle not in ct:
        raise SystemExit(f"u-2 census missing {needle}")
print("u2_census_doc=1")
PY

cargo test -p syx-driver --test credibility system_import_smoke -- --test-threads=8 >/tmp/u2_cred.txt 2>&1 || {
  tail -40 /tmp/u2_cred.txt
  die "credibility test binary failed"
}
ok "credibility_smoke" "system_import_smoke"
grep -E "^test result:" /tmp/u2_cred.txt | tail -1 || true

SYX=./target/debug/syx
if [[ ! -x "$SYX" ]]; then
  cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
fi

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/u2_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/u2_book_out.txt
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

python3 - <<'PY' || die "roadmap U-2 exit / next park missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if "U-2" not in text:
    raise SystemExit("roadmap missing U-2")
rows = [l for l in text.splitlines() if "| **U-2** |" in l]
if not rows or not any("[x]" in r for r in rows):
    raise SystemExit(f"U-2 not [x]: {rows}")
if "默认下一" in text:
    chunk = text.split("默认下一", 1)[1][:400]
    if "R-50" in chunk and "不" not in chunk and "park" not in chunk.lower() and "未编号" not in chunk:
        raise SystemExit("roadmap must not default next to R-50")
print("roadmap_u2_exit=1")
print("roadmap_no_r50_default=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "u2_credibility_diet_census_ok=0"
  exit 1
fi
echo "u2_credibility_diet_census_ok=1"
echo "current=U-2-credibility-diet"
echo "next=U-3-or-park"
