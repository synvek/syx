//! SYX driver doc_project face.
//! Extracted in S2-I233 (zero semantics).

use crate::{collect_sy_files, driver_error_at};
use std::path::Path;
use syx_diagnostics::{SourceFile, Span, SyxResult};

/// Generate API docs for a `.sy` file or a directory of sources.
///
/// Directories are scanned for `.sy` files (same walk as `syx format --check`),
/// so package roots with `mod.sy` (and no `main.sy` / `syx.toml`) work.
pub fn doc_project(path: &Path) -> SyxResult<String> {
    let files = collect_sy_files(path)?;
    if files.is_empty() {
        return Err(driver_error_at(
            format!("No .sy source files found under {}", path.display()),
            None,
            Span::DUMMY,
            "D003",
        ));
    }
    let mut sources = Vec::with_capacity(files.len());
    for file in &files {
        let text = std::fs::read_to_string(file).map_err(|e| {
            driver_error_at(
                format!("Failed to read {}: {e}", file.display()),
                None,
                Span::DUMMY,
                "D001",
            )
        })?;
        sources.push(SourceFile::new(file.display().to_string(), text));
    }
    syx_doc::generate_markdown_sources(&sources)
}
