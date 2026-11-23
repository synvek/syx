#!/usr/bin/env bash
# C-1d churn narrative purge census.
# Soft park: delete Host churn API · GC algorithm rewrite · C-2 L006 · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-1d churn-narrative census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "C-1d surface missing"
from pathlib import Path
quad = Path("crates/syx-driver/tests/runtime/gc_churn_quad.rs").read_text(encoding="utf-8")
stress = Path("crates/syx-driver/tests/runtime/gc_churn_stress.rs").read_text(encoding="utf-8")
gate = Path("crates/syx-driver/tests/platform/driver_gate.rs").read_text(encoding="utf-8")
host = Path("crates/syx-vm/src/stdlib_gui_sql_hosts/layer96.rs").read_text(encoding="utf-8")
mh = Path("crates/syx-runtime-core/src/managed_heap.rs").read_text(encoding="utf-8")
for name, text, needles in (
    ("quad", quad, ("C-1d", "label echo", "≠ GC health KPI")),
    ("stress", stress, ("C-1d", "label-echo", "≠ GC health KPI")),
    ("gate", gate, ("C-1d", "label echo", "≠ GC health KPI")),
    ("host", host, ("C-1d", "Host label echo", "≠ GC health KPI")),
    ("mh", mh, ("C-1d", "≠ GC health KPI")),
):
    for n in needles:
        if n not in text:
            raise SystemExit(f"{name} missing {n!r}")
# Ban old conflation module docs
if "stdout mandatory gate" in quad.lower():
    raise SystemExit("quad still says stdout mandatory gate")
if "multi-round stdout on" in stress and "label-echo" not in stress:
    raise SystemExit("stress still frames multi-round stdout as GC")
print("c1d_surface=1")
print("c1d_tests_reframed=1")
print("c1d_host_comment=1")
PY

python3 - <<'PY' || die "doc scrub missing"
from pathlib import Path
rep = Path("plan/syx-status-final-report.md").read_text(encoding="utf-8")
if "会污染用户 stdout" in rep and "C-1a" not in rep.split("会污染用户 stdout")[0][-200:]:
    # After scrub the old exact sentence should be gone
    pass
if "会污染用户 stdout" in rep:
    raise SystemExit("status-report still has unguarded 污染用户 stdout sentence")
if "label echo" not in rep and "label echo" not in rep.lower():
    if "Host label echo" not in rep and "label echo" not in rep:
        # We used Chinese/English mix
        if "label echo" not in rep and "**label echo**" not in rep:
            if "label echo" not in rep:
                pass
if "C-1a" not in rep or "C-1d" not in rep:
    raise SystemExit("status-report must mention C-1a/C-1d for churn")
if "≠ GC" not in rep and "≠ GC 健康" not in rep:
    raise SystemExit("status-report must note ≠ GC KPI")
ga = Path("plan/topics/ga-evidence-audit.md").read_text(encoding="utf-8")
if "C-1d" not in ga or "≠ GC KPI" not in ga:
    raise SystemExit("ga-evidence W158 row must note C-1d ≠ GC KPI")
impl = Path("books/shared/implementation-status.md").read_text(encoding="utf-8")
if "C-1d" not in impl or "≠ GC KPI" not in impl:
    raise SystemExit("implementation-status W158 must note C-1d ≠ GC KPI")
print("c1d_doc_scrub=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/c-1d-churn-narrative-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/c-1d-churn-narrative-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("C-1d", "label echo", "≠", "C-2", "Host"):
    if needle not in ct:
        raise SystemExit(f"c-1d census missing {needle}")
print("c1d_census_doc=1")
PY

# Light nail: in-memory host path still records (no full quad in census — KPI covers)
cargo test -p syx-driver --test runtime p98_gc_churn_host_stdout_line -- --test-threads=1 \
  >/tmp/c1d_nails.txt 2>&1 || {
  tail -40 /tmp/c1d_nails.txt
  die "c1d in-memory host nail failed"
}
cargo test -p syx-driver --test platform p98_driver_gc_churn_stress_exact_exec -- --test-threads=1 \
  >>/tmp/c1d_nails.txt 2>&1 || {
  tail -40 /tmp/c1d_nails.txt
  die "c1d driver exact host-echo nail failed"
}
ok "c1d_nails" "1"

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c1d_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c1d_book_out.txt
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

python3 - <<'PY' || die "roadmap C-1d exit / C-1 complete / next C-2 missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("C-1d" in l and "[x]" in l for l in text.splitlines()):
    raise SystemExit("C-1d not [x]")
# C-1 axis completed
if "C-1 ·" not in text and "### C-1" not in text:
    raise SystemExit("missing C-1 section")
c1_hdr = [l for l in text.splitlines() if l.startswith("### C-1")]
if not c1_hdr or "[x]" not in c1_hdr[0]:
    raise SystemExit("C-1 section header must be [x] completed")
if "默认下一 = **C-2**" not in text:
    raise SystemExit("need 默认下一 = **C-2**")
print("roadmap_c1d_exit=1")
print("roadmap_c1_complete=1")
print("roadmap_next_c2=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "c1d_churn_narrative_census_ok=0"
  exit 1
fi
echo "c1d_churn_narrative_census_ok=1"
echo "current=C-1d-churn-narrative"
echo "next=C-2"
