#!/usr/bin/env bash
# C-2b-12 io.file.raii·stream·watch paid raise census.
# Soft park: tls (C-2b-13) · macOS LLVM · C-2c/d · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-2b-12 io-file-raise census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "capability raise contract broken"
from pathlib import Path
import re
caps = Path("packages/internal/capabilities.toml").read_text(encoding="utf-8")
blocks = re.split(r"(?=^\[\[capability\]\])", caps, flags=re.M)
want = {
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
}
must_partial = {
    "net.tls.client",
    "net.tls.server",
}
raise_ids = {"io.file.raii", "io.file.stream", "io.file.watch"}
got = set()
extra = []
for b in blocks:
    if not b.startswith("[[capability]]"):
        continue
    m = re.search(r'^id\s*=\s*"([^"]+)"', b, re.M)
    if not m:
        continue
    cid = m.group(1)
    nm = re.search(r'backends\s*=\s*\{[^}]*\bnative\s*=\s*"([^"]+)"', b, re.S)
    if not nm:
        continue
    if nm.group(1) == "real":
        if cid in want:
            got.add(cid)
            if cid in raise_ids:
                if "C-2b-12" not in b:
                    raise SystemExit(f"{cid} missing C-2b-12 notes")
                if "unpaid ≠ full-native GA" in b:
                    raise SystemExit(f"{cid} still has unpaid note")
        else:
            extra.append(cid)
    if cid in must_partial and nm.group(1) != "partial":
        raise SystemExit(f"{cid} must stay partial, got {nm.group(1)}")
if got != want:
    raise SystemExit(f"expected real ids {sorted(want)}, got {sorted(got)}")
if extra:
    raise SystemExit(f"unexpected native=real: {extra}")
print("c2b12_native_real_count=15")
print("c2b12_raise_ids=io.path,io.file.read_all_bytes,net.tcp,net.tcp.listener,data.sqlite,net.http,data.sqlite.pool,data.sqlite.row,net.http.types,net.http.server,net.http.stream,net.http.timeout_tls,io.file.raii,io.file.stream,io.file.watch")
print("c2b12_tls_residual_partial=1")
PY

python3 - <<'PY' || die "ci.yml io.file paid contract broken"
from pathlib import Path
import re
ci = Path(".github/workflows/ci.yml").read_text(encoding="utf-8")
m = re.search(
    r"(?m)^  full-native-linux:\n(.*?)(?=\n  [a-zA-Z0-9_-]+:|\Z)",
    ci,
    re.S,
)
if not m:
    raise SystemExit("full-native-linux missing")
block = m.group(0)
if "continue-on-error: true" in block:
    raise SystemExit("full-native-linux must stay required")
for needle in (
    "m2_s3_02b_with_open_native_llvm",
    "m2_s3_03_file_stream_read_chunk_native_llvm",
    "m2_s3_03_file_watcher_native_llvm",
    "m2_s4_02b_timeout_hang_loopback_native_aot",
    "m2_s4_04_http_body_stream_native_llvm",
    "m2_s4_03b_graceful_shutdown_native_llvm",
    "m2_s4_01_response_accessors_native_llvm",
    "hostfn_sql_pool_lifecycle_native_llvm",
    "hostfn_sql_row_accessors_native_llvm",
    "path_file_native_wasm_ga",
    "tcp_native_wasm_ga",
    "sql_native_wasm_ga",
    "http_native_wasm_ga",
    "examples/hello.sy",
):
    if needle not in block:
        raise SystemExit(f"missing {needle}")
if "stdlib" not in block:
    raise SystemExit("full-native-linux must use --test stdlib for file llvm")
print("c2b12_ci_file_llvm=1")
print("c2b12_ci_still_required=1")
PY

python3 - <<'PY' || die "census/SSOT hygiene failed"
from pathlib import Path
import re as _re
census = Path("docs/census/c-2b-12-io-file-raise-census.md")
if not census.is_file():
    raise SystemExit("missing c-2b-12 census")
ct = census.read_text(encoding="utf-8")
for needle in (
    "C-2b-12",
    "io.file.raii",
    "io.file.stream",
    "io.file.watch",
    "C-2b-13",
    "m2_s3_02b_with_open_native_llvm",
    "m2_s3_03_file_stream_read_chunk_native_llvm",
    "m2_s3_03_file_watcher_native_llvm",
    "tls",
):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")
ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
if "C-2b-12" not in ssot or "C-2b-13" not in ssot:
    raise SystemExit("SSOT must mention C-2b-12 and next C-2b-13")
for needle in (
    "m2_s3_02b_with_open_native_llvm",
    "m2_s3_03_file_stream_read_chunk_native_llvm",
    "m2_s3_03_file_watcher_native_llvm",
):
    if needle not in ssot:
        raise SystemExit(f"SSOT must mention {needle}")
next_sec = ssot.split("## Next", 1)[-1]
if "+++" in next_sec:
    raise SystemExit("SSOT Next must not use +++")
rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("**C-2b-12**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-2b-12 [x]")
if not any("**C-2b-13**" in l and "默认下一" in l for l in rm.splitlines()) and "默认下一 = **C-2b-13**" not in rm:
    raise SystemExit("02 must set 默认下一 C-2b-13")
hits = []
for path in Path("crates").rglob("*.rs"):
    text = path.read_text(encoding="utf-8", errors="ignore")
    code = "\n".join(
        ln for ln in text.splitlines() if not ln.lstrip().startswith("//")
    )
    if _re.search(r"\brun_native_strict_or_mirror_vm\b", code):
        hits.append(str(path))
if hits:
    raise SystemExit(f"mirror helper rebound: {hits}")
print("c2b12_census_doc=1")
print("c2b12_ssot_updated=1")
print("c2b12_roadmap_ok=1")
print("c2b12_no_mirror=1")
PY

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c2b12_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c2b12_book_out.txt
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
  echo "c2b_12_io_file_raise_census_ok=0"
  exit 1
fi
echo "c2b_12_io_file_raise_census_ok=1"
echo "current=C-2b-12-io-file-raise"
echo "next=C-2b-13"
