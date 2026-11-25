#!/usr/bin/env bash
# C-5a macro/embed threat-model census.
# Soft park: C-5b embed fail-closed · ModuleId · tls.server · macOS · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-5a threat-model census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "threat-model / claim scrub contract broken"
from pathlib import Path
import re

adr = Path("docs/adr/macro-embed-threat-model.md")
if not adr.is_file():
    raise SystemExit("missing macro-embed-threat-model ADR")
at = adr.read_text(encoding="utf-8")
for needle in (
    "crash-containment reexec",
    "soft resource",
    "HostFn S024",
    "label/probe until wired",
    "C-5b",
):
    if needle not in at:
        raise SystemExit(f"ADR missing {needle}")

arch = Path("docs/adr/architecture-decisions.md").read_text(encoding="utf-8")
if "macro-embed-threat-model.md" not in arch or "reexec-only" not in arch:
    raise SystemExit("architecture-decisions must link threat-model / reexec-only")

cli = Path("crates/syx-cli/src/main.rs").read_text(encoding="utf-8")
# Find embed-sandbox doc comment near the flag
m = re.search(r"///([^\n]*)\n\s*#\[arg\(long = \"embed-sandbox\"", cli)
if not m:
    raise SystemExit("missing --embed-sandbox flag")
help_line = m.group(1).lower()
if "isolation" in help_line and "not" not in help_line and "≠" not in m.group(1):
    # Allow "not OS isolation" honesty; forbid bare "sandbox isolation" claim
    if "sandbox isolation" in help_line or "enable embed sandbox isolation" in help_line:
        raise SystemExit("CLI help must not claim sandbox isolation")
if "sandbox isolation" in help_line:
    raise SystemExit("CLI help still says sandbox isolation")

ug = Path("docs/9_UsageGuide.md").read_text(encoding="utf-8")
# Embed API section honesty
emb = ug.split("### Embed API", 1)[-1].split("### ", 1)[0]
if "Not** OS isolation" not in emb and "not** OS isolation" not in emb.lower() and "Not OS isolation" not in emb:
    if "macro-embed-threat-model" not in emb:
        raise SystemExit("UsageGuide Embed must link threat-model / deny OS isolation claim")
if "sandbox / timeout / memory & RSS budget / capability allowlist" in emb:
    raise SystemExit("UsageGuide still has old capability-list isolation claim")

qr = Path("docs/8_QuickReference.md").read_text(encoding="utf-8")
if "MVP hardening" in qr:
    raise SystemExit("QuickReference must not say MVP hardening")
if "SYX_EMBED_SECCOMP" in qr and "label/probe" not in qr and "unwired" not in qr:
    raise SystemExit("QuickReference SECCOMP row must say label/probe or unwired")

rep = Path("docs/6_ComprehensiveAnalysisReport.md").read_text(encoding="utf-8")
if "Sandboxed scripts via" in rep:
    raise SystemExit("ComprehensiveAnalysisReport must not claim Sandboxed scripts")
if "seccomp hardening" in rep:
    raise SystemExit("ComprehensiveAnalysisReport must not claim seccomp hardening")

ext = Path("crates/syx-macro/src/external_sandbox.rs").read_text(encoding="utf-8")
if 'return "seccomp"' in ext or '{\n        "seccomp"\n' in ext or '"seccomp"\n    } else' in ext:
    raise SystemExit('macro_sandbox_mode must not return bare "seccomp"')
# Exact return string check via regex on string literals in function body
fn = ext.split("pub fn macro_sandbox_mode", 1)[1].split("\npub fn ", 1)[0]
literals = re.findall(r'"([^"]+)"', fn)
if "seccomp" in literals:
    raise SystemExit('macro_sandbox_mode still returns exact "seccomp"')
# C-5a required seccomp-label-unwired; C-5c-2 evolves Linux path to seccomp-netdeny-wired
# (historical token may remain as in-fn literal). Accept either.
if "seccomp-label-unwired" not in literals and "seccomp-netdeny-wired" not in literals \
        and "MODE_SECCOMP_NETDENY_WIRED" not in Path("crates/syx-macro/src/external_sandbox.rs").read_text(encoding="utf-8"):
    raise SystemExit("macro_sandbox_mode must expose seccomp-label-unwired or seccomp-netdeny-wired")
if "sandbox-reexec-only" not in literals:
    raise SystemExit("macro_sandbox_mode must keep sandbox-reexec-only default")
if "seccomp subprocess sandbox" in ext.split("\n", 3)[0]:
    raise SystemExit("module header must not claim seccomp subprocess sandbox")

print("c5a_adr=1")
print("c5a_claim_scrub=1")
print("c5a_mode_label=1")
PY

python3 - <<'PY' || die "native=real / tls.server / ledger hygiene failed"
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

census = Path("docs/census/c-5a-threat-model-census.md")
if not census.is_file():
    raise SystemExit("missing c-5a census")
ct = census.read_text(encoding="utf-8")
for needle in ("C-5a", "C-5b", "threat-model", "seccomp-label-unwired"):
    if needle not in ct:
        raise SystemExit(f"census missing {needle}")

ssot = Path("docs/gates/backend-ci-matrix.md").read_text(encoding="utf-8")
if "C-5a (done" not in ssot:
    raise SystemExit("SSOT must mark C-5a done")
next_sec = ssot.split("## Next", 1)[-1]
if "C-5b" not in next_sec:
    raise SystemExit("SSOT Next must be C-5b")
if "+++" in next_sec:
    raise SystemExit("SSOT Next must not use +++")

rm = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("**C-5a**" in l and "[x]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-5a [x]")
if not any("### C-5 ·" in l and "[~]" in l for l in rm.splitlines()):
    raise SystemExit("02 must keep C-5 as [~]")
if not any("**C-5b**" in l and "[ ]" in l for l in rm.splitlines()):
    raise SystemExit("02 must mark C-5b [ ] as next")
if "### C-3 ·" not in rm or "[~]" not in rm:
    raise SystemExit("02 must keep C-3 as [~]")

st = Path("plan/01-project-status.md").read_text(encoding="utf-8")
if "C-5a" not in st or "C-5b" not in st:
    raise SystemExit("01 must cite C-5a / next C-5b")

cl = Path("CHANGELOG.md").read_text(encoding="utf-8")
head = cl.split("### SYX 0.4.0", 1)[-1].split("###", 1)[0]
if "C-5a" not in head:
    raise SystemExit("CHANGELOG 0.4.0 must mention C-5a")

print("c5a_native_real_count=16")
print("c5a_tls_server_partial=1")
print("c5a_census_doc=1")
print("c5a_ssot_updated=1")
print("c5a_roadmap_ok=1")
PY

echo "== c5a mode unit smoke =="
cargo test -p syx-macro p119_macro_sandbox_mode_exported -- --test-threads=1 >/tmp/c5a_mode.txt 2>&1 \
  || die "macro_sandbox_mode unit failed"
ok "c5a_mode_unit" "1"
tail -8 /tmp/c5a_mode.txt

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
# CLI help must not claim isolation
if "$SYX" --help 2>&1 | grep -qi "embed sandbox isolation"; then
  die "syx --help still claims embed sandbox isolation"
fi
ok "c5a_cli_help" "1"

python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c5a_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c5a_book_out.txt
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
  echo "c5a_threat_model_census_ok=0"
  exit 1
fi
echo "c5a_threat_model_census_ok=1"
echo "current=C-5a-threat-model"
echo "next=C-5b"
