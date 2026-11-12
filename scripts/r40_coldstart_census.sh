#!/usr/bin/env bash
# R-40 cold-start / cold-exec path honesty census.
# KPIs: clean-dir hello/fib cold+warm · embed tag match probe · browser_failed=0 · wasm 46/46 · object ≤186/11
# Non-gate: cold <0.3 s is NOT an exit criterion this window.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-40 coldstart honesty census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

if [[ -x target/release/syx ]]; then
  SYX="$ROOT/target/release/syx"
elif [[ -x target/debug/syx ]]; then
  SYX="$ROOT/target/debug/syx"
  echo "note: using debug syx (prefer release for wall-clock honesty)" >&2
else
  die "syx binary missing (build target/release/syx)"
  echo "r40_coldstart_census_ok=0"
  exit 1
fi
ok "syx" "$SYX"

ASSET="crates/syx-driver/assets/prelude_snapshot.bin"
if [[ ! -f "$ASSET" ]]; then
  die "missing $ASSET"
  prelude_embed_asset_bytes=0
else
  prelude_embed_asset_bytes=$(wc -c <"$ASSET" | tr -d ' ')
fi
ok "prelude_embed_asset_bytes" "$prelude_embed_asset_bytes"

# Tag match: embedded snapshot vs baked tag in syx binary (fingerprint miss → miss).
ASSET_TAG=$(
  python3 - "$ASSET" <<'PY'
import re, sys
from pathlib import Path
data = Path(sys.argv[1]).read_bytes()
idx = data.find(b"syx-cache-v1:")
if idx < 0:
    print("")
    raise SystemExit(0)
chunk = "".join(chr(b) if 32 <= b < 127 else "" for b in data[idx : idx + 220])
m = re.match(r"(syx-cache-v1:[0-9.]+:rustc [^:]+:[0-9a-f]+)", chunk)
print(m.group(1) if m else "")
PY
)
BIN_TAG=$(
  strings "$SYX" | awk '/^syx-cache-v1:/ { print; exit }' | tr -d '"' || true
)
# Normalize: BIN_TAG may carry trailing junk from string table
BIN_TAG_NORM=$(
  python3 -c 'import re,sys; s=sys.argv[1]; m=re.match(r"(syx-cache-v1:[0-9.]+:rustc [^:]+:[0-9a-f]+)", s); print(m.group(1) if m else s)' "$BIN_TAG"
)
ok "embed_asset_tag" "${ASSET_TAG:-(empty)}"
ok "embed_bin_tag" "${BIN_TAG_NORM:-(empty)}"
if [[ -n "$ASSET_TAG" && "$ASSET_TAG" == "$BIN_TAG_NORM" ]]; then
  embed_tag_match=1
else
  embed_tag_match=0
fi
ok "embed_tag_match" "$embed_tag_match"
if [[ "$embed_tag_match" -eq 0 ]]; then
  ok "embed_probe" "MISS (fingerprint drift — regen via scripts/gen_prelude_snapshot.sh)"
else
  ok "embed_probe" "HIT (tag match; empty-cache cold may still be multi-second)"
fi

wall_run() {
  # args: syx path → prints seconds on stdout
  python3 - "$1" "$2" <<'PY'
import subprocess, sys, time
syx, path = sys.argv[1], sys.argv[2]
t0 = time.perf_counter()
r = subprocess.run([syx, "run", path], capture_output=True, text=True)
dt = time.perf_counter() - t0
if r.returncode != 0:
    sys.stderr.write(r.stderr or r.stdout or "run failed\n")
    raise SystemExit(r.returncode)
print(f"{dt:.3f}")
PY
}

# Warm the syx binary pages once so first clean-dir sample is not dominated by OS page-in.
python3 - "$SYX" <<'PY' >/dev/null 2>&1 || true
import subprocess, sys, tempfile, os
from pathlib import Path
syx = sys.argv[1]
td = tempfile.mkdtemp()
src = Path(td) / "warmup.sy"
src.write_text("func main() { return 0 }\n", encoding="utf-8")
subprocess.run([syx, "run", str(src)], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
PY

# --- hello clean-dir cold + same-path warm ---
TD=$(mktemp -d)
cp examples/hello.sy "$TD/hello.sy"
HELLO_COLD=$(wall_run "$SYX" "$TD/hello.sy") || { die "hello cold run failed"; HELLO_COLD=fail; }
ok "hello_cold_s" "$HELLO_COLD"
HELLO_WARM=$(wall_run "$SYX" "$TD/hello.sy") || { die "hello warm run failed"; HELLO_WARM=fail; }
ok "hello_warm_s" "$HELLO_WARM"
rm -rf "$TD"

# --- fib(30) clean-dir cold + same-path warm ---
TD=$(mktemp -d)
cp examples/fib.sy "$TD/fib.sy"
FIB_COLD=$(wall_run "$SYX" "$TD/fib.sy") || { die "fib cold run failed"; FIB_COLD=fail; }
ok "fib30_cold_s" "$FIB_COLD"
FIB_WARM=$(wall_run "$SYX" "$TD/fib.sy") || { die "fib warm run failed"; FIB_WARM=fail; }
ok "fib30_warm_s" "$FIB_WARM"
rm -rf "$TD"

# Honesty policy: do NOT fail if cold >> 0.3; do fail if numbers missing/non-numeric.
python3 - <<PY || die "coldstart numbers invalid"
cold = float("$HELLO_COLD")
warm = float("$HELLO_WARM")
fc = float("$FIB_COLD")
fw = float("$FIB_WARM")
print(f"hello_cold_vs_warm cold={cold} warm={warm}")
print(f"fib30_cold_vs_warm cold={fc} warm={fw}")
# Product E1 <0.3 is NOT a gate. Record only.
if cold < 0.3:
    print("note: hello_cold_s <0.3 (unusual vs R-38 ~2.6s); still not claiming GA")
else:
    print("note: hello_cold_s >=0.3 — Stage-3 <0.15 claim superseded; honesty OK")
PY

# Surfaces: census docs + Stage-3 demotion markers
python3 - <<'PY' || die "R-40 census / demotion docs missing"
from pathlib import Path
c = Path("docs/census/r-40-coldstart-census.md").read_text(encoding="utf-8")
for needle in ("hello_cold_s", "fib30_cold_s", "embed_tag_match", "superseded", "R-40"):
    if needle not in c:
        raise SystemExit(f"r-40 census missing {needle}")
print("r40_census_doc=1")
s3 = Path("docs/census/stage3-stdlib-cache-census.md").read_text(encoding="utf-8")
if "superseded by R-40" not in s3 and "Superseded by R-40" not in s3:
    raise SystemExit("stage3 census must mark superseded by R-40")
print("stage3_census_superseded=1")
adr = Path("docs/adr/stage3-s3-03-claim.md").read_text(encoding="utf-8")
if "superseded by R-40" not in adr and "Superseded by R-40" not in adr:
    raise SystemExit("S3-03 claim ADR must mark superseded by R-40")
print("stage3_claim_superseded=1")
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

# Roadmap exit pointer (P1 ledger row; progress summary may say `[x] 退出`)
python3 - <<'PY' || die "roadmap R-40 exit / R-41 default missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
rows = [l for l in text.splitlines() if "| **R-40** |" in l]
if not rows:
    raise SystemExit("missing R-40 roadmap row")
ok_x = any("`[x]`" in r or "| [x] |" in r for r in rows)
if not ok_x:
    raise SystemExit(f"R-40 not [x]: {rows}")
tail = text.split("默认下一窗", 1)
if len(tail) < 2 or "R-41" not in tail[1][:120]:
    raise SystemExit("roadmap must default next window to R-41")
print("roadmap_r40_exit=1")
print("roadmap_next_r41=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "r40_coldstart_census_ok=0"
  exit 1
fi
echo "r40_coldstart_census_ok=1"
