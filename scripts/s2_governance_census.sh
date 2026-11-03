#!/usr/bin/env bash
# S2-I1: Stage-2 governance surface census (product crates/).
# Baseline for S2-01 — does not delete anything.
# See docs/stage2-governance-census.md and docs/adr/stage2-open.md
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== S2-01 governance census (crates/) =="

# rg exits 1 on zero matches; keep census printable at inventory=0 (S2-I49+).
inventory_fn=$( (rg -n --type rust --glob '!**/target/**' 'fn[[:space:]]+\w+_inventory\b' crates 2>/dev/null || true) | wc -l | tr -d ' ')
fingerprint_const=$( (rg -n --type rust --glob '!**/target/**' '_FINGERPRINT\b|FINGERPRINT[[:space:]]*:' crates 2>/dev/null || true) | wc -l | tr -d ' ')
fingerprint_fn=$( (rg -n --type rust --glob '!**/target/**' 'fn[[:space:]]+\w+_fingerprint\b' crates 2>/dev/null || true) | wc -l | tr -d ' ')
driver_reports=$(find crates/syx-driver/src -maxdepth 1 -name '*_report.rs' 2>/dev/null | wc -l | tr -d ' ')
lang_props=$(find crates/syx-driver/src -maxdepth 1 -name 'lang_props_v*.rs' 2>/dev/null | wc -l | tr -d ' ')
always_true_false=$( (rg -n --type rust --glob '!**/target/**' 'Always `(true|false)`' crates 2>/dev/null || true) | wc -l | tr -d ' ')

echo "inventory_fn_matches=${inventory_fn}"
echo "fingerprint_const_matches=${fingerprint_const}"
echo "fingerprint_fn_matches=${fingerprint_fn}"
echo "syx_driver_src_report_rs=${driver_reports}"
echo "syx_driver_src_lang_props_v=${lang_props}"
echo "always_true_false_comment_matches=${always_true_false}"
echo "== census done =="
