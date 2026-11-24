#!/usr/bin/env bash
# C-4c publish-requires-usable-lock census.
# Soft park: C-4d lock v16 trim · path hash · ModuleId · tls.server · macOS · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-4c publish-lock census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "require_publish_lock API contract broken"
from pathlib import Path

pkg = Path("crates/syx-pkg/src/package.rs").read_text(encoding="utf-8")
if "pub fn require_publish_lock" not in pkg:
    raise SystemExit("missing require_publish_lock")
if "P014" not in pkg or "C-4c" not in pkg:
    raise SystemExit("P014 / C-4c missing in package.rs")
if "enforce_lock_v16" not in pkg:
    raise SystemExit("publish lock must enforce_lock_v16")
if "require_publish_lock(root)" not in pkg:
    raise SystemExit("publish_package must call require_publish_lock")

lib = Path("crates/syx-pkg/src/lib.rs").read_text(encoding="utf-8")
if "pub fn require_publish_lock" not in lib:
    raise SystemExit("lib.rs must export require_publish_lock")

cli = Path("crates/syx-cli/src/main.rs").read_text(encoding="utf-8")
if "require_publish_lock" not in cli or "dry_run" not in cli:
    raise SystemExit("CLI dry_run must call require_publish_lock")

integ = Path("crates/syx-pkg/tests/integration.rs").read_text(encoding="utf-8")
for needle in (
    "c4c_publish_without_lock_rejected",
    "c4c_publish_after_install_ok",
    "c4c_require_publish_lock_api",
):
    if needle not in integ:
        raise SystemExit(f"integration missing {needle}")

ev = Path("crates/syx-driver/tests/evergreen/registry.rs").read_text(encoding="utf-8")
if "ensure_publish_lock" not in ev:
    raise SystemExit("evergreen registry must seed publish lock")

print("c4c_api=1")
print("c4c_tests_present=1")
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

census = Path("docs/census/c-4c-publish-lock-census.md")
if not census.is_file():
    raise SystemExit("missing c-4c census")
ct = census.read_text(encoding="utf-8")
for needle in ("C-4c", "C-4d", "P014", "require_publish_lock", "enforce_lock_v16"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")

ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
if "C-4c (done" not in ssot:
    raise SystemExit("SSOT must mark C-4c done")
next_sec = ssot.split("## Next", 1)[-1]
if "C-4d" not in next_sec:
    raise SystemExit("SSOT Next must be C-4d")
if "+++" in next_sec:
    raise SystemExit("SSOT Next must not use +++")

rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("**C-4c**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-4c [x]")
if not any("### C-4 ·" in l and "[~]" in l for l in rm.splitlines()):
    raise SystemExit("02 must keep C-4 as [~]")
if not any("**C-4d**" in l and "[ ]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-4d [ ] as next slice")
if "默认下一" not in rm or "C-4d" not in rm:
    raise SystemExit("02 must set 默认下一 C-4d")
if "### C-3 ·" not in rm or "[~]" not in rm:
    raise SystemExit("02 must keep C-3 as [~]")

st = Path("plan/01-project-status.md").read_text(encoding="utf-8")
if "C-4c" not in st or "C-4d" not in st:
    raise SystemExit("01 must cite C-4c / next C-4d")

cl = Path("CHANGELOG.md").read_text(encoding="utf-8")
head = cl.split("### SYX 0.4.0", 1)[-1].split("###", 1)[0]
if "C-4c" not in head:
    raise SystemExit("CHANGELOG 0.4.0 must mention C-4c")

print("c4c_native_real_count=16")
print("c4c_tls_server_partial=1")
print("c4c_census_doc=1")
print("c4c_ssot_updated=1")
print("c4c_roadmap_ok=1")
PY

echo "== c4c integration =="
cargo test -p syx-pkg --test integration -- --test-threads=1 c4c_ >/tmp/c4c_integ.txt 2>&1 \
  || die "c4c integration failed"
for needle in \
  c4c_publish_without_lock_rejected \
  c4c_publish_after_install_ok \
  c4c_require_publish_lock_api
do
  if ! grep -q "$needle" /tmp/c4c_integ.txt; then
    die "integration missing $needle"
  fi
done
if ! grep -qE "test result: ok\. 4 passed" /tmp/c4c_integ.txt; then
  die "expected 4 passed c4c tests"
fi
ok "c4c_integ_cargo" "1"
tail -15 /tmp/c4c_integ.txt

echo "== evergreen registry publish smoke =="
cargo test -p syx-driver --test evergreen -- --test-threads=1 \
  registry::publish_file_registry \
  registry::registry_unsigned_publish_rejected \
  registry::publish_install_roundtrip \
  >/tmp/c4c_evergreen.txt 2>&1 \
  || die "evergreen publish smoke failed"
if ! grep -qE "test result: ok\. 3 passed" /tmp/c4c_evergreen.txt; then
  die "expected 3 passed evergreen publish smokes"
fi
ok "c4c_evergreen_smoke" "1"
tail -12 /tmp/c4c_evergreen.txt

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c4c_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c4c_book_out.txt
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
  echo "c4c_publish_lock_census_ok=0"
  exit 1
fi
echo "c4c_publish_lock_census_ok=1"
echo "current=C-4c-publish-lock"
echo "next=C-4d"
