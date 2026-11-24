#!/usr/bin/env bash
# C-4a git rev pin + locked git install census.
# Soft park: C-4b default-locked · publish lock · lock v16 trim · ModuleId · tls.server · macOS · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-4a git-rev-pin census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "git rev / locked git API contract broken"
from pathlib import Path

git = Path("crates/syx-pkg/src/sources/git.rs").read_text(encoding="utf-8")
if 'unwrap_or("main")' in git or "unwrap_or('main')" in git:
    raise SystemExit("git.rs must not default rev to main")
if "P043" not in git or "git rev required" not in git or "C-4a" not in git:
    raise SystemExit("P043 / git rev required / C-4a missing in git.rs")
if "resolve_git_dependency_locked" not in git:
    raise SystemExit("missing resolve_git_dependency_locked")
if "install_locked" not in git:
    raise SystemExit("locked path must consult install_locked")
if "P014" not in git:
    raise SystemExit("locked git must emit P014")
if "revs_equal" not in git:
    raise SystemExit("locked path must compare HEAD vs lock rev")

integ = Path("crates/syx-pkg/tests/integration.rs").read_text(encoding="utf-8")
for needle in (
    "c4a_git_manifest_missing_rev_rejected",
    "c4a_locked_git_reuses_pin",
    "c4a_locked_git_rejects_missing_or_wrong_pin",
    "c04_locked_install_reuses_pin",
):
    if needle not in integ:
        raise SystemExit(f"integration missing {needle}")

print("c4a_git_api=1")
print("c4a_tests_present=1")
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

census = Path("docs/census/c-4a-git-rev-pin-census.md")
if not census.is_file():
    raise SystemExit("missing c-4a census")
ct = census.read_text(encoding="utf-8")
for needle in ("C-4a", "C-4b", "P043", "P014", "SYX_INSTALL_LOCKED", "git rev"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")

ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
if "C-4a (done" not in ssot:
    raise SystemExit("SSOT must mark C-4a done")
next_sec = ssot.split("## Next", 1)[-1]
if "C-4b" not in next_sec:
    raise SystemExit("SSOT Next must be C-4b")
if "+++" in next_sec:
    raise SystemExit("SSOT Next must not use +++")

rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("**C-4a**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-4a [x]")
if not any("### C-4 ·" in l and "[~]" in l for l in rm.splitlines()):
    raise SystemExit("02 must keep C-4 as [~]")
if not any("**C-4b**" in l and "[ ]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-4b [ ] as next slice")
if "默认下一" not in rm or "C-4b" not in rm:
    raise SystemExit("02 must set 默认下一 C-4b")
if "### C-3 ·" not in rm or "[~]" not in rm:
    raise SystemExit("02 must keep C-3 as [~]")

st = Path("plan/01-project-status.md").read_text(encoding="utf-8")
if "C-4a" not in st or "C-4b" not in st:
    raise SystemExit("01 must cite C-4a / next C-4b")

cl = Path("CHANGELOG.md").read_text(encoding="utf-8")
head = cl.split("### SYX 0.4.0", 1)[-1].split("###", 1)[0]
if "C-4a" not in head:
    raise SystemExit("CHANGELOG 0.4.0 must mention C-4a")

print("c4a_native_real_count=16")
print("c4a_tls_server_partial=1")
print("c4a_census_doc=1")
print("c4a_ssot_updated=1")
print("c4a_roadmap_ok=1")
PY

echo "== c4a + c04 integration =="
cargo test -p syx-pkg --test integration -- --test-threads=1 c4a_ c04_locked >/tmp/c4a_integ.txt 2>&1 \
  || die "c4a/c04 integration failed"
# Match test names + overall result (git may interleave lines before quieting).
for needle in \
  c4a_git_manifest_missing_rev_rejected \
  c4a_locked_git_reuses_pin \
  c4a_locked_git_rejects_missing_or_wrong_pin \
  c04_locked_install_reuses_pin
do
  if ! grep -q "$needle" /tmp/c4a_integ.txt; then
    die "integration missing $needle"
  fi
done
if ! grep -qE "test result: ok\. 9 passed" /tmp/c4a_integ.txt; then
  die "expected 9 passed c4a+c04 tests"
fi
ok "c4a_integ_cargo" "1"
tail -20 /tmp/c4a_integ.txt

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c4a_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c4a_book_out.txt
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
  echo "c4a_git_rev_pin_census_ok=0"
  exit 1
fi
echo "c4a_git_rev_pin_census_ok=1"
echo "current=C-4a-git-rev-pin"
echo "next=C-4b"
