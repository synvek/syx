#![allow(clippy::result_large_err)]

mod token;

pub use token::{keyword, InterpPart, Token, TokenKind};

use smol_str::SmolStr;
use syx_diagnostics::{SourceFile, Span, SyxDiagnostic, SyxResult};

pub struct Lexer {
    source: SourceFile,
    chars: Vec<char>,
    current: usize,
    start: usize,
    /// 1-based line/column of the next character to read (`peek`).
    line: usize,
    column: usize,
    /// Snapshot of [`Self::line`]/[`Self::column`] at token start (before first `advance`).
    token_line: usize,
    token_column: usize,
    tokens: Vec<Token>,
    last_sig: Option<TokenKind>,
}

impl Lexer {
    pub fn new(source: SourceFile) -> Self {
        let chars: Vec<char> = source.contents.chars().collect();
        Self {
            source,
            chars,
            current: 0,
            start: 0,
            line: 1,
            column: 1,
            token_line: 1,
            token_column: 1,
            tokens: Vec::new(),
            last_sig: None,
        }
    }

    pub fn from_str(name: &str, contents: &str) -> Self {
        Self::new(SourceFile::new(name, contents))
    }

    pub fn scan_all(mut self) -> SyxResult<Vec<Token>> {
        while !self.is_at_end() {
            self.begin_token();
            self.scan_token()?;
        }
        self.begin_token();
        self.add_token(TokenKind::Eof);
        Ok(self.tokens)
    }

    fn begin_token(&mut self) {
        self.start = self.current;
        self.token_line = self.line;
        self.token_column = self.column;
    }

    fn scan_token(&mut self) -> SyxResult<()> {
        let ch = self.advance();
        match ch {
            '(' => self.add_token(TokenKind::LParen),
            ')' => self.add_token(TokenKind::RParen),
            '{' => self.add_token(TokenKind::LBrace),
            '}' => self.add_token(TokenKind::RBrace),
            '[' => self.add_token(TokenKind::LBracket),
            ']' => self.add_token(TokenKind::RBracket),
            ',' => self.add_token(TokenKind::Comma),
            ';' => self.add_token(TokenKind::Semicolon),
            '@' => self.add_token(TokenKind::At),
            '#' => self.add_token(TokenKind::Hash),
            '\\' => self.add_token(TokenKind::Backslash),
            '+' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::PlusEqual);
                } else {
                    self.add_token(TokenKind::Plus);
                }
            }
            '*' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::StarEqual);
                } else {
                    self.add_token(TokenKind::Star);
                }
            }
            '%' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::PercentEqual);
                } else {
                    self.add_token(TokenKind::Percent);
                }
            }
            '!' => {
                if self.match_char('=') {
                    if self.match_char('=') {
                        self.add_token(TokenKind::NotEqualEqual);
                    } else {
                        self.add_token(TokenKind::NotEqual);
                    }
                } else {
                    self.add_token(TokenKind::Bang);
                }
            }
            '=' => {
                if self.match_char('=') {
                    if self.match_char('=') {
                        self.add_token(TokenKind::EqualEqualEqual);
                    } else {
                        self.add_token(TokenKind::EqualEqual);
                    }
                } else if self.match_char('>') {
                    self.add_token(TokenKind::FatArrow);
                } else {
                    self.add_token(TokenKind::Assign);
                }
            }
            '-' => {
                if self.match_char('>') {
                    self.add_token(TokenKind::Arrow);
                } else if self.match_char('=') {
                    self.add_token(TokenKind::MinusEqual);
                } else {
                    self.add_token(TokenKind::Minus);
                }
            }
            '/' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::SlashEqual);
                } else if self.match_char('/') {
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                    self.add_token(TokenKind::LineComment);
                } else if self.match_char('*') {
                    self.block_comment()?;
                    self.add_token(TokenKind::BlockComment);
                } else if self.can_start_regex() {
                    self.regex_literal()?;
                } else {
                    self.add_token(TokenKind::Slash);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::LessEqual);
                } else if self.match_char('<') {
                    if self.match_char('=') {
                        self.add_token(TokenKind::LessLessEqual);
                    } else {
                        self.add_token(TokenKind::LessLess);
                    }
                } else if self.match_char('.') {
                    self.add_token(TokenKind::DotDotLess);
                } else {
                    self.add_token(TokenKind::Less);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::GreaterEqual);
                } else if self.match_char('>') {
                    if self.match_char('=') {
                        self.add_token(TokenKind::GreaterGreaterEqual);
                    } else {
                        self.add_token(TokenKind::GreaterGreater);
                    }
                } else {
                    self.add_token(TokenKind::Greater);
                }
            }
            '&' => {
                if self.match_char('&') {
                    self.add_token(TokenKind::AndAnd);
                } else if self.match_char('=') {
                    self.add_token(TokenKind::AmpEqual);
                } else {
                    self.add_token(TokenKind::Amp);
                }
            }
            '|' => {
                if self.match_char('|') {
                    self.add_token(TokenKind::OrOr);
                } else if self.match_char('=') {
                    self.add_token(TokenKind::PipeEqual);
                } else {
                    self.add_token(TokenKind::Pipe);
                }
            }
            '^' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::CaretEqual);
                } else {
                    self.add_token(TokenKind::Caret);
                }
            }
            '~' => self.add_token(TokenKind::Tilde),
            '?' => {
                if self.match_char('?') {
                    self.add_token(TokenKind::NilCoalesce);
                } else {
                    self.add_token(TokenKind::Question);
                }
            }
            '.' => {
                if self.match_char('.') {
                    if self.match_char('<') {
                        self.add_token(TokenKind::DotDotLess);
                    } else if self.match_char('.') {
                        self.add_token(TokenKind::DotDotDot);
                    } else {
                        self.add_token(TokenKind::Range);
                    }
                } else {
                    self.add_token(TokenKind::Dot);
                }
            }
            ':' => self.add_token(TokenKind::Colon),
            '\'' => {
                if !is_ident_start(self.peek()) {
                    return self.error("Expected lifetime name after '\\''");
                }
                while is_ident_part(self.peek()) {
                    self.advance();
                }
                let text: String = self.chars[self.start..self.current].iter().collect();
                self.add_token(TokenKind::Lifetime(SmolStr::new(text)));
            }
            '"' => self.string()?,
            '\n' => {
                self.add_token(TokenKind::Newline);
                // `advance` already consumed `\n` and bumped column; reset for next line.
                self.line += 1;
                self.column = 1;
            }
            ' ' | '\r' | '\t' => {
                while matches!(self.peek(), ' ' | '\r' | '\t') {
                    self.advance();
                }
                self.add_token(TokenKind::Whitespace);
            }
            ch if ch.is_ascii_digit() => self.number(),
            ch if is_ident_start(ch) => self.identifier(),
            ch => return self.error(format!("Unexpected character '{ch}'")),
        }
        Ok(())
    }

    fn block_comment(&mut self) -> SyxResult<()> {
        let mut depth = 1;
        while depth > 0 && !self.is_at_end() {
            let ch = self.advance();
            if ch == '/' && self.peek() == '*' {
                self.advance();
                depth += 1;
            } else if ch == '*' && self.peek() == '/' {
                self.advance();
                depth -= 1;
            } else if ch == '\n' {
                self.line += 1;
                self.column = 1;
            }
        }
        if depth > 0 {
            return self.error("Unterminated block comment");
        }
        Ok(())
    }

    fn string(&mut self) -> SyxResult<()> {
        // Opening `"` already consumed by `scan_token`; `"""` shows two quotes ahead.
        if self.peek() == '"' && self.peek_next() == '"' {
            self.advance();
            self.advance();
            return self.multiline_string();
        }
        let mut parts: Vec<InterpPart> = Vec::new();
        let mut current = String::new();
        while !self.is_at_end() && self.peek() != '"' {
            if self.read_string_escape(&mut current, &mut parts)? {
                continue;
            }
            let ch = self.advance();
            if ch == '\n' {
                return self.error("Unterminated string literal");
            }
            current.push(ch);
        }
        if self.is_at_end() {
            return self.error("Unterminated string literal");
        }
        self.advance();
        self.finish_string(parts, current)
    }

    fn multiline_string(&mut self) -> SyxResult<()> {
        let mut parts: Vec<InterpPart> = Vec::new();
        let mut current = String::new();
        let mut closed = false;
        while !self.is_at_end() {
            if self.peek() == '"' && self.peek_next() == '"' && self.peek_at(2) == '"' {
                self.advance();
                self.advance();
                self.advance();
                closed = true;
                break;
            }
            if self.read_string_escape(&mut current, &mut parts)? {
                continue;
            }
            let ch = self.advance();
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            }
            current.push(ch);
        }
        if !closed {
            return self.error("Unterminated multiline string literal");
        }
        self.finish_string(parts, current)
    }

    fn read_string_escape(
        &mut self,
        current: &mut String,
        parts: &mut Vec<InterpPart>,
    ) -> SyxResult<bool> {
        if self.peek() != '\\' {
            return Ok(false);
        }
        self.advance();
        if self.is_at_end() {
            self.error("Unterminated string literal")?;
        }
        let escaped = self.advance();
        if escaped == '(' {
            if !current.is_empty() {
                parts.push(InterpPart::Text(std::mem::take(current)));
            }
            let mut expr = String::new();
            let mut depth = 1;
            while depth > 0 && !self.is_at_end() {
                let c = self.advance();
                if c == '(' {
                    depth += 1;
                    expr.push(c);
                } else if c == ')' {
                    depth -= 1;
                    if depth > 0 {
                        expr.push(c);
                    }
                } else if c == '"' {
                    self.error("Unterminated string interpolation")?;
                } else {
                    expr.push(c);
                }
            }
            if depth > 0 {
                self.error("Unterminated string interpolation")?;
            }
            parts.push(InterpPart::Expr(expr));
            Ok(true)
        } else {
            match escaped {
                'n' => current.push('\n'),
                't' => current.push('\t'),
                '\\' => current.push('\\'),
                '"' => current.push('"'),
                _ => {
                    current.push('\\');
                    current.push(escaped);
                }
            }
            Ok(true)
        }
    }

    fn finish_string(&mut self, mut parts: Vec<InterpPart>, current: String) -> SyxResult<()> {
        if !current.is_empty() {
            parts.push(InterpPart::Text(current));
        }
        if parts.len() == 1 {
            if let InterpPart::Text(s) = &parts[0] {
                self.add_token(TokenKind::String(s.clone()));
                return Ok(());
            }
        }
        if parts.is_empty() {
            self.add_token(TokenKind::String(String::new()));
        } else {
            self.add_token(TokenKind::InterpolatedString(parts));
        }
        Ok(())
    }

    fn regex_literal(&mut self) -> SyxResult<()> {
        let mut pattern = String::new();
        while !self.is_at_end() && self.peek() != '/' {
            let ch = self.advance();
            if ch == '\\' && !self.is_at_end() {
                pattern.push(ch);
                pattern.push(self.advance());
            } else {
                pattern.push(ch);
            }
        }
        if self.is_at_end() {
            return self.error("Unterminated regex literal");
        }
        self.advance();
        self.add_token(TokenKind::Regex(pattern));
        Ok(())
    }

    fn number(&mut self) {
        // Leading digit already consumed. `0x` / `0b` / `0o` radix prefixes (docs).
        if self.chars[self.start] == '0' {
            let base_ch = self.peek().to_ascii_lowercase();
            let radix = match base_ch {
                'x' => Some(16),
                'b' => Some(2),
                'o' => Some(8),
                _ => None,
            };
            if let Some(radix) = radix {
                self.advance();
                let digits_start = self.current;
                while is_radix_digit(self.peek(), radix) || self.peek() == '_' {
                    self.advance();
                }
                let numeric_end = self.current;
                let suffix = self.scan_numeric_suffix();
                let digits: String = self.chars[digits_start..numeric_end]
                    .iter()
                    .copied()
                    .filter(|c| *c != '_')
                    .collect();
                let value = i128::from_str_radix(&digits, radix).unwrap_or(0);
                self.add_token(TokenKind::Integer(value, suffix));
                return;
            }
        }

        while self.peek().is_ascii_digit() || self.peek() == '_' {
            self.advance();
        }
        let mut is_float = false;
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            is_float = true;
            self.advance();
            while self.peek().is_ascii_digit() || self.peek() == '_' {
                self.advance();
            }
        }
        let numeric_end = self.current;
        let suffix = self.scan_numeric_suffix();
        let numeric_text: String = self.chars[self.start..numeric_end]
            .iter()
            .copied()
            .filter(|c| *c != '_')
            .collect();
        if is_float {
            let value = numeric_text.parse::<f64>().unwrap_or(0.0);
            self.add_token(TokenKind::Double(value, suffix));
        } else if suffix.as_deref() == Some("n") {
            if let Ok(value) = numeric_text.parse::<i128>() {
                self.add_token(TokenKind::Integer(value, suffix));
            } else {
                self.add_token(TokenKind::BigIntLiteral(numeric_text));
            }
        } else {
            let value = numeric_text.parse::<i128>().unwrap_or(0);
            self.add_token(TokenKind::Integer(value, suffix));
        }
    }

    fn scan_numeric_suffix(&mut self) -> Option<SmolStr> {
        if !is_ident_start(self.peek()) {
            return None;
        }
        let suffix_start = self.current;
        while is_ident_part(self.peek()) {
            self.advance();
        }
        let suffix_text: String = self.chars[suffix_start..self.current].iter().collect();
        Some(SmolStr::new(suffix_text.to_ascii_lowercase()))
    }

    fn identifier(&mut self) {
        while is_ident_part(self.peek()) {
            self.advance();
        }
        let text: String = self.chars[self.start..self.current].iter().collect();
        let kind = keyword(&text).unwrap_or_else(|| TokenKind::Identifier(SmolStr::new(text)));
        self.add_token(kind);
    }

    fn add_token(&mut self, kind: TokenKind) {
        let lexeme: String = self.chars[self.start..self.current].iter().collect();
        let span = Span::new(self.start, self.current - self.start);
        // O(1): use snapshot from `begin_token`. Do **not** call `SourceFile::line_col`
        // per token (that was O(n) scan → O(n²) lex of large packages like Linalg).
        let (line, column) = (self.token_line, self.token_column);
        if !matches!(
            kind,
            TokenKind::Newline
                | TokenKind::Eof
                | TokenKind::LineComment
                | TokenKind::BlockComment
                | TokenKind::Whitespace
        ) {
            self.last_sig = Some(kind.clone());
        }
        self.tokens.push(Token {
            kind,
            lexeme: SmolStr::new(lexeme),
            span,
            line,
            column,
        });
    }

    fn can_start_regex(&self) -> bool {
        match &self.last_sig {
            None => true,
            Some(kind) => matches!(
                kind,
                TokenKind::LParen
                    | TokenKind::LBracket
                    | TokenKind::Comma
                    | TokenKind::Assign
                    | TokenKind::Colon
                    | TokenKind::Semicolon
                    | TokenKind::Return
                    | TokenKind::Plus
                    | TokenKind::Minus
                    | TokenKind::Star
                    | TokenKind::Slash
                    | TokenKind::Percent
                    | TokenKind::EqualEqual
                    | TokenKind::EqualEqualEqual
                    | TokenKind::NotEqual
                    | TokenKind::NotEqualEqual
                    | TokenKind::Less
                    | TokenKind::LessEqual
                    | TokenKind::Greater
                    | TokenKind::GreaterEqual
                    | TokenKind::AndAnd
                    | TokenKind::OrOr
                    | TokenKind::Arrow
                    | TokenKind::FatArrow
                    | TokenKind::In
                    | TokenKind::Try
            ),
        }
    }

    fn error(&self, message: impl Into<String>) -> SyxResult<()> {
        Err(SyxDiagnostic::error(message).with_code("L001").with_span(
            &self.source,
            Span::new(self.start, (self.current - self.start).max(1)),
            "here",
        ))
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.chars[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.chars.len() {
            '\0'
        } else {
            self.chars[self.current + 1]
        }
    }

    fn peek_at(&self, offset: usize) -> char {
        let idx = self.current + offset;
        if idx >= self.chars.len() {
            '\0'
        } else {
            self.chars[idx]
        }
    }

    fn advance(&mut self) -> char {
        let ch = self.chars[self.current];
        self.current += 1;
        // Newline handlers reset `line`/`column` after the token; for other chars
        // bump column of the *next* character to read.
        if ch != '\n' {
            self.column += 1;
        }
        ch
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.chars[self.current] != expected {
            false
        } else {
            let ch = self.chars[self.current];
            self.current += 1;
            if ch != '\n' {
                self.column += 1;
            }
            true
        }
    }
}

/// ADR-229 / M2-S2-04: UAX #31 XID_Start, plus `_`.
fn is_radix_digit(ch: char, radix: u32) -> bool {
    match radix {
        2 => matches!(ch, '0' | '1'),
        8 => matches!(ch, '0'..='7'),
        16 => ch.is_ascii_hexdigit(),
        _ => ch.is_ascii_digit(),
    }
}

fn is_ident_start(ch: char) -> bool {
    use unicode_xid::UnicodeXID;
    ch == '_' || ch.is_xid_start()
}

/// ADR-229 / M2-S2-04: UAX #31 XID_Continue, plus `_`.
fn is_ident_part(ch: char) -> bool {
    use unicode_xid::UnicodeXID;
    ch == '_' || ch.is_xid_continue()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_let_binding() {
        let tokens = Lexer::from_str("test.sy", "let x = 42")
            .scan_all()
            .unwrap();
        let kinds: Vec<_> = tokens
            .iter()
            .map(|t| &t.kind)
            .filter(|k| !matches!(k, TokenKind::Whitespace | TokenKind::Newline))
            .collect();
        assert_eq!(kinds[0], &TokenKind::Let);
        assert_eq!(kinds[1], &TokenKind::Identifier("x".into()));
        assert_eq!(kinds[2], &TokenKind::Assign);
        assert_eq!(kinds[3], &TokenKind::Integer(42, None));
    }

    #[test]
    fn lexes_keywords_and_string() {
        let tokens = Lexer::from_str("test.sy", r#"func greet() { print("hi") }"#)
            .scan_all()
            .unwrap();
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Func));
        assert!(tokens.iter().any(|t| matches!(&t.kind, TokenKind::String(s) if s == "hi")));
    }

    #[test]
    fn lexes_multiline_string_literal() {
        let src = concat!(
            "let s = \"\"\"\n",
            "line one\n",
            "line two\n",
            "\"\"\"\n",
        );
        let tokens = Lexer::from_str("test.sy", src).scan_all().unwrap();
        assert!(
            tokens.iter().any(|t| matches!(
                &t.kind,
                TokenKind::String(s) if s.contains("line one") && s.contains("line two")
            )),
            "tokens: {:?}",
            tokens.iter().map(|t| (&t.kind, &t.lexeme)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn lexes_multiline_string_interpolation() {
        let src = concat!("let s = \"\"\"\n", "hello \\(name)\n", "\"\"\"\n",);
        let tokens = Lexer::from_str("test.sy", src).scan_all().unwrap();
        let interpolated = tokens
            .iter()
            .find_map(|t| match &t.kind {
                TokenKind::InterpolatedString(parts) => Some(parts),
                _ => None,
            })
            .expect("interpolated multiline token");
        assert_eq!(interpolated.len(), 3);
        assert!(matches!(&interpolated[0], InterpPart::Text(t) if t.contains("hello")));
        assert!(matches!(&interpolated[1], InterpPart::Expr(e) if e == "name"));
    }

    #[test]
    fn lexes_closed_range_tokens() {
        let tokens = Lexer::from_str("test.sy", "1...5")
            .scan_all()
            .unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Integer(1, None));
        assert_eq!(tokens[1].kind, TokenKind::DotDotDot);
        assert_eq!(tokens[2].kind, TokenKind::Integer(5, None));
    }

    #[test]
    fn lexes_bitwise_operator_tokens() {
        let tokens = Lexer::from_str("test.sy", "a & b | c ^ d << e >> f ~g")
            .scan_all()
            .unwrap();
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(kinds.contains(&&TokenKind::Amp));
        assert!(kinds.contains(&&TokenKind::Pipe));
        assert!(kinds.contains(&&TokenKind::Caret));
        assert!(kinds.contains(&&TokenKind::LessLess));
        assert!(kinds.contains(&&TokenKind::GreaterGreater));
        assert!(kinds.contains(&&TokenKind::Tilde));
    }

    #[test]
    fn lexes_radix_integer_literals() {
        let tokens = Lexer::from_str("test.sy", "0b1100 0b1010 0xFF 0o17 1_000")
            .scan_all()
            .unwrap();
        let ints: Vec<i128> = tokens
            .iter()
            .filter_map(|t| match t.kind {
                TokenKind::Integer(v, _) => Some(v),
                _ => None,
            })
            .collect();
        assert_eq!(ints, vec![12, 10, 255, 15, 1000]);
    }

    #[test]
    fn lexes_lifetime_on_reference() {
        let tokens = Lexer::from_str("test.sy", "func show(x: &'a Int) {}")
            .scan_all()
            .unwrap();
        assert!(
            tokens
                .iter()
                .any(|t| matches!(&t.kind, TokenKind::Lifetime(s) if s == "'a")),
            "tokens: {:?}",
            tokens.iter().map(|t| (&t.kind, &t.lexeme)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn lexes_line_and_block_comments() {
        let tokens = Lexer::from_str("test.sy", "// ahead\nfunc main() {}\n/* trail */")
            .scan_all()
            .unwrap();
        assert!(tokens.iter().any(|t| t.kind == TokenKind::LineComment));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::BlockComment));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Func));
        let line = tokens
            .iter()
            .find(|t| t.kind == TokenKind::LineComment)
            .unwrap();
        assert!(line.lexeme.as_str().starts_with("//"));
    }

    #[test]
    fn lexes_whitespace_runs() {
        let tokens = Lexer::from_str("test.sy", "func  main")
            .scan_all()
            .unwrap();
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Whitespace));
    }

    #[test]
    fn lexes_unicode_xid_greek_ident() {
        let tokens = Lexer::from_str("test.sy", "let α = 1")
            .scan_all()
            .unwrap();
        assert!(
            tokens
                .iter()
                .any(|t| matches!(&t.kind, TokenKind::Identifier(s) if s == "α")),
            "tokens: {:?}",
            tokens.iter().map(|t| &t.kind).collect::<Vec<_>>()
        );
    }

    #[test]
    fn rejects_emoji_as_ident_start() {
        let err = Lexer::from_str("test.sy", "let 😀 = 1")
            .scan_all()
            .expect_err("emoji must not be XID_Start");
        let msg = format!("{err:?}");
        assert!(
            msg.contains("Unexpected") || msg.contains("😀"),
            "unexpected diagnostic: {msg}"
        );
    }
}
