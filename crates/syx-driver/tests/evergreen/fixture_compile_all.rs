//! Phase 64 fixture compile matrix — compile smoke for all permanent fixtures.
// was: fixture_compile_all
use std::path::PathBuf;
use syx_driver::{compile_file_with_options, DriverOptions};
fn compiles(rel: &str) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(rel);
    if !path.exists() { return; }
    compile_file_with_options(&path, &DriverOptions::default()).unwrap_or_else(|e| panic!("compile {{rel}}: {{e:?}}"));
}
#[test]
fn fc_all_000_collection_array_dict_props_syx() { compiles("collection/array_dict_props.sy"); }

#[test]
fn fc_all_001_types_as_is_cast_syx() { compiles("types/as_is_cast.sy"); }

#[test]
fn fc_all_002_types_dict_hashable_key_syx() { compiles("types/dict_hashable_key.sy"); }

#[test]
fn fc_all_003_types_greeter_extension_syx() { compiles("types/greeter_extension.sy"); }

#[test]
fn fc_all_004_types_inheritance_basic_syx() { compiles("types/inheritance_basic.sy"); }

#[test]
fn fc_all_005_types_nested_types_syx() { compiles("types/nested_types.sy"); }

#[test]
fn fc_all_006_types_property_observer_syx() { compiles("types/property_observer.sy"); }

#[test]
fn fc_all_007_access_fileprivate_local_syx() { compiles("access/fileprivate_local.sy"); }

#[test]
fn fc_all_008_access_fileprivate_cross_module_app_helper_syx() { compiles("access/fileprivate_cross_module/app/helper.sy"); }

#[test]
fn fc_all_009_access_fileprivate_cross_module_app_main_syx() { compiles("access/fileprivate_cross_module/app/main.sy"); }

#[test]
fn fc_all_010_operators_assign_host_syx() { compiles("operators/assign_host.sy"); }

#[test]
fn fc_all_011_operators_host_matrix_syx() { compiles("operators/host_matrix.sy"); }

#[test]
fn fc_all_012_operators_user_vec_syx() { compiles("operators/user_vec.sy"); }

#[test]
fn fc_all_013_projects_conformance_cross_module_witness_lib_syx() { compiles("projects/conformance_cross_module/witness/lib.sy"); }

#[test]
fn fc_all_014_projects_conformance_cross_module_app_main_syx() { compiles("projects/conformance_cross_module/app/main.sy"); }

#[test]
fn fc_all_015_parity_actor_isolation_syx() { compiles("parity/actor_isolation.sy"); }

#[test]
fn fc_all_016_parity_array_hof_syx() { compiles("parity/array_hof.sy"); }

#[test]
fn fc_all_017_parity_as_is_cast_syx() { compiles("parity/as_is_cast.sy"); }

#[test]
fn fc_all_018_parity_closures_capture_syx() { compiles("parity/closures_capture.sy"); }

#[test]
fn fc_all_019_parity_dataclass_synth_syx() { compiles("parity/dataclass_synth.sy"); }

#[test]
fn fc_all_020_parity_dict_hashable_key_syx() { compiles("parity/dict_hashable_key.sy"); }

#[test]
fn fc_all_021_parity_generics_syx() { compiles("parity/generics.sy"); }

#[test]
fn fc_all_022_parity_hof_syx() { compiles("parity/hof.sy"); }

#[test]
fn fc_all_023_parity_protocol_witness_syx() { compiles("parity/protocol_witness.sy"); }

#[test]
fn fc_all_024_lsp_borrow_regions_v8_full_ok_syx() { compiles("lsp/borrow_regions_v8_full_ok.sy"); }

#[test]
fn fc_all_025_lsp_distributed_actor_uds_prod_syx() { compiles("lsp/distributed_actor_uds_prod.sy"); }

#[test]
fn fc_all_026_lsp_embed_sandbox_v14_syx() { compiles("lsp/embed_sandbox_v14.sy"); }

#[test]
fn fc_all_027_lsp_freestanding_decl_syx() { compiles("lsp/freestanding_decl.sy"); }

#[test]
fn fc_all_028_lsp_freestanding_decl_role_a_syx() { compiles("lsp/freestanding_decl_role_a.sy"); }

#[test]
fn fc_all_029_lsp_freestanding_decl_role_b_syx() { compiles("lsp/freestanding_decl_role_b.sy"); }

#[test]
fn fc_all_030_lsp_jit_tier2_hotloop_syx() { compiles("lsp/jit_tier2_hotloop.sy"); }

#[test]
fn fc_all_031_lsp_mir_jit_deopt_barrier_syx() { compiles("lsp/mir_jit_deopt_barrier.sy"); }

#[test]
fn fc_all_032_lsp_net_stream_tls_syx() { compiles("lsp/net_stream_tls.sy"); }

#[test]
fn fc_all_033_lsp_result_generic_syx() { compiles("lsp/result_generic.sy"); }

#[test]
fn fc_all_034_lsp_sendable_fail_syx() { compiles("lsp/sendable_fail.sy"); }

#[test]
fn fc_all_035_lsp_websocket_wss_prod_syx() { compiles("lsp/websocket_wss_prod.sy"); }

#[test]
fn fc_all_036_lsp_witness_v11_syx() { compiles("lsp/witness_v11.sy"); }

#[test]
fn fc_all_037_lsp_cross_module_app_main_syx() { compiles("lsp/cross_module/app/main.sy"); }

#[test]
fn fc_all_038_lsp_cross_module_math_math_syx() { compiles("lsp/cross_module/math/math.sy"); }

#[test]
fn fc_all_039_protocol_for_in_array_syx() { compiles("protocol/for_in_array.sy"); }

#[test]
fn fc_all_040_nested_cross_module_main_syx() { compiles("nested_cross_module/main.sy"); }

#[test]
fn fc_all_041_nested_cross_module_nest_pkg_nest_syx() { compiles("nested_cross_module/nest_pkg/nest.sy"); }

#[test]
fn fc_all_042_concurrency_actor_counter_syx() { compiles("concurrency/actor_counter.sy"); }

#[test]
fn fc_all_043_concurrency_main_actor_fn_syx() { compiles("concurrency/main_actor_fn.sy"); }

#[test]
fn fc_all_044_concurrency_sendable_detach_rejected_syx() { compiles("concurrency/sendable_detach_rejected.sy"); }

#[test]
fn fc_all_045_macros_attached_conformance_dispatch_syx() { compiles("macros/attached_conformance_dispatch.sy"); }

#[test]
fn fc_all_046_macros_attached_wrapper_syx() { compiles("macros/attached_wrapper.sy"); }

#[test]
fn fc_all_047_macros_codable_l2_external_syx() { compiles("macros/codable_l2_external.sy"); }

#[test]
fn fc_all_048_macros_generic_witness_constraint_syx() { compiles("macros/generic_witness_constraint.sy"); }

#[test]
fn fc_all_049_macros_generic_witness_missing_syx() { compiles("macros/generic_witness_missing.sy"); }

#[test]
fn fc_all_050_macros_hygiene_escape_ok_syx() { compiles("macros/hygiene_escape_ok.sy"); }

#[test]
fn fc_all_051_macros_macro_engine_syx() { compiles("macros/macro_engine.sy"); }

#[test]
fn fc_all_052_macros_macro_matrix_syx() { compiles("macros/macro_matrix.sy"); }

#[test]
fn fc_all_053_common_plugin_echo_syx() { compiles("common/plugin_echo.sy"); }

#[test]
fn fc_all_054_stdlib_file_handle_bytes_syx() { compiles("stdlib/file_handle_bytes.sy"); }

#[test]
fn fc_all_055_stdlib_io_bytes_roundtrip_syx() { compiles("stdlib/io_bytes_roundtrip.sy"); }

#[test]
fn fc_all_056_gc_gc_object_ref_syx() { compiles("gc/gc_object_ref.sy"); }

#[test]
fn fc_all_057_errors_typed_catch_syx() { compiles("errors/typed_catch.sy"); }

#[test]
fn fc_all_058_syntax_control_flow_no_parens_syx() { compiles("syntax/control_flow_no_parens.sy"); }

#[test]
fn fc_all_059_syntax_top_level_stmts_syx() { compiles("syntax/top_level_stmts.sy"); }

#[test]
fn fc_all_060_syntax_top_level_rejected_lib_syx() { compiles("syntax/top_level_rejected/lib.sy"); }

#[test]
fn fc_all_061_syntax_top_level_rejected_main_syx() { compiles("syntax/top_level_rejected/main.sy"); }

