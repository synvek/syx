#!/usr/bin/env bash
# C-5f macOS Seatbelt net-deny thin census.
# Soft park: C-5g full filter · tls.server · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-5f macOS Seatbelt thin census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "seatbelt API / hooks broken"
from pathlib import Path

sb = Path("crates/syx-macro/src/macos_sandbox.rs").read_text(encoding="utf-8")
for needle in (
    "sandbox_init",
    "SANDBOX_NAMED",
    "no-network",
    "try_install_seatbelt_netdeny",
    "seatbelt-netdeny-wired",
    "sandbox-exec",
    "C-5f",
):
    if needle not in sb:
        raise SystemExit(f"macos_sandbox missing {needle}")

embed = Path("crates/syx-driver/src/embed_v2.rs").read_text(encoding="utf-8")
if "embed_macos_sandbox_enabled" not in embed or "sandbox-exec" not in embed:
    raise SystemExit("embed must wire macOS sandbox-exec")
if "SYX_EMBED_MACOS_SANDBOX" not in embed or "C-5f" not in embed:
    raise SystemExit("embed must cite SYX_EMBED_MACOS_SANDBOX + C-5f")

mac = Path("crates/syx-macro/src/external_sandbox.rs").read_text(encoding="utf-8")
if "SYX_MACRO_MACOS_SANDBOX" not in mac or "try_install_seatbelt_netdeny" not in mac:
    raise SystemExit("macro must install Seatbelt")
if "C-5f" not in mac:
    raise SystemExit("macro must cite C-5f")

print("c5f_api=1")
print("c5f_hooks=1")
PY

python3 - <<'PY' || die "neg pin missing"
from pathlib import Path
smoke = Path("crates/syx-driver/tests/evergreen/smoke.rs").read_text(encoding="utf-8")
for needle in ("c5f_macos_seatbelt_hooks_and_fail_closed", "C-5f", "sandbox_init"):
    if needle not in smoke:
        raise SystemExit(f"evergreen missing {needle}")
sb = Path("crates/syx-macro/src/macos_sandbox.rs").read_text(encoding="utf-8")
if "c5f_non_macos_install_fails_closed" not in sb:
    raise SystemExit("macro unit nail missing")
print("c5f_neg_pin=1")
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

census = Path("docs/census/c-5f-macos-sandbox-thin-census.md")
if not census.is_file():
    raise SystemExit("missing census doc")
ct = census.read_text(encoding="utf-8")
for needle in ("C-5f", "C-5g", "Seatbelt", "sandbox_init", "no-network"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")

ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
if "C-5f (done" not in ssot:
    raise SystemExit("SSOT must mark C-5f done")
next_sec = ssot.split("## Next", 1)[-1]
if "C-5g" not in next_sec:
    raise SystemExit("SSOT Next must be C-5g")
if "+++" in next_sec:
    raise SystemExit("SSOT Next must not use +++")

rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("**C-5f**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-5f [x]")
if "默认下一" not in rm or "C-5g" not in rm:
    raise SystemExit("02 must set 默认下一 C-5g")
if "### C-5 ·" not in rm or "[~]" not in rm:
    raise SystemExit("02 must keep C-5 as [~]")

st = Path("plan/01-project-status.md").read_text(encoding="utf-8")
if "C-5g" not in st or "C-5f" not in st:
    raise SystemExit("01 must cite C-5f / next C-5g")

adr = Path("docs/adr/macro-embed-threat-model.md").read_text(encoding="utf-8")
if "C-5f" not in adr or "Seatbelt" not in adr:
    raise SystemExit("ADR must cite C-5f Seatbelt")

envr = Path("docs/env-registry.md").read_text(encoding="utf-8")
if "SYX_EMBED_MACOS_SANDBOX" not in envr or "SYX_MACRO_MACOS_SANDBOX" not in envr:
    raise SystemExit("env-registry must register MACOS_SANDBOX envs")

cl = Path("CHANGELOG.md").read_text(encoding="utf-8")
head = cl.split("### SYX 0.4.0", 1)[-1].split("###", 1)[0]
if "C-5f" not in head:
    raise SystemExit("CHANGELOG must mention C-5f")

print("c5f_native_real_count=16")
print("c5f_tls_server_partial=1")
print("c5f_census_doc=1")
print("c5f_ssot_updated=1")
print("c5f_roadmap_ok=1")
PY

echo "== nails cargo =="
cargo test -p syx-macro c5f_ -- --nocapture >/tmp/c5f_macro.txt 2>&1 \
  || die "c5f macro unit failed"
ok "c5f_macro_unit" "1"

cargo test -p syx-driver --test evergreen c5f_ -- --nocapture >/tmp/c5f_ever.txt 2>&1 \
  || die "c5f evergreen failed"
if ! grep -q "c5f_macos_seatbelt_hooks_and_fail_closed ... ok" /tmp/c5f_ever.txt; then
  die "evergreen nail did not pass"
fi
ok "c5f_evergreen" "1"
tail -16 /tmp/c5f_ever.txt

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c5f_book_out.txt 2>&1 || die "opt-in output pins failed"
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
  echo "c5f_macos_sandbox_thin_census_ok=0"
  exit 1
fi
echo "c5f_macos_sandbox_thin_census_ok=1"
echo "current=C-5f-macos-sandbox-thin"
echo "next=C-5g"
