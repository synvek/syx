#!/usr/bin/env bash
# SA-2 language-surface census.
# Hinges: do/catch surface · packages/examples no data class · no trailing `-> T throws` · ProcessInfo nail.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== SA-2 language surface census =="

python3 - <<'PY'
import re, sys
from pathlib import Path

root = Path(".")
fail = 0

def die(msg):
    global fail
    print(f"FAIL: {msg}", file=sys.stderr)
    fail = 1

decl_dc = re.compile(
    r"^\s*(?:public|internal|private|fileprivate)?\s*(?:@\w+\s+)*data\s+class\b",
    re.M,
)
trailing_throws = re.compile(r"->\s*[^\n{]+?\s+throws\b")
stmt_try = re.compile(r"\btry\s*\{")

pkg_hits = []
for base in ("packages", "examples"):
    for p in Path(base).rglob("*.sy"):
        text = p.read_text(encoding="utf-8", errors="replace")
        for m in decl_dc.finditer(text):
            pkg_hits.append(f"{p}:{text[:m.start()].count(chr(10))+1}")
if pkg_hits:
    die("data class decls remain in packages/examples:\n  " + "\n  ".join(pkg_hits[:20]))
else:
    print("packages_examples_data_class=0")

trail = []
for p in Path("packages").rglob("*.sy"):
    text = p.read_text(encoding="utf-8", errors="replace")
    for m in trailing_throws.finditer(text):
        trail.append(f"{p}:{text[:m.start()].count(chr(10))+1}")
if trail:
    die("trailing -> T throws in packages:\n  " + "\n  ".join(trail[:20]))
else:
    print("packages_trailing_throws=0")

try_hits = []
for p in Path("packages").rglob("*.sy"):
    text = p.read_text(encoding="utf-8", errors="replace")
    for m in stmt_try.finditer(text):
        try_hits.append(f"{p}:{text[:m.start()].count(chr(10))+1}")
if try_hits:
    die("statement try { remains in packages:\n  " + "\n  ".join(try_hits[:20]))
else:
    print("packages_stmt_try=0")

os_mod = (root / "packages/os/mod.sy").read_text(encoding="utf-8", errors="replace")
if "enum ProcessInfo" not in os_mod or "processorCount" not in os_mod:
    die("packages/os/mod.sy missing ProcessInfo.processorCount")
else:
    print("processinfo_nail=1")

parser = (root / "crates/syx-parser/src/lib.rs").read_text(encoding="utf-8", errors="replace")
if "public data class is removed (SA-2-04)" not in parser:
    die("parser missing SA-2-04 public data class reject")
else:
    print("public_data_class_reject=1")
if "parse_do_catch" not in parser:
    die("parser missing parse_do_catch")
else:
    print("do_catch_parser=1")

lex = (root / "crates/syx-lexer/src/token.rs").read_text(encoding="utf-8", errors="replace")
if "Do" not in lex:
    die("lexer missing Do token")
else:
    print("lexer_do=1")

adr = root / "docs/adr/sa-2-01-do-catch.md"
if not adr.exists():
    die("missing docs/adr/sa-2-01-do-catch.md")
else:
    print("sa2_01_adr=1")

print("sa2_language_surface_helper=1")
if fail:
    sys.exit(1)
print("OK")
PY
