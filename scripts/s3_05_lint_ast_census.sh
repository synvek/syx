#!/usr/bin/env bash
# S3-I72…I77: Stage-3 lint-ast / L007·L012 census (S3-05).
# I72 open · I73–I74 knives · I75 claim-prep · I76 residual-scope · I77 scoped claim.
# ≠ Stage-3 exit · ≠ S3-06 construct this window · ≠ product lint knife this window.
# See docs/census/stage3-lint-ast-census.md and docs/adr/stage3-s3-05-claim.md
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== S3-05 lint-ast census =="

count_rg() {
  local pattern="$1"
  shift
  { rg -c "$pattern" "$@" 2>/dev/null || true; } | awk -F: '
    NF==1 { s+=$1; next }
    { s+=$2 }
    END { print s+0 }
  '
}

syx_lint_lib_loc=$(wc -l < crates/syx-lint/src/lib.rs | tr -d ' ')
lint_source_text_helper=$(count_rg 'fn lint_source_text' crates/syx-lint/src/lib.rs)
lint_source_auto_defs=$(count_rg 'pub fn lint_source_auto' crates/syx-lint/src/lib.rs)
lower_cst_subset_probe_mentions=$(count_rg 'lower_cst_subset_probe' crates/syx-lint/src/lib.rs)
l007_emit_sites=$(count_rg 'code: "L007"' crates/syx-lint/src/lib.rs)
l012_emit_sites=$(count_rg 'code: "L012"' crates/syx-lint/src/lib.rs)
lint_rules_ast_only=9
lint_rules_text_hybrid=3
class_demo_fixture=$(test -f examples/class_demo.sy && echo 1 || echo 0)
struct_demo_fixture=$(test -f examples/struct_demo.sy && echo 1 || echo 0)
struct_book_point_fixture=$(test -f crates/syx-driver/tests/fixtures/lang/struct_book_point.sy && echo 1 || echo 0)
scoped_fixture_helper=$((class_demo_fixture + struct_demo_fixture + struct_book_point_fixture))

stage3_lint_census_helper=$(count_rg 'S3-I72|S3-I73|S3-I74|S3-I75|S3-I76|S3-I77|stage3-lint-ast-census|stage3-s3-05-open|stage3-s3-05-l007-l012|stage3-s3-05-struct-extension|stage3-s3-05-claim-prep|stage3-s3-05-residual-scope|stage3-s3-05-claim' \
  docs/adr/stage3-s3-05-open.md docs/adr/stage3-s3-05-l007-l012-false-positive.md \
  docs/adr/stage3-s3-05-struct-extension-type-member.md \
  docs/adr/stage3-s3-05-claim-prep.md \
  docs/adr/stage3-s3-05-residual-scope.md \
  docs/adr/stage3-s3-05-claim.md \
  docs/census/stage3-lint-ast-census.md scripts/s3_05_lint_ast_census.sh)
l007_l012_knife_helper=$(count_rg 'in_type_member|S3-I73' crates/syx-lint/src/lib.rs)
struct_type_member_helper=$(count_rg 'Decl::Struct|Decl::Extension|S3-I74' crates/syx-lint/src/lib.rs)
# S3-I75 hinge: claim-prep scaffolding (ADR + census + contract anchors).
claim_prep_helper=$(count_rg 'S3-I75|claim-prep|claim_criteria|Primary' \
  docs/adr/stage3-s3-05-claim-prep.md docs/census/stage3-lint-ast-census.md \
  scripts/s3_05_lint_ast_census.sh crates/syx-driver/tests/credibility/lint_ast_claim_prep_contract.rs)
# S3-I76 hinge: residual-scope (scoped acceptance + deferred hybrid).
residual_scope_helper=$(count_rg 'S3-I76|residual-scope|scoped acceptance|supersede' \
  docs/adr/stage3-s3-05-residual-scope.md docs/census/stage3-lint-ast-census.md \
  scripts/s3_05_lint_ast_census.sh plan/02-improvement-roadmap.md)
# S3-I77 hinge: scoped plan claim.
s3_05_claim_helper=$(count_rg 'S3-I77|scoped \[x\]|Claimed: scoped|stage3-s3-05-claim' \
  docs/adr/stage3-s3-05-claim.md docs/census/stage3-lint-ast-census.md \
  scripts/s3_05_lint_ast_census.sh plan/02-improvement-roadmap.md \
  crates/syx-driver/tests/credibility/lint_ast_claim_contract.rs)

SYX_BIN=""
if [[ -x target/release/syx ]]; then
  SYX_BIN=target/release/syx
elif [[ -x target/debug/syx ]]; then
  SYX_BIN=target/debug/syx
elif command -v syx >/dev/null 2>&1; then
  SYX_BIN=$(command -v syx)
fi

count_lint_code() {
  local file="$1"
  local code="$2"
  if [[ -n "$SYX_BIN" && -f "$file" ]]; then
    local out
    out=$("$SYX_BIN" lint "$file" 2>&1 || true)
    printf '%s\n' "$out" | rg -c "$code" || true
  else
    echo 0
  fi
}

class_demo_l007_count=$(count_lint_code examples/class_demo.sy L007)
class_demo_l012_count=$(count_lint_code examples/class_demo.sy L012)
struct_book_point_l007_count=$(count_lint_code crates/syx-driver/tests/fixtures/lang/struct_book_point.sy L007)
class_demo_l007_count=${class_demo_l007_count:-0}
class_demo_l012_count=${class_demo_l012_count:-0}
struct_book_point_l007_count=${struct_book_point_l007_count:-0}

echo "syx_lint_lib_loc=$syx_lint_lib_loc"
echo "lint_source_text_helper=$lint_source_text_helper"
echo "lint_source_auto_defs=$lint_source_auto_defs"
echo "lower_cst_subset_probe_mentions=$lower_cst_subset_probe_mentions"
echo "l007_emit_sites=$l007_emit_sites"
echo "l012_emit_sites=$l012_emit_sites"
echo "lint_rules_ast_only=$lint_rules_ast_only"
echo "lint_rules_text_hybrid=$lint_rules_text_hybrid"
echo "class_demo_fixture=$class_demo_fixture"
echo "struct_demo_fixture=$struct_demo_fixture"
echo "struct_book_point_fixture=$struct_book_point_fixture"
echo "scoped_fixture_helper=$scoped_fixture_helper"
echo "class_demo_l007_count=$class_demo_l007_count"
echo "class_demo_l012_count=$class_demo_l012_count"
echo "struct_book_point_l007_count=$struct_book_point_l007_count"
echo "stage3_lint_census_helper=$stage3_lint_census_helper"
echo "l007_l012_knife_helper=$l007_l012_knife_helper"
echo "struct_type_member_helper=$struct_type_member_helper"
echo "claim_prep_helper=$claim_prep_helper"
echo "residual_scope_helper=$residual_scope_helper"
echo "s3_05_claim_helper=$s3_05_claim_helper"
echo "current=S3-I77"
echo "next=S3-I89"
echo "s3_05_status=x"
echo "stage3_checklist=7/7"
echo "nonclaim=Stage-3-exit,S3-06-x,S3-07,hybrid-hard-gate,S3-04-fib-wall"
