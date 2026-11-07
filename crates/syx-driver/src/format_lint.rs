//! SYX driver format / lint face.
//! Extracted in S2-I227 (zero semantics).

use std::path::Path;
use syx_diagnostics::{SourceFile, SyxDiagnostic, SyxResult};
use syx_fmt::format_dry_run;
use syx_lint::lint_source_auto;
use syx_pkg::load_manifest;

pub fn format_project(path: &Path) -> SyxResult<()> {
    if path.is_file() {
        return format_file(path);
    }
    let root = path;
    let manifest = load_manifest(root)?;
    let entry = root.join(&manifest.main);
    format_file(&entry)
}

fn format_file(path: &Path) -> SyxResult<()> {
    if path.extension().and_then(|e| e.to_str()) != Some("sy") {
        return Err(SyxDiagnostic::error(format!(
            "format expects a .sy file or project directory, got {}",
            path.display()
        )));
    }
    let text = std::fs::read_to_string(path)
        .map_err(|e| SyxDiagnostic::error(format!("Failed to read {}: {e}", path.display())))?;
    let source = SourceFile::new(path.display().to_string(), text);
    // S2-I151 / ADR stage2-s2-04-fmt-write-migration: write shares `--check` SSOT
    // (`format_dry_run` → `format_source_auto`; CST reindent GA + fail-closed).
    let report = format_dry_run(&source)?;
    std::fs::write(path, report.formatted)
        .map_err(|e| SyxDiagnostic::error(format!("Failed to write {}: {e}", path.display())))
}

/// S2-I152 / ADR stage2-s2-04-lsp-document-formatting: buffer format for LSP
/// (`textDocument/formatting`) — same SSOT as CLI write/`--check` (`format_dry_run`).
pub fn format_document_for_lsp(name: &str, contents: &str) -> SyxResult<String> {
    let source = SourceFile::new(name, contents);
    let report = format_dry_run(&source)?;
    Ok(report.formatted)
}

/// W683 / C-06: aggregate for `syx format --check` (never writes).
#[derive(Debug, Clone, Default)]
pub struct FormatCheckReport {
    pub checked: usize,
    pub would_change: usize,
    pub failed: usize,
    pub failures: Vec<(std::path::PathBuf, String)>,
}

impl FormatCheckReport {
    pub fn ok(&self) -> bool {
        self.would_change == 0 && self.failed == 0
    }
}

pub(crate) fn collect_sy_files(path: &Path) -> SyxResult<Vec<std::path::PathBuf>> {
    if path.is_file() {
        if path.extension().and_then(|e| e.to_str()) == Some("sy") {
            return Ok(vec![path.to_path_buf()]);
        }
        return Err(SyxDiagnostic::error(format!(
            "expected a .sy file or directory, got {}",
            path.display()
        )));
    }
    if !path.is_dir() {
        return Err(SyxDiagnostic::error(format!(
            "path not found: {}",
            path.display()
        )));
    }
    let mut out = Vec::new();
    fn walk(dir: &Path, out: &mut Vec<std::path::PathBuf>) -> SyxResult<()> {
        let entries = std::fs::read_dir(dir)
            .map_err(|e| SyxDiagnostic::error(format!("Failed to read {}: {e}", dir.display())))?;
        for entry in entries {
            let entry = entry
                .map_err(|e| SyxDiagnostic::error(format!("Failed to read dir entry: {e}")))?;
            let p = entry.path();
            if p.is_dir() {
                let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                // W873: cst_comment_* no longer skipped — default CST-first preserves comments.
                if name == "target" || name == ".git" || name == ".sy" {
                    continue;
                }
                walk(&p, out)?;
            } else if p.extension().and_then(|e| e.to_str()) == Some("sy") {
                out.push(p);
            }
        }
        Ok(())
    }
    walk(path, &mut out)?;
    out.sort();
    Ok(out)
}

fn format_check_path_with(
    path: &Path,
    dry_run: fn(&SourceFile) -> SyxResult<syx_fmt::DryRunReport>,
) -> SyxResult<FormatCheckReport> {
    let files = collect_sy_files(path)?;
    let mut report = FormatCheckReport::default();
    for file in files {
        report.checked += 1;
        let text = match std::fs::read_to_string(&file) {
            Ok(t) => t,
            Err(e) => {
                report.failed += 1;
                report
                    .failures
                    .push((file.clone(), format!("read failed: {e}")));
                continue;
            }
        };
        let before = text.clone();
        let source = SourceFile::new(file.display().to_string(), text);
        match dry_run(&source) {
            Ok(dry) => {
                if dry.would_change {
                    report.would_change += 1;
                    report
                        .failures
                        .push((file, "would reformat (dry-run differs from on-disk)".into()));
                }
                // Sanity: never claim write — before bytes unchanged on disk.
                let _ = before;
            }
            Err(err) => {
                report.failed += 1;
                let code = err.code.as_deref().unwrap_or("?");
                report
                    .failures
                    .push((file, format!("{code}: {}", err.message)));
            }
        }
    }
    Ok(report)
}

/// Recursively dry-run format under `path` (file or directory). Does **not** write.
pub fn format_check_path(path: &Path) -> SyxResult<FormatCheckReport> {
    format_check_path_with(path, syx_fmt::format_dry_run)
}

/// W872 / ADR-1216: CST-only `--check` aggregate (no env).
///
/// Kept for pure-CST nails; W873 default path uses [`format_check_path`].
pub fn format_check_path_cst(path: &Path) -> SyxResult<FormatCheckReport> {
    format_check_path_with(path, syx_fmt::format_dry_run_cst)
}

/// Lint a single `.sy` file or a project directory (manifest `main` entry).
pub fn lint_project(path: &Path) -> SyxResult<Vec<syx_lint::LintIssue>> {
    if path.is_file() {
        return lint_file(path);
    }
    let root = path;
    let manifest = load_manifest(root)?;
    let entry = root.join(&manifest.main);
    lint_file(&entry)
}

fn lint_file(path: &Path) -> SyxResult<Vec<syx_lint::LintIssue>> {
    if path.extension().and_then(|e| e.to_str()) != Some("sy") {
        return Err(SyxDiagnostic::error(format!(
            "lint expects a .sy file or project directory, got {}",
            path.display()
        )));
    }
    let text = std::fs::read_to_string(path)
        .map_err(|e| SyxDiagnostic::error(format!("Failed to read {}: {e}", path.display())))?;
    let source = SourceFile::new(path.display().to_string(), text);
    // S2-I153 / ADR stage2-s2-04-lint-cst-first: CST-first policy (aligned with fmt).
    lint_source_auto(&source)
}

/// S2-I153: buffer lint for future LSP / tests — same auto policy as CLI `syx lint`.
/// Does **not** publish LSP diagnostics this window.
pub fn lint_document_for_lsp(name: &str, contents: &str) -> SyxResult<Vec<syx_lint::LintIssue>> {
    let source = SourceFile::new(name, contents);
    lint_source_auto(&source)
}
