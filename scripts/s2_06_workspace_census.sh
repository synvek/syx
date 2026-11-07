#!/usr/bin/env bash
# S2-I260/I261: Stage-2 workspace slim census (S2-06).
# Does not claim S2-06 [x].
# See docs/stage2-workspace-census.md and docs/adr/stage2-s2-06-exclude-syx-gui.md
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== S2-06 workspace census =="

member_count=$(find crates -mindepth 1 -maxdepth 1 -type d | wc -l | tr -d ' ')
has_syx_gui=0
if [[ -d crates/syx-gui ]]; then
  has_syx_gui=1
fi

members_glob=0
if grep -F 'members = ["crates/*"]' Cargo.toml >/dev/null 2>&1; then
  members_glob=1
fi

has_default_members=0
if grep -E '^[[:space:]]*default-members[[:space:]]*=' Cargo.toml >/dev/null 2>&1; then
  has_default_members=1
fi

syx_gui_workspace_excluded=0
if grep -F 'exclude = ["crates/syx-gui"]' Cargo.toml >/dev/null 2>&1 \
  || grep -E 'exclude[[:space:]]*=[[:space:]]*\[[^]]*crates/syx-gui' Cargo.toml >/dev/null 2>&1; then
  syx_gui_workspace_excluded=1
fi

ci_excludes_gui=0
if grep -F 'exclude syx-gui' .github/workflows/ci.yml >/dev/null 2>&1; then
  ci_excludes_gui=1
fi

echo "workspace_member_count=${member_count}"
echo "has_syx_gui=${has_syx_gui}"
echo "members_glob_crates_star=${members_glob}"
echo "has_default_members=${has_default_members}"
echo "syx_gui_workspace_excluded=${syx_gui_workspace_excluded}"
echo "ci_excludes_syx_gui=${ci_excludes_gui}"
echo "== census done (workspace slim; S2-06 claimed scoped; ≠ Stage-2 exit) =="
