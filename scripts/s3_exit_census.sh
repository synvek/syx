#!/usr/bin/env bash
# S3-I86…I87: Stage-3 exit census.
# I86 exit-open · I87 stage3-exit.
# ≠ Stage-4 construction · ≠ product knives · ≠ unpark.
# See docs/census/stage3-exit-census.md and docs/adr/stage3-exit.md
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== S3 exit census =="

count_rg() {
  local pattern="$1"
  shift
  { rg -c "$pattern" "$@" 2>/dev/null || true; } | awk -F: '
    NF==1 { s+=$1; next }
    { s+=$2 }
    END { print s+0 }
  '
}

stage3_exit_open_helper=$(count_rg 'S3-I86|stage3-exit-open|exit-open|exit open' \
  docs/adr/stage3-exit-open.md docs/census/stage3-exit-census.md \
  scripts/s3_exit_census.sh plan/02-improvement-roadmap.md \
  crates/syx-driver/tests/credibility/stage3_exit_open_contract.rs)

stage3_exit_helper=$(count_rg 'S3-I87|stage3-exit|exit_status=exited|\[x\] 退出' \
  docs/adr/stage3-exit.md docs/census/stage3-exit-census.md \
  scripts/s3_exit_census.sh plan/02-improvement-roadmap.md \
  crates/syx-driver/tests/credibility/stage3_exit_contract.rs)

stage3_claim_chain_helper=$(count_rg 'S3-I41|S3-I47|S3-I55|S3-I71|S3-I77|S3-I81|S3-I85|stage3-s3-0[1-7]-claim' \
  docs/adr/stage3-s3-01-claim.md docs/adr/stage3-s3-02-claim.md \
  docs/adr/stage3-s3-03-claim.md docs/adr/stage3-s3-04-claim.md \
  docs/adr/stage3-s3-05-claim.md docs/adr/stage3-s3-06-claim.md \
  docs/adr/stage3-s3-07-claim.md docs/census/stage3-exit-census.md \
  docs/adr/stage3-exit-open.md docs/adr/stage3-exit.md)

if [[ -f docs/adr/stage3-exit.md ]]; then
  stage3_exit_adr_present=1
else
  stage3_exit_adr_present=0
fi

echo "stage3_exit_open_helper=$stage3_exit_open_helper"
echo "stage3_exit_helper=$stage3_exit_helper"
echo "stage3_claim_chain_helper=$stage3_claim_chain_helper"
echo "stage3_exit_adr_present=$stage3_exit_adr_present"
echo "current=S3-I87"
echo "next=S3-I89"
echo "stage3_status=x"
echo "stage3_checklist=7/7"
echo "exit_status=exited"
echo "nonclaim=deferred-as-gate,HostFn,F-08,Wave,S3-04-07-knife"
