#!/usr/bin/env bash
# C-5e Windows Job Object thin gate census.
# Soft park: C-5f macOS sandbox · tls.server · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-5e Job Object thin census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "jobobject API / hooks broken"
from pathlib import Path

jo = Path("crates/syx-macro/src/jobobject_windows.rs").read_text(encoding="utf-8")
for needle in (
    "CreateJobObjectW",
    "AssignProcessToJobObject",
    "PROCESS_MEMORY_LIMIT",
    "JOBOBJECT_EXTENDED_LIMIT_INFORMATION",
    "try_apply_rss_hard_wall_bytes",
    "try_apply_rss_hard_wall_for_pid",
    "MODE_JOB_OBJECT_MEMORY_WIRED",
    "C-5e",
):
    if needle not in jo:
        raise SystemExit(f"jobobject_windows missing {needle}")

embed = Path("crates/syx-driver/src/embed_v2.rs").read_text(encoding="utf-8")
if "try_apply_rss_hard_wall" not in embed or "C-5e" not in embed:
    raise SystemExit("embed must wire RSS hard wall dispatcher + C-5e")
if 'target_os = "windows"' not in embed and "target_os = \"windows\"" not in embed:
    # allow either quote style
    if "windows" not in embed.split("rss_limit.is_some()", 1)[-1][:400]:
        raise SystemExit("embed RSS fail-closed must allow windows")

mac = Path("crates/syx-macro/src/external_sandbox.rs").read_text(encoding="utf-8")
if "try_apply_rss_hard_wall_for_pid" not in mac or "C-5e" not in mac:
    raise SystemExit("macro must parent-assign Job Object")

print("c5e_api=1")
print("c5e_hooks=1")
PY

python3 - <<'PY' || die "neg pin missing"
from pathlib import Path
smoke = Path("crates/syx-driver/tests/evergreen/smoke.rs").read_text(encoding="utf-8")
for needle in ("c5e_jobobject_hooks_and_fail_closed", "C-5e", "CreateJobObjectW"):
    if needle not in smoke:
        raise SystemExit(f"evergreen missing {needle}")
jo = Path("crates/syx-macro/src/jobobject_windows.rs").read_text(encoding="utf-8")
if "c5e_non_windows_create_fails_closed" not in jo:
    raise SystemExit("macro unit nail missing")
print("c5e_neg_pin=1")
PY

python3 - <<'PY' || die "ledger / KPI hygiene failed"
from pathlib import Path
import re as _re

caps = Path("packages/internal/capabilities.toml").read_text(encoding="utf-8")
blocks = _re.split(r"(?=^\[\[capability\]\])", caps, flags=_re.M)
want_real = {
    "io.path", "io.file.read_all_bytes", "net.tcp", "net.tcp.listener", "data.sqlite",
    "net.http", "data.sqlite.pool", "data.sqlite.row", "net.http.types", "net.http.server",
    "net.http.stream", "net.http.timeout_tls", "io.file.raii", "io.file.stream",
    "io.file.watch", "net.tls.client",
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
    raise SystemExit(f"native=real drift: {sorted(real)}")
if not tls_server_partial:
    raise SystemExit("net.tls.server missing")

census = Path("docs/census/c-5e-jobobject-thin-census.md")
if not census.is_file():
    raise SystemExit("missing census doc")
ct = census.read_text(encoding="utf-8")
for needle in ("C-5e", "C-5f", "Job Object", "PROCESS_MEMORY_LIMIT"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")

ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
if "C-5e (done" not in ssot:
    raise SystemExit("SSOT must mark C-5e done")
next_sec = ssot.split("## Next", 1)[-1]
if "C-5f" not in next_sec:
    raise SystemExit("SSOT Next must be C-5f")
if "+++" in next_sec:
    raise SystemExit("SSOT Next must not use +++")

rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("**C-5e**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-5e [x]")
if "默认下一" not in rm or "C-5f" not in rm:
    raise SystemExit("02 must set 默认下一 C-5f")
if "### C-5 ·" not in rm or "[~]" not in rm:
    raise SystemExit("02 must keep C-5 as [~]")

st = Path("plan/01-project-status.md").read_text(encoding="utf-8")
if "C-5f" not in st or "C-5e" not in st:
    raise SystemExit("01 must cite C-5e / next C-5f")

adr = Path("docs/adr/macro-embed-threat-model.md").read_text(encoding="utf-8")
if "C-5e" not in adr or "Job Object" not in adr:
    raise SystemExit("ADR must cite C-5e Job Object")

envr = Path("docs/env-registry.md").read_text(encoding="utf-8")
if "Job Object" not in envr or "SYX_EMBED_RSS_MB" not in envr:
    raise SystemExit("env-registry must document Windows Job Object for RSS_MB")

qr = Path("docs/8_QuickReference.md").read_text(encoding="utf-8")
if "label/probe only" in qr and "SYX_EMBED_SECCOMP" in qr:
    # ensure the SECCOMP row is no longer the unwired claim
    for line in qr.splitlines():
        if "SYX_EMBED_SECCOMP" in line and "unwired" in line:
            raise SystemExit("QuickRef still claims SECCOMP unwired")

cl = Path("CHANGELOG.md").read_text(encoding="utf-8")
head = cl.split("### SYX 0.4.0", 1)[-1].split("###", 1)[0]
if "C-5e" not in head:
    raise SystemExit("CHANGELOG must mention C-5e")

print("c5e_native_real_count=16")
print("c5e_tls_server_partial=1")
print("c5e_census_doc=1")
print("c5e_ssot_updated=1")
print("c5e_roadmap_ok=1")
PY

echo "== nails cargo =="
cargo test -p syx-macro c5e_ -- --nocapture >/tmp/c5e_macro.txt 2>&1 \
  || die "c5e macro unit failed"
ok "c5e_macro_unit" "1"

cargo test -p syx-driver --test evergreen c5e_ -- --nocapture >/tmp/c5e_ever.txt 2>&1 \
  || die "c5e evergreen failed"
if ! grep -q "c5e_jobobject_hooks_and_fail_closed ... ok" /tmp/c5e_ever.txt; then
  die "evergreen nail did not pass"
fi
ok "c5e_evergreen" "1"
tail -16 /tmp/c5e_ever.txt

# Also hold C-5d RSS nail (macOS fail-closed / Linux-Windows wired).
cargo test -p syx-driver --test evergreen c5d_embed_rss -- --nocapture >/tmp/c5e_c5d.txt 2>&1 \
  || die "c5d rss nail regress"
ok "c5e_c5d_rss_held" "1"

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c5e_book_out.txt 2>&1 || die "opt-in output pins failed"
python3 - <<PY || die "sy,output must be 12/12"
import json, sys
from pathlib import Path
d = json.loads(Path("$out_json").read_text())
ok_n, tot = d.get("output_run_ok", 0), d.get("output_run_total", 0)
en = [r for r in d["results"] if "zh-cn" not in r["file"]]
zh = [r for r in d["results"] if "zh-cn" in r["file"]]
print(f"output_run_ok={ok_n}")
print(f"output_run_total={tot}")
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
[[ "$browser_failed" -eq 0 ]] || die "browser_failed must be 0"
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
[[ "$wasm_failed" -eq 0 && "$wasm_passed" -eq 46 ]] || die "wasm must be 46/46"
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
print(f"simd_object_residual={len(obj_re.findall(code))}")
sys.exit(0 if len(obj_re.findall(code)) >= 159 else 1)
PY

if [[ "$fail" -ne 0 ]]; then
  echo "c5e_jobobject_thin_census_ok=0"
  exit 1
fi
echo "c5e_jobobject_thin_census_ok=1"
echo "current=C-5e-jobobject-thin"
echo "next=C-5f"
