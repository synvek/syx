#!/usr/bin/env bash
# S3-I88…I91: Stage-4 / S4-01 stdlib stability + doctest census.
# I88 open · I89 SSOT · I90 residual-scope · I91 scoped claim.
# ≠ Stage-4 exit · ≠ manifest schema · ≠ syx doctest.
# See docs/stage4-stdlib-stability-doctest-census.md
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== S4-01 stdlib stability/doctest census =="

count_rg() {
  local pattern="$1"
  shift
  { rg -c "$pattern" "$@" 2>/dev/null || true; } | awk -F: '
    NF==1 { s+=$1; next }
    { s+=$2 }
    END { print s+0 }
  '
}

stage4_open_helper=$(count_rg 'S3-I88|S3-I89|S3-I90|S3-I91|S3-I92|stage4-open|Stage-4 \[~\]|S4-01' \
  docs/adr/stage4-open.md docs/adr/stage4-s4-01-stability-ssot.md \
  docs/adr/stage4-s4-01-residual-scope.md docs/adr/stage4-s4-01-claim.md \
  docs/stage4-stdlib-stability-doctest-census.md \
  scripts/s4_01_stdlib_stability_doctest_census.sh plan/02-improvement-roadmap.md \
  crates/syx-driver/tests/lang/s4_01_stdlib_stability_doctest_census_contract.rs \
  crates/syx-driver/tests/lang/s4_01_stability_ssot_contract.rs \
  crates/syx-driver/tests/lang/s4_01_residual_scope_contract.rs \
  crates/syx-driver/tests/lang/s4_01_claim_contract.rs)

s4_01_census_helper=$(count_rg 'S4-01|stdlib stability|doctest census|stage4-stdlib-stability|stability-ssot|stdlib-stability-v0|residual-scope|scoped acceptance|scoped \[x\]' \
  docs/adr/stage4-open.md docs/adr/stage4-s4-01-stability-ssot.md \
  docs/adr/stage4-s4-01-residual-scope.md docs/adr/stage4-s4-01-claim.md \
  docs/stage4-stdlib-stability-doctest-census.md docs/stdlib-stability-v0.md \
  docs/s4-01-manifest-doctest-scope-v0.md \
  scripts/s4_01_stdlib_stability_doctest_census.sh plan/02-improvement-roadmap.md)

stability_ssot_helper=$(count_rg 'S3-I89|stability\.toml|syx-stdlib-stability-v0|stability_ssot' \
  packages/internal/stability.toml docs/stdlib-stability-v0.md \
  docs/adr/stage4-s4-01-stability-ssot.md docs/stage4-stdlib-stability-doctest-census.md \
  scripts/s4_01_stdlib_stability_doctest_census.sh \
  crates/syx-driver/tests/lang/s4_01_stability_ssot_contract.rs \
  packages/README.md plan/02-improvement-roadmap.md)

residual_scope_helper=$(count_rg 'S3-I90|residual-scope|scoped acceptance|supersede|manifest-doctest-scope' \
  docs/adr/stage4-s4-01-residual-scope.md docs/s4-01-manifest-doctest-scope-v0.md \
  docs/stage4-stdlib-stability-doctest-census.md \
  scripts/s4_01_stdlib_stability_doctest_census.sh plan/02-improvement-roadmap.md \
  crates/syx-driver/tests/lang/s4_01_residual_scope_contract.rs)

s4_01_claim_helper=$(count_rg 'S3-I91|stage4-s4-01-claim|Claimed: scoped|scoped \[x\]|s4_01_claim' \
  docs/adr/stage4-s4-01-claim.md docs/stage4-stdlib-stability-doctest-census.md \
  scripts/s4_01_stdlib_stability_doctest_census.sh plan/02-improvement-roadmap.md \
  crates/syx-driver/tests/lang/s4_01_claim_contract.rs)

stability_ssot_package_rows=$(count_rg '^\[\[package\]\]' packages/internal/stability.toml)
packages_readme_status_rows=$(count_rg '^\| \*\*' packages/README.md)
packages_readme_plan_ternary_preview=$(count_rg '\| preview \||\| Preview \|' packages/README.md)
manifest_stability_field_count=$(count_rg '"stability"' packages/internal/manifest.json)
syx_doctest_subcommand_count=$(count_rg 'Doctest|doctest' crates/syx-cli/src/main.rs)
doctest_annotations_doc_helper=$(count_rg 'doctest|Doctest' docs/doctest-annotations.md)
capabilities_experimental_helper=$(count_rg 'experimental' packages/internal/capabilities.toml)

echo "stage4_open_helper=$stage4_open_helper"
echo "s4_01_census_helper=$s4_01_census_helper"
echo "stability_ssot_helper=$stability_ssot_helper"
echo "residual_scope_helper=$residual_scope_helper"
echo "s4_01_claim_helper=$s4_01_claim_helper"
echo "stability_ssot_package_rows=$stability_ssot_package_rows"
echo "packages_readme_status_rows=$packages_readme_status_rows"
echo "packages_readme_plan_ternary_preview=$packages_readme_plan_ternary_preview"
echo "manifest_stability_field_count=$manifest_stability_field_count"
echo "syx_doctest_subcommand_count=$syx_doctest_subcommand_count"
echo "doctest_annotations_doc_helper=$doctest_annotations_doc_helper"
echo "capabilities_experimental_helper=$capabilities_experimental_helper"
echo "current=S3-I91"
echo "next=S3-I92"
echo "stage4_status=~"
echo "s4_01_status=x"
echo "stage4_checklist=1/5"
echo "nonclaim=Stage-4-exit,manifest-schema,syx-doctest,S4-02-construct,S4-03-05,HostFn,F-08,Wave"
