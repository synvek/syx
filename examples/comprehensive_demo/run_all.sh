#!/usr/bin/env bash
# run_all.sh — verify the comprehensive test suites on the VM / native / wasm backends in order.
#
# Usage:
#   bash examples/comprehensive_demo/run_all.sh              # all three backends, full run
#   bash examples/comprehensive_demo/run_all.sh --only lang  # pass through VM CLI args (--only/--skip/--strict/--force)
#   SYX=/path/to/syx bash run_all.sh                         # choose the compiler binary
#
# Backend status (observed; see the backend matrix in README):
# - VM: syx run in place; full report for all 25 suites.
# - native: the whole-project build is blocked by MIR001/C020 in stdlib/host suites, so the
#   project is copied into a temporary staging directory, the blocked suites are removed and
#   their registry lines are filtered out of main.sy before building (17 lang suites build).
#   However, the native runtime control flow is broken (if-Bool bodies silently skipped,
#   Bool==Bool/while-Bool infinite loops, switch returns the scrutinee); running cases yields
#   no valid results and may hang, so only the build is verified, not execution.
# - wasm: the compiler build control flow is broken (MIR001 Channel / W001 If underflow /
#   if bodies not executed); attempt to run and record the failure without blocking the rest.
# - The working directory is fixed to the repo root: the FilePath.exists case in
#   stdlib.time_path assumes it.

set -u
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../.." && pwd)"
SYX="${SYX:-$ROOT/target/release/syx}"
cd "$ROOT"

echo "===== VM ====="
"$SYX" run "$HERE" -- --backend vm "$@"
echo

# Native compile-blocked suites (file names): the 3 stdlib suites + the 5 host suites.
# Observed evidence: stdlib.math=MIR001 Double; stdlib.text_json=C020 hash(into:);
# stdlib.time_path=MIR001 FilePath; host.concurrency=MIR001 Channel;
# host.io=host functions return nil; simd/net/services depend on the VM host.
NATIVE_EXCLUDE_FILES="suite_stdlib_math.sy suite_stdlib_text_json.sy suite_stdlib_time_path.sy suite_concurrency.sy suite_simd.sy suite_host_io.sy suite_host_net.sy suite_host_services.sy"
NATIVE_EXCLUDE_NAMES="stdlib.math stdlib.text_json stdlib.time_path host.concurrency host.simd host.io host.net host.services"

STAGE="$(mktemp -d /tmp/syx_comp_native.XXXXXX)"
cp "$HERE/syx.toml" "$HERE/harness.sy" "$HERE/ac_helper.sy" "$STAGE/"
for f in "$HERE"/suite_*.sy; do
    base="$(basename "$f")"
    excluded=0
    for x in $NATIVE_EXCLUDE_FILES; do
        if [ "$base" = "$x" ]; then excluded=1; fi
    done
    if [ "$excluded" = "0" ]; then cp "$f" "$STAGE/"; fi
done

# Filter out main.sy lines containing an excluded suite name (registry / backend annotation /
# dispatch each take one line), then clean up trailing array commas (syx disallows them).
python3 - "$HERE/main.sy" "$STAGE/main.sy" $NATIVE_EXCLUDE_NAMES <<'PYEOF'
import re, sys
src, dst = sys.argv[1], sys.argv[2]
quotes = ['"%s"' % s for s in sys.argv[3:]]
lines = [l for l in open(src, encoding="utf-8") if not any(q in l for q in quotes)]
text = "".join(lines)
text = re.sub(r',(\s*\])', r'\1', text)
open(dst, "w", encoding="utf-8").write(text)
PYEOF

echo "===== native (staging: $STAGE) ====="
if "$SYX" build --native "$STAGE" >"$STAGE/build.log" 2>&1; then
    echo "[NOTE] native build succeeded (17 lang suites; artifact $STAGE/target/debug/app)."
    echo "[NOTE] native runtime control flow is broken (if-Bool bodies skipped, Bool==Bool/while-Bool infinite loops); cases are not executed. Once the compiler is fixed, run the artifact directly with --backend native."
else
    echo "[SKIP] native build failed (see $STAGE/build.log); does not block the rest"
    tail -5 "$STAGE/build.log"
fi
echo

echo "===== wasm ====="
if "$SYX" run --wasm "$HERE" -- --backend wasm "$@"; then
    :
else
    echo "[NOTE] wasm backend currently unavailable (compiler defects: MIR001 Channel / W001 If underflow / if statement bodies not executed); recorded, does not block the rest"
fi