#!/usr/bin/env bash
# S2-I165: Stage-2 file-split census (S2-05 open baseline).
# Does not split modules or claim S2-05 [x].
# See docs/census/stage2-file-split-census.md and docs/adr/stage2-s2-05-open.md
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== S2-05 file-split census =="

count_non_lib() {
  local dir="$1"
  find "$dir" -name '*.rs' ! -name 'lib.rs' ! -path '*/bin/*' 2>/dev/null \
    | wc -l | tr -d ' '
}

mir_lib=$(wc -l < crates/syx-mir/src/lib.rs | tr -d ' ')
driver_lib=$(wc -l < crates/syx-driver/src/lib.rs | tr -d ' ')
semantic_lib=$(wc -l < crates/syx-semantic/src/lib.rs | tr -d ' ')
combined=$((mir_lib + driver_lib + semantic_lib))

mir_mods=$(count_non_lib crates/syx-mir/src)
driver_mods=$(count_non_lib crates/syx-driver/src)
semantic_mods=$(count_non_lib crates/syx-semantic/src)

echo "syx_mir_lib_loc=${mir_lib}"
echo "syx_driver_lib_loc=${driver_lib}"
echo "syx_semantic_lib_loc=${semantic_lib}"
echo "combined_lib_loc=${combined}"
echo "syx_mir_sidecar_rs_count=${mir_mods}"
echo "syx_driver_sidecar_rs_count=${driver_mods}"
echo "syx_semantic_sidecar_rs_count=${semantic_mods}"
echo "== census done (file-split baseline; ≠ S2-05 [x]) =="
