#!/usr/bin/env bash
# S3-I82…I85: Stage-3 init-template census (S3-07).
# I82 open · I83 default-vm · I84 residual-scope · I85 scoped claim.
# ≠ Stage-3 exit · ≠ change Init this window.
# See docs/stage3-init-template-census.md and docs/adr/stage3-s3-07-claim.md
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== S3-07 init-template census =="

count_rg() {
  local pattern="$1"
  shift
  { rg -c "$pattern" "$@" 2>/dev/null || true; } | awk -F: '
    NF==1 { s+=$1; next }
    { s+=$2 }
    END { print s+0 }
  '
}

stage3_init_census_helper=$(count_rg 'S3-I82|S3-I83|S3-I84|S3-I85|S3-I86|S3-I87|S3-I88|stage3-exit|stage3-exit-open|stage3-init-template-census|stage3-s3-07-open|stage3-s3-07-default-vm|stage3-s3-07-residual-scope|stage3-s3-07-claim' \
  docs/adr/stage3-s3-07-open.md docs/adr/stage3-s3-07-default-vm.md \
  docs/adr/stage3-s3-07-residual-scope.md docs/adr/stage3-s3-07-claim.md \
  docs/stage3-init-template-census.md scripts/s3_07_init_template_census.sh \
  plan/02-improvement-roadmap.md \
  crates/syx-driver/tests/lang/s3_07_init_template_census_contract.rs \
  crates/syx-driver/tests/lang/s3_07_default_vm_contract.rs \
  crates/syx-driver/tests/lang/s3_07_residual_scope_contract.rs \
  crates/syx-driver/tests/lang/s3_07_claim_contract.rs)
residual_scope_helper=$(count_rg 'S3-I84|residual-scope|scoped acceptance|supersede' \
  docs/adr/stage3-s3-07-residual-scope.md docs/stage3-init-template-census.md \
  scripts/s3_07_init_template_census.sh plan/02-improvement-roadmap.md)
# S3-I85 hinge: scoped plan claim.
s3_07_claim_helper=$(count_rg 'S3-I85|scoped \[x\]|Claimed: scoped|stage3-s3-07-claim' \
  docs/adr/stage3-s3-07-claim.md docs/stage3-init-template-census.md \
  scripts/s3_07_init_template_census.sh plan/02-improvement-roadmap.md \
  crates/syx-driver/tests/lang/s3_07_claim_contract.rs)
init_commands_helper=$(count_rg 'Commands::Init|Init \{' \
  crates/syx-cli/src/main.rs)
init_default_template_native=$(count_rg 'default_value = "native"' \
  crates/syx-cli/src/main.rs)
init_default_template_vm=$(count_rg 'default_value = "vm"' \
  crates/syx-cli/src/main.rs)
init_vm_named_template_count=$(count_rg '"vm"' \
  crates/syx-cli/src/main.rs)
init_inline_match_arm_helper=$(count_rg '"wasm"|"plugin"|"lib"|print\("native"\)|print\("vm"\)' \
  crates/syx-cli/src/main.rs)
init_integration_test_count=$(count_rg 'run_project_with_options|S3-I83|print\(\"vm\"\)' \
  crates/syx-driver/tests/lang/s3_07_default_vm_contract.rs)

echo "stage3_init_census_helper=$stage3_init_census_helper"
echo "residual_scope_helper=$residual_scope_helper"
echo "s3_07_claim_helper=$s3_07_claim_helper"
echo "init_commands_helper=$init_commands_helper"
echo "init_default_template_native=$init_default_template_native"
echo "init_default_template_vm=$init_default_template_vm"
echo "init_vm_named_template_count=$init_vm_named_template_count"
echo "init_inline_match_arm_helper=$init_inline_match_arm_helper"
echo "init_integration_test_count=$init_integration_test_count"
echo "current=S3-I85"
echo "next=S3-I89"
echo "s3_07_status=x"
echo "stage3_checklist=7/7"
echo "nonclaim=Stage-3-exit,stage3-exit-adr,UsageGuide-rewrite,Init-crate-extract,Init-product-knife,HostFn,Wave,S3-06-lsp-knife"
