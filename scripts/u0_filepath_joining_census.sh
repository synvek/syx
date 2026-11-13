#!/usr/bin/env bash
# U-0 FilePath joining return-type census — chain + reassign keep FilePath.
# Soft park: JIT Suite-1 · func…async · SyxHost wipe · HTTPClient · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== U-0 FilePath joining census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "U-0 fix surface missing"
from pathlib import Path

mir_idx = Path("crates/syx-mir/src/mir_module_index.rs").read_text(encoding="utf-8")
if "U-0: register instance method return types" not in mir_idx:
    raise SystemExit("mir_module_index missing U-0 Struct method returns")
if "extension_method_returns" not in mir_idx:
    raise SystemExit("mir_module_index missing extension_method_returns for Struct")

sema = Path("crates/syx-semantic/src/semantic_analyzer.rs").read_text(encoding="utf-8")
if "U-0: struct/class instance methods are also keyed" not in sema:
    raise SystemExit("semantic_analyzer missing U-0 mangled return lookup")

ctx = Path("crates/syx-driver/src/compile_file_context.rs").read_text(encoding="utf-8")
if "inject_imported_public_structs" not in ctx:
    raise SystemExit("compile_file_context missing pre-semantic Struct inject")
pg = Path("crates/syx-driver/src/prelude_glue.rs").read_text(encoding="utf-8")
if "fn inject_imported_public_structs" not in pg:
    raise SystemExit("prelude_glue missing inject_imported_public_structs")

call = Path("crates/syx-mir/src/mir_lower_expr_call_member.rs").read_text(encoding="utf-8")
if "U-0: zero-arg" not in call or "is_stored_field" not in call:
    raise SystemExit("mir_lower_expr_call_member missing U-0 field ObjectGet carve-out")

nail = Path("crates/syx-driver/tests/lang/filepath_joining_chain.rs")
if not nail.is_file():
    raise SystemExit("missing lang nail filepath_joining_chain.rs")
nt = nail.read_text(encoding="utf-8")
for needle in ("joining", "normalized", "reassign", "FilePath", "rawValue"):
    if needle not in nt:
        raise SystemExit(f"filepath_joining_chain nail missing {needle}")
print("u0_fix_surface=1")
print("u0_lang_nail=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/u-0-filepath-joining-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/u-0-filepath-joining-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("U-0", "FilePath", "joining", "≠", "R-50", "JIT"):
    if needle not in ct:
        raise SystemExit(f"u-0 census missing {needle}")
print("u0_census_doc=1")
PY

SYX=./target/debug/syx
if [[ ! -x "$SYX" ]]; then
  cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
fi

PROBE=$(mktemp /tmp/u0_probe_XXXX.sy)
cat >"$PROBE" <<'EOF'
import System
public func main() {
    let a = FilePath(rawValue: "/tmp")
    let b = a.joining(other: "x")
    let c = b.joining(other: "y").normalized()
    print(b.rawValue())
    print(c.rawValue())
    var p = FilePath(rawValue: "/a")
    p = p.joining(other: "b")
    print(p.rawValue())
}
EOF

"$SYX" check "$PROBE" >/tmp/u0_probe_check.txt 2>&1 || {
  tail -20 /tmp/u0_probe_check.txt
  die "probe check failed"
}
ok "probe_check" "ok"

out=$("$SYX" run "$PROBE" 2>/tmp/u0_probe_run_err.txt) || {
  cat /tmp/u0_probe_run_err.txt
  die "probe run failed"
}
expected=$'/tmp/x\n/tmp/x/y\n/a/b'
if [[ "$out" != "$expected" ]]; then
  echo "got: $out" >&2
  die "probe run output mismatch"
fi
ok "probe_run" "chain+reassign"
rm -f "$PROBE"

cargo test -p syx-driver --test lang filepath_joining_chain -- --test-threads=4 \
  >/tmp/u0_lang_nail.txt 2>&1 || {
  tail -40 /tmp/u0_lang_nail.txt
  die "lang filepath_joining_chain nail failed"
}
ok "lang_nail" "filepath_joining_chain"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/u0_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -20 /tmp/u0_book_out.txt
python3 - <<PY || die "sy,output must be 12/12 (en+zh)"
import json, sys
from pathlib import Path
d = json.loads(Path("$out_json").read_text())
ok_n = d.get("output_run_ok", 0)
tot = d.get("output_run_total", 0)
print(f"output_run_ok={ok_n}")
print(f"output_run_total={tot}")
en = [r for r in d["results"] if "zh-cn" not in r["file"]]
zh = [r for r in d["results"] if "zh-cn" in r["file"]]
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

python3 - <<'PY' || die "roadmap U-0 exit / next park missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if "U-0" not in text:
    raise SystemExit("roadmap missing U-0 section")
rows = [l for l in text.splitlines() if "| **U-0** |" in l or "| U-0 |" in l]
if not rows or not any("`[x]`" in r or "| [x] |" in r or "[x]" in r for r in rows):
    raise SystemExit(f"U-0 not [x]: {rows}")
# Must not resurrect R-50 as default next numbered window.
if "默认下一" in text:
    # Prefer unnumbered / park language after U-0.
    pass
if "开 R-50" in text or "新开 R-50" in text:
    # Allowed as explicit non-goal; fail only if default next is R-50.
    pass
tail = text
if "R-50" in text and "默认下一" in text:
    chunk = text.split("默认下一", 1)[1][:400]
    if "R-50" in chunk and "不" not in chunk and "park" not in chunk.lower() and "未编号" not in chunk:
        raise SystemExit("roadmap must not default next to R-50")
print("roadmap_u0_exit=1")
print("roadmap_no_r50_default=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "u0_filepath_joining_census_ok=0"
  exit 1
fi
echo "u0_filepath_joining_census_ok=1"
echo "current=U-0-filepath-joining"
echo "next=U-1-or-park"
