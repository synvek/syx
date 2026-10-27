use miette::{Diagnostic, LabeledSpan, NamedSource, SourceCode, SourceOffset, SourceSpan};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub name: String,
    pub contents: Arc<str>,
    /// Byte offsets of each line start (line 1 at index 0). Built once in [`Self::new`].
    line_starts: Arc<[usize]>,
}

impl PartialEq for SourceFile {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.contents == other.contents
    }
}

impl Eq for SourceFile {}

impl Hash for SourceFile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.contents.hash(state);
    }
}

impl SourceFile {
    pub fn new(name: impl Into<String>, contents: impl Into<Arc<str>>) -> Self {
        let contents: Arc<str> = contents.into();
        let line_starts = Arc::from(compute_line_starts(&contents));
        Self {
            name: name.into(),
            contents,
            line_starts,
        }
    }

    pub fn line_col(&self, offset: usize) -> (usize, usize) {
        line_col_with_starts(&self.contents, &self.line_starts, offset)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: u32,
    pub len: u32,
}

impl Span {
    pub const DUMMY: Self = Self { start: 0, len: 0 };

    pub fn new(start: usize, len: usize) -> Self {
        Self {
            start: start as u32,
            len: len as u32,
        }
    }

    pub fn merge(a: Span, b: Span) -> Span {
        let start = a.start.min(b.start);
        let end = (a.start + a.len).max(b.start + b.len);
        Span {
            start,
            len: end - start,
        }
    }

    pub fn end(&self) -> usize {
        self.start as usize + self.len as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Note,
}

#[derive(Debug, Clone)]
pub struct SyxDiagnostic {
    pub message: String,
    pub severity: Severity,
    pub code: Option<String>,
    pub source: Option<NamedSource<String>>,
    pub primary_span: Option<SourceSpan>,
}

impl fmt::Display for SyxDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some((file, line, col)) = self.location() {
            write!(f, "{file}:{line}:{col}: ")?;
        }
        if let Some(code) = &self.code {
            write!(f, "[{code}] ")?;
        }
        write!(f, "{}", self.message)
    }
}

impl Diagnostic for SyxDiagnostic {
    fn code<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        self.code
            .as_ref()
            .map(|c| Box::new(c.as_str()) as Box<dyn fmt::Display>)
    }

    fn severity(&self) -> Option<miette::Severity> {
        Some(match self.severity {
            Severity::Error => miette::Severity::Error,
            Severity::Warning => miette::Severity::Warning,
            Severity::Note => miette::Severity::Advice,
        })
    }

    fn source_code(&self) -> Option<&dyn SourceCode> {
        self.source
            .as_ref()
            .map(|s| s as &dyn SourceCode)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        let span = self.primary_span?;
        Some(Box::new(std::iter::once(LabeledSpan::new_with_span(
            None::<String>,
            span,
        ))))
    }
}

impl std::error::Error for SyxDiagnostic {}

impl SyxDiagnostic {
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            severity: Severity::Error,
            code: None,
            source: None,
            primary_span: None,
        }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            severity: Severity::Warning,
            code: None,
            source: None,
            primary_span: None,
        }
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    pub fn with_span(mut self, file: &SourceFile, span: Span, _label: impl Into<String>) -> Self {
        self.source = Some(NamedSource::new(file.name.clone(), file.contents.to_string()));
        self.primary_span = Some(SourceSpan::new(
            SourceOffset::from(span.start as usize),
            span.len as usize,
        ));
        self
    }

    /// 1-based line/column when a source span is attached.
    pub fn location(&self) -> Option<(String, usize, usize)> {
        let named = self.source.as_ref()?;
        let span = self.primary_span.as_ref()?;
        let (line, col) = line_col_in_str(named.inner(), span.offset());
        Some((named.name().to_string(), line, col))
    }
}

fn compute_line_starts(contents: &str) -> Vec<usize> {
    let mut starts = Vec::with_capacity(contents.len() / 32 + 1);
    starts.push(0);
    for (idx, ch) in contents.char_indices() {
        if ch == '\n' {
            starts.push(idx + ch.len_utf8());
        }
    }
    starts
}

fn line_col_with_starts(contents: &str, line_starts: &[usize], offset: usize) -> (usize, usize) {
    let offset = offset.min(contents.len());
    let line_idx = match line_starts.binary_search(&offset) {
        Ok(i) => i,
        Err(i) => i.saturating_sub(1),
    };
    let line_start = line_starts.get(line_idx).copied().unwrap_or(0);
    let col = contents
        .get(line_start..offset)
        .map(|s| s.chars().count())
        .unwrap_or(0)
        + 1;
    (line_idx + 1, col)
}

fn line_col_in_str(contents: &str, offset: usize) -> (usize, usize) {
    let starts = compute_line_starts(contents);
    line_col_with_starts(contents, &starts, offset)
}

pub type SyxResult<T> = Result<T, SyxDiagnostic>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostic_location_in_display() {
        let file = SourceFile::new("demo.sy", "func main() {\n    join(t)\n}");
        let join_offset = file.contents.find("join").unwrap();
        let diag = SyxDiagnostic::error("Undefined function 'join'")
            .with_code("S002")
            .with_span(&file, Span::new(join_offset, 4), "");
        assert_eq!(diag.location(), Some(("demo.sy".into(), 2, 5)));
        assert!(format!("{diag}").contains("demo.sy:2:5:"));
    }

    #[test]
    fn line_col_matches_naive_scan_on_multiline() {
        let src = "a\nbb\nccc\n";
        let file = SourceFile::new("t.sy", src);
        for (offset, expect) in [
            (0, (1, 1)),
            (1, (1, 2)),
            (2, (2, 1)),
            (4, (2, 3)),
            (5, (3, 1)),
            (8, (3, 4)),
            (9, (4, 1)),
        ] {
            assert_eq!(file.line_col(offset), expect, "offset {offset}");
            assert_eq!(line_col_in_str(src, offset), expect);
        }
    }
}
