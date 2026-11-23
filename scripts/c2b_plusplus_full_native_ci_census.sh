#!/usr/bin/env bash
# C-2b++ full-native CI census.
# Soft park: selective native=real raise (C-2b+++) · macOS LLVM · C-2c/d · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-2b++ full-native-ci census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "full-native-linux job contract broken"
from pathlib import Path
import re
ci = Path(".github/workflows/ci.yml").read_text(encoding="utf-8")
# Extract the full-native-linux job block until next top-level job key.
m = re.search(
    r"(?m)^  full-native-linux:\n(.*?)(?=\n  [a-zA-Z0-9_-]+:|\Z)",
    ci,
    re.S,
)
if not m:
    raise SystemExit("full-native-linux job missing")
block = m.group(0)
if "continue-on-error: true" in block:
    raise SystemExit("full-native-linux must not set continue-on-error: true")
if "scripts/ci_install_llvm.sh" not in block:
    raise SystemExit("full-native-linux must call ci_install_llvm.sh")
if "--features full-native" not in block:
    raise SystemExit("full-native-linux must build with --features full-native")
# Forbidden fallback on this job
if re.search(r"full-native[^\n]*\|\|[^\n]*--release", block):
    raise SystemExit("full-native-linux must not fall back to --release")
if "examples/hello.sy" not in block:
    raise SystemExit("full-native-linux must smoke hello.sy --native")
if "build --native" not in block and "build --native examples/hello.sy" not in block:
    # allow split across lines
    if "build --native" not in block.replace("\n", " "):
        raise SystemExit("full-native-linux must run syx build --native")
inst = Path("scripts/ci_install_llvm.sh")
if not inst.is_file():
    raise SystemExit("missing scripts/ci_install_llvm.sh")
it = inst.read_text(encoding="utf-8")
for needle in ("LLVM_VERSION", "LLVM_ASSET", "LLVM_SYS_211_PREFIX", "SYX_LLVM_PREFIX"):
    if needle not in it:
        raise SystemExit(f"ci_install_llvm.sh missing {needle}")
rel = Path(".github/workflows/release.yml").read_text(encoding="utf-8")
if "scripts/ci_install_llvm.sh" not in rel:
    raise SystemExit("release.yml must reuse ci_install_llvm.sh")
print("c2bpp_full_native_job=1")
print("c2bpp_no_continue_on_error=1")
print("c2bpp_install_script=1")
print("c2bpp_release_reuses_install=1")
PY

python3 - <<'PY' || die "product native=real re-raise detected"
from pathlib import Path
import re
caps = Path("packages/internal/capabilities.toml").read_text(encoding="utf-8")
hits = []
for i, line in enumerate(caps.splitlines(), 1):
    if re.search(r"\bnative\s*=\s*\"real\"", line):
        hits.append(f"{i}:{line.strip()}")
if hits:
    raise SystemExit("native=real present (C-2b++ must raise 0):\n" + "\n".join(hits[:20]))
print("c2bpp_native_real_count=0")
PY

python3 - <<'PY' || die "census/SSOT hygiene failed"
from pathlib import Path
census = Path("docs/census/c-2b-plusplus-full-native-ci-census.md")
if not census.is_file():
    raise SystemExit("missing c-2b-plusplus census")
ct = census.read_text(encoding="utf-8")
for needle in ("C-2b++", "full-native-linux", "C-2b+++", "hello", "LLVM"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")
ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
if "full-native-linux" not in ssot:
    raise SystemExit("SSOT must register full-native-linux")
if "C-2b+++" not in ssot:
    raise SystemExit("SSOT must point next C-2b+++")
if "paid floor" not in ssot.lower() and "Native paid floor" not in ssot:
    raise SystemExit("SSOT must list Native paid floor")
print("c2bpp_census_doc=1")
print("c2bpp_ssot_updated=1")
PY

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c2bpp_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c2bpp_book_out.txt
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

python3 - <<'PY' || die "roadmap C-2b++ exit / next C-2b+++ missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
ok_row = False
for l in text.splitlines():
    if "**C-2b++**" in l and "[x]" in l and "C-2b+++" not in l:
        ok_row = True
if not ok_row:
    raise SystemExit("C-2b++ not [x]")
if not any("**C-2b+++**" in l and "默认下一" in l for l in text.splitlines()) and "默认下一 = **C-2b+++**" not in text:
    raise SystemExit("need 默认下一 C-2b+++")
print("roadmap_c2bpp_exit=1")
print("roadmap_next_c2bppp=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "c2b_plusplus_full_native_ci_census_ok=0"
  exit 1
fi
echo "c2b_plusplus_full_native_ci_census_ok=1"
echo "current=C-2b++-full-native-ci"
echo "next=C-2b+++"
