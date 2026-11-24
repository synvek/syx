#!/usr/bin/env bash
# C-4d lock v16 infra trim census.
# Soft park: C-4e path content hash · ModuleId · tls.server · macOS · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-4d lock-trim census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "graph_to_lockfile trim contract broken"
from pathlib import Path

rs = Path("crates/syx-pkg/src/resolver.rs").read_text(encoding="utf-8")
if "pub fn graph_to_lockfile" not in rs:
    raise SystemExit("missing graph_to_lockfile")
if "C-4d" not in rs:
    raise SystemExit("C-4d marker missing in resolver")
# Must not hard-fill phase8 regions or stamp CDN env in the writer body.
fn = rs.split("pub fn graph_to_lockfile", 1)[1]
# Take until next top-level pub fn or EOF
body = fn.split("\npub fn ", 1)[0]
if "registry_cdn_phase8_regions" in body:
    raise SystemExit("graph_to_lockfile must not call registry_cdn_phase8_regions")
if "SYX_REGISTRY_CDN" in body:
    raise SystemExit("graph_to_lockfile must not read SYX_REGISTRY_CDN")
if "SYX_BUILD_FARM_QUEUE" in body or "SYX_BILLING_METER_ID" in body:
    raise SystemExit("graph_to_lockfile must not stamp farm/billing env")
if "SYX_MARKETPLACE_CDN" in body or "SYX_HSM_KEY_REF" in body:
    raise SystemExit("graph_to_lockfile must not stamp marketplace/hsm env")
if "..SyxLockfile::default()" not in body and "..Default::default()" not in body:
    # Accept explicit Default fill pattern
    if "artifact_cdn_base: None" not in body and "artifact_cdn_regions" in body:
        raise SystemExit("expected Default fill for infra fields")

# Schema + enforce retained
lf = Path("crates/syx-pkg/src/lockfile.rs").read_text(encoding="utf-8")
if "artifact_cdn_base" not in lf or "billing_meter_id" not in lf:
    raise SystemExit("SyxLockfile schema must retain infra fields")
en = Path("crates/syx-pkg/src/lock_enforce.rs").read_text(encoding="utf-8")
if "pub fn enforce_lock_v16" not in en:
    raise SystemExit("enforce_lock_v16 must remain")

integ = Path("crates/syx-pkg/tests/integration.rs").read_text(encoding="utf-8")
if "c4d_graph_to_lockfile_omits_infra_env" not in integ:
    raise SystemExit("missing c4d integration pin")

ev = Path("crates/syx-driver/tests/evergreen/registry.rs").read_text(encoding="utf-8")
if "must not stamp SYX_REGISTRY_CDN" not in ev:
    raise SystemExit("registry_cdn_hard must assert no CDN stamp in lock")

print("c4d_api=1")
print("c4d_tests_present=1")
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

census = Path("docs/census/c-4d-lock-trim-census.md")
if not census.is_file():
    raise SystemExit("missing c-4d census")
ct = census.read_text(encoding="utf-8")
for needle in ("C-4d", "C-4e", "graph_to_lockfile", "artifact_cdn"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")

ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
if "C-4d (done" not in ssot:
    raise SystemExit("SSOT must mark C-4d done")
next_sec = ssot.split("## Next", 1)[-1]
if "C-4e" not in next_sec:
    raise SystemExit("SSOT Next must be C-4e")
if "+++" in next_sec:
    raise SystemExit("SSOT Next must not use +++")

rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("**C-4d**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-4d [x]")
if not any("### C-4 ·" in l and "[~]" in l for l in rm.splitlines()):
    raise SystemExit("02 must keep C-4 as [~]")
if not any("**C-4e**" in l and "[ ]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-4e [ ] as next slice")
if "默认下一" not in rm or "C-4e" not in rm:
    raise SystemExit("02 must set 默认下一 C-4e")
if "### C-3 ·" not in rm or "[~]" not in rm:
    raise SystemExit("02 must keep C-3 as [~]")

st = Path("plan/01-project-status.md").read_text(encoding="utf-8")
if "C-4d" not in st or "C-4e" not in st:
    raise SystemExit("01 must cite C-4d / next C-4e")

cl = Path("CHANGELOG.md").read_text(encoding="utf-8")
head = cl.split("### SYX 0.4.0", 1)[-1].split("###", 1)[0]
if "C-4d" not in head:
    raise SystemExit("CHANGELOG 0.4.0 must mention C-4d")

print("c4d_native_real_count=16")
print("c4d_tls_server_partial=1")
print("c4d_census_doc=1")
print("c4d_ssot_updated=1")
print("c4d_roadmap_ok=1")
PY

echo "== c4d integration =="
cargo test -p syx-pkg --test integration -- --test-threads=1 c4d_ >/tmp/c4d_integ.txt 2>&1 \
  || die "c4d integration failed"
if ! grep -q "c4d_graph_to_lockfile_omits_infra_env" /tmp/c4d_integ.txt; then
  die "missing c4d pin run"
fi
if ! grep -qE "test result: ok\. 1 passed" /tmp/c4d_integ.txt; then
  die "expected 1 passed c4d test"
fi
ok "c4d_integ_cargo" "1"
tail -12 /tmp/c4d_integ.txt

echo "== CDN behavior + lockfield smoke =="
cargo test -p syx-driver --test evergreen -- --test-threads=1 \
  registry::registry_cdn_hard \
  registry::graph_to_lockfile_v6_fields \
  registry::registry_phase70_charter \
  >/tmp/c4d_evergreen.txt 2>&1 \
  || die "evergreen CDN/lock smoke failed"
if ! grep -qE "test result: ok\. 3 passed" /tmp/c4d_evergreen.txt; then
  die "expected 3 passed evergreen smokes"
fi
ok "c4d_evergreen_smoke" "1"
tail -12 /tmp/c4d_evergreen.txt

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c4d_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c4d_book_out.txt
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
  echo "c4d_lock_trim_census_ok=0"
  exit 1
fi
echo "c4d_lock_trim_census_ok=1"
echo "current=C-4d-lock-trim"
echo "next=C-4e"
