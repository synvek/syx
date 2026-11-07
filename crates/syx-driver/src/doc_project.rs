//! SYX driver doc_project face.
//! Extracted in S2-I233 (zero semantics).

use crate::collect_sy_files;
use std::path::Path;
use syx_diagnostics::{SourceFile, SyxDiagnostic, SyxResult};

/// Generate API docs for a `.sy` file or a directory of sources.
///
/// Directories are scanned for `.sy` files (same walk as `syx format --check`),
/// so package roots with `mod.sy` (and no `main.sy` / `syx.toml`) work.
pub fn doc_project(path: &Path) -> SyxResult<String> {
    let files = collect_sy_files(path)?;
    if files.is_empty() {
        return Err(SyxDiagnostic::error(format!(
            "No .sy source files found under {}",
            path.display()
        )));
    }
    let mut sources = Vec::with_capacity(files.len());
    for file in &files {
        let text = std::fs::read_to_string(file).map_err(|e| {
            SyxDiagnostic::error(format!("Failed to read {}: {e}", file.display()))
        })?;
        sources.push(SourceFile::new(file.display().to_string(), text));
    }
    syx_doc::generate_markdown_sources(&sources)
}
