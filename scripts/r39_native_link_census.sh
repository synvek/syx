#!/usr/bin/env bash
# R-39 full-native link fail-closed census.
# KPIs: Linux no ld64.lld · -Wl, normalized · full-native hello runs ·
#       link Err never prints Built native · browser_failed=0 · wasm 46/46 · object ≤186/11
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-39 native link census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

# --- Source policy: Linux name lists must not include ld64.lld ---
python3 - <<'PY' || die "ld64.lld still listed for Linux resolvers"
from pathlib import Path
files = [
    Path("crates/syx-rt/src/link.rs"),
    Path("crates/syx-lld/src/lib.rs"),
    Path("crates/syx-llvm-prefix/src/lib.rs"),
]
for path in files:
    text = path.read_text(encoding="utf-8")
    # Find linux cfg blocks and ensure ld64 is absent in the associated names slice.
    import re
    # Crude but effective: after `target_os = "linux"` within ~8 lines, names must not
    # contain ld64.lld.
    for m in re.finditer(r'#\[cfg\(target_os = "linux"\)\]\s*\n\s*let names:[^\n]+\n', text):
        start = m.end()
        window = text[start : start + 200]
        if "ld64.lld" in window:
            raise SystemExit(f"{path}: Linux names still include ld64.lld near {m.start()}")
    if "normalize_lld_link_flag" not in text and path.name == "link.rs":
        raise SystemExit("syx-rt missing normalize_lld_link_flag")
print("linux_ld64_banned=1")
PY

python3 - <<'PY' || die "normalize_lld_link_flag / Linux cc-first missing"
from pathlib import Path
link = Path("crates/syx-rt/src/link.rs").read_text(encoding="utf-8")
if "normalize_lld_link_flag" not in link:
    raise SystemExit("missing normalize_lld_link_flag")
if 'strip_prefix("-Wl,' not in link:
    raise SystemExit("missing -Wl, strip")
nb = Path("crates/syx-driver/src/native_build.rs").read_text(encoding="utf-8")
if "try_lld_first" not in nb:
    raise SystemExit("native_build missing try_lld_first (Linux cc-first)")
if 'target_os = "linux"' not in nb:
    raise SystemExit("native_build missing linux cfg for try_lld_first")
print("cc_first_and_wl_normalize=1")
PY

# --- Unit tests (host) ---
cargo test -p syx-rt normalize_lld -- --nocapture >/tmp/r39-rt-test.txt 2>&1 \
  || { tail -20 /tmp/r39-rt-test.txt; die "syx-rt normalize_lld tests failed"; }
ok "syx_rt_normalize_lld" "pass"

# --- full-native hello ---
if [[ ! -x target/debug/syx ]]; then
  cargo build -p syx-cli --features full-native
fi
# Ensure full-native feature is present (rebuild when needed).
cargo build -p syx-cli --features full-native >/tmp/r39-build-fn.txt 2>&1 \
  || { tail -30 /tmp/r39-build-fn.txt; die "full-native build failed"; }
ok "full_native_build" "pass"

TMP=$(mktemp -d)
set +e
bout=$(./target/debug/syx build --native examples/hello.sy -o "$TMP/hello" 2>&1)
brc=$?
set -e
ok "hello_native_build_rc" "$brc"
if [[ $brc -ne 0 ]]; then
  echo "$bout" | tail -20
  if echo "$bout" | grep -qi "Built native"; then
    die "printed Built native on failed link"
  fi
  die "full-native hello build failed"
fi
echo "$bout" | grep -q "Built native executable" || die "expected Built native executable on success"
set +e
rout=$("$TMP/hello" 2>&1)
rrc=$?
set -e
ok "hello_native_run_rc" "$rrc"
ok "hello_native_stdout" "$(echo "$rout" | tr '\n' ' ' | head -c 80)"
if [[ $rrc -ne 0 ]] || ! echo "$rout" | grep -q "Hello"; then
  die "full-native hello did not run/print Hello"
fi
rm -rf "$TMP"
ok "hello_full_native_run" "pass"

# Smoke path (hello require_run)
NATIVE_FULL=1 SYX=./target/debug/syx bash scripts/s0_backend_smoke.sh --native-full \
  >/tmp/r39-smoke.txt 2>&1 || { tail -40 /tmp/r39-smoke.txt; die "s0 smoke --native-full failed"; }
grep -q "native hello: PASS" /tmp/r39-smoke.txt || die "smoke missing native hello PASS"
ok "s0_native_full_hello" "pass"

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

# native CI jobs remain soft (R-37); do not reclaim this window
python3 - <<'PY' || die "native soft-fail policy drifted"
import re
from pathlib import Path
text = Path(".github/workflows/ci.yml").read_text()
blocks = re.split(r"\n(?=  [a-z0-9_-]+:)", text)
need = {"native-strict-main", "native-macos"}
seen = set()
for block in blocks:
    m = re.match(r"  ([a-z0-9_-]+):", block)
    if not m:
        continue
    name = m.group(1)
    if name in need:
        seen.add(name)
        if "continue-on-error: true" not in block:
            raise SystemExit(f"{name} lost soft-fail")
        print(f"soft_job_{name}=1")
if seen != need:
    raise SystemExit(f"missing soft native jobs: {need - seen}")
PY

ok "object_track" "parked_le_186_11"
ok "r39_native_link_census_ok" 1

if [[ "$fail" -ne 0 ]]; then
  exit 1
fi
echo "current=R-39-native-link"
echo "next=R-40-coldstart"
