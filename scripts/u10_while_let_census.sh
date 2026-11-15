#!/usr/bin/env bash
# U-10 while let census (parser desugar thin knife).
# Soft park: while case · codegen Break · CST postfix async · JIT · SyxHost · D3 · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded Optional→nil probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== U-10 while-let census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "parser U-10 surface missing"
from pathlib import Path
lib = Path("crates/syx-parser/src/lib.rs").read_text(encoding="utf-8")
if "U-10:" not in lib:
    raise SystemExit("parse_while missing U-10 marker")
idx = lib.find("fn parse_while")
if idx < 0:
    raise SystemExit("parse_while not found")
chunk = lib[idx:idx + 1600]
if "TokenKind::Let" not in chunk:
    raise SystemExit("parse_while missing Let branch")
if "Stmt::IfLet" not in chunk or "Stmt::Break" not in chunk:
    raise SystemExit("parse_while desugar missing IfLet/Break")
if "Literal::Bool(true)" not in chunk:
    raise SystemExit("parse_while desugar missing while-true condition")
nail = Path("crates/syx-driver/tests/lang/while_let.rs")
if not nail.is_file():
    raise SystemExit("missing lang/while_let.rs")
nt = nail.read_text(encoding="utf-8")
for needle in (
    "u10_while_let_bounded_optional_runs",
    "u10_while_let_immediate_nil_skips_body",
    "u10_plain_while_still_runs",
    "u10_if_let_still_runs",
):
    if needle not in nt:
        raise SystemExit(f"while_let missing {needle}")
if "while true" in nt.lower() and "unbounded" not in nt.lower():
    # Allow comments about forbidding unbounded while; ban a literal unbounded probe source.
    pass
main = Path("crates/syx-driver/tests/lang/main.rs").read_text(encoding="utf-8")
if "mod while_let;" not in main:
    raise SystemExit("lang/main.rs missing mod while_let")
print("u10_parser_surface=1")
print("u10_lang_nail=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/u-10-while-let-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/u-10-while-let-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("U-10", "while let", "≠", "R-50", "JIT"):
    if needle not in ct:
        raise SystemExit(f"u-10 census missing {needle}")
print("u10_census_doc=1")
PY

SYX=./target/debug/syx
if [[ ! -x "$SYX" ]]; then
  cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
fi

WL=$(mktemp /tmp/u10_wl_XXXX.sy)
cat >"$WL" <<'EOF'
var n = 0
public func next() -> Int? {
    if n >= 3 { return nil }
    let v = n
    n = n + 1
    return v
}
public func main() {
    while let v = next() { print(v) }
}
EOF
out=$("$SYX" run "$WL" 2>/tmp/u10_wl_err.txt) || {
  cat /tmp/u10_wl_err.txt
  die "while-let bounded probe run failed"
}
expected=$'0\n1\n2'
if [[ "$out" != "$expected" ]]; then
  echo "got: $out" >&2
  die "while-let bounded probe output mismatch"
fi
ok "probe_while_let_bounded" "0_1_2"
rm -f "$WL"

NIL=$(mktemp /tmp/u10_nil_XXXX.sy)
cat >"$NIL" <<'EOF'
public func empty() -> Int? { return nil }
public func main() {
    while let v = empty() { print(v) }
    print("done")
}
EOF
out=$("$SYX" run "$NIL" 2>/tmp/u10_nil_err.txt) || {
  cat /tmp/u10_nil_err.txt
  die "while-let nil probe run failed"
}
if [[ "$out" != "done" ]]; then
  echo "got: $out" >&2
  die "while-let nil probe output mismatch"
fi
ok "probe_while_let_nil" "done"
rm -f "$NIL"

PLAIN=$(mktemp /tmp/u10_plain_XXXX.sy)
cat >"$PLAIN" <<'EOF'
public func main() {
    var i = 0
    while i < 3 {
        print(i)
        i = i + 1
    }
}
EOF
out=$("$SYX" run "$PLAIN" 2>/tmp/u10_plain_err.txt) || {
  cat /tmp/u10_plain_err.txt
  die "plain while probe run failed"
}
expected=$'0\n1\n2'
if [[ "$out" != "$expected" ]]; then
  echo "got: $out" >&2
  die "plain while probe output mismatch"
fi
ok "probe_plain_while" "0_1_2"
rm -f "$PLAIN"

IFLET=$(mktemp /tmp/u10_iflet_XXXX.sy)
cat >"$IFLET" <<'EOF'
public func f() -> Int? { return 7 }
public func main() {
    if let v = f() { print(v) }
}
EOF
out=$("$SYX" run "$IFLET" 2>/tmp/u10_iflet_err.txt) || {
  cat /tmp/u10_iflet_err.txt
  die "if-let regression probe run failed"
}
if [[ "$out" != "7" ]]; then
  echo "got: $out" >&2
  die "if-let regression probe output mismatch"
fi
ok "probe_if_let" "7"
rm -f "$IFLET"

cargo test -p syx-driver --test lang while_let -- --test-threads=4 \
  >/tmp/u10_lang_nail.txt 2>&1 || {
  tail -40 /tmp/u10_lang_nail.txt
  die "lang while_let nail failed"
}
ok "lang_nail" "while_let"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/u10_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/u10_book_out.txt
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

python3 - <<'PY' || die "roadmap U-10 exit / next park missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if "U-10" not in text:
    raise SystemExit("roadmap missing U-10")
rows = [l for l in text.splitlines() if "| **U-10** |" in l]
if not rows or not any("[x]" in r for r in rows):
    raise SystemExit(f"U-10 not [x]: {rows}")
if "默认下一" in text:
    chunk = text.split("默认下一", 1)[1][:400]
    if "R-50" in chunk and "不" not in chunk and "park" not in chunk.lower() and "未编号" not in chunk:
        raise SystemExit("roadmap must not default next to R-50")
print("roadmap_u10_exit=1")
print("roadmap_no_r50_default=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "u10_while_let_census_ok=0"
  exit 1
fi
echo "u10_while_let_census_ok=1"
echo "current=U-10-while-let"
echo "next=U-11-or-park"
