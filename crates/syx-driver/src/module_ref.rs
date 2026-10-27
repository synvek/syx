use std::path::{Path, PathBuf};

use syx_diagnostics::{SourceFile, SyxDiagnostic, SyxResult};

use crate::embedded_std;

#[derive(Debug, Clone)]
pub enum ModuleRef {
    File(PathBuf),
    Embedded {
        key: PathBuf,
        display_name: String,
        source: &'static str,
    },
}

impl ModuleRef {
    pub fn from_file(path: PathBuf) -> Self {
        Self::File(path)
    }

    pub fn key(&self) -> PathBuf {
        match self {
            Self::File(path) => path.canonicalize().unwrap_or_else(|_| path.clone()),
            Self::Embedded { key, .. } => key.clone(),
        }
    }

    pub fn load(&self) -> SyxResult<(PathBuf, SourceFile)> {
        match self {
            Self::File(path) => {
                let text = std::fs::read_to_string(path).map_err(|e| {
                    SyxDiagnostic::error(format!("Failed to read {}: {e}", path.display()))
                })?;
                let key = path.canonicalize().unwrap_or_else(|_| path.clone());
                Ok((key, SourceFile::new(path.display().to_string(), text)))
            }
            Self::Embedded {
                key,
                display_name,
                source,
            } => Ok((
                key.clone(),
                SourceFile::new(display_name.clone(), (*source).to_string()),
            )),
        }
    }

    pub fn importer_path<'a>(&'a self, fallback: &'a Path) -> &'a Path {
        match self {
            Self::File(path) => path.as_path(),
            Self::Embedded { .. } => fallback,
        }
    }
}

pub fn embedded_module(path_str: &str) -> Option<ModuleRef> {
    let first = path_str.split('.').next().unwrap_or(path_str);
    if embedded_std::is_forbidden_import(first) {
        return None;
    }
    embedded_std::lookup(path_str).map(|(name, source)| ModuleRef::Embedded {
        key: embedded_std::virtual_path(name),
        display_name: format!("<stdlib>/{name}.sy"),
        source,
    })
}
