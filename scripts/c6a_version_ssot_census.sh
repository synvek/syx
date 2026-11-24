#!/usr/bin/env bash
# C-6a version/capability SSOT census.
# Soft park: C-6b gen-from-Cargo · ModuleId · C-5c+ · tls.server · macOS · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-6a version-ssot census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "version SSOT surface contract broken"
from pathlib import Path
import re
import json

cargo = Path("Cargo.toml").read_text(encoding="utf-8")
m = re.search(r'\[workspace\.package\][\s\S]*?^version\s*=\s*"([^"]+)"', cargo, re.M)
if not m:
    raise SystemExit("missing workspace.package version")
V = m.group(1)
print(f"workspace_semver={V}")

def must_contain(path: str, needle: str):
    t = Path(path).read_text(encoding="utf-8")
    if needle not in t:
        raise SystemExit(f"{path}: missing {needle!r}")

for p in (
    "docs/README.md",
    "docs/zh/README.zh-CN.md",
    "books/README.md",
    "docs/adr/current.md",
):
    must_contain(p, V)

# plan/01 version table
p01 = Path("plan/01-project-status.md").read_text(encoding="utf-8")
if f"| Crate / workspace | **{V}**" not in p01 and f"**{V}**" not in p01.split("## 2.", 1)[-1][:800]:
    # Require explicit workspace row
    if not re.search(rf"Crate / workspace \| \*\*{re.escape(V)}\*\*", p01):
        raise SystemExit(f"plan/01 must declare Crate/workspace {V}")

for toml_path in (
    "packages/internal/capabilities.toml",
    "packages/internal/host_fn_capabilities.toml",
    "packages/internal/mir_op_capabilities.toml",
):
    must_contain(toml_path, f'syx_version = "{V}"')

manifest = json.loads(Path("packages/internal/manifest.json").read_text(encoding="utf-8"))
if manifest.get("version") != V:
    raise SystemExit(f"manifest.json version {manifest.get('version')!r} != {V}")

# credibility must not hardcode obsolete 0.2.3
for rel in (
    "crates/syx-driver/tests/credibility/inject_cargo_pkg_version_contract.rs",
    "crates/syx-driver/tests/credibility/version_claim_contract.rs",
    "crates/syx-driver/tests/credibility/r29_lang_super_contract.rs",
):
    t = Path(rel).read_text(encoding="utf-8")
    if '0.2.3' in t:
        raise SystemExit(f"{rel}: still hardcodes 0.2.3")
    if "CARGO_PKG_VERSION" not in t and "env!" not in t:
        # r29 uses env!("CARGO_PKG_VERSION")
        if 'env!("CARGO_PKG_VERSION")' not in t:
            raise SystemExit(f"{rel}: must key off CARGO_PKG_VERSION")

# generated JSON meta
for gj in (
    "packages/internal/capabilities.generated.json",
    "packages/internal/host_fn_capabilities.generated.json",
    "packages/internal/mir_op_capabilities.generated.json",
):
    d = json.loads(Path(gj).read_text(encoding="utf-8"))
    meta = d.get("meta") or d
    sv = meta.get("syx_version") if isinstance(meta, dict) else None
    if sv is None and isinstance(d, dict):
        sv = d.get("syx_version")
    # capabilities.generated.json structure: {"meta": {"syx_version": ...}}
    if isinstance(d.get("meta"), dict):
        sv = d["meta"].get("syx_version")
    if sv != V:
        raise SystemExit(f"{gj}: syx_version {sv!r} != {V}")

print("c6a_ssot_parse=1")
print("c6a_surfaces_aligned=1")
print("c6a_credibility_clean=1")
PY

python3 scripts/gen_capabilities.py --check >/tmp/c6a_gen_caps.txt 2>&1 \
  || die "gen_capabilities --check failed"
python3 scripts/gen_ir_capabilities.py --check >/tmp/c6a_gen_ir.txt 2>&1 \
  || die "gen_ir_capabilities --check failed"
ok "c6a_gen_check" "1"

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

census = Path("docs/census/c-6a-version-ssot-census.md")
if not census.is_file():
    raise SystemExit("missing c-6a census")
ct = census.read_text(encoding="utf-8")
for needle in ("C-6a", "C-6b", "workspace", "SSOT"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")

ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
if "C-6a (done" not in ssot:
    raise SystemExit("SSOT must mark C-6a done")
next_sec = ssot.split("## Next", 1)[-1]
if "C-6b" not in next_sec:
    raise SystemExit("SSOT Next must be C-6b")
if "+++" in next_sec:
    raise SystemExit("SSOT Next must not use +++")

rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("**C-6a**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-6a [x]")
if not any("### C-6 ·" in l and "[~]" in l for l in rm.splitlines()):
    raise SystemExit("02 must keep C-6 as [~]")
if not any("**C-6b**" in l and "[ ]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-6b [ ] as next")

st = Path("plan/01-project-status.md").read_text(encoding="utf-8")
if "C-6a" not in st or "C-6b" not in st:
    raise SystemExit("01 must cite C-6a / next C-6b")

cl = Path("CHANGELOG.md").read_text(encoding="utf-8")
head = cl.split("### SYX 0.4.0", 1)[-1].split("###", 1)[0]
if "C-6a" not in head:
    raise SystemExit("CHANGELOG 0.4.0 must mention C-6a")

print("c6a_native_real_count=16")
print("c6a_tls_server_partial=1")
print("c6a_census_doc=1")
print("c6a_ssot_updated=1")
print("c6a_roadmap_ok=1")
PY

echo "== c6a credibility =="
cargo test -p syx-driver --test credibility -- --test-threads=1 \
  s2_07_inject_cargo_pkg_version_locked \
  r29_manifest_phase_is_139 \
  >/tmp/c6a_cred.txt 2>&1 \
  || die "credibility version nails failed"
# Also run version_claim if filter matches
cargo test -p syx-driver --test credibility version_claim -- --test-threads=1 \
  >/tmp/c6a_cred2.txt 2>&1 || true
ok "c6a_credibility" "1"
tail -20 /tmp/c6a_cred.txt

bash scripts/s2_07_version_census.sh >/tmp/c6a_s207.txt 2>&1 || die "s2_07 census failed"
ok "c6a_s207" "1"

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
# CLI version must match workspace
cli_v="$("$SYX" --version 2>/dev/null | head -1 || true)"
ok "c6a_cli_version_line" "$cli_v"
python3 - <<PY || die "CLI version must contain workspace V"
import re, sys
from pathlib import Path
cargo = Path("Cargo.toml").read_text(encoding="utf-8")
m = re.search(r'\[workspace\.package\][\s\S]*?^version\s*=\s*"([^"]+)"', cargo, re.M)
V = m.group(1)
line = """$cli_v"""
if V not in line:
    raise SystemExit(f"CLI --version {line!r} missing {V}")
print("c6a_cli_matches_ssot=1")
PY

python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c6a_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c6a_book_out.txt
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
  echo "c6a_version_ssot_census_ok=0"
  exit 1
fi
echo "c6a_version_ssot_census_ok=1"
echo "current=C-6a-version-ssot"
echo "next=C-6b"
