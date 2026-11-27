#!/usr/bin/env bash
# C-3by MIR CallWitness stack-height thin floor census.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-3by mir-call-witness-stack-floor census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "call-witness-stack-floor API / hooks contract broken"
from pathlib import Path

api = Path("crates/syx-mir/src/mir_call_witness_stack_floor.rs").read_text(encoding="utf-8")
for needle in (
    "verify_mir_call_witness_stack_floor",
    "verify_mir_call_witness_stack_floor_functions",
    "MIR077",
    "C-3by",
    "MirOp::CallWitness",
    "Not a change to `is_suite1_pilot_mir_op`",
    "by_index.get",
):
    if needle not in api:
        raise SystemExit(f"mir_call_witness_stack_floor missing {needle}")

lib = Path("crates/syx-mir/src/lib.rs").read_text(encoding="utf-8")
if "verify_mir_call_witness_stack_floor" not in lib:
    raise SystemExit("lib.rs must re-export verify_mir_call_witness_stack_floor")

unify = Path("crates/syx-driver/src/mir_unify.rs").read_text(encoding="utf-8")
or_f = unify.find("verify_mir_iterator_next_stack_floor")
cs_f = unify.find("verify_mir_call_witness_stack_floor")
if or_f < 0 or cs_f < 0 or cs_f < or_f:
    raise SystemExit("unify must call-witness-stack floor after iterator-next-stack floor")

codegen = Path("crates/syx-codegen/src/lib.rs").read_text(encoding="utf-8")
start = codegen.find("pub fn compile_from_mir_with_source")
if start < 0:
    raise SystemExit("missing compile_from_mir_with_source")
after = codegen[start + 1 :]
rel = after.find("\n    pub fn ")
body = codegen[start : start + 1 + rel if rel >= 0 else start + 5000]
or_c = body.find("verify_mir_iterator_next_stack_floor_functions")
cs_c = body.find("verify_mir_call_witness_stack_floor_functions")
if or_c < 0 or cs_c < 0 or cs_c < or_c:
    raise SystemExit("compile_from_mir must call-witness-stack floor after iterator-next-stack floor")

print("c3by_call_witness_stack_api=1")
print("c3by_hooks=1")
PY

python3 - <<'PY' || die "call_witness pin missing"
from pathlib import Path

neg = Path("crates/syx-driver/tests/credibility/c3by_mir_call_witness_stack_floor_contract.rs")
if not neg.is_file():
    raise SystemExit("missing c3by_mir_call_witness_stack_floor_contract.rs")
nt = neg.read_text(encoding="utf-8")
for needle in (
    "c3by_rejects_call_witness_underflow",
    "c3by_call_witness_ok",
    "c3by_no_call_witness_ok",
    "c3by_compile_from_mir_rejects_call_witness_underflow",
    "c3by_hooks_after_iterator_next_stack",
    "MIR077",
):
    if needle not in nt:
        raise SystemExit(f"call_witness pin missing {needle}")

main = Path("crates/syx-driver/tests/credibility/main.rs").read_text(encoding="utf-8")
if "mod c3by_mir_call_witness_stack_floor_contract" not in main:
    raise SystemExit("credibility main must mod c3by_mir_call_witness_stack_floor_contract")

print("c3by_call_witness_pin=1")
PY

python3 - <<'PY' || die "native=real / tls.server / ledger hygiene failed"
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

census = Path("docs/census/c-3by-mir-call-witness-stack-floor-census.md")
if not census.is_file():
    raise SystemExit("missing c-3bw census")
ct = census.read_text(encoding="utf-8")
for needle in ("C-3by", "MIR077", "verify_mir_call_witness_stack_floor", "跨轴"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")

ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
next_sec = ssot.split("## Next", 1)[-1]
if "跨轴" not in next_sec:
    raise SystemExit("SSOT Next must be 跨轴")
if "by" not in next_sec:
    raise SystemExit("SSOT Next must note by done")

rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("**C-3by**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-3by [x]")
c3_header = [l for l in rm.splitlines() if l.startswith("### C-3 ·")]
if not c3_header or "[~]" not in c3_header[0]:
    raise SystemExit("02 must keep C-3 as [~]")

st = Path("plan/01-project-status.md").read_text(encoding="utf-8")
if "C-3by" not in st:
    raise SystemExit("01 must cite C-3by")

cl = Path("CHANGELOG.md").read_text(encoding="utf-8")
head = cl.split("### SYX 0.4.0", 1)[-1].split("###", 1)[0]
if "C-3by" not in head or "MIR077" not in head:
    raise SystemExit("CHANGELOG must mention C-3by / MIR077")

print("c3by_native_real_count=16")
print("c3by_tls_server_partial=1")
print("c3by_census_doc=1")
print("c3by_ssot_updated=1")
print("c3by_roadmap_ok=1")
PY

echo "== nails cargo =="
cargo test -p syx-mir mir_call_witness_stack_floor -- --nocapture >/tmp/c3by_unit.txt 2>&1 \
  || die "c3by mir unit failed"
ok "c3by_mir_unit" "1"

cargo test -p syx-driver --test credibility c3by_mir_call_witness_stack -- --nocapture >/tmp/c3by_cred.txt 2>&1 \
  || die "c3by credibility failed"
if ! grep -q "c3by_rejects_call_witness_underflow ... ok" /tmp/c3by_cred.txt; then
  die "credibility CallWitness underflow nail did not pass"
fi
ok "c3by_credibility" "1"
tail -20 /tmp/c3by_cred.txt

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c3by_book_out.txt 2>&1 || die "opt-in output pins failed"
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
    code = "\n".join(ln for ln in path.read_text(encoding="utf-8").splitlines() if not ln.strip().startswith("//"))
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
  echo "c3by_mir_call_witness_stack_floor_census_ok=0"
  exit 1
fi
echo "c3by_mir_call_witness_stack_floor_census_ok=1"
echo "current=C-3by-mir-call-witness-stack-floor"
echo "next=cross-axis"
