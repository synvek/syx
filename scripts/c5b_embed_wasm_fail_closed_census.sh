#!/usr/bin/env bash
# C-5b embed wasm fail-closed census.
# Soft park: C-6 version SSOT · C-5c+ timeout/seccomp · ModuleId · tls.server · macOS · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-5b embed-wasm-fail-closed census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "fail-closed contract broken"
from pathlib import Path
import re

ev = Path("crates/syx-driver/src/embed_v2.rs").read_text(encoding="utf-8")
if "C-5b" not in ev:
    raise SystemExit("embed_v2 missing C-5b marker")
if "embed_wasm_inprocess_fallback_allowed" not in ev:
    raise SystemExit("missing embed_wasm_inprocess_fallback_allowed")
# Must not keep always-true probe body
if re.search(r"fn embed_wasm_sandbox_probe\(\)[^{]*\{\s*true\s*\}", ev):
    raise SystemExit("embed_wasm_sandbox_probe must not be constantly true")
# syx_embed_v2_run must fail-closed when sandbox enabled
fn = ev.split("pub extern \"C\" fn syx_embed_v2_run", 1)[1].split("\npub extern \"C\" fn ", 1)[0]
if "embed_wasm_sandbox_enabled()" not in fn:
    raise SystemExit("syx_embed_v2_run must gate on embed_wasm_sandbox_enabled")
if "return std::ptr::null_mut()" not in fn:
    raise SystemExit("syx_embed_v2_run must return null on fail-closed path")
# After enabled block, must not fall into compile_and_run_embedded without return
# Heuristic: enabled() block contains null return before in-process path
idx = fn.find("if embed_wasm_sandbox_enabled()")
if idx < 0:
    raise SystemExit("missing enabled gate in syx_embed_v2_run")
after = fn[idx:]
# First null return after enabled should appear before compile_and_run_embedded
null_i = after.find("return std::ptr::null_mut()")
inproc_i = after.find("compile_and_run_embedded")
if null_i < 0 or inproc_i < 0 or null_i > inproc_i:
    raise SystemExit("fail-closed null must precede in-process path under enabled gate")

smoke = Path("crates/syx-driver/tests/evergreen/smoke.rs").read_text(encoding="utf-8")
for t in (
    "c5b_wasm_sandbox_fail_closed_without_wasmtime",
    "c5b_default_still_inprocess",
):
    if t not in smoke:
        raise SystemExit(f"missing {t}")

adr = Path("docs/adr/macro-embed-threat-model.md").read_text(encoding="utf-8")
if "C-5b" not in adr or "no silent in-process" not in adr.lower() and "无静默" not in adr and "null" not in adr:
    if "SYX_EMBED_WASM_SANDBOX=1" not in adr or "null" not in adr:
        raise SystemExit("ADR pin2 must document C-5b fail-closed")

envr = Path("docs/env-registry.md").read_text(encoding="utf-8")
if "SYX_EMBED_WASM_SANDBOX" not in envr:
    raise SystemExit("env-registry must list SYX_EMBED_WASM_SANDBOX")

print("c5b_api=1")
print("c5b_tests_present=1")
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

census = Path("docs/census/c-5b-embed-wasm-fail-closed-census.md")
if not census.is_file():
    raise SystemExit("missing c-5b census")
ct = census.read_text(encoding="utf-8")
for needle in ("C-5b", "C-6", "fail-closed", "SYX_EMBED_WASM_SANDBOX"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")

ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
if "C-5b (done" not in ssot:
    raise SystemExit("SSOT must mark C-5b done")
next_sec = ssot.split("## Next", 1)[-1]
if "C-6" not in next_sec:
    raise SystemExit("SSOT Next must be C-6")
if "+++" in next_sec:
    raise SystemExit("SSOT Next must not use +++")

rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("**C-5b**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-5b [x]")
if not any("### C-5 ·" in l and "[~]" in l for l in rm.splitlines()):
    raise SystemExit("02 must keep C-5 as [~]")
if "默认下一" not in rm or "C-6" not in rm:
    raise SystemExit("02 must set 默认下一 C-6")
if "### C-3 ·" not in rm or "[~]" not in rm:
    raise SystemExit("02 must keep C-3 as [~]")

st = Path("plan/01-project-status.md").read_text(encoding="utf-8")
if "C-5b" not in st or "C-6" not in st:
    raise SystemExit("01 must cite C-5b / next C-6")

cl = Path("CHANGELOG.md").read_text(encoding="utf-8")
head = cl.split("### SYX 0.4.0", 1)[-1].split("###", 1)[0]
if "C-5b" not in head:
    raise SystemExit("CHANGELOG 0.4.0 must mention C-5b")

print("c5b_native_real_count=16")
print("c5b_tls_server_partial=1")
print("c5b_census_doc=1")
print("c5b_ssot_updated=1")
print("c5b_roadmap_ok=1")
PY

echo "== c5b evergreen =="
cargo test -p syx-driver --test evergreen -- --test-threads=1 c5b_ >/tmp/c5b_evergreen.txt 2>&1 \
  || die "c5b evergreen failed"
if ! grep -qE "test result: ok\. 2 passed" /tmp/c5b_evergreen.txt; then
  die "expected 2 passed c5b tests"
fi
ok "c5b_evergreen_smoke" "1"
tail -14 /tmp/c5b_evergreen.txt

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c5b_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c5b_book_out.txt
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
  echo "c5b_embed_wasm_fail_closed_census_ok=0"
  exit 1
fi
echo "c5b_embed_wasm_fail_closed_census_ok=1"
echo "current=C-5b-embed-wasm-fail-closed"
echo "next=C-6"
