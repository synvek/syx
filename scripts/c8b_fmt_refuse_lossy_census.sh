#!/usr/bin/env bash
# C-8b formatter refuse-lossy census.
# Soft park: C-8c · CallSuper · CST expand · C-7c+ · ModuleId · C-5c+ · tls.server · macOS · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-8b formatter refuse-lossy census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "refuse-lossy source/CLI/LSP contract broken"
from pathlib import Path

fmt = Path("crates/syx-fmt/src/lib.rs").read_text(encoding="utf-8")
if "struct FormatPolicy" not in fmt or "allow_lossy" not in fmt:
    raise SystemExit("syx-fmt must define FormatPolicy.allow_lossy")
if '"F003"' not in fmt:
    raise SystemExit("syx-fmt must define F003")
# Default dry-run must not silently call format_source_auto on CST001.
dry = fmt.split("pub fn format_dry_run(", 1)[1].split("pub fn ", 1)[0]
if "FormatPolicy::default" not in dry and "allow_lossy: false" not in dry:
    raise SystemExit("format_dry_run must default refuse-lossy")
policy_fn = fmt.split("pub fn format_with_policy(", 1)[1].split("pub fn ", 1)[0]
if "F003" not in policy_fn or "lossy" not in policy_fn.lower():
    raise SystemExit("format_with_policy must refuse CST001 with F003/lossy")

cli = Path("crates/syx-cli/src/main.rs").read_text(encoding="utf-8")
if "allow_lossy: bool" not in cli:
    raise SystemExit("CLI Format must expose allow_lossy")
if "format_project_with" not in cli or "format_check_path_with_policy" not in cli:
    raise SystemExit("CLI must wire allow_lossy into format paths")

driver = Path("crates/syx-driver/src/format_lint.rs").read_text(encoding="utf-8")
lsp = driver.split("pub fn format_document_for_lsp(", 1)[1].split("pub fn ", 1)[0]
if "allow_lossy: false" not in lsp:
    raise SystemExit("LSP must hardcode allow_lossy=false")
if "allow_lossy: true" in lsp:
    raise SystemExit("LSP must never set allow_lossy=true")

en = Path("docs/9_UsageGuide.md").read_text(encoding="utf-8")
zh = Path("docs/zh/9_UsageGuide.zh-CN.md").read_text(encoding="utf-8")
for label, text in (("en", en), ("zh", zh)):
    if "--allow-lossy" not in text or "F003" not in text:
        raise SystemExit(f"{label} UsageGuide must document --allow-lossy / F003")
    if "CST-first" not in text:
        raise SystemExit(f"{label} UsageGuide must state CST-first")

print("c8b_src_policy_ok=1")
print("c8b_cli_allow_lossy=1")
print("c8b_lsp_hard_refuse=1")
print("c8b_usageguide_ok=1")
PY

cargo test -p syx-driver --test credibility c8b_fmt_refuse_lossy -- --test-threads=1 \
  >/tmp/c8b_nails.txt 2>&1 || die "c8b credibility nails failed"
ok "c8b_refuse_lossy_nails" "1"
tail -12 /tmp/c8b_nails.txt

cargo test -p syx-fmt --test fmt_dry_run fmt_default_cst_fallback_ast_f001 -- --test-threads=1 \
  >/tmp/c8b_fmt_nail.txt 2>&1 || die "syx-fmt refuse-lossy nail failed"
ok "c8b_fmt_dry_run_nail" "1"
tail -8 /tmp/c8b_fmt_nail.txt

# Negative: default refuse write; --allow-lossy may rewrite (comment drop ok).
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT
cat >"$TMP/lossy.sy" <<'SY'
// keep?
func main() { while ((((a)))) { print(1) } }
SY
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
SYX=./target/debug/syx
if "$SYX" format "$TMP/lossy.sy" >/tmp/c8b_fmt_default.txt 2>&1; then
  die "default format must refuse CST001 lossy write"
fi
ok "c8b_default_refuse_write" "1"
if ! grep -E 'F003|lossy|allow-lossy' /tmp/c8b_fmt_default.txt >/dev/null; then
  # miette may wrap; also accept non-zero alone if file unchanged
  if grep -q 'keep?' "$TMP/lossy.sy"; then
    ok "c8b_default_msg_or_unchanged" "1"
  else
    die "default refuse left unexpected rewrite"
  fi
else
  ok "c8b_default_msg_or_unchanged" "1"
fi
# File must still contain comment (no write).
grep -q 'keep?' "$TMP/lossy.sy" || die "default path must not write lossy output"

"$SYX" format --allow-lossy "$TMP/lossy.sy" >/tmp/c8b_fmt_lossy.txt 2>&1 \
  || die "allow-lossy format must succeed"
ok "c8b_allow_lossy_write" "1"
if grep -q 'keep?' "$TMP/lossy.sy"; then
  die "allow-lossy AST path should drop comment (honesty)"
fi

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

census = Path("docs/census/c-8b-fmt-refuse-lossy-census.md")
if not census.is_file():
    raise SystemExit("missing c-8b census")
ct = census.read_text(encoding="utf-8")
for needle in ("C-8b", "C-8c", "F003", "allow-lossy", "c8_exited"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")

ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
if "C-8b (done" not in ssot:
    raise SystemExit("SSOT must mark C-8b done")
if "C-8 remains `[~]`" not in ssot:
    raise SystemExit("SSOT must keep C-8 as [~]")
next_sec = ssot.split("## Next", 1)[-1]
if "C-8c" not in next_sec:
    raise SystemExit("SSOT Next must be C-8c")
if "+++" in next_sec:
    raise SystemExit("SSOT Next must not use +++")

rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("**C-8b**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-8b [x]")
if not any(l.startswith("### C-8 ·") and "[~]" in l for l in rm.splitlines()):
    raise SystemExit("02 must keep C-8 as [~]")
if "默认下一 = **C-8c**" not in rm:
    raise SystemExit("02 must set default next to C-8c")

st = Path("plan/01-project-status.md").read_text(encoding="utf-8")
if "C-8b" not in st or "C-8c" not in st:
    raise SystemExit("01 must cite C-8b / next C-8c")

cl = Path("CHANGELOG.md").read_text(encoding="utf-8")
head = cl.split("### SYX 0.4.0", 1)[-1].split("###", 1)[0]
if "C-8b" not in head:
    raise SystemExit("CHANGELOG 0.4.0 must mention C-8b")

print("c8b_native_real_count=16")
print("c8b_tls_server_partial=1")
print("c8b_census_doc=1")
print("c8b_ssot_updated=1")
print("c8b_roadmap_ok=1")
print("c8_exited=0")
PY

out_json=$(mktemp)
SYX=./target/debug/syx
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c8b_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c8b_book_out.txt
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
  echo "c8b_fmt_refuse_lossy_census_ok=0"
  exit 1
fi
echo "c8b_fmt_refuse_lossy_census_ok=1"
echo "current=C-8b"
echo "next=C-8c"
echo "c8_exited=0"
