#!/usr/bin/env bash
# C-2a Backend/CI matrix SSOT census.
# Soft park: full-native hard gate (C-2b) · JIT label scrub (C-2c) · job rename (C-2d) · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-2a backend-ci-matrix census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "C-2a SSOT surface missing"
from pathlib import Path
ssot = Path("docs/gates/backend-ci-matrix.md")
if not ssot.is_file():
    raise SystemExit("missing docs/gates/backend-ci-matrix.md")
text = ssot.read_text(encoding="utf-8")
for needle in (
    "default_hard",
    "soft_non_default",
    "experimental",
    "C-2a",
    "46/46",
    "browser",
    "native-linux",
    "native-macos",
    "L006",
    "non_default_kpi",
    "jit",
):
    if needle not in text and needle.lower() not in text.lower():
        # case-sensitive for structural tokens
        if needle not in text:
            raise SystemExit(f"SSOT missing {needle!r}")
# Explicit hard floor wording
if "Wasm" not in text and "wasm" not in text:
    raise SystemExit("SSOT must mention Wasm")
if "VM" not in text or "MIR" not in text:
    raise SystemExit("SSOT must name VM and MIR")
# Ban conflation
banned = (
    "L006 skip = native supported",
    "L006 skip=native supported",
    "soft native job green = native supported",
)
low = text.lower()
for b in banned:
    if b.lower() in low:
        raise SystemExit(f"SSOT must not claim {b!r}")
if "l006 skip / soft native job green ≠ native supported" not in low and \
   "l006 skip ≠ native supported" not in low and \
   "≠ native supported" not in low:
    raise SystemExit("SSOT must state L006/soft ≠ native supported")
print("c2a_ssot=1")
print("c2a_default_hard_named=1")
print("c2a_no_claim_l006_is_native_green=1")
PY

python3 - <<'PY' || die "ci.yml soft jobs not registered"
from pathlib import Path
ci = Path(".github/workflows/ci.yml").read_text(encoding="utf-8")
ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
for job in ("native-linux", "native-macos", "browser-e2e", "jit"):
    if f"{job}:" not in ci and f"  {job}:" not in ci:
        # yaml may indent as "  native-linux:"
        if job not in ci:
            raise SystemExit(f"ci.yml missing job {job}")
    if job not in ssot:
        raise SystemExit(f"SSOT must list soft job {job}")
# Soft jobs should still be continue-on-error in ci.yml (C-2a registers; C-2b hardens)
for marker in ("continue-on-error: true",):
    if marker not in ci:
        raise SystemExit("ci.yml expected continue-on-error soft walls")
print("c2a_ci_soft_native_listed=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/c-2a-backend-ci-matrix-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/c-2a-backend-ci-matrix-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("C-2a", "default_hard", "≠", "C-2b", "L006"):
    if needle not in ct:
        raise SystemExit(f"c-2a census missing {needle}")
print("c2a_census_doc=1")
PY

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c2a_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c2a_book_out.txt
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

python3 - <<'PY' || die "roadmap C-2a exit / next C-2b missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("C-2a" in l and "[x]" in l for l in text.splitlines()):
    raise SystemExit("C-2a not [x]")
if "默认下一 = **C-2b**" not in text:
    raise SystemExit("need 默认下一 = **C-2b**")
if "backend-ci-matrix.md" not in text:
    raise SystemExit("roadmap must link SSOT")
print("roadmap_c2a_exit=1")
print("roadmap_next_c2b=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "c2a_backend_ci_matrix_census_ok=0"
  exit 1
fi
echo "c2a_backend_ci_matrix_census_ok=1"
echo "current=C-2a-backend-ci-matrix"
echo "next=C-2b"
