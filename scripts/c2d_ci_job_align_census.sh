#!/usr/bin/env bash
# C-2d ci.job.* surface ↔ yaml job align census.
# Soft park: tls.server · macOS full-native · R-37/R-39 fossil scripts · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-2d ci-job-align census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "ci.job.* surface align contract broken"
from pathlib import Path
import re

caps = Path("packages/internal/capabilities.toml").read_text(encoding="utf-8")
blocks = re.split(r"(?=^\[\[capability\]\])", caps, flags=re.M)

# capability id -> required yaml job substring that MUST appear in surface
want = {
    "ci.job.native-strict-main": "native-linux",
    "ci.job.native-macos": "native-macos",
    "ci.job.parity-gate": "extended-suites",
    "ci.job.jit-tier2-default": "job jit",
    "ci.job.phase125-gui-xvfb": "gui-xvfb",
    "ci.job.phase125-driver-gates": "extended-suites",
    "ci.job.windows-smoke": "windows-smoke",
    "ci.job.book-pass": "book-doctest",
}
fossils = (
    "native-strict-main",
    "parity-gate",
    "jit-tier2-default",
    "phase125-gui-xvfb",
    "phase125-driver-gates",
    "book-pass",
)

got = set()
for b in blocks:
    if not b.startswith("[[capability]]"):
        continue
    m = re.search(r'^id\s*=\s*"([^"]+)"', b, re.M)
    if not m:
        continue
    cid = m.group(1)
    if not cid.startswith("ci.job."):
        continue
    sm = re.search(r'^surface\s*=\s*"([^"]+)"', b, re.M)
    if not sm:
        raise SystemExit(f"{cid} missing surface")
    surface = sm.group(1)
    got.add(cid)
    if cid not in want:
        raise SystemExit(f"unexpected ci.job.* id {cid}")
    needle = want[cid]
    if needle not in surface:
        raise SystemExit(f"{cid} surface must cite {needle}: {surface}")
    if "C-2d" not in b:
        raise SystemExit(f"{cid} notes must cite C-2d")
    # surface must not claim fossil job names (id may still use them)
    for fos in fossils:
        # allow fossil only if it is not appearing as a claimed job token
        if fos in surface:
            raise SystemExit(f"{cid} surface must not claim fossil job name {fos}: {surface}")
    if cid == "ci.job.native-strict-main" and "full-native-linux" not in surface and "≠ full-native" not in surface and "≠ full-native-linux" not in surface:
        raise SystemExit("native-strict-main surface must fence ≠ full-native-linux")

if got != set(want):
    raise SystemExit(f"expected {sorted(want)}, got {sorted(got)}")
print("c2d_ci_job_surface_aligned=8")
PY

python3 - <<'PY' || die "ci.yml soft/hard contract broken"
from pathlib import Path
import re

ci = Path(".github/workflows/ci.yml").read_text(encoding="utf-8")

def job_block(name: str) -> str:
    m = re.search(rf"(?m)^  {re.escape(name)}:\n(.*?)(?=\n  [a-zA-Z0-9_-]+:|\Z)", ci, re.S)
    if not m:
        raise SystemExit(f"missing job {name}")
    return m.group(0)

soft = [
    "native-linux",
    "native-macos",
    "jit",
    "gui-xvfb",
    "windows-smoke",
    "book-doctest",
    "extended-suites",
    "browser-e2e",
]
for name in soft:
    block = job_block(name)
    if "continue-on-error: true" not in block:
        raise SystemExit(f"{name} must stay continue-on-error: true")

fn = job_block("full-native-linux")
if "continue-on-error: true" in fn:
    raise SystemExit("full-native-linux must not be soft")
print("c2d_soft_jobs_ok=1")
print("c2d_full_native_hard=1")
PY

python3 - <<'PY' || die "native=real / tls.server / census hygiene failed"
from pathlib import Path
import re as _re

caps = Path("packages/internal/capabilities.toml").read_text(encoding="utf-8")
blocks = _re.split(r"(?=^\[\[capability\]\])", caps, flags=_re.M)
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
real = set()
tls_server_partial = False
for b in blocks:
    if not b.startswith("[[capability]]"):
        continue
    m = _re.search(r'^id\s*=\s*"([^"]+)"', b, _re.M)
    if not m:
        continue
    cid = m.group(1)
    nm = _re.search(r'backends\s*=\s*\{[^}]*\bnative\s*=\s*"([^"]+)"', b, _re.S)
    if not nm:
        continue
    if nm.group(1) == "real":
        real.add(cid)
    if cid == "net.tls.server":
        if nm.group(1) != "partial":
            raise SystemExit("net.tls.server must stay partial")
        tls_server_partial = True
if real != want_real:
    raise SystemExit(f"native=real must stay 16 product ids; got {sorted(real)}")
if not tls_server_partial:
    raise SystemExit("net.tls.server missing")

census = Path("docs/census/c-2d-ci-job-align-census.md")
if not census.is_file():
    raise SystemExit("missing c-2d census")
ct = census.read_text(encoding="utf-8")
for needle in ("C-2d", "C-3", "native-linux", "book-doctest", "extended-suites", "r37", "C-2"):
    if needle not in ct and needle.lower() not in ct.lower():
        # r37 may be R-37
        if needle == "r37" and ("R-37" in ct or "r37" in ct):
            continue
        if needle == "r37":
            if "R-37" not in ct:
                raise SystemExit("census missing R-37 park note")
            continue
        raise SystemExit(f"census missing {needle}")

ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
if "C-2d (done" not in ssot and "### C-2d (done" not in ssot:
    raise SystemExit("SSOT must mark C-2d done")
next_sec = ssot.split("## Next", 1)[-1]
if "C-3" not in next_sec:
    raise SystemExit("SSOT Next must be C-3")
if "+++" in next_sec:
    raise SystemExit("SSOT Next must not use +++")

rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("### C-2 ·" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-2 axis [x]")
if not any("**C-2d**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-2d [x]")
if "默认下一" not in rm or "**C-3**" not in rm:
    raise SystemExit("02 must set 默认下一 C-3")

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

print("c2d_native_real_count=16")
print("c2d_tls_server_partial=1")
print("c2d_census_doc=1")
print("c2d_ssot_updated=1")
print("c2d_roadmap_ok=1")
print("c2d_no_mirror=1")
PY

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c2d_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c2d_book_out.txt
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
  echo "c2d_ci_job_align_census_ok=0"
  exit 1
fi
echo "c2d_ci_job_align_census_ok=1"
echo "current=C-2d-ci-job-align"
echo "next=C-3"
