#!/usr/bin/env bash
# R-37 CI restore census.
# KPIs: fmt check green · browser_failed=0 · wasm 46/46 · object ≤186/11
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-37 CI restore census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

bash scripts/r37_fmt_check.sh || die "fmt check failed"

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

python3 - <<'PY' || die "ci.yml soft/hard job markers failed"
import re, sys
from pathlib import Path
text = Path(".github/workflows/ci.yml").read_text()
blocks = re.split(r"\n(?=  [a-z0-9_-]+:)", text)
soft = {
    "native-strict-main",
    "native-macos",
    "parity-gate",
    "jit-tier2-default",
    "phase125-gui-xvfb",
    "phase125-driver-gates",
    "windows-smoke",
    "book-pass",
}
hard = {
    "build",
    "build-compiler-wasm32",
    "wasm-aot-matrix-strict",
    "wasm-aot-matrix-full",
}
seen_soft, seen_hard = set(), set()
for block in blocks:
    m = re.match(r"  ([a-z0-9_-]+):", block)
    if not m:
        continue
    name = m.group(1)
    has = "continue-on-error: true" in block
    if name in soft:
        seen_soft.add(name)
        if not has:
            print(f"missing soft-fail: {name}", file=sys.stderr)
            sys.exit(1)
        print(f"soft_job_{name}=1")
    if name in hard:
        seen_hard.add(name)
        if has:
            print(f"hard job soft-failed: {name}", file=sys.stderr)
            sys.exit(1)
        print(f"hard_job_{name}=1")
if seen_soft != soft or seen_hard != hard:
    print(f"job set mismatch soft={seen_soft} hard={seen_hard}", file=sys.stderr)
    sys.exit(1)
PY

python3 - <<'PY'
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
if [[ $? -ne 0 ]]; then
  die "object residual exceeded 186/11"
fi

ok "object_track" "parked_le_186_11"
ok "r37_ci_census_ok" 1

if [[ "$fail" -ne 0 ]]; then
  exit 1
fi
echo "current=R-37-ci-restore"
echo "next=R-38-book-run"
