#!/usr/bin/env bash
# C-3a MIR quality heuristic demote census.
# Soft park: DefId · full verifier · tls.server · macOS full-native · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-3a mir-quality-demote census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "codegen must not silent-quality-pick same-name MIR bodies"
from pathlib import Path
import re

lib = Path("crates/syx-codegen/src/lib.rs").read_text(encoding="utf-8")
# Strip line comments for crude probe
code = "\n".join(ln for ln in lib.splitlines() if not ln.lstrip().startswith("//"))

if "fn mir_function_quality" in code or "mir_function_quality(" in code:
    raise SystemExit("mir_function_quality must be removed from select path")

# Fail-closed helpers must exist; no score compare for body pick.
if "unique_mir_functions_by_name" not in code:
    raise SystemExit("missing unique_mir_functions_by_name")
if "unique_owned_mir_functions_by_name" not in code:
    raise SystemExit("missing unique_owned_mir_functions_by_name")
if "quality heuristic forbidden" not in lib:
    raise SystemExit("missing C-3a fail-closed diagnostic text")
if "MIR001" not in lib:
    raise SystemExit("missing MIR001")

# No silent "higher quality wins" compare in uniqueness helpers.
uniq = re.search(
    r"fn unique_mir_functions_by_name[\s\S]*?fn unique_owned_mir_functions_by_name",
    code,
)
if not uniq:
    raise SystemExit("could not locate unique_mir_functions_by_name body")
body = uniq.group(0)
if re.search(r"mir_function_quality|new_score\s*>|quality\s*>", body):
    raise SystemExit("uniqueness helper still compares quality/scores")

print("c3a_no_silent_quality_pick=1")
PY

python3 - <<'PY' || die "neg pin / arity uniquify missing"
from pathlib import Path

neg = Path("crates/syx-driver/tests/credibility/c3a_mir_quality_demote_contract.rs")
if not neg.is_file():
    raise SystemExit("missing c3a_mir_quality_demote_contract.rs")
nt = neg.read_text(encoding="utf-8")
for needle in (
    "c3a_duplicate_mir_body_different_fingerprint_fails",
    "MIR001",
    "quality heuristic forbidden",
    "identical fingerprint",
):
    if needle not in nt and "identical fingerprints" not in nt:
        if needle == "identical fingerprint" and "identical fingerprints" in nt:
            continue
        raise SystemExit(f"neg pin missing {needle}")

main = Path("crates/syx-driver/tests/credibility/main.rs").read_text(encoding="utf-8")
if "mod c3a_mir_quality_demote_contract" not in main:
    raise SystemExit("credibility main must mod c3a_mir_quality_demote_contract")

mir = Path("crates/syx-mir/src/mir_lower_type_members.rs").read_text(encoding="utf-8")
if "mangle_type_method_name" not in mir or "__a" not in mir:
    raise SystemExit("arity mangling helper missing")
if "collect_arity_overload_mir_names" not in mir:
    raise SystemExit("collect_arity_overload_mir_names missing")

print("c3a_neg_pin=1")
print("c3a_arity_uniquify=1")
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

census = Path("docs/census/c-3a-mir-quality-demote-census.md")
if not census.is_file():
    raise SystemExit("missing c-3a census")
ct = census.read_text(encoding="utf-8")
for needle in ("C-3a", "C-3b", "MIR001", "quality", "≠"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")
if "C-3 `[x]`" in ct or "宣称 C-3 `[x]`" in ct:
    # allow non-claim text mentioning ≠ C-3 [x]
    pass

ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
if "C-3a (done" not in ssot:
    raise SystemExit("SSOT must mark C-3a done")
next_sec = ssot.split("## Next", 1)[-1]
if "C-3b" not in next_sec:
    raise SystemExit("SSOT Next must be C-3b")
if "+++" in next_sec:
    raise SystemExit("SSOT Next must not use +++")

rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("**C-3a**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-3a [x]")
if not any("**C-3b**" in l and "[ ]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-3b [ ] default next")
if "### C-3 ·" not in rm or "[~]" not in rm:
    raise SystemExit("02 must keep C-3 as [~]")
if "默认下一" not in rm or "**C-3b**" not in rm:
    raise SystemExit("02 must set 默认下一 C-3b")

st = Path("plan/01-project-status.md").read_text(encoding="utf-8")
if "C-3b" not in st or "C-3a" not in st:
    raise SystemExit("01 must cite C-3a / next C-3b")

cl = Path("CHANGELOG.md").read_text(encoding="utf-8")
if "C-3a" not in cl.split("### SYX 0.4.0", 1)[-1].split("###", 1)[0]:
    raise SystemExit("CHANGELOG 0.4.0 must mention C-3a")

print("c3a_native_real_count=16")
print("c3a_tls_server_partial=1")
print("c3a_census_doc=1")
print("c3a_ssot_updated=1")
print("c3a_roadmap_ok=1")
PY

echo "== neg pin cargo =="
cargo test -p syx-driver --test credibility c3a_ -- --nocapture >/tmp/c3a_neg.txt 2>&1 \
  || die "c3a credibility tests failed"
if ! grep -q "c3a_duplicate_mir_body_different_fingerprint_fails ... ok" /tmp/c3a_neg.txt; then
  die "neg pin did not pass"
fi
ok "c3a_neg_cargo" "1"
tail -8 /tmp/c3a_neg.txt

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c3a_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c3a_book_out.txt
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
  echo "c3a_mir_quality_demote_census_ok=0"
  exit 1
fi
echo "c3a_mir_quality_demote_census_ok=1"
echo "current=C-3a-mir-quality-demote"
echo "next=C-3b"
