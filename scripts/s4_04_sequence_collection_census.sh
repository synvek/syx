#!/usr/bin/env bash
# S3-I100…I103: Stage-4 / S4-04 Sequence/Collection census.
# I100 open · I101 hinge · I102 residual · I103 scoped claim.
# ≠ Stage-4 exit · ≠ semantics/stdlib/codegen knife · ≠ makeIterator GA · ≠ S4-05.
# See docs/census/stage4-sequence-collection-census.md
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== S4-04 Sequence/Collection census =="

count_rg() {
  local pattern="$1"
  shift
  { rg -c "$pattern" "$@" 2>/dev/null || true; } | awk -F: '
    NF==1 { s+=$1; next }
    { s+=$2 }
    END { print s+0 }
  '
}

s4_04_open_helper=$(count_rg 'S3-I100|S3-I101|S3-I102|S3-I103|S3-I104|stage4-s4-04-open|S4-04 \[~\]|S4-04 \[x\]|Sequence/Collection|sequence-collection' \
  docs/adr/stage4-s4-04-open.md docs/adr/stage4-s4-04-iterator-forin-hinge.md \
  docs/adr/stage4-s4-04-residual-scope.md docs/adr/stage4-s4-04-claim.md \
  docs/census/stage4-sequence-collection-census.md \
  scripts/s4_04_sequence_collection_census.sh plan/02-improvement-roadmap.md \
  crates/syx-driver/tests/credibility/sequence_collection_census_contract.rs \
  crates/syx-driver/tests/credibility/iterator_forin_hinge_contract.rs \
  crates/syx-driver/tests/credibility/sequence_collection_residual_scope_contract.rs \
  crates/syx-driver/tests/credibility/sequence_collection_claim_contract.rs)

s4_04_census_helper=$(count_rg 'S4-04|Sequence|Collection|Iterator|makeIterator|map/filter|stage4-sequence-collection|iterator-forin|residual-scope|scoped \[x\]' \
  docs/adr/stage4-s4-04-open.md docs/adr/stage4-s4-04-iterator-forin-hinge.md \
  docs/adr/stage4-s4-04-residual-scope.md docs/adr/stage4-s4-04-claim.md \
  docs/census/s4-04-iterator-forin-v0.md docs/census/s4-04-sequence-residual-scope-v0.md \
  docs/census/stage4-sequence-collection-census.md \
  scripts/s4_04_sequence_collection_census.sh plan/02-improvement-roadmap.md)

custom_iterator_forin_hinge_helper=$(count_rg 'S3-I101|iterator-forin-hinge|s4-04-iterator-forin-v0|custom_iterator_forin_hinge' \
  docs/adr/stage4-s4-04-iterator-forin-hinge.md docs/census/s4-04-iterator-forin-v0.md \
  docs/census/stage4-sequence-collection-census.md \
  scripts/s4_04_sequence_collection_census.sh plan/02-improvement-roadmap.md \
  crates/syx-driver/tests/credibility/iterator_forin_hinge_contract.rs)

residual_scope_helper=$(count_rg 'S3-I102|residual-scope|scoped acceptance|supersede|sequence-residual-scope' \
  docs/adr/stage4-s4-04-residual-scope.md docs/census/s4-04-sequence-residual-scope-v0.md \
  docs/census/stage4-sequence-collection-census.md \
  scripts/s4_04_sequence_collection_census.sh plan/02-improvement-roadmap.md \
  crates/syx-driver/tests/credibility/sequence_collection_residual_scope_contract.rs)

s4_04_claim_helper=$(count_rg 'S3-I103|stage4-s4-04-claim|Claimed: scoped|scoped \[x\]|s4_04_claim' \
  docs/adr/stage4-s4-04-claim.md docs/census/stage4-sequence-collection-census.md \
  scripts/s4_04_sequence_collection_census.sh plan/02-improvement-roadmap.md \
  crates/syx-driver/tests/credibility/sequence_collection_claim_contract.rs)

custom_iterator_parity_fixture_helper=$(count_rg 'Counter: Iterator|protocol_iterator_custom|for x in Counter' \
  crates/syx-driver/tests/fixtures/parity/protocol_iterator_custom.sy)

iterator_forin_test_helper=$(count_rg 'for.?in|Iterator|makeIterator|Counter|protocol_iterator|collection_sequence' \
  crates/syx-driver/tests/stdlib/collection_sequence_iterator.rs \
  crates/syx-driver/tests/lang/protocol_iterator.rs)

collections_protocol_helper=$(count_rg 'collections\.protocol|Sequence\.makeIterator|Collection Index' \
  packages/internal/capabilities.toml)

make_iterator_surface_helper=$(count_rg 'MakeIterator|makeIterator|still open' \
  packages/internal/mir_op_capabilities.toml \
  packages/internal/capabilities.toml \
  docs/census/stage4-sequence-collection-census.md \
  docs/adr/stage4-s4-04-open.md \
  docs/adr/stage4-s4-04-iterator-forin-hinge.md \
  docs/adr/stage4-s4-04-residual-scope.md \
  docs/adr/stage4-s4-04-claim.md)

collection_host_map_filter_helper=$(count_rg 'CollectionHostOp::(Map|Filter)|__syx_array_map|__syx_array_filter' \
  crates/syx-stdlib/src/collection_hosts.rs)

echo "s4_04_open_helper=$s4_04_open_helper"
echo "s4_04_census_helper=$s4_04_census_helper"
echo "custom_iterator_forin_hinge_helper=$custom_iterator_forin_hinge_helper"
echo "residual_scope_helper=$residual_scope_helper"
echo "s4_04_claim_helper=$s4_04_claim_helper"
echo "custom_iterator_parity_fixture_helper=$custom_iterator_parity_fixture_helper"
echo "iterator_forin_test_helper=$iterator_forin_test_helper"
echo "collections_protocol_helper=$collections_protocol_helper"
echo "make_iterator_surface_helper=$make_iterator_surface_helper"
echo "collection_host_map_filter_helper=$collection_host_map_filter_helper"
echo "current=S3-I103"
echo "next=S3-I104"
echo "stage4_status=x"
echo "s4_01_status=x"
echo "s4_02_status=x"
echo "s4_03_status=x"
echo "s4_04_status=x"
echo "stage4_checklist=4/5"
echo "nonclaim=Stage-4-exit,semantics-stdlib-codegen-knife,makeIterator-GA,user-map-filter-GA,Collection-Index-GA,S4-05-construct,HostFn,F-08,Wave,M2-S4-04-HTTP"
