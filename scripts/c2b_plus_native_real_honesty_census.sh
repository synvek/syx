#!/usr/bin/env bash
# C-2b+ native=real honesty census.
# Soft park: CI LLVM / required full-native / re-raise native=real (C-2b++) · C-2c/d · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-2b+ native-real-honesty census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "unpaid native=real still present"
from pathlib import Path
import re
caps = Path("packages/internal/capabilities.toml").read_text(encoding="utf-8")
# Product backends.native = "real" must be 0 this window.
hits = []
for i, line in enumerate(caps.splitlines(), 1):
    if re.search(r"\bnative\s*=\s*\"real\"", line):
        hits.append(f"capabilities.toml:{i}:{line.strip()}")
if hits:
    raise SystemExit("native=real still live:\n" + "\n".join(hits[:40]))
gen = Path("packages/internal/capabilities.generated.json").read_text(encoding="utf-8")
if '"native": "real"' in gen:
    raise SystemExit("generated.json still has native real")
# Notes should mark demote on at least one demoted row
if "C-2b+: unpaid ≠ full-native GA" not in caps:
    raise SystemExit("missing C-2b+ demote notes")
print("c2bp_native_real_count=0")
print("c2bp_demote_notes=1")
PY

python3 - <<'PY' || die "ci.yml still soft-skips L006 to success"
from pathlib import Path
import re
ci = Path(".github/workflows/ci.yml").read_text(encoding="utf-8")
# Forbidden: after L006|full-native match, return 0 / exit 0 in soft_native / macos native link.
# Detect classic soft-skip pattern (L006 branch then return/exit 0).
blocks = []
# native-linux soft_native function
m = re.search(r"soft_native\(\)\s*\{(.*?)\n          \}", ci, re.S)
if not m:
    raise SystemExit("soft_native() missing")
body = m.group(1)
if re.search(r"L006\|full-native[\s\S]{0,400}?return 0", body):
    raise SystemExit("soft_native still return 0 on L006")
if "return 1" not in body:
    raise SystemExit("soft_native must return 1 on L006 path")
# macos: L006 branch must exit 1
mac = re.search(r"name: Native link hello project[\s\S]*?run: \|([\s\S]*?)(?=\n  [a-z]|\n  #|\Z)", ci)
if not mac:
    # fallback: search L006 block near native-macos
    mac_body = ci
else:
    mac_body = mac.group(1)
# Find L006 branch that exits 0 (forbidden)
for m in re.finditer(
    r"if grep -Eq 'L006\|full-native' \"\$err\"; then\n(.*?)(?:fi)",
    ci,
    re.S,
):
    blk = m.group(1)
    if "exit 0" in blk or "return 0" in blk:
        raise SystemExit("L006 branch still exits/returns 0:\n" + blk[:200])
    if "exit 1" not in blk and "return 1" not in blk:
        raise SystemExit("L006 branch must exit/return 1:\n" + blk[:200])
if "continue-on-error: true" not in ci:
    raise SystemExit("native jobs should keep continue-on-error")
print("c2bp_l006_no_soft_success=1")
print("c2bp_continue_on_error_kept=1")
PY

python3 - <<'PY' || die "census/SSOT hygiene failed"
from pathlib import Path
census = Path("docs/census/c-2b-plus-native-real-honesty-census.md")
if not census.is_file():
    raise SystemExit("missing c-2b-plus census")
ct = census.read_text(encoding="utf-8")
for needle in ("C-2b+", "native=real", "L006", "C-2b++", "partial"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")
ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
if "C-2b+" not in ssot:
    raise SystemExit("SSOT must mention C-2b+")
if "C-2b++" not in ssot:
    raise SystemExit("SSOT must point next C-2b++")
if "L006 → **step fail**" not in ssot and "L006 → **exit 1**" not in ssot:
    raise SystemExit("SSOT must mark L006 step fail")
print("c2bp_census_doc=1")
print("c2bp_ssot_updated=1")
PY

# Smoke: capability-locked stdlib nails still compile/pass after demote
cargo test -p syx-driver --test stdlib website_capability_deepen -- --test-threads=1 \
  >/tmp/c2bp_nails.txt 2>&1 || {
  tail -40 /tmp/c2bp_nails.txt
  die "website_capability_deepen failed"
}
ok "c2bp_website_cap_nail" "1"

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c2bp_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c2bp_book_out.txt
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

python3 - <<'PY' || die "roadmap C-2b+ exit / next C-2b++ missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
ok_c2bp = False
for l in text.splitlines():
    if "**C-2b+**" in l and "[x]" in l and "C-2b++" not in l:
        ok_c2bp = True
if not ok_c2bp:
    raise SystemExit("C-2b+ not [x]")
if "默认下一 = **C-2b++**" not in text and "**C-2b++**" not in text:
    raise SystemExit("need C-2b++ as next")
# Prefer explicit 默认下一 header or table row marked 默认下一
if "默认下一 = **C-2b++**" not in text and not any(
    "**C-2b++**" in l and "默认下一" in l for l in text.splitlines()
):
    raise SystemExit("need 默认下一 = **C-2b++** or table 默认下一")
print("roadmap_c2bp_exit=1")
print("roadmap_next_c2bpp=1")
PY

# Mirror must not regress
python3 - <<'PY' || die "mirror_vm regress"
from pathlib import Path
import re
live_pat = re.compile(r"\b(run_native_strict_or_mirror_vm|mirror_vm)\b")
hits = []
for path in Path("crates").rglob("*.rs"):
    if "/target/" in path.as_posix():
        continue
    for i, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        if line.strip().startswith("//"):
            continue
        if live_pat.search(line):
            hits.append(f"{path}:{i}")
if hits:
    raise SystemExit("mirror_vm regress:\n" + "\n".join(hits[:20]))
print("c2bp_mirror_still_gone=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "c2b_plus_native_real_honesty_census_ok=0"
  exit 1
fi
echo "c2b_plus_native_real_honesty_census_ok=1"
echo "current=C-2b+-native-real-honesty"
echo "next=C-2b++"
