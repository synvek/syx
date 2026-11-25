#!/usr/bin/env bash
# C-5h permission-sandbox default-on census.
# Soft park: C-5 [x] · true full syscall GA · tls.server · R-50 · full MirOp.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-5h permission-sandbox default-on census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "default-on API / CLI broken"
from pathlib import Path

cfg = Path("crates/syx-config/src/lib.rs").read_text(encoding="utf-8")
for needle in (
    'env_falsey_opt_out("SYX_PERMISSION_SANDBOX", true)',
    "C-5h",
    "PermissionGrants",
):
    if needle not in cfg:
        raise SystemExit(f"syx-config missing {needle}")

cli = Path("crates/syx-cli/src/main.rs").read_text(encoding="utf-8")
for needle in (
    "no-permission-sandbox",
    "no_permission_sandbox",
    "C-5h",
    "permission_sandbox",
):
    if needle not in cli:
        raise SystemExit(f"syx-cli missing {needle}")

sem = Path("crates/syx-semantic/src/semantic_infer_host.rs").read_text(encoding="utf-8")
if "no-permission-sandbox" not in sem or "S024" not in sem:
    raise SystemExit("semantic must mention opt-out in S024 path")

print("c5h_api=1")
print("c5h_cli=1")
PY

python3 - <<'PY' || die "neg pin missing"
from pathlib import Path
grants = Path("crates/syx-driver/tests/security/permission_grants.rs").read_text(encoding="utf-8")
for needle in (
    "m3_e02_permission_grants_default_on_s024",
    "c5h_permission_sandbox_opt_out_allows_tcp",
):
    if needle not in grants:
        raise SystemExit(f"permission_grants missing {needle}")
if "default_off_unchanged" in grants:
    raise SystemExit("default_off_unchanged must be retired")

fs = Path("crates/syx-driver/tests/security/permission_fs.rs").read_text(encoding="utf-8")
if "c5h_permission_default_on_fs_s024" not in fs:
    raise SystemExit("permission_fs missing c5h fs S024 nail")

smoke = Path("crates/syx-driver/tests/evergreen/smoke.rs").read_text(encoding="utf-8")
if "c5h_permission_sandbox_default_on_and_opt_out" not in smoke:
    raise SystemExit("evergreen missing c5h nail")
print("c5h_neg_pin=1")
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
    native = nm.group(1)
    if native == "real":
        real.add(cid)
    if cid == "net.tls.server" and native == "partial":
        tls_server_partial = True
if real != want_real:
    raise SystemExit(f"native=real drift: {sorted(real)}")
if not tls_server_partial:
    raise SystemExit("net.tls.server must remain native=partial")

ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
next_sec = ssot.split("## Next", 1)[-1]
if "跨轴" not in next_sec and "C-3" not in next_sec:
    raise SystemExit("SSOT Next must be cross-axis (not auto C-5 [x])")
if "C-5 `[x]`" in next_sec or "claim C-5 `[x]`" in next_sec.lower():
    # allow the phrase "不宣称 / 不自动 C-5 [x]"
    if "不" not in next_sec and "not" not in next_sec.lower() and "≠" not in next_sec:
        raise SystemExit("SSOT must not claim C-5 [x]")
if "+++" in next_sec:
    raise SystemExit("SSOT Next must not use +++")

rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("**C-5h**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-5h [x]")
if "### C-5 ·" not in rm:
    raise SystemExit("02 must keep C-5 section")
# C-5 axis must remain [~]
c5_header = [l for l in rm.splitlines() if l.startswith("### C-5 ·")]
if not c5_header or "[~]" not in c5_header[0]:
    raise SystemExit("02 must keep C-5 as [~]")
if "跨轴" not in rm and "不自动 C-5" not in rm:
    raise SystemExit("02 must set 默认下一 跨轴 / 不自动 C-5 [x]")

st = Path("plan/01-project-status.md").read_text(encoding="utf-8")
if "C-5h" not in st:
    raise SystemExit("01 must cite C-5h")
if "跨轴" not in st:
    raise SystemExit("01 must set next 跨轴")

adr = Path("docs/adr/macro-embed-threat-model.md").read_text(encoding="utf-8")
adr_l = adr.lower()
if "C-5h" not in adr or ("default-on" not in adr_l and "default on" not in adr_l):
    raise SystemExit("ADR must cite C-5h default-on")
if "no-permission-sandbox" not in adr:
    raise SystemExit("ADR must cite opt-out path")

envr = Path("docs/env-registry.md").read_text(encoding="utf-8")
if "SYX_PERMISSION_SANDBOX" not in envr or "default-on" not in envr:
    raise SystemExit("env-registry must mark SYX_PERMISSION_SANDBOX default-on")

pol = Path("docs/capability-sandbox-policy-v0.md").read_text(encoding="utf-8")
if "C-5h" not in pol or "no-permission-sandbox" not in pol:
    raise SystemExit("capability-sandbox-policy must cite C-5h opt-out")

cl = Path("CHANGELOG.md").read_text(encoding="utf-8")
head = cl.split("### SYX 0.4.0", 1)[-1].split("###", 1)[0]
if "C-5h" not in head:
    raise SystemExit("CHANGELOG must mention C-5h")

print("c5h_native_real_count=16")
print("c5h_tls_server_partial=1")
print("c5h_census_doc=1")
print("c5h_ssot_updated=1")
print("c5h_roadmap_ok=1")
PY

echo "== nails cargo =="
cargo test -p syx-config config_permission_sandbox_default_on_c5h -- --nocapture >/tmp/c5h_cfg.txt 2>&1 \
  || die "c5h config unit failed"
ok "c5h_config_unit" "1"

cargo test -p syx-driver --test security c5h_ -- --nocapture >/tmp/c5h_sec.txt 2>&1 \
  || die "c5h security nails failed"
cargo test -p syx-driver --test security permission_grants_default_on_s024 -- --nocapture \
  >>/tmp/c5h_sec.txt 2>&1 \
  || die "default_on_s024 nail failed"
if ! grep -q "c5h_permission_sandbox_opt_out_allows_tcp ... ok" /tmp/c5h_sec.txt; then
  die "security opt-out nail did not pass"
fi
if ! grep -q "m3_e02_permission_grants_default_on_s024 ... ok" /tmp/c5h_sec.txt; then
  die "default_on S024 nail did not pass"
fi
ok "c5h_security" "1"
tail -24 /tmp/c5h_sec.txt

cargo test -p syx-driver --test evergreen c5h_ -- --nocapture >/tmp/c5h_ever.txt 2>&1 \
  || die "c5h evergreen failed"
if ! grep -q "c5h_permission_sandbox_default_on_and_opt_out ... ok" /tmp/c5h_ever.txt; then
  die "evergreen nail did not pass"
fi
ok "c5h_evergreen" "1"
tail -16 /tmp/c5h_ever.txt

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c5h_book_out.txt 2>&1 || die "opt-in output pins failed"
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
  echo "C-5h census FAILED" >&2
  exit 1
fi
echo "C-5h census OK"
ok "c5h_exit" "0"
