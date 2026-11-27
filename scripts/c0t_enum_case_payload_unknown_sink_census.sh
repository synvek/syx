#!/usr/bin/env bash
# C-0t enum case payload Unknown→ground sink census (S004).
# Soft park: Module.Type · cross-enum same case · C-0a+ 三分 · close soft arm · C-0e · host/extension method GA · C-3/C-5 [x] · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-0t enum-case-payload-unknown-sink census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "C-0t surface missing"
from pathlib import Path

col = Path("crates/syx-semantic/src/semantic_collect_decls.rs").read_text(encoding="utf-8")
if "fn register_enum_case_payload_params" not in col:
    raise SystemExit("collect must host C-0t register_enum_case_payload_params")
if col.count("register_enum_case_payload_params") < 5:
    raise SystemExit("helper must be called at all enum_case_field_types sites")
if "C-0t: enum case payload params" not in col:
    raise SystemExit("toplevel Decl::Enum must cite C-0t")

tc = Path("crates/syx-semantic/src/type_checker.rs").read_text(encoding="utf-8")
if "(_, TypeId::Unknown) => true" not in tc:
    raise SystemExit("must not remove (_, Unknown) soft arm")

# Call face must remain untouched for this knife (collect-only).
call = Path("crates/syx-semantic/src/semantic_analyze_call.rs")
call_txt = call.read_text(encoding="utf-8") if call.is_file() else ""
if "C-0t" in call_txt:
    raise SystemExit("C-0t must not patch Call analyze")

nail = Path("crates/syx-driver/tests/lang/type_soundness.rs").read_text(encoding="utf-8")
for needle in (
    "c0t_untyped_enum_case_member_arg_int_s004",
    "c0t_literal_enum_case_arg_still_ok",
    "c0t_any_enum_case_arg_still_ok",
    "c0o_untyped_dataclass_init_arg_int_s004",
    "c0s_untyped_subscript_get_index_int_s004",
):
    if needle not in nail:
        raise SystemExit(f"lang nail missing {needle}")

print("c0t_surface=1")
print("c0t_lang_nail=1")
print("c0t_soft_arms=1")
PY

python3 - <<'PY' || die "ledger / census hygiene failed"
from pathlib import Path

census = Path("docs/census/c-0t-enum-case-payload-unknown-sink-census.md")
if not census.is_file():
    raise SystemExit("missing c-0t census")
ct = census.read_text(encoding="utf-8")
for needle in ("C-0t", "S004", "跨轴", "enum", "payload"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")

rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("**C-0t**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-0t [x]")
if not any("**C-0a+**" in l and "park" in l for l in rm.splitlines()):
    raise SystemExit("02 must keep C-0a+ park")

st = Path("plan/01-project-status.md").read_text(encoding="utf-8")
if "C-0t" not in st:
    raise SystemExit("01 must cite C-0t")

ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
next_sec = ssot.split("## Next", 1)[-1]
if "跨轴" not in next_sec or "C-0t" not in next_sec:
    raise SystemExit("SSOT Next must cite post-C-0t 跨轴")

cl = Path("CHANGELOG.md").read_text(encoding="utf-8")
head = cl.split("### SYX 0.4.0", 1)[-1].split("###", 1)[0]
if "C-0t" not in head:
    raise SystemExit("CHANGELOG must mention C-0t")

print("c0t_census_doc=1")
print("c0t_roadmap_ok=1")
print("c0t_ssot_updated=1")
PY

echo "== nails cargo =="
cargo test -p syx-driver --test lang -- c0t_ --nocapture >/tmp/c0t_lang.txt 2>&1 \
  || die "c0t lang nails failed"
if ! grep -q "c0t_untyped_enum_case_member_arg_int_s004 ... ok" /tmp/c0t_lang.txt; then
  die "c0t S004 nail did not pass"
fi
ok "c0t_lang" "1"
cargo test -p syx-driver --test lang -- c0o_ --nocapture >/tmp/c0t_c0o.txt 2>&1 || die "c0o regress failed"
cargo test -p syx-driver --test lang -- c0s_ --nocapture >/tmp/c0t_c0s.txt 2>&1 || die "c0s regress failed"
ok "c0o_s_regress" "1"
tail -20 /tmp/c0t_lang.txt

cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
SYX=./target/debug/syx

PROBE=$(mktemp /tmp/c0t_probe_XXXX.sy)
cat >"$PROBE" <<'EOF'
enum E { wrap(n: Int) }
func take(_ x) { E.wrap(n: x) }
func main() { take(1) }
EOF
set +e
out=$("$SYX" check "$PROBE" --enable-std=false 2>&1)
ec=$?
set -e
if [[ "$ec" -eq 0 ]] || ! echo "$out" | grep -q '\[S004\]'; then
  echo "$out" >&2
  die "untyped enum case member arg must S004"
fi
if echo "$out" | grep -q 'Unknown'; then
  echo "$out" >&2
  die "diag must not print literal Unknown"
fi
ok "probe_member_arg" "S004"
rm -f "$PROBE"

LIT=$(mktemp /tmp/c0t_lit_XXXX.sy)
cat >"$LIT" <<'EOF'
enum E { wrap(n: Int) }
func main() { E.wrap(n: 1) }
EOF
"$SYX" check "$LIT" --enable-std=false >/tmp/c0t_lit_check.txt 2>&1 || {
  cat /tmp/c0t_lit_check.txt
  die "literal enum case arg must still ok"
}
ok "probe_literal" "0"
rm -f "$LIT"

ANY=$(mktemp /tmp/c0t_any_XXXX.sy)
cat >"$ANY" <<'EOF'
enum E { wrap(n: Any) }
func take(_ x) { E.wrap(n: x) }
func main() { take(1) }
EOF
"$SYX" check "$ANY" --enable-std=false >/tmp/c0t_any_check.txt 2>&1 || {
  cat /tmp/c0t_any_check.txt
  die "Any enum case arg must still ok"
}
ok "probe_any" "0"
rm -f "$ANY"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c0t_book_out.txt 2>&1 || die "opt-in output pins failed"
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
[[ "$browser_failed" -eq 0 ]] || die "browser_failed must be 0"
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
[[ "$wasm_failed" -eq 0 && "$wasm_passed" -eq 46 ]] || die "wasm must be 46/46"
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
print(f"simd_object_residual={len(obj_re.findall(code))}")
sys.exit(0 if len(obj_re.findall(code)) >= 159 else 1)
PY

if [[ "$fail" -ne 0 ]]; then
  echo "c0t_enum_case_payload_unknown_sink_census_ok=0"
  exit 1
fi
echo "c0t_enum_case_payload_unknown_sink_census_ok=1"
echo "current=C-0t-enum-case-payload-unknown-sink"
echo "next=cross-axis"
