#!/usr/bin/env bash
# S2-I143: Stage-2 parser dual-track census (S2-04 open baseline).
# Does not unify parsers or delete syx-cst.
# See docs/stage2-parser-census.md and docs/adr/stage2-s2-04-open.md
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== S2-04 parser dual-track census =="

parser_loc=$(wc -l < crates/syx-parser/src/lib.rs | tr -d ' ')
cst_loc=$(wc -l < crates/syx-cst/src/lib.rs | tr -d ' ')
combined=$((parser_loc + cst_loc))

cst_cargo=$( (rg -l 'syx-cst' crates --glob 'Cargo.toml' 2>/dev/null || true) | wc -l | tr -d ' ')
parser_cargo=$( (rg -l 'syx-parser' crates --glob 'Cargo.toml' 2>/dev/null || true) | wc -l | tr -d ' ')

fmt_auto=$( (rg -n 'pub fn format_source_auto\b' crates/syx-fmt/src/lib.rs 2>/dev/null || true) | wc -l | tr -d ' ')
fmt_cst=$( (rg -n 'pub fn format_source_cst\b' crates/syx-fmt/src/lib.rs 2>/dev/null || true) | wc -l | tr -d ' ')
fmt_ast=$( (rg -n 'pub fn format_source\b' crates/syx-fmt/src/lib.rs 2>/dev/null || true) | wc -l | tr -d ' ')
cst001=$( (rg -n 'CST001' crates/syx-fmt/src/lib.rs 2>/dev/null || true) | wc -l | tr -d ' ')

echo "syx_parser_lib_loc=${parser_loc}"
echo "syx_cst_lib_loc=${cst_loc}"
echo "combined_grammar_loc=${combined}"
echo "cargo_toml_syx_cst_deps=${cst_cargo}"
echo "cargo_toml_syx_parser_deps=${parser_cargo}"
echo "fmt_format_source_auto_defs=${fmt_auto}"
echo "fmt_format_source_cst_defs=${fmt_cst}"
echo "fmt_format_source_defs=${fmt_ast}"
echo "fmt_CST001_mentions=${cst001}"
echo "== census done (dual-track baseline; ≠ S2-04 [x]) =="
