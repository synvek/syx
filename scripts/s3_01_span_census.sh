#!/usr/bin/env bash
# S3-I1…I40: Stage-3 diagnostic Span census (S3-01).
# I2–I23: bare gates · I24: quality KPI names · I25: miette=1 ·
# I26–I39: positionless face gates (driver/vm/…/residual) ·
# I40: positionless_helper_calls filtered =0 (noise/policy subtracted).
# Does not claim S3-01 [x].
# See docs/census/stage3-diagnostic-span-census.md and docs/adr/stage3-s3-01-noise-claim-prep.md
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== S3-01 diagnostic span census =="

count_rg() {
  local pattern="$1"
  shift
  { rg -c "$pattern" "$@" 2>/dev/null || true; } | awk -F: '{s+=$2} END {print s+0}'
}

syx_diag_error_warning=$(count_rg 'SyxDiagnostic::(error|warning)\(' crates --glob '!**/tests/**')
with_span_calls=$(count_rg '\.with_span\(' crates --glob '!**/tests/**')
primary_span_none=$(count_rg 'primary_span:\s*None' crates --glob '!**/tests/**')

semantic_analyze_main_bare=$(count_rg 'SyxDiagnostic::(error|warning)\(' \
  crates/syx-semantic/src/semantic_analyze_expr.rs \
  crates/syx-semantic/src/semantic_analyze_stmt.rs \
  crates/syx-semantic/src/semantic_analyze_call.rs)

semantic_analyze_type_decl_bare=$(count_rg 'SyxDiagnostic::(error|warning)\(' \
  crates/syx-semantic/src/semantic_analyze_type_member.rs \
  crates/syx-semantic/src/semantic_analyze_decl.rs)

semantic_concurrency_bare=$(count_rg 'SyxDiagnostic::(error|warning)\(' \
  crates/syx-semantic/src/semantic_concurrency.rs)

semantic_hub_bare=$(count_rg 'SyxDiagnostic::(error|warning)\(' \
  crates/syx-semantic/src/semantic_analyzer.rs \
  crates/syx-semantic/src/type_checker.rs \
  crates/syx-semantic/src/semantic_types.rs \
  crates/syx-semantic/src/semantic_infer_host.rs \
  crates/syx-semantic/src/import_resolver.rs)

host_driver_compile_bare=$(count_rg 'SyxDiagnostic::(error|warning)\(' \
  crates/syx-driver/src/compile_linked.rs \
  crates/syx-driver/src/build_syx.rs \
  crates/syx-driver/src/emit.rs \
  crates/syx-driver/src/check_project.rs \
  crates/syx-driver/src/module_ref.rs)

# S3-I7 gate: driver ops residual.
host_driver_ops_bare=$(count_rg 'SyxDiagnostic::(error|warning)\(' \
  crates/syx-driver/src/format_lint.rs \
  crates/syx-driver/src/test_runner.rs \
  crates/syx-driver/src/native_build.rs \
  crates/syx-driver/src/run_exec.rs \
  crates/syx-driver/src/debug_repl.rs \
  crates/syx-driver/src/doc_project.rs \
  crates/syx-driver/src/driver_options.rs \
  crates/syx-driver/src/run_wasm.rs \
  crates/syx-driver/src/cranelift_stubs.rs)

# S3-I8 gate: llvm lib.rs residual.
host_llvm_bare=$(count_rg 'SyxDiagnostic::(error|warning)\(' \
  crates/syx-llvm/src/lib.rs)

# S3-I9 gate: async crate residual.
host_async_bare=$(count_rg 'SyxDiagnostic::(error|warning)\(' \
  crates/syx-async/src --glob '!**/tests/**')

# S3-I10 gate: codegen lib.rs residual.
host_codegen_bare=$(count_rg 'SyxDiagnostic::(error|warning)\(' \
  crates/syx-codegen/src/lib.rs)

# S3-I11 gate: wasm-run crate residual.
host_wasm_run_bare=$(count_rg 'SyxDiagnostic::(error|warning)\(' \
  crates/syx-wasm-run/src --glob '!**/tests/**')

# S3-I12 gate: pkg crate residual.
host_pkg_bare=$(count_rg 'SyxDiagnostic::(error|warning)\(' \
  crates/syx-pkg/src --glob '!**/tests/**')

# S3-I13 gate: macro crate residual (*.rs only).
host_macro_bare=$(count_rg 'SyxDiagnostic::(error|warning)\(' \
  crates/syx-macro/src --glob '*.rs')

# S3-I14 gate: vm hosts.rs slice.
host_vm_hosts_bare=$(count_rg 'SyxDiagnostic::(error|warning)\(' \
  crates/syx-vm/src/hosts.rs)

# S3-I15 gate: vm stdlib_phase89_hosts.rs slice.
host_vm_phase89_bare=$(count_rg 'SyxDiagnostic::(error|warning)\(' \
  crates/syx-vm/src/stdlib_phase89_hosts.rs)

# S3-I16 gate: vm stdlib_phase84_hosts.rs slice.
host_vm_phase84_bare=$(count_rg 'SyxDiagnostic::(error|warning)\(' \
  crates/syx-vm/src/stdlib_phase84_hosts.rs)

# S3-I17 gate: vm stdlib_phase120_hosts.rs slice.
host_vm_phase120_bare=$(count_rg 'SyxDiagnostic::(error|warning)\(' \
  crates/syx-vm/src/stdlib_phase120_hosts.rs)

# S3-I18 gate: vm stdlib_phase83_hosts.rs slice.
host_vm_phase83_bare=$(count_rg 'SyxDiagnostic::(error|warning)\(' \
  crates/syx-vm/src/stdlib_phase83_hosts.rs)

# S3-I19 gate: remaining stdlib_phase*_hosts batch (phase-tail).
host_vm_phase_tail_bare=$(count_rg 'SyxDiagnostic::(error|warning)\(' \
  crates/syx-vm/src/stdlib_phase90_hosts.rs \
  crates/syx-vm/src/stdlib_phase91_hosts.rs \
  crates/syx-vm/src/stdlib_phase92_hosts.rs \
  crates/syx-vm/src/stdlib_phase95_hosts.rs \
  crates/syx-vm/src/stdlib_phase86_hosts.rs \
  crates/syx-vm/src/stdlib_phase87_hosts.rs)

# S3-I20 gate: remaining non-wasm_stub mid hosts batch.
host_vm_mid_bare=$(count_rg 'SyxDiagnostic::(error|warning)\(' \
  crates/syx-vm/src/lib.rs \
  crates/syx-vm/src/http.rs \
  crates/syx-vm/src/os_hosts.rs \
  crates/syx-vm/src/channel_hosts.rs \
  crates/syx-vm/src/script_hosts.rs \
  crates/syx-vm/src/async_iter_hosts.rs \
  crates/syx-vm/src/gui_invoke.rs \
  crates/syx-vm/src/gui_dlopen.rs \
  crates/syx-vm/src/synui_hosts_stub.rs \
  crates/syx-vm/src/jit.rs)

# S3-I21 gate: vm wasm_stubs batch.
host_vm_wasm_stubs_bare=$(count_rg 'SyxDiagnostic::(error|warning)\(' \
  crates/syx-vm/src/wasm_stubs/stdlib_phase89_hosts.rs \
  crates/syx-vm/src/wasm_stubs/http.rs)

# S3-I22 gate: rt + cranelift-jit + debug batch.
host_rt_jit_debug_bare=$(count_rg 'SyxDiagnostic::(error|warning)\(' \
  crates/syx-rt/src/bridge.rs \
  crates/syx-rt/src/abi.rs \
  crates/syx-cranelift-jit/src/lib.rs \
  crates/syx-debug/src/lib.rs)

# S3-I23 gate: residual scatter batch (excl. syx-diagnostics).
host_residual_bare=$(count_rg 'SyxDiagnostic::(error|warning)\(' \
  crates/syx-mir-run/src/runtime.rs \
  crates/syx-mir/src/mir_codec.rs \
  crates/syx-mir/src/mir_lower_ctx.rs \
  crates/syx-meta/src/lib.rs \
  crates/syx-hir/src/lib.rs \
  crates/syx-cst/src/lib.rs \
  crates/syx-cst/src/lower_probe.rs \
  crates/syx-parser/src/lib.rs \
  crates/syx-wasm-emit/src/mir_emit.rs \
  crates/syx-fmt/src/lib.rs \
  crates/syx-compiler-wasm/src/lib.rs \
  crates/syx-lexer/src/lib.rs \
  crates/syx-lint/src/lib.rs)

mir_ir_span_none=$(count_rg 'span:\s*None' crates/syx-mir --glob '!**/tests/**')

# S3-I24…I40 claim-prep quality: production convenience *_error( (excl *_error_at / *_diag).
# S3-I40: subtract documented noise + policy carve-outs; nail filtered calls = 0.
# Noise: set_error/from_error/last_os_error/host_log_error/registered_body_error/
#        diagnostics_on_parse_error / *_is|has|no_parse|non_error / pN_*_error
# Policy (no SourceFile DUMMY wrappers): lockfile_error / reflect_specialization_miss_error
_pos_noise_re='(set_error|from_error|last_os_error|host_log_error|registered_body_error|diagnostics_on_parse_error|[a-z0-9_]+_(is|has|no_parse|non)_error|p[0-9]+_[a-z0-9_]*_error)\('
_pos_policy_re='(lockfile_error|reflect_specialization_miss_error)\('
_pos_exclude_re='(set_error|from_error|last_os_error|host_log_error|registered_body_error|diagnostics_on_parse_error|[a-z0-9_]+_(is|has|no_parse|non)_error|p[0-9]+_[a-z0-9_]*_error|lockfile_error|reflect_specialization_miss_error)\('
_pos_raw_lines=$(rg -N --no-heading '[a-z_][a-z0-9_]*_error\(' crates \
  --glob '!**/tests/**' --glob '!**/*_diag.rs' --glob '!**/*diag.rs' 2>/dev/null || true)
positionless_helper_noise=$(printf '%s\n' "$_pos_raw_lines" | rg -c "$_pos_noise_re" 2>/dev/null || true)
positionless_helper_noise=${positionless_helper_noise:-0}
positionless_helper_policy=$(printf '%s\n' "$_pos_raw_lines" | rg -c "$_pos_policy_re" 2>/dev/null || true)
positionless_helper_policy=${positionless_helper_policy:-0}
positionless_helper_calls=$(printf '%s\n' "$_pos_raw_lines" | rg -v "$_pos_exclude_re" 2>/dev/null \
  | rg -c '[a-z_][a-z0-9_]*_error\(' 2>/dev/null || true)
positionless_helper_calls=${positionless_helper_calls:-0}

# S3-I26: parser + driver IN faces — bare parser_error(/driver_error( only (excl _error_at).
positionless_parser_driver_bare=$(count_rg '(parser_error|driver_error)\(' \
  crates/syx-parser/src/lib.rs \
  crates/syx-driver/src/format_lint.rs \
  crates/syx-driver/src/check_project.rs \
  crates/syx-driver/src/run_exec.rs \
  crates/syx-driver/src/debug_repl.rs \
  crates/syx-driver/src/module_ref.rs \
  crates/syx-driver/src/driver_options.rs \
  crates/syx-driver/src/doc_project.rs)

# S3-I27: driver compile faces — bare driver_error( only (excl _error_at).
positionless_driver_compile_bare=$(count_rg 'driver_error\(' \
  crates/syx-driver/src/build_syx.rs \
  crates/syx-driver/src/compile_linked.rs)

# S3-I28: driver emit/test/native residual faces.
positionless_driver_emit_test_native_bare=$(count_rg 'driver_error\(' \
  crates/syx-driver/src/emit.rs \
  crates/syx-driver/src/test_runner.rs \
  crates/syx-driver/src/native_build.rs \
  crates/syx-driver/src/run_wasm.rs \
  crates/syx-driver/src/cranelift_stubs.rs)

# S3-I29: VM phase84 positionless — bare vm_error( only (excl _error_at).
positionless_vm_phase84_bare=$(count_rg 'vm_error\(' \
  crates/syx-vm/src/stdlib_phase84_hosts.rs)

# S3-I30: VM hosts.rs positionless.
positionless_vm_hosts_bare=$(count_rg 'vm_error\(' \
  crates/syx-vm/src/hosts.rs)

# S3-I31: VM phase89 native + wasm twin.
positionless_vm_phase89_bare=$(count_rg 'vm_error\(' \
  crates/syx-vm/src/stdlib_phase89_hosts.rs \
  crates/syx-vm/src/wasm_stubs/stdlib_phase89_hosts.rs)

# S3-I32: VM phase120 + http(+stub) + os accelerate pack.
positionless_vm_phase120_bare=$(count_rg 'vm_error\(' \
  crates/syx-vm/src/stdlib_phase120_hosts.rs)
positionless_vm_http_bare=$(count_rg 'vm_error\(' \
  crates/syx-vm/src/http.rs \
  crates/syx-vm/src/wasm_stubs/http.rs)
positionless_vm_os_bare=$(count_rg 'vm_error\(' \
  crates/syx-vm/src/os_hosts.rs)

# S3-I33: VM lib + phase83 + channel + phase_tail accelerate pack.
positionless_vm_lib_bare=$(count_rg 'vm_error\(' \
  crates/syx-vm/src/lib.rs)
positionless_vm_phase83_bare=$(count_rg 'vm_error\(' \
  crates/syx-vm/src/stdlib_phase83_hosts.rs)
positionless_vm_channel_bare=$(count_rg 'vm_error\(' \
  crates/syx-vm/src/channel_hosts.rs)
positionless_vm_phase_tail_bare=$(count_rg 'vm_error\(' \
  crates/syx-vm/src/stdlib_phase90_hosts.rs \
  crates/syx-vm/src/stdlib_phase91_hosts.rs \
  crates/syx-vm/src/stdlib_phase92_hosts.rs \
  crates/syx-vm/src/stdlib_phase95_hosts.rs \
  crates/syx-vm/src/stdlib_phase86_hosts.rs \
  crates/syx-vm/src/stdlib_phase87_hosts.rs)

# S3-I34: VM mid residual + wasm-run accelerate pack.
positionless_vm_mid_bare=$(count_rg 'vm_error\(' \
  crates/syx-vm/src/script_hosts.rs \
  crates/syx-vm/src/async_iter_hosts.rs \
  crates/syx-vm/src/gui_invoke.rs \
  crates/syx-vm/src/gui_dlopen.rs \
  crates/syx-vm/src/synui_hosts_stub.rs \
  crates/syx-vm/src/jit.rs)
positionless_wasm_run_bare=$(count_rg 'wasm_run_error\(' \
  crates/syx-wasm-run/src/lib.rs \
  crates/syx-wasm-run/src/bridge.rs)

# S3-I35: syx-pkg full crate positionless (excl pkg_diag definition).
positionless_pkg_bare=$(count_rg 'pkg_error\(' \
  crates/syx-pkg/src/registry.rs \
  crates/syx-pkg/src/sources/registry.rs \
  crates/syx-pkg/src/lock_enforce.rs \
  crates/syx-pkg/src/sources/git.rs \
  crates/syx-pkg/src/manifest.rs \
  crates/syx-pkg/src/lockfile.rs \
  crates/syx-pkg/src/lib.rs \
  crates/syx-pkg/src/sources/path.rs \
  crates/syx-pkg/src/installer.rs \
  crates/syx-pkg/src/package.rs \
  crates/syx-pkg/src/resolver.rs)

# S3-I36: syx-macro full crate positionless (excl macro_diag definition).
positionless_macro_bare=$(count_rg 'macro_error\(' \
  crates/syx-macro/src/service.rs \
  crates/syx-macro/src/builtin.rs \
  crates/syx-macro/src/user.rs \
  crates/syx-macro/src/external_sandbox.rs \
  crates/syx-macro/src/external_protocol.rs \
  crates/syx-macro/src/hygiene.rs \
  crates/syx-macro/src/engine.rs \
  crates/syx-macro/src/roles.rs \
  crates/syx-macro/src/parse_fragment.rs \
  crates/syx-macro/src/external.rs \
  crates/syx-macro/src/attached.rs)

# S3-I37: syx-async + syx-llvm positionless (excl *_diag definitions).
positionless_async_bare=$(count_rg 'async_error\(' \
  crates/syx-async/src/runtime.rs \
  crates/syx-async/src/hosts.rs \
  crates/syx-async/src/task.rs \
  crates/syx-async/src/lib.rs \
  crates/syx-async/src/lower.rs)

positionless_llvm_bare=$(count_rg 'llvm_error\(' \
  crates/syx-llvm/src/lib.rs)

# S3-I38: rt + cranelift-jit + debug + codegen positionless (excl *_diag).
positionless_rt_bare=$(count_rg 'rt_error\(' \
  crates/syx-rt/src/bridge.rs \
  crates/syx-rt/src/abi.rs)

positionless_jit_bare=$(count_rg 'jit_error\(' \
  crates/syx-cranelift-jit/src/lib.rs)

positionless_debug_bare=$(count_rg 'debug_error\(' \
  crates/syx-debug/src/lib.rs)

positionless_codegen_bare=$(count_rg 'codegen_error\(' \
  crates/syx-codegen/src/lib.rs)

# S3-I39: residual real positionless helpers (excl *_diag).
positionless_mir_bare=$(count_rg 'mir_error\(' \
  crates/syx-mir/src/mir_codec.rs \
  crates/syx-mir/src/mir_lower_ctx.rs)

positionless_mir_run_bare=$(count_rg 'mir_run_error\(' \
  crates/syx-mir-run/src/runtime.rs)

positionless_meta_bare=$(count_rg 'meta_error\(' \
  crates/syx-meta/src/lib.rs)

positionless_hir_bare=$(count_rg 'hir_error\(' \
  crates/syx-hir/src/lib.rs)

positionless_cst_bare=$(count_rg 'cst_error\(' \
  crates/syx-cst/src/lib.rs \
  crates/syx-cst/src/lower_probe.rs)

positionless_wasm_emit_bare=$(count_rg 'wasm_emit_error\(' \
  crates/syx-wasm-emit/src/mir_emit.rs)

positionless_fmt_bare=$(count_rg 'fmt_error\(' \
  crates/syx-fmt/src/lib.rs)

positionless_compiler_wasm_bare=$(count_rg 'compiler_wasm_error\(' \
  crates/syx-compiler-wasm/src/lib.rs)

positionless_lexer_bare=$(count_rg 'lexer_error\(' \
  crates/syx-lexer/src/lib.rs)

positionless_lint_bare=$(count_rg 'lint_error\(' \
  crates/syx-lint/src/lib.rs)

miette_help_impl=0
if rg -n '^\s*fn help\b' crates/syx-diagnostics/src/lib.rs >/dev/null 2>&1; then
  miette_help_impl=1
fi

# 1 only when with_span takes a non-underscore-discarded label parameter name.
with_span_label_wired=0
if rg -n 'fn with_span\(' crates/syx-diagnostics/src/lib.rs >/dev/null 2>&1 \
  && ! rg -n 'fn with_span\(.*_label' crates/syx-diagnostics/src/lib.rs >/dev/null 2>&1; then
  with_span_label_wired=1
fi

workspace_has_miette=0
if grep -E '^miette[[:space:]]*=' Cargo.toml >/dev/null 2>&1; then
  workspace_has_miette=1
fi

diag_crate_has_span_type=0
if grep -F 'pub struct Span' crates/syx-diagnostics/src/lib.rs >/dev/null 2>&1 \
  && grep -F 'primary_span' crates/syx-diagnostics/src/lib.rs >/dev/null 2>&1; then
  diag_crate_has_span_type=1
fi

echo "syx_diag_error_warning=${syx_diag_error_warning}"
echo "with_span_calls=${with_span_calls}"
echo "primary_span_none_literal=${primary_span_none}"
echo "semantic_analyze_main_bare=${semantic_analyze_main_bare}"
echo "semantic_analyze_type_decl_bare=${semantic_analyze_type_decl_bare}"
echo "semantic_concurrency_bare=${semantic_concurrency_bare}"
echo "semantic_hub_bare=${semantic_hub_bare}"
echo "host_driver_compile_bare=${host_driver_compile_bare}"
echo "host_driver_ops_bare=${host_driver_ops_bare}"
echo "host_llvm_bare=${host_llvm_bare}"
echo "host_async_bare=${host_async_bare}"
echo "host_codegen_bare=${host_codegen_bare}"
echo "host_wasm_run_bare=${host_wasm_run_bare}"
echo "host_pkg_bare=${host_pkg_bare}"
echo "host_macro_bare=${host_macro_bare}"
echo "host_vm_hosts_bare=${host_vm_hosts_bare}"
echo "host_vm_phase89_bare=${host_vm_phase89_bare}"
echo "host_vm_phase84_bare=${host_vm_phase84_bare}"
echo "host_vm_phase120_bare=${host_vm_phase120_bare}"
echo "host_vm_phase83_bare=${host_vm_phase83_bare}"
echo "host_vm_phase_tail_bare=${host_vm_phase_tail_bare}"
echo "host_vm_mid_bare=${host_vm_mid_bare}"
echo "host_vm_wasm_stubs_bare=${host_vm_wasm_stubs_bare}"
echo "host_rt_jit_debug_bare=${host_rt_jit_debug_bare}"
echo "host_residual_bare=${host_residual_bare}"
echo "positionless_helper_calls=${positionless_helper_calls}"
echo "positionless_helper_noise=${positionless_helper_noise}"
echo "positionless_helper_policy=${positionless_helper_policy}"
echo "positionless_parser_driver_bare=${positionless_parser_driver_bare}"
echo "positionless_driver_compile_bare=${positionless_driver_compile_bare}"
echo "positionless_driver_emit_test_native_bare=${positionless_driver_emit_test_native_bare}"
echo "positionless_vm_phase84_bare=${positionless_vm_phase84_bare}"
echo "positionless_vm_hosts_bare=${positionless_vm_hosts_bare}"
echo "positionless_vm_phase89_bare=${positionless_vm_phase89_bare}"
echo "positionless_vm_phase120_bare=${positionless_vm_phase120_bare}"
echo "positionless_vm_http_bare=${positionless_vm_http_bare}"
echo "positionless_vm_os_bare=${positionless_vm_os_bare}"
echo "positionless_vm_lib_bare=${positionless_vm_lib_bare}"
echo "positionless_vm_phase83_bare=${positionless_vm_phase83_bare}"
echo "positionless_vm_channel_bare=${positionless_vm_channel_bare}"
echo "positionless_vm_phase_tail_bare=${positionless_vm_phase_tail_bare}"
echo "positionless_vm_mid_bare=${positionless_vm_mid_bare}"
echo "positionless_wasm_run_bare=${positionless_wasm_run_bare}"
echo "positionless_pkg_bare=${positionless_pkg_bare}"
echo "positionless_macro_bare=${positionless_macro_bare}"
echo "positionless_async_bare=${positionless_async_bare}"
echo "positionless_llvm_bare=${positionless_llvm_bare}"
echo "positionless_rt_bare=${positionless_rt_bare}"
echo "positionless_jit_bare=${positionless_jit_bare}"
echo "positionless_debug_bare=${positionless_debug_bare}"
echo "positionless_codegen_bare=${positionless_codegen_bare}"
echo "positionless_mir_bare=${positionless_mir_bare}"
echo "positionless_mir_run_bare=${positionless_mir_run_bare}"
echo "positionless_meta_bare=${positionless_meta_bare}"
echo "positionless_hir_bare=${positionless_hir_bare}"
echo "positionless_cst_bare=${positionless_cst_bare}"
echo "positionless_wasm_emit_bare=${positionless_wasm_emit_bare}"
echo "positionless_fmt_bare=${positionless_fmt_bare}"
echo "positionless_compiler_wasm_bare=${positionless_compiler_wasm_bare}"
echo "positionless_lexer_bare=${positionless_lexer_bare}"
echo "positionless_lint_bare=${positionless_lint_bare}"
echo "miette_help_impl=${miette_help_impl}"
echo "with_span_label_wired=${with_span_label_wired}"
echo "mir_ir_span_none=${mir_ir_span_none}"
echo "workspace_has_miette=${workspace_has_miette}"
echo "diag_crate_has_span_type=${diag_crate_has_span_type}"
echo "== census done (S3-01 [~]; …+helper_calls filtered=0; ≠ S3-01 [x]) =="
