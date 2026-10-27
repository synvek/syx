#!/usr/bin/env bash
# Build the Rust cdylib and point plugin.json at the correct shared library name.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT"

cargo build --release

case "$(uname -s)" in
  Darwin) LIB_NAME="libmygreet_plugin.dylib" ;;
  Linux)  LIB_NAME="libmygreet_plugin.so" ;;
  MINGW*|MSYS*|CYGWIN*|Windows_NT) LIB_NAME="mygreet_plugin.dll" ;;
  *)
    echo "unsupported OS: $(uname -s)" >&2
    exit 1
    ;;
esac

SRC="$ROOT/target/release/$LIB_NAME"
if [[ ! -f "$SRC" ]]; then
  echo "missing built library: $SRC" >&2
  exit 1
fi

cp -f "$SRC" "$ROOT/$LIB_NAME"

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
