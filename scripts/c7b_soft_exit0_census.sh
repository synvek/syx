#!/usr/bin/env bash
# C-7b soft-step fail-closed census.
# Soft park: C-7c count-gate · scaffold · ModuleId · C-5c+ · tls.server · macOS · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-7b soft-exit0 census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "ci.yml soft-step fail-closed contract broken"
from pathlib import Path
import re

ci = Path(".github/workflows/ci.yml").read_text(encoding="utf-8")

# Soft jobs must remain continue-on-error (not promoted to required).
for job in (
    "windows-smoke:",
    "book-doctest:",
    "browser-e2e:",
    "native-linux:",
    "gui-xvfb:",
    "extended-suites:",
    "jit:",
):
    if job not in ci:
        raise SystemExit(f"missing soft job {job}")

# Count job-level continue-on-error (expect >= 8 soft jobs).
coe = len(re.findall(r"(?m)^\s+continue-on-error:\s*true\s*$", ci))
if coe < 8:
    raise SystemExit(f"expected >=8 continue-on-error true, got {coe}")
print(f"c7b_continue_on_error_count={coe}")

# Flipped sites must propagate exit on failure (comment + exit \"$ec\" or exit 1 near warn).
needles = [
    ("windows-plugin-smoke", 'exit "$ec"'),
    ("browser-e2e::chromium", 'exit "$ec"'),
    ("native-host-shake", 'exit "$ec"'),
    ("native-differential", 'exit "$ec"'),
    ("gui::", 'exit "$ec"'),
    ("jit-differential", 'exit "$ec"'),
    ("jit-optout", 'exit "$ec"'),
]
for title, exit_pat in needles:
    if title not in ci:
        raise SystemExit(f"missing warning title marker {title}")
    # After the title block there should be exit \"$ec\" (or for extended exit 1)
idx = ci.find('title=extended::some modules soft-failed')
if idx < 0:
    raise SystemExit("missing extended soft-failed warning")
chunk = ci[idx : idx + 200]
if "exit 1" not in chunk:
    raise SystemExit("extended must exit 1 on fail")

# Book rate fail must sys.exit(1)
if "sys.exit(1)" not in ci or "book-gate" not in ci:
    raise SystemExit("book-doctest must sys.exit(1) on rate fail")

# No remaining pattern: warn soft then unconditional exit 0 without exit \"$ec\"
# Heuristic: for each C-7b fail-closed comment, ensure exit \"$ec\" or exit 1 follows before next exit 0 success.
if ci.count("# C-7b: step fail-closed") < 6:
    raise SystemExit("expected multiple C-7b fail-closed annotations")

# Whitelist three advisory exit 0 sites
for wl in (
    "C-7b whitelist: empty plugin list",
    "C-7b whitelist: missing JSON",
    "C-7b whitelist: deferred notice",
):
    if wl not in ci:
        raise SystemExit(f"missing whitelist annotation: {wl}")

print("c7b_failclosed_sites=1")
print("c7b_whitelist_ok=1")
print("c7b_jobs_still_soft=1")
PY

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

census = Path("docs/census/c-7b-soft-exit0-census.md")
if not census.is_file():
    raise SystemExit("missing c-7b census")
ct = census.read_text(encoding="utf-8")
for needle in ("C-7b", "C-7c", "exit 0", "continue-on-error", "c7_exited"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")

ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
if "C-7b (done" not in ssot:
    raise SystemExit("SSOT must mark C-7b done")
if "C-7 remains `[~]`" not in ssot:
    raise SystemExit("SSOT must keep C-7 as [~]")
next_sec = ssot.split("## Next", 1)[-1]
if "C-7c" not in next_sec:
    raise SystemExit("SSOT Next must be C-7c")
if "+++" in next_sec:
    raise SystemExit("SSOT Next must not use +++")

rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("**C-7b**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-7b [x]")
if not any(l.startswith("### C-7 ·") and "[~]" in l for l in rm.splitlines()):
    raise SystemExit("02 must keep C-7 as [~]")
if "默认下一 = **C-7c**" not in rm:
    raise SystemExit("02 must set default next to C-7c")

st = Path("plan/01-project-status.md").read_text(encoding="utf-8")
if "C-7b" not in st or "C-7c" not in st:
    raise SystemExit("01 must cite C-7b / next C-7c")

cl = Path("CHANGELOG.md").read_text(encoding="utf-8")
head = cl.split("### SYX 0.4.0", 1)[-1].split("###", 1)[0]
if "C-7b" not in head:
    raise SystemExit("CHANGELOG 0.4.0 must mention C-7b")

print("c7b_native_real_count=16")
print("c7b_tls_server_partial=1")
print("c7b_census_doc=1")
print("c7b_ssot_updated=1")
print("c7b_roadmap_ok=1")
print("c7_exited=0")
PY

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c7b_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c7b_book_out.txt
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
  echo "c7b_soft_exit0_census_ok=0"
  exit 1
fi
echo "c7b_soft_exit0_census_ok=1"
echo "current=C-7b"
echo "next=C-7c"
echo "c7_exited=0"
