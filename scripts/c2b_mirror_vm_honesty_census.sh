#!/usr/bin/env bash
# C-2b mirror-VM honesty census.
# Soft park: demote native=real · soft_native exit0 · CI LLVM (C-2b+) · C-2c/d · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-2b mirror-vm-honesty census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "mirror_vm still live"
from pathlib import Path
import re
# Only Rust sources: live def / use / call (docs may still name the symbol).
live_pat = re.compile(r"\b(run_native_strict_or_mirror_vm|mirror_vm)\b")
hits = []
for path in Path("crates").rglob("*.rs"):
    rel = path.as_posix()
    if "/target/" in rel:
        continue
    text = path.read_text(encoding="utf-8")
    for i, line in enumerate(text.splitlines(), 1):
        if line.strip().startswith("//"):
            continue
        if live_pat.search(line):
            hits.append(f"{rel}:{i}:{line.strip()[:120]}")
if hits:
    raise SystemExit("live mirror_vm refs in .rs:\n" + "\n".join(hits[:40]))
ns = Path("crates/syx-driver/tests/backends/native_strict.rs").read_text(encoding="utf-8")
if "pub fn try_run_native_strict" not in ns:
    raise SystemExit("try_run_native_strict missing")
if "pub fn run_native_strict_or_mirror_vm" in ns:
    raise SystemExit("helper still defined")
print("c2b_mirror_vm_gone=1")
print("c2b_try_run_present=1")
PY

python3 - <<'PY' || die "census/SSOT hygiene failed"
from pathlib import Path
census = Path("docs/census/c-2b-mirror-vm-honesty-census.md")
if not census.is_file():
    raise SystemExit("missing c-2b census")
ct = census.read_text(encoding="utf-8")
for needle in ("C-2b", "mirror", "≠", "C-2b+", "try_run_native_strict"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")
ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
if "C-2b exited" not in ssot and "forbidden" not in ssot.lower():
    raise SystemExit("SSOT must mark mirror forbidden/exited")
if "C-2b+" not in ssot:
    raise SystemExit("SSOT must point next C-2b+")
print("c2b_census_doc=1")
print("c2b_ssot_updated=1")
PY

cargo test -p syx-driver --test backends http_native_wasm_ga -- --test-threads=1 \
  >/tmp/c2b_nails.txt 2>&1 || {
  tail -40 /tmp/c2b_nails.txt
  die "backends http_ga compile/run failed"
}
ok "c2b_backends_http_ga" "1"

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c2b_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c2b_book_out.txt
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

python3 - <<'PY' || die "roadmap C-2b exit / next C-2b+ missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("C-2b" in l and "[x]" in l and "C-2b+" not in l for l in text.splitlines()):
    # Allow line with both C-2b [x] without +
    if not any(
        "C-2b" in l and "[x]" in l and "C-2b+" not in l.split("|")[1] if "|" in l else ("C-2b" in l and "[x]" in l and "b+" not in l)
        for l in text.splitlines()
    ):
        pass
ok_c2b = False
for l in text.splitlines():
    if "**C-2b**" in l and "[x]" in l and "C-2b+" not in l:
        ok_c2b = True
if not ok_c2b:
    raise SystemExit("C-2b not [x]")
if "默认下一 = **C-2b+**" not in text:
    raise SystemExit("need 默认下一 = **C-2b+**")
print("roadmap_c2b_exit=1")
print("roadmap_next_c2b_plus=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "c2b_mirror_vm_honesty_census_ok=0"
  exit 1
fi
echo "c2b_mirror_vm_honesty_census_ok=1"
echo "current=C-2b-mirror-vm-honesty"
echo "next=C-2b+"
