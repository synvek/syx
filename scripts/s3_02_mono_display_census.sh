#!/usr/bin/env bash
# S3-I42…I46: Stage-3 mono-display census (S3-02).
# Measures user-face vs internal mangled-name surfaces.
# Does not claim S3-02 [x].
# See docs/stage3-mono-display-census.md and docs/adr/stage3-s3-02-meta-type-display.md
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== S3-02 mono-display census =="

count_rg() {
  local pattern="$1"
  shift
  { rg -c "$pattern" "$@" 2>/dev/null || true; } | awk -F: '
    NF==1 { s+=$1; next }
    { s+=$2 }
    END { print s+0 }
  '
}

# User-face display pipe (S3-02 debt surface — counts call sites / helpers).
type_id_to_name_semantic=$(count_rg 'type_id_to_name' \
  crates/syx-semantic/src --glob '!**/tests/**')
type_id_to_name_lsp=$(count_rg 'type_id_to_name' \
  crates/syx-lsp/src \
  crates/syx-driver/src/lsp_ide.rs \
  crates/syx-semantic/src/semantic_ide.rs)
type_id_to_name_named_passthrough=$(count_rg 'TypeId::Named\(n\) => n\.clone\(\)' \
  crates/syx-semantic/src/semantic_type_codec.rs)
user_facing_type_name_helper=$(count_rg 'user_facing_type_name|demangle_mono_name' \
  crates --glob '!**/tests/**')
user_facing_mir_fn_name_helper=$(count_rg 'user_facing_mir_fn_name' \
  crates --glob '!**/tests/**')
user_facing_runtime_type_name_helper=$(count_rg 'user_facing_runtime_type_name' \
  crates --glob '!**/tests/**')
user_facing_meta_type_name_helper=$(count_rg 'user_facing_meta_type_name' \
  crates --glob '!**/tests/**')

# Debug / runtime / meta identity surfaces.
debug_mangled_fn_nail=$(count_rg 'Greeter_hello' crates/syx-debug/src/lib.rs)
# Identity API only (exclude user-facing wrapper name).
object_ref_type_name=$(count_rg '\bobject_ref_type_name\b' \
  crates/syx-runtime-core/src --glob '!**/tests/**')
# Identity API only (exclude type_expr_to_type_name_core / user_facing).
meta_type_expr_to_type_name=$(count_rg '\btype_expr_to_type_name\b' \
  crates/syx-meta/src --glob '!**/tests/**')

# Partial demangle / nested helpers (exist but incomplete for S3-02).
class_name_from_mangled=$(count_rg 'class_name_from_mangled|parse_mangled' \
  crates --glob '!**/tests/**')
nested_type_dotted=$(count_rg 'nested_type_dotted' \
  crates --glob '!**/tests/**')

# Internal mono / codegen (NOT S3-02 user-face debt — transparency only).
mangle_name_mono_def=$(count_rg 'pub fn mangle_name' crates/syx-mono/src)
mangle_name_codegen=$(count_rg 'mangle_name' \
  crates/syx-codegen/src --glob '!**/tests/**')

echo "type_id_to_name_semantic=${type_id_to_name_semantic}"
echo "type_id_to_name_lsp=${type_id_to_name_lsp}"
echo "type_id_to_name_named_passthrough=${type_id_to_name_named_passthrough}"
echo "user_facing_type_name_helper=${user_facing_type_name_helper}"
echo "user_facing_mir_fn_name_helper=${user_facing_mir_fn_name_helper}"
echo "user_facing_runtime_type_name_helper=${user_facing_runtime_type_name_helper}"
echo "user_facing_meta_type_name_helper=${user_facing_meta_type_name_helper}"
echo "debug_mangled_fn_nail=${debug_mangled_fn_nail}"
echo "object_ref_type_name=${object_ref_type_name}"
echo "meta_type_expr_to_type_name=${meta_type_expr_to_type_name}"
echo "class_name_from_mangled=${class_name_from_mangled}"
echo "nested_type_dotted=${nested_type_dotted}"
echo "mangle_name_mono_def=${mangle_name_mono_def}"
echo "mangle_name_codegen=${mangle_name_codegen}"
echo "== census done (S3-02 open baseline; ≠ S3-02 [x] / ≠ Stage-3 exit) =="
