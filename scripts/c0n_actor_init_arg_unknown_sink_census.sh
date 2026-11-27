#!/usr/bin/env bash
# C-0n Actor-init-arg Unknown→ground sink census (S004).
# Soft park: Module.Type · C-0a+ 三分 · close soft arm · C-0e · host/extension method GA · C-3/C-5 [x] · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-0n actor-init-arg-unknown-sink census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "C-0n surface missing"
from pathlib import Path

col = Path("crates/syx-semantic/src/semantic_collect_decls.rs").read_text(encoding="utf-8")
if "C-0n: Actor ctor params" not in col:
    raise SystemExit("Actor collect must host C-0n function_params")

call = Path("crates/syx-semantic/src/semantic_analyze_call.rs").read_text(encoding="utf-8")
if "C-0l/C-0n" not in call and "actor_types.contains(name)" not in call:
    raise SystemExit("Call must resolve actor ctor via {name}_init (C-0n)")
if "C-0n: Actor ctor labels" not in call:
    raise SystemExit("Call must check Actor ctor labels (C-0n)")

tc = Path("crates/syx-semantic/src/type_checker.rs").read_text(encoding="utf-8")
if "(_, TypeId::Unknown) => true" not in tc:
    raise SystemExit("must not remove (_, Unknown) soft arm")

nail = Path("crates/syx-driver/tests/lang/type_soundness.rs").read_text(encoding="utf-8")
for needle in (
    "c0n_untyped_actor_init_arg_int_s004",
    "c0n_literal_actor_init_arg_still_ok",
    "c0n_any_actor_init_arg_still_ok",
    "c0l_untyped_init_arg_int_s004",
    "c0m_untyped_class_subscript_set_int_s004",
):
    if needle not in nail:
        raise SystemExit(f"lang nail missing {needle}")

print("c0n_surface=1")
print("c0n_lang_nail=1")
print("c0n_soft_arms=1")
PY

python3 - <<'PY' || die "ledger / census hygiene failed"
from pathlib import Path

census = Path("docs/census/c-0n-actor-init-arg-unknown-sink-census.md")
if not census.is_file():
    raise SystemExit("missing c-0n census")
ct = census.read_text(encoding="utf-8")
for needle in ("C-0n", "S004", "跨轴", "Actor", "function_params"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")

rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("**C-0n**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-0n [x]")
if not any("**C-0a+**" in l and "park" in l for l in rm.splitlines()):
    raise SystemExit("02 must keep C-0a+ park")

st = Path("plan/01-project-status.md").read_text(encoding="utf-8")
if "C-0n" not in st:
    raise SystemExit("01 must cite C-0n")

ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
next_sec = ssot.split("## Next", 1)[-1]
if "跨轴" not in next_sec or "C-0n" not in next_sec:
    raise SystemExit("SSOT Next must cite post-C-0n 跨轴")

cl = Path("CHANGELOG.md").read_text(encoding="utf-8")
head = cl.split("### SYX 0.4.0", 1)[-1].split("###", 1)[0]
if "C-0n" not in head:
    raise SystemExit("CHANGELOG must mention C-0n")

print("c0n_census_doc=1")
print("c0n_roadmap_ok=1")
print("c0n_ssot_updated=1")
PY

echo "== nails cargo =="
cargo test -p syx-driver --test lang -- c0n_ --nocapture >/tmp/c0n_lang.txt 2>&1 \
  || die "c0n lang nails failed"
if ! grep -q "c0n_untyped_actor_init_arg_int_s004 ... ok" /tmp/c0n_lang.txt; then
  die "c0n S004 nail did not pass"
fi
ok "c0n_lang" "1"
tail -20 /tmp/c0n_lang.txt

cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
SYX=./target/debug/syx

PROBE=$(mktemp /tmp/c0n_probe_XXXX.sy)
cat >"$PROBE" <<'EOF'
actor Counter {
    init(_ n: Int) { }
}
func wrap(_ x) { Counter(x) }
func main() { wrap(1) }
EOF
set +e
out=$("$SYX" check "$PROBE" --enable-std=false 2>&1)
ec=$?
set -e
if [[ "$ec" -eq 0 ]] || ! echo "$out" | grep -q '\[S004\]'; then
  echo "$out" >&2
  die "untyped actor init-arg to Int must S004"
fi
if echo "$out" | grep -q 'Unknown'; then
  echo "$out" >&2
  die "diag must not print literal Unknown"
fi
ok "probe_actor_init" "S004"
rm -f "$PROBE"

LIT=$(mktemp /tmp/c0n_lit_XXXX.sy)
cat >"$LIT" <<'EOF'
actor Counter {
    init(_ n: Int) { }
}
func main() { Counter(1) }
EOF
"$SYX" check "$LIT" --enable-std=false >/tmp/c0n_lit_check.txt 2>&1 || {
  cat /tmp/c0n_lit_check.txt
  die "literal actor init-arg must still ok"
}
ok "probe_literal" "0"
rm -f "$LIT"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c0n_book_out.txt 2>&1 || die "opt-in output pins failed"
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
  echo "c0n_actor_init_arg_unknown_sink_census_ok=0"
  exit 1
fi
echo "c0n_actor_init_arg_unknown_sink_census_ok=1"
echo "current=C-0n-actor-init-arg-unknown-sink"
echo "next=cross-axis"
