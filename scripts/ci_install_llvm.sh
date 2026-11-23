#!/usr/bin/env bash
# Install a trimmed official LLVM tarball for full-native CI / release.
# Required env:
#   LLVM_VERSION  e.g. 21.1.6
#   LLVM_ASSET    e.g. LLVM-21.1.6-Linux-X64.tar.xz
# Optional:
#   GITHUB_WORKSPACE — checkout root (defaults to repo root relative to this script)
#   GITHUB_ENV       — if set, append SYX_LLVM_PREFIX + LLVM_SYS_211_PREFIX
# Side effect: sibling ../llvm next to the workspace.
set -euo pipefail

if [ -z "${LLVM_VERSION:-}" ]; then
  echo "ERROR: LLVM_VERSION is required" >&2
  exit 1
fi
if [ -z "${LLVM_ASSET:-}" ]; then
  echo "ERROR: LLVM_ASSET is required" >&2
  exit 1
fi

# Git Bash on Windows exposes RUNNER_TEMP / GITHUB_WORKSPACE as C:\...
# which breaks tar/curl path handling ("Cannot connect to C:"). Normalize.
to_unix() {
  if command -v cygpath >/dev/null 2>&1; then
    cygpath -u "$1"
  else
    printf '%s\n' "$1"
  fi
}
to_native() {
  # Path for Rust / llvm-sys (Windows needs drive-letter paths).
  if command -v cygpath >/dev/null 2>&1; then
    cygpath -w "$1"
  elif (cd "$1" && pwd -W >/dev/null 2>&1); then
    (cd "$1" && pwd -W)
  else
    (cd "$1" && pwd)
  fi
}

if [ -n "${GITHUB_WORKSPACE:-}" ]; then
  WS_UNIX="$(to_unix "${GITHUB_WORKSPACE}")"
else
  ROOT="$(cd "$(dirname "$0")/.." && pwd)"
  WS_UNIX="$(to_unix "${ROOT}")"
fi

# Prefer workspace-adjacent dirs so we never depend on RUNNER_TEMP shape.
PREFIX_UNIX="$(cd "${WS_UNIX}/.." && pwd)/llvm"
STAGE_UNIX="$(cd "${WS_UNIX}/.." && pwd)/llvm-extract"
DL_UNIX="$(cd "${WS_UNIX}/.." && pwd)/llvm-download"
URL="https://github.com/llvm/llvm-project/releases/download/llvmorg-${LLVM_VERSION}/${LLVM_ASSET}"

rm -rf "${PREFIX_UNIX}" "${STAGE_UNIX}"
mkdir -p "${STAGE_UNIX}" "${DL_UNIX}"

echo "Downloading ${URL}"
ARCHIVE_UNIX="${DL_UNIX}/${LLVM_ASSET}"
# Download to disk first (reliable on Windows Git Bash), then extract.
curl -fsSL --retry 5 --retry-delay 5 -o "${ARCHIVE_UNIX}" "${URL}"
tar -xJf "${ARCHIVE_UNIX}" -C "${STAGE_UNIX}"
rm -f "${ARCHIVE_UNIX}"

# Official tarballs have a single top-level directory.
TOP="$(find "${STAGE_UNIX}" -mindepth 1 -maxdepth 1 -type d | head -n 1)"
if [ -z "${TOP}" ]; then
  echo "ERROR: no top-level directory in ${LLVM_ASSET}" >&2
  ls -la "${STAGE_UNIX}" >&2 || true
  exit 1
fi
mkdir -p "${PREFIX_UNIX}"
shopt -s dotglob nullglob
mv "${TOP}"/* "${PREFIX_UNIX}/"
shopt -u dotglob nullglob
rm -rf "${STAGE_UNIX}" "${DL_UNIX}"

# Drop components not needed for inkwell/llvm-sys + syx-lld:
# need: bin/llvm-config*, bin/lld*, bin/clang* (Linux full-native link
# driver; gcc rejects -fuse-ld=/abs/path), include/{llvm,llvm-c,lld},
# lib/LLVM*|liblld*|Polly*
# Keep lib/clang (clang resource dir); drop only unused include trees.
rm -rf \
  "${PREFIX_UNIX}/share" \
  "${PREFIX_UNIX}/libexec" \
  "${PREFIX_UNIX}/python" \
  "${PREFIX_UNIX}/lib/python"* \
  "${PREFIX_UNIX}/lib/ocaml" \
  "${PREFIX_UNIX}/include/clang" \
  "${PREFIX_UNIX}/include/clang-c" \
  "${PREFIX_UNIX}/include/clang-tidy" \
  "${PREFIX_UNIX}/include/mlir" \
  "${PREFIX_UNIX}/include/mlir-c" \
  "${PREFIX_UNIX}/include/flang" \
  "${PREFIX_UNIX}/include/lldb" \
  "${PREFIX_UNIX}/include/openmp" \
  "${PREFIX_UNIX}/include/c++" \
  "${PREFIX_UNIX}/include/polly" \
  || true

if [ -d "${PREFIX_UNIX}/bin" ]; then
  while IFS= read -r -d '' f; do
    base="$(basename "${f}")"
    case "${base}" in
      llvm-config|llvm-config.exe|lld|lld.exe|ld64.lld|ld.lld|lld-link|lld-link.exe|wasm-ld|wasm-ld.exe|clang|clang++|clang-[0-9]*|clang++-[0-9]*|*.dll)
        ;;
      *)
        rm -rf "${f}"
        ;;
    esac
  done < <(find "${PREFIX_UNIX}/bin" -mindepth 1 -maxdepth 1 -print0)
fi

if [ -d "${PREFIX_UNIX}/lib" ]; then
  while IFS= read -r -d '' f; do
    base="$(basename "${f}")"
    case "${base}" in
      libLLVM*|LLVM*|liblld*|lld*|libPolly*|Polly*|cmake|libzstd*|zstd*|libxml2*|Remarks*)
        ;;
      *)
        rm -rf "${f}"
        ;;
    esac
  done < <(find "${PREFIX_UNIX}/lib" -mindepth 1 -maxdepth 1 -print0)
fi

CFG="${PREFIX_UNIX}/bin/llvm-config"
if [ ! -f "${CFG}" ]; then
  CFG="${PREFIX_UNIX}/bin/llvm-config.exe"
fi
if [ ! -f "${CFG}" ]; then
  echo "ERROR: llvm-config missing under ${PREFIX_UNIX}/bin" >&2
  ls -la "${PREFIX_UNIX}/bin" >&2 || true
  exit 1
fi
echo "Installed LLVM at ${PREFIX_UNIX}"
"${CFG}" --version
"${CFG}" --prefix
du -sh "${PREFIX_UNIX}" "${PREFIX_UNIX}/bin" "${PREFIX_UNIX}/lib" "${PREFIX_UNIX}/include" || true
df -h || true

# llvm-sys reads LLVM_SYS_211_PREFIX (not SYX_LLVM_PREFIX). Export both
# as native absolute paths so CI does not rely on .cargo relative env.
NATIVE_PREFIX="$(to_native "${PREFIX_UNIX}")"
if [ -n "${GITHUB_ENV:-}" ]; then
  {
    echo "SYX_LLVM_PREFIX=${NATIVE_PREFIX}"
    echo "LLVM_SYS_211_PREFIX=${NATIVE_PREFIX}"
  } >> "${GITHUB_ENV}"
fi
export SYX_LLVM_PREFIX="${NATIVE_PREFIX}"
export LLVM_SYS_211_PREFIX="${NATIVE_PREFIX}"
echo "SYX_LLVM_PREFIX=${NATIVE_PREFIX}"
echo "LLVM_SYS_211_PREFIX=${NATIVE_PREFIX}"
