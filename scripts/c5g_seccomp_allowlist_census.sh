#!/usr/bin/env bash
# C-5g Linux seccomp allowlist thin census.
# Soft park: C-5h permission-sandbox default-on · tls.server · R-50.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-5g seccomp allowlist census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "allowlist API / hooks broken"
from pathlib import Path

filt = Path("crates/syx-macro/src/seccomp_linux.rs").read_text(encoding="utf-8")
for needle in (
    "try_install_seccomp_allowlist",
    "SECCOMP_RET_ALLOW",
    "SECCOMP_ALLOWLIST_SYSCALL_NAMES",
    "seccomp-allowlist-wired",
    "futex",
    "C-5g",
):
    if needle not in filt:
        raise SystemExit(f"seccomp_linux missing {needle}")

embed = Path("crates/syx-driver/src/embed_v2.rs").read_text(encoding="utf-8")
if "embed_seccomp_allowlist_enabled" not in embed or "SYX_EMBED_SECCOMP_ALLOWLIST" not in embed:
    raise SystemExit("embed must wire SECCOMP_ALLOWLIST")
if "try_install_seccomp_allowlist" not in embed or "C-5g" not in embed:
    raise SystemExit("embed must call try_install_seccomp_allowlist")

mac = Path("crates/syx-macro/src/external_sandbox.rs").read_text(encoding="utf-8")
if "SYX_MACRO_SECCOMP_ALLOWLIST" not in mac or "try_install_seccomp_allowlist" not in mac:
    raise SystemExit("macro must wire allowlist")
if "mutually exclusive" not in mac:
    raise SystemExit("macro must mutex SECCOMP vs ALLOWLIST")

print("c5g_api=1")
print("c5g_hooks=1")
PY

python3 - <<'PY' || die "neg pin missing"
from pathlib import Path
smoke = Path("crates/syx-driver/tests/evergreen/smoke.rs").read_text(encoding="utf-8")
for needle in ("c5g_seccomp_allowlist_hooks_and_mutex", "C-5g", "SECCOMP_ALLOWLIST"):
    if needle not in smoke:
        raise SystemExit(f"evergreen missing {needle}")
filt = Path("crates/syx-macro/src/seccomp_linux.rs").read_text(encoding="utf-8")
if "c5g_non_linux_allowlist_fails_closed" not in filt:
    raise SystemExit("macro unit nail missing")
print("c5g_neg_pin=1")
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

census = Path("docs/census/c-5g-seccomp-allowlist-census.md")
if not census.is_file():
    raise SystemExit("missing census doc")
ct = census.read_text(encoding="utf-8")
for needle in ("C-5g", "C-5h", "allowlist", "default-deny"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")

ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
if "C-5g (done" not in ssot:
    raise SystemExit("SSOT must mark C-5g done")
next_sec = ssot.split("## Next", 1)[-1]
if "C-5h" not in next_sec:
    raise SystemExit("SSOT Next must be C-5h")
if "+++" in next_sec:
    raise SystemExit("SSOT Next must not use +++")

rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("**C-5g**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-5g [x]")
if "默认下一" not in rm or "C-5h" not in rm:
    raise SystemExit("02 must set 默认下一 C-5h")
if "### C-5 ·" not in rm or "[~]" not in rm:
    raise SystemExit("02 must keep C-5 as [~]")

st = Path("plan/01-project-status.md").read_text(encoding="utf-8")
if "C-5h" not in st or "C-5g" not in st:
    raise SystemExit("01 must cite C-5g / next C-5h")

adr = Path("docs/adr/macro-embed-threat-model.md").read_text(encoding="utf-8")
if "C-5g" not in adr or "allowlist" not in adr.lower():
    raise SystemExit("ADR must cite C-5g allowlist")

envr = Path("docs/env-registry.md").read_text(encoding="utf-8")
if "SYX_EMBED_SECCOMP_ALLOWLIST" not in envr or "SYX_MACRO_SECCOMP_ALLOWLIST" not in envr:
    raise SystemExit("env-registry must register ALLOWLIST envs")

cl = Path("CHANGELOG.md").read_text(encoding="utf-8")
head = cl.split("### SYX 0.4.0", 1)[-1].split("###", 1)[0]
if "C-5g" not in head:
    raise SystemExit("CHANGELOG must mention C-5g")

print("c5g_native_real_count=16")
print("c5g_tls_server_partial=1")
print("c5g_census_doc=1")
print("c5g_ssot_updated=1")
print("c5g_roadmap_ok=1")
PY

echo "== nails cargo =="
cargo test -p syx-macro c5g_ -- --nocapture >/tmp/c5g_macro.txt 2>&1 \
  || die "c5g macro unit failed"
ok "c5g_macro_unit" "1"

cargo test -p syx-driver --test evergreen c5g_ -- --nocapture >/tmp/c5g_ever.txt 2>&1 \
  || die "c5g evergreen failed"
if ! grep -q "c5g_seccomp_allowlist_hooks_and_mutex ... ok" /tmp/c5g_ever.txt; then
  die "evergreen nail did not pass"
fi
ok "c5g_evergreen" "1"
tail -16 /tmp/c5g_ever.txt

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c5g_book_out.txt 2>&1 || die "opt-in output pins failed"
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
  echo "c5g_seccomp_allowlist_census_ok=0"
  exit 1
fi
echo "c5g_seccomp_allowlist_census_ok=1"
echo "current=C-5g-seccomp-allowlist"
echo "next=C-5h"
