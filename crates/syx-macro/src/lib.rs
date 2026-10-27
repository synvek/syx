//! Phase 49 ADR-070 M1: macro expansion engine.

#[cfg(test)]
mod phase119_unit;
#[cfg(test)]
mod phase120_unit;
#[cfg(test)]
mod phase121_unit;
#[cfg(test)]
mod phase122_unit;
#[cfg(test)]
mod phase123_unit;
#[cfg(test)]
mod phase124_unit;
#[cfg(test)]
mod phase125_unit;
#[cfg(test)]
mod phase126_unit;
#[cfg(test)]
mod phase127_unit;
#[cfg(test)]
mod phase128_unit;
#[cfg(test)]
mod phase129_unit;
#[cfg(test)]
mod phase130_unit;
#[cfg(test)]
mod phase131_unit;
#[cfg(test)]
mod phase132_unit;
#[cfg(test)]
mod phase133_unit;
#[cfg(test)]
mod phase134_unit;
#[cfg(test)]
mod phase135_unit;
#[cfg(test)]
mod phase136_unit;
#[cfg(test)]
mod phase137_unit;
#[cfg(test)]
mod phase138_unit;
mod attached;
mod attached_config;
mod builtin;
mod context;
mod engine;
mod external;
mod external_protocol;
mod external_sandbox;
mod hygiene;
mod parse_fragment;
mod registry;
mod result_builder;
mod roles;
mod service;
mod user;
mod warnings;
mod wrapper;

pub use attached_config::{attached_hygienic_v2_enabled, attached_source_template, macro_hygiene_enabled};
pub use external::{
    clear_imported_macros, expand_external_macros, lookup_external_macro, package_macro_registered,
    register_imported_external_macros, register_imported_macros, register_package_macros,
    ExternalMacroSpec,
};
pub use external_protocol::{
    expand_decls_via_protocol, expand_via_protocol, request_to_json, response_from_json,
    ExternalMacroExpander, InProcessMacroExpander, PluginMacroExpander, MacroExpandRequest, MacroExpandResponse,
};
pub use external_sandbox::{
    is_macro_sandbox_child_argv, macro_external_plugin_prod_fingerprint, macro_in_process_enabled,
    macro_sandbox_child_main, macro_sandbox_mode, wasm_external_macro_disabled,
};
pub use warnings::take_macro_warnings;

use syx_ast::Module;
use syx_diagnostics::SyxResult;

/// AST in / AST out: fixed-point macro expansion (Phase 56 M5).
/// Phase 119 M9: `@resultBuilder` is the post-fixed-point engine pass (`macro-pipeline-v2`).
/// Phase 120 M10 / Phase 121 M11: `@Service` DI scan + registry codegen.
pub fn expand_module(module: Module) -> SyxResult<Module> {
    let _ = take_macro_warnings();
    let mut module = engine::expand_to_fixed_point(module)?;
    result_builder::desugar_result_builders(&mut module)?;
    service::expand_service_macros(&mut module)?;
    Ok(module)
}

pub use result_builder::desugar_result_builders;
pub use service::{
    check_service_cycles, collect_beans, collect_pointcuts, collect_services, expand_service_macros,
    has_service_attr, parse_pointcut_pattern, pointcut_matches_join, pointcut_matches_method,
    queue_services_from_module,
    spec_to_meta_entry, BeanSpec, FieldAutowire, PointcutPattern, ServiceSpec, WithinWildcard,
};
pub use user::{
    decl_base_name_from_tokens, enumerate_decl_param_tokens, expand_decl_param_tokens,
    DeclExpandMode, MacroTokenKind,
};
