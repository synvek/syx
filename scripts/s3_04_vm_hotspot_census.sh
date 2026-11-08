#!/usr/bin/env bash
# S3-I56…I71: Stage-3 VM-hotspot / fib(30) census (S3-04).
# I57–I70 construct/policy/scope · I71 scoped [x] claim.
# S3-04 claimed scoped [x]; next default I72 S3-05 open (≠ Stage-3 exit).
# See docs/stage3-vm-hotspot-census.md and docs/adr/stage3-s3-04-claim.md
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== S3-04 VM-hotspot census =="

count_rg() {
  local pattern="$1"
  shift
  { rg -c "$pattern" "$@" 2>/dev/null || true; } | awk -F: '
    NF==1 { s+=$1; next }
    { s+=$2 }
    END { print s+0 }
  '
}

# Structural axes (call / Value / JIT hooks).
value_clone_mentions=$(count_rg '\.clone\(\)' \
  crates/syx-vm/src --glob '!**/tests/**')
arg_clone_helper=$(count_rg 'arg\.clone\(\)' \
  crates/syx-vm/src --glob '!**/tests/**')
# S3-I57 hinge: bind-path clone must be 0.
arg_bind_clone=$(count_rg 'locals\[i\] = arg\.clone\(\)' \
  crates/syx-vm/src/lib.rs)
owned_args_helper=$(count_rg 'run_function_with_owned_args|S3-I57' \
  crates/syx-vm/src/lib.rs)
# S3-I58 hinge: locals frame buffer pool.
locals_frame_reuse_helper=$(count_rg 'locals_buf_pool|push_caller_locals_frame|recycle_and_restore_locals|S3-I58' \
  crates/syx-vm/src/lib.rs)
# S3-I59 hinge: LoadLocal/Dup scalar fastpath + StoreLocal move.
load_local_exec_fastpath=$(count_rg 'copy_value_for_stack|S3-I59' \
  crates/syx-vm/src/lib.rs)
store_local_slot_clone=$(count_rg 'locals\[\*i as usize\], v\.clone\(\)' \
  crates/syx-vm/src/lib.rs)
# S3-I60 hinge: Call free-function stack→locals (no intermediate Vec).
call_stack_args_helper=$(count_rg 'run_function_with_stack_args|call_can_stack_bind|S3-I60' \
  crates/syx-vm/src/lib.rs)
# S3-I61 hinge: try-free stack-args Call (conditional try_handler_frames).
try_free_call_helper=$(count_rg 'stack_args_needs_try_frame|track_try_frame|S3-I61' \
  crates/syx-vm/src/lib.rs)
# S3-I62 hinge: bind stack args then Nil-pad (no full-slot resize overwrite).
locals_prep_helper=$(count_rg 'bind_stack_args_into_locals|S3-I62' \
  crates/syx-vm/src/lib.rs)
# S3-I63 hinge: locals frame checkout+swap (no take empty window).
locals_frame_swap_helper=$(count_rg 'mem::swap|S3-I63' \
  crates/syx-vm/src/lib.rs)
# S3-I64 hinge: BeginTry compile-time cache (no per-call full-table scan).
begin_try_cache_helper=$(count_rg 'has_begin_try|S3-I64' \
  crates/syx-codegen/src/lib.rs crates/syx-vm/src/lib.rs)
# S3-I65 hinge: try-free interpret hot path (no catch/throw polls + hot opcodes).
tryfree_dispatch_helper=$(count_rg 'run_function_body_tryfree|S3-I65' \
  crates/syx-vm/src/lib.rs)
# S3-I66 hinge: JIT env gate cache (no per-Call env::var after first read).
jit_env_gate_cache_helper=$(count_rg 'JIT_FASTPATH_GATE|load_or_init_gate|S3-I66|reset_jit_env_gates_for_test' \
  crates/syx-vm/src/jit.rs)
# S3-I67 hinge: can_stack_bind compile-time cache (no per-Call mangle/ends_with).
can_stack_bind_cache_helper=$(count_rg 'can_stack_bind|compute_can_stack_bind|S3-I67' \
  crates/syx-codegen/src/lib.rs crates/syx-vm/src/lib.rs)
# S3-I68 hinge: claim-prep scaffolding (examples/fib + fib_criterion + ADR).
claim_prep_helper=$(count_rg 'S3-I68|claim-prep|fib_criterion' \
  docs/adr/stage3-s3-04-claim-prep.md crates/syx-driver/benches/fib_criterion.rs examples/fib.sy examples/fib20.sy)
# S3-I69 hinge: wall-strategy policy (ceiling + whitelist + dual axis).
wall_strategy_helper=$(count_rg 'S3-I69|wall-strategy|fib30_interp_ceiling' \
  docs/adr/stage3-s3-04-wall-strategy.md docs/stage3-vm-hotspot-census.md scripts/s3_04_vm_hotspot_census.sh)
# S3-I70 hinge: residual-scope (scoped acceptance + deferred <0.5).
residual_scope_helper=$(count_rg 'S3-I70|residual-scope|scoped acceptance' \
  docs/adr/stage3-s3-04-residual-scope.md docs/stage3-vm-hotspot-census.md plan/02-improvement-roadmap.md)
# S3-I71 hinge: scoped [x] claim.
s3_04_claim_helper=$(count_rg 'S3-I71|stage3-s3-04-claim|Claimed: scoped' \
  docs/adr/stage3-s3-04-claim.md docs/stage3-vm-hotspot-census.md plan/02-improvement-roadmap.md)
call_path_helper=$(count_rg 'run_function_with_args|run_function_with_owned_args|run_function_with_stack_args|fn call_' \
  crates/syx-vm/src/lib.rs)
suspend_frame_helper=$(count_rg 'VmSuspendFrame|struct VmSuspendFrame' \
  crates/syx-vm/src --glob '!**/tests/**')
hotspot_count_helper=$(count_rg 'fn hotspot_count|hotspot_count\(' \
  crates/syx-vm/src/jit.rs)
syx_jit_env_mentions=$(count_rg 'SYX_JIT_' \
  crates/syx-vm/src --glob '!**/tests/**')
# Criterion: fib bench absent at open.
fib_criterion_bench=0
if compgen -G "crates/syx-driver/benches/*fib*" >/dev/null 2>&1; then
  fib_criterion_bench=$(compgen -G "crates/syx-driver/benches/*fib*" | wc -l | tr -d ' ')
fi
examples_fib=0
if compgen -G "examples/fib*" >/dev/null 2>&1; then
  examples_fib=$(compgen -G "examples/fib*" | wc -l | tr -d ' ')
fi

echo "value_clone_mentions=${value_clone_mentions}"
echo "arg_clone_helper=${arg_clone_helper}"
echo "arg_bind_clone=${arg_bind_clone}"
echo "owned_args_helper=${owned_args_helper}"
echo "locals_frame_reuse_helper=${locals_frame_reuse_helper}"
echo "load_local_exec_fastpath=${load_local_exec_fastpath}"
echo "store_local_slot_clone=${store_local_slot_clone}"
echo "call_stack_args_helper=${call_stack_args_helper}"
echo "try_free_call_helper=${try_free_call_helper}"
echo "locals_prep_helper=${locals_prep_helper}"
echo "locals_frame_swap_helper=${locals_frame_swap_helper}"
echo "begin_try_cache_helper=${begin_try_cache_helper}"
echo "tryfree_dispatch_helper=${tryfree_dispatch_helper}"
echo "jit_env_gate_cache_helper=${jit_env_gate_cache_helper}"
echo "can_stack_bind_cache_helper=${can_stack_bind_cache_helper}"
echo "claim_prep_helper=${claim_prep_helper}"
echo "wall_strategy_helper=${wall_strategy_helper}"
echo "residual_scope_helper=${residual_scope_helper}"
echo "s3_04_claim_helper=${s3_04_claim_helper}"
echo "call_path_helper=${call_path_helper}"
echo "suspend_frame_helper=${suspend_frame_helper}"
echo "hotspot_count_helper=${hotspot_count_helper}"
echo "syx_jit_env_mentions=${syx_jit_env_mentions}"
echo "fib_criterion_bench=${fib_criterion_bench}"
echo "examples_fib=${examples_fib}"
echo "fib30_target_s=0.5"
echo "fib30_interp_ceiling_s=1.5-1.8"

SYX_BIN=""
if [[ -x "$ROOT/target/release/syx" ]]; then
  SYX_BIN="$ROOT/target/release/syx"
elif [[ -x "$ROOT/target/debug/syx" ]]; then
  SYX_BIN="$ROOT/target/debug/syx"
elif command -v syx >/dev/null 2>&1; then
  SYX_BIN="$(command -v syx)"
fi

fib30_s="na"
fib30_warm_s="na"
fib30_tier2_s="na"
hot_loop_s="na"
if [[ -n "$SYX_BIN" ]]; then
  WORK="$(mktemp -d "${TMPDIR:-/tmp}/syx-s3-04-fib.XXXXXX")"
  # S3-I68: prefer checked-in examples/fib.sy when present.
  if [[ -f "$ROOT/examples/fib.sy" ]]; then
    cp "$ROOT/examples/fib.sy" "$WORK/fib.sy"
  else
    cat >"$WORK/fib.sy" <<'EOF'
func fib(n: Int) -> Int {
    if (n < 2) { return n }
    return fib(n: n - 1) + fib(n: n - 2)
}
func main() {
    return fib(n: 30)
}
EOF
  fi
  cold_out="$(mktemp)"
  if /usr/bin/time -p env -u SYX_JIT_TIER2 -u SYX_JIT_FASTPATH "$SYX_BIN" run "$WORK/fib.sy" >"$WORK/out1.txt" 2>"$cold_out"; then
    fib30_s="$(awk '/^real / { print $2; exit }' "$cold_out")"
  else
    fib30_s="fail"
  fi
  warm_out="$(mktemp)"
  if /usr/bin/time -p env -u SYX_JIT_TIER2 -u SYX_JIT_FASTPATH "$SYX_BIN" run "$WORK/fib.sy" >"$WORK/out2.txt" 2>"$warm_out"; then
    fib30_warm_s="$(awk '/^real / { print $2; exit }' "$warm_out")"
  else
    fib30_warm_s="fail"
  fi
  # S3-I69 secondary axis: opt-in Tier2 (evidence only; ≠ claim primary).
  tier2_out="$(mktemp)"
  if /usr/bin/time -p env SYX_JIT_TIER2=1 "$SYX_BIN" run "$WORK/fib.sy" >"$WORK/out_t2.txt" 2>"$tier2_out"; then
    fib30_tier2_s="$(awk '/^real / { print $2; exit }' "$tier2_out")"
  else
    # Binary may lack jit-tier2 wiring; still emit a key (na/fail).
    if grep -qiE 'jit|tier2|unknown|unsupported' "$tier2_out" 2>/dev/null; then
      fib30_tier2_s="na"
    else
      fib30_tier2_s="fail"
    fi
  fi
  rm -f "$tier2_out"
  if [[ -f "$ROOT/examples/hot_loop.sy" ]]; then
    cp "$ROOT/examples/hot_loop.sy" "$WORK/hot_loop.sy"
    hl_out="$(mktemp)"
    if /usr/bin/time -p env -u SYX_JIT_TIER2 -u SYX_JIT_FASTPATH "$SYX_BIN" run "$WORK/hot_loop.sy" >"$WORK/out_hl.txt" 2>"$hl_out"; then
      hot_loop_s="$(awk '/^real / { print $2; exit }' "$hl_out")"
    else
      hot_loop_s="fail"
    fi
    rm -f "$hl_out"
  fi
  rm -rf "$WORK" "$cold_out" "$warm_out"
fi

echo "fib30_s=${fib30_s}"
echo "fib30_warm_s=${fib30_warm_s}"
echo "fib30_tier2_s=${fib30_tier2_s}"
echo "hot_loop_s=${hot_loop_s}"
echo "== census done (S3-04 I71 scoped [x]; ≠ Stage-3 exit / ≠ I72 this window) =="
