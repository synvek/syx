#!/usr/bin/env bash
# S2-07 version census (historical).
# C-6a: reads Cargo workspace version as SSOT (superseded for ongoing gates by
# scripts/c6a_version_ssot_census.sh). Still useful as a thin inject/no-117 probe.
# See docs/census/stage2-version-census.md and docs/adr/stage2-s2-07-inject-cargo-pkg-version.md
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== S2-07 version census (SSOT=Cargo; see also c6a_*) =="

workspace_semver="$(python3 - <<'PY'
from pathlib import Path
import re
cargo = Path("Cargo.toml").read_text(encoding="utf-8")
m = re.search(r'\[workspace\.package\][\s\S]*?^version\s*=\s*"([^"]+)"', cargo, re.M)
print(m.group(1) if m else "")
PY
)"
if [[ -z "$workspace_semver" ]]; then
  echo "FAIL: missing workspace.package version" >&2
  exit 1
fi

manifest_has_117=0
if grep -F '"version": "117.0"' packages/internal/manifest.json >/dev/null 2>&1; then
  manifest_has_117=1
fi

manifest_has_semver=0
if grep -E "\"version\"[[:space:]]*:[[:space:]]*\"${workspace_semver}\"" packages/internal/manifest.json >/dev/null 2>&1; then
  manifest_has_semver=1
fi

capabilities_has_117=0
if grep -F 'syx_version = "117.0"' packages/internal/capabilities.toml >/dev/null 2>&1; then
  capabilities_has_117=1
fi

capabilities_has_semver=0
if grep -F "syx_version = \"${workspace_semver}\"" packages/internal/capabilities.toml >/dev/null 2>&1; then
  capabilities_has_semver=1
fi

build_rs_injects_cargo_pkg=0
if grep -F 'CARGO_PKG_VERSION' crates/syx-driver/build.rs >/dev/null 2>&1; then
  build_rs_injects_cargo_pkg=1
fi

build_rs_has_117=0
if grep -F '"version": "117.0"' crates/syx-driver/build.rs >/dev/null 2>&1; then
  build_rs_has_117=1
fi

evergreen_nails_117=0
if grep -F '"117.0"' crates/syx-driver/tests/evergreen/smoke.rs >/dev/null 2>&1 \
  && grep -E 'assert_eq!\(version, "117\.0"' crates/syx-driver/tests/evergreen/stdlib_parity.rs >/dev/null 2>&1; then
  evergreen_nails_117=1
fi

evergreen_uses_cargo_pkg=0
if grep -F 'CARGO_PKG_VERSION' crates/syx-driver/tests/evergreen/smoke.rs >/dev/null 2>&1 \
  && grep -F 'CARGO_PKG_VERSION' crates/syx-driver/tests/evergreen/stdlib_parity.rs >/dev/null 2>&1; then
  evergreen_uses_cargo_pkg=1
fi

manifest_phase_139=0
if grep -F '"phase": 139' packages/internal/manifest.json >/dev/null 2>&1 \
  || grep -E 'phase[[:space:]]*=[[:space:]]*139' packages/internal/capabilities.toml >/dev/null 2>&1; then
  manifest_phase_139=1
fi

echo "workspace_semver=${workspace_semver}"
echo "manifest_has_117=${manifest_has_117}"
echo "manifest_has_semver=${manifest_has_semver}"
echo "capabilities_has_117=${capabilities_has_117}"
echo "capabilities_has_semver=${capabilities_has_semver}"
echo "build_rs_injects_cargo_pkg=${build_rs_injects_cargo_pkg}"
echo "build_rs_has_117=${build_rs_has_117}"
echo "evergreen_nails_117=${evergreen_nails_117}"
echo "evergreen_uses_cargo_pkg=${evergreen_uses_cargo_pkg}"
echo "manifest_phase_139=${manifest_phase_139}"

if [[ "$manifest_has_117" -ne 0 || "$capabilities_has_117" -ne 0 || "$build_rs_has_117" -ne 0 ]]; then
  echo "FAIL: 117.0 marketing version still present" >&2
  exit 1
fi
if [[ "$manifest_has_semver" -ne 1 || "$capabilities_has_semver" -ne 1 ]]; then
  echo "FAIL: manifest/capabilities must match workspace ${workspace_semver}" >&2
  exit 1
fi
if [[ "$build_rs_injects_cargo_pkg" -ne 1 || "$evergreen_uses_cargo_pkg" -ne 1 ]]; then
  echo "FAIL: CARGO_PKG_VERSION inject/nails missing" >&2
  exit 1
fi

echo "== census done (public semver=${workspace_semver}; S2-07 scoped; ≠ Stage-2 exit; next gate=c6a) =="
