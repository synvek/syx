#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$ROOT/../../../.." && pwd)"
INCLUDE="$REPO_ROOT/crates/syx-host-plugin/include"

if [[ "$(uname -s)" == "Darwin" ]]; then
  LIB_NAME="libmycrypto_plugin.dylib"
  EXTRA_FLAGS=(-undefined dynamic_lookup)
else
  LIB_NAME="libmycrypto_plugin.so"
  EXTRA_FLAGS=()
fi

cc -shared -fPIC -O2 \
  "${EXTRA_FLAGS[@]}" \
  -o "$ROOT/$LIB_NAME" \
  "$ROOT/mycrypto_plugin.c" \
  -I"$INCLUDE"

python3 - "$ROOT/plugin.json" "$LIB_NAME" <<'PY'
import json, sys
path, lib = sys.argv[1], sys.argv[2]
with open(path, "r", encoding="utf-8") as f:
    data = json.load(f)
data["library"] = lib
with open(path, "w", encoding="utf-8") as f:
    json.dump(data, f, indent=2)
    f.write("\n")
PY

echo "Built $ROOT/$LIB_NAME"
