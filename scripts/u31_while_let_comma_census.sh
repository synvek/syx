#!/usr/bin/env bash
# U-31 comma while-let census (parser nest desugar; U-14/U-16 symmetry).
# Soft park: field-default nil · closure throws · CST · Break · JIT · SyxHost · D3 · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== U-31 while-let-comma census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "parser U-31 surface missing"
from pathlib import Path
lib = Path("crates/syx-parser/src/lib.rs").read_text(encoding="utf-8")
if "U-10 / U-31:" not in lib and "U-31:" not in lib:
    raise SystemExit("parse_while missing U-31 marker")
idx = lib.find("fn parse_while")
if idx < 0:
    raise SystemExit("parse_while not found")
chunk = lib[idx:idx + 4500]
if "TokenKind::Comma" not in chunk:
    raise SystemExit("while let arm missing Comma loop")
if "trailing_cond" not in chunk:
    raise SystemExit("while let arm missing trailing_cond")
if "break_arm" not in chunk and "Break" not in chunk:
    raise SystemExit("while let arm missing break else")
nail = Path("crates/syx-driver/tests/lang/while_let_comma.rs")
if not nail.is_file():
    raise SystemExit("missing lang/while_let_comma.rs")
nt = nail.read_text(encoding="utf-8")
for needle in (
    "u31_comma_both_some_enters_body",
    "u31_comma_first_nil_skips_body",
    "u31_comma_second_nil_skips_body",
    "u31_comma_trailing_cond_true",
    "u31_comma_trailing_cond_false_skips_body",
    "u31_single_while_let_still_runs",
    "u31_if_let_comma_still_runs",
):
    if needle not in nt:
        raise SystemExit(f"while_let_comma missing {needle}")
main = Path("crates/syx-driver/tests/lang/main.rs").read_text(encoding="utf-8")
if "mod while_let_comma;" not in main:
    raise SystemExit("lang/main.rs missing mod while_let_comma")
en = Path("books/language-guide/06-control-flow.md").read_text(encoding="utf-8")
zh = Path("books/zh-cn/language-guide/06-control-flow.md").read_text(encoding="utf-8")
if "while let x = a, let y = b" not in en:
    raise SystemExit("en Ch06 missing while let comma row")
if "while let x = a, let y = b" not in zh:
    raise SystemExit("zh Ch06 missing while let comma row")
print("u31_parser_surface=1")
print("u31_lang_nail=1")
print("u31_book_ch06=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/u-31-while-let-comma-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/u-31-while-let-comma-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("U-31", "while let", "≠", "R-50", "JIT"):
    if needle not in ct:
        raise SystemExit(f"u-31 census missing {needle}")
print("u31_census_doc=1")
PY

SYX=./target/debug/syx
# Always rebuild so parser U-31 arm is not shadowed by a stale CLI binary.
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"

BOTH=$(mktemp /tmp/u31_both_XXXX.sy)
cat >"$BOTH" <<'EOF'
var step = 0
public func left() -> Int? {
    if step >= 1 { return nil }
    return 1
}
public func right() -> Int? {
    return 2
}
public func main() {
    while let x = left(), let y = right() {
        print(x)
        print(y)
        step = step + 1
    }
}
EOF
out=$("$SYX" run "$BOTH" 2>/tmp/u31_both_err.txt) || {
  cat /tmp/u31_both_err.txt
  die "both-some probe run failed"
}
expected=$'1\n2'
if [[ "$out" != "$expected" ]]; then
  echo "got: $out" >&2
  die "both-some probe output mismatch"
fi
ok "probe_both_some" "1_2"
rm -f "$BOTH"

FNIL=$(mktemp /tmp/u31_fnil_XXXX.sy)
cat >"$FNIL" <<'EOF'
public func empty() -> Int? { return nil }
public func right() -> Int? { return 2 }
public func main() {
    while let x = empty(), let y = right() {
        print("no")
    }
    print("done")
}
EOF
out=$("$SYX" run "$FNIL" 2>/tmp/u31_fnil_err.txt) || {
  cat /tmp/u31_fnil_err.txt
  die "first-nil probe run failed"
}
if [[ "$out" != "done" ]]; then
  echo "got: $out" >&2
  die "first-nil probe output mismatch"
fi
ok "probe_first_nil" "done"
rm -f "$FNIL"

SNIL=$(mktemp /tmp/u31_snil_XXXX.sy)
cat >"$SNIL" <<'EOF'
public func left() -> Int? { return 1 }
public func empty() -> Int? { return nil }
public func main() {
    while let x = left(), let y = empty() {
        print("no")
    }
    print("done")
}
EOF
out=$("$SYX" run "$SNIL" 2>/tmp/u31_snil_err.txt) || {
  cat /tmp/u31_snil_err.txt
  die "second-nil probe run failed"
}
if [[ "$out" != "done" ]]; then
  echo "got: $out" >&2
  die "second-nil probe output mismatch"
fi
ok "probe_second_nil" "done"
rm -f "$SNIL"

CT=$(mktemp /tmp/u31_ct_XXXX.sy)
cat >"$CT" <<'EOF'
var n = 0
public func next() -> Int? {
    if n >= 1 { return nil }
    let v = 3
    n = n + 1
    return v
}
public func main() {
    while let x = next(), x > 0 {
        print(x)
    }
}
EOF
out=$("$SYX" run "$CT" 2>/tmp/u31_ct_err.txt) || {
  cat /tmp/u31_ct_err.txt
  die "trailing-true probe run failed"
}
if [[ "$out" != "3" ]]; then
  echo "got: $out" >&2
  die "trailing-true probe output mismatch"
fi
ok "probe_trailing_true" "3"
rm -f "$CT"

CF=$(mktemp /tmp/u31_cf_XXXX.sy)
cat >"$CF" <<'EOF'
public func once() -> Int? { return 3 }
public func main() {
    while let x = once(), x < 0 {
        print("no")
    }
    print("done")
}
EOF
out=$("$SYX" run "$CF" 2>/tmp/u31_cf_err.txt) || {
  cat /tmp/u31_cf_err.txt
  die "trailing-false probe run failed"
}
if [[ "$out" != "done" ]]; then
  echo "got: $out" >&2
  die "trailing-false probe output mismatch"
fi
ok "probe_trailing_false" "done"
rm -f "$CF"

SINGLE=$(mktemp /tmp/u31_single_XXXX.sy)
cat >"$SINGLE" <<'EOF'
var n = 0
public func next() -> Int? {
    if n >= 1 { return nil }
    let v = n
    n = n + 1
    return v
}
public func main() {
    while let v = next() {
        print(v)
    }
}
EOF
out=$("$SYX" run "$SINGLE" 2>/tmp/u31_single_err.txt) || {
  cat /tmp/u31_single_err.txt
  die "single while-let probe run failed"
}
if [[ "$out" != "0" ]]; then
  echo "got: $out" >&2
  die "single while-let probe output mismatch"
fi
ok "probe_single_while_let" "0"
rm -f "$SINGLE"

IFCOMMA=$(mktemp /tmp/u31_ifcomma_XXXX.sy)
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
out=$("$SYX" run "$IFCOMMA" 2>/tmp/u31_ifcomma_err.txt) || {
  cat /tmp/u31_ifcomma_err.txt
  die "if-let comma probe run failed"
}
expected=$'1\n2'
if [[ "$out" != "$expected" ]]; then
  echo "got: $out" >&2
  die "if-let comma probe output mismatch"
fi
ok "probe_if_let_comma" "1_2"
rm -f "$IFCOMMA"

cargo test -p syx-driver --test lang while_let_comma -- --test-threads=4 \
  >/tmp/u31_lang_nail.txt 2>&1 || {
  tail -40 /tmp/u31_lang_nail.txt
  die "lang while_let_comma nail failed"
}
ok "lang_nail" "while_let_comma"

out_json=$(mktemp)
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/u31_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/u31_book_out.txt
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

python3 - <<'PY' || die "roadmap U-31 exit / next park missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if "U-31" not in text:
    raise SystemExit("roadmap missing U-31")
rows = [l for l in text.splitlines() if "| **U-31** |" in l]
if not rows or not any("[x]" in r for r in rows):
    raise SystemExit(f"U-31 not [x]: {rows}")
if "默认下一" in text:
    chunk = text.split("默认下一", 1)[1][:500]
    if "R-50" in chunk and "不" not in chunk and "park" not in chunk.lower() and "未编号" not in chunk:
        raise SystemExit("roadmap must not default next to R-50")
print("roadmap_u31_exit=1")
print("roadmap_no_r50_default=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "u31_while_let_comma_census_ok=0"
  exit 1
fi
echo "u31_while_let_comma_census_ok=1"
echo "current=U-31-while-let-comma"
echo "next=park"
