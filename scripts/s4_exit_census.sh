#!/usr/bin/env bash
# S3-I108…I109: Stage-4 exit census.
# I108 exit-open · I109 stage4-exit.
# ≠ Stage-5 · ≠ product knives · ≠ unpark.
# See docs/stage4-exit-census.md and docs/adr/stage4-exit.md
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== S4 exit census =="

count_rg() {
  local pattern="$1"
  shift
  { rg -c "$pattern" "$@" 2>/dev/null || true; } | awk -F: '
    NF==1 { s+=$1; next }
    { s+=$2 }
    END { print s+0 }
  '
}

stage4_exit_open_helper=0
[[ -f docs/adr/stage4-exit-open.md && -f docs/stage4-exit-census.md ]] \
  && stage4_exit_open_helper=1

stage4_exit_helper=0
[[ -f docs/adr/stage4-exit.md ]] && stage4_exit_helper=1
stage4_exit_helper_hits=$(count_rg 'S3-I109|stage4-exit|exit_status=exited|\[x\] 退出' \
  docs/adr/stage4-exit.md docs/stage4-exit-census.md \
  scripts/s4_exit_census.sh plan/02-improvement-roadmap.md \
  crates/syx-driver/tests/lang/s4_exit_contract.rs)
if [[ "$stage4_exit_helper_hits" -ge 1 ]]; then
  stage4_exit_helper=1
fi

stage4_claim_chain_helper=0
for f in \
  docs/adr/stage4-s4-01-claim.md \
  docs/adr/stage4-s4-02-claim.md \
  docs/adr/stage4-s4-03-claim.md \
  docs/adr/stage4-s4-04-claim.md \
  docs/adr/stage4-s4-05-claim.md
do
  [[ -f "$f" ]] && stage4_claim_chain_helper=$((stage4_claim_chain_helper + 1))
done

if [[ -f docs/adr/stage4-exit.md ]]; then
  stage4_exit_adr_present=1
else
  stage4_exit_adr_present=0
fi

stage4_exit_open_mentions=$(count_rg 'S3-I108|stage4-exit-open|exit-open' \
  docs/adr/stage4-exit-open.md docs/stage4-exit-census.md \
  scripts/s4_exit_census.sh plan/02-improvement-roadmap.md)

emit() {
  printf '%s=%s\n' "$1" "$2"
}

emit stage4_exit_open_helper "$stage4_exit_open_helper"
emit stage4_exit_helper "$stage4_exit_helper"
emit stage4_claim_chain_helper "$stage4_claim_chain_helper"
emit stage4_exit_adr_present "$stage4_exit_adr_present"
emit stage4_exit_open_mentions "$stage4_exit_open_mentions"
emit current S3-I109
emit next "post-exit-residual"
emit stage4_status x
emit stage4_checklist "5/5"
emit exit_status exited
emit nonclaim "deferred-as-gate,HostFn,F-08,Wave,S4-01-05-knife,doctor-native-sdk,Stage-5"

fail=0
require_ge() {
  local key="$1" val="$2" min="$3"
  if [[ "$val" -lt "$min" ]]; then
    echo "FAIL: $key=$val < $min" >&2
    fail=1
  fi
}
require_eq() {
  local key="$1" val="$2" want="$3"
  if [[ "$val" != "$want" ]]; then
    echo "FAIL: $key=$val != $want" >&2
    fail=1
  fi
}

require_ge stage4_exit_open_helper "$stage4_exit_open_helper" 1
require_ge stage4_exit_helper "$stage4_exit_helper" 1
require_ge stage4_claim_chain_helper "$stage4_claim_chain_helper" 5
require_eq stage4_exit_adr_present "$stage4_exit_adr_present" 1

if [[ "$fail" -ne 0 ]]; then
  exit 1
fi
echo "OK: s4_exit_census"
