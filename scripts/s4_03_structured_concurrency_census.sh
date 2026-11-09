#!/usr/bin/env bash
# S3-I96…I99: Stage-4 / S4-03 structured concurrency census.
# I96 open · I97 cancel/TaskGroup hinge · I98 residual-scope · I99 scoped claim.
# ≠ Stage-4 exit · ≠ syx-async product knife · ≠ withTaskGroup GA.
# See docs/stage4-structured-concurrency-census.md
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== S4-03 structured concurrency census =="

count_rg() {
  local pattern="$1"
  shift
  { rg -c "$pattern" "$@" 2>/dev/null || true; } | awk -F: '
    NF==1 { s+=$1; next }
    { s+=$2 }
    END { print s+0 }
  '
}

s4_03_open_helper=$(count_rg 'S3-I96|S3-I97|S3-I98|S3-I99|S3-I100|stage4-s4-03-open|S4-03 \[~\]|S4-03 \[x\]|structured concurrency|cancel/TaskGroup' \
  docs/adr/stage4-s4-03-open.md docs/adr/stage4-s4-03-cancel-taskgroup-hinge.md \
  docs/adr/stage4-s4-03-residual-scope.md docs/adr/stage4-s4-03-claim.md \
  docs/stage4-structured-concurrency-census.md \
  scripts/s4_03_structured_concurrency_census.sh plan/02-improvement-roadmap.md \
  crates/syx-driver/tests/lang/s4_03_structured_concurrency_census_contract.rs \
  crates/syx-driver/tests/lang/s4_03_cancel_taskgroup_hinge_contract.rs \
  crates/syx-driver/tests/lang/s4_03_residual_scope_contract.rs \
  crates/syx-driver/tests/lang/s4_03_claim_contract.rs)

s4_03_census_helper=$(count_rg 'S4-03|structured concurrency|CancellationToken|TaskGroup|Sendable|task_tree|stage4-structured-concurrency|cancel-taskgroup|residual-scope|scoped acceptance|scoped \[x\]' \
  docs/adr/stage4-s4-03-open.md docs/adr/stage4-s4-03-cancel-taskgroup-hinge.md \
  docs/adr/stage4-s4-03-residual-scope.md docs/adr/stage4-s4-03-claim.md \
  docs/stage4-structured-concurrency-census.md docs/s4-03-cancel-taskgroup-v0.md \
  docs/s4-03-concurrency-residual-scope-v0.md \
  scripts/s4_03_structured_concurrency_census.sh plan/02-improvement-roadmap.md)

cancel_taskgroup_hinge_helper=$(count_rg 'S3-I97|cancel-taskgroup-hinge|s4-03-cancel-taskgroup-v0|cancel_taskgroup_hinge' \
  docs/adr/stage4-s4-03-cancel-taskgroup-hinge.md docs/s4-03-cancel-taskgroup-v0.md \
  docs/stage4-structured-concurrency-census.md \
  scripts/s4_03_structured_concurrency_census.sh plan/02-improvement-roadmap.md \
  crates/syx-driver/tests/lang/s4_03_cancel_taskgroup_hinge_contract.rs)

residual_scope_helper=$(count_rg 'S3-I98|residual-scope|scoped acceptance|supersede|concurrency-residual-scope' \
  docs/adr/stage4-s4-03-residual-scope.md docs/s4-03-concurrency-residual-scope-v0.md \
  docs/stage4-structured-concurrency-census.md \
  scripts/s4_03_structured_concurrency_census.sh plan/02-improvement-roadmap.md \
  crates/syx-driver/tests/lang/s4_03_residual_scope_contract.rs)

s4_03_claim_helper=$(count_rg 'S3-I99|stage4-s4-03-claim|Claimed: scoped|scoped \[x\]|s4_03_claim' \
  docs/adr/stage4-s4-03-claim.md docs/stage4-structured-concurrency-census.md \
  scripts/s4_03_structured_concurrency_census.sh plan/02-improvement-roadmap.md \
  crates/syx-driver/tests/lang/s4_03_claim_contract.rs)

cancel_token_helper=$(count_rg 'CancellationToken|struct CancellationToken' \
  crates/syx-async/src/cancellation.rs)

task_group_hostfn_helper=$(count_rg 'name = "TaskGroup(Create|Add|Wait|Cancel)"' \
  packages/internal/host_fn_capabilities.toml)

sendable_surface_helper=$(count_rg 'Sendable|C003|unchecked_sendable' \
  crates/syx-semantic/src/semantic_concurrency.rs \
  crates/syx-driver/tests/concurrency/concurrency_surface.rs)

task_tree_cancel_test_count=$(count_rg 'fn m3_a01_task_tree_cancel|fn m3_c04_chaos_task_group_cancel' \
  crates/syx-driver/tests/concurrency/task_tree.rs \
  crates/syx-driver/tests/concurrency/chaos_sampling.rs)

echo "s4_03_open_helper=$s4_03_open_helper"
echo "s4_03_census_helper=$s4_03_census_helper"
echo "cancel_taskgroup_hinge_helper=$cancel_taskgroup_hinge_helper"
echo "residual_scope_helper=$residual_scope_helper"
echo "s4_03_claim_helper=$s4_03_claim_helper"
echo "cancel_token_helper=$cancel_token_helper"
echo "task_group_hostfn_helper=$task_group_hostfn_helper"
echo "sendable_surface_helper=$sendable_surface_helper"
echo "task_tree_cancel_test_count=$task_tree_cancel_test_count"
echo "current=S3-I99"
echo "next=S3-I100"
echo "stage4_status=x"
echo "s4_01_status=x"
echo "s4_02_status=x"
echo "s4_03_status=x"
echo "stage4_checklist=4/5"
echo "nonclaim=Stage-4-exit,syx-async-knife,withTaskGroup-GA,TSAN-loom-GA,S4-04-05-construct,HostFn,F-08,Wave,GC-Nursery"
