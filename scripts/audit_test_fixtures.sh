#!/usr/bin/env bash
# Audit .sy fixtures referenced by permanent driver test suites.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TESTS="$ROOT/crates/syx-driver/tests"
FIXTURES="$TESTS/fixtures"

PERMANENT=(
  integration.rs smoke.rs macro.rs backend.rs
  parity_vm.rs parity_native.rs parity_wasm.rs plugin_smoke.rs
  concurrency.rs stdlib.rs
)

usage() {
  echo "Usage: $0 [--check]"
  echo "  --check  fail if orphaned fixtures exist under tests/fixtures/"
}

CHECK=0
for arg in "$@"; do
  case "$arg" in
    --check) CHECK=1 ;;
    -h|--help) usage; exit 0 ;;
    *) echo "unknown arg: $arg"; usage; exit 1 ;;
  esac
done

REFERENCED_FILE="$(mktemp)"
trap 'rm -f "$REFERENCED_FILE"' EXIT

for f in "${PERMANENT[@]}"; do
  rs="$TESTS/$f"
  [[ -f "$rs" ]] || continue
  prefix=""
  case "$f" in
    parity_*.rs) prefix="parity/" ;;
    concurrency.rs) prefix="concurrency/" ;;
    stdlib.rs) prefix="stdlib/" ;;
  esac
  rg -o 'fixtures/([a-zA-Z0-9_/]+)\.sy' "$rs" 2>/dev/null | sed 's|fixtures/||;s|\.sy||' >> "$REFERENCED_FILE" || true
  if [[ -n "$prefix" ]]; then
    rg -o '(run|run_mir_printed|run_native_fixture|run_wasm|fixture)\("([a-zA-Z0-9_/]+)\.sy"\)' "$rs" 2>/dev/null \
      | sed "s|.*(\"\\([^\"]*\\)\\.sy\")|${prefix}\\1|" >> "$REFERENCED_FILE" || true
    rg -o '(run|run_mir_printed|run_native_fixture|run_wasm|fixture)\("([a-zA-Z0-9_/]+)"\)' "$rs" 2>/dev/null \
      | sed "s|.*(\"\\([^\"]*\\)\")|${prefix}\\1|" >> "$REFERENCED_FILE" || true
  else
    rg -o '(run|run_mir_printed|run_native_fixture|run_wasm|fixture)\("([a-zA-Z0-9_/]+)\.sy"\)' "$rs" 2>/dev/null \
      | sed 's/.*("\([^"]*\)\.sy")/\1/' >> "$REFERENCED_FILE" || true
  fi
  rg -o 'fixture\("([a-zA-Z0-9_/]+)"\)' "$rs" 2>/dev/null | sed 's/.*("\([^"]*\)")/\1/' >> "$REFERENCED_FILE" || true
  { rg -o 'projects/conformance_cross_module' "$rs" 2>/dev/null || true; } | while read -r _; do
    echo "projects/conformance_cross_module/app/main" >> "$REFERENCED_FILE"
    echo "projects/conformance_cross_module/witness/lib" >> "$REFERENCED_FILE"
  done
  { rg -o 'access/fileprivate_cross_module' "$rs" 2>/dev/null || true; } | while read -r _; do
    echo "access/fileprivate_cross_module/app/main" >> "$REFERENCED_FILE"
    echo "access/fileprivate_cross_module/app/helper" >> "$REFERENCED_FILE"
  done
  { rg -o 'syntax/top_level_rejected' "$rs" 2>/dev/null || true; } | while read -r _; do
    echo "syntax/top_level_rejected/main" >> "$REFERENCED_FILE"
    echo "syntax/top_level_rejected/lib" >> "$REFERENCED_FILE"
  done
done

sort -u "$REFERENCED_FILE" -o "$REFERENCED_FILE"

ORPHAN=0
while IFS= read -r -d '' syx; do
  rel="${syx#"$FIXTURES"/}"
  base="${rel%.sy}"
  [[ "$base" == lsp/* ]] && continue
  if ! grep -Fxq "$base" "$REFERENCED_FILE" 2>/dev/null; then
    echo "orphan: $base"
    ORPHAN=$((ORPHAN + 1))
  fi
done < <(find "$FIXTURES" -name '*.sy' -print0 2>/dev/null || true)

echo "Permanent suites: ${#PERMANENT[@]} rs files"
echo "Referenced fixtures: $(wc -l < "$REFERENCED_FILE" | tr -d ' ')"
echo "Orphan fixtures: $ORPHAN"

if (( CHECK )) && (( ORPHAN > 0 )); then
  exit 1
fi

exit 0
