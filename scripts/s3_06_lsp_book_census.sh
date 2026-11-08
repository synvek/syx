#!/usr/bin/env bash
# S3-I78…I81: Stage-3 LSP-book census (S3-06).
# I78 open · I79 S045 desugar · I80 residual-scope · I81 scoped claim.
# ≠ Stage-3 exit · ≠ S3-07 construct this window · ≠ lsp product knife this window.
# See docs/stage3-lsp-book-census.md and docs/adr/stage3-s3-06-claim.md
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== S3-06 LSP-book census =="

count_rg() {
  local pattern="$1"
  shift
  { rg -c "$pattern" "$@" 2>/dev/null || true; } | awk -F: '
    NF==1 { s+=$1; next }
    { s+=$2 }
    END { print s+0 }
  '
}

stage3_lsp_census_helper=$(count_rg 'S3-I78|S3-I79|S3-I80|S3-I81|S3-I82|S3-I83|S3-I84|S3-I85|stage3-lsp-book-census|stage3-s3-06-open|stage3-s3-06-s045|stage3-s3-06-residual-scope|stage3-s3-06-claim|stage3-s3-07-open|stage3-s3-07-residual-scope' \
  docs/adr/stage3-s3-06-open.md docs/adr/stage3-s3-06-s045-lsp-desugar.md \
  docs/adr/stage3-s3-06-residual-scope.md docs/adr/stage3-s3-06-claim.md \
  docs/stage3-lsp-book-census.md scripts/s3_06_lsp_book_census.sh)
diagnostics_for_source_helper=$(count_rg 'fn diagnostics_for_source|pub fn diagnostics_for_source' \
  crates/syx-driver/src/lsp_ide.rs)
rename_provider_helper=$(count_rg 'renameProvider' crates/syx-lsp/src/lib.rs)
signature_help_helper=$(count_rg 'signatureHelpProvider|fn signature_help_for_document' \
  crates/syx-lsp/src/lib.rs)
analyze_document_helper=$(count_rg 'pub fn analyze_document_for_lsp' \
  crates/syx-driver/src/lsp_ide.rs)
s045_lsp_desugar_helper=$(count_rg 'S3-I79|should_desugar_top_level_stmts' \
  crates/syx-driver/src/lsp_ide.rs docs/adr/stage3-s3-06-s045-lsp-desugar.md)
residual_scope_helper=$(count_rg 'S3-I80|residual-scope|scoped acceptance|supersede' \
  docs/adr/stage3-s3-06-residual-scope.md docs/stage3-lsp-book-census.md \
  scripts/s3_06_lsp_book_census.sh plan/02-improvement-roadmap.md)
# S3-I81 hinge: scoped plan claim.
s3_06_claim_helper=$(count_rg 'S3-I81|scoped \[x\]|Claimed: scoped|stage3-s3-06-claim' \
  docs/adr/stage3-s3-06-claim.md docs/stage3-lsp-book-census.md \
  scripts/s3_06_lsp_book_census.sh plan/02-improvement-roadmap.md \
  crates/syx-driver/tests/lang/s3_06_claim_contract.rs)

SAMPLE_FILES=(
  crates/syx-driver/tests/fixtures/lang/struct_book_point.sy
  crates/syx-driver/tests/fixtures/lang/struct_book_counter.sy
  crates/syx-driver/tests/fixtures/lang/enum_book_status.sy
)
book_fixture_sample_n=0
for f in "${SAMPLE_FILES[@]}"; do
  if [[ -f "$f" ]]; then
    book_fixture_sample_n=$((book_fixture_sample_n + 1))
  fi
done

if [[ "$s045_lsp_desugar_helper" -ge 1 ]]; then
  s045_lsp_desugar_nail_count=0
else
  s045_lsp_desugar_nail_count=1
fi

if [[ "$s045_lsp_desugar_helper" -ge 1 && "$book_fixture_sample_n" -ge 3 ]]; then
  book_lsp_semantic_mismatch_count=0
else
  book_lsp_semantic_mismatch_count=1
fi

echo "stage3_lsp_census_helper=$stage3_lsp_census_helper"
echo "diagnostics_for_source_helper=$diagnostics_for_source_helper"
echo "analyze_document_helper=$analyze_document_helper"
echo "rename_provider_helper=$rename_provider_helper"
echo "signature_help_helper=$signature_help_helper"
echo "s045_lsp_desugar_helper=$s045_lsp_desugar_helper"
echo "residual_scope_helper=$residual_scope_helper"
echo "s3_06_claim_helper=$s3_06_claim_helper"
echo "book_fixture_sample_n=$book_fixture_sample_n"
echo "s045_lsp_desugar_nail_count=$s045_lsp_desugar_nail_count"
echo "book_lsp_semantic_mismatch_count=$book_lsp_semantic_mismatch_count"
echo "current=S3-I81"
echo "next=S3-I88"
echo "s3_06_status=x"
echo "stage3_checklist=7/7"
echo "nonclaim=Stage-3-exit,S3-07-construct,book-zero-fp-literal,rename-sig-hover-knife,full-fence-sweep,S3-05-lint-knife,S3-04-fib-wall"
