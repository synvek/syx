#!/usr/bin/env bash
# Prebuild slim libsyx_rt.a + syx-host-{sql,http,tls} for `syx build --native`.
# Link path never runs cargo — this script (or the release packager) must run first.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

PROFILE="${1:-release}" # release | debug
TARGET_DIR="${SYX_NATIVE_SHAKE_TARGET_DIR:-$ROOT/target/native-host-shake}"

# Match default native link: slim rt with file hosts; async only if requested.
features="file"
if [[ "${SYX_NATIVE_ASYNC:-}" == "1" || "${SYX_NATIVE_ASYNC:-}" == "true" ]]; then
  features="async-runtime,file"
fi
args=(build -p syx-rt -p syx-host-sql -p syx-host-http -p syx-host-tls
  --no-default-features --features "$features")
if [[ "$PROFILE" == "release" ]]; then
  args+=(--release)
elif [[ "$PROFILE" != "debug" ]]; then
  echo "usage: $0 [release|debug]" >&2
  exit 2
fi

echo "building native shake libs → ${TARGET_DIR}/${PROFILE} (features=$features)"
CARGO_TARGET_DIR="$TARGET_DIR" cargo "${args[@]}"

dir="$TARGET_DIR/$PROFILE"
for f in libsyx_rt.a syx_rt.lib libsyx_host_sql.a syx_host_sql.lib \
  libsyx_host_http.a syx_host_http.lib libsyx_host_tls.a syx_host_tls.lib; do
  if [[ -f "$dir/$f" ]]; then
    echo "  ok $dir/$f ($(wc -c <"$dir/$f" | tr -d ' ') bytes)"
  fi
done

if [[ ! -f "$dir/libsyx_rt.a" && ! -f "$dir/syx_rt.lib" ]]; then
  echo "ERROR: slim syx-rt archive missing under $dir" >&2
  exit 1
fi
echo "done — syx build --native will resolve these without spawning cargo"
