#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT"

chmod +x "$ROOT/plugins/mygreet/build.sh"
"$ROOT/plugins/mygreet/build.sh"

export SYX_HOST_PLUGINS=1
export SYX_HOST_PLUGIN_PATH="$ROOT/plugins/mygreet/plugin.json"

if command -v syx >/dev/null 2>&1; then
  SYX=(syx)
else
  SYX=(cargo run -q -p syx-cli --)
fi

"${SYX[@]}" run .
