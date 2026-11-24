#!/usr/bin/env bash
# C-3d MIR stack-height floor census.
# Soft park: ModuleId · full MirOp verifier · tls.server · macOS full-native · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-3d mir-stack-floor census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "stack floor API / hooks contract broken"
from pathlib import Path

api = Path("crates/syx-mir/src/mir_stack_floor.rs").read_text(encoding="utf-8")
if "pub fn verify_mir_stack_floor" not in api:
    raise SystemExit("missing verify_mir_stack_floor")
if "pub fn verify_mir_stack_floor_functions" not in api:
    raise SystemExit("missing verify_mir_stack_floor_functions")
if "MIR004" not in api or "C-3d" not in api:
    raise SystemExit("MIR004 / C-3d missing in mir_stack_floor")

lib = Path("crates/syx-mir/src/lib.rs").read_text(encoding="utf-8")
if "verify_mir_stack_floor" not in lib or "verify_mir_stack_floor_functions" not in lib:
    raise SystemExit("lib.rs must re-export stack floor")

unify = Path("crates/syx-driver/src/mir_unify.rs").read_text(encoding="utf-8")
cfg = unify.find("verify_mir_cfg_floor")
fnx = unify.find("MirFnIndex::build")
stk = unify.find("verify_mir_stack_floor")
if cfg < 0 or fnx < 0 or stk < 0 or not (stk > fnx > cfg):
    raise SystemExit("unify must stack floor after FnIndex after CFG")

codegen = Path("crates/syx-codegen/src/lib.rs").read_text(encoding="utf-8")
start = codegen.find("pub fn compile_from_mir_with_source")
if start < 0:
    raise SystemExit("missing compile_from_mir_with_source")
after = codegen[start + 1 :]
rel = after.find("\n    pub fn ")
body = codegen[start : start + 1 + rel if rel >= 0 else start + 5000]
cfg = body.find("verify_mir_cfg_floor")
fnx = body.find("MirFnIndex::build_from_functions")
stk = body.find("verify_mir_stack_floor_functions")
if cfg < 0 or fnx < 0 or stk < 0 or not (stk > fnx > cfg):
    raise SystemExit("compile_from_mir must stack floor after FnIndex after CFG")

print("c3d_stack_api=1")
print("c3d_hooks=1")
PY

python3 - <<'PY' || die "neg pin missing"
from pathlib import Path

neg = Path("crates/syx-driver/tests/credibility/c3d_mir_stack_floor_contract.rs")
if not neg.is_file():
    raise SystemExit("missing c3d_mir_stack_floor_contract.rs")
nt = neg.read_text(encoding="utf-8")
for needle in (
    "c3d_rejects_add_underflow",
    "c3d_rejects_store_local_underflow",
    "c3d_rejects_call_arity_underflow",
    "c3d_accepts_const_add_return",
    "c3d_accepts_known_call_arity",
    "c3d_compile_from_mir_rejects_add_underflow",
    "MIR004",
    "verify_mir_stack_floor",
):
    if needle not in nt:
        raise SystemExit(f"neg pin missing {needle}")

main = Path("crates/syx-driver/tests/credibility/main.rs").read_text(encoding="utf-8")
if "mod c3d_mir_stack_floor_contract" not in main:
    raise SystemExit("credibility main must mod c3d_mir_stack_floor_contract")

print("c3d_neg_pin=1")
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

census = Path("docs/census/c-3d-mir-stack-floor-census.md")
if not census.is_file():
    raise SystemExit("missing c-3d census")
ct = census.read_text(encoding="utf-8")
for needle in ("C-3d", "C-3e", "MIR004", "verify_mir_stack_floor", "stack"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")

ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
if "C-3d (done" not in ssot:
    raise SystemExit("SSOT must mark C-3d done")
next_sec = ssot.split("## Next", 1)[-1]
if "C-3e" not in next_sec:
    raise SystemExit("SSOT Next must be C-3e")
if "+++" in next_sec:
    raise SystemExit("SSOT Next must not use +++")

rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("**C-3d**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-3d [x]")
if not any("**C-3e**" in l and "[ ]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-3e [ ] default next")
if "### C-3 ·" not in rm or "[~]" not in rm:
    raise SystemExit("02 must keep C-3 as [~]")
if "默认下一" not in rm or "**C-3e**" not in rm:
    raise SystemExit("02 must set 默认下一 C-3e")

st = Path("plan/01-project-status.md").read_text(encoding="utf-8")
if "C-3e" not in st or "C-3d" not in st:
    raise SystemExit("01 must cite C-3d / next C-3e")

cl = Path("CHANGELOG.md").read_text(encoding="utf-8")
head = cl.split("### SYX 0.4.0", 1)[-1].split("###", 1)[0]
if "C-3d" not in head:
    raise SystemExit("CHANGELOG 0.4.0 must mention C-3d")

print("c3d_native_real_count=16")
print("c3d_tls_server_partial=1")
print("c3d_census_doc=1")
print("c3d_ssot_updated=1")
print("c3d_roadmap_ok=1")
PY

echo "== neg pin cargo =="
cargo test -p syx-driver --test credibility c3d_ -- --nocapture >/tmp/c3d_neg.txt 2>&1 \
  || die "c3d credibility tests failed"
if ! grep -q "c3d_rejects_add_underflow ... ok" /tmp/c3d_neg.txt; then
  die "neg pin did not pass"
fi
ok "c3d_neg_cargo" "1"
tail -16 /tmp/c3d_neg.txt

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c3d_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c3d_book_out.txt
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
  echo "c3d_mir_stack_floor_census_ok=0"
  exit 1
fi
echo "c3d_mir_stack_floor_census_ok=1"
echo "current=C-3d-mir-stack-floor"
echo "next=C-3e"
