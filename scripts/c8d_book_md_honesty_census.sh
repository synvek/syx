#!/usr/bin/env bash
# C-8d book MD honesty census (F20–F23).
# Soft park: C-8e · CST lossless · CallSuper · C-7c+ · ModuleId · C-5c+ · tls.server · macOS · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-8d book MD honesty census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "F20–F23 MD honesty contract broken"
from pathlib import Path

def read(p):
    return Path(p).read_text(encoding="utf-8")

en6 = read("books/language-guide/06-control-flow.md")
zh6 = read("books/zh-cn/language-guide/06-control-flow.md")
if "are **all Implemented**" in en6:
    raise SystemExit("en Ch06 still all Implemented")
if "labeled `break`/`continue` **Partially implemented**" not in en6:
    raise SystemExit("en Ch06 status labeled not Partially")
if "Honesty (C-8d)" not in en6:
    raise SystemExit("en Ch06 missing Honesty (C-8d)")
if "**全部已实现**" in zh6:
    raise SystemExit("zh Ch06 still 全部已实现")
if "标签 `break`/`continue` **部分实现**" not in zh6:
    raise SystemExit("zh Ch06 status labeled not 部分实现")
if "诚实（C-8d）" not in zh6:
    raise SystemExit("zh Ch06 missing 诚实（C-8d）")

en15 = read("books/language-guide/15-initialization.md")
zh15 = read("books/zh-cn/language-guide/15-initialization.md")
if "`required init` / `convenience init` **Not implemented**" not in en15:
    raise SystemExit("en Ch15 required/convenience not Not implemented")
if "**Partially implemented** / **Not implemented**" in en15:
    raise SystemExit("en Ch15 still Partially/Not ambiguity")
if "`required init` / `convenience init` **未实现**" not in zh15:
    raise SystemExit("zh Ch15 required/convenience not 未实现")
if "**部分实现** / **未实现**" in zh15:
    raise SystemExit("zh Ch15 still 部分/未 ambiguity")

en14 = read("books/language-guide/14-inheritance.md")
zh14 = read("books/zh-cn/language-guide/14-inheritance.md")
en14c = next(l for l in en14.splitlines() if "`convenience init`" in l and "|" in l)
zh14c = next(l for l in zh14.splitlines() if "`convenience init`" in l and "|" in l)
if "Partially" in en14c or "**Not implemented**" not in en14c:
    raise SystemExit(f"en Ch14 convenience bad: {en14c}")
if "部分实现" in zh14c or "**未实现**" not in zh14c:
    raise SystemExit(f"zh Ch14 convenience bad: {zh14c}")

en10 = read("books/language-guide/10-structures-and-classes.md")
zh10 = read("books/zh-cn/language-guide/10-structures-and-classes.md")
en10d = en10.split("## Differences from Swift", 1)[1]
zh10d = zh10.split("## 与 Swift 的差异", 1)[1]
en10m = next(l for l in en10d.splitlines() if "`mutating func`" in l)
zh10m = next(l for l in zh10d.splitlines() if "`mutating func`" in l)
if "**Partially implemented**" not in en10m or "**Implemented** (Phase" in en10m:
    raise SystemExit(f"en Ch10 mutating Swift-diff bad: {en10m}")
if "**部分实现**" not in zh10m or "**已实现**" in zh10m:
    raise SystemExit(f"zh Ch10 mutating Swift-diff bad: {zh10m}")
if "Honesty (C-8d)" not in en10 or "诚实（C-8d）" not in zh10:
    raise SystemExit("Ch10 missing C-8d Honesty")

en17 = read("books/language-guide/17-error-handling.md")
zh17 = read("books/zh-cn/language-guide/17-error-handling.md")
if "Honesty (C-8d)" not in en17 or "≠" not in en17:
    raise SystemExit("en Ch17 missing Honesty ≠")
if "诚实（C-8d）" not in zh17 or "≠" not in zh17:
    raise SystemExit("zh Ch17 missing 诚实 ≠")

# Inventory: no live doc_debt / book_ahead rows.
inv = read("docs/census/c-8c-book-parser-fork-census.md")
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
if "doc_debt" in present or "book_ahead" in present:
    raise SystemExit(f"inventory still has doc_debt/book_ahead: {present}")
# C-8e may clear live legacy_accept; historical C-8d only required doc_debt/book_ahead clear.
# Prefer: if legacy_accept still present, OK for old runs; after C-8e it should be absent.
# Soft: do not require legacy_accept anymore.

print("c8d_md_f20_ok=1")
print("c8d_md_f21_ok=1")
print("c8d_md_f22_ok=1")
print("c8d_md_f23_ok=1")
print("c8d_inventory_cleared=1")
PY

cargo test -p syx-driver --test lang c8d_book_md_honesty -- --test-threads=1 \
  >/tmp/c8d_nails.txt 2>&1 || die "c8d lang honesty nails failed"
ok "c8d_honesty_nails" "1"
tail -12 /tmp/c8d_nails.txt

cargo test -p syx-driver --test lang c8c_book_parser_fork -- --test-threads=1 \
  >/tmp/c8d_c8c_nails.txt 2>&1 || die "c8c inventory nails (post C-8d) failed"
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

census = Path("docs/census/c-8d-book-md-honesty-census.md")
if not census.is_file():
    raise SystemExit("missing c-8d census")
ct = census.read_text(encoding="utf-8")
for needle in ("C-8d", "C-8e", "F20", "c8_exited"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")

ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
if "C-8d (done" not in ssot:
    raise SystemExit("SSOT must mark C-8d done")
if "C-8 remains `[~]`" not in ssot:
    raise SystemExit("SSOT must keep C-8 as [~]")
next_sec = ssot.split("## Next", 1)[-1]
if "C-8e" not in next_sec:
    raise SystemExit("SSOT Next must be C-8e")

rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("**C-8d**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-8d [x]")
if not any("**C-8e**" in l and "[ ]" in l for l in rm.splitlines()):
    raise SystemExit("02 must introduce C-8e [ ]")
if not any(l.startswith("### C-8 ·") and "[~]" in l for l in rm.splitlines()):
    raise SystemExit("02 must keep C-8 as [~]")
if "默认下一 = **C-8e**" not in rm:
    raise SystemExit("02 must set default next to C-8e")

st = Path("plan/01-project-status.md").read_text(encoding="utf-8")
if "C-8d" not in st or "C-8e" not in st:
    raise SystemExit("01 must cite C-8d / next C-8e")

cl = Path("CHANGELOG.md").read_text(encoding="utf-8")
head = cl.split("### SYX 0.4.0", 1)[-1].split("###", 1)[0]
if "C-8d" not in head:
    raise SystemExit("CHANGELOG 0.4.0 must mention C-8d")

print("c8d_native_real_count=16")
print("c8d_tls_server_partial=1")
print("c8d_census_doc=1")
print("c8d_ssot_updated=1")
print("c8d_roadmap_ok=1")
print("c8_exited=0")
PY

cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
out_json=$(mktemp)
SYX=./target/debug/syx
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c8d_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c8d_book_out.txt
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
  echo "c8d_book_md_honesty_census_ok=0"
  exit 1
fi
echo "c8d_book_md_honesty_census_ok=1"
echo "current=C-8d"
echo "next=C-8e"
echo "c8_exited=0"
