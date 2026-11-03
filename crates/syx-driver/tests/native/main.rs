//! Surface harness: native

#[path = "../support/matrix_fingerprint.rs"]
mod matrix_fingerprint;

#[path = "../support/construction_progress.rs"]
mod construction_progress;

mod lsp_stdio_initialize_smoke;
mod syx_test_report_formats;
mod syx_test_at_test_discovery;
mod syx_test_hooks;
mod syx_test_before;
mod syx_test_teardown;
mod syx_test_recursive_discovery;
mod syx_test_testing_asserts;
mod syx_test_registry;
mod syx_test_hostfn_honesty;
mod syx_test_async;
mod syx_bench_cli_allowlist;
mod language_exports_export_list_trailing_comma;
mod llvm_phi_clang;
mod llvm_stub_audit;

mod native_build_lto;
mod native_build_profile;
mod native_link;
mod native_llvm_async;
mod native_llvm_direct;
mod native_llvm_full;
mod native_llvm_stack;
mod native_llvm_sync_main;
mod native_llvm_user_fn;
mod native_locals_v10;
mod native_strict_parity;
mod native_strict_parity_env;
mod native_string_interp;
mod native_tier2_full_matrix;
mod wave_b_exit_thin;
