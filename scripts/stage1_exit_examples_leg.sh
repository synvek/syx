#!/usr/bin/env bash
# S1-I26: one-shot Stage-1 exit examples-leg smoke (≠ stage-1 exit claim).
# Runs VM allowlist contract + S0 three-backend thin smoke (hello / hot_loop).
# See docs/adr/stage1-exit-examples-leg.md
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== Stage-1 exit examples leg (smoke only; NOT an exit claim) =="
bash scripts/examples_vm_contract.sh
bash scripts/s0_backend_smoke.sh
echo "== exit-leg smoke OK (≠ stage-1 exit claim) =="
