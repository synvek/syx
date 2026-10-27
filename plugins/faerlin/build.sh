#!/usr/bin/env bash
# Build standalone faerlin cdylib (W1050) and install as a *system* plugin under repo bin/.
# Layout (install prefix next to syx):
#   bin/syx
#   bin/libfaerlin_plugin.{dylib,so,dll}   # system dylib
#   bin/faerlin/plugin.json                # system manifest
#   bin/plugins/                           # user extension plugins
set -euo pipefail
ROOT="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$ROOT/../.." && pwd)"
BIN_DIR="$REPO_ROOT/bin"
cd "$ROOT"

cargo build --release

case "$(uname -s)" in
  Darwin) LIB_NAME="libfaerlin_plugin.dylib" ;;
  Linux)  LIB_NAME="libfaerlin_plugin.so" ;;
  MINGW*|MSYS*|CYGWIN*|Windows_NT) LIB_NAME="faerlin_plugin.dll" ;;
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

# Keep a copy next to the crate for tests that load plugins/faerlin/<lib>.
cp -f "$SRC" "$ROOT/$LIB_NAME"

# Install into bin/ (system plugin prefix).
mkdir -p "$BIN_DIR/faerlin" "$BIN_DIR/plugins"
cp -f "$SRC" "$BIN_DIR/$LIB_NAME"

python3 - "$ROOT/plugin.json" "$BIN_DIR/faerlin/plugin.json" "$LIB_NAME" <<'PY'
import json, sys
src, dest, lib = sys.argv[1], sys.argv[2], sys.argv[3]
with open(src, "r", encoding="utf-8") as f:
    data = json.load(f)
# Manifest lives in bin/faerlin/; dylib is flat in bin/.
data["library"] = f"../{lib}"
with open(dest, "w", encoding="utf-8") as f:
    json.dump(data, f, indent=2)
    f.write("\n")
# Keep source-tree plugin.json library basename for local tests.
data["library"] = lib
with open(src, "w", encoding="utf-8") as f:
    json.dump(data, f, indent=2)
    f.write("\n")
PY

echo "Built $ROOT/$LIB_NAME"
echo "Installed system plugin → $BIN_DIR/$LIB_NAME + $BIN_DIR/faerlin/plugin.json"
