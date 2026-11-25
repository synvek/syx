#!/usr/bin/env bash
# C-8e legacy compat notes census (F18/F19 Ch10).
# Soft park: C-8f · CST lossless · CallSuper · remove legacy parser · C-7c+ · ModuleId · C-5c+ · tls.server · macOS · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-8e legacy compat notes census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "C-8e Compatibility contract broken"
from pathlib import Path

en = Path("books/language-guide/10-structures-and-classes.md").read_text(encoding="utf-8")
zh = Path("books/zh-cn/language-guide/10-structures-and-classes.md").read_text(encoding="utf-8")
if en.count("Compatibility (C-8e)") < 2:
    raise SystemExit("en Ch10 must have ≥2 Compatibility (C-8e) blocks")
if zh.count("兼容（C-8e）") < 2:
    raise SystemExit("zh Ch10 must have ≥2 兼容（C-8e） blocks")
if "bare `name: Type`" not in en and "legacy bare" not in en:
    raise SystemExit("en Ch10 missing bare field note")
if "裸字段" not in zh and "name: Type" not in zh:
    raise SystemExit("zh Ch10 missing bare field note")
if "mutating func" not in en or "S060" not in en or "U-30" not in en:
    raise SystemExit("en Ch10 missing mutating Compatibility hinge")
if "mutating func" not in zh or "S060" not in zh or "U-30" not in zh:
    raise SystemExit("zh Ch10 missing mutating Compatibility hinge")
if "language-book-ssot" not in en or "language-book-ssot" not in zh:
    raise SystemExit("Ch10 must cite language-book-ssot")

inv = Path("docs/census/c-8c-book-parser-fork-census.md").read_text(encoding="utf-8")
rows = []
in_table = False
for line in inv.splitlines():
    if line.startswith("| id |"):
        in_table = True
        continue
    if in_table:
        if not line.startswith("|"):
            break
        if "---" in line:
            continue
        cols = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cols) >= 7 and cols[0].startswith("F"):
            rows.append(cols)
present = {r[4] for r in rows}
if "legacy_accept" in present:
    raise SystemExit(f"inventory still has live legacy_accept: {present}")
f18 = next(r for r in rows if r[0] == "F18")
f19 = next(r for r in rows if r[0] == "F19")
if f18[4] != "parked_honest" or f19[4] != "parked_honest":
    raise SystemExit("F18/F19 must be parked_honest")

adr = Path("docs/adr/language-book-ssot.md").read_text(encoding="utf-8")
if "c-8e-legacy-compat-notes-census" not in adr and "Compatibility notes (C-8e)" not in adr:
    raise SystemExit("ADR must point at C-8e notes")

print("c8e_ch10_en_ok=1")
print("c8e_ch10_zh_ok=1")
print("c8e_inventory_cleared=1")
print("c8e_adr_pointer=1")
PY

cargo test -p syx-driver --test lang c8e_legacy_compat -- --test-threads=1 \
  >/tmp/c8e_nails.txt 2>&1 || die "c8e lang nails failed"
ok "c8e_compat_nails" "1"
tail -12 /tmp/c8e_nails.txt

cargo test -p syx-driver --test lang c8c_book_parser_fork -- --test-threads=1 \
  >/tmp/c8e_c8c_nails.txt 2>&1 || die "c8c inventory nails (post C-8e) failed"
ok "c8c_inventory_nails_post" "1"

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

census = Path("docs/census/c-8e-legacy-compat-notes-census.md")
if not census.is_file():
    raise SystemExit("missing c-8e census")
ct = census.read_text(encoding="utf-8")
for needle in ("C-8e", "C-8f", "F18", "F19", "c8_exited"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")

ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
if "C-8e (done" not in ssot:
    raise SystemExit("SSOT must mark C-8e done")
if "C-8 remains `[~]`" not in ssot:
    raise SystemExit("SSOT must keep C-8 as [~]")
next_sec = ssot.split("## Next", 1)[-1]
if "C-8f" not in next_sec:
    raise SystemExit("SSOT Next must be C-8f")

rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("**C-8e**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-8e [x]")
if not any("**C-8f**" in l and "[ ]" in l for l in rm.splitlines()):
    raise SystemExit("02 must introduce C-8f [ ]")
if not any(l.startswith("### C-8 ·") and "[~]" in l for l in rm.splitlines()):
    raise SystemExit("02 must keep C-8 as [~]")
if "默认下一 = **C-8f**" not in rm:
    raise SystemExit("02 must set default next to C-8f")

st = Path("plan/01-project-status.md").read_text(encoding="utf-8")
if "C-8e" not in st or "C-8f" not in st:
    raise SystemExit("01 must cite C-8e / next C-8f")

cl = Path("CHANGELOG.md").read_text(encoding="utf-8")
head = cl.split("### SYX 0.4.0", 1)[-1].split("###", 1)[0]
if "C-8e" not in head:
    raise SystemExit("CHANGELOG 0.4.0 must mention C-8e")

print("c8e_native_real_count=16")
print("c8e_tls_server_partial=1")
print("c8e_census_doc=1")
print("c8e_ssot_updated=1")
print("c8e_roadmap_ok=1")
print("c8_exited=0")
PY

cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
out_json=$(mktemp)
SYX=./target/debug/syx
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c8e_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c8e_book_out.txt
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
  echo "c8e_legacy_compat_notes_census_ok=0"
  exit 1
fi
echo "c8e_legacy_compat_notes_census_ok=1"
echo "current=C-8e"
echo "next=C-8f"
echo "c8_exited=0"
