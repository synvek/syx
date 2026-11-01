//! Canonical frontend pipeline (ADR-135 / M1-A01).
//!
//! Order: parse → expand_macros(+inline desugar) → top-level desugar? → meta → async
//! → semantic → (mono → MIR). Pass errors are never swallowed.

mod desugar;

pub use desugar::{
    desugar_property_decls, desugar_qualified_ctor_calls, desugar_subscript_decls,
    desugar_top_level_stmts, expand_macros,
};

use std::collections::HashSet;

use indexmap::IndexMap;
use sha2::{Digest, Sha256};
use smol_str::SmolStr;
use syx_ast::{Module, WherePred};
use syx_diagnostics::{SourceFile, SyxDiagnostic, SyxResult};
use syx_meta::{check_effects, register_data_classes, validate_pure_functions};
use syx_mir::{encode_module, lower_module, MirModule};
use syx_mono::monomorphize_module;
use syx_parser::Parser;
use syx_semantic::{SemanticAnalyzer, TypeId};

/// Where the pipeline stops.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopAfter {
    /// After macro expand + optional top-level desugar (before meta/async).
    Expanded,
    /// After meta + async prepare (before semantic). Driver injects imports then continues.
    Prepared,
    Semantic,
    Mir,
}

/// Options for [`FrontendPipeline::run`]. Graph/import injection stays in the driver.
#[derive(Debug, Clone)]
pub struct FrontendOptions {
    pub stop_after: StopAfter,
    pub desugar_top_level: bool,
    pub collecting_semantic: bool,
    pub strict_concurrency: bool,
    pub default_isolation: Option<SmolStr>,
    pub import_symbols: Vec<(SmolStr, IndexMap<SmolStr, TypeId>)>,
    pub import_isolation: Vec<(SmolStr, IndexMap<SmolStr, SmolStr>)>,
    pub import_generic_methods: IndexMap<SmolStr, (Vec<SmolStr>, Vec<WherePred>)>,
    pub skip_prelude_name_shadow: bool,
    pub source: Option<SourceFile>,
}

impl Default for FrontendOptions {
    fn default() -> Self {
        Self {
            stop_after: StopAfter::Mir,
            desugar_top_level: true,
            collecting_semantic: false,
            strict_concurrency: false,
            default_isolation: None,
            import_symbols: Vec::new(),
            import_isolation: Vec::new(),
            import_generic_methods: IndexMap::new(),
            skip_prelude_name_shadow: false,
            source: None,
        }
    }
}

#[derive(Debug)]
pub struct FrontendArtifact {
    pub module: Module,
    pub semantic: Option<SemanticAnalyzer>,
    pub diagnostics: Vec<SyxDiagnostic>,
    pub instantiations: HashSet<(SmolStr, Vec<SmolStr>)>,
    pub mir: Option<MirModule>,
    pub mir_hash: Option<String>,
}

pub fn apply_meta_passes(module: &Module) -> SyxResult<()> {
    register_data_classes(module);
    let _effects = check_effects(module)?;
    validate_pure_functions(module)?;
    Ok(())
}

pub fn prepare_async_module(module: &mut Module) -> SyxResult<()> {
    syx_async::prepare_async_module(module)
}

pub struct FrontendPipeline;

impl FrontendPipeline {
    pub fn run(source: SourceFile, options: &FrontendOptions) -> SyxResult<FrontendArtifact> {
        let mut opts = options.clone();
        if opts.source.is_none() {
            opts.source = Some(source.clone());
        }
        let parsed = Parser::parse(source)?;
        Self::run_parsed(parsed, &opts)
    }

    pub fn run_parsed(module: Module, options: &FrontendOptions) -> SyxResult<FrontendArtifact> {
        let mut diagnostics = Vec::new();
        let mut module = expand_macros(module)?;
        if options.desugar_top_level {
            desugar_top_level_stmts(&mut module)?;
        }
        if matches!(options.stop_after, StopAfter::Expanded) {
            return Ok(FrontendArtifact {
                module,
                semantic: None,
                diagnostics,
                instantiations: HashSet::new(),
                mir: None,
                mir_hash: None,
            });
        }

        apply_meta_passes(&module)?;
        prepare_async_module(&mut module)?;

        if matches!(options.stop_after, StopAfter::Prepared) {
            return Ok(FrontendArtifact {
                module,
                semantic: None,
                diagnostics,
                instantiations: HashSet::new(),
                mir: None,
                mir_hash: None,
            });
        }

        let semantic = if options.collecting_semantic {
            let analyzer = SemanticAnalyzer::analyze_collecting_with_options(
                &module,
                &options.import_symbols,
                options.strict_concurrency,
                options.default_isolation.clone(),
                &options.import_isolation,
                options.source.as_ref(),
                &options.import_generic_methods,
                options.skip_prelude_name_shadow,
            );
            diagnostics.extend(analyzer.errors.clone());
            analyzer
        } else if options.import_symbols.is_empty()
            && options.import_generic_methods.is_empty()
            && !options.strict_concurrency
        {
            SemanticAnalyzer::analyze(&module)?
        } else {
            let analyzer = SemanticAnalyzer::analyze_collecting_with_options(
                &module,
                &options.import_symbols,
                options.strict_concurrency,
                options.default_isolation.clone(),
                &options.import_isolation,
                options.source.as_ref(),
                &options.import_generic_methods,
                options.skip_prelude_name_shadow,
            );
            if let Some(err) = analyzer.errors.first() {
                return Err(err.clone());
            }
            analyzer
        };
        let instantiations = semantic.instantiations.clone();

        if matches!(options.stop_after, StopAfter::Semantic) {
            return Ok(FrontendArtifact {
                module,
                semantic: Some(semantic),
                diagnostics,
                instantiations,
                mir: None,
                mir_hash: None,
            });
        }

        let mono = monomorphize_module(&module, &instantiations);
        let mut mir = lower_module(&mono)?;
        syx_mir::optimize(&mut mir);
        let mir_hash = Some(mir_module_hash(&mir));
        Ok(FrontendArtifact {
            module,
            semantic: Some(semantic),
            diagnostics,
            instantiations,
            mir: Some(mir),
            mir_hash,
        })
    }
}

pub fn mir_module_hash(mir: &MirModule) -> String {
    let bytes = encode_module(mir);
    let digest = Sha256::digest(&bytes);
    format!("{digest:x}")
}

pub fn diagnostic_codes(diags: &[SyxDiagnostic]) -> Vec<String> {
    let mut codes: Vec<String> = diags
        .iter()
        .map(|d| {
            d.code
                .clone()
                .unwrap_or_else(|| format!("msg:{}", d.message.chars().take(64).collect::<String>()))
        })
        .collect();
    codes.sort();
    codes.dedup();
    codes
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn pipeline_order_fingerprint() {
        let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));
        // Use unique call sites inside `run_parsed`.
        let expand = src
            .find("let mut module = expand_macros(module)?;")
            .expect("expand");
        let top = src
            .find("desugar_top_level_stmts(&mut module)?;")
            .expect("top");
        let meta = src.find("apply_meta_passes(&module)?;").expect("meta");
        let async_p = src
            .find("prepare_async_module(&mut module)?;")
            .expect("async");
        let sem = src
            .find("SemanticAnalyzer::analyze_collecting_with_options")
            .expect("sem");
        let mono = src
            .find("let mono = monomorphize_module(&module, &instantiations);")
            .expect("mono");
        assert!(
            expand < top && top < meta && meta < async_p && async_p < sem && sem < mono,
            "order: expand={expand} top={top} meta={meta} async={async_p} sem={sem} mono={mono}"
        );
    }

    #[test]
    fn run_arith_to_mir() {
        let source = SourceFile::new("t.sy", "func main() { print(10 + 5) }\n");
        let art = FrontendPipeline::run(source, &FrontendOptions::default()).expect("pipeline");
        assert!(art.mir.is_some());
        assert!(art.mir_hash.is_some());
        assert!(art.diagnostics.is_empty());
    }

    #[test]
    fn stop_after_semantic_has_no_mir() {
        let source = SourceFile::new("t.sy", "func main() { print(1) }\n");
        let mut opts = FrontendOptions::default();
        opts.stop_after = StopAfter::Semantic;
        let art = FrontendPipeline::run(source, &opts).expect("pipeline");
        assert!(art.mir.is_none());
        assert!(art.semantic.is_some());
    }

    #[test]
    fn stop_after_prepared_skips_semantic() {
        let source = SourceFile::new("t.sy", "func main() { print(1) }\n");
        let mut opts = FrontendOptions::default();
        opts.stop_after = StopAfter::Prepared;
        let art = FrontendPipeline::run(source, &opts).expect("pipeline");
        assert!(art.semantic.is_none());
        assert!(art.mir.is_none());
        assert!(!art.module.decls.is_empty());
    }
}
