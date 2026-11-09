#!/usr/bin/env bash
# SA-0-03: stdlib SSOT census — prelude / importable / retired honesty.
# Charter: docs/adr/stdlib-swift-alignment.md
# ≠ SA-1 prelude shrink · ≠ module rename landing.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== SA-0 SSOT census =="

python3 - <<'PY'
import json, re, sys
from pathlib import Path

root = Path(".")
fail = 0

def die(msg):
    global fail
    print(f"FAIL: {msg}", file=sys.stderr)
    fail = 1

# --- files ---
adr = root / "docs/adr/stdlib-swift-alignment.md"
prelude_toml = root / "packages/internal/prelude_core.toml"
rename_toml = root / "packages/internal/module_rename_map.toml"
for p, key in [
    (adr, "stdlib_swift_alignment_adr"),
    (prelude_toml, "prelude_core_toml"),
    (rename_toml, "module_rename_map_toml"),
]:
    ok = 1 if p.is_file() else 0
    print(f"{key}={ok}")
    if not ok:
        die(f"missing {p}")

# --- CORE_PRELUDE_MODULES from syx-config ---
cfg = (root / "crates/syx-config/src/lib.rs").read_text()
m = re.search(
    r"pub const CORE_PRELUDE_MODULES: &\[&str\] = &\[(.*?)\];",
    cfg,
    re.S,
)
if not m:
    die("CORE_PRELUDE_MODULES not found")
    core_prelude = []
else:
    core_prelude = re.findall(r'"([^"]+)"', m.group(1))

# --- manifest ---
manifest = json.loads((root / "packages/internal/manifest.json").read_text())
man_prelude = list(manifest["prelude_modules"])
man_importable = list(manifest["importable_modules"])
man_retired = list(manifest["retired_modules"])

# --- IMPORTABLE names from embedded_std ---
emb = (root / "crates/syx-driver/src/embedded_std.rs").read_text()
# Grab the IMPORTABLE table body only
im = re.search(r"const IMPORTABLE: &\[\(&str, &str\)\] = &\[(.*?)\];", emb, re.S)
if not im:
    die("IMPORTABLE not found")
    emb_importable = []
else:
    # Names appear as ("IO", …) or (\n        "Crypto",\n        include_str!(…))
    emb_importable = re.findall(r'\(\s*"([^"]+)"\s*,', im.group(1))

# --- retired_ops from builtin_kind ---
bk = (root / "crates/syx-stdlib/src/builtin_kind.rs").read_text()
rm = re.search(
    r"pub fn retired_ops_module_names\(\) -> &'static \[ &'static str\] \{\s*&\[(.*?)\],?\s*\]\s*\}",
    bk,
    re.S,
)
# simpler: find function and take quoted names until closing of that fn's array
rm = re.search(
    r"pub fn retired_ops_module_names\(\)[^{]*\{(.*?)\n\}",
    bk,
    re.S,
)
body = rm.group(1) if rm else ""
# Drop // comments so narrative mentions of Text are not counted as retired names.
body = re.sub(r"//.*?$", "", body, flags=re.M)
retired_ops = re.findall(r'"([^"]+)"', body)

print(f"core_prelude_count={len(core_prelude)}")
print(f"manifest_prelude_count={len(man_prelude)}")
print(f"emb_importable_count={len(emb_importable)}")
print(f"manifest_importable_count={len(man_importable)}")

if core_prelude != man_prelude:
    die(f"prelude mismatch\n  config={core_prelude}\n  manifest={man_prelude}")
else:
    print("prelude_match=1")

if emb_importable != man_importable:
    die(
        f"importable mismatch\n  embedded={emb_importable}\n  manifest={man_importable}"
    )
else:
    print("importable_match=1")

# SA-1: Fmt left prelude; must stay importable.
if "Fmt" in man_prelude:
    die("Fmt must not remain in prelude_modules after SA-1")
if "Fmt" not in man_importable:
    die("Fmt missing from importable_modules")
print("fmt_importable=1")

for name in ("Tracing", "Metrics", "Profiling"):
    if name not in man_importable:
        die(f"{name} missing from importable_modules")
print("observability_in_importable=1")

if "Text" in man_retired and "Text" in man_importable:
    die("Text still dual-listed importable+retired")
if "Text" in man_retired:
    die("Text still in retired_modules")
if "Text" in retired_ops:
    die("Text still in retired_ops_module_names")
print("text_dual_cleared=1")

# prelude_core current_injected should match CORE_PRELUDE
pt = prelude_toml.read_text()
# extract current_injected modules array
cm = re.search(r"\[current_injected\]\s*modules\s*=\s*\[(.*?)\]", pt, re.S)
if not cm:
    die("prelude_core.toml missing [current_injected]")
    cur = []
else:
    cur = re.findall(r'"([^"]+)"', cm.group(1))
if cur != core_prelude:
    die(f"prelude_core current_injected != CORE_PRELUDE\n  toml={cur}\n  config={core_prelude}")
else:
    print("prelude_core_current_match=1")

print("sa0_ssot_helper=1")
print("current=SA-0-03")
print("next=SA-1-01")

if fail:
    sys.exit(1)
print("OK")
PY
