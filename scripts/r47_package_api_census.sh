#!/usr/bin/env bash
# R-47 package API census — System Env enum + FilePath struct + F3 gen-only.
# Soft park: SyxHost full wipe · HTTPClient · JIT · func…async.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-47 package API census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "Env/FilePath shape gate failed"
from pathlib import Path

def assert_env(path: Path) -> None:
    t = path.read_text(encoding="utf-8")
    if "public class Env" in t:
        raise SystemExit(f"{path}: Env still class")
    if "public enum Env" not in t:
        raise SystemExit(f"{path}: missing public enum Env")

def assert_filepath(path: Path) -> None:
    t = path.read_text(encoding="utf-8")
    if "public enum FilePath" in t and "public struct FilePath" not in t:
        raise SystemExit(f"{path}: FilePath still helper enum only")
    if "public struct FilePath" not in t:
        raise SystemExit(f"{path}: missing public struct FilePath")
    if "rawValue" not in t:
        raise SystemExit(f"{path}: FilePath missing rawValue")
    if "joining(" not in t:
        raise SystemExit(f"{path}: FilePath missing joining")

assert_env(Path("packages/core/env.sy"))
print("env_shape_packages_core_env.sy=enum")
assert_filepath(Path("packages/core/path.sy"))
print("filepath_shape_packages_core_path.sy=struct")

sys_p = Path("packages/system/mod.sy")
assert_env(sys_p)
assert_filepath(sys_p)
print("system_aggregate_shape=1")

pg = Path("crates/syx-driver/src/prelude_glue.rs").read_text(encoding="utf-8")
if "R-47: public structs" not in pg:
    raise SystemExit("prelude_glue missing R-47 Struct inject comment")
if "Decl::Struct" not in pg:
    raise SystemExit("prelude_glue missing Decl::Struct")
if "struct {name}" not in pg:
    raise SystemExit("prelude_glue missing struct import_exports")
print("struct_inject_export=1")
PY

python3 - <<'PY' || die "F3 gen-only README boundary missing"
from pathlib import Path
sys_r = Path("packages/system/README.md").read_text(encoding="utf-8")
net_r = Path("packages/networking/README.md").read_text(encoding="utf-8")
for needle in ("build-time", "gen", "import System"):
    if needle.lower() not in sys_r.lower() and needle not in sys_r:
        # allow case variants
        pass
for needle in ("build-time", "domain", "import System"):
    if needle not in sys_r:
        raise SystemExit(f"system README missing {needle}")
for needle in ("build-time", "domain", "import Networking"):
    if needle not in net_r:
        raise SystemExit(f"networking README missing {needle}")
print("f3_gen_only_readme=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/r-47-package-api-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/r-47-package-api-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("R-47", "Env", "FilePath", "struct", "≠", "R-48", "SyxHost"):
    if needle not in ct:
        raise SystemExit(f"r-47 census missing {needle}")
print("r47_census_doc=1")
PY

SYX=./target/debug/syx
if [[ ! -x "$SYX" ]]; then
  cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
fi

# Touched examples must check
for ex in examples/path_tool/main.sy examples/io_demo.sy examples/file_tool/main.sy; do
  "$SYX" check "$ex" >/tmp/r47_ex_check.txt 2>&1 || {
    tail -20 /tmp/r47_ex_check.txt
    die "check failed: $ex"
  }
  ok "check_ok" "$ex"
done

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/r47_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -20 /tmp/r47_book_out.txt
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

python3 - <<'PY' || die "roadmap R-47 exit / R-48 default missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
rows = [l for l in text.splitlines() if "| **R-47** |" in l]
if not rows or not any("`[x]`" in r or "| [x] |" in r for r in rows):
    raise SystemExit(f"R-47 not [x]: {rows}")
tail = text.split("默认下一窗", 1)
if len(tail) < 2 or "R-48" not in tail[1][:240]:
    raise SystemExit("roadmap must default next window to R-48")
print("roadmap_r47_exit=1")
print("roadmap_next_r48=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "r47_package_api_census_ok=0"
  exit 1
fi
echo "r47_package_api_census_ok=1"
echo "current=R-47-package-api"
echo "next=R-48-credibility-contracts"
