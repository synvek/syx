//! Compile smoke for permanent fixtures (Phase 62 gate).
// was: fixture_compile

use std::path::PathBuf;
use syx_driver::{compile_file_with_options, DriverOptions};

fn compiles(rel: &str) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(rel);
    if !path.exists() {
        return;
    }
    compile_file_with_options(&path, &DriverOptions::default()).expect(&format!("compile {rel}"));
}

macro_rules! fixture_compile {
    ($name:ident, $path:expr) => {
        #[test]
        fn $name() {
            compiles($path);
        }
    };
}

fixture_compile!(fc_parity_generics, "parity/generics.sy");
fixture_compile!(fc_parity_hof, "parity/hof.sy");
fixture_compile!(fc_parity_closures, "parity/closures_capture.sy");
fixture_compile!(fc_parity_protocol, "parity/protocol_witness.sy");
fixture_compile!(fc_parity_actor, "parity/actor_isolation.sy");
fixture_compile!(fc_parity_dataclass, "parity/dataclass_synth.sy");
fixture_compile!(fc_parity_array_hof, "parity/array_hof.sy");
fixture_compile!(fc_macros_matrix, "macros/macro_matrix.sy");
fixture_compile!(fc_macros_engine, "macros/macro_engine.sy");
fixture_compile!(fc_macros_hygiene, "macros/hygiene_escape_ok.sy");
fixture_compile!(fc_macros_attached, "macros/attached_wrapper.sy");
fixture_compile!(fc_macros_codable, "macros/codable_l2_external.sy");
fixture_compile!(fc_gc_object, "gc/gc_object_ref.sy");
fixture_compile!(fc_stdlib_io, "stdlib/io_bytes_roundtrip.sy");
fixture_compile!(fc_stdlib_file, "stdlib/file_handle_bytes.sy");
fixture_compile!(fc_syntax_no_parens, "syntax/control_flow_no_parens.sy");
fixture_compile!(fc_access_fileprivate, "access/fileprivate_local.sy");
fixture_compile!(fc_operators_matrix, "operators/host_matrix.sy");
fixture_compile!(fc_language_array, "language/array_dict_props.sy");
fixture_compile!(fc_concurrency_main_actor, "concurrency/main_actor_fn.sy");
fixture_compile!(fc_lsp_result, "lsp/result_generic.sy");
fixture_compile!(fc_lsp_freestanding, "lsp/freestanding_decl.sy");
fixture_compile!(fc_lsp_witness, "lsp/witness_v11.sy");
fixture_compile!(fc_lsp_jit, "lsp/jit_tier2_hotloop.sy");
fixture_compile!(fc_lsp_embed, "lsp/embed_sandbox_v14.sy");
fixture_compile!(fc_macros_generic_witness, "macros/generic_witness_constraint.sy");
fixture_compile!(fc_macros_attached_conf, "macros/attached_conformance_dispatch.sy");
fixture_compile!(fc_parity_as_is, "parity/as_is_cast.sy");
fixture_compile!(fc_types_dict, "types/dict_hashable_key.sy");
fixture_compile!(fc_protocol_for_in, "protocol/for_in_array.sy");
