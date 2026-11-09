#!/usr/bin/env bash
# S3-I92…I95: Stage-4 / S4-02 package registry census.
# I92 open · I93 file-registry hinge · I94 residual-scope · I95 scoped claim.
# ≠ Stage-4 exit · ≠ syx-pkg/CLI product knife · ≠ public registry GA.
# See docs/stage4-package-registry-census.md
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== S4-02 package registry census =="

count_rg() {
  local pattern="$1"
  shift
  { rg -c "$pattern" "$@" 2>/dev/null || true; } | awk -F: '
    NF==1 { s+=$1; next }
    { s+=$2 }
    END { print s+0 }
  '
}

s4_02_open_helper=$(count_rg 'S3-I92|S3-I93|S3-I94|S3-I95|S3-I96|stage4-s4-02-open|S4-02 \[~\]|S4-02 \[x\]|package.registry' \
  docs/adr/stage4-s4-02-open.md docs/adr/stage4-s4-02-file-registry-hinge.md \
  docs/adr/stage4-s4-02-residual-scope.md docs/adr/stage4-s4-02-claim.md \
  docs/stage4-package-registry-census.md \
  scripts/s4_02_package_registry_census.sh plan/02-improvement-roadmap.md \
  crates/syx-driver/tests/lang/s4_02_package_registry_census_contract.rs \
  crates/syx-driver/tests/lang/s4_02_file_registry_hinge_contract.rs \
  crates/syx-driver/tests/lang/s4_02_residual_scope_contract.rs \
  crates/syx-driver/tests/lang/s4_02_claim_contract.rs)

s4_02_census_helper=$(count_rg 'S4-02|package registry|add/update/publish|file://|lock-semver|stage4-package-registry|file-registry|residual-scope|scoped acceptance|scoped \[x\]' \
  docs/adr/stage4-s4-02-open.md docs/adr/stage4-s4-02-file-registry-hinge.md \
  docs/adr/stage4-s4-02-residual-scope.md docs/adr/stage4-s4-02-claim.md \
  docs/stage4-package-registry-census.md docs/s4-02-file-registry-v0.md \
  docs/s4-02-registry-residual-scope-v0.md \
  scripts/s4_02_package_registry_census.sh plan/02-improvement-roadmap.md)

file_registry_hinge_helper=$(count_rg 'S3-I93|file-registry-hinge|s4-02-file-registry-v0|file_registry_hinge' \
  docs/adr/stage4-s4-02-file-registry-hinge.md docs/s4-02-file-registry-v0.md \
  docs/stage4-package-registry-census.md \
  scripts/s4_02_package_registry_census.sh plan/02-improvement-roadmap.md \
  crates/syx-driver/tests/lang/s4_02_file_registry_hinge_contract.rs)

residual_scope_helper=$(count_rg 'S3-I94|residual-scope|scoped acceptance|supersede|registry-residual-scope' \
  docs/adr/stage4-s4-02-residual-scope.md docs/s4-02-registry-residual-scope-v0.md \
  docs/stage4-package-registry-census.md \
  scripts/s4_02_package_registry_census.sh plan/02-improvement-roadmap.md \
  crates/syx-driver/tests/lang/s4_02_residual_scope_contract.rs)

s4_02_claim_helper=$(count_rg 'S3-I95|stage4-s4-02-claim|Claimed: scoped|scoped \[x\]|s4_02_claim' \
  docs/adr/stage4-s4-02-claim.md docs/stage4-package-registry-census.md \
  scripts/s4_02_package_registry_census.sh plan/02-improvement-roadmap.md \
  crates/syx-driver/tests/lang/s4_02_claim_contract.rs)

file_registry_integration_test_count=$(count_rg 'fn registry_install_from_file_registry' \
  crates/syx-pkg/tests/integration.rs)
file_registry_evergreen_roundtrip_count=$(count_rg 'fn publish_file_registry|fn publish_install_roundtrip' \
  crates/syx-driver/tests/evergreen/registry.rs)
registry_semver_r001_count=$(count_rg 'fn r001_diamond_registry_' \
  crates/syx-pkg/tests/integration.rs)

syx_add_helper=$(count_rg 'Commands::Add|/// Add a dependency' crates/syx-cli/src/main.rs)
syx_update_helper=$(count_rg 'Commands::Update|/// Update project dependencies' crates/syx-cli/src/main.rs)
syx_publish_helper=$(count_rg 'Commands::Publish|/// Publish a package' crates/syx-cli/src/main.rs)
syx_lock_helper=$(count_rg 'SyxLockfile|syx\.lock|struct SyxLock' crates/syx-pkg/src/lockfile.rs)
registry_source_helper=$(count_rg 'file://|RegistryClient|registry' \
  crates/syx-pkg/src/registry.rs crates/syx-pkg/src/sources/registry.rs)
semver_dep_helper=$(count_rg 'VersionReq|semver::' crates/syx-pkg/src/manifest.rs crates/syx-pkg/src/resolver.rs)

echo "s4_02_open_helper=$s4_02_open_helper"
echo "s4_02_census_helper=$s4_02_census_helper"
echo "file_registry_hinge_helper=$file_registry_hinge_helper"
echo "residual_scope_helper=$residual_scope_helper"
echo "s4_02_claim_helper=$s4_02_claim_helper"
echo "file_registry_integration_test_count=$file_registry_integration_test_count"
echo "file_registry_evergreen_roundtrip_count=$file_registry_evergreen_roundtrip_count"
echo "registry_semver_r001_count=$registry_semver_r001_count"
echo "syx_add_helper=$syx_add_helper"
echo "syx_update_helper=$syx_update_helper"
echo "syx_publish_helper=$syx_publish_helper"
echo "syx_lock_helper=$syx_lock_helper"
echo "registry_source_helper=$registry_source_helper"
echo "semver_dep_helper=$semver_dep_helper"
echo "current=S3-I95"
echo "next=S3-I96"
echo "stage4_status=x"
echo "s4_01_status=x"
echo "s4_02_status=x"
echo "stage4_checklist=4/5"
echo "nonclaim=Stage-4-exit,syx-pkg-CLI-knife,public-registry-GA,lock-v16-trim,S4-03-05-construct,HostFn,F-08,Wave"
