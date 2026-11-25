#!/usr/bin/env bash
# C-8a Ch15 implicit-super MD honesty census.
# Soft park: C-8b · C-8c · CallSuper · C-7c+ · ModuleId · C-5c+ · tls.server · macOS · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-8a Ch15 implicit-super MD census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "Ch15 MD honesty contract broken"
from pathlib import Path

en = Path("books/language-guide/15-initialization.md").read_text(encoding="utf-8")
zh = Path("books/zh-cn/language-guide/15-initialization.md").read_text(encoding="utf-8")

for bad in ("the compiler inserts it", "automatically inserts"):
    if bad in en:
        raise SystemExit(f"en Ch15 still contains {bad!r}")
if "编译器自动插入" in zh:
    raise SystemExit("zh Ch15 still contains 编译器自动插入")

if "≠ CallSuper" not in en and "≠ `CallSuper`" not in en:
    raise SystemExit("en Ch15 must state ≠ CallSuper")
if "≠ CallSuper" not in zh and "≠ `CallSuper`" not in zh:
    raise SystemExit("zh Ch15 must state ≠ CallSuper")

if "Not implemented" not in en or "CallSuper" not in en:
    raise SystemExit("en Ch15 must mark CallSuper not implemented")
if "未实现" not in zh or "CallSuper" not in zh:
    raise SystemExit("zh Ch15 must mark CallSuper 未实现")

if "U-38" not in en:
    raise SystemExit("en Ch15 must cite U-38")
if "U-38" not in zh:
    raise SystemExit("zh Ch15 must cite U-38")

print("c8a_md_en_ok=1")
print("c8a_md_zh_ok=1")
PY

cargo test -p syx-driver --test lang c8a_ch15 -- --test-threads=1 \
  >/tmp/c8a_nails.txt 2>&1 || die "c8a lang honesty nails failed"
ok "c8a_honesty_nails" "1"
tail -8 /tmp/c8a_nails.txt

python3 - <<'PY' || die "native=real / ledger hygiene failed"
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

census = Path("docs/census/c-8a-ch15-implicit-super-md-census.md")
if not census.is_file():
    raise SystemExit("missing c-8a census")
ct = census.read_text(encoding="utf-8")
for needle in ("C-8a", "C-8b", "CallSuper", "U-38", "c8_exited"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")

ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
if "C-8a (done" not in ssot:
    raise SystemExit("SSOT must mark C-8a done")
if "C-8 remains `[~]`" not in ssot:
    raise SystemExit("SSOT must keep C-8 as [~]")
next_sec = ssot.split("## Next", 1)[-1]
if "C-8b" not in next_sec:
    raise SystemExit("SSOT Next must be C-8b")
if "+++" in next_sec:
    raise SystemExit("SSOT Next must not use +++")

rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("**C-8a**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-8a [x]")
if not any(l.startswith("### C-8 ·") and "[~]" in l for l in rm.splitlines()):
    raise SystemExit("02 must keep C-8 as [~]")
if "默认下一 = **C-8b**" not in rm:
    raise SystemExit("02 must set default next to C-8b")

st = Path("plan/01-project-status.md").read_text(encoding="utf-8")
if "C-8a" not in st or "C-8b" not in st:
    raise SystemExit("01 must cite C-8a / next C-8b")

cl = Path("CHANGELOG.md").read_text(encoding="utf-8")
head = cl.split("### SYX 0.4.0", 1)[-1].split("###", 1)[0]
if "C-8a" not in head:
    raise SystemExit("CHANGELOG 0.4.0 must mention C-8a")

print("c8a_native_real_count=16")
print("c8a_tls_server_partial=1")
print("c8a_census_doc=1")
print("c8a_ssot_updated=1")
print("c8a_roadmap_ok=1")
print("c8_exited=0")
PY

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c8a_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c8a_book_out.txt
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
  echo "c8a_ch15_md_census_ok=0"
  exit 1
fi
echo "c8a_ch15_md_census_ok=1"
echo "current=C-8a"
echo "next=C-8b"
echo "c8_exited=0"
