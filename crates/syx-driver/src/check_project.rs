//! SYX driver check_project / check_file face.
//! Extracted in S2-I225 (zero semantics).

use crate::{
    compile_file_with_options, compile_project_with_options, DriverOptions, ModuleUnit,
};
use indexmap::IndexMap;
use std::path::Path;
use syx_diagnostics::{SourceFile, SyxDiagnostic, SyxResult};
use syx_frontend::expand_macros;
use syx_parser::{Parser, MAX_COLLECTED_PARSE_DIAGNOSTICS};
use syx_semantic::SemanticAnalyzer;

pub fn check_project(root: &Path) -> Result<(), Vec<SyxDiagnostic>> {
    check_project_with_options(root, &DriverOptions::default())
}

/// Project `syx check`: aggregate capped parse multi-diag across package `.sy` files, then
/// linked compile if parse-clean. Build / run stay first-error. Semantic multi-diag remains
/// single-file (`check_file_with_options`).
pub fn check_project_with_options(
    root: &Path,
    options: &DriverOptions,
) -> Result<(), Vec<SyxDiagnostic>> {
    let mut diags = Vec::new();
    collect_project_parse_diagnostics(root, &mut diags).map_err(|e| vec![e])?;
    if !diags.is_empty() {
        return Err(diags);
    }
    compile_project_with_options(root, options).map(|_| ()).map_err(|e| vec![e])
}

/// Walk package `.sy` files (same skip rules as service scan) and append parse_collecting diags.
fn collect_project_parse_diagnostics(
    dir: &Path,
    out: &mut Vec<SyxDiagnostic>,
) -> SyxResult<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)
        .map_err(|e| SyxDiagnostic::error(format!("Failed to read {}: {e}", dir.display())))?
    {
        if out.len() >= MAX_COLLECTED_PARSE_DIAGNOSTICS {
            break;
        }
        let entry =
            entry.map_err(|e| SyxDiagnostic::error(format!("Failed to read entry: {e}")))?;
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|n| {
                n == "tests" || n == "target" || n == ".sy-cache" || n == "node_modules"
            }) {
                continue;
            }
            if path.join("syx.toml").is_file() {
                continue;
            }
            collect_project_parse_diagnostics(&path, out)?;
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("sy") {
            continue;
        }
        let text = std::fs::read_to_string(&path).map_err(|e| {
            SyxDiagnostic::error(format!("Failed to read {}: {e}", path.display()))
        })?;
        let source = SourceFile::new(path.display().to_string(), text);
        let outcome = Parser::parse_collecting(source);
        for err in outcome.errors {
            if out.len() >= MAX_COLLECTED_PARSE_DIAGNOSTICS {
                break;
            }
            out.push(err);
        }
    }
    Ok(())
}

/// Single-file `syx check`: capped parse multi-diag; on linked compile failure, enrich with
/// capped semantic collecting when it yields ≥2 real diagnostics.
///
/// Linked compile remains authoritative for success (imports / desugar / prelude).
/// Thin collecting skips `S045` (raw top-level stmts before desugar) so book `error=`
/// tags and import-aware failures keep the compile first-error when multi-diag is weak.
/// Project / build / run stay first-error.
pub fn check_file_with_options(
    path: &Path,
    options: &DriverOptions,
) -> Result<ModuleUnit, Vec<SyxDiagnostic>> {
    let text = std::fs::read_to_string(path).map_err(|e| {
        vec![SyxDiagnostic::error(format!(
            "Failed to read {}: {e}",
            path.display()
        ))]
    })?;
    let source = SourceFile::new(path.display().to_string(), text);
    let outcome = Parser::parse_collecting(source.clone());
    if !outcome.errors.is_empty() {
        return Err(outcome.errors);
    }
    match compile_file_with_options(path, options) {
        Ok(unit) => Ok(unit),
        Err(compile_err) => {
            let module = match expand_macros(outcome.module) {
                Ok(m) => m,
                Err(_) => return Err(vec![compile_err]),
            };
            let analyzer = SemanticAnalyzer::analyze_collecting_with_options(
                &module,
                &[],
                false,
                None,
                &[],
                Some(&source),
                &IndexMap::new(),
                false,
            );
            // Raw AST may flag top-level stmts as S045 before pipeline desugar — ignore those.
            let useful: Vec<_> = analyzer
                .errors
                .into_iter()
                .filter(|e| e.code.as_deref() != Some("S045"))
                .collect();
            if useful.len() >= 2 {
                Err(useful)
            } else {
                Err(vec![compile_err])
            }
        }
    }
}
