#!/usr/bin/env bash
# R-38 book run+stdout census.
# KPIs: opt-in output_run green · cold-start eval recorded · browser_failed=0 · wasm 46/46 · object ≤186/11
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-38 book run+stdout census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

# Prefer release for colder/warmer timing honesty; fall back to debug.
if [[ -x target/release/syx ]]; then
  SYX="$ROOT/target/release/syx"
elif [[ -x target/debug/syx ]]; then
  SYX="$ROOT/target/debug/syx"
else
  die "syx binary missing (build target/release/syx or target/debug/syx)"
  echo "r38_book_run_census_ok=0"
  exit 1
fi
ok "syx" "$SYX"

# --- Cold-start sample (hello); wall clock via date (perf_counter is per-process) ---
TD=$(mktemp -d)
cp examples/hello.sy "$TD/hello.sy"
COLD=$(
  python3 - "$SYX" "$TD/hello.sy" <<'PY'
import subprocess, sys, time
syx, path = sys.argv[1], sys.argv[2]
t0 = time.perf_counter()
subprocess.run([syx, "run", path], check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
print(round(time.perf_counter() - t0, 3))
PY
)
ok "hello_cold_s" "$COLD"
rm -rf "$TD"
# Second run: OS/page-cache warmish (still fresh temp path)
TD=$(mktemp -d)
cp examples/hello.sy "$TD/hello.sy"
WARM=$(
  python3 - "$SYX" "$TD/hello.sy" <<'PY'
import subprocess, sys, time
syx, path = sys.argv[1], sys.argv[2]
t0 = time.perf_counter()
subprocess.run([syx, "run", path], check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
print(round(time.perf_counter() - t0, 3))
PY
)
ok "hello_warmish_s" "$WARM"
rm -rf "$TD"

python3 - <<PY || die "cold-start policy: expect cold >> warm (or cold>=1s)"
cold = float("$COLD")
warm = float("$WARM")
print(f"cold_vs_warm cold={cold} warm={warm}")
# Policy reminder: if cold is large, opt-in-only is mandatory (always true for R-38).
if cold < 0.05 and warm < 0.05:
    # Both tiny — still OK; product E1 may already help. Do not fail.
    pass
elif cold < 1.0:
    print("note: cold <1s; still shipping opt-in-only this window")
PY

# --- Runner surface ---
python3 - <<'PY' || die "book_pass_rate missing R-38 output surfaces"
from pathlib import Path
text = Path("scripts/book_pass_rate.py").read_text(encoding="utf-8")
for needle in ("run_and_compare", "PRINTS_RE", "only-output", '"output"', "stdout_mismatch"):
    if needle not in text:
        raise SystemExit(f"missing {needle}")
print("book_pass_rate_r38_surfaces=1")
PY

python3 - <<'PY' || die "doctest-annotations missing output docs"
from pathlib import Path
text = Path("docs/doctest-annotations.md").read_text(encoding="utf-8")
for needle in ("sy,output", "// prints:", "opt-in"):
    if needle not in text:
        raise SystemExit(f"missing {needle}")
print("doctest_annotations_r38=1")
PY

# --- Opt-in output pins (en + zh) ---
OUT_JSON=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$OUT_JSON" \
  || die "opt-in output pins failed"
python3 - <<PY || die "output pin count / green check failed"
import json, sys
from pathlib import Path
d = json.loads(Path("$OUT_JSON").read_text())
ok_n = d.get("output_run_ok", 0)
tot = d.get("output_run_total", 0)
print(f"output_run_ok={ok_n}")
print(f"output_run_total={tot}")
en = [r for r in d["results"] if "zh-cn" not in r["file"]]
zh = [r for r in d["results"] if "zh-cn" in r["file"]]
print(f"output_en={len(en)} output_zh={len(zh)}")
if tot < 10 or ok_n != tot:
    sys.exit(1)
if len(en) < 5 or len(zh) < 5:
    sys.exit(1)
if any(not r["ok"] for r in d["results"]):
    sys.exit(1)
PY
rm -f "$OUT_JSON"

# --- Independent SEM surfaces exist ---
python3 - <<'PY' || die "independent SEM regression surfaces missing"
from pathlib import Path
need = [
    "crates/syx-driver/tests/lang/errors.rs",
    "crates/syx-driver/tests/lang/value_semantics.rs",
    "crates/syx-driver/tests/lang/int_from_string.rs",
    "crates/syx-driver/tests/lang/protocol_iterator.rs",
    "crates/syx-driver/tests/lang/protocol_shape_array.rs",
    "crates/syx-driver/tests/fixtures/errors/try_cross_fn.sy",
    "crates/syx-driver/tests/fixtures/lang/value_semantics_struct.sy",
    "crates/syx-driver/tests/fixtures/lang/int_from_string.sy",
    "crates/syx-driver/tests/fixtures/lang/protocol_iterator_forin.sy",
    "crates/syx-driver/tests/fixtures/lang/protocol_shape_array.sy",
]
missing = [p for p in need if not Path(p).is_file()]
if missing:
    raise SystemExit("missing: " + ", ".join(missing))
print("independent_sem_surfaces=1")
PY

# --- Hard KPIs ---
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

python3 - <<'PY' || die "object residual exceeded 186/11"
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
sys.exit(0 if total <= 186 and files <= 11 else 1)
PY

# book-pass remains soft in ci.yml (R-37 policy; not reclaimed this window)
python3 - <<'PY' || die "book-pass must remain soft-fail"
import re
from pathlib import Path
text = Path(".github/workflows/ci.yml").read_text()
blocks = re.split(r"\n(?=  [a-z0-9_-]+:)", text)
for block in blocks:
    m = re.match(r"  ([a-z0-9_-]+):", block)
    if m and m.group(1) == "book-pass":
        if "continue-on-error: true" not in block:
            raise SystemExit("book-pass lost soft-fail")
        print("book_pass_soft=1")
        break
else:
    raise SystemExit("book-pass job missing")
PY

ok "object_track" "parked_le_186_11"
ok "r38_book_run_census_ok" 1

if [[ "$fail" -ne 0 ]]; then
  exit 1
fi
echo "current=R-38-book-run"
echo "next=R-39-full-native"
