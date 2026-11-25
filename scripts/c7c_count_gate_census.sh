#!/usr/bin/env bash
# C-7c count-gate align census.
# Soft park: C-7c+ scaffold · ModuleId · C-5c+ · tls.server · macOS · R-50 · C-8.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-7c count-gate census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "count-gate floor contract broken"
from pathlib import Path
import re

script = Path("scripts/audit_behavioral_tests.sh").read_text(encoding="utf-8")
m_d = re.search(r"^MIN_DRIVER=(\d+)", script, re.M)
m_l = re.search(r"^MIN_LSP=(\d+)", script, re.M)
if not m_d or not m_l:
    raise SystemExit("missing MIN_DRIVER/MIN_LSP defaults in audit script")
if m_d.group(1) != "2600" or m_l.group(1) != "1700":
    raise SystemExit(f"script defaults must be 2600/1700, got {m_d.group(1)}/{m_l.group(1)}")
print("c7c_script_defaults=2600/1700")

ci = Path(".github/workflows/ci.yml").read_text(encoding="utf-8")
if "--min-driver 2600 --min-lsp 1700" not in ci:
    raise SystemExit("ci.yml must use --min-driver 2600 --min-lsp 1700")
print("c7c_ci_floors_match=1")

nightly = Path(".github/workflows/nightly.yml").read_text(encoding="utf-8")
if "--min-driver 2600 --min-lsp 1700" not in nightly:
    raise SystemExit("nightly.yml must use --min-driver 2600 --min-lsp 1700")
if "5150" in nightly.split("Behavioral test count", 1)[-1][:400]:
    raise SystemExit("nightly still mentions obsolete 5150 near count gate")
print("c7c_nightly_floors_match=1")

# Live public surfaces must not claim LSP ≥3050 / audit 2600/3050 as current floor.
live = [
    "docs/README.md",
    "docs/zh/README.zh-CN.md",
    "books/README.md",
    "books/shared/implementation-status.md",
    "docs/adr/architecture-decisions.md",
    "site/docs/about/architecture.md",
    "site/docs/zh/about/architecture.md",
]
# For implementation-status: only the live audit-floor table row (first 120 lines) must be clean.
for rel in live:
    text = Path(rel).read_text(encoding="utf-8")
    head = text if "implementation-status" not in rel else "\n".join(text.splitlines()[:120])
    # Charter / banner floors
    if re.search(r"LSP\s*≥\s*3050|LSP \*\*≥3050\*\*|audit 2600/3050|≥2600 / LSP ≥3050|≥2600 · LSP ≥3050", head):
        raise SystemExit(f"{rel}: still claims LSP 3050 floor in live surface")
    if "implementation-status" in rel:
        if "LSP **≥1700**" not in head and "LSP ≥1700" not in head:
            raise SystemExit(f"{rel}: live audit row must say ≥1700")
    elif "architecture-decisions" in rel or "about/architecture" in rel:
        if "LSP ≥1700" not in head:
            raise SystemExit(f"{rel}: charter must say LSP ≥1700")
    else:
        if "1700" not in head:
            raise SystemExit(f"{rel}: must cite 1700 LSP floor")

print("c7c_readme_no_3050=1")
print("c7c_count_gate_aligned=1")
PY

# Script actually passes with defaults
bash scripts/audit_behavioral_tests.sh --min-driver 2600 --min-lsp 1700 \
  >/tmp/c7c_audit.txt 2>&1 || die "audit_behavioral_tests.sh 2600/1700 failed"
ok "c7c_audit_run" "1"
tail -5 /tmp/c7c_audit.txt

python3 - <<'PY' || die "native=real / ledger hygiene failed"
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

census = Path("docs/census/c-7c-count-gate-census.md")
if not census.is_file():
    raise SystemExit("missing c-7c census")
ct = census.read_text(encoding="utf-8")
for needle in ("C-7c", "C-8", "2600", "1700", "c7_exited"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")

ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
if "C-7c (done" not in ssot:
    raise SystemExit("SSOT must mark C-7c done")
if "C-7 remains `[~]`" not in ssot:
    raise SystemExit("SSOT must keep C-7 as [~]")
next_sec = ssot.split("## Next", 1)[-1]
if "C-8" not in next_sec:
    raise SystemExit("SSOT Next must be C-8")
if "+++" in next_sec:
    raise SystemExit("SSOT Next must not use +++")

rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("**C-7c**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-7c [x]")
if not any(l.startswith("### C-7 ·") and "[~]" in l for l in rm.splitlines()):
    raise SystemExit("02 must keep C-7 as [~]")
if "默认下一 = **C-8**" not in rm:
    raise SystemExit("02 must set default next to C-8")

st = Path("plan/01-project-status.md").read_text(encoding="utf-8")
if "C-7c" not in st or "C-8" not in st:
    raise SystemExit("01 must cite C-7c / next C-8")

cl = Path("CHANGELOG.md").read_text(encoding="utf-8")
head = cl.split("### SYX 0.4.0", 1)[-1].split("###", 1)[0]
if "C-7c" not in head:
    raise SystemExit("CHANGELOG 0.4.0 must mention C-7c")

print("c7c_native_real_count=16")
print("c7c_tls_server_partial=1")
print("c7c_census_doc=1")
print("c7c_ssot_updated=1")
print("c7c_roadmap_ok=1")
print("c7_exited=0")
PY

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c7c_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c7c_book_out.txt
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
  echo "c7c_count_gate_census_ok=0"
  exit 1
fi
echo "c7c_count_gate_census_ok=1"
echo "current=C-7c"
echo "next=C-8"
echo "c7_exited=0"
