use smol_str::SmolStr;
use syx_ast::{
    AccessLevel, Attribute, AttributeArg, BinOp, CallArg, ClosureBody, CompoundAssignOp, Decl, Expr,
    Literal,
    Module, Param, ParamLabel, Stmt, TypeExpr, UnaryOp, WherePred,
};
use syx_diagnostics::{SourceFile, Span, SyxDiagnostic, SyxResult};
use syx_lexer::{InterpPart, Lexer, Token, TokenKind};

#[derive(Clone)]
enum SelectCaseKind {
    Recv { binding: SmolStr, channel: Expr },
    Send { channel: Expr, value: Expr },
}

#[derive(Clone)]
struct SelectCase {
    kind: SelectCaseKind,
    body: Vec<Stmt>,
}

/// Phase 108: mixed send+recv select lowers to one tagged `__syx_channel_select` array poll.
/// Tagged arm convention: `[dir, channel, payload?]` with dir 0=recv, 1=send.
pub const SELECT_MIXED_UNIFIED_FINGERPRINT: &str = "select-mixed-unified-v1:tagged-array-poll";

/// Phase 109: mixed select case bodies may interleave `await` (hard gate).
pub const SELECT_MIXED_AWAIT_FINGERPRINT: &str = "select-mixed-await-v1:interleave";

/// Phase 110: mixed+await select poll must honor cancel or deadline timeout.
pub const SELECT_MIXED_TIMEOUT_FINGERPRINT: &str = "select-mixed-timeout-v1:cancel-or-deadline";

/// Phase 111: language-level `timeout(ms):` / `deadline ms:` select arms (not env-only GA).
pub const SELECT_TIMEOUT_SYNTAX_FINGERPRINT: &str = "select-timeout-syntax-v1:lang-arm";

/// Phase 112 (historical): mixed+await unified with language timeout/deadline ms.
pub const SELECT_DEADLINE_UNIFY_FINGERPRINT: &str = "select-deadline-unify-v1:lang-over-env";

/// Phase 113: mixed+await requires syntax timeout/deadline; env fallback hard-banned.
pub const SELECT_DEADLINE_STRICT_FINGERPRINT: &str = "select-deadline-strict-v1:no-env";

/// Legacy alias name kept for call sites; value is the Phase 108 unified fingerprint.
pub const SELECT_MIXED_POLL_FINGERPRINT: &str = SELECT_MIXED_UNIFIED_FINGERPRINT;

pub fn select_mixed_poll_fingerprint() -> &'static str {
    SELECT_MIXED_UNIFIED_FINGERPRINT
}

pub fn select_mixed_unified_fingerprint() -> &'static str {
    SELECT_MIXED_UNIFIED_FINGERPRINT
}

pub fn select_mixed_await_fingerprint() -> &'static str {
    SELECT_MIXED_AWAIT_FINGERPRINT
}

pub fn select_mixed_timeout_fingerprint() -> &'static str {
    SELECT_MIXED_TIMEOUT_FINGERPRINT
}

pub fn select_timeout_syntax_fingerprint() -> &'static str {
    SELECT_TIMEOUT_SYNTAX_FINGERPRINT
}

pub fn select_deadline_unify_fingerprint() -> &'static str {
    SELECT_DEADLINE_UNIFY_FINGERPRINT
}

pub fn select_deadline_strict_fingerprint() -> &'static str {
    SELECT_DEADLINE_STRICT_FINGERPRINT
}

/// Cap on syntax diagnostics retained in collecting mode (LSP).
pub const MAX_COLLECTED_PARSE_DIAGNOSTICS: usize = 64;

/// Module plus all syntax diagnostics gathered in one parse (panic-mode recovery).
pub struct ParseOutcome {
    pub module: Module,
    pub errors: Vec<SyxDiagnostic>,
}

pub struct Parser {
    source: SourceFile,
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<SyxDiagnostic>,
    /// Extra decls synthesized while parsing one logical decl (enum methods → Extension, etc.).
    pending_decls: Vec<Decl>,
    /// S1-I7: disable trailing `{ … }` after Call so `for x in Counter() { … }` keeps the brace as body.
    allow_trailing_closure: bool,
}

fn is_known_protocol_name(name: &str) -> bool {
    matches!(
        name,
        "Equatable"
            | "Hashable"
            | "Codable"
            | "Encodable"
            | "Decodable"
            | "Encoder"
            | "Decoder"
            | "CustomStringConvertible"
            | "Iterator"
            | "Sequence"
            | "Sendable"
            | "Comparable"
            | "Reader"
            | "Writer"
            | "Seekable"
            | "Closeable"
    )
}

impl Parser {
    pub fn new(source: SourceFile, tokens: Vec<Token>) -> Self {
        Self {
            source,
            tokens,
            current: 0,
            errors: Vec::new(),
            pending_decls: Vec::new(),
            allow_trailing_closure: true,
        }
    }

    fn take_pending_decls(&mut self) -> Vec<Decl> {
        std::mem::take(&mut self.pending_decls)
    }

    pub fn parse(source: SourceFile) -> SyxResult<Module> {
        let tokens = Lexer::new(source.clone()).scan_all()?;
        let mut parser = Self::new(source, tokens);
        parser.parse_module()
    }

    /// Parse with panic-mode recovery: keep going after syntax errors (capped).
    /// Used by LSP so the editor can highlight multiple P001s at once.
    pub fn parse_collecting(source: SourceFile) -> ParseOutcome {
        let tokens = match Lexer::new(source.clone()).scan_all() {
            Ok(t) => t,
            Err(e) => {
                return ParseOutcome {
                    module: Module {
                        name: SmolStr::new("main"),
                        decls: vec![],
                    },
                    errors: vec![e],
                };
            }
        };
        let mut parser = Self::new(source, tokens);
        let module = parser.parse_module_collecting();
        ParseOutcome {
            module,
            errors: parser.errors,
        }
    }

    fn push_parse_diagnostic(&mut self, diag: SyxDiagnostic) {
        if self.errors.len() < MAX_COLLECTED_PARSE_DIAGNOSTICS {
            self.errors.push(diag);
        }
    }

    fn parse_module(&mut self) -> SyxResult<Module> {
        let mut decls = Vec::new();
        while !self.is_at_end() {
            self.skip_newlines();
            if self.is_at_end() {
                break;
            }
            if self.is_decl_start() {
                decls.push(self.parse_decl()?);
                decls.append(&mut self.take_pending_decls());
            } else {
                let stmt = self.parse_stmt()?;
                decls.push(Decl::TopLevelStmt {
                    span: stmt_span(&stmt),
                    stmt,
                });
                decls.append(&mut self.take_pending_decls());
            }
        }
        Ok(Module {
            name: SmolStr::new("main"),
            decls,
        })
    }

    fn parse_module_collecting(&mut self) -> Module {
        let mut decls = Vec::new();
        while !self.is_at_end() {
            if self.errors.len() >= MAX_COLLECTED_PARSE_DIAGNOSTICS {
                break;
            }
            self.skip_newlines();
            if self.is_at_end() {
                break;
            }
            let before = self.current;
            let parsed = if self.is_decl_start() {
                self.parse_decl()
            } else {
                self.parse_stmt().map(|stmt| Decl::TopLevelStmt {
                    span: stmt_span(&stmt),
                    stmt,
                })
            };
            match parsed {
                Ok(d) => {
                    decls.push(d);
                    decls.append(&mut self.take_pending_decls());
                }
                Err(e) => {
                    self.pending_decls.clear();
                    self.push_parse_diagnostic(e);
                    if self.is_decl_start() {
                        // Already on the next declaration (e.g. missing `{` before next `func`).
                        // Only skip if this attempt made no progress (avoid infinite loop).
                        if self.current == before {
                            if !self.is_at_end() {
                                self.advance();
                            }
                            self.synchronize_to_decl();
                        }
                    } else {
                        self.synchronize_to_decl();
                        if self.current == before && !self.is_at_end() {
                            self.advance();
                            self.synchronize_to_decl();
                        }
                    }
                }
            }
        }
        Module {
            name: SmolStr::new("main"),
            decls,
        }
    }

    /// Skip tokens until the next top-level declaration keyword (or EOF).
    fn synchronize_to_decl(&mut self) {
        while !self.is_at_end() {
            if self.is_decl_start() {
                return;
            }
            if matches!(self.peek().kind, TokenKind::RBrace) {
                self.advance();
                self.skip_newlines();
                if self.is_decl_start() {
                    return;
                }
                continue;
            }
            self.advance();
        }
    }

    fn is_decl_start(&self) -> bool {
        matches!(
            self.peek().kind,
            TokenKind::At
                | TokenKind::Import
                | TokenKind::Export
                | TokenKind::Public
                | TokenKind::Internal
                | TokenKind::Private
                | TokenKind::FilePrivate
                | TokenKind::Func
                | TokenKind::Async
                | TokenKind::Class
                | TokenKind::Data
                | TokenKind::Final
                | TokenKind::Extension
                | TokenKind::Global
                | TokenKind::Actor
                | TokenKind::Struct
                | TokenKind::Protocol
                | TokenKind::Enum
                | TokenKind::Var
                | TokenKind::Let
                | TokenKind::Typealias
                | TokenKind::Hash
                | TokenKind::Macro
        )
    }

    fn parse_decl(&mut self) -> SyxResult<Decl> {
        let attrs = self.parse_attributes()?;
        if !attrs.is_empty() {
            return match &self.peek().kind {
                TokenKind::Func => self.parse_function(AccessLevel::Internal, false, attrs),
                TokenKind::Async => {
                    self.advance();
                    self.consume(TokenKind::Func, "Expected 'func' after async")?;
                    self.parse_function(AccessLevel::Internal, true, attrs)
                }
                TokenKind::Data => {
                    self.advance();
                    self.consume(TokenKind::Class, "Expected 'class' after data")?;
                    self.parse_data_class(AccessLevel::Internal, attrs)
                }
                TokenKind::Class => self.parse_class(AccessLevel::Internal, attrs),
                // S1-I9: `@resultBuilder struct ArrayBuilder { … }`
                TokenKind::Struct => self.parse_struct(AccessLevel::Internal, attrs),
                TokenKind::Enum => self.parse_enum(AccessLevel::Internal, attrs),
                TokenKind::Macro => self.parse_macro_decl(AccessLevel::Internal),
                TokenKind::Hash => self.parse_user_macro_decl_with_attrs(attrs),
                TokenKind::Public | TokenKind::Internal | TokenKind::Private | TokenKind::FilePrivate => {
                    let access = self.parse_access();
                    match &self.peek().kind {
                        TokenKind::Class => self.parse_class(access, attrs),
                        TokenKind::Struct => self.parse_struct(access, attrs),
                        TokenKind::Protocol => self.parse_protocol(access),
                        _ => self.error("Expected 'class' or 'protocol' after attribute and access modifier"),
                    }
                }
                _ => self.error("Expected 'func' or 'data class' after attribute"),
            };
        }
        match &self.peek().kind {
            TokenKind::Import => self.parse_import(),
            TokenKind::Export => self.parse_export_decl(),
            TokenKind::Public | TokenKind::Internal | TokenKind::Private | TokenKind::FilePrivate => {
                let access = self.parse_access();
                self.parse_decl_with_access(access)
            }
            TokenKind::Func => self.parse_function(AccessLevel::Internal, false, vec![]),
            TokenKind::Async => {
                self.advance();
                self.consume(TokenKind::Func, "Expected 'func' after async")?;
                self.parse_function(AccessLevel::Internal, true, vec![])
            }
            TokenKind::Class => self.parse_class(AccessLevel::Internal, vec![]),
            TokenKind::Data => {
                self.advance();
                self.consume(TokenKind::Class, "Expected 'class' after data")?;
                self.parse_data_class(AccessLevel::Internal, vec![])
            }
            TokenKind::Final => {
                self.advance();
                self.consume(TokenKind::Class, "Expected 'class' after final")?;
                self.parse_final_class(AccessLevel::Internal)
            }
            TokenKind::Extension => self.parse_extension(),
            TokenKind::Global => {
                self.advance();
                self.consume(TokenKind::Actor, "Expected 'actor' after global")?;
                self.parse_global_actor(AccessLevel::Internal)
            }
            TokenKind::Actor => self.parse_actor(AccessLevel::Internal),
            TokenKind::Struct => self.parse_struct(AccessLevel::Internal, vec![]),
            TokenKind::Protocol => self.parse_protocol(AccessLevel::Internal),
            TokenKind::Enum => self.parse_enum(AccessLevel::Internal, vec![]),
            TokenKind::Var | TokenKind::Let => self.parse_global_var(AccessLevel::Internal, vec![], false, false),
            TokenKind::Typealias if syx_semantic::typealias_parser_enabled() => {
                self.parse_typealias(AccessLevel::Internal)
            }
            TokenKind::Hash => self.parse_user_macro_decl_with_attrs(vec![]),
            TokenKind::Macro => self.parse_macro_decl(AccessLevel::Internal),
            TokenKind::Init => self.error("init is only allowed inside class or actor"),
            TokenKind::Deinit => self.error("deinit is only allowed inside class or actor"),
            _ => self.error("Expected declaration"),
        }
    }

    /// W480 / ADR-824: `export func …` → `Decl::Export`.
    /// W483 / ADR-827: `export { Name, … }` → `Decl::ExportList`.
    fn parse_export_decl(&mut self) -> SyxResult<Decl> {
        let start = self.advance().span;
        if self.check(TokenKind::LBrace) {
            return self.parse_export_list(start);
        }
        let item = match &self.peek().kind {
            TokenKind::Func => self.parse_function(AccessLevel::Internal, false, vec![])?,
            TokenKind::Async => {
                self.advance();
                self.consume(TokenKind::Func, "Expected 'func' after async")?;
                self.parse_function(AccessLevel::Internal, true, vec![])?
            }
            _ => {
                return self.error("Expected 'func' or '{' after 'export'");
            }
        };
        let item_span = match &item {
            Decl::Function { span, .. } => *span,
            _ => start,
        };
        Ok(Decl::Export {
            span: Span::merge(start, item_span),
            item: Box::new(item),
        })
    }

    /// W483 / ADR-827: `export { Ident ("," Ident)* }`.
    /// W486 / ADR-830: optional trailing comma allowed; empty list still rejected.
    fn parse_export_list(&mut self, start: Span) -> SyxResult<Decl> {
        self.consume(TokenKind::LBrace, "Expected '{' after 'export'")?;
        let mut names = Vec::new();
        if !self.check(TokenKind::RBrace) {
            loop {
                names.push(self.consume_identifier("Expected export name")?);
                if !self.match_kind(TokenKind::Comma) {
                    break;
                }
                // Wave A W486 / ADR-830: trailing comma before `}` is accepted.
                if self.check(TokenKind::RBrace) {
                    break;
                }
            }
        }
        let end = self.consume(TokenKind::RBrace, "Expected '}' after export list")?.span;
        if names.is_empty() {
            return self.error("export list must contain at least one name");
        }
        Ok(Decl::ExportList {
            span: Span::merge(start, end),
            names,
        })
    }

    fn parse_decl_with_access(&mut self, access: AccessLevel) -> SyxResult<Decl> {
        match &self.peek().kind {
            TokenKind::Import => self.parse_import(),
            TokenKind::Protocol => self.parse_protocol(access),
            TokenKind::Enum => self.parse_enum(access, vec![]),
            TokenKind::Func => {
                let attrs = self.parse_attributes()?;
                self.parse_function(access, false, attrs)
            }
            TokenKind::Async => {
                let attrs = self.parse_attributes()?;
                self.advance();
                self.consume(TokenKind::Func, "Expected 'func' after async")?;
                self.parse_function(access, true, attrs)
            }
            TokenKind::Class => {
                let attrs = self.parse_attributes()?;
                self.parse_class(access, attrs)
            }
            TokenKind::Data => {
                self.advance();
                self.consume(TokenKind::Class, "Expected 'class' after data")?;
                self.parse_data_class(access, vec![])
            }
            TokenKind::Final => {
                self.advance();
                self.consume(TokenKind::Class, "Expected 'class' after final")?;
                self.parse_final_class(access)
            }
            TokenKind::Extension => self.parse_extension(),
            TokenKind::Global => {
                self.advance();
                self.consume(TokenKind::Actor, "Expected 'actor' after global")?;
                self.parse_global_actor(access)
            }
            TokenKind::Actor => self.parse_actor(access),
            TokenKind::Struct => self.parse_struct(access, vec![]),
            TokenKind::Var | TokenKind::Let => self.parse_global_var(access, vec![], false, false),
            TokenKind::Typealias if syx_semantic::typealias_parser_enabled() => {
                self.parse_typealias(access)
            }
            _ => self.error("Expected declaration after access modifier"),
        }
    }

    fn parse_access(&mut self) -> AccessLevel {
        match self.advance().kind {
            TokenKind::Public => AccessLevel::Public,
            TokenKind::Private => AccessLevel::Private,
            TokenKind::FilePrivate => AccessLevel::FilePrivate,
            _ => AccessLevel::Internal,
        }
    }

    fn parse_import(&mut self) -> SyxResult<Decl> {
        let span = self.advance().span;
        let first = self.consume_module_segment("Expected module path after import")?;
        let mut path = vec![first];
        while self.match_kind(TokenKind::Dot) {
            path.push(self.consume_module_segment("Expected identifier after '.' in import path")?);
        }
        let alias = if matches!(
            &self.peek().kind,
            TokenKind::Identifier(name) if name.as_str() == "as"
        ) {
            self.advance();
            Some(self.consume_identifier("Expected alias after 'as'")?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Decl::Import { span, path, alias })
    }

    fn parse_attributes(&mut self) -> SyxResult<Vec<Attribute>> {
        let mut attrs = Vec::new();
        while self.check(TokenKind::At) {
            self.advance();
            // Phase 120: `Service` is a reserved keyword but valid as an attribute name.
            let name = match &self.peek().kind {
                TokenKind::Identifier(ident) => {
                    let n = ident.clone();
                    self.advance();
                    n
                }
                TokenKind::Service => {
                    self.advance();
                    SmolStr::new("Service")
                }
                TokenKind::Macro => {
                    self.advance();
                    SmolStr::new("macro")
                }
                // W900: re-parse safety if any emitter still writes `@static`.
                TokenKind::Static => {
                    self.advance();
                    SmolStr::new("static")
                }
                _ => return self.error("Expected attribute name after @"),
            };
            let mut args = Vec::new();
            if self.match_kind(TokenKind::LParen) {
                if !self.check(TokenKind::RParen) {
                    loop {
                        match &self.peek().kind {
                            TokenKind::Identifier(ident) => {
                                args.push(AttributeArg::Ident(ident.clone()));
                                self.advance();
                            }
                            TokenKind::String(s) => {
                                args.push(AttributeArg::String(s.clone()));
                                self.advance();
                            }
                            TokenKind::Deinit => {
                                args.push(AttributeArg::Ident(SmolStr::new("deinit")));
                                self.advance();
                            }
                            TokenKind::Extension => {
                                args.push(AttributeArg::Ident(SmolStr::new("extension")));
                                self.advance();
                            }
                            TokenKind::Service => {
                                args.push(AttributeArg::Ident(SmolStr::new("Service")));
                                self.advance();
                            }
                            _ => {
                                return self.error("Expected identifier or string in attribute args")
                            }
                        }
                        if !self.match_kind(TokenKind::Comma) {
                            break;
                        }
                    }
                }
                self.consume(TokenKind::RParen, "Expected ')' after attribute args")?;
            }
            attrs.push(Attribute { name, args });
        }
        Ok(attrs)
    }

    fn parse_function(
        &mut self,
        access: AccessLevel,
        is_async: bool,
        attrs: Vec<Attribute>,
    ) -> SyxResult<Decl> {
        let start = if is_async {
            self.previous().span
        } else {
            self.advance().span
        };
        let name = self.consume_identifier("Expected function name")?;
        self.parse_function_with_name(access, is_async, attrs, start, name, false, false)
    }

    fn parse_function_with_name(
        &mut self,
        access: AccessLevel,
        is_async: bool,
        attrs: Vec<Attribute>,
        start: Span,
        name: SmolStr,
        is_override: bool,
        is_final: bool,
    ) -> SyxResult<Decl> {
        self.parse_function_with_name_opts(
            access,
            is_async,
            attrs,
            start,
            name,
            is_override,
            is_final,
            true,
        )
    }

    /// S1-I5: protocol requirements may be signature-only (`func f() -> T` with no `{` body).
    fn parse_function_with_name_opts(
        &mut self,
        access: AccessLevel,
        is_async: bool,
        attrs: Vec<Attribute>,
        start: Span,
        name: SmolStr,
        is_override: bool,
        is_final: bool,
        body_required: bool,
    ) -> SyxResult<Decl> {
        let (type_params, mut where_clause) = self.parse_type_params()?;
        self.consume(TokenKind::LParen, "Expected '(' after function name")?;
        let params = self.parse_params()?;
        self.consume(TokenKind::RParen, "Expected ')' after parameters")?;
        // S1-I7: `throws -> T` (Swift order) as well as `-> T throws`.
        let parse_throws_type = |p: &mut Self| -> SyxResult<TypeExpr> {
            if matches!(
                p.peek().kind,
                TokenKind::Identifier(_)
                    | TokenKind::Less
                    | TokenKind::LParen
                    | TokenKind::LBracket
            ) {
                p.parse_type()
            } else {
                Ok(TypeExpr::Named {
                    name: SmolStr::new("String"),
                    generics: Vec::new(),
                })
            }
        };
        let (return_type, throws_type) = if self.match_kind(TokenKind::Throws) {
            let throws_type = Some(parse_throws_type(self)?);
            let return_type = if self.match_kind(TokenKind::Arrow) {
                Some(self.parse_type()?)
            } else {
                None
            };
            (return_type, throws_type)
        } else if self.match_kind(TokenKind::Arrow) {
            let return_type = Some(self.parse_type()?);
            let throws_type = if self.match_kind(TokenKind::Throws) {
                Some(parse_throws_type(self)?)
            } else {
                None
            };
            (return_type, throws_type)
        } else {
            (None, None)
        };
        let effect = if self.match_kind(TokenKind::Requires) {
            Some(match self.advance().kind {
                TokenKind::Pure => syx_ast::FunctionEffect::Pure,
                TokenKind::Io => syx_ast::FunctionEffect::Io,
                _ => {
                    return self.error("Expected 'Pure' or 'IO' after requires");
                }
            })
        } else {
            None
        };
        where_clause.extend(self.parse_where_clause()?);
        let body = if self.check(TokenKind::LBrace) {
            self.advance();
            self.parse_block_stmts()?
        } else if body_required {
            return self.error("Expected '{' before function body");
        } else {
            // Protocol requirement / default-less signature.
            self.match_kind(TokenKind::Semicolon);
            Vec::new()
        };
        let span = Span::merge(start, self.previous().span);
        Ok(Decl::Function {
            span,
            access,
            attrs,
            name,
            type_params,
            params,
            return_type,
            throws_type,
            effect,
            where_clause,
            body,
            is_async,
            is_override,
            is_final,
        })
    }

    /// Type params with optional inline constraints: `<T>`, `<T: Equatable>`, `<T: P, U>`.
    /// Inline constraints are returned as WherePreds (merged by callers into where_clause).
    fn parse_type_params(&mut self) -> SyxResult<(Vec<SmolStr>, Vec<WherePred>)> {
        let (params, inline, _defaults) = self.parse_type_params_with_defaults()?;
        Ok((params, inline))
    }

    /// S1-I6: try to consume `<Type, …>` for `Holder<Int>(…)`. Returns false on failure
    /// without reporting (caller restores `current`).
    fn try_skip_generic_type_args(&mut self) -> bool {
        if !self.match_kind(TokenKind::Less) {
            return false;
        }
        loop {
            if self.parse_type().is_err() {
                return false;
            }
            if self.match_kind(TokenKind::Comma) {
                continue;
            }
            break;
        }
        self.consume_rangle("").is_ok()
    }

    /// S1-I6: `T` / `T: P` / `T = Int` / `T: P = Int` type-parameter lists.
    fn parse_type_params_with_defaults(
        &mut self,
    ) -> SyxResult<(Vec<SmolStr>, Vec<WherePred>, Vec<Option<TypeExpr>>)> {
        let mut params = Vec::new();
        let mut inline = Vec::new();
        let mut defaults = Vec::new();
        if self.match_kind(TokenKind::Less) {
            loop {
                let name = self.consume_identifier("Expected type parameter name")?;
                if self.match_kind(TokenKind::Colon) {
                    // S1-I7: `T: A & B` → multiple WherePreds.
                    loop {
                        let constraint =
                            self.consume_identifier("Expected protocol constraint after ':'")?;
                        inline.push(WherePred {
                            type_name: name.clone(),
                            constraint,
                        });
                        if !self.match_kind(TokenKind::Amp) {
                            break;
                        }
                    }
                }
                let default = if self.match_kind(TokenKind::Assign) {
                    Some(self.parse_type()?)
                } else {
                    None
                };
                params.push(name);
                defaults.push(default);
                if !self.match_kind(TokenKind::Comma) {
                    break;
                }
            }
            self.consume_rangle("Expected '>' after type parameters")?;
        }
        Ok((params, inline, defaults))
    }

    fn parse_where_clause(&mut self) -> SyxResult<Vec<WherePred>> {
        let mut preds = Vec::new();
        if self.match_kind(TokenKind::Where) {
            loop {
                let type_name = self.consume_identifier("Expected type name in where clause")?;
                self.consume(TokenKind::Colon, "Expected ':' in where clause")?;
                // S1-I7: `where T: A & B`
                loop {
                    let constraint =
                        self.consume_identifier("Expected constraint in where clause")?;
                    preds.push(WherePred {
                        type_name: type_name.clone(),
                        constraint,
                    });
                    if !self.match_kind(TokenKind::Amp) {
                        break;
                    }
                }
                if !self.match_kind(TokenKind::Comma) {
                    break;
                }
            }
        }
        Ok(preds)
    }

    fn parse_enum(&mut self, access: AccessLevel, attrs: Vec<syx_ast::Attribute>) -> SyxResult<Decl> {
        let start = self.advance().span;
        let name = self.consume_identifier("Expected enum name")?;
        let (type_params, _inline_constraints) = self.parse_type_params()?;
        // S1-I2: optional conformance list (`enum E: Error`) — parse-accept; semantic thin.
        if self.match_kind(TokenKind::Colon) {
            loop {
                let _ = self.consume_identifier("Expected conformance name")?;
                if !self.match_kind(TokenKind::Comma) {
                    break;
                }
            }
        }
        self.consume(TokenKind::LBrace, "Expected '{' after enum name")?;
        let mut cases = Vec::new();
        let mut methods = Vec::new();
        let mut anon_field_i = 0u32;
        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            self.skip_newlines();
            if self.check(TokenKind::RBrace) {
                break;
            }
            // Book: methods after cases (`func isFinal() …`) → Extension on the enum type.
            if matches!(
                self.peek().kind,
                TokenKind::Func | TokenKind::Mutating | TokenKind::Public | TokenKind::Private
                    | TokenKind::Internal | TokenKind::FilePrivate | TokenKind::At
            ) {
                methods.push(self.parse_class_member()?);
                continue;
            }
            // Book requires `case`; legacy fixtures without `case` still accepted.
            let _ = self.match_kind(TokenKind::Case);
            let case_name = self.consume_identifier("Expected enum case name")?;
            let mut associated_fields = Vec::new();
            let mut value = None;
            if self.match_kind(TokenKind::LParen) {
                while !self.check(TokenKind::RParen) {
                    // Named `code: Int` / `data: String` (soft kw) or unlabeled `String` / `T`.
                    if self.peek_is_label_colon() {
                        let field = self.consume_identifier("Expected associated field name")?;
                        self.consume(TokenKind::Colon, "Expected ':' after associated field")?;
                        associated_fields.push((field, self.parse_type()?));
                    } else {
                        let ty = self.parse_type()?;
                        let field = SmolStr::new(format!("_{anon_field_i}"));
                        anon_field_i += 1;
                        associated_fields.push((field, ty));
                    }
                    if !self.match_kind(TokenKind::Comma) {
                        break;
                    }
                }
                self.consume(TokenKind::RParen, "Expected ')' after associated fields")?;
            }
            if self.match_kind(TokenKind::Assign) {
                value = Some(self.parse_expr()?);
            }
            cases.push((case_name, value, associated_fields));
            // Optional trailing comma between cases (book style).
            // Multi-name `case ok, err, pending` — more identifiers after comma.
            while self.match_kind(TokenKind::Comma) {
                self.skip_newlines();
                if self.check(TokenKind::RBrace)
                    || matches!(
                        self.peek().kind,
                        TokenKind::Case
                            | TokenKind::Func
                            | TokenKind::Mutating
                            | TokenKind::Public
                            | TokenKind::Private
                            | TokenKind::Internal
                            | TokenKind::FilePrivate
                            | TokenKind::At
                    )
                {
                    break;
                }
                if matches!(self.peek().kind, TokenKind::Identifier(_))
                    || Self::soft_keyword_ident(&self.peek().kind).is_some()
                {
                    let extra = self.consume_identifier("Expected enum case name")?;
                    cases.push((extra, None, Vec::new()));
                } else {
                    break;
                }
            }
            self.skip_newlines();
        }
        self.consume(TokenKind::RBrace, "Expected '}' after enum cases")?;
        let end = self.previous().span;
        if !methods.is_empty() {
            self.pending_decls.push(Decl::Extension {
                span: Span::merge(start, end),
                type_name: name.clone(),
                conformances: Vec::new(),
                where_clause: Vec::new(),
                methods,
            });
        }
        Ok(Decl::Enum {
            span: Span::merge(start, end),
            access,
            attrs,
            name,
            type_params,
            cases,
        })
    }

    fn parse_init(&mut self, access: AccessLevel) -> SyxResult<Decl> {
        let start = self.advance().span;
        // `init?` / `init!` failable (IUO) — marker only for S1-I10 check surface.
        let failable = matches!(
            self.peek().kind,
            TokenKind::Question | TokenKind::Bang
        );
        if failable {
            self.advance();
        }
        self.consume(TokenKind::LParen, "Expected '(' after init")?;
        let params = self.parse_params()?;
        self.consume(TokenKind::RParen, "Expected ')' after init parameters")?;
        let throws = self.match_kind(TokenKind::Throws);
        if throws && self.match_kind(TokenKind::Arrow) {
            let _ = self.parse_type()?;
        }
        self.consume(TokenKind::LBrace, "Expected '{' before init body")?;
        let body = self.parse_block_stmts()?;
        Ok(Decl::Init {
            span: Span::merge(start, self.previous().span),
            access,
            params,
            body,
            failable,
            throws,
        })
    }

    fn parse_deinit(&mut self, access: AccessLevel) -> SyxResult<Decl> {
        let start = self.advance().span;
        self.consume(TokenKind::LBrace, "Expected '{' before deinit body")?;
        let body = self.parse_block_stmts()?;
        Ok(Decl::Deinit {
            span: Span::merge(start, self.previous().span),
            access,
            body,
        })
    }

    fn parse_class_member(&mut self) -> SyxResult<Decl> {
        let access = if matches!(
            self.peek().kind,
            TokenKind::Public | TokenKind::Internal | TokenKind::Private | TokenKind::FilePrivate
        ) {
            self.parse_access()
        } else {
            AccessLevel::Internal
        };
        let attrs = self.parse_attributes()?;
        if self.check_identifier("operator") {
            self.advance();
            self.consume(TokenKind::Func, "Expected 'func' after operator")?;
            let op = self.parse_operator_symbol()?;
            let start = self.previous().span;
            let (_type_params, _) = self.parse_type_params()?;
            self.consume(TokenKind::LParen, "Expected '(' after operator")?;
            let params = self.parse_params()?;
            self.consume(TokenKind::RParen, "Expected ')' after parameters")?;
            let return_type = if self.match_kind(TokenKind::Arrow) {
                Some(self.parse_type()?)
            } else {
                None
            };
            self.consume(TokenKind::LBrace, "Expected '{' before operator body")?;
            let body = self.parse_block_stmts()?;
            return Ok(Decl::OperatorFunc {
                span: Span::merge(start, self.previous().span),
                access,
                attrs,
                op,
                params,
                return_type,
                body,
            });
        }
        let is_override = self.match_identifier("override");
        if is_override && self.check(TokenKind::Func) {
            let start = self.advance().span;
            let name = self.consume_identifier("Expected function name after override")?;
            return self.parse_function_with_name(access, false, attrs, start, name, true, false);
        }
        if is_override && matches!(&self.peek().kind, TokenKind::Var | TokenKind::Let) {
            return self.parse_global_var(access, attrs, false, true);
        }
        let is_final = self.match_kind(TokenKind::Final);
        if is_final && self.check(TokenKind::Func) {
            let start = self.advance().span;
            let name = self.consume_identifier("Expected function name after final")?;
            return self.parse_function_with_name(access, false, attrs, start, name, false, true);
        }
        // S1-02: `mutating func` — record as `@mutating` attr (semantic S020 deepen → S1-06/I2).
        if self.match_kind(TokenKind::Mutating) {
            let mut attrs = attrs;
            attrs.push(Attribute {
                name: SmolStr::new("mutating"),
                args: vec![],
            });
            if self.check(TokenKind::Func) {
                return self.parse_function(access, false, attrs);
            }
            if self.check(TokenKind::Async) {
                self.advance();
                self.consume(TokenKind::Func, "Expected 'func' after mutating async")?;
                return self.parse_function(access, true, attrs);
            }
            return self.error("Expected 'func' after mutating");
        }
        if self.match_kind(TokenKind::Static) {
            match &self.peek().kind {
                TokenKind::Func => {
                    let mut attrs = self.parse_attributes()?;
                    attrs.push(Attribute {
                        name: SmolStr::new("static"),
                        args: vec![],
                    });
                    return self.parse_function(access, false, attrs);
                }
                TokenKind::Async => {
                    let mut attrs = self.parse_attributes()?;
                    attrs.push(Attribute {
                        name: SmolStr::new("static"),
                        args: vec![],
                    });
                    self.advance();
                    self.consume(TokenKind::Func, "Expected 'func' after async")?;
                    return self.parse_function(access, true, attrs);
                }
                TokenKind::Var | TokenKind::Let => {
                    let mut attrs = self.parse_attributes()?;
                    attrs.push(Attribute {
                        name: SmolStr::new("static"),
                        args: vec![],
                    });
                    return self.parse_global_var(access, attrs, false, false);
                }
                _ => return self.error("Expected 'func', 'var', or 'let' after static"),
            }
        }
        let is_lazy = self.match_identifier("lazy");
        if is_lazy && !self.check(TokenKind::Var) {
            return self.error("Expected 'var' after lazy");
        }
        match &self.peek().kind {
            TokenKind::Init => self.parse_init(access),
            TokenKind::Deinit => self.parse_deinit(access),
            TokenKind::Subscript => self.parse_subscript_member(access),
            TokenKind::Var | TokenKind::Let => self.parse_global_var(access, attrs, is_lazy, false),
            TokenKind::Class => self.parse_class(access, attrs),
            TokenKind::Struct => self.parse_struct(access, attrs),
            TokenKind::Enum => self.parse_enum(access, attrs),
            TokenKind::Final => {
                self.advance();
                self.consume(TokenKind::Class, "Expected 'class' after final")?;
                self.parse_final_class(access)
            }
            TokenKind::Async => {
                self.advance();
                self.consume(TokenKind::Func, "Expected 'func' after async")?;
                self.parse_function(access, true, attrs)
            }
            TokenKind::Func => self.parse_function(access, false, attrs),
            _ => self.parse_decl_with_access(access),
        }
    }

    fn parse_typealias(&mut self, access: AccessLevel) -> SyxResult<Decl> {
        let start = self.advance().span;
        let name = self.consume_identifier("Expected name after typealias")?;
        self.consume(TokenKind::Assign, "Expected '=' in typealias")?;
        let ty = self.parse_type()?;
        self.skip_newlines();
        Ok(Decl::TypeAlias {
            span: Span::merge(start, self.previous().span),
            access,
            name,
            ty,
        })
    }

    fn parse_user_macro_decl_with_attrs(&mut self, attrs: Vec<Attribute>) -> SyxResult<Decl> {
        let start = self.advance().span;
        let first = self.consume_macro_name("Expected macro name after '#")?;
        if first.as_str() != "macro" {
            if !self.check(TokenKind::LParen) {
                return self.error("Expected '(' after macro name");
            }
            self.consume(TokenKind::LParen, "Expected '(' after macro name")?;
            let mut args = Vec::new();
            if !self.check(TokenKind::RParen) {
                loop {
                    args.push(self.parse_expr()?);
                    if !self.match_kind(TokenKind::Comma) {
                        break;
                    }
                }
            }
            self.consume(TokenKind::RParen, "Expected ')' after macro arguments")?;
            self.skip_newlines();
            return Ok(Decl::MacroSiteInvoke {
                span: Span::merge(start, self.previous().span),
                name: first,
                args,
            });
        }
        let name = self.consume_identifier("Expected macro name")?;
        self.consume(TokenKind::LParen, "Expected '(' after macro name")?;
        let mut params = Vec::new();
        if !self.check(TokenKind::RParen) {
            loop {
                params.push(self.consume_identifier("Expected macro parameter")?);
                if !self.match_kind(TokenKind::Comma) {
                    break;
                }
            }
        }
        self.consume(TokenKind::RParen, "Expected ')' after macro parameters")?;
        self.consume(TokenKind::LBrace, "Expected '{' before macro body")?;
        let is_decl = attrs.iter().any(|a| {
            a.name.as_str() == "freestanding"
                && a.args.iter().any(|arg| {
                    matches!(arg, AttributeArg::Ident(s) if s.as_str() == "declaration")
                })
        });
        let (body, decl_body) = if is_decl {
            (Vec::new(), self.parse_macro_decl_body()?)
        } else {
            (self.parse_block_stmts()?, Vec::new())
        };
        Ok(Decl::MacroDecl {
            span: Span::merge(start, self.previous().span),
            access: syx_ast::AccessLevel::Internal,
            attrs,
            name,
            implementation: syx_ast::MacroImplementation::User {
                params,
                body,
                decl_body,
            },
        })
    }

    fn parse_macro_decl_body(&mut self) -> SyxResult<Vec<Decl>> {
        let mut decls = Vec::new();
        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            decls.push(self.parse_decl()?);
        }
        self.consume(TokenKind::RBrace, "Expected '}' after macro body")?;
        Ok(decls)
    }

    #[allow(dead_code)] // entry retained; attrs path used by decl parse
    fn parse_user_macro_decl(&mut self) -> SyxResult<Decl> {
        self.parse_user_macro_decl_with_attrs(vec![])
    }

    fn parse_macro_decl(&mut self, access: AccessLevel) -> SyxResult<Decl> {
        let start = self.advance().span;
        let name = self.consume_identifier("Expected macro name")?;
        self.consume(TokenKind::Assign, "Expected '=' in macro declaration")?;
        self.consume(TokenKind::Hash, "Expected '#externalMacro' or '#builtInMacro'")?;
        let ext = self.consume_macro_name("Expected macro name after '#")?;
        if ext.as_str() == "builtInMacro" {
            self.consume(TokenKind::LParen, "Expected '(' after builtInMacro")?;
            let id = match self.peek().kind.clone() {
                TokenKind::String(s) => {
                    self.advance();
                    s
                }
                _ => return self.error("Expected string id in builtInMacro"),
            };
            self.consume(TokenKind::RParen, "Expected ')' after builtInMacro")?;
            return Ok(Decl::MacroDecl {
                span: Span::merge(start, self.previous().span),
                access,
                attrs: vec![],
                name,
                implementation: syx_ast::MacroImplementation::BuiltIn { id },
            });
        }
        if ext.as_str() != "externalMacro" {
            return self.error("Expected '#externalMacro(...)' or '#builtInMacro(...)' in macro declaration");
        }
        self.consume(TokenKind::LParen, "Expected '(' after externalMacro")?;
        let mut module = String::new();
        let mut type_name = String::new();
        while !self.check(TokenKind::RParen) {
            let label = match self.peek().kind.clone() {
                TokenKind::Identifier(s) => {
                    self.advance();
                    s.to_string()
                }
                TokenKind::Type => {
                    self.advance();
                    "type".to_string()
                }
                _ => return self.error("Expected label in externalMacro"),
            };
            self.consume(TokenKind::Colon, "Expected ':' after externalMacro label")?;
            let value = match self.peek().kind.clone() {
                TokenKind::String(s) => {
                    self.advance();
                    s
                }
                TokenKind::Identifier(s) => {
                    self.advance();
                    s.to_string()
                }
                _ => return self.error("Expected string in externalMacro argument"),
            };
            match label.as_str() {
                "module" => module = value,
                "type" => type_name = value,
                _ => {}
            }
            if !self.match_kind(TokenKind::Comma) {
                break;
            }
        }
        self.consume(TokenKind::RParen, "Expected ')' after externalMacro")?;
        Ok(Decl::MacroDecl {
            span: Span::merge(start, self.previous().span),
            access,
            attrs: vec![],
            name,
            implementation: syx_ast::MacroImplementation::External { module, type_name },
        })
    }

    fn parse_subscript_member(&mut self, access: AccessLevel) -> SyxResult<Decl> {
        let _ = access;
        let start = self.advance().span;
        self.consume(TokenKind::LParen, "Expected '(' after subscript")?;
        let params = self.parse_params()?;
        self.consume(TokenKind::RParen, "Expected ')' after subscript parameters")?;
        self.consume(TokenKind::Arrow, "Expected '->' after subscript parameters")?;
        let return_type = self.parse_type()?;
        self.consume(TokenKind::LBrace, "Expected '{' before subscript body")?;
        self.consume_identifier("Expected 'get' in subscript")?;
        self.consume(TokenKind::LBrace, "Expected '{' before subscript getter")?;
        let getter = self.parse_block_stmts()?;
        let setter = if matches!(
            &self.peek().kind,
            TokenKind::Identifier(name) if name.as_str() == "set"
        ) {
            self.advance();
            self.consume(TokenKind::LBrace, "Expected '{' before subscript setter")?;
            let body = self.parse_block_stmts()?;
            Some(body)
        } else {
            None
        };
        self.consume(TokenKind::RBrace, "Expected '}' after subscript")?;
        Ok(Decl::Subscript {
            span: Span::merge(start, self.previous().span),
            type_name: SmolStr::new("__enclosing"),
            params,
            return_type,
            getter,
            setter,
        })
    }

    fn parse_extension_member(&mut self) -> SyxResult<Decl> {
        let access = if matches!(
            self.peek().kind,
            TokenKind::Public | TokenKind::Internal | TokenKind::Private | TokenKind::FilePrivate
        ) {
            self.parse_access()
        } else {
            AccessLevel::Internal
        };
        if matches!(self.peek().kind, TokenKind::Subscript) {
            return self.parse_subscript_member(access);
        }
        if self.match_kind(TokenKind::Static) {
            let mut attrs = vec![syx_ast::Attribute {
                name: SmolStr::new("static"),
                args: vec![],
            }];
            attrs.extend(self.parse_attributes()?);
            if !self.check(TokenKind::Func) {
                return self.error("Expected 'func' after static");
            }
            return self.parse_function(access, false, attrs);
        }
        let attrs = self.parse_attributes()?;
        // Phase 136: extension computed `{ get }` / `{ get set }` properties.
        if matches!(&self.peek().kind, TokenKind::Var | TokenKind::Let) {
            return self.parse_global_var(access, attrs, false, false);
        }
        // S1-I7: convenience `init` inside extension.
        if self.check(TokenKind::Init) {
            return self.parse_init(access);
        }
        if !self.check(TokenKind::Func) {
            return self.error("Expected 'func', 'init', or computed 'var'/'let' in extension");
        }
        self.parse_function(access, false, attrs)
    }

    fn parse_data_class(&mut self, access: AccessLevel, attrs: Vec<Attribute>) -> SyxResult<Decl> {
        let start = self.previous().span;
        let name = self.consume_identifier("Expected data class name")?;
        let (type_params, mut where_clause) = self.parse_type_params()?;
        self.consume(TokenKind::LParen, "Expected '(' after data class name")?;
        let mut fields = Vec::new();
        if !self.check(TokenKind::RParen) {
            loop {
                let field = self.consume_identifier("Expected field name")?;
                self.consume(TokenKind::Colon, "Expected ':' after field")?;
                let ty = self.parse_type()?;
                fields.push((field, ty));
                if !self.match_kind(TokenKind::Comma) {
                    break;
                }
            }
        }
        self.consume(TokenKind::RParen, "Expected ')' after data class fields")?;
        where_clause.extend(self.parse_where_clause()?);
        self.skip_newlines();
        Ok(Decl::DataClass {
            span: Span::merge(start, self.previous().span),
            access,
            attrs,
            name,
            type_params,
            where_clause,
            fields,
        })
    }

    fn parse_final_class(&mut self, access: AccessLevel) -> SyxResult<Decl> {
        let start = self.previous().span;
        let name = self.consume_identifier("Expected final class name")?;
        let (type_params, _) = self.parse_type_params()?;
        // OOP `final class Token { let …; init … }` vs ADT `final class Result { Ok(…) Err(…) }`.
        self.consume(TokenKind::LBrace, "Expected '{' after final class")?;
        self.skip_newlines();
        if self.looks_like_oop_final_class_member() {
            let mut members = Vec::new();
            while !self.check(TokenKind::RBrace) && !self.is_at_end() {
                self.skip_newlines();
                if self.check(TokenKind::RBrace) {
                    break;
                }
                while self.match_kind(TokenKind::Semicolon) {}
                if self.check(TokenKind::RBrace) {
                    break;
                }
                members.push(self.parse_class_member()?);
            }
            self.consume(TokenKind::RBrace, "Expected '}' after final class")?;
            return Ok(Decl::Class {
                span: Span::merge(start, self.previous().span),
                access,
                attrs: vec![Attribute {
                    name: SmolStr::new("final"),
                    args: vec![],
                }],
                name,
                type_params,
                type_param_defaults: Vec::new(),
                superclass: None,
                conformances: Vec::new(),
                members,
            });
        }
        let mut variants = Vec::new();
        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            self.skip_newlines();
            if self.check(TokenKind::RBrace) {
                break;
            }
            let variant = self.consume_identifier("Expected variant name")?;
            let mut fields = Vec::new();
            if self.match_kind(TokenKind::LParen) {
                while !self.check(TokenKind::RParen) {
                    let f = self.consume_identifier("Expected field name")?;
                    self.consume(TokenKind::Colon, "Expected ':'")?;
                    fields.push((f, self.parse_type()?));
                    if !self.match_kind(TokenKind::Comma) {
                        break;
                    }
                }
                self.consume(TokenKind::RParen, "Expected ')'")?;
            }
            variants.push((variant, fields));
        }
        self.consume(TokenKind::RBrace, "Expected '}' after final class")?;
        Ok(Decl::FinalClass {
            span: Span::merge(start, self.previous().span),
            access,
            name,
            type_params,
            variants,
        })
    }

    /// True when `final class` body starts with OOP members (`let`/`var`/`init`/`func`/…).
    fn looks_like_oop_final_class_member(&self) -> bool {
        matches!(
            self.peek().kind,
            TokenKind::Var
                | TokenKind::Let
                | TokenKind::Init
                | TokenKind::Deinit
                | TokenKind::Func
                | TokenKind::Async
                | TokenKind::Static
                | TokenKind::Public
                | TokenKind::Private
                | TokenKind::Internal
                | TokenKind::FilePrivate
                | TokenKind::Subscript
                | TokenKind::Final
                | TokenKind::Mutating
                | TokenKind::At
        ) || self.check_identifier("override")
            || self.check_identifier("lazy")
            || self.check_identifier("operator")
    }

    fn parse_conformances(&mut self) -> SyxResult<Vec<SmolStr>> {
        let mut conformances = Vec::new();
        if self.match_kind(TokenKind::Colon) {
            loop {
                conformances.push(self.consume_identifier("Expected protocol name")?);
                if !self.match_kind(TokenKind::Comma) {
                    break;
                }
            }
        }
        Ok(conformances)
    }

    fn parse_extension(&mut self) -> SyxResult<Decl> {
        let start = self.advance().span;
        let type_name = self.consume_identifier("Expected type name after extension")?;
        let conformances = self.parse_conformances()?;
        let where_clause = self.parse_where_clause()?;
        self.consume(TokenKind::LBrace, "Expected '{' after extension type")?;
        let mut methods = Vec::new();
        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            self.skip_newlines();
            if self.check(TokenKind::RBrace) {
                break;
            }
            methods.push(self.parse_extension_member()?);
        }
        self.consume(TokenKind::RBrace, "Expected '}' after extension")?;
        Ok(Decl::Extension {
            span: Span::merge(start, self.previous().span),
            type_name,
            conformances,
            where_clause,
            methods,
        })
    }

    fn parse_global_actor(&mut self, access: AccessLevel) -> SyxResult<Decl> {
        let start = self.previous().span;
        let name = self.consume_identifier("Expected global actor name")?;
        self.consume(TokenKind::LBrace, "Expected '{' after global actor name")?;
        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            self.skip_newlines();
            if self.check(TokenKind::RBrace) {
                break;
            }
            return self.error("Global actor body must be empty in MVP");
        }
        self.consume(TokenKind::RBrace, "Expected '}' after global actor")?;
        Ok(Decl::GlobalActor {
            span: Span::merge(start, self.previous().span),
            access,
            name,
        })
    }

    fn parse_actor(&mut self, access: AccessLevel) -> SyxResult<Decl> {
        let start = self.advance().span;
        let name = self.consume_identifier("Expected actor name")?;
        // S1-I5/I6: `actor Holder<T>` / `actor Holder<T = Int>`.
        let (type_params, _inline, type_param_defaults) = self.parse_type_params_with_defaults()?;
        let conformances = self.parse_conformances()?;
        self.consume(TokenKind::LBrace, "Expected '{' after actor name")?;
        let mut members = Vec::new();
        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            self.skip_newlines();
            if self.check(TokenKind::RBrace) {
                break;
            }
            while self.match_kind(TokenKind::Semicolon) {}
            if self.check(TokenKind::RBrace) {
                break;
            }
            members.push(self.parse_class_member()?);
        }
        self.consume(TokenKind::RBrace, "Expected '}' after actor body")?;
        Ok(Decl::Actor {
            span: Span::merge(start, self.previous().span),
            access,
            name,
            type_params,
            type_param_defaults,
            conformances,
            members,
        })
    }

    fn parse_class_super_and_conformances(
        &mut self,
    ) -> SyxResult<(Option<SmolStr>, Vec<SmolStr>)> {
        if !self.match_kind(TokenKind::Colon) {
            return Ok((None, Vec::new()));
        }
        let first = self.consume_identifier("Expected superclass or protocol name")?;
        if self.match_kind(TokenKind::Comma) {
            let mut conformances = Vec::new();
            loop {
                conformances.push(self.consume_identifier("Expected protocol name")?);
                if !self.match_kind(TokenKind::Comma) {
                    break;
                }
            }
            if is_known_protocol_name(first.as_str()) {
                let mut all = vec![first];
                all.extend(conformances);
                return Ok((None, all));
            }
            return Ok((Some(first), conformances));
        }
        if is_known_protocol_name(first.as_str()) {
            Ok((None, vec![first]))
        } else {
            Ok((Some(first), Vec::new()))
        }
    }

    fn parse_class(&mut self, access: AccessLevel, attrs: Vec<Attribute>) -> SyxResult<Decl> {
        let start = if self.check(TokenKind::Class) {
            self.advance().span
        } else {
            self.peek().span
        };
        let name = self.consume_identifier("Expected class name")?;
        // S1-I6: `class Holder<T>` / `class Holder<T = Int>` before superclass/conformances.
        let (type_params, _inline, type_param_defaults) = self.parse_type_params_with_defaults()?;
        let (superclass, conformances) = self.parse_class_super_and_conformances()?;
        self.consume(TokenKind::LBrace, "Expected '{' after class name")?;
        let mut members = Vec::new();
        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            self.skip_newlines();
            if self.check(TokenKind::RBrace) {
                break;
            }
            while self.match_kind(TokenKind::Semicolon) {}
            if self.check(TokenKind::RBrace) {
                break;
            }
            members.push(self.parse_class_member()?);
        }
        self.consume(TokenKind::RBrace, "Expected '}' after class body")?;
        let span = Span::merge(start, self.previous().span);
        Ok(Decl::Class {
            span,
            access,
            attrs,
            name,
            type_params,
            type_param_defaults,
            superclass,
            conformances,
            members,
        })
    }

    fn parse_struct(&mut self, access: AccessLevel, attrs: Vec<Attribute>) -> SyxResult<Decl> {
        let start = self.advance().span;
        let name = self.consume_identifier("Expected struct name")?;
        let (type_params, _inline, type_param_defaults) = self.parse_type_params_with_defaults()?;
        // S1-02 first slice: optional protocol conformances (`struct Foo: Bar`).
        let conformances = if self.match_kind(TokenKind::Colon) {
            let mut list = Vec::new();
            loop {
                list.push(self.consume_identifier("Expected conformance name")?);
                if !self.match_kind(TokenKind::Comma) {
                    break;
                }
            }
            list
        } else {
            Vec::new()
        };
        let _ = conformances; // parse-accept; semantic wiring → S1-I2
        self.consume(TokenKind::LBrace, "Expected '{' after struct name")?;
        let mut fields = Vec::new();
        let mut members = Vec::new();
        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            self.skip_newlines();
            if self.check(TokenKind::RBrace) {
                break;
            }
            // Legacy bare field: `x: Int` (no var/let).
            if matches!(self.peek().kind, TokenKind::Identifier(_))
                && matches!(
                    self.peek_ahead(1).map(|t| &t.kind),
                    Some(TokenKind::Colon)
                )
            {
                let field = self.consume_identifier("Expected field name")?;
                self.consume(TokenKind::Colon, "Expected ':' after field name")?;
                let ty = self.parse_type()?;
                fields.push((field, ty));
                self.skip_newlines();
                continue;
            }
            // Book-style: `var`/`let` fields, `mutating func`, `init`, nested decls.
            let member = self.parse_class_member()?;
            match &member {
                Decl::VarGlobal {
                    name: fname,
                    ty: Some(ty),
                    ..
                } => {
                    fields.push((fname.clone(), ty.clone()));
                    members.push(member);
                }
                Decl::VarGlobal { ty: None, .. } => {
                    members.push(member);
                }
                _ => members.push(member),
            }
            self.skip_newlines();
        }
        self.consume(TokenKind::RBrace, "Expected '}' after struct body")?;
        let span = Span::merge(start, self.previous().span);
        Ok(Decl::Struct {
            span,
            access,
            attrs,
            name,
            type_params,
            type_param_defaults,
            fields,
            members,
        })
    }

    fn parse_global_var(&mut self, access: AccessLevel, attrs: Vec<Attribute>, is_lazy: bool, is_override: bool) -> SyxResult<Decl> {
        let start = self.advance().span;
        let mutable = matches!(self.previous().kind, TokenKind::Var);
        // Top-level `let (x, y) = …` — desugar like stmt tuple destructure.
        if self.match_kind(TokenKind::LParen) {
            let mut bindings = Vec::new();
            if !self.check(TokenKind::RParen) {
                loop {
                    bindings.push(self.consume_identifier("Expected binding name in tuple pattern")?);
                    if !self.match_kind(TokenKind::Comma) {
                        break;
                    }
                }
            }
            self.consume(TokenKind::RParen, "Expected ')' after tuple pattern")?;
            let ty = if self.match_kind(TokenKind::Colon) {
                Some(self.parse_type()?)
            } else {
                None
            };
            let init = if self.match_kind(TokenKind::Assign) {
                Some(self.parse_expr()?)
            } else {
                None
            };
            self.skip_newlines();
            let end = self.previous().span;
            let tmp = SmolStr::new(format!("__tuple_{}_{}", start.start, bindings.len()));
            for (i, bind) in bindings.iter().enumerate() {
                self.pending_decls.push(Decl::VarGlobal {
                    span: Span::merge(start, end),
                    access,
                    attrs: Vec::new(),
                    mutable,
                    name: bind.clone(),
                    ty: None,
                    init: Some(Expr::Member {
                        span: end,
                        object: Box::new(Expr::Ident {
                            span: end,
                            name: tmp.clone(),
                        }),
                        name: SmolStr::new(i.to_string()),
                    }),
                    will_set: None,
                    did_set: None,
                    computed_getter: None,
                    computed_setter: None,
                    is_override: false,
                    is_lazy: false,
                });
            }
            return Ok(Decl::VarGlobal {
                span: Span::merge(start, end),
                access,
                attrs,
                mutable: false,
                name: tmp,
                ty,
                init,
                will_set: None,
                did_set: None,
                computed_getter: None,
                computed_setter: None,
                is_override,
                is_lazy,
            });
        }
        let name = self.consume_identifier("Expected variable name")?;
        let ty = if self.match_kind(TokenKind::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };
        let mut init = None;
        let mut computed_getter = None;
        let mut computed_setter = None;
        let mut will_set = None;
        let mut did_set = None;
        if self.check(TokenKind::LBrace) {
            self.advance();
            if matches!(
                &self.peek().kind,
                TokenKind::Identifier(label) if label.as_str() == "get"
            ) {
                self.advance();
                self.consume(TokenKind::LBrace, "Expected '{' before computed property getter")?;
                // `parse_block_stmts` consumes the getter's closing `}`.
                computed_getter = Some(self.parse_block_stmts()?);
                if matches!(
                    &self.peek().kind,
                    TokenKind::Identifier(label) if label.as_str() == "set"
                ) {
                    if !mutable {
                        return self.error("Computed property setter requires 'var'");
                    }
                    self.advance();
                    self.consume(TokenKind::LBrace, "Expected '{' before computed property setter")?;
                    computed_setter = Some(self.parse_block_stmts()?);
                }
                self.consume(TokenKind::RBrace, "Expected '}' after computed property")?;
            } else {
                // S1-I7: `{ return … }` get-only shorthand (no explicit `get`).
                computed_getter = Some(self.parse_block_stmts()?);
            }
        } else {
            init = if self.match_kind(TokenKind::Assign) {
                Some(self.parse_expr()?)
            } else {
                None
            };
            // Swift-style braced observers after init: `var x: Int = 0 { willSet {…} didSet {…} }`
            if self.check(TokenKind::LBrace) {
                self.advance();
                self.skip_newlines();
                if matches!(
                    &self.peek().kind,
                    TokenKind::Identifier(name) if name.as_str() == "willSet"
                ) {
                    self.advance();
                    self.consume(TokenKind::LBrace, "Expected '{' after willSet")?;
                    will_set = Some(self.parse_block_stmts()?);
                    self.skip_newlines();
                }
                if matches!(
                    &self.peek().kind,
                    TokenKind::Identifier(name) if name.as_str() == "didSet"
                ) {
                    self.advance();
                    self.consume(TokenKind::LBrace, "Expected '{' after didSet")?;
                    did_set = Some(self.parse_block_stmts()?);
                    self.skip_newlines();
                }
                self.consume(
                    TokenKind::RBrace,
                    "Expected '}' after property observers",
                )?;
            } else {
                if matches!(
                    &self.peek().kind,
                    TokenKind::Identifier(name) if name.as_str() == "willSet"
                ) {
                    self.advance();
                    self.consume(TokenKind::LBrace, "Expected '{' after willSet")?;
                    will_set = Some(self.parse_block_stmts()?);
                }
                if matches!(
                    &self.peek().kind,
                    TokenKind::Identifier(name) if name.as_str() == "didSet"
                ) {
                    self.advance();
                    self.consume(TokenKind::LBrace, "Expected '{' after didSet")?;
                    did_set = Some(self.parse_block_stmts()?);
                }
            }
        }
        self.skip_newlines();
        let span = Span::merge(start, self.previous().span);
        Ok(Decl::VarGlobal {
            span,
            access,
            attrs,
            mutable,
            name,
            ty,
            init,
            will_set,
            did_set,
            computed_getter,
            computed_setter,
            is_override,
            is_lazy,
        })
    }

    /// Parse parameter external label and internal name (`_ name`, `label name`, or `name`).
    fn parse_param_name(&mut self) -> SyxResult<(ParamLabel, SmolStr)> {
        let first = self.consume_identifier("Expected parameter name")?;
        if first.as_str() == "_" {
            let name = self.consume_identifier("Expected internal parameter name after '_'")?;
            return Ok((ParamLabel::Omitted, name));
        }
        if self.check_identifier_name() && !self.check(TokenKind::Colon) && !self.check(TokenKind::Comma) && !self.check(TokenKind::RParen) {
            let internal = self.consume_identifier("Expected internal parameter name after external label")?;
            return Ok((ParamLabel::Named(first), internal));
        }
        Ok((ParamLabel::Same, first))
    }

    fn parse_params(&mut self) -> SyxResult<Vec<Param>> {
        let mut params = Vec::new();
        if self.check(TokenKind::RParen) {
            return Ok(params);
        }
        loop {
            let attrs = self.parse_attributes()?;
            let isolated = self.match_kind(TokenKind::Isolated);
            let sending = self.match_kind(TokenKind::Sending);
            let (external_label, name) = self.parse_param_name()?;
            let ty = if self.match_kind(TokenKind::Colon) {
                Some(self.parse_type()?)
            } else {
                None
            };
            let default = if self.match_kind(TokenKind::Assign) {
                Some(self.parse_expr()?)
            } else {
                None
            };
            params.push(Param {
                name,
                external_label,
                ty,
                default,
                isolated,
                sending,
                attrs,
            });
            if !self.match_kind(TokenKind::Comma) {
                break;
            }
        }
        Ok(params)
    }

    fn parse_block_stmts(&mut self) -> SyxResult<Vec<Stmt>> {
        let mut stmts = Vec::new();
        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            self.skip_newlines();
            if self.check(TokenKind::RBrace) {
                break;
            }
            stmts.push(self.parse_stmt()?);
        }
        self.consume(TokenKind::RBrace, "Expected '}' after block")?;
        Ok(stmts)
    }

    fn parse_stmt_or_block(&mut self) -> SyxResult<Vec<Stmt>> {
        if self.match_kind(TokenKind::LBrace) {
            self.parse_block_stmts()
        } else {
            Ok(vec![self.parse_stmt()?])
        }
    }

    fn parse_stmt(&mut self) -> SyxResult<Stmt> {
        if matches!(self.peek().kind, TokenKind::Async) {
            if matches!(
                self.peek_ahead(1).map(|t| &t.kind),
                Some(TokenKind::Let) | Some(TokenKind::Var)
            ) {
                return self.parse_async_let_decl();
            }
        }
        if matches!(self.peek().kind, TokenKind::Weak) {
            if matches!(
                self.peek_ahead(1).map(|t| &t.kind),
                Some(TokenKind::Let) | Some(TokenKind::Var)
            ) {
                return self.parse_weak_var_decl();
            }
        }
        match &self.peek().kind {
            TokenKind::Var | TokenKind::Let => self.parse_var_decl(),
            TokenKind::Return => self.parse_return(),
            TokenKind::If => self.parse_if_or_if_let(),
            TokenKind::Guard => self.parse_guard_let(),
            TokenKind::Switch => self.parse_switch(),
            TokenKind::Select => self.parse_select(),
            TokenKind::While => self.parse_while(None),
            TokenKind::For => self.parse_for_in(None),
            TokenKind::Try => self.parse_try(),
            TokenKind::Throw => {
                let start = self.advance().span;
                let expr = self.parse_expr()?;
                self.skip_newlines();
                Ok(Stmt::Throw {
                    span: Span::merge(start, expr_span(&expr)),
                    expr,
                })
            }
            TokenKind::Repeat => self.parse_repeat_while(None),
            TokenKind::Hash => self.parse_macro_invoke(),
            TokenKind::Break => {
                let span = self.advance().span;
                let label = match &self.peek().kind {
                    TokenKind::Identifier(name) => {
                        let n = name.clone();
                        self.advance();
                        Some(n)
                    }
                    _ => None,
                };
                self.skip_newlines();
                Ok(Stmt::Break { span, label })
            }
            TokenKind::Continue => {
                let span = self.advance().span;
                let label = match &self.peek().kind {
                    TokenKind::Identifier(name) => {
                        let n = name.clone();
                        self.advance();
                        Some(n)
                    }
                    _ => None,
                };
                self.skip_newlines();
                Ok(Stmt::Continue { span, label })
            }
            TokenKind::LBrace => {
                let start = self.advance().span;
                let stmts = self.parse_block_stmts()?;
                Ok(Stmt::Block {
                    span: Span::merge(start, self.previous().span),
                    stmts,
                })
            }
            TokenKind::Identifier(_) => {
                // S1-I8: `outer: for` / `outer: while` / `outer: repeat`
                if matches!(
                    self.peek_ahead(1).map(|t| &t.kind),
                    Some(TokenKind::Colon)
                ) && matches!(
                    self.peek_ahead(2).map(|t| &t.kind),
                    Some(TokenKind::For) | Some(TokenKind::While) | Some(TokenKind::Repeat)
                ) {
                    let label = self.consume_identifier("Expected loop label")?;
                    self.consume(TokenKind::Colon, "Expected ':' after loop label")?;
                    self.skip_newlines();
                    return match &self.peek().kind {
                        TokenKind::For => self.parse_for_in(Some(label)),
                        TokenKind::While => self.parse_while(Some(label)),
                        TokenKind::Repeat => self.parse_repeat_while(Some(label)),
                        _ => Err(SyxDiagnostic::error("Expected for/while/repeat after label")
                            .with_code("P001")),
                    };
                }
                let expr = self.parse_expr()?;
                let span = expr_span(&expr);
                if self.match_kind(TokenKind::Semicolon) {
                    // optional semicolon
                }
                self.skip_newlines();
                if is_print_call(&expr) {
                    if let Expr::Call { args, .. } = expr {
                        return Ok(Stmt::Print {
                            span,
                            expr: args
                                .first()
                                .map(|a| a.value.clone())
                                .unwrap_or(Expr::Literal {
                                    span,
                                    value: Literal::String(String::new()),
                                }),
                        });
                    }
                }
                Ok(Stmt::Expr { span, expr })
            }
            _ => {
                let expr = self.parse_expr()?;
                let span = expr_span(&expr);
                if self.match_kind(TokenKind::Semicolon) {
                    // optional semicolon
                }
                self.skip_newlines();
                if let Expr::Ident { name, .. } = &expr {
                    if self.previous_non_newline().kind == TokenKind::Identifier(name.clone()) {
                        // check if it's print call pattern handled in expr
                    }
                }
                if is_print_call(&expr) {
                    if let Expr::Call { args, .. } = expr {
                        return Ok(Stmt::Print {
                            span,
                            expr: args
                                .first()
                                .map(|a| a.value.clone())
                                .unwrap_or(Expr::Literal {
                                    span,
                                    value: Literal::String(String::new()),
                                }),
                        });
                    }
                }
                Ok(Stmt::Expr { span, expr })
            }
        }
    }

    fn parse_macro_invoke(&mut self) -> SyxResult<Stmt> {
        let start = self.advance().span;
        let name = self.consume_macro_name("Expected macro name after '#")?;
        self.consume(TokenKind::LParen, "Expected '(' after macro name")?;
        let mut args = Vec::new();
        if !self.check(TokenKind::RParen) {
            loop {
                args.push(self.parse_expr()?);
                if !self.match_kind(TokenKind::Comma) {
                    break;
                }
            }
        }
        self.consume(TokenKind::RParen, "Expected ')' after macro arguments")?;
        self.skip_newlines();
        Ok(Stmt::MacroInvoke {
            span: Span::merge(start, self.previous().span),
            name,
            args,
        })
    }

    fn parse_async_let_decl(&mut self) -> SyxResult<Stmt> {
        let start = self.advance().span;
        let _async_kw = start;
        let mutable = matches!(self.advance().kind, TokenKind::Var);
        let name = self.consume_identifier("Expected variable name after async let")?;
        let ty = if self.match_kind(TokenKind::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };
        let init = if self.match_kind(TokenKind::Assign) {
            let parsed = self.parse_expr()?;
            Some(Self::desugar_async_let_init(parsed))
        } else {
            None
        };
        self.skip_newlines();
        Ok(Stmt::VarDecl {
            span: Span::merge(start, self.previous().span),
            mutable,
            name,
            ty,
            init,
        })
    }

    fn desugar_async_let_init(init: Expr) -> Expr {
        let Expr::Call {
            span,
            callee,
            args,
        } = init
        else {
            return init;
        };
        let Expr::Ident { name, .. } = callee.as_ref() else {
            return Expr::Call { span, callee, args };
        };
        if name.as_str().starts_with("__syx_") {
            return Expr::Call { span, callee, args };
        }
        let fn_name = name.clone();
        let arg_values: Vec<Expr> = args.into_iter().map(|a| a.value).collect();
        Expr::Call {
            span,
            callee: Box::new(Expr::Ident {
                span,
                name: "__syx_async_let_detach".into(),
            }),
            args: vec![
                CallArg {
                    label: None,
                    value: Expr::Literal {
                        span,
                        value: Literal::String(fn_name.into()),
                    },
                },
                CallArg {
                    label: None,
                    value: Expr::Array { span, elements: arg_values },
                },
            ],
        }
    }

    fn peek_ahead(&self, n: usize) -> Option<&Token> {
        let mut idx = self.current;
        let mut seen = 0usize;
        while idx < self.tokens.len() {
            if Self::is_trivia_kind(&self.tokens[idx].kind) {
                idx += 1;
                continue;
            }
            if seen == n {
                return Some(&self.tokens[idx]);
            }
            seen += 1;
            idx += 1;
        }
        None
    }

    fn parse_var_decl(&mut self) -> SyxResult<Stmt> {
        let start = self.advance().span;
        let mutable = matches!(self.previous().kind, TokenKind::Var);
        if self.match_kind(TokenKind::LParen) {
            return self.parse_tuple_var_decl(start, mutable);
        }
        let name = self.consume_identifier("Expected variable name")?;
        let ty = if self.match_kind(TokenKind::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };
        let init = if self.match_kind(TokenKind::Assign) {
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Stmt::VarDecl {
            span: Span::merge(start, self.previous().span),
            mutable,
            name,
            ty,
            init,
        })
    }

    fn parse_tuple_var_decl(&mut self, start: Span, mutable: bool) -> SyxResult<Stmt> {
        let mut bindings = Vec::new();
        if !self.check(TokenKind::RParen) {
            loop {
                if self.match_kind(TokenKind::Let) {
                    bindings.push(self.consume_identifier("Expected binding name in tuple pattern")?);
                } else {
                    bindings.push(self.consume_identifier("Expected binding name in tuple pattern")?);
                }
                if !self.match_kind(TokenKind::Comma) {
                    break;
                }
            }
        }
        self.consume(TokenKind::RParen, "Expected ')' after tuple pattern")?;
        let ty = if self.match_kind(TokenKind::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };
        let init = if self.match_kind(TokenKind::Assign) {
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.skip_newlines();
        let end = self.previous().span;
        let tmp = SmolStr::new(format!("__tuple_{}_{}", start.start, bindings.len()));
        let mut stmts = vec![Stmt::VarDecl {
            span: Span::merge(start, end),
            mutable: false,
            name: tmp.clone(),
            ty,
            init,
        }];
        for (i, bind) in bindings.iter().enumerate() {
            stmts.push(Stmt::VarDecl {
                span: Span::merge(start, end),
                mutable,
                name: bind.clone(),
                ty: None,
                init: Some(Expr::Member {
                    span: end,
                    object: Box::new(Expr::Ident {
                        span: end,
                        name: tmp.clone(),
                    }),
                    name: SmolStr::new(i.to_string()),
                }),
            });
        }
        Ok(Stmt::Seq {
            span: Span::merge(start, end),
            stmts,
        })
    }

    /// Phase 103: `weak var` / `weak let` — init via `__syx_weak_from` without retaining.
    fn parse_weak_var_decl(&mut self) -> SyxResult<Stmt> {
        let _weak_kw = self.advance();
        let mut decl = self.parse_var_decl()?;
        if let Stmt::VarDecl {
            init: Some(expr),
            span,
            name,
            mutable,
            ty,
            ..
        } = decl
        {
            decl = Stmt::VarDecl {
                span,
                mutable,
                name,
                ty,
                init: Some(Expr::Call {
                    span,
                    callee: Box::new(Expr::Ident {
                        span,
                        name: SmolStr::new("__syx_weak_from"),
                    }),
                    args: vec![syx_ast::CallArg {
                        label: None,
                        value: expr,
                    }],
                }),
            };
        }
        Ok(decl)
    }

    fn parse_return(&mut self) -> SyxResult<Stmt> {
        let start = self.advance().span;
        let value = if self.check_expr_start() {
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.skip_newlines();
        Ok(Stmt::Return {
            span: Span::merge(start, self.previous().span),
            value,
        })
    }

    fn parse_if_or_if_let(&mut self) -> SyxResult<Stmt> {
        let start = self.advance().span;
        if self.match_kind(TokenKind::Case) {
            let pattern = self.parse_switch_pattern()?;
            self.consume(TokenKind::Assign, "Expected '=' in if case")?;
            let subject = self.parse_expr()?;
            let body = self.parse_stmt_or_block()?;
            let else_branch = if self.match_kind(TokenKind::Else) {
                Some(self.parse_stmt_or_block()?)
            } else {
                None
            };
            return Ok(Stmt::IfCase {
                span: Span::merge(start, self.previous().span),
                pattern,
                subject,
                body,
                else_branch,
            });
        }
        if self.match_kind(TokenKind::Let) {
            let name = self.consume_identifier("Expected binding name")?;
            self.consume(TokenKind::Assign, "Expected '=' in if let")?;
            let pattern = self.parse_expr()?;
            let body = self.parse_stmt_or_block()?;
            let else_branch = if self.match_kind(TokenKind::Else) {
                Some(self.parse_stmt_or_block()?)
            } else {
                None
            };
            return Ok(Stmt::IfLet {
                span: Span::merge(start, self.previous().span),
                name,
                pattern,
                body,
                else_branch,
            });
        }
        // S1-I8: `if obj.prop { … }` — do not treat the then-brace as trailing closure.
        let prev = self.allow_trailing_closure;
        self.allow_trailing_closure = false;
        let condition = self.parse_expr()?;
        self.allow_trailing_closure = prev;
        let then_branch = self.parse_stmt_or_block()?;
        let else_branch = if self.match_kind(TokenKind::Else) {
            Some(self.parse_stmt_or_block()?)
        } else {
            None
        };
        Ok(Stmt::If {
            span: Span::merge(start, self.previous().span),
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_guard_let(&mut self) -> SyxResult<Stmt> {
        let start = self.advance().span;
        self.consume(TokenKind::Let, "Expected 'let' after guard")?;
        let name = self.consume_identifier("Expected binding name")?;
        self.consume(TokenKind::Assign, "Expected '=' in guard let")?;
        let pattern = self.parse_expr()?;
        self.consume(TokenKind::Else, "Expected 'else' after guard let")?;
        self.consume(TokenKind::LBrace, "Expected '{' after guard else")?;
        let else_branch = self.parse_block_stmts()?;
        // S1-I5: book/Swift style has no success brace — continuation is following stmts.
        // Legacy SYX form still accepts an optional `{ … }` success body after `else`.
        let body = if self.check(TokenKind::LBrace) {
            self.advance();
            self.parse_block_stmts()?
        } else {
            Vec::new()
        };
        Ok(Stmt::GuardLet {
            span: Span::merge(start, self.previous().span),
            name,
            pattern,
            body,
            else_branch,
        })
    }

    fn parse_switch(&mut self) -> SyxResult<Stmt> {
        let start = self.advance().span;
        let subject = self.parse_expr()?;
        self.consume(TokenKind::LBrace, "Expected '{' after switch")?;
        let mut cases = Vec::new();
        let mut default = None;
        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            self.skip_newlines();
            if self.match_kind(TokenKind::Case) {
                // S1-I5: `case 2, 3, 5` multi-pattern → duplicate arms sharing body/guard.
                let mut patterns = vec![self.parse_switch_pattern()?];
                while self.match_kind(TokenKind::Comma) {
                    patterns.push(self.parse_switch_pattern()?);
                }
                let guard = if self.match_kind(TokenKind::Where) {
                    Some(self.parse_expr()?)
                } else {
                    None
                };
                // Book style `case .ok { … }` or classic `case .ok: …`.
                let stmts = if self.match_kind(TokenKind::LBrace) {
                    self.parse_block_stmts()?
                } else {
                    self.consume(TokenKind::Colon, "Expected ':' or '{' after case")?;
                    let mut stmts = Vec::new();
                    while !self.check(TokenKind::Case)
                        && !self.check(TokenKind::Default)
                        && !self.check(TokenKind::RBrace)
                    {
                        self.skip_newlines();
                        if self.check(TokenKind::Case)
                            || self.check(TokenKind::Default)
                            || self.check(TokenKind::RBrace)
                        {
                            break;
                        }
                        stmts.push(self.parse_stmt()?);
                    }
                    stmts
                };
                for pattern in patterns {
                    cases.push(syx_ast::SwitchCase {
                        pattern,
                        guard: guard.clone(),
                        body: stmts.clone(),
                    });
                }
            } else if self.match_kind(TokenKind::Default) {
                let stmts = if self.match_kind(TokenKind::LBrace) {
                    self.parse_block_stmts()?
                } else {
                    self.consume(TokenKind::Colon, "Expected ':' or '{' after default")?;
                    let mut stmts = Vec::new();
                    while !self.check(TokenKind::RBrace) {
                        self.skip_newlines();
                        if self.check(TokenKind::RBrace) {
                            break;
                        }
                        stmts.push(self.parse_stmt()?);
                    }
                    stmts
                };
                default = Some(stmts);
            } else {
                break;
            }
        }
        self.consume(TokenKind::RBrace, "Expected '}' after switch")?;
        Ok(Stmt::Switch {
            span: Span::merge(start, self.previous().span),
            subject,
            cases,
            default,
        })
    }

    /// Phase 86/105/106/107: `select { case v <- ch: recv | case ch <- expr: send | default: ... }`.
    fn parse_select(&mut self) -> SyxResult<Stmt> {
        let start = self.advance().span;
        self.consume(TokenKind::LBrace, "Expected '{' after select")?;
        let mut cases: Vec<SelectCase> = Vec::new();
        let mut default_body: Option<Vec<Stmt>> = None;
        let mut timeout_arm: Option<(Expr, Vec<Stmt>)> = None;
        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            self.skip_newlines();
            if self.match_kind(TokenKind::Case) {
                let left = self.consume_identifier("Expected identifier in select case")?;
                let left_span = self.previous().span;
                if !self.match_kind(TokenKind::Less) {
                    return self.error("Expected '<-' in select case");
                }
                if !self.match_kind(TokenKind::Minus) {
                    return self.error("Expected '<-' in select case");
                }
                let right = self.parse_expr()?;
                let kind = classify_select_case(left, left_span, right);
                self.consume(TokenKind::Colon, "Expected ':' after select case")?;
                let mut stmts = Vec::new();
                while !self.check(TokenKind::Case)
                    && !self.check(TokenKind::Default)
                    && !self.check(TokenKind::Timeout)
                    && !self.check(TokenKind::Deadline)
                    && !self.check(TokenKind::RBrace)
                {
                    self.skip_newlines();
                    if self.check(TokenKind::Case)
                        || self.check(TokenKind::Default)
                        || self.check(TokenKind::Timeout)
                        || self.check(TokenKind::Deadline)
                        || self.check(TokenKind::RBrace)
                    {
                        break;
                    }
                    stmts.push(self.parse_stmt()?);
                }
                cases.push(SelectCase { kind, body: stmts });
            } else if self.match_kind(TokenKind::Timeout) {
                self.consume(TokenKind::LParen, "Expected '(' after timeout")?;
                let ms = self.parse_expr()?;
                self.consume(TokenKind::RParen, "Expected ')' after timeout(ms)")?;
                self.consume(TokenKind::Colon, "Expected ':' after timeout(ms)")?;
                let mut stmts = Vec::new();
                while !self.check(TokenKind::Case)
                    && !self.check(TokenKind::Default)
                    && !self.check(TokenKind::Timeout)
                    && !self.check(TokenKind::Deadline)
                    && !self.check(TokenKind::RBrace)
                {
                    self.skip_newlines();
                    if self.check(TokenKind::RBrace) {
                        break;
                    }
                    stmts.push(self.parse_stmt()?);
                }
                timeout_arm = Some((ms, stmts));
            } else if self.match_kind(TokenKind::Deadline) {
                let ms = self.parse_expr()?;
                self.consume(TokenKind::Colon, "Expected ':' after deadline")?;
                let mut stmts = Vec::new();
                while !self.check(TokenKind::Case)
                    && !self.check(TokenKind::Default)
                    && !self.check(TokenKind::Timeout)
                    && !self.check(TokenKind::Deadline)
                    && !self.check(TokenKind::RBrace)
                {
                    self.skip_newlines();
                    if self.check(TokenKind::RBrace) {
                        break;
                    }
                    stmts.push(self.parse_stmt()?);
                }
                timeout_arm = Some((ms, stmts));
            } else if self.match_kind(TokenKind::Default) {
                self.consume(TokenKind::Colon, "Expected ':' after default")?;
                let mut stmts = Vec::new();
                while !self.check(TokenKind::RBrace) {
                    self.skip_newlines();
                    if self.check(TokenKind::RBrace) {
                        break;
                    }
                    stmts.push(self.parse_stmt()?);
                }
                default_body = Some(stmts);
            } else {
                break;
            }
        }
        self.consume(TokenKind::RBrace, "Expected '}' after select")?;
        let span = Span::merge(start, self.previous().span);
        let stmts = self.lower_select_cases(span, &cases, default_body, timeout_arm)?;
        Ok(Stmt::Block { span, stmts })
    }

    fn lower_select_cases(
        &self,
        span: Span,
        cases: &[SelectCase],
        default_body: Option<Vec<Stmt>>,
        timeout_arm: Option<(Expr, Vec<Stmt>)>,
    ) -> SyxResult<Vec<Stmt>> {
        if cases.is_empty() {
            return Ok(timeout_arm.map(|(_, b)| b).or(default_body).unwrap_or_default());
        }
        if cases.len() == 1 && timeout_arm.is_none() {
            return Ok(self.lower_one_select_case(span, &cases[0], default_body));
        }

        // Phase 108: ≥2 cases (recv-only or mixed send+recv) lower to one
        // `__syx_channel_select` / `__syx_channel_select_default` array poll
        // (`select-mixed-unified-v1:tagged-array-poll`). nested if/else chain
        // (never `while true` + Break).
        // Mixed / send arms use `__syx_select_tag(dir, ch, payload)` into temps
        // then a homogeneous Object[] poll array (S014). Nested CallHost inside
        // ArrayBuild corrupts the MIR stack, so arms are bound first.
        // Host equivalent: `[dir, channel, payload?]` (dir 0=recv, 1=send).
        // Phase 109: case bodies may contain `await` (`select-mixed-await-v1:interleave`).
        // Phase 110: mixed+await uses cancel/deadline gate (`select-mixed-timeout-v1:cancel-or-deadline`).
        // Phase 112: mixed+await + syntax timeout/deadline unifies language ms over env.
        // Phase 113: env fallback hard-banned; mixed+await without timeout/deadline does not compile
        // (`select-deadline-strict-v1:no-env`).
        let _ = SELECT_MIXED_UNIFIED_FINGERPRINT;
        let _ = SELECT_MIXED_AWAIT_FINGERPRINT;
        let mixed_await = select_cases_have_await(cases);
        if mixed_await {
            let _ = SELECT_MIXED_TIMEOUT_FINGERPRINT;
        }
        let lang_timeout_ms = timeout_arm.as_ref().map(|(ms, _)| ms.clone());
        if mixed_await && lang_timeout_ms.is_none() {
            return Err(SyxDiagnostic::error(
                "mixed select + await requires timeout(ms): or deadline ms: arm (SYX_SELECT_MIXED_TIMEOUT_MS env fallback removed)",
            )
            .with_code("P001")
            .with_span(&self.source, span, "select"));
        }
        if timeout_arm.is_some() {
            let _ = SELECT_TIMEOUT_SYNTAX_FINGERPRINT;
            if mixed_await {
                let _ = SELECT_DEADLINE_UNIFY_FINGERPRINT;
                let _ = SELECT_DEADLINE_STRICT_FINGERPRINT;
            }
        }
        let mixed_await_lang_timeout = mixed_await && lang_timeout_ms.is_some();
        let has_send = cases
            .iter()
            .any(|c| matches!(c.kind, SelectCaseKind::Send { .. }));

        // Unified indexed `__syx_channel_select_default` array poll for all multi-case.
        let miss = select_default_miss_expr(span);
        let mut stmts = Vec::new();
        let elements: Vec<Expr> = if has_send {
            let mut arms = Vec::with_capacity(cases.len());
            for (i, c) in cases.iter().enumerate() {
                let arm_name = SmolStr::new(format!("__syx_sel_arm_{i}"));
                stmts.push(Stmt::VarDecl {
                    span,
                    mutable: false,
                    name: arm_name.clone(),
                    ty: None,
                    init: Some(select_tagged_arm_expr(span, c)),
                });
                arms.push(Expr::Ident {
                    span,
                    name: arm_name,
                });
            }
            arms
        } else {
            cases
                .iter()
                .map(|c| match &c.kind {
                    SelectCaseKind::Recv { channel, .. }
                    | SelectCaseKind::Send { channel, .. } => channel.clone(),
                })
                .collect()
        };

        stmts.push(Stmt::VarDecl {
            span,
            mutable: false,
            name: SmolStr::new("__syx_sel_arr"),
            ty: None,
            init: Some(Expr::Array {
                span,
                elements,
            }),
        });
        let _select_surface = if mixed_await {
            "__syx_channel_select_mixed_await"
        } else {
            "__syx_channel_select"
        };
        let _ = _select_surface;
        let select_callee = if mixed_await {
            "__syx_channel_select_mixed_await"
        } else if timeout_arm.is_some() {
            "__syx_channel_select_timeout"
        } else {
            "__syx_channel_select_default"
        };
        let mut select_args = vec![
            syx_ast::CallArg {
                label: None,
                value: Expr::Ident {
                    span,
                    name: SmolStr::new("__syx_sel_arr"),
                },
            },
        ];
        if timeout_arm.is_some() && !mixed_await {
            select_args.push(syx_ast::CallArg {
                label: None,
                value: lang_timeout_ms.clone().expect("timeout arm ms"),
            });
        } else {
            select_args.push(syx_ast::CallArg {
                label: None,
                value: miss.clone(),
            });
            if mixed_await {
                // Phase 113: GA path is language timeout/deadline only (`select-deadline-strict-v1:no-env`).
                let _ = mixed_await_lang_timeout;
                select_args.push(syx_ast::CallArg {
                    label: None,
                    value: lang_timeout_ms
                        .clone()
                        .expect("mixed+await GA requires syntax timeout/deadline ms"),
                });
            }
        }
        stmts.push(Stmt::VarDecl {
            span,
            mutable: false,
            name: SmolStr::new("__syx_sel_res"),
            ty: None,
            init: Some(Expr::Call {
                span,
                callee: Box::new(Expr::Ident {
                    span,
                    name: SmolStr::new(select_callee),
                }),
                args: select_args,
            }),
        });

        let res_ident = Expr::Ident {
            span,
            name: SmolStr::new("__syx_sel_res"),
        };
        stmts.push(Stmt::VarDecl {
            span,
            mutable: false,
            name: SmolStr::new("__syx_sel_idx"),
            ty: None,
            init: Some(Expr::Index {
                span,
                object: Box::new(res_ident.clone()),
                index: Box::new(Expr::Literal {
                    span,
                    value: Literal::Integer {
                        value: 0,
                        suffix: None,
                    },
                }),
            }),
        });
        stmts.push(Stmt::VarDecl {
            span,
            mutable: false,
            name: SmolStr::new("__syx_sel_val"),
            ty: None,
            init: Some(Expr::Index {
                span,
                object: Box::new(res_ident.clone()),
                index: Box::new(Expr::Literal {
                    span,
                    value: Literal::Integer {
                        value: 1,
                        suffix: None,
                    },
                }),
            }),
        });
        let idx_ident = Expr::Ident {
            span,
            name: SmolStr::new("__syx_sel_idx"),
        };
        let val_ident = Expr::Ident {
            span,
            name: SmolStr::new("__syx_sel_val"),
        };

        let n = cases.len();
        let fallback_body = timeout_arm
            .map(|(_, body)| body)
            .or(default_body);
        let mut else_chain: Option<Vec<Stmt>> = fallback_body.map(|body| {
            vec![Stmt::If {
                span,
                condition: Expr::Binary {
                    span,
                    left: Box::new(idx_ident.clone()),
                    op: BinOp::Eq,
                    right: Box::new(Expr::Literal {
                        span,
                        value: Literal::String("-1".into()),
                    }),
                },
                then_branch: body,
                else_branch: None,
            }]
        });
        if else_chain.is_none() {
            else_chain = Some(vec![]);
        }
        for i in (0..n).rev() {
            let then_branch = match &cases[i].kind {
                SelectCaseKind::Recv { binding, .. } => {
                    let mut arm = vec![Stmt::VarDecl {
                        span,
                        mutable: false,
                        name: binding.clone(),
                        ty: None,
                        init: Some(val_ident.clone()),
                    }];
                    arm.extend(cases[i].body.clone());
                    arm
                }
                SelectCaseKind::Send { .. } => cases[i].body.clone(),
            };
            else_chain = Some(vec![Stmt::If {
                span,
                condition: Expr::Binary {
                    span,
                    left: Box::new(idx_ident.clone()),
                    op: BinOp::Eq,
                    right: Box::new(Expr::Literal {
                        span,
                        value: Literal::String(i.to_string()),
                    }),
                },
                then_branch,
                else_branch: else_chain.take(),
            }]);
        }
        stmts.extend(else_chain.unwrap_or_default());
        Ok(stmts)
    }

    fn lower_one_select_case(
        &self,
        span: Span,
        case: &SelectCase,
        default_body: Option<Vec<Stmt>>,
    ) -> Vec<Stmt> {
        match (&case.kind, &default_body) {
            (SelectCaseKind::Recv { binding, channel }, None) => {
                let mut stmts = vec![Stmt::VarDecl {
                    span,
                    mutable: false,
                    name: binding.clone(),
                    ty: None,
                    init: Some(Expr::Call {
                        span,
                        callee: Box::new(Expr::Ident {
                            span,
                            name: SmolStr::new("__syx_channel_recv"),
                        }),
                        args: vec![syx_ast::CallArg {
                            label: None,
                            value: channel.clone(),
                        }],
                    }),
                }];
                stmts.extend(case.body.clone());
                stmts
            }
            (SelectCaseKind::Send { channel, value }, None) => {
                let mut stmts = vec![Stmt::Expr {
                    span,
                    expr: Expr::Call {
                        span,
                        callee: Box::new(Expr::Ident {
                            span,
                            name: SmolStr::new("__syx_channel_try_send"),
                        }),
                        args: vec![
                            syx_ast::CallArg {
                                label: None,
                                value: channel.clone(),
                            },
                            syx_ast::CallArg {
                                label: None,
                                value: value.clone(),
                            },
                        ],
                    },
                }];
                stmts.extend(case.body.clone());
                stmts
            }
            _ => self
                .lower_select_case_arm(span, case, select_default_miss_expr(span), default_body)
                .unwrap_or_default(),
        }
    }

    fn lower_select_case_arm(
        &self,
        span: Span,
        case: &SelectCase,
        miss: Expr,
        else_branch: Option<Vec<Stmt>>,
    ) -> SyxResult<Vec<Stmt>> {
        match &case.kind {
            SelectCaseKind::Recv { binding, channel } => {
                let sel_tmp = SmolStr::new(format!("__syx_sel_tmp_{binding}"));
                let mut case_stmts = vec![Stmt::VarDecl {
                    span,
                    mutable: false,
                    name: sel_tmp.clone(),
                    ty: None,
                    init: Some(Expr::Call {
                        span,
                        callee: Box::new(Expr::Ident {
                            span,
                            name: SmolStr::new("__syx_channel_select_default"),
                        }),
                        args: vec![
                            syx_ast::CallArg {
                                label: None,
                                value: channel.clone(),
                            },
                            syx_ast::CallArg {
                                label: None,
                                value: miss.clone(),
                            },
                        ],
                    }),
                }];
                let mut then_branch = vec![Stmt::VarDecl {
                    span,
                    mutable: false,
                    name: binding.clone(),
                    ty: None,
                    init: Some(Expr::Ident {
                        span,
                        name: sel_tmp.clone(),
                    }),
                }];
                then_branch.extend(case.body.clone());
                case_stmts.push(Stmt::If {
                    span,
                    condition: Expr::Binary {
                        span,
                        left: Box::new(Expr::Ident {
                            span,
                            name: sel_tmp,
                        }),
                        op: BinOp::NotEq,
                        right: Box::new(miss),
                    },
                    then_branch,
                    else_branch,
                });
                Ok(case_stmts)
            }
            SelectCaseKind::Send { channel, value } => {
                let ok_tmp = SmolStr::new("__syx_sel_send_ok");
                let mut case_stmts = vec![Stmt::VarDecl {
                    span,
                    mutable: false,
                    name: ok_tmp.clone(),
                    ty: None,
                    init: Some(Expr::Call {
                        span,
                        callee: Box::new(Expr::Ident {
                            span,
                            name: SmolStr::new("__syx_channel_try_send"),
                        }),
                        args: vec![
                            syx_ast::CallArg {
                                label: None,
                                value: channel.clone(),
                            },
                            syx_ast::CallArg {
                                label: None,
                                value: value.clone(),
                            },
                        ],
                    }),
                }];
                let then_branch = case.body.clone();
                case_stmts.push(Stmt::If {
                    span,
                    condition: Expr::Ident {
                        span,
                        name: ok_tmp,
                    },
                    then_branch,
                    else_branch,
                });
                Ok(case_stmts)
            }
        }
    }

    fn parse_switch_pattern(&mut self) -> SyxResult<Expr> {
        // S1-I5: `case let x` / `case let x where …` — bind whole subject (`__let` marker).
        if self.match_kind(TokenKind::Let) {
            let name = self.consume_identifier("Expected binding name after 'case let'")?;
            return Ok(Expr::VariantPattern {
                span: self.previous().span,
                variant: SmolStr::new("__let"),
                bindings: vec![(None, name)],
            });
        }
        if self.match_kind(TokenKind::LParen) {
            let mut bindings = Vec::new();
            if !self.check(TokenKind::RParen) {
                loop {
                    if self.match_kind(TokenKind::Let) {
                        bindings.push(self.consume_identifier("Expected binding name in tuple pattern")?);
                    } else {
                        bindings.push(self.consume_identifier("Expected binding name in tuple pattern")?);
                    }
                    if !self.match_kind(TokenKind::Comma) {
                        break;
                    }
                }
            }
            self.consume(TokenKind::RParen, "Expected ')' after tuple pattern")?;
            return Ok(Expr::TuplePattern {
                span: self.previous().span,
                bindings,
            });
        }
        if self.match_kind(TokenKind::Dot) {
            let variant = self.consume_identifier("Expected variant name after '.'")?;
            let mut bindings = Vec::new();
            if self.match_kind(TokenKind::LParen) {
                // `.success(let data)` | `.failure(code: let c, message: let m)` | `.fail(let a, let b)`
                while !self.check(TokenKind::RParen) {
                    let label = if self.peek_is_label_colon() {
                        let lab = self.consume_identifier("Expected associated label")?;
                        self.consume(TokenKind::Colon, "Expected ':' after associated label")?;
                        Some(lab)
                    } else {
                        None
                    };
                    self.consume(TokenKind::Let, "Expected 'let' in variant binding")?;
                    let name = self.consume_identifier("Expected binding name")?;
                    bindings.push((label, name));
                    if !self.match_kind(TokenKind::Comma) {
                        break;
                    }
                }
                self.consume(TokenKind::RParen, "Expected ')' after variant binding")?;
            }
            return Ok(Expr::VariantPattern {
                span: self.previous().span,
                variant,
                bindings,
            });
        }
        if let TokenKind::Integer(start, _) = &self.peek().kind {
            let start_val = *start as i64;
            let start_span = self.peek().span;
            self.advance();
            if self.match_kind(TokenKind::DotDotLess) {
                let end = match &self.peek().kind {
                    TokenKind::Integer(v, _) => {
                        let v = *v as i64;
                        self.advance();
                        v
                    }
                    _ => return self.error("Expected integer after '..<' in range case"),
                };
                return Ok(Expr::HalfOpenRange {
                    span: Span::merge(start_span, self.previous().span),
                    start: start_val,
                    end,
                });
            }
            if self.match_kind(TokenKind::DotDotDot) {
                let end = match &self.peek().kind {
                    TokenKind::Integer(v, _) => {
                        let v = *v as i64;
                        self.advance();
                        v
                    }
                    _ => return self.error("Expected integer after '...' in range case"),
                };
                return Ok(Expr::ClosedRange {
                    span: Span::merge(start_span, self.previous().span),
                    start: start_val,
                    end,
                });
            }
            return Ok(Expr::Literal {
                span: start_span,
                value: Literal::Integer {
                    value: start_val as i128,
                    suffix: None,
                },
            });
        }
        if self.match_kind(TokenKind::Is) {
            let start = self.previous().span;
            let target = self.parse_type()?;
            return Ok(Expr::Is {
                span: Span::merge(start, self.previous().span),
                expr: Box::new(Expr::Ident {
                    span: start,
                    name: SmolStr::new("__switch_subject"),
                }),
                target,
            });
        }
        self.parse_expr()
    }

    fn parse_while(&mut self, label: Option<SmolStr>) -> SyxResult<Stmt> {
        let start = self.advance().span;
        let prev = self.allow_trailing_closure;
        self.allow_trailing_closure = false;
        let condition = self.parse_expr()?;
        self.allow_trailing_closure = prev;
        let body = self.parse_stmt_or_block()?;
        Ok(Stmt::While {
            span: Span::merge(start, self.previous().span),
            condition,
            body,
            label,
        })
    }

    fn parse_try(&mut self) -> SyxResult<Stmt> {
        let start = self.advance().span;
        self.consume(TokenKind::LBrace, "Expected '{' after try")?;
        let body = self.parse_block_stmts()?;
        self.consume(TokenKind::Catch, "Expected 'catch' after try block")?;
        self.consume(TokenKind::LParen, "Expected '(' after catch")?;
        let catch_name = self.consume_identifier("Expected catch variable name")?;
        let catch_type = if self.match_kind(TokenKind::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };
        self.consume(TokenKind::RParen, "Expected ')' after catch variable")?;
        self.consume(TokenKind::LBrace, "Expected '{' after catch")?;
        let catch_body = self.parse_block_stmts()?;
        Ok(Stmt::Try {
            span: Span::merge(start, self.previous().span),
            body,
            catch_name,
            catch_type,
            catch_body,
        })
    }

    fn parse_for_in(&mut self, label: Option<SmolStr>) -> SyxResult<Stmt> {
        let start = self.advance().span;
        let is_async_iter = self.match_kind(TokenKind::Await);
        let name = self.consume_identifier("Expected loop variable")?;
        self.consume(TokenKind::In, "Expected 'in' in for loop")?;
        // S1-I7: `for x in Counter() { … }` — do not treat the loop body brace as trailing closure.
        let prev = self.allow_trailing_closure;
        self.allow_trailing_closure = false;
        let iterable = self.parse_expr()?;
        self.allow_trailing_closure = prev;
        self.consume(TokenKind::LBrace, "Expected '{' after for-in")?;
        let body = self.parse_block_stmts()?;
        Ok(Stmt::ForIn {
            span: Span::merge(start, self.previous().span),
            name,
            iterable,
            body,
            is_async_iter,
            label,
        })
    }

    fn parse_repeat_while(&mut self, label: Option<SmolStr>) -> SyxResult<Stmt> {
        let _label = label; // accepted; RepeatWhile has no label field yet
        let start = self.advance().span;
        self.consume(TokenKind::LBrace, "Expected '{' after repeat")?;
        let body = self.parse_block_stmts()?;
        self.consume(TokenKind::While, "Expected 'while' after repeat body")?;
        let condition = self.parse_expr()?;
        Ok(Stmt::RepeatWhile {
            span: Span::merge(start, self.previous().span),
            body,
            condition,
        })
    }

    fn parse_type(&mut self) -> SyxResult<TypeExpr> {
        if self.match_kind(TokenKind::Amp) {
            let lifetime = if matches!(self.peek().kind, TokenKind::Lifetime(_)) {
                let lt = self.peek().lexeme.clone();
                self.advance();
                Some(lt)
            } else {
                None
            };
            let mutable = self.match_mut_keyword();
            let inner = self.parse_type()?;
            return Ok(TypeExpr::Reference(Box::new(inner), mutable, lifetime));
        }
        if self.match_kind(TokenKind::SelfKw) || self.match_kind(TokenKind::SelfType) {
            return Ok(TypeExpr::Named {
                name: SmolStr::new("Self"),
                generics: vec![],
            });
        }
        let ty = match &self.peek().kind {
            TokenKind::LParen => self.parse_paren_type()?,
            TokenKind::LBracket => {
                self.advance();
                let first = self.parse_type()?;
                if self.match_kind(TokenKind::Colon) {
                    let value = self.parse_type()?;
                    self.consume(TokenKind::RBracket, "Expected ']' after dictionary type")?;
                    TypeExpr::Dictionary {
                        key: Box::new(first),
                        value: Box::new(value),
                    }
                } else {
                    self.consume(TokenKind::RBracket, "Expected ']' after array type")?;
                    TypeExpr::Array(Box::new(first))
                }
            }
            _ => {
                let name = self.consume_identifier("Expected type name")?;
                if name.as_str() == "some" {
                    let inner = self.parse_type()?;
                    return Ok(if self.match_kind(TokenKind::Question) {
                        TypeExpr::Optional(Box::new(TypeExpr::OpaqueSome(Box::new(inner))))
                    } else {
                        TypeExpr::OpaqueSome(Box::new(inner))
                    });
                }
                if name.as_str() == "any" {
                    let inner = self.parse_type()?;
                    return Ok(if self.match_kind(TokenKind::Question) {
                        TypeExpr::Optional(Box::new(TypeExpr::OpaqueAny(Box::new(inner))))
                    } else {
                        TypeExpr::OpaqueAny(Box::new(inner))
                    });
                }
                let mut generics = Vec::new();
                if self.match_kind(TokenKind::Less) {
                    loop {
                        generics.push(self.parse_type()?);
                        if !self.match_kind(TokenKind::Comma) {
                            break;
                        }
                    }
                    self.consume_rangle("Expected '>' after generic arguments")?;
                }
                let mut qualified = name;
                while self.match_kind(TokenKind::Dot) {
                    let inner = self.consume_identifier("Expected nested type name after '.'")?;
                    qualified = SmolStr::new(format!("{qualified}_{inner}"));
                }
                TypeExpr::Named {
                    name: qualified,
                    generics,
                }
            }
        };
        if self.match_kind(TokenKind::Question) {
            Ok(TypeExpr::Optional(Box::new(ty)))
        } else {
            Ok(ty)
        }
    }

    fn parse_paren_type(&mut self) -> SyxResult<TypeExpr> {
        self.advance();
        if self.check(TokenKind::RParen) {
            self.advance();
            let returns = if self.match_kind(TokenKind::Arrow) {
                Some(Box::new(self.parse_type()?))
            } else {
                None
            };
            return Ok(TypeExpr::Function {
                params: vec![],
                returns,
            });
        }
        let first = self.parse_type_tuple_element()?;
        if self.match_kind(TokenKind::Comma) {
            let mut elements = vec![first];
            loop {
                elements.push(self.parse_type_tuple_element()?);
                if !self.match_kind(TokenKind::Comma) {
                    break;
                }
            }
            self.consume(TokenKind::RParen, "Expected ')' after tuple type")?;
            if self.match_kind(TokenKind::Arrow) {
                let returns = Some(Box::new(self.parse_type()?));
                let params: Vec<TypeExpr> = elements.into_iter().map(|(_, ty)| ty).collect();
                return Ok(TypeExpr::Function { params, returns });
            }
            return Ok(TypeExpr::Tuple { elements });
        }
        self.consume(TokenKind::RParen, "Expected ')' after type")?;
        if self.match_kind(TokenKind::Arrow) {
            let returns = Some(Box::new(self.parse_type()?));
            return Ok(TypeExpr::Function {
                params: vec![first.1],
                returns,
            });
        }
        Ok(first.1)
    }

    fn parse_type_tuple_element(&mut self) -> SyxResult<(Option<SmolStr>, TypeExpr)> {
        let label = if self.check_identifier_name()
            && self.peek_ahead(1).is_some_and(|t| t.kind == TokenKind::Colon)
        {
            let name = self.consume_identifier("Expected tuple element label")?;
            self.consume(TokenKind::Colon, "Expected ':' after tuple element label")?;
            Some(name)
        } else {
            None
        };
        let ty = self.parse_type()?;
        Ok((label, ty))
    }

    fn check_identifier_name(&self) -> bool {
        matches!(&self.peek().kind, TokenKind::Identifier(_))
            || Self::soft_keyword_ident(&self.peek().kind).is_some()
    }

    fn parse_paren_expr(&mut self, start_span: Span) -> SyxResult<Expr> {
        if self.check(TokenKind::RParen) {
            return self.error("Expected expression in parentheses");
        }
        let first = self.parse_tuple_literal_element()?;
        if self.match_kind(TokenKind::Comma) {
            let mut elements = vec![first];
            loop {
                elements.push(self.parse_tuple_literal_element()?);
                if !self.match_kind(TokenKind::Comma) {
                    break;
                }
            }
            self.consume(TokenKind::RParen, "Expected ')' after tuple literal")?;
            return Ok(Expr::Tuple {
                span: Span::merge(start_span, self.previous().span),
                elements,
            });
        }
        if first.0.is_some() {
            self.consume(TokenKind::RParen, "Expected ')' after tuple literal")?;
            return Ok(Expr::Tuple {
                span: Span::merge(start_span, self.previous().span),
                elements: vec![first],
            });
        }
        self.consume(TokenKind::RParen, "Expected ')' after expression")?;
        Ok(Expr::Group {
            span: Span::merge(start_span, self.previous().span),
            expr: Box::new(first.1),
        })
    }

    fn parse_tuple_literal_element(&mut self) -> SyxResult<(Option<SmolStr>, Expr)> {
        let label = if self.check_identifier_name()
            && self.peek_ahead(1).is_some_and(|t| t.kind == TokenKind::Colon)
        {
            let name = self.consume_identifier("Expected tuple element label")?;
            self.consume(TokenKind::Colon, "Expected ':' after tuple element label")?;
            Some(name)
        } else {
            None
        };
        let expr = self.parse_expr()?;
        Ok((label, expr))
    }

    /// S1-I9: contents of `[…]` in `recv[…]` — one expr, or comma list / labeled args as Tuple.
    fn parse_index_contents(&mut self) -> SyxResult<Expr> {
        let start = self.peek().span;
        let first = self.parse_tuple_literal_element()?;
        if !self.check(TokenKind::Comma) {
            // Single unlabeled index stays a bare expression (Array/Dict paths).
            if first.0.is_none() {
                return Ok(first.1);
            }
            return Ok(Expr::Tuple {
                span: Span::merge(start, expr_span(&first.1)),
                elements: vec![first],
            });
        }
        let mut elements = vec![first];
        while self.match_kind(TokenKind::Comma) {
            elements.push(self.parse_tuple_literal_element()?);
        }
        Ok(Expr::Tuple {
            span: Span::merge(start, self.previous().span),
            elements,
        })
    }

    fn consume_member_name(&mut self, context: &str) -> SyxResult<SmolStr> {
        match &self.peek().kind {
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            // Soft keywords as members: `self.data`, `obj.type`, …
            kind if Self::soft_keyword_ident(kind).is_some() => {
                let name = SmolStr::new(Self::soft_keyword_ident(kind).unwrap());
                self.advance();
                Ok(name)
            }
            TokenKind::Integer(v, _) => {
                let name = SmolStr::new(v.to_string());
                self.advance();
                Ok(name)
            }
            _ => self.error(&format!("Expected member name{context}")),
        }
    }

    /// True when `peek` is `?` and the following token looks like a ternary then-branch.
    fn looks_like_ternary_tail(&self) -> bool {
        if !matches!(self.peek().kind, TokenKind::Question) {
            return false;
        }
        matches!(
            self.peek_non_trivia_kind(1),
            Some(
                TokenKind::Identifier(_)
                    | TokenKind::Integer(_, _)
                    | TokenKind::Double(_, _)
                    | TokenKind::String(_)
                    | TokenKind::Bool(_)
                    | TokenKind::Nil
                    | TokenKind::LParen
                    | TokenKind::LBrace
                    | TokenKind::LBracket
                    | TokenKind::Minus
                    | TokenKind::Bang
            )
        )
    }

    fn parse_expr(&mut self) -> SyxResult<Expr> {
        // S1-I7: `let label = if x > 0 { "pos" } else { "non-pos" }`
        if self.check(TokenKind::If) {
            return self.parse_if_expr();
        }
        // S1-I7: `a ? b : c` after comparison/or chain (not postfix `?` try).
        let condition = self.parse_precedence(0)?;
        if !self.looks_like_ternary_tail() {
            return Ok(condition);
        }
        self.advance(); // consume `?`
        let then_branch = self.parse_expr()?;
        self.consume(TokenKind::Colon, "Expected ':' in ternary expression")?;
        let else_branch = self.parse_expr()?;
        Ok(Expr::Ternary {
            span: Span::merge(expr_span(&condition), expr_span(&else_branch)),
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        })
    }

    /// S1-I7: if-expression → Ternary (single-expr branches in braces).
    fn parse_if_expr(&mut self) -> SyxResult<Expr> {
        let start = self.advance().span; // `if`
        let condition = self.parse_precedence(0)?;
        self.consume(TokenKind::LBrace, "Expected '{' after if-expression condition")?;
        let then_branch = self.parse_expr()?;
        self.consume(TokenKind::RBrace, "Expected '}' after if-expression then branch")?;
        self.consume(TokenKind::Else, "if-expression requires else branch")?;
        self.consume(TokenKind::LBrace, "Expected '{' after else in if-expression")?;
        let else_branch = self.parse_expr()?;
        self.consume(TokenKind::RBrace, "Expected '}' after if-expression else branch")?;
        Ok(Expr::Ternary {
            span: Span::merge(start, self.previous().span),
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        })
    }

    fn parse_precedence(&mut self, min_prec: u8) -> SyxResult<Expr> {
        let mut left = self.parse_unary()?;
        loop {
            let Some((op, prec)) = infix_op(&self.peek().kind) else {
                break;
            };
            if prec < min_prec {
                break;
            }
            self.advance();
            let right = self.parse_precedence(prec + 1)?;
            let span = Span::merge(expr_span(&left), expr_span(&right));
            left = Expr::Binary {
                span,
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> SyxResult<Expr> {
        match &self.peek().kind {
            TokenKind::Await => {
                let token = self.advance().clone();
                let expr = self.parse_unary()?;
                Ok(Expr::Await {
                    span: Span::merge(token.span, expr_span(&expr)),
                    expr: Box::new(expr),
                })
            }
            TokenKind::Try => {
                let token = self.advance().clone();
                // S1-I10: `try?` / `try!` — optional / IUO try (same UnaryOp::Try for check).
                if matches!(
                    self.peek().kind,
                    TokenKind::Question | TokenKind::Bang
                ) {
                    self.advance();
                }
                let expr = self.parse_unary()?;
                Ok(Expr::Unary {
                    span: Span::merge(token.span, expr_span(&expr)),
                    op: UnaryOp::Try,
                    expr: Box::new(expr),
                })
            }
            TokenKind::Minus | TokenKind::Bang | TokenKind::Tilde => {
                let token = self.advance().clone();
                let op = match token.kind {
                    TokenKind::Minus => UnaryOp::Neg,
                    TokenKind::Tilde => UnaryOp::BitNot,
                    _ => UnaryOp::Not,
                };
                let expr = self.parse_unary()?;
                Ok(Expr::Unary {
                    span: Span::merge(token.span, expr_span(&expr)),
                    op,
                    expr: Box::new(expr),
                })
            }
            TokenKind::Amp => {
                let token = self.advance().clone();
                let mutable = self.match_mut_keyword();
                let expr = self.parse_unary()?;
                Ok(Expr::Borrow {
                    span: Span::merge(token.span, expr_span(&expr)),
                    expr: Box::new(expr),
                    mutable,
                })
            }
            TokenKind::Backslash => {
                let token = self.advance().clone();
                // `\Array<Int>.[0]` / `\Person.name` / `\Box.value?.description` / `\Type?.prop`
                let mut root_type = self.consume_identifier("Expected identifier after '\\'")?;
                if self.match_kind(TokenKind::Less) {
                    let mut generics = String::new();
                    loop {
                        let g = self.parse_type()?;
                        if !generics.is_empty() {
                            generics.push_str(", ");
                        }
                        generics.push_str(&type_expr_keypath_name(&g));
                        if !self.match_kind(TokenKind::Comma) {
                            break;
                        }
                    }
                    self.consume_rangle("Expected '>' after key path generic arguments")?;
                    root_type = SmolStr::new(format!("{root_type}<{generics}>"));
                }
                let mut optional = false;
                // Root optional: `\Type?.prop` (Question before the first path Dot).
                if matches!(self.peek().kind, TokenKind::Question) {
                    optional = true;
                    self.advance();
                }
                if !self.check(TokenKind::Dot) {
                    return Ok(Expr::Ident {
                        span: Span::merge(token.span, self.previous().span),
                        name: smol_str::SmolStr::new(format!("\\{root_type}")),
                    });
                }
                self.consume(TokenKind::Dot, "Expected '.' in key path")?;
                let mut property = self.parse_keypath_segment()?;
                loop {
                    // Mid-path optional: `value?.description`
                    if matches!(self.peek().kind, TokenKind::Question)
                        && matches!(self.peek_nth(1).kind, TokenKind::Dot)
                    {
                        optional = true;
                        self.advance(); // ?
                        self.advance(); // .
                        let next = self.parse_keypath_segment()?;
                        property = SmolStr::new(format!("{property}.{next}"));
                        continue;
                    }
                    if !self.match_kind(TokenKind::Dot) {
                        break;
                    }
                    let next = self.parse_keypath_segment()?;
                    property = SmolStr::new(format!("{property}.{next}"));
                }
                Ok(Expr::KeyPath {
                    span: Span::merge(token.span, self.previous().span),
                    root_type,
                    property,
                    optional,
                })
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> SyxResult<Expr> {
        let mut expr = self.parse_primary()?;
        if let Expr::Ident { name, span } = &expr {
            if let Some(op) = compound_assign_op(&self.peek().kind) {
                self.advance();
                let value = self.parse_expr()?;
                return Ok(Expr::CompoundAssign {
                    span: Span::merge(*span, expr_span(&value)),
                    name: name.clone(),
                    op,
                    value: Box::new(value),
                });
            }
        }
        loop {
            match &self.peek().kind {
                // S1-I6: `Holder<Int>(…)` / `Holder<Int>()` — type args before call;
                // only commit when `<…>` is followed by `(`, else leave `<` for binary.
                TokenKind::Less if matches!(&expr, Expr::Ident { .. }) => {
                    let save = self.current;
                    if self.try_skip_generic_type_args() && self.check(TokenKind::LParen) {
                        // Type args consumed; next iteration builds Call on bare Ident.
                        continue;
                    }
                    self.current = save;
                    break;
                }
                TokenKind::LParen => {
                    self.advance();
                    let args = self.parse_args()?;
                    self.consume(TokenKind::RParen, "Expected ')' after arguments")?;
                    let span = Span::merge(expr_span(&expr), self.previous().span);
                    expr = Expr::Call {
                        span,
                        callee: Box::new(expr),
                        args,
                    };
                }
                TokenKind::Dot => {
                    self.advance();
                    if self.match_kind(TokenKind::SelfKw) || self.match_kind(TokenKind::SelfType) {
                        if let Expr::Ident { name, span: type_span } = expr {
                            let span = Span::merge(type_span, self.previous().span);
                            expr = Expr::TypeSelf {
                                span,
                                type_name: name,
                            };
                        } else {
                            return self.error("`.self` is only valid on a type name (e.g. Point.self)");
                        }
                    } else {
                        let name = self.consume_member_name("")?;
                        let span = Span::merge(expr_span(&expr), self.previous().span);
                        expr = Expr::Member {
                            span,
                            object: Box::new(expr),
                            name,
                        };
                    }
                }
                TokenKind::Question if matches!(self.tokens.get(self.peek_index() + 1).map(|t| &t.kind), Some(TokenKind::Dot)) => {
                    self.advance();
                    self.advance();
                    let name = self.consume_member_name(" after ?.")?;
                    let span = Span::merge(expr_span(&expr), self.previous().span);
                    expr = Expr::OptionalChain {
                        span,
                        object: Box::new(expr),
                        name,
                    };
                }
                // S1-I7: bare `?` is postfix Try only when not starting `cond ? then : else`.
                TokenKind::Question if !self.looks_like_ternary_tail() => {
                    let token = self.advance().clone();
                    let span = Span::merge(expr_span(&expr), token.span);
                    expr = Expr::Unary {
                        span,
                        op: UnaryOp::Try,
                        expr: Box::new(expr),
                    };
                }
                TokenKind::As if matches!(self.tokens.get(self.peek_index() + 1).map(|t| &t.kind), Some(TokenKind::Question)) => {
                    let start = expr_span(&expr);
                    self.advance();
                    self.advance();
                    let target = self.parse_type()?;
                    let span = Span::merge(start, self.previous().span);
                    expr = Expr::OptionalCast {
                        span,
                        expr: Box::new(expr),
                        target,
                    };
                }
                TokenKind::As => {
                    let start = expr_span(&expr);
                    self.advance();
                    let target = self.parse_type()?;
                    let span = Span::merge(start, self.previous().span);
                    expr = Expr::Cast {
                        span,
                        expr: Box::new(expr),
                        target,
                    };
                }
                TokenKind::Is => {
                    let start = expr_span(&expr);
                    self.advance();
                    let target = self.parse_type()?;
                    let span = Span::merge(start, self.previous().span);
                    expr = Expr::Is {
                        span,
                        expr: Box::new(expr),
                        target,
                    };
                }
                TokenKind::Bang if !matches!(self.previous().kind, TokenKind::NotEqual) => {
                    self.advance();
                    let span = Span::merge(expr_span(&expr), self.previous().span);
                    expr = Expr::ForceUnwrap {
                        span,
                        expr: Box::new(expr),
                    };
                }
                TokenKind::LBracket => {
                    self.advance();
                    // S1-I9: `m[row, col]` / `a[safe: i, default: d]` → Index with tuple / call-arg list.
                    let index = self.parse_index_contents()?;
                    self.consume(TokenKind::RBracket, "Expected ']' after index")?;
                    let span = Span::merge(expr_span(&expr), self.previous().span);
                    expr = Expr::Index {
                        span,
                        object: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                // S1-I3/I4: trailing closure `nums.map { $0 * 2 }` / `reduce(0) { $0 + $1 }`.
                // Only after Call/Member — never bare `items {` (for-in body).
                // S1-I7: also off while parsing for-in iterable (`Counter() {` is loop body).
                TokenKind::LBrace
                    if self.allow_trailing_closure
                        && self.looks_like_brace_closure()
                        && matches!(expr, Expr::Call { .. } | Expr::Member { .. } | Expr::OptionalChain { .. }) =>
                {
                    let clo = self.parse_primary()?;
                    let span = Span::merge(expr_span(&expr), expr_span(&clo));
                    let arg = CallArg {
                        label: None,
                        value: clo,
                    };
                    expr = match expr {
                        Expr::Call {
                            span: _,
                            callee,
                            mut args,
                        } => {
                            args.push(arg);
                            Expr::Call {
                                span,
                                callee,
                                args,
                            }
                        }
                        other => Expr::Call {
                            span,
                            callee: Box::new(other),
                            args: vec![arg],
                        },
                    };
                }
                _ => break,
            }
        }
        if self.match_kind(TokenKind::Assign) {
            let value = self.parse_expr()?;
            let end = expr_span(&value);
            return match expr {
                Expr::Ident { name, span } => Ok(Expr::Assign {
                    span: Span::merge(span, end),
                    name,
                    value: Box::new(value),
                }),
                Expr::Member {
                    object,
                    name,
                    span,
                } => Ok(Expr::MemberAssign {
                    span: Span::merge(span, end),
                    object,
                    name,
                    value: Box::new(value),
                }),
                Expr::Index {
                    object,
                    index,
                    span,
                } => Ok(Expr::IndexAssign {
                    span: Span::merge(span, end),
                    object,
                    index,
                    value: Box::new(value),
                }),
                _ => self.error("Invalid assignment target"),
            };
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> SyxResult<Expr> {
        let token = self.advance().clone();
        let span = token.span;
        match token.kind {
            TokenKind::Integer(v, suffix) => {
                if self.match_kind(TokenKind::DotDotLess) {
                    // S1-I7: `0..<self` / `0..<n` — integer end stays HalfOpenRange.
                    if let TokenKind::Integer(end_v, _) = &self.peek().kind {
                        let end_v = *end_v;
                        self.advance();
                        return Ok(Expr::HalfOpenRange {
                            span: Span::merge(span, self.previous().span),
                            start: v as i64,
                            end: end_v as i64,
                        });
                    }
                    let end = self.parse_expr()?;
                    return Ok(Expr::RangeDyn {
                        span: Span::merge(span, expr_span(&end)),
                        start: Box::new(Expr::Literal {
                            span,
                            value: Literal::Integer {
                                value: v,
                                suffix: suffix.clone(),
                            },
                        }),
                        end: Box::new(end),
                        inclusive: false,
                    });
                }
                if self.match_kind(TokenKind::DotDotDot) {
                    if let TokenKind::Integer(end_v, _) = &self.peek().kind {
                        let end_v = *end_v;
                        self.advance();
                        return Ok(Expr::ClosedRange {
                            span: Span::merge(span, self.previous().span),
                            start: v as i64,
                            end: end_v as i64,
                        });
                    }
                    let end = self.parse_expr()?;
                    return Ok(Expr::RangeDyn {
                        span: Span::merge(span, expr_span(&end)),
                        start: Box::new(Expr::Literal {
                            span,
                            value: Literal::Integer {
                                value: v,
                                suffix: suffix.clone(),
                            },
                        }),
                        end: Box::new(end),
                        inclusive: true,
                    });
                }
                Ok(Expr::Literal {
                    span,
                    value: Literal::Integer { value: v, suffix },
                })
            }
            TokenKind::BigIntLiteral(text) => Ok(Expr::Literal {
                span,
                value: Literal::BigInt(text),
            }),
            TokenKind::Double(v, suffix) => Ok(Expr::Literal {
                span,
                value: Literal::Double { value: v, suffix },
            }),
            TokenKind::String(s) => Ok(Expr::Literal {
                span,
                value: Literal::String(s),
            }),
            TokenKind::InterpolatedString(parts) => {
                let mut string_parts = Vec::new();
                for part in parts {
                    match part {
                        InterpPart::Text(t) => {
                            string_parts.push(syx_ast::StringPart::Text(t));
                        }
                        InterpPart::Expr(raw) => {
                            let expr = self.parse_embedded_expr(&raw)?;
                            string_parts.push(syx_ast::StringPart::Expr(Box::new(expr)));
                        }
                    }
                }
                Ok(Expr::Literal {
                    span,
                    value: Literal::Interpolated(string_parts),
                })
            }
            TokenKind::Regex(p) => Ok(Expr::Literal {
                span,
                value: Literal::Regex(p),
            }),
            TokenKind::Bool(v) => Ok(Expr::Literal {
                span,
                value: Literal::Bool(v),
            }),
            TokenKind::Nil => Ok(Expr::Literal {
                span,
                value: Literal::Nil,
            }),
            TokenKind::Hash => {
                let start = span;
                let name = self.consume_macro_name("Expected macro name after '#")?;
                self.consume(TokenKind::LParen, "Expected '(' after macro name")?;
                let mut args = Vec::new();
                if !self.check(TokenKind::RParen) {
                    loop {
                        args.push(self.parse_expr()?);
                        if !self.match_kind(TokenKind::Comma) {
                            break;
                        }
                    }
                }
                self.consume(TokenKind::RParen, "Expected ')' after macro arguments")?;
                let end_span = self.previous().span;
                if name.as_str() == "line" && args.is_empty() {
                    return Ok(Expr::Literal {
                        span: Span::merge(start, end_span),
                        value: Literal::Integer {
                            value: 1,
                            suffix: None,
                        },
                    });
                }
                if name.as_str() == "file" && args.is_empty() {
                    return Ok(Expr::Literal {
                        span: Span::merge(start, end_span),
                        value: Literal::String("syx.sy".into()),
                    });
                }
                Ok(Expr::MacroInvoke {
                    span: Span::merge(start, end_span),
                    name,
                    args,
                })
            }
            TokenKind::Identifier(name) => Ok(Expr::Ident { span, name }),
            kind if Self::soft_keyword_ident(&kind).is_some() => Ok(Expr::Ident {
                span,
                name: SmolStr::new(Self::soft_keyword_ident(&kind).unwrap()),
            }),
            TokenKind::Task => {
                let start = span;
                let mut detached = false;
                if self.match_kind(TokenKind::Dot) {
                    let member = self.consume_identifier("Expected member after Task.")?;
                    if member.as_str() != "detached" {
                        return self.error("Expected 'detached' after Task.");
                    }
                    detached = true;
                }
                if self.match_kind(TokenKind::LBrace) {
                    let body = self.parse_block_stmts()?;
                    return Ok(Expr::TaskBlock {
                        span: Span::merge(start, self.previous().span),
                        detached,
                        body,
                    });
                }
                Ok(Expr::Ident {
                    span,
                    name: SmolStr::new("Task"),
                })
            }
            TokenKind::SelfKw => Ok(Expr::Ident {
                span,
                name: SmolStr::new("self"),
            }),
            TokenKind::SuperKw => Ok(Expr::Super { span }),
            TokenKind::LParen => self.parse_paren_expr(span),
            TokenKind::LBracket => self.parse_bracket_literal(span),
            TokenKind::LBrace => {
                if self.check(TokenKind::Pipe) {
                    return self.parse_closure(span);
                }
                if self.check(TokenKind::LParen) {
                    return self.parse_swift_closure(span);
                }
                if self.looks_like_brace_closure() {
                    return self.parse_brace_closure(span);
                }
                self.parse_dict_literal(span)
            }
            _ => self.error_at(token.clone(), "Expected expression"),
        }
    }

    fn parse_closure(&mut self, start_span: Span) -> SyxResult<Expr> {
        self.consume(TokenKind::Pipe, "Expected '|' in closure")?;
        let params = self.parse_closure_params()?;
        self.consume(TokenKind::Pipe, "Expected '|' after closure parameters")?;
        let body = if self.check(TokenKind::LBrace) {
            self.advance();
            let stmts = self.parse_block_stmts()?;
            ClosureBody::Block(stmts)
        } else {
            let expr = self.parse_expr()?;
            self.consume(TokenKind::RBrace, "Expected '}' after closure expression")?;
            ClosureBody::Expr(Box::new(expr))
        };
        Ok(Expr::Closure {
            span: Span::merge(start_span, self.previous().span),
            params,
            return_type: None,
            body,
        })
    }

    /// Phase 86: Swift-style `{ (a: Int, b: Int) -> Int in return a - b }`.
    fn parse_swift_closure(&mut self, start_span: Span) -> SyxResult<Expr> {
        self.consume(TokenKind::LParen, "Expected '(' in closure")?;
        let mut params = Vec::new();
        if !self.check(TokenKind::RParen) {
            loop {
                let name = self.consume_identifier("Expected closure parameter name")?;
                let ty = if self.match_kind(TokenKind::Colon) {
                    Some(self.parse_type()?)
                } else {
                    None
                };
                params.push(Param {
                    name,
                    external_label: ParamLabel::Same,
                    ty,
                    default: None,
                    isolated: false,
                    sending: false,
                    attrs: vec![],
                });
                if !self.match_kind(TokenKind::Comma) {
                    break;
                }
            }
        }
        self.consume(TokenKind::RParen, "Expected ')' after closure parameters")?;
        let return_type = if self.match_kind(TokenKind::Arrow) {
            Some(self.parse_type()?)
        } else {
            None
        };
        self.consume(TokenKind::In, "Expected 'in' in closure")?;
        // Prefer expression body so `in a + b` returns the value (not Expr-stmt + Pop).
        let body = if self.check(TokenKind::LBrace) {
            self.advance();
            let stmts = self.parse_block_stmts()?;
            ClosureBody::Block(stmts)
        } else {
            let expr = self.parse_expr()?;
            self.consume(TokenKind::RBrace, "Expected '}' after closure")?;
            return Ok(Expr::Closure {
                span: Span::merge(start_span, self.previous().span),
                params,
                return_type,
                body: ClosureBody::Expr(Box::new(expr)),
            });
        };
        self.consume(TokenKind::RBrace, "Expected '}' after closure")?;
        Ok(Expr::Closure {
            span: Span::merge(start_span, self.previous().span),
            params,
            return_type,
            body,
        })
    }

    fn parse_closure_params(&mut self) -> SyxResult<Vec<Param>> {
        let mut params = Vec::new();
        if self.check(TokenKind::Pipe) {
            return Ok(params);
        }
        loop {
            let name = self.consume_identifier("Expected closure parameter name")?;
            let ty = if self.match_kind(TokenKind::Colon) {
                Some(self.parse_type()?)
            } else {
                None
            };
                params.push(Param {
                    name,
                    external_label: ParamLabel::Same,
                    ty,
                    default: None,
                    isolated: false,
                    sending: false,
                    attrs: vec![],
                });
            if !self.match_kind(TokenKind::Comma) {
                break;
            }
        }
        Ok(params)
    }

    /// Non-trivia token `n` steps after current peek (0 = peek itself).
    fn peek_non_trivia_kind(&self, n: usize) -> Option<&TokenKind> {
        let mut idx = self.peek_index();
        let mut seen = 0usize;
        while idx < self.tokens.len() {
            if Self::is_trivia_kind(&self.tokens[idx].kind) {
                idx += 1;
                continue;
            }
            if seen == n {
                return Some(&self.tokens[idx].kind);
            }
            seen += 1;
            idx += 1;
        }
        None
    }

    /// S1-I8: after `{ params in`, body starts with a statement keyword (`return` / `if` / …).
    fn looks_like_closure_stmt_body(&self) -> bool {
        matches!(
            self.peek().kind,
            TokenKind::Return
                | TokenKind::If
                | TokenKind::While
                | TokenKind::For
                | TokenKind::Var
                | TokenKind::Let
                | TokenKind::Guard
                | TokenKind::Switch
                | TokenKind::Throw
                | TokenKind::Break
                | TokenKind::Continue
                | TokenKind::Try
                | TokenKind::Repeat
        )
    }

    /// `{ $0 * 2 }` / `{ x in x + 1 }` / `{ print("hi") }` (not a dict `{ k: v }`).
    /// When called from postfix, `peek` is still `{`; when from primary, `{` was consumed.
    fn looks_like_brace_closure(&self) -> bool {
        let mut i = if matches!(self.peek().kind, TokenKind::LBrace) {
            1
        } else {
            0
        };
        match self.peek_non_trivia_kind(i) {
            Some(TokenKind::Identifier(n)) if n.starts_with('$') => true,
            Some(TokenKind::Pipe) | Some(TokenKind::LParen) => true,
            // S1-I7: `{ return "later" }` / `{ if … }` statement-bodied closures.
            Some(
                TokenKind::Return
                    | TokenKind::If
                    | TokenKind::While
                    | TokenKind::For
                    | TokenKind::Var
                    | TokenKind::Let
                    | TokenKind::Guard
                    | TokenKind::Switch
                    | TokenKind::Throw
                    | TokenKind::Break
                    | TokenKind::Continue
                    | TokenKind::Try
                    | TokenKind::Repeat,
            ) => true,
            Some(TokenKind::Identifier(_)) => {
                // `{ x: v }` dict vs `{ x in … }` / `{ x = … }` / `{ print(…) }` closure.
                match self.peek_non_trivia_kind(i + 1) {
                    Some(TokenKind::Colon) => false,
                    Some(TokenKind::In) => true,
                    Some(TokenKind::Comma) => {
                        // `{ x, y in … }`
                        i += 2;
                        loop {
                            match self.peek_non_trivia_kind(i) {
                                Some(TokenKind::Identifier(_)) => {
                                    i += 1;
                                    match self.peek_non_trivia_kind(i) {
                                        Some(TokenKind::Comma) => {
                                            i += 1;
                                            continue;
                                        }
                                        Some(TokenKind::In) => return true,
                                        _ => return false,
                                    }
                                }
                                _ => return false,
                            }
                        }
                    }
                    // Zero-param / body starts with identifier (call, assign, …).
                    _ => true,
                }
            }
            // `{ "k": v }` is a dict; bare string expr rare as closure start.
            Some(TokenKind::String(_)) | Some(TokenKind::Integer(_, _)) => {
                !matches!(self.peek_non_trivia_kind(i + 1), Some(TokenKind::Colon))
            }
            _ => false,
        }
    }

    fn parse_brace_closure(&mut self, start_span: Span) -> SyxResult<Expr> {
        // `{ $0 … }` shorthand — body is an expression (or block stmts until `}`).
        if matches!(&self.peek().kind, TokenKind::Identifier(n) if n.starts_with('$')) {
            let body_expr = self.parse_expr()?;
            self.consume(TokenKind::RBrace, "Expected '}' after closure shorthand")?;
            let max_idx = Self::max_shorthand_index(&body_expr).unwrap_or(0);
            let params: Vec<Param> = (0..=max_idx)
                .map(|i| Param {
                    name: SmolStr::new(format!("${i}")),
                    external_label: ParamLabel::Same,
                    ty: None,
                    default: None,
                    isolated: false,
                    sending: false,
                    attrs: vec![],
                })
                .collect();
            return Ok(Expr::Closure {
                span: Span::merge(start_span, self.previous().span),
                params,
                return_type: None,
                body: ClosureBody::Expr(Box::new(body_expr)),
            });
        }
        // Zero-param `{ print("hi") }` / `{ n = n + 1; n }` — no `in`.
        let zero_param = !matches!(
            (self.peek_non_trivia_kind(0), self.peek_non_trivia_kind(1)),
            (Some(TokenKind::Identifier(_)), Some(TokenKind::In | TokenKind::Comma))
        );
        if zero_param {
            let stmts = self.parse_stmts_until_rbrace()?;
            return Ok(Expr::Closure {
                span: Span::merge(start_span, self.previous().span),
                params: Vec::new(),
                return_type: None,
                body: ClosureBody::Block(stmts),
            });
        }
        // `{ x in … }` / `{ x, y in … }`
        let mut params = Vec::new();
        loop {
            let name = self.consume_identifier("Expected closure parameter name")?;
            params.push(Param {
                name,
                external_label: ParamLabel::Same,
                ty: None,
                default: None,
                isolated: false,
                sending: false,
                attrs: vec![],
            });
            if !self.match_kind(TokenKind::Comma) {
                break;
            }
        }
        self.consume(TokenKind::In, "Expected 'in' in brace closure")?;
        // S1-I7: `{ _ in }` / `{ x in }` — empty body (Void / discard).
        if self.check(TokenKind::RBrace) {
            self.advance();
            return Ok(Expr::Closure {
                span: Span::merge(start_span, self.previous().span),
                params,
                return_type: None,
                body: ClosureBody::Block(Vec::new()),
            });
        }
        // S1-I8: `{ n, s in return "…" }` — statement body after `in` (not an expression).
        let body = if self.check(TokenKind::LBrace) {
            self.advance();
            ClosureBody::Block(self.parse_block_stmts()?)
        } else if self.looks_like_closure_stmt_body() {
            ClosureBody::Block(self.parse_stmts_until_rbrace()?)
        } else {
            let expr = self.parse_expr()?;
            self.consume(TokenKind::RBrace, "Expected '}' after brace closure")?;
            ClosureBody::Expr(Box::new(expr))
        };
        Ok(Expr::Closure {
            span: Span::merge(start_span, self.previous().span),
            params,
            return_type: None,
            body,
        })
    }

    fn parse_stmts_until_rbrace(&mut self) -> SyxResult<Vec<Stmt>> {
        let mut stmts = Vec::new();
        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            self.skip_newlines();
            if self.check(TokenKind::RBrace) {
                break;
            }
            stmts.push(self.parse_stmt()?);
            self.skip_newlines();
        }
        self.consume(TokenKind::RBrace, "Expected '}' after closure body")?;
        Ok(stmts)
    }

    fn max_shorthand_index(expr: &Expr) -> Option<u32> {
        let mut max = None;
        fn walk(e: &Expr, max: &mut Option<u32>) {
            match e {
                Expr::Ident { name, .. } => {
                    if let Some(rest) = name.strip_prefix('$') {
                        if let Ok(i) = rest.parse::<u32>() {
                            *max = Some(max.map_or(i, |m| m.max(i)));
                        }
                    }
                }
                Expr::Binary { left, right, .. } => {
                    walk(left, max);
                    walk(right, max);
                }
                Expr::Ternary {
                    condition,
                    then_branch,
                    else_branch,
                    ..
                } => {
                    walk(condition, max);
                    walk(then_branch, max);
                    walk(else_branch, max);
                }
                Expr::Unary { expr, .. }
                | Expr::Group { expr, .. }
                | Expr::ForceUnwrap { expr, .. }
                | Expr::OptionalCast { expr, .. }
                | Expr::Cast { expr, .. }
                | Expr::Is { expr, .. } => walk(expr, max),
                Expr::Call { callee, args, .. } => {
                    walk(callee, max);
                    for a in args {
                        walk(&a.value, max);
                    }
                }
                Expr::Member { object, .. } | Expr::OptionalChain { object, .. } => {
                    walk(object, max);
                }
                Expr::Index { object, index, .. } => {
                    walk(object, max);
                    walk(index, max);
                }
                Expr::Array { elements, .. } => {
                    for el in elements {
                        walk(el, max);
                    }
                }
                _ => {}
            }
        }
        walk(expr, &mut max);
        max
    }

    fn finish_dict_literal(
        &mut self,
        start_span: Span,
        end_span: Span,
        entries: Vec<(Expr, Expr)>,
    ) -> SyxResult<Expr> {
        Ok(Expr::Literal {
            span: Span::merge(start_span, end_span),
            value: Literal::Dict(entries),
        })
    }

    /// `[T]` array vs `[K: V]` dictionary literal (matches type syntax `[String: Int]`).
    /// Empty dictionary is `[:]` (parallel to Swift); empty array remains `[]`.
    fn parse_bracket_literal(&mut self, start_span: Span) -> SyxResult<Expr> {
        if self.check(TokenKind::RBracket) {
            self.advance();
            return Ok(Expr::Array {
                span: Span::merge(start_span, self.previous().span),
                elements: vec![],
            });
        }
        if self.check(TokenKind::Colon) {
            self.advance();
            self.consume(TokenKind::RBracket, "Expected ']' after empty dictionary '[:]'")?;
            return self.finish_dict_literal(start_span, self.previous().span, vec![]);
        }
        let first = self.parse_expr()?;
        if self.match_kind(TokenKind::Colon) {
            let mut entries = vec![(first, self.parse_expr()?)];
            while self.match_kind(TokenKind::Comma) {
                let key = self.parse_expr()?;
                self.consume(TokenKind::Colon, "Expected ':' after dictionary key")?;
                entries.push((key, self.parse_expr()?));
            }
            self.consume(TokenKind::RBracket, "Expected ']' after dictionary entries")?;
            return self.finish_dict_literal(start_span, self.previous().span, entries);
        }
        let mut elements = vec![first];
        while self.match_kind(TokenKind::Comma) {
            elements.push(self.parse_expr()?);
        }
        self.consume(TokenKind::RBracket, "Expected ']' after array elements")?;
        Ok(Expr::Array {
            span: Span::merge(start_span, self.previous().span),
            elements,
        })
    }

    fn parse_dict_literal(&mut self, start_span: Span) -> SyxResult<Expr> {
        let mut entries: Vec<(Expr, Expr)> = Vec::new();
        if !self.check(TokenKind::RBrace) {
            loop {
                let key = self.parse_expr()?;
                self.consume(TokenKind::Colon, "Expected ':' after dictionary key")?;
                entries.push((key, self.parse_expr()?));
                if !self.match_kind(TokenKind::Comma) {
                    break;
                }
            }
        }
        self.consume(TokenKind::RBrace, "Expected '}' after dictionary entries")?;
        self.finish_dict_literal(start_span, self.previous().span, entries)
    }

    fn parse_protocol(&mut self, access: AccessLevel) -> SyxResult<Decl> {
        let span = self.advance().span;
        let name = self.consume_identifier("Expected protocol name")?;
        // ADR-230 Codable: Encodable, Decodable — comma-separated inherited protocols.
        let mut inherited = Vec::new();
        if self.match_kind(TokenKind::Colon) {
            loop {
                inherited.push(self.consume_identifier("Expected inherited protocol name")?);
                if !self.match_kind(TokenKind::Comma) {
                    break;
                }
                self.skip_newlines();
            }
        }
        self.consume(TokenKind::LBrace, "Expected '{' after protocol name")?;
        let mut methods = Vec::new();
        let mut associated_types = Vec::new();
        while !self.check(TokenKind::RBrace) && !self.is_at_end() {
            self.skip_newlines();
            if self.check(TokenKind::RBrace) {
                break;
            }
            if self.check_identifier("associatedtype") {
                self.advance();
                let assoc_name = self.consume_identifier("Expected associated type name")?;
                let mut constraint = TypeExpr::Named {
                    name: SmolStr::new("Any"),
                    generics: vec![],
                };
                if self.match_kind(TokenKind::Colon) {
                    constraint = self.parse_type()?;
                }
                associated_types.push((assoc_name, constraint));
                continue;
            }
            // S1-I5: `var name: T { get }` / `{ get set }` property requirements.
            if matches!(&self.peek().kind, TokenKind::Var | TokenKind::Let) {
                methods.push(self.parse_protocol_property_requirement()?);
                continue;
            }
            // S1-I5: `func ==(...)` operator requirements inside protocols.
            if self.check_identifier("operator") {
                methods.push(self.parse_protocol_operator_requirement()?);
                continue;
            }
            if !self.check(TokenKind::Func) {
                return self.error("Expected 'func', 'var', 'let', or 'associatedtype' in protocol");
            }
            let start = self.advance().span;
            // `func ==(lhs: Self, rhs: Self)` — operator as bare name after func.
            let fname = if let Some(op) = self.try_parse_operator_as_name() {
                op
            } else {
                self.consume_identifier("Expected function name")?
            };
            methods.push(self.parse_function_with_name_opts(
                AccessLevel::Internal,
                false,
                Vec::new(),
                start,
                fname,
                false,
                false,
                false,
            )?);
        }
        self.consume(TokenKind::RBrace, "Expected '}' after protocol body")?;
        Ok(Decl::Protocol {
            span,
            access,
            name,
            inherited,
            associated_types,
            methods,
        })
    }

    /// Protocol `var`/`let` requirement: `var name: Type { get }` or `{ get set }` (no bodies).
    fn parse_protocol_property_requirement(&mut self) -> SyxResult<Decl> {
        let start = self.peek().span;
        let mutable = self.match_kind(TokenKind::Var);
        if !mutable {
            self.consume(TokenKind::Let, "Expected 'var' or 'let'")?;
        }
        let name = self.consume_identifier("Expected property name")?;
        self.consume(TokenKind::Colon, "Expected ':' after property name")?;
        let ty = self.parse_type()?;
        self.consume(TokenKind::LBrace, "Expected '{' after protocol property type")?;
        // `{ get }` or `{ get set }` — accessors are markers only.
        if !matches!(
            &self.peek().kind,
            TokenKind::Identifier(label) if label.as_str() == "get"
        ) {
            return self.error("Expected 'get' in protocol property requirement");
        }
        self.advance();
        let mut has_set = false;
        if matches!(
            &self.peek().kind,
            TokenKind::Identifier(label) if label.as_str() == "set"
        ) {
            if !mutable {
                return self.error("Protocol '{ get set }' property requires 'var'");
            }
            self.advance();
            has_set = true;
        }
        self.consume(TokenKind::RBrace, "Expected '}' after protocol property accessors")?;
        self.skip_newlines();
        Ok(Decl::VarGlobal {
            span: Span::merge(start, self.previous().span),
            access: AccessLevel::Internal,
            attrs: if has_set {
                vec![syx_ast::Attribute {
                    name: SmolStr::new("protocol_get_set"),
                    args: vec![],
                }]
            } else {
                vec![syx_ast::Attribute {
                    name: SmolStr::new("protocol_get"),
                    args: vec![],
                }]
            },
            mutable,
            name,
            ty: Some(ty),
            init: None,
            will_set: None,
            did_set: None,
            // Empty marker: requirement has no implementation body.
            computed_getter: Some(Vec::new()),
            computed_setter: if has_set { Some(Vec::new()) } else { None },
            is_override: false,
            is_lazy: false,
        })
    }

    fn parse_protocol_operator_requirement(&mut self) -> SyxResult<Decl> {
        let start = self.advance().span; // 'operator'
        self.consume(TokenKind::Func, "Expected 'func' after operator")?;
        let op = self.parse_operator_symbol()?;
        let (_type_params, _) = self.parse_type_params()?;
        self.consume(TokenKind::LParen, "Expected '(' after operator")?;
        let params = self.parse_params()?;
        self.consume(TokenKind::RParen, "Expected ')' after parameters")?;
        let return_type = if self.match_kind(TokenKind::Arrow) {
            Some(self.parse_type()?)
        } else {
            None
        };
        let body = if self.check(TokenKind::LBrace) {
            self.advance();
            self.parse_block_stmts()?
        } else {
            self.match_kind(TokenKind::Semicolon);
            Vec::new()
        };
        Ok(Decl::OperatorFunc {
            span: Span::merge(start, self.previous().span),
            access: AccessLevel::Internal,
            attrs: Vec::new(),
            op,
            params,
            return_type,
            body,
        })
    }

    fn try_parse_operator_as_name(&mut self) -> Option<SmolStr> {
        // Mirror `parse_operator_symbol` without erroring on non-ops.
        let op = match &self.peek().kind {
            TokenKind::EqualEqual => Some("=="),
            TokenKind::NotEqual => Some("!="),
            TokenKind::LessEqual => Some("<="),
            TokenKind::GreaterEqual => Some(">="),
            TokenKind::Less => Some("<"),
            TokenKind::Greater => Some(">"),
            TokenKind::Plus => Some("+"),
            TokenKind::Minus => Some("-"),
            TokenKind::Star => Some("*"),
            TokenKind::Slash => Some("/"),
            TokenKind::Percent => Some("%"),
            _ => None,
        }?;
        self.advance();
        Some(SmolStr::new(op))
    }

    fn check_identifier(&self, text: &str) -> bool {
        matches!(
            &self.peek().kind,
            TokenKind::Identifier(name) if name.as_str() == text
        )
    }

    fn match_identifier(&mut self, text: &str) -> bool {
        if self.check_identifier(text) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn parse_operator_symbol(&mut self) -> SyxResult<SmolStr> {
        let kind = self.peek().kind.clone();
        self.advance();
        let s = match kind {
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Star => "*",
            TokenKind::EqualEqual => "==",
            TokenKind::LBracket => {
                self.consume(TokenKind::RBracket, "Expected ']' after '['")?;
                "[]"
            }
            TokenKind::Identifier(name) => return Ok(name),
            _ => {
                return self.error("Expected operator symbol after 'operator func'");
            }
        };
        Ok(SmolStr::new(s))
    }

    fn parse_embedded_expr(&self, raw: &str) -> SyxResult<Expr> {
        let source = SourceFile::new("<interp>", raw);
        let tokens = Lexer::new(source.clone()).scan_all()?;
        let mut parser = Parser::new(source, tokens);
        parser.parse_expr()
    }

    fn parse_args(&mut self) -> SyxResult<Vec<CallArg>> {
        let mut args = Vec::new();
        if self.check(TokenKind::RParen) {
            return Ok(args);
        }
        loop {
            let label = {
                let idx = self.peek_index();
                let next_is_colon =
                    idx + 1 < self.tokens.len() && self.tokens[idx + 1].kind == TokenKind::Colon;
                if next_is_colon {
                    match &self.peek().kind {
                        TokenKind::Identifier(name) => {
                            let name = name.clone();
                            self.advance();
                            self.advance();
                            Some(name)
                        }
                        kind if Self::soft_keyword_ident(kind).is_some() => {
                            let name = SmolStr::new(Self::soft_keyword_ident(kind).unwrap());
                            self.advance();
                            self.advance();
                            Some(name)
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            };
            let value = self.parse_expr()?;
            args.push(CallArg { label, value });
            if !self.match_kind(TokenKind::Comma) {
                break;
            }
        }
        Ok(args)
    }

    fn consume_macro_name(&mut self, message: &str) -> SyxResult<SmolStr> {
        let name = match &self.peek().kind {
            TokenKind::Identifier(name) => name.clone(),
            TokenKind::If => SmolStr::new("if"),
            TokenKind::For => SmolStr::new("for"),
            TokenKind::While => SmolStr::new("while"),
            TokenKind::Macro => SmolStr::new("macro"),
            _ => return self.error(message),
        };
        self.advance();
        Ok(name)
    }

    fn match_mut_keyword(&mut self) -> bool {
        if let TokenKind::Identifier(name) = &self.peek().kind {
            if name.as_str() == "mut" {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, kind: TokenKind, message: &str) -> SyxResult<&Token> {
        if self.check(kind.clone()) {
            Ok(self.advance())
        } else {
            self.error(message)
        }
    }

    /// Accept `>` or the first half of `>>` (nested generics like `Foo<Bar<Baz>>`).
    fn consume_rangle(&mut self, message: &str) -> SyxResult<&Token> {
        if self.check(TokenKind::Greater) {
            return Ok(self.advance());
        }
        if self.check(TokenKind::GreaterGreater) {
            let tok = self.tokens[self.current].clone();
            let half = Span::new(tok.span.start as usize, 1);
            let rest = Span::new(tok.span.start as usize + 1, 1);
            self.tokens[self.current] = Token {
                kind: TokenKind::Greater,
                lexeme: SmolStr::new(">"),
                span: half,
                line: tok.line,
                column: tok.column,
            };
            self.tokens.insert(
                self.current + 1,
                Token {
                    kind: TokenKind::Greater,
                    lexeme: SmolStr::new(">"),
                    span: rest,
                    line: tok.line,
                    column: tok.column.saturating_add(1),
                },
            );
            return Ok(self.advance());
        }
        self.error(message)
    }

    /// Contextual keywords that may appear as identifiers (params, labels, exprs).
    /// `data` must work in `func f(data: String)` / `f(data: x)` while remaining
    /// a declaration keyword for `data class`.
    fn soft_keyword_ident(kind: &TokenKind) -> Option<&'static str> {
        match kind {
            TokenKind::Data => Some("data"),
            TokenKind::Service => Some("Service"),
            TokenKind::Type => Some("type"),
            TokenKind::Pure => Some("Pure"),
            TokenKind::Final => Some("final"),
            TokenKind::Global => Some("global"),
            TokenKind::Macro => Some("macro"),
            TokenKind::Init => Some("init"),
            TokenKind::Deinit => Some("deinit"),
            TokenKind::Select => Some("select"),
            TokenKind::Timeout => Some("timeout"),
            // S1-I9: `subscript(…, default d: Int)` / `obj.default`
            TokenKind::Default => Some("default"),
            _ => None,
        }
    }

    /// `name:` label start — Identifier or soft keyword (`data: String`, etc.).
    fn peek_is_label_colon(&self) -> bool {
        let is_name = matches!(self.peek().kind, TokenKind::Identifier(_))
            || Self::soft_keyword_ident(&self.peek().kind).is_some();
        is_name
            && matches!(
                self.peek_ahead(1).map(|t| &t.kind),
                Some(TokenKind::Colon)
            )
    }

    fn consume_identifier(&mut self, message: &str) -> SyxResult<SmolStr> {
        match &self.peek().kind {
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            kind if Self::soft_keyword_ident(kind).is_some() => {
                let name = SmolStr::new(Self::soft_keyword_ident(kind).unwrap());
                self.advance();
                Ok(name)
            }
            _ => self.error(message),
        }
    }

    fn consume_module_segment(&mut self, message: &str) -> SyxResult<SmolStr> {
        match &self.peek().kind {
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            TokenKind::Io => {
                self.advance();
                Ok(SmolStr::new("IO"))
            }
            _ => self.error(message),
        }
    }

    fn match_kind(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, kind: TokenKind) -> bool {
        !self.is_at_end() && self.peek().kind == kind
    }

    fn check_expr_start(&self) -> bool {
        matches!(
            self.peek().kind,
            TokenKind::Integer(_, _)
                | TokenKind::Double(_, _)
                | TokenKind::String(_)
                | TokenKind::InterpolatedString(_)
                | TokenKind::Regex(_)
                | TokenKind::Bool(_)
                | TokenKind::Nil
                | TokenKind::Identifier(_)
                | TokenKind::LParen
                | TokenKind::LBracket
                | TokenKind::LBrace
                | TokenKind::Throw
                | TokenKind::Minus
                | TokenKind::Bang
                | TokenKind::Await
                | TokenKind::Task
                | TokenKind::Try
                | TokenKind::Amp
                | TokenKind::SelfKw
        ) || Self::soft_keyword_ident(&self.peek().kind).is_some()
    }

    fn is_trivia_kind(kind: &TokenKind) -> bool {
        matches!(
            kind,
            TokenKind::Newline
                | TokenKind::LineComment
                | TokenKind::BlockComment
                | TokenKind::Whitespace
        )
    }

    fn advance(&mut self) -> &Token {
        self.current = self.peek_index();
        let token = &self.tokens[self.current];
        self.current += 1;
        token
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current.saturating_sub(1)]
    }

    fn previous_non_newline(&self) -> &Token {
        let mut idx = self.current.saturating_sub(1);
        while idx > 0 && Self::is_trivia_kind(&self.tokens[idx].kind) {
            idx -= 1;
        }
        &self.tokens[idx]
    }

    fn peek_index(&self) -> usize {
        let mut idx = self.current;
        while idx < self.tokens.len() && Self::is_trivia_kind(&self.tokens[idx].kind) {
            idx += 1;
        }
        idx.min(self.tokens.len().saturating_sub(1))
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.peek_index()]
    }

    /// Peek the n-th non-trivia token ahead (0 == [`Self::peek`]).
    fn peek_nth(&self, n: usize) -> &Token {
        let mut idx = self.peek_index();
        let mut seen = 0usize;
        while seen < n {
            idx += 1;
            while idx < self.tokens.len() && Self::is_trivia_kind(&self.tokens[idx].kind) {
                idx += 1;
            }
            if idx >= self.tokens.len() {
                return self.tokens.last().expect("token stream non-empty");
            }
            seen += 1;
        }
        &self.tokens[idx.min(self.tokens.len().saturating_sub(1))]
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }

    fn parse_keypath_segment(&mut self) -> SyxResult<SmolStr> {
        if matches!(self.peek().kind, TokenKind::LBracket) {
            self.advance();
            let idx = match &self.peek().kind {
                TokenKind::Integer(v, _) => {
                    let v = *v;
                    self.advance();
                    v.to_string()
                }
                TokenKind::Identifier(s) => {
                    let s = s.clone();
                    self.advance();
                    s.to_string()
                }
                _ => {
                    return self.error("Expected index in key path subscript");
                }
            };
            self.consume(TokenKind::RBracket, "Expected ']' after key path index")?;
            Ok(SmolStr::new(format!("[{idx}]")))
        } else {
            self.consume_identifier("Expected property name in key path")
        }
    }

    fn skip_newlines(&mut self) {
        while self.current < self.tokens.len() && Self::is_trivia_kind(&self.tokens[self.current].kind)
        {
            self.current += 1;
        }
    }

    fn error<T>(&self, message: &str) -> SyxResult<T> {
        Err(SyxDiagnostic::error(message).with_code("P001").with_span(
            &self.source,
            self.peek().span,
            "here",
        ))
    }

    fn error_at<T>(&self, token: Token, message: &str) -> SyxResult<T> {
        Err(SyxDiagnostic::error(message).with_code("P001").with_span(
            &self.source,
            token.span,
            "here",
        ))
    }
}

/// Compact type name for KeyPath roots (`Array<Int>`, nested generics).
fn type_expr_keypath_name(ty: &TypeExpr) -> String {
    match ty {
        TypeExpr::Named { name, generics } => {
            if generics.is_empty() {
                name.to_string()
            } else {
                let inner = generics
                    .iter()
                    .map(type_expr_keypath_name)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{name}<{inner}>")
            }
        }
        TypeExpr::Array(inner) => format!("[{}]", type_expr_keypath_name(inner)),
        TypeExpr::Dictionary { key, value } => format!(
            "Dictionary<{}, {}>",
            type_expr_keypath_name(key),
            type_expr_keypath_name(value)
        ),
        TypeExpr::Optional(inner) => format!("{}?", type_expr_keypath_name(inner)),
        TypeExpr::Void => "Void".to_string(),
        other => format!("{other:?}"),
    }
}

fn infix_op(kind: &TokenKind) -> Option<(BinOp, u8)> {
    // Precedence (higher = tighter), Swift-like:
    // || < && < | < ^ < & < equality < compare < << >> < + - < * / % < ??
    Some(match kind {
        TokenKind::OrOr => (BinOp::Or, 1),
        TokenKind::AndAnd => (BinOp::And, 2),
        TokenKind::Pipe => (BinOp::BitOr, 3),
        TokenKind::Caret => (BinOp::BitXor, 4),
        TokenKind::Amp => (BinOp::BitAnd, 5),
        TokenKind::EqualEqual => (BinOp::Eq, 6),
        TokenKind::EqualEqualEqual => (BinOp::IdenticalEq, 6),
        TokenKind::NotEqual => (BinOp::NotEq, 6),
        TokenKind::NotEqualEqual => (BinOp::IdenticalNe, 6),
        TokenKind::Less => (BinOp::Lt, 7),
        TokenKind::LessEqual => (BinOp::LtEq, 7),
        TokenKind::Greater => (BinOp::Gt, 7),
        TokenKind::GreaterEqual => (BinOp::GtEq, 7),
        TokenKind::LessLess => (BinOp::Shl, 8),
        TokenKind::GreaterGreater => (BinOp::Shr, 8),
        TokenKind::Plus => (BinOp::Add, 9),
        TokenKind::Minus => (BinOp::Sub, 9),
        TokenKind::Star => (BinOp::Mul, 10),
        TokenKind::Slash => (BinOp::Div, 10),
        TokenKind::Percent => (BinOp::Mod, 10),
        TokenKind::NilCoalesce => (BinOp::NilCoalesce, 11),
        _ => return None,
    })
}

fn compound_assign_op(kind: &TokenKind) -> Option<CompoundAssignOp> {
    Some(match kind {
        TokenKind::PlusEqual => CompoundAssignOp::Add,
        TokenKind::MinusEqual => CompoundAssignOp::Sub,
        TokenKind::StarEqual => CompoundAssignOp::Mul,
        TokenKind::SlashEqual => CompoundAssignOp::Div,
        TokenKind::PercentEqual => CompoundAssignOp::Mod,
        TokenKind::AmpEqual => CompoundAssignOp::BitAnd,
        TokenKind::PipeEqual => CompoundAssignOp::BitOr,
        TokenKind::CaretEqual => CompoundAssignOp::BitXor,
        TokenKind::LessLessEqual => CompoundAssignOp::Shl,
        TokenKind::GreaterGreaterEqual => CompoundAssignOp::Shr,
        _ => return None,
    })
}

fn expr_span(expr: &Expr) -> Span {
    match expr {
        Expr::Literal { span, .. }
        | Expr::Ident { span, .. }
        | Expr::Super { span, .. }
        | Expr::Binary { span, .. }
        | Expr::Ternary { span, .. }
        | Expr::Unary { span, .. }
        | Expr::Call { span, .. }
        | Expr::Member { span, .. }
        | Expr::Index { span, .. }
        | Expr::Assign { span, .. }
        | Expr::CompoundAssign { span, .. }
        | Expr::MemberAssign { span, .. }
        | Expr::IndexAssign { span, .. }
        | Expr::Group { span, .. }
        | Expr::Tuple { span, .. }
        | Expr::Array { span, .. }
        | Expr::Await { span, .. }
        | Expr::OptionalChain { span, .. }
        | Expr::ForceUnwrap { span, .. }
        | Expr::HalfOpenRange { span, .. }
        | Expr::ClosedRange { span, .. }
        | Expr::RangeDyn { span, .. }
        | Expr::VariantPattern { span, .. }
        | Expr::TuplePattern { span, .. }
        | Expr::Closure { span, .. }
        | Expr::TaskBlock { span, .. }
        | Expr::Borrow { span, .. }
        | Expr::KeyPath { span, .. }
        | Expr::MacroInvoke { span, .. }
        | Expr::OptionalCast { span, .. }
        | Expr::Is { span, .. }
        | Expr::Cast { span, .. }
        | Expr::TypeSelf { span, .. } => *span,
    }
}

fn stmt_span(stmt: &Stmt) -> Span {
    match stmt {
        Stmt::Expr { span, .. }
        | Stmt::Print { span, .. }
        | Stmt::VarDecl { span, .. }
        | Stmt::Return { span, .. }
        | Stmt::If { span, .. }
        | Stmt::IfLet { span, .. }
        | Stmt::IfCase { span, .. }
        | Stmt::GuardLet { span, .. }
        | Stmt::Switch { span, .. }
        | Stmt::While { span, .. }
        | Stmt::ForIn { span, .. }
        | Stmt::Try { span, .. }
        | Stmt::Throw { span, .. }
        | Stmt::RepeatWhile { span, .. }
        | Stmt::Break { span, .. }
        | Stmt::Continue { span, .. }
        | Stmt::Block { span, .. }
        | Stmt::Seq { span, .. }
        | Stmt::MacroInvoke { span, .. } => *span,
    }
}

fn select_default_miss_expr(span: Span) -> Expr {
    Expr::Literal {
        span,
        value: Literal::String("__syx_select_default_miss__".to_string()),
    }
}

fn select_cases_have_await(cases: &[SelectCase]) -> bool {
    cases.iter().any(|c| stmts_have_await(&c.body))
}

fn stmts_have_await(stmts: &[Stmt]) -> bool {
    stmts.iter().any(stmt_has_await)
}

fn stmt_has_await(stmt: &Stmt) -> bool {
    match stmt {
        Stmt::Return { value: Some(v), .. } => expr_has_await(v),
        Stmt::Expr { expr, .. } | Stmt::Print { expr, .. } => expr_has_await(expr),
        Stmt::VarDecl { init: Some(v), .. } => expr_has_await(v),
        Stmt::If {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            expr_has_await(condition)
                || stmts_have_await(then_branch)
                || else_branch
                    .as_ref()
                    .is_some_and(|b| stmts_have_await(b))
        }
        Stmt::While { condition, body, .. } | Stmt::RepeatWhile { condition, body, .. } => {
            expr_has_await(condition) || stmts_have_await(body)
        }
        Stmt::ForIn { iterable, body, .. } => {
            expr_has_await(iterable) || stmts_have_await(body)
        }
        Stmt::Block { stmts, .. } => stmts_have_await(stmts),
        Stmt::Try { body, catch_body, .. } => {
            stmts_have_await(body) || stmts_have_await(catch_body)
        }
        Stmt::GuardLet { body, else_branch, .. } => {
            stmts_have_await(body) || stmts_have_await(else_branch)
        }
        Stmt::Switch { subject, cases, default, .. } => {
            expr_has_await(subject)
                || cases
                    .iter()
                    .any(|c| stmts_have_await(&c.body))
                || default.as_ref().is_some_and(|b| stmts_have_await(b))
        }
        Stmt::IfCase {
            subject,
            body,
            else_branch,
            ..
        } => {
            expr_has_await(subject)
                || stmts_have_await(body)
                || else_branch
                    .as_ref()
                    .is_some_and(|b| stmts_have_await(b))
        }
        Stmt::Throw { expr, .. } => expr_has_await(expr),
        _ => false,
    }
}

fn expr_has_await(expr: &Expr) -> bool {
    match expr {
        Expr::Await { .. } => true,
        Expr::Binary { left, right, .. } => expr_has_await(left) || expr_has_await(right),
        Expr::Unary { expr, .. } => expr_has_await(expr),
        Expr::Call { callee, args, .. } => {
            expr_has_await(callee) || args.iter().any(|a| expr_has_await(&a.value))
        }
        Expr::Member { object, .. } => expr_has_await(object),
        Expr::Index { object, index, .. } => {
            expr_has_await(object) || expr_has_await(index)
        }
        Expr::Assign { value, .. } | Expr::CompoundAssign { value, .. } => expr_has_await(value),
        Expr::Group { expr, .. } => expr_has_await(expr),
        Expr::Array { elements, .. } => elements.iter().any(expr_has_await),
        Expr::TaskBlock { body, .. } => stmts_have_await(body),
        Expr::Closure { body, .. } => match body {
            ClosureBody::Expr(e) => expr_has_await(e),
            ClosureBody::Block(stmts) => stmts_have_await(stmts),
        },
        Expr::Borrow { expr, .. } | Expr::OptionalCast { expr, .. } => expr_has_await(expr),
        Expr::Literal { value, .. } => match value {
            Literal::Interpolated(parts) => parts.iter().any(|p| match p {
                syx_ast::StringPart::Expr(e) => expr_has_await(e),
                _ => false,
            }),
            _ => false,
        },
        _ => false,
    }
}

/// Phase 108: Object-tagged select arm via `__syx_select_tag(dir, channel, payload)`.
/// Returns Object so the poll array stays homogeneous (S014). Host equivalent of
/// `[dir, channel, payload?]` (dir 0=recv, 1=send).
fn select_tagged_arm_expr(span: Span, case: &SelectCase) -> Expr {
    let (dir, channel, payload) = match &case.kind {
        SelectCaseKind::Recv { channel, .. } => (
            0i128,
            channel.clone(),
            Expr::Literal {
                span,
                value: Literal::Nil,
            },
        ),
        SelectCaseKind::Send { channel, value } => (1i128, channel.clone(), value.clone()),
    };
    Expr::Call {
        span,
        callee: Box::new(Expr::Ident {
            span,
            name: SmolStr::new("__syx_select_tag"),
        }),
        args: vec![
            syx_ast::CallArg {
                label: None,
                value: Expr::Literal {
                    span,
                    value: Literal::Integer {
                        value: dir,
                        suffix: None,
                    },
                },
            },
            syx_ast::CallArg {
                label: None,
                value: channel,
            },
            syx_ast::CallArg {
                label: None,
                value: payload,
            },
        ],
    }
}

fn classify_select_case(left: SmolStr, left_span: Span, right: Expr) -> SelectCaseKind {
    match &right {
        Expr::Ident { .. } => SelectCaseKind::Recv {
            binding: left,
            channel: right,
        },
        _ => SelectCaseKind::Send {
            channel: Expr::Ident {
                span: left_span,
                name: left,
            },
            value: right,
        },
    }
}

fn is_print_call(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Call {
            callee,
            ..
        } if matches!(callee.as_ref(), Expr::Ident { name, .. } if name == "print")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stmt_contains_while(stmt: &Stmt) -> bool {
        match stmt {
            Stmt::While { .. } => true,
            Stmt::Block { stmts, .. } => stmts.iter().any(stmt_contains_while),
            Stmt::If {
                then_branch,
                else_branch,
                ..
            } => {
                then_branch.iter().any(stmt_contains_while)
                    || else_branch
                        .as_ref()
                        .is_some_and(|e| e.iter().any(stmt_contains_while))
            }
            _ => false,
        }
    }

    #[test]
    fn p107_select_mixed_send_recv_lowers_to_array_poll() {
        let source = SourceFile::new(
            "test.sy",
            r#"func main() {
    let ch1 = __syx_channel_create(1)
    let ch2 = __syx_channel_create(1)
    select {
        case v <- ch1:
            print("recv")
        case ch2 <- 1:
            print("sent")
        default:
            print("d")
    }
}"#,
        );
        let module = Parser::parse(source).unwrap();
        let Decl::Function { body, .. } = &module.decls[0] else {
            panic!("expected function");
        };
        assert!(
            !body.iter().any(stmt_contains_while),
            "mixed select must not lower to while"
        );
        let dump = format!("{body:?}");
        assert!(
            dump.contains("__syx_sel_arr"),
            "mixed select must build select array: {dump}"
        );
        assert!(
            dump.contains("__syx_channel_select_default"),
            "mixed select must poll via unified __syx_channel_select_default: {dump}"
        );
        assert!(
            dump.contains("__syx_sel_idx") && dump.contains("__syx_sel_res"),
            "mixed select must index-dispatch after one poll: {dump}"
        );
        // Must not use per-arm try_send GA path.
        assert!(
            !dump.contains("__syx_channel_try_send"),
            "mixed select must not lower to per-arm try_send: {dump}"
        );
        assert_eq!(
            select_mixed_unified_fingerprint(),
            "select-mixed-unified-v1:tagged-array-poll"
        );
    }

    #[test]
    fn p105_select_multi_case_lowers_without_while() {
        let source = SourceFile::new(
            "test.sy",
            r#"func main() {
    let ch1 = __syx_channel_create(1)
    let ch2 = __syx_channel_create(1)
    select {
        case v <- ch1:
            print("c1")
        case w <- ch2:
            print("c2")
        default:
            print("d")
    }
}"#,
        );
        let module = Parser::parse(source).unwrap();
        let Decl::Function { body, .. } = &module.decls[0] else {
            panic!("expected function");
        };
        assert!(
            !body.iter().any(stmt_contains_while),
            "multi-case select must not lower to while (OOM risk)"
        );
    }

    #[test]
    fn parses_reflect_class_line() {
        let source = SourceFile::new(
            "test.sy",
            r#"class Point { var x: Int; init(x: Int) { self.x = x } }
func main() {
    print(__syx_fmt_debug(reflect(Point.self)))
}"#,
        );
        Parser::parse(source).unwrap();
    }

    #[test]
    fn parses_function_and_print() {
        let source = SourceFile::new(
            "test.sy",
            r#"func main() {
    print("Hello, World")
}"#,
        );
        let module = Parser::parse(source).unwrap();
        assert_eq!(module.decls.len(), 1);
    }

    #[test]
    fn parses_hello_world() {
        let source = SourceFile::new(
            "hello.sy",
            "func main() {\n    print(\"Hello, World\")\n}\n",
        );
        Parser::parse(source).unwrap();
    }

    #[test]
    fn parses_external_parameter_labels() {
        let source = SourceFile::new(
            "test.sy",
            r#"public func join(_ task: Object) -> Object {
    return task
}

public func taskGroupAdd(_ group: Object, _ task: Object) -> Bool {
    return true
}"#,
        );
        let module = Parser::parse(source).unwrap();
        assert_eq!(module.decls.len(), 2);
    }

    #[test]
    fn parses_data_as_parameter_name_and_label() {
        // `data` is a declaration keyword for `data class`, but must remain usable as a param.
        let source = SourceFile::new(
            "crypto.sy",
            r#"public func sha256(data: String) -> String {
    return data
}
func main() {
    print(sha256(data: "hello"))
}
data class Point(x: Int, y: Int)
"#,
        );
        let module = Parser::parse(source).unwrap();
        assert!(module.decls.len() >= 3);
    }

    #[test]
    fn parses_bracket_dictionary_literal() {
        let source = SourceFile::new(
            "test.sy",
            r#"func main() {
    let scores: [String: Int] = ["a": 1, "b": 2]
    print(scores["a"])
}"#,
        );
        Parser::parse(source).unwrap();
    }

    #[test]
    fn parses_int_key_dictionary_literal() {
        let source = SourceFile::new(
            "test.sy",
            r#"func main() {
    let scores: Dictionary<Int, Int> = {2: 4, 3: 6}
    print(scores[2])
}"#,
        );
        Parser::parse(source).unwrap();
    }

    #[test]
    fn parses_bracket_int_key_dictionary_literal() {
        let source = SourceFile::new(
            "test.sy",
            r#"func main() {
    let m: [Int: Int] = [1: 10, 2: 20]
}"#,
        );
        Parser::parse(source).unwrap();
    }

    #[test]
    fn parse_tuple_fixture() {
        let src = include_str!("../../../crates/syx-driver/tests/fixtures/lang/tuple.sy");
        Parser::parse(SourceFile::new("tuple.sy", src)).unwrap();
    }

    #[test]
    fn parse_stdlib_time_and_character() {
        Parser::parse(SourceFile::new(
            "if.sy",
            "func main() { if (s == \"\") { return 0 } }",
        ))
        .expect("if empty string");
        let time = include_str!("../../../packages/core/time.sy");
        Parser::parse(SourceFile::new("time.sy", time)).expect("time.sy");
    }

    /// W480 / ADR-824: `export func` → Decl::Export wrapping Function.
    #[test]
    fn export_func_parses_to_decl_export() {
        let source = SourceFile::new(
            "test.sy",
            r#"export func greet() {
    print("hi")
}
func main() {
    greet()
}
"#,
        );
        let module = Parser::parse(source).expect("export func should parse");
        assert!(
            matches!(
                &module.decls[0],
                Decl::Export {
                    item,
                    ..
                } if matches!(item.as_ref(), Decl::Function { name, .. } if name.as_str() == "greet")
            ),
            "expected Decl::Export wrapping Function greet, got {:?}",
            module.decls[0]
        );
    }

    /// W483 / ADR-827: `export { a, b }` → Decl::ExportList.
    #[test]
    fn export_list_parses_to_decl_export_list() {
        let source = SourceFile::new(
            "test.sy",
            r#"export { greet, farewell }
func greet() {}
func farewell() {}
func main() {}
"#,
        );
        let module = Parser::parse(source).expect("export list should parse");
        match &module.decls[0] {
            Decl::ExportList { names, .. } => {
                assert_eq!(
                    names.iter().map(|n| n.as_str()).collect::<Vec<_>>(),
                    vec!["greet", "farewell"]
                );
            }
            other => panic!("expected Decl::ExportList, got {other:?}"),
        }
    }

    #[test]
    fn export_list_accepts_trailing_comma_and_rejects_empty() {
        // Wave A W486 / ADR-830: trailing comma allowed.
        let trailing = SourceFile::new("t.sy", "export { greet, }\nfunc greet() {}\n");
        let module = Parser::parse(trailing).expect("trailing comma should parse");
        match &module.decls[0] {
            Decl::ExportList { names, .. } => {
                assert_eq!(names.iter().map(|n| n.as_str()).collect::<Vec<_>>(), vec!["greet"]);
            }
            other => panic!("expected Decl::ExportList, got {other:?}"),
        }
        let trailing_multi = SourceFile::new(
            "t.sy",
            "export { a, b, }\nfunc a() {}\nfunc b() {}\n",
        );
        let module2 = Parser::parse(trailing_multi).expect("multi trailing comma should parse");
        match &module2.decls[0] {
            Decl::ExportList { names, .. } => {
                assert_eq!(
                    names.iter().map(|n| n.as_str()).collect::<Vec<_>>(),
                    vec!["a", "b"]
                );
            }
            other => panic!("expected Decl::ExportList, got {other:?}"),
        }
        let empty = SourceFile::new("t.sy", "export { }\n");
        assert!(Parser::parse(empty).is_err(), "empty export list must be rejected");
    }

    #[test]
    fn parse_collecting_reports_multiple_syntax_errors() {
        // Mirrors fixtures/multi_syntax_errors.sy — two funcs missing `{` bodies.
        let source = SourceFile::new(
            "multi_syntax_errors.sy",
            "func broken_a()\nfunc broken_b()\nfunc main() {}\n",
        );
        let outcome = Parser::parse_collecting(source);
        let p001: Vec<_> = outcome
            .errors
            .iter()
            .filter(|e| e.code.as_deref() == Some("P001"))
            .collect();
        assert!(
            p001.len() >= 2,
            "expected ≥2 P001 syntax errors at once, got {} in {:?}",
            p001.len(),
            outcome.errors
        );
        assert!(
            p001.iter().any(|e| e.message.contains('{') || e.message.contains("Expected")),
            "messages={:?}",
            p001.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn parse_collecting_fixture_multi_syntax_errors_sy() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../fixtures/multi_syntax_errors.sy");
        let text = std::fs::read_to_string(path).expect("fixtures/multi_syntax_errors.sy");
        let source = SourceFile::new("multi_syntax_errors.sy", text);
        let outcome = Parser::parse_collecting(source);
        let p001 = outcome
            .errors
            .iter()
            .filter(|e| e.code.as_deref() == Some("P001"))
            .count();
        assert!(
            p001 >= 2,
            "fixture must keep ≥2 syntax errors for LSP multi-diag nail; got {p001} in {:?}",
            outcome.errors
        );
    }
}
