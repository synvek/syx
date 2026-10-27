#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT"

"$ROOT/plugins/mycrypto/build.sh"

export SYX_HOST_PLUGINS=1
export SYX_HOST_PLUGIN_PATH="$ROOT/plugins/mycrypto/plugin.json"

if command -v syx >/dev/null 2>&1; then
  SYX=syx
else
  SYX=(cargo run -q -p syx-cli --)
fi

if [[ ${#SYX[@]} -eq 1 ]]; then
  "$SYX" run .
else
  "${SYX[@]}" run .
fi
