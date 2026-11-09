#!/usr/bin/env bash
# SA-1 prelude shrink census.
# Injected set == prelude_core.toml [target]/[current_injected].
# ≠ SA-3 rename · ≠ SA-2 do/catch.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== SA-1 prelude census =="

python3 - <<'PY'
import json, re, sys
from pathlib import Path

root = Path(".")
fail = 0

def die(msg):
    global fail
    print(f"FAIL: {msg}", file=sys.stderr)
    fail = 1

toml = (root / "packages/internal/prelude_core.toml").read_text()
assert 'status = "active"' in toml or "status = 'active'" in toml

def section_modules(name):
    m = re.search(rf"\[{re.escape(name)}\]\s*modules\s*=\s*\[(.*?)\]", toml, re.S)
    if not m:
        die(f"missing [{name}] modules")
        return []
    return re.findall(r'"([^"]+)"', m.group(1))

target = section_modules("target")
current = section_modules("current_injected")
remove = section_modules("remove_in_sa1")

cfg = (root / "crates/syx-config/src/lib.rs").read_text()
m = re.search(r"pub const CORE_PRELUDE_MODULES: &\[&str\] = &\[(.*?)\];", cfg, re.S)
core = re.findall(r'"([^"]+)"', m.group(1)) if m else []

manifest = json.loads((root / "packages/internal/manifest.json").read_text())
man_prelude = list(manifest["prelude_modules"])
man_importable = set(manifest["importable_modules"])

emb = (root / "crates/syx-driver/src/embedded_std.rs").read_text()
cm = re.search(r"const CORE: &\[\(&str, &str\)\] = &\[(.*?)\n\];", emb, re.S)
core_emb = re.findall(r'\(\s*"([^"]+)"\s*,', cm.group(1)) if cm else []
im = re.search(r"const IMPORTABLE: &\[\(&str, &str\)\] = &\[(.*?)\n\];", emb, re.S)
imp_emb = re.findall(r'\(\s*"([^"]+)"\s*,', im.group(1)) if im else []

print(f"target_count={len(target)}")
print(f"core_prelude_count={len(core)}")
print(f"embedded_core_count={len(core_emb)}")
print(f"manifest_prelude_count={len(man_prelude)}")

if current != target:
    die(f"current_injected != target\n  current={current}\n  target={target}")
else:
    print("target_current_match=1")

if core != target:
    die(f"CORE_PRELUDE_MODULES != target\n  config={core}\n  target={target}")
else:
    print("config_target_match=1")

if man_prelude != target:
    die(f"manifest prelude != target\n  manifest={man_prelude}\n  target={target}")
else:
    print("manifest_target_match=1")

if set(core_emb) != set(target):
    # CORE may be ordered differently; compare as sets + Concurrency present
    die(f"embedded CORE set != target\n  emb={core_emb}\n  target={target}")
else:
    print("embedded_core_match=1")

for name in remove:
    if name in core or name in man_prelude:
        die(f"{name} still in prelude")
    if name not in man_importable and name not in imp_emb:
        die(f"{name} not importable after leaving prelude")
print("removed_are_importable=1")

# Swift surfaces present in numeric sources
dbl = (root / "packages/core/builtins/numeric/double.sy").read_text()
intr = (root / "packages/core/builtins/numeric/int.sy").read_text()
if "static var pi" not in dbl and "pi:" not in dbl:
    die("Double.pi missing")
if "func random(in" not in intr:
    die("Int.random(in:) missing")
print("swift_surfaces=1")

print("sa1_prelude_helper=1")
print("current=SA-1-exit")
print("next=SA-2-01")
if fail:
    sys.exit(1)
print("OK")
PY
