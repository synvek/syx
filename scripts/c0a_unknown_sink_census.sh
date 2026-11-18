#!/usr/bin/env bash
# C-0a Unknown→ground sink census (S004).
# Soft park: close (_, Unknown) · ErrorType · call-arg Unknown · C-0e · C-1 detail · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12 · hello std
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-0a unknown-sink census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "C-0a surface missing"
from pathlib import Path
ana = Path("crates/syx-semantic/src/semantic_analyzer.rs").read_text(encoding="utf-8")
if "current_untyped_params" not in ana:
    raise SystemExit("missing current_untyped_params field")
call = Path("crates/syx-semantic/src/semantic_analyze_call.rs").read_text(encoding="utf-8")
if "check_unknown_to_ground_sink" not in call:
    raise SystemExit("missing check_unknown_to_ground_sink")
# Soft arms must remain
tc = Path("crates/syx-semantic/src/type_checker.rs").read_text(encoding="utf-8")
if "(_, TypeId::Unknown) => true" not in tc:
    raise SystemExit("must not remove (_, Unknown) soft arm")
if 'n.as_str() == "Object"' not in tc or "(_, TypeId::Named(n))" not in tc:
    raise SystemExit("must not remove found-Object soft arm")
stmt = Path("crates/syx-semantic/src/semantic_analyze_stmt.rs").read_text(encoding="utf-8")
if "check_unknown_to_ground_sink" not in stmt:
    raise SystemExit("stmt sinks missing gate")
expr = Path("crates/syx-semantic/src/semantic_analyze_expr.rs").read_text(encoding="utf-8")
if "check_unknown_to_ground_sink" not in expr:
    raise SystemExit("assign sink missing gate")
nail = Path("crates/syx-driver/tests/lang/type_soundness.rs").read_text(encoding="utf-8")
for needle in (
    "c0a_untyped_param_return_concrete_s004",
    "c0a_object_missing_member_to_int_s004",
    "c0a_any_accepts_missing_member_ok",
    "c0a_empty_array_still_ok",
    "c0a_found_object_to_int_still_ok",
):
    if needle not in nail:
        raise SystemExit(f"type_soundness missing {needle}")
print("c0a_surface=1")
print("c0a_lang_nail=1")
print("c0a_soft_arms=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/c-0a-unknown-sink-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/c-0a-unknown-sink-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("C-0a", "S004", "≠", "C-1", "Unknown"):
    if needle not in ct:
        raise SystemExit(f"c-0a census missing {needle}")
print("c0a_census_doc=1")
PY

SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

RET=$(mktemp /tmp/c0a_ret_XXXX.sy)
cat >"$RET" <<'EOF'
func id(_ x) -> Int { return x }
func main() { }
EOF
set +e
out=$("$SYX" check "$RET" --enable-std=false 2>&1)
ec=$?
set -e
if [[ "$ec" -eq 0 ]] || ! echo "$out" | grep -q '\[S004\]'; then
  echo "$out" >&2
  die "untyped return must S004"
fi
if echo "$out" | grep -q 'Unknown'; then
  echo "$out" >&2
  die "diag must not print literal Unknown"
fi
ok "probe_untyped_return" "S004"
rm -f "$RET"

MISS=$(mktemp /tmp/c0a_miss_XXXX.sy)
cat >"$MISS" <<'EOF'
func main() {
    let o: Object = [:]
    let n: Int = o.missing
}
EOF
set +e
out=$("$SYX" check "$MISS" --enable-std=false 2>&1)
ec=$?
set -e
if [[ "$ec" -eq 0 ]] || ! echo "$out" | grep -q '\[S004\]'; then
  echo "$out" >&2
  die "missing member to Int must S004"
fi
ok "probe_missing_member" "S004"
rm -f "$MISS"

ANY=$(mktemp /tmp/c0a_any_XXXX.sy)
cat >"$ANY" <<'EOF'
func main() {
    let o: Object = [:]
    let a: Any = o.missing
}
EOF
"$SYX" check "$ANY" --enable-std=false >/tmp/c0a_any_check.txt 2>&1 || {
  cat /tmp/c0a_any_check.txt
  die "Any must still accept missing member"
}
ok "probe_any" "0"
rm -f "$ANY"

"$SYX" check examples/hello.sy >/tmp/c0a_hello.txt 2>&1 || {
  cat /tmp/c0a_hello.txt
  die "hello.sy with default std must stay green"
}
ok "probe_hello_std" "0"

cargo test -p syx-driver --test lang type_soundness -- --test-threads=4 \
  >/tmp/c0a_lang_nail.txt 2>&1 || {
  tail -40 /tmp/c0a_lang_nail.txt
  die "lang type_soundness nail failed"
}
ok "lang_nail" "type_soundness"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c0a_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c0a_book_out.txt
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

python3 - <<'PY' || die "roadmap C-0a exit / next C-1 missing"
from pathlib import Path
import re
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
rows = [l for l in text.splitlines() if "C-0a" in l and "[x]" in l]
if not rows:
    raise SystemExit("C-0a not [x]")
header = "\n".join(text.splitlines()[:12])
if "C-1" not in header and not any("C-1" in l and "默认下一" in l for l in text.splitlines()):
    # also accept 默认下一 = **C-1**
    if "默认下一 = **C-1**" not in text:
        raise SystemExit("roadmap default next must be C-1")
if re.search(r"默认下一\s*=\s*\*?\*?R-50", text) or re.search(r"默认下一\s*=\s*\*?\*?U-41", text):
    raise SystemExit("roadmap must not default next to R-50/U-41")
print("roadmap_c0a_exit=1")
print("roadmap_next_c1=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "c0a_unknown_sink_census_ok=0"
  exit 1
fi
echo "c0a_unknown_sink_census_ok=1"
echo "current=C-0a-unknown-sink"
echo "next=C-1"
