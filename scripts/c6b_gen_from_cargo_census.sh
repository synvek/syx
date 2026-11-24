#!/usr/bin/env bash
# C-6b gen-from-Cargo census.
# Soft park: ModuleId · C-5c+ · tls.server · macOS · R-50 · C-7 implement.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-6b gen-from-Cargo census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "gen source contract / Cargo SSOT broken"
from pathlib import Path
import re

# Shared helper must exist and parse V
import sys
sys.path.insert(0, "scripts")
from syx_cargo_ssot import read_workspace_semver, read_toml_syx_version

V = read_workspace_semver(Path("."))
print(f"workspace_semver={V}")

# Gen scripts: Cargo inject + no 117.0 fossil fallback
for rel in ("scripts/gen_capabilities.py", "scripts/gen_ir_capabilities.py"):
    t = Path(rel).read_text(encoding="utf-8")
    if "syx_cargo_ssot" not in t and "read_workspace_semver" not in t:
        raise SystemExit(f"{rel}: must use Cargo SSOT helper")
    if 'meta.get("syx_version", "117.0")' in t or ', "117.0")' in t:
        raise SystemExit(f"{rel}: still has 117.0 fossil default")
    if '"117.0"' in t and "no 117.0" not in t and "fossil" not in t.lower():
        # allow comment mentioning removal; disallow string default
        if re.search(r'default.*117\.0|,\s*"117\.0"\)', t):
            raise SystemExit(f"{rel}: 117.0 default still present")

# gen_ir must pass syx_version into build_table (no silent fallback)
ir = Path("scripts/gen_ir_capabilities.py").read_text(encoding="utf-8")
if "syx_version: str" not in ir and "syx_version," not in ir:
    raise SystemExit("gen_ir_capabilities.py: build_table must take syx_version")
if 'meta.get("syx_version"' in ir:
    raise SystemExit("gen_ir_capabilities.py: must not read syx_version from meta with fallback")

# TOMLs aligned to V
for toml_path in (
    "packages/internal/capabilities.toml",
    "packages/internal/host_fn_capabilities.toml",
    "packages/internal/mir_op_capabilities.toml",
):
    tv = read_toml_syx_version(Path(toml_path))
    if tv != V:
        raise SystemExit(f"{toml_path}: syx_version={tv!r} != V={V!r}")

print("c6b_source_contract=1")
print("c6b_toml_aligned=1")
PY

python3 scripts/gen_capabilities.py --check >/tmp/c6b_gen_caps.txt 2>&1 \
  || die "gen_capabilities --check failed"
python3 scripts/gen_ir_capabilities.py --check >/tmp/c6b_gen_ir.txt 2>&1 \
  || die "gen_ir_capabilities --check failed"
ok "c6b_gen_check" "1"

python3 - <<'PY' || die "negative TOML drift --check must fail"
"""Forge stale syx_version → --check non-zero; restore backups."""
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, "scripts")
from syx_cargo_ssot import (
    read_toml_syx_version,
    read_workspace_semver,
    rewrite_toml_syx_version,
)

ROOT = Path(".").resolve()
V = read_workspace_semver(ROOT)
paths = [
    ROOT / "packages/internal/capabilities.toml",
    ROOT / "packages/internal/mir_op_capabilities.toml",
    ROOT / "packages/internal/host_fn_capabilities.toml",
]
backups = {p: p.read_text(encoding="utf-8") for p in paths}
ok_neg = True
try:
    for p in paths:
        rewrite_toml_syx_version(p, "0.0.0-stale")
    r1 = subprocess.run(
        [sys.executable, "scripts/gen_capabilities.py", "--check"], cwd=ROOT
    )
    r2 = subprocess.run(
        [sys.executable, "scripts/gen_ir_capabilities.py", "--check"], cwd=ROOT
    )
    if r1.returncode == 0:
        print("NEG FAIL: gen_capabilities --check should fail", file=sys.stderr)
        ok_neg = False
    if r2.returncode == 0:
        print("NEG FAIL: gen_ir_capabilities --check should fail", file=sys.stderr)
        ok_neg = False
finally:
    for p, text in backups.items():
        p.write_text(text, encoding="utf-8")
    for p in paths:
        if read_toml_syx_version(p) != V:
            raise SystemExit(f"restore failed: {p}")

r3 = subprocess.run(
    [sys.executable, "scripts/gen_capabilities.py", "--check"], cwd=ROOT
)
r4 = subprocess.run(
    [sys.executable, "scripts/gen_ir_capabilities.py", "--check"], cwd=ROOT
)
if not ok_neg or r3.returncode != 0 or r4.returncode != 0:
    raise SystemExit("negative drift probe failed")
print("c6b_negative_check=1")
print("c6b_post_restore_check=1")
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

census = Path("docs/census/c-6b-gen-from-cargo-census.md")
if not census.is_file():
    raise SystemExit("missing c-6b census")
ct = census.read_text(encoding="utf-8")
for needle in ("C-6b", "C-7", "Cargo", "117.0", "c6_exited"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")

ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
if "C-6b (done" not in ssot:
    raise SystemExit("SSOT must mark C-6b done")
if "C-6 `[x]`" not in ssot:
    raise SystemExit("SSOT must claim C-6 [x]")
next_sec = ssot.split("## Next", 1)[-1]
if "C-7" not in next_sec:
    raise SystemExit("SSOT Next must be C-7")
if "+++" in next_sec:
    raise SystemExit("SSOT Next must not use +++")

rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("**C-6b**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-6b [x]")
if not any(l.startswith("### C-6 ·") and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-6 as [x]")
if "默认下一 = **C-7**" not in rm:
    raise SystemExit("02 must set default next to C-7")

st = Path("plan/01-project-status.md").read_text(encoding="utf-8")
if "C-6b" not in st or "C-7" not in st:
    raise SystemExit("01 must cite C-6b / next C-7")
if "C-6 `[x]`" not in st and "**C-6** `[x]`" not in st:
    raise SystemExit("01 must claim C-6 exited")

cl = Path("CHANGELOG.md").read_text(encoding="utf-8")
head = cl.split("### SYX 0.4.0", 1)[-1].split("###", 1)[0]
if "C-6b" not in head:
    raise SystemExit("CHANGELOG 0.4.0 must mention C-6b")

print("c6b_native_real_count=16")
print("c6b_tls_server_partial=1")
print("c6b_census_doc=1")
print("c6b_ssot_updated=1")
print("c6b_roadmap_ok=1")
print("c6_exited=1")
PY

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c6b_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c6b_book_out.txt
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
  echo "c6b_gen_from_cargo_census_ok=0"
  exit 1
fi
echo "c6b_gen_from_cargo_census_ok=1"
echo "current=C-6b"
echo "next=C-7"
echo "c6_exited=1"
