#!/usr/bin/env bash
# Collect platform link import libs / stubs into syx/lib/sysroot/<triple>/.
# NOT invoked by release.yml for now (Windows official .lib redistrib concerns).
# Kept for experiments / a future self-built import-lib pipeline.
set -euo pipefail

DEST="${1:?usage: collect-sysroot.sh <dest-dir> [triple]}"
TRIPLE="${2:-}"
mkdir -p "${DEST}"

copy_one() {
  local src="$1"
  local name
  name="$(basename "${src}")"
  if [ -f "${src}" ]; then
    cp -f "${src}" "${DEST}/${name}"
    echo "  + ${name}"
    return 0
  fi
  return 1
}

case "$(uname -s)" in
  Darwin)
    TRIPLE="${TRIPLE:-$(uname -m | sed 's/arm64/aarch64/')-apple-darwin}"
    # Marker for SDK layout; prefer host CLT/Xcode via xcrun when present.
    {
      echo "SYX link sysroot (macOS)"
      echo "triple=${TRIPLE}"
      if command -v xcrun >/dev/null 2>&1; then
        sdk="$(xcrun --sdk macosx --show-sdk-path 2>/dev/null || true)"
        echo "build_host_sdk=${sdk}"
      fi
      echo "End-user machines need Command Line Tools (libSystem) or this SDK's documented fallback."
    } > "${DEST}/README.txt"
    # Record sdk path for machines that keep the same CLT layout (best-effort).
    if command -v xcrun >/dev/null 2>&1; then
      sdk="$(xcrun --sdk macosx --show-sdk-path 2>/dev/null || true)"
      if [ -n "${sdk}" ] && [ -d "${sdk}" ]; then
        printf '%s\n' "${sdk}" > "${DEST}/sdk-path.txt"
      fi
    fi
    touch "${DEST}/.syx-sysroot"
    echo "macOS sysroot marker written → ${DEST}"
    ;;
  Linux)
    TRIPLE="${TRIPLE:-$(uname -m | sed 's/x86_64/x86_64/;s/aarch64/aarch64/')-unknown-linux-gnu}"
    {
      echo "SYX link sysroot (Linux)"
      echo "triple=${TRIPLE}"
      echo "Default link uses host libc (-lc -ldl -lpthread -lm)."
    } > "${DEST}/README.txt"
    touch "${DEST}/.syx-sysroot"
    echo "Linux sysroot marker written → ${DEST}"
    ;;
  MINGW*|MSYS*|CYGWIN*|Windows_NT)
    echo "Use collect-sysroot.ps1 on Windows runners" >&2
    exit 1
    ;;
  *)
    echo "Unsupported host $(uname -s)" >&2
    exit 1
    ;;
esac
