#!/usr/bin/env bash
# U-16 comma guard-let census (parser Seq desugar; book claim align).
# Soft park: comma while-let · bare guard · CST · Break · JIT · SyxHost · D3 · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== U-16 guard-let-comma census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "parser U-16 surface missing"
from pathlib import Path
lib = Path("crates/syx-parser/src/lib.rs").read_text(encoding="utf-8")
if "U-16:" not in lib:
    raise SystemExit("parse_guard_let missing U-16 marker")
idx = lib.find("fn parse_guard_let")
if idx < 0:
    raise SystemExit("parse_guard_let not found")
chunk = lib[idx:idx + 4500]
if "TokenKind::Comma" not in chunk:
    raise SystemExit("parse_guard_let missing Comma loop")
if "trailing_cond" not in chunk:
    raise SystemExit("parse_guard_let missing trailing_cond")
if "Stmt::Seq" not in chunk:
    raise SystemExit("parse_guard_let missing Stmt::Seq")
if "else_branch.clone()" not in chunk:
    raise SystemExit("parse_guard_let missing else clone")
nail = Path("crates/syx-driver/tests/lang/guard_let_comma.rs")
if not nail.is_file():
    raise SystemExit("missing lang/guard_let_comma.rs")
nt = nail.read_text(encoding="utf-8")
for needle in (
    "u16_comma_both_some_continues",
    "u16_comma_first_nil_takes_else",
    "u16_comma_second_nil_takes_else",
    "u16_comma_trailing_cond_true",
    "u16_comma_trailing_cond_false_takes_else",
    "u16_single_guard_let_still_runs",
    "u16_if_let_comma_still_runs",
):
    if needle not in nt:
        raise SystemExit(f"guard_let_comma missing {needle}")
main = Path("crates/syx-driver/tests/lang/main.rs").read_text(encoding="utf-8")
if "mod guard_let_comma;" not in main:
    raise SystemExit("lang/main.rs missing mod guard_let_comma")
print("u16_parser_surface=1")
print("u16_lang_nail=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/u-16-guard-let-comma-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/u-16-guard-let-comma-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("U-16", "guard let", "≠", "R-50", "JIT", "书"):
    if needle not in ct:
        raise SystemExit(f"u-16 census missing {needle}")
print("u16_census_doc=1")
PY

SYX=./target/debug/syx
# Always rebuild so parser U-16 arm is not shadowed by a stale CLI binary.
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

BOTH=$(mktemp /tmp/u16_both_XXXX.sy)
cat >"$BOTH" <<'EOF'
public func main() {
    let a: Int? = 1
    let b: Int? = 2
    guard let x = a, let y = b else {
        print("else")
        return
    }
    print(x)
    print(y)
}
EOF
out=$("$SYX" run "$BOTH" 2>/tmp/u16_both_err.txt) || {
  cat /tmp/u16_both_err.txt
  die "both-some probe run failed"
}
expected=$'1\n2'
if [[ "$out" != "$expected" ]]; then
  echo "got: $out" >&2
  die "both-some probe output mismatch"
fi
ok "probe_both_some" "1_2"
rm -f "$BOTH"

FNIL=$(mktemp /tmp/u16_fnil_XXXX.sy)
cat >"$FNIL" <<'EOF'
public func main() {
    let a: Int? = nil
    let b: Int? = 2
    guard let x = a, let y = b else {
        print("else")
        return
    }
    print("no")
}
EOF
out=$("$SYX" run "$FNIL" 2>/tmp/u16_fnil_err.txt) || {
  cat /tmp/u16_fnil_err.txt
  die "first-nil probe run failed"
}
if [[ "$out" != "else" ]]; then
  echo "got: $out" >&2
  die "first-nil probe output mismatch"
fi
ok "probe_first_nil" "else"
rm -f "$FNIL"

SNIL=$(mktemp /tmp/u16_snil_XXXX.sy)
cat >"$SNIL" <<'EOF'
public func main() {
    let a: Int? = 1
    let b: Int? = nil
    guard let x = a, let y = b else {
        print("else")
        return
    }
    print("no")
}
EOF
out=$("$SYX" run "$SNIL" 2>/tmp/u16_snil_err.txt) || {
  cat /tmp/u16_snil_err.txt
  die "second-nil probe run failed"
}
if [[ "$out" != "else" ]]; then
  echo "got: $out" >&2
  die "second-nil probe output mismatch"
fi
ok "probe_second_nil" "else"
rm -f "$SNIL"

CT=$(mktemp /tmp/u16_ct_XXXX.sy)
cat >"$CT" <<'EOF'
public func main() {
    let a: Int? = 3
    guard let x = a, x > 0 else {
        print("else")
        return
    }
    print(x)
}
EOF
out=$("$SYX" run "$CT" 2>/tmp/u16_ct_err.txt) || {
  cat /tmp/u16_ct_err.txt
  die "trailing-true probe run failed"
}
if [[ "$out" != "3" ]]; then
  echo "got: $out" >&2
  die "trailing-true probe output mismatch"
fi
ok "probe_trailing_true" "3"
rm -f "$CT"

CF=$(mktemp /tmp/u16_cf_XXXX.sy)
cat >"$CF" <<'EOF'
public func main() {
    let a: Int? = 3
    guard let x = a, x < 0 else {
        print("else")
        return
    }
    print("no")
}
EOF
out=$("$SYX" run "$CF" 2>/tmp/u16_cf_err.txt) || {
  cat /tmp/u16_cf_err.txt
  die "trailing-false probe run failed"
}
if [[ "$out" != "else" ]]; then
  echo "got: $out" >&2
  die "trailing-false probe output mismatch"
fi
ok "probe_trailing_false" "else"
rm -f "$CF"

SINGLE=$(mktemp /tmp/u16_single_XXXX.sy)
cat >"$SINGLE" <<'EOF'
public func main() {
    let a: Int? = 7
    guard let x = a else { return }
    print(x)
}
EOF
out=$("$SYX" run "$SINGLE" 2>/tmp/u16_single_err.txt) || {
  cat /tmp/u16_single_err.txt
  die "single guard-let probe run failed"
}
if [[ "$out" != "7" ]]; then
  echo "got: $out" >&2
  die "single guard-let probe output mismatch"
fi
ok "probe_single_guard_let" "7"
rm -f "$SINGLE"

IFCOMMA=$(mktemp /tmp/u16_ifcomma_XXXX.sy)
cat >"$IFCOMMA" <<'EOF'
public func main() {
    let a: Int? = 1
    let b: Int? = 2
    if let x = a, let y = b {
        print(x)
        print(y)
    }
}
EOF
out=$("$SYX" run "$IFCOMMA" 2>/tmp/u16_ifcomma_err.txt) || {
  cat /tmp/u16_ifcomma_err.txt
  die "if-let comma probe run failed"
}
expected=$'1\n2'
if [[ "$out" != "$expected" ]]; then
  echo "got: $out" >&2
  die "if-let comma probe output mismatch"
fi
ok "probe_if_let_comma" "1_2"
rm -f "$IFCOMMA"

cargo test -p syx-driver --test lang guard_let_comma -- --test-threads=4 \
  >/tmp/u16_lang_nail.txt 2>&1 || {
  tail -40 /tmp/u16_lang_nail.txt
  die "lang guard_let_comma nail failed"
}
ok "lang_nail" "guard_let_comma"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/u16_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/u16_book_out.txt
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

python3 - <<'PY' || die "roadmap U-16 exit / next park missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if "U-16" not in text:
    raise SystemExit("roadmap missing U-16")
rows = [l for l in text.splitlines() if "| **U-16** |" in l]
if not rows or not any("[x]" in r for r in rows):
    raise SystemExit(f"U-16 not [x]: {rows}")
if "默认下一" in text:
    chunk = text.split("默认下一", 1)[1][:500]
    if "R-50" in chunk and "不" not in chunk and "park" not in chunk.lower() and "未编号" not in chunk:
        raise SystemExit("roadmap must not default next to R-50")
print("roadmap_u16_exit=1")
print("roadmap_no_r50_default=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "u16_guard_let_comma_census_ok=0"
  exit 1
fi
echo "u16_guard_let_comma_census_ok=1"
echo "current=U-16-guard-let-comma"
echo "next=park"
