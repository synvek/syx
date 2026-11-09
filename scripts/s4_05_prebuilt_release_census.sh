#!/usr/bin/env bash
# S3-I104…I107 — S4-05 prebuilt / full-native release census KPIs (open→claim).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

count_rg() {
  local pat="$1"
  shift
  rg -l --glob '!target/**' --glob '!.git/**' "$pat" "$@" 2>/dev/null | wc -l | tr -d ' '
}

s4_05_open_helper=0
[[ -f docs/adr/stage4-s4-05-open.md ]] && s4_05_open_helper=1

s4_05_census_helper=0
[[ -f docs/stage4-prebuilt-release-census.md && -f scripts/s4_05_prebuilt_release_census.sh ]] && s4_05_census_helper=1

doctor_native_sdk_hinge_helper=0
[[ -f docs/adr/stage4-s4-05-doctor-native-sdk-hinge.md && -f docs/s4-05-doctor-native-sdk-v0.md ]] \
  && doctor_native_sdk_hinge_helper=1

residual_scope_helper=0
[[ -f docs/adr/stage4-s4-05-residual-scope.md && -f docs/s4-05-prebuilt-residual-scope-v0.md ]] \
  && residual_scope_helper=1

s4_05_claim_helper=0
[[ -f docs/adr/stage4-s4-05-claim.md ]] && s4_05_claim_helper=1

full_native_build_helper=0
[[ -f scripts/build-full-native ]] && full_native_build_helper=1

syx_sdk_packaging_helper=0
[[ -d packaging/syx-sdk ]] && syx_sdk_packaging_helper="$(find packaging/syx-sdk -type f | wc -l | tr -d ' ')"

native_sdk_layout_helper="$(count_rg 'fn native_sdk_layout_report' crates/syx-rt)"

layout_report_unit_helper="$(count_rg 'fn native_sdk_layout_report_runs' crates/syx-rt)"

doctor_cli_helper="$(count_rg 'Commands::Doctor' crates/syx-cli/src/main.rs)"

# Honesty: docs/release claim `syx doctor --native-sdk` (CLI may still be unwired).
doctor_native_sdk_flag_helper="$(
  rg -l --glob '!target/**' --glob '!.git/**' -- 'doctor --native-sdk' \
    docs packaging .github 2>/dev/null | wc -l | tr -d ' '
)"

emit() {
  printf '%s=%s\n' "$1" "$2"
}

emit s4_05_open_helper "$s4_05_open_helper"
emit s4_05_census_helper "$s4_05_census_helper"
emit doctor_native_sdk_hinge_helper "$doctor_native_sdk_hinge_helper"
emit residual_scope_helper "$residual_scope_helper"
emit s4_05_claim_helper "$s4_05_claim_helper"
emit full_native_build_helper "$full_native_build_helper"
emit syx_sdk_packaging_helper "$syx_sdk_packaging_helper"
emit native_sdk_layout_helper "$native_sdk_layout_helper"
emit layout_report_unit_helper "$layout_report_unit_helper"
emit doctor_cli_helper "$doctor_cli_helper"
emit doctor_native_sdk_flag_helper "$doctor_native_sdk_flag_helper"
emit current S3-I107
emit next S3-I108
emit s4_05_status x
emit stage4_status "x"
emit stage4_checklist "5/5"

fail=0
require_ge() {
  local key="$1" val="$2" min="$3"
  if [[ "$val" -lt "$min" ]]; then
    echo "FAIL: $key=$val < $min" >&2
    fail=1
  fi
}

require_ge s4_05_open_helper "$s4_05_open_helper" 1
require_ge s4_05_census_helper "$s4_05_census_helper" 1
require_ge doctor_native_sdk_hinge_helper "$doctor_native_sdk_hinge_helper" 1
require_ge residual_scope_helper "$residual_scope_helper" 1
require_ge s4_05_claim_helper "$s4_05_claim_helper" 1
require_ge full_native_build_helper "$full_native_build_helper" 1
require_ge syx_sdk_packaging_helper "$syx_sdk_packaging_helper" 1
require_ge native_sdk_layout_helper "$native_sdk_layout_helper" 1
require_ge layout_report_unit_helper "$layout_report_unit_helper" 1
require_ge doctor_cli_helper "$doctor_cli_helper" 1
require_ge doctor_native_sdk_flag_helper "$doctor_native_sdk_flag_helper" 1

if [[ "$fail" -ne 0 ]]; then
  exit 1
fi
echo "OK: s4_05_prebuilt_release_census"
