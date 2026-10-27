#!/usr/bin/env bash
# Behavioral test *count* gate — SMOKE / anti-delete floor only (ADR-132 / ADR-133 / M0-E07).
# Wave A W1 / ADR-345 / C-10: these floors are NOT release quality KPI and NOT coverage.
# Test buckets (M0-E01 / ADR-133):
#   semantic  — language/semantics gates
#   golden    — fixed expected output fixtures
#   differential — four-backend exact (`cargo test -p syx-driver --test differential`)
#   fingerprint — count/fingerprint smoke (this script); NOT GA proof
# Prefer golden + differential for Truth; fingerprint must not be treated as coverage.
set -euo pipefail

MIN_DRIVER=2600
MIN_LSP=3050

while [[ $# -gt 0 ]]; do
  case "$1" in
    --min-driver) MIN_DRIVER="$2"; shift 2 ;;
    --min-lsp) MIN_LSP="$2"; shift 2 ;;
    *) echo "Unknown arg: $1" >&2; exit 2 ;;
  esac
done

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

EXCLUDE='query_hash_matrix|lsp_phase64_|lsp_phase66_|lsp_phase67_|lsp_p63_smoke|semantic_embed_matrix.*tautology|_p65|behavioral_p65|lsp_behavioral_p65|p66_.*_[0-9]{2,}|p67_fn_def_[0-9]+|poly_v[0-9]+|iter_custom_[0-9]+|custom_iter_[0-9]+|test_count_marker|native_compile_ok!.*print\(1\)|p68_build_s401_\d+|p69_lsp_semantic_\d+|p70_lsp_gate_\d+|p71_lsp_gate_\d+|p71_sem_|p72_lsp_gate_\d+|p72_borrow_mat_|p73_heap_|p74_borrow_|p74_native_|p74_lsp_gate_\d+|p75_borrow_mat_|p75_lsp_gate_\d+|p76_borrow_sem_|p76_lsp_gate_\d+|p77_borrow_sem_|p77_lsp_gate_\d+|p78_sem_|p79_lsp_gate_\d+|p79_doc_ok_|p80_lsp_gate_\d+|p80_borrow_fail_ext_|p81_lsp_gate_\d+|p81_borrow_fail_|p82_lsp_gate_\d+|p83_lsp_gate_\d+|p84_lsp_gate_\d+|p85_lsp_gate_\d+|p86_lsp_gate_\d+|p87_lsp_gate_\d+|p88_lsp_gate_\d+|p104_.*_audit_probe_|p105_.*_audit_probe_|p104_.*_audit_extra|p105_.*_audit_extra|p105_gate0_|p106_gate0_|p106_behavioral_audit_fill|p106_behavioral_fill_|lsp_phase106_audit_fill|p106_lsp_fill_|p107_gate0_|p107_behavioral_arith|p107_lsp_semantic_|lsp_phase107_semantic_fill|p108_gate0_|p108_behavioral_cat|p108_semantic_extra_|p109_gate0_|p109_behavioral_real|p109_hover_extra_|p110_gate0_|p110_behavioral_cat|p110_sem_probe_|p111_gate0_|p111_behavioral_fill|p111_sem_gate_|p112_gate0_|p112_behavioral_cat|lsp_phase112_audit_extra|p112_audit_|p113_gate0_|p113_hover_bind_fill_|p114_gate0_|p114_hover_sem_|p115_gate0_|p115_real_bind_|p116_gate0_|p117_gate0_|p136_behavioral|p136_bind_b|p136_gate0_|p137_gate0_|p137_behavioral|p137_bind_b|p138_behavioral|p138_bind_b|p138_gate0_|p139_gate0_|p126_bind_b|p126_gate0_|p127_gate0_|behavioral_gate\.rs'

count_behavioral() {
  local path="$1"
  rg '#\[test\]' "$path" --count-matches 2>/dev/null \
    | rg -v "$EXCLUDE" \
    | awk -F: '{s+=$2} END {print s+0}'
}

DRIVER=$(count_behavioral "crates/syx-driver")
LSP=$(count_behavioral "crates/syx-lsp")

echo "behavioral driver tests: $DRIVER (min $MIN_DRIVER)"
echo "behavioral lsp tests:    $LSP (min $MIN_LSP)"

fail=0
if [[ "$DRIVER" -lt "$MIN_DRIVER" ]]; then
  echo "FAIL: behavioral driver $DRIVER < $MIN_DRIVER" >&2
  fail=1
fi
if [[ "$LSP" -lt "$MIN_LSP" ]]; then
  echo "FAIL: behavioral lsp $LSP < $MIN_LSP" >&2
  fail=1
fi

if [[ "$fail" -ne 0 ]]; then
  exit 1
fi
echo "OK: behavioral test counts meet Phase 139 feature-preserving audit gates"
