#!/usr/bin/env bash
# C-2c JIT experimental label scrub census.
# Soft park: tls.server · macOS LLVM · C-2d job rename · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-2c jit-experimental-label census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "JIT public label contract broken"
from pathlib import Path
import re

fp = Path("docs/7_FeatureImplementationPriority.md").read_text(encoding="utf-8")
# §23 heading + body
m = re.search(r"### 23\. JIT compilation.*?(?=\n### |\Z)", fp, re.S)
if not m:
    raise SystemExit("FeaturePriority missing §23 JIT")
sec = m.group(0)
if "Stub" in sec and "experimental-real-subset" not in sec:
    raise SystemExit("FeaturePriority §23 still Stub without experimental-real-subset")
if "⚠️ Stub" in sec or "/ ⚠️ Stub" in sec:
    raise SystemExit("FeaturePriority §23 still product Stub badge")
for needle in ("experimental-real-subset", "default off", "Suite-1"):
    if needle not in sec:
        raise SystemExit(f"FeaturePriority §23 missing {needle}")

fp_zh = Path("docs/zh/7_FeatureImplementationPriority.zh-CN.md").read_text(encoding="utf-8")
mz = re.search(r"### 23\. JIT 编译.*?(?=\n### |\Z)", fp_zh, re.S)
if not mz:
    raise SystemExit("FeaturePriority ZH missing §23")
secz = mz.group(0)
if "⚠️ Stub" in secz or "/ ⚠️ Stub" in secz:
    raise SystemExit("FeaturePriority ZH §23 still Stub badge")
if "experimental-real-subset" not in secz:
    raise SystemExit("FeaturePriority ZH §23 missing experimental-real-subset")
if "默认关闭" not in secz:
    raise SystemExit("FeaturePriority ZH §23 missing 默认关闭")

env = Path("docs/env-registry.md").read_text(encoding="utf-8")
env_line = None
for ln in env.splitlines():
    if "SYX_JIT_TIER2" in ln:
        env_line = ln
        break
if not env_line:
    raise SystemExit("env-registry missing SYX_JIT_TIER2")
if "default on" in env_line.lower():
    raise SystemExit(f"env-registry SYX_JIT_TIER2 still default on: {env_line}")
if "default off" not in env_line.lower():
    raise SystemExit(f"env-registry SYX_JIT_TIER2 must say default off: {env_line}")

envz = Path("docs/zh/env-registry.zh-CN.md").read_text(encoding="utf-8")
envz_line = None
for ln in envz.splitlines():
    if "SYX_JIT_TIER2" in ln:
        envz_line = ln
        break
if not envz_line:
    raise SystemExit("env-registry ZH missing SYX_JIT_TIER2")
if "默认开启" in envz_line:
    raise SystemExit(f"env-registry ZH still 默认开启: {envz_line}")
if "默认关闭" not in envz_line:
    raise SystemExit(f"env-registry ZH must say 默认关闭: {envz_line}")

rep = Path("docs/6_ComprehensiveAnalysisReport.md").read_text(encoding="utf-8")
if "Hot-path counter stub" in rep:
    raise SystemExit("AnalysisReport still has Hot-path counter stub")
if "experimental-real-subset" not in rep or "Real JIT" not in rep:
    raise SystemExit("AnalysisReport must mention Real JIT experimental-real-subset")

print("c2c_featurepriority_ok=1")
print("c2c_env_registry_ok=1")
print("c2c_analysis_report_ok=1")
PY

python3 - <<'PY' || die "cap/CI/native count contract broken"
from pathlib import Path
import re

caps = Path("packages/internal/capabilities.toml").read_text(encoding="utf-8")
blocks = re.split(r"(?=^\[\[capability\]\])", caps, flags=re.M)
jit_ok = False
real_ids = []
tls_server_partial = False
want_real = {
    "io.path",
    "io.file.read_all_bytes",
    "net.tcp",
    "net.tcp.listener",
    "data.sqlite",
    "net.http",
    "data.sqlite.pool",
    "data.sqlite.row",
    "net.http.types",
    "net.http.server",
    "net.http.stream",
    "net.http.timeout_tls",
    "io.file.raii",
    "io.file.stream",
    "io.file.watch",
    "net.tls.client",
}
for b in blocks:
    if not b.startswith("[[capability]]"):
        continue
    m = re.search(r'^id\s*=\s*"([^"]+)"', b, re.M)
    if not m:
        continue
    cid = m.group(1)
    if cid == "runtime.jit.tier2":
        if 'status = "experimental-real-subset"' not in b:
            raise SystemExit("runtime.jit.tier2 status must stay experimental-real-subset")
        if "default off" not in b:
            raise SystemExit("runtime.jit.tier2 notes must keep default off")
        jit_ok = True
    nm = re.search(r'backends\s*=\s*\{[^}]*\bnative\s*=\s*"([^"]+)"', b, re.S)
    if nm and nm.group(1) == "real":
        real_ids.append(cid)
    if cid == "net.tls.server":
        if not nm or nm.group(1) != "partial":
            raise SystemExit("net.tls.server must stay native=partial")
        tls_server_partial = True
if not jit_ok:
    raise SystemExit("runtime.jit.tier2 missing")
if set(real_ids) != want_real:
    raise SystemExit(f"native=real must stay exactly 16 product ids; got {sorted(real_ids)}")
if not tls_server_partial:
    raise SystemExit("net.tls.server block missing")

ci = Path(".github/workflows/ci.yml").read_text(encoding="utf-8")
jm = re.search(r"(?m)^  jit:\n(.*?)(?=\n  [a-zA-Z0-9_-]+:|\Z)", ci, re.S)
if not jm:
    raise SystemExit("ci.yml missing jit job")
jblock = jm.group(0)
if "continue-on-error: true" not in jblock:
    raise SystemExit("jit job must stay continue-on-error: true")
print("c2c_jit_cap_ok=1")
print("c2c_jit_ci_soft=1")
print("c2c_native_real_count=16")
print("c2c_tls_server_partial=1")
PY

python3 - <<'PY' || die "census/SSOT hygiene failed"
from pathlib import Path
import re as _re
census = Path("docs/census/c-2c-jit-experimental-label-census.md")
if not census.is_file():
    raise SystemExit("missing c-2c census")
ct = census.read_text(encoding="utf-8")
for needle in ("C-2c", "experimental-real-subset", "C-2d", "default off", "Stub", "SYX_JIT_TIER2"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")
ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
if "C-2c (done" not in ssot and "### C-2c (done" not in ssot:
    raise SystemExit("SSOT must mark C-2c done")
next_sec = ssot.split("## Next", 1)[-1]
if "C-2d" not in next_sec:
    raise SystemExit("SSOT Next must be C-2d")
if "+++" in next_sec:
    raise SystemExit("SSOT Next must not use +++")
rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("**C-2c**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-2c [x]")
if not any("**C-2d**" in l and "默认下一" in l for l in rm.splitlines()) and "默认下一 = **C-2d**" not in rm:
    raise SystemExit("02 must set 默认下一 C-2d")
hits = []
for path in Path("crates").rglob("*.rs"):
    text = path.read_text(encoding="utf-8", errors="ignore")
    code = "\n".join(
        ln for ln in text.splitlines() if not ln.lstrip().startswith("//")
    )
    if _re.search(r"\brun_native_strict_or_mirror_vm\b", code):
        hits.append(str(path))
if hits:
    raise SystemExit(f"mirror helper rebound: {hits}")
print("c2c_census_doc=1")
print("c2c_ssot_updated=1")
print("c2c_roadmap_ok=1")
print("c2c_no_mirror=1")
PY

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c2c_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c2c_book_out.txt
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

if [[ "$fail" -ne 0 ]]; then
  echo "c2c_jit_experimental_label_census_ok=0"
  exit 1
fi
echo "c2c_jit_experimental_label_census_ok=1"
echo "current=C-2c-jit-experimental-label"
echo "next=C-2d"
