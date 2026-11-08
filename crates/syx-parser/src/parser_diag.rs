//! parser-local diagnostic constructors (avoid bare `SyxDiagnostic::error` / `warning`).

use syx_diagnostics::{Severity, SourceFile, Span, SyxDiagnostic};

pub(crate) fn parser_error_at(
    message: impl Into<String>,
    source: Option<&SourceFile>,
    span: Span,
    code: &str,
) -> SyxDiagnostic {
    let mut diag = SyxDiagnostic {
        message: message.into(),
        severity: Severity::Error,
        code: Some(code.to_string()),
        source: None,
        primary_span: None,
        label: None,
        help: None,
    };
    if span != Span::DUMMY {
        if let Some(src) = source {
            diag = diag.with_span(src, span, "");
        }
    }
    diag
}

/// Convenience when no SourceFile / use DUMMY span.
#[allow(dead_code)]
pub(crate) fn parser_error(message: impl Into<String>, code: &str) -> SyxDiagnostic {
    parser_error_at(message, None, Span::DUMMY, code)
}

#[allow(dead_code)]
pub(crate) fn parser_warning_at(
    message: impl Into<String>,
    source: Option<&SourceFile>,
    span: Span,
    code: &str,
) -> SyxDiagnostic {
    let mut diag = SyxDiagnostic {
        message: message.into(),
        severity: Severity::Warning,
        code: Some(code.to_string()),
        source: None,
        primary_span: None,
        label: None,
        help: None,
    };
    if span != Span::DUMMY {
        if let Some(src) = source {
            diag = diag.with_span(src, span, "");
        }
    }
    diag
}

/// Convenience when no SourceFile / use DUMMY span.
#[allow(dead_code)]
pub(crate) fn parser_warning(message: impl Into<String>, code: &str) -> SyxDiagnostic {
    parser_warning_at(message, None, Span::DUMMY, code)
}
