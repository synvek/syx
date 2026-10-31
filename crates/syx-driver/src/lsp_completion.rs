use smol_str::SmolStr;
use syx_ast::{Decl, Expr, Module, Stmt};
use syx_diagnostics::{SourceFile, Span};
use syx_parser::Parser;

/// LSP `character` (UTF-16 code units) → byte index within `line`.
pub fn lsp_character_to_byte(line: &str, character: u32) -> usize {
    let target = character as usize;
    let mut units = 0usize;
    for (byte_idx, ch) in line.char_indices() {
        if units >= target {
            return byte_idx;
        }
        units += ch.len_utf16();
    }
    line.len()
}

/// Byte offset of the start of `line` (0-based) in `source`.
pub fn lsp_line_byte_offset(source: &str, line: usize) -> usize {
    if line == 0 {
        return 0;
    }
    let mut current = 0usize;
    for (idx, ch) in source.char_indices() {
        if ch == '\n' {
            current += 1;
            if current == line {
                return idx + ch.len_utf8();
            }
        }
    }
    source.len()
}

pub fn lsp_position_to_byte_offset(source: &str, line: usize, character: u32) -> usize {
    let line_text = source.lines().nth(line).unwrap_or("");
    lsp_line_byte_offset(source, line) + lsp_character_to_byte(line_text, character)
}

fn expr_span(expr: &Expr) -> Span {
    match expr {
        Expr::Literal { span, .. }
        | Expr::Ident { span, .. }
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
        | Expr::Closure { span, .. }
        | Expr::Group { span, .. }
        | Expr::Array { span, .. }
        | Expr::Tuple { span, .. }
        | Expr::Await { span, .. }
        | Expr::OptionalChain { span, .. }
        | Expr::ForceUnwrap { span, .. }
        | Expr::HalfOpenRange { span, .. }
        | Expr::ClosedRange { span, .. }
        | Expr::RangeDyn { span, .. }
        | Expr::VariantPattern { span, .. }
        | Expr::TuplePattern { span, .. }
        | Expr::TaskBlock { span, .. }
        | Expr::Borrow { span, .. }
        | Expr::KeyPath { span, .. }
        | Expr::MacroInvoke { span, .. }
        | Expr::OptionalCast { span, .. }
        | Expr::Is { span, .. }
        | Expr::Cast { span, .. }
        | Expr::TypeSelf { span, .. }
        | Expr::Super { span, .. } => *span,
    }
}

fn partial_member_prefix(source: &str, byte_offset: usize) -> String {
    let line_start = source[..byte_offset.min(source.len())]
        .rfind('\n')
        .map(|i| i + 1)
        .unwrap_or(0);
    let prefix = &source[line_start..byte_offset.min(source.len())];
    let Some(dot) = prefix.rfind('.') else {
        return String::new();
    };
    prefix[dot + 1..].to_string()
}

fn visit_expr(expr: &Expr, f: &mut dyn FnMut(&Expr)) {
    f(expr);
    match expr {
        Expr::Binary { left, right, .. } => {
            visit_expr(left, f);
            visit_expr(right, f);
        }
        Expr::Unary { expr, .. } => visit_expr(expr, f),
        Expr::Call { callee, args, .. } => {
            visit_expr(callee, f);
            for arg in args {
                visit_expr(&arg.value, f);
            }
        }
        Expr::Member { object, .. } => visit_expr(object, f),
        Expr::Index { object, index, .. } => {
            visit_expr(object, f);
            visit_expr(index, f);
        }
        Expr::Assign { value, .. }
        | Expr::CompoundAssign { value, .. }
        | Expr::MemberAssign { value, .. }
        | Expr::IndexAssign { value, .. } => visit_expr(value, f),
        Expr::Closure { body, .. } => match body {
            syx_ast::ClosureBody::Expr(e) => visit_expr(e, f),
            syx_ast::ClosureBody::Block(stmts) => visit_stmts(stmts, f),
        },
        Expr::Group { expr, .. } => visit_expr(expr, f),
        Expr::Array { elements, .. } => {
            for el in elements {
                visit_expr(el, f);
            }
        }
        Expr::Await { expr, .. } => visit_expr(expr, f),
        Expr::OptionalChain { object, .. } => visit_expr(object, f),
        Expr::ForceUnwrap { expr, .. } => visit_expr(expr, f),
        Expr::Borrow { expr, .. } => visit_expr(expr, f),
        Expr::TaskBlock { body, .. } => visit_stmts(body, f),
        _ => {}
    }
}

fn visit_stmts(stmts: &[Stmt], f: &mut dyn FnMut(&Expr)) {
    for stmt in stmts {
        match stmt {
            Stmt::VarDecl {
                init: Some(expr), ..
            }
            | Stmt::Expr { expr, .. }
            | Stmt::Print { expr, .. }
            | Stmt::Return {
                value: Some(expr), ..
            }
            | Stmt::Throw { expr, .. } => visit_expr(expr, f),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                visit_expr(condition, f);
                visit_stmts(then_branch, f);
                if let Some(branch) = else_branch {
                    visit_stmts(branch, f);
                }
            }
            Stmt::While {
                condition, body, ..
            } => {
                visit_expr(condition, f);
                visit_stmts(body, f);
            }
            Stmt::ForIn { iterable, body, .. } => {
                visit_expr(iterable, f);
                visit_stmts(body, f);
            }
            Stmt::Block { stmts, .. } => visit_stmts(stmts, f),
            Stmt::Try {
                body, catch_body, ..
            } => {
                visit_stmts(body, f);
                visit_stmts(catch_body, f);
            }
            Stmt::Switch {
                subject,
                cases,
                default,
                ..
            } => {
                visit_expr(subject, f);
                for case in cases {
                    visit_expr(&case.pattern, f);
                    if let Some(g) = &case.guard {
                        visit_expr(g, f);
                    }
                    visit_stmts(&case.body, f);
                }
                if let Some(def) = default {
                    visit_stmts(def, f);
                }
            }
            Stmt::IfCase {
                pattern,
                subject,
                body,
                else_branch,
                ..
            } => {
                visit_expr(pattern, f);
                visit_expr(subject, f);
                visit_stmts(body, f);
                if let Some(branch) = else_branch {
                    visit_stmts(branch, f);
                }
            }
            Stmt::IfLet {
                pattern,
                body,
                else_branch,
                ..
            } => {
                visit_expr(pattern, f);
                visit_stmts(body, f);
                if let Some(branch) = else_branch {
                    visit_stmts(branch, f);
                }
            }
            Stmt::GuardLet {
                pattern,
                body,
                else_branch,
                ..
            } => {
                visit_expr(pattern, f);
                visit_stmts(body, f);
                visit_stmts(&else_branch, f);
            }
            Stmt::RepeatWhile {
                body, condition, ..
            } => {
                visit_stmts(body, f);
                visit_expr(condition, f);
            }
            Stmt::MacroInvoke { args, .. } => {
                for arg in args {
                    visit_expr(arg, f);
                }
            }
            _ => {}
        }
    }
}

fn visit_module_decls(decls: &[Decl], f: &mut dyn FnMut(&Expr)) {
    for decl in decls {
        match decl {
            Decl::Function { body, .. } | Decl::Init { body, .. } => visit_stmts(body, f),
            Decl::Class { members, .. } | Decl::Actor { members, .. } => {
                visit_module_decls(members, f);
            }
            Decl::Extension { methods, .. } => visit_module_decls(methods, f),
            Decl::VarGlobal {
                init: Some(expr), ..
            } => visit_expr(expr, f),
            _ => {}
        }
    }
}

fn visit_module(module: &Module, f: &mut dyn FnMut(&Expr)) {
    visit_module_decls(&module.decls, f);
}

/// Innermost receiver expression when the cursor is completing a member after `.`.
pub fn receiver_expr_at_offset(module: &Module, _source: &str, byte_offset: usize) -> Option<Expr> {
    let mut best: Option<(Expr, u32)> = None;
    visit_module(module, &mut |expr| {
        let Expr::Member { object, span, .. } = expr else {
            return;
        };
        let obj_end = expr_span(object.as_ref()).end();
        if byte_offset >= obj_end && byte_offset <= span.end() + 1 {
            let len = span.len;
            if best.as_ref().map(|(_, l)| len < *l).unwrap_or(true) {
                best = Some((object.as_ref().clone(), len));
            }
        }
    });
    best.map(|(e, _)| e)
}

pub fn member_access_context_text(line: &str, byte_col: usize) -> Option<(String, String)> {
    let on_dot = line.as_bytes().get(byte_col) == Some(&b'.');
    let end = if on_dot {
        byte_col + 1
    } else {
        byte_col.min(line.len())
    };
    let prefix = &line[..end];
    let dot = prefix.rfind('.')?;
    let before_dot = prefix[..dot].trim_end();
    let start = before_dot
        .rfind(|c: char| {
            !c.is_alphanumeric() && c != '_' && c != '.' && c != ')' && c != ']' && c != '>'
        })
        .map(|i| i + 1)
        .unwrap_or(0);
    let object = before_dot[start..].trim().to_string();
    if object.is_empty() {
        return None;
    }
    let partial = prefix[dot + 1..].to_string();
    Some((object, partial))
}

/// Whether `line[open + 2..]` still needs a closing `)` for the interpolation at `open`.
pub fn interpolation_needs_close_paren(line: &str, open: usize, _byte_col: usize) -> bool {
    let after_open = &line[open + 2..];
    let mut depth = 1i32;
    for c in after_open.chars() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return false;
                }
            }
            _ => {}
        }
    }
    true
}

fn interpolation_open_before(line: &str, byte_col: usize) -> Option<usize> {
    let before = &line[..byte_col.min(line.len())];
    let open = before.rfind("\\(")?;
    let after_open = &line[open + 2..];
    let rel_col = byte_col.saturating_sub(open + 2);
    let mut depth = 0i32;
    for c in after_open.chars().take(rel_col) {
        match c {
            '(' => depth += 1,
            ')' if depth > 0 => depth -= 1,
            ')' => return None,
            _ => {}
        }
    }
    Some(open)
}

/// Member access inside string interpolation `\(...)` e.g. `"\(self.name)"`.
pub fn interpolation_member_context(line: &str, byte_col: usize) -> Option<(String, String)> {
    let open = interpolation_open_before(line, byte_col)?;
    let after_open = &line[open + 2..];
    let rel_col = byte_col.saturating_sub(open + 2);
    let segment = &after_open[..rel_col.min(after_open.len())];
    member_access_context_text(segment, segment.len())
}

/// Bare identifier prefix inside `\(...)` e.g. `"\(name)"` or `"\(na"`.
pub fn interpolation_identifier_context(line: &str, byte_col: usize) -> Option<String> {
    let open = interpolation_open_before(line, byte_col)?;
    let after_open = &line[open + 2..];
    let rel_col = byte_col.saturating_sub(open + 2);
    let segment = &after_open[..rel_col.min(after_open.len())];
    if segment.contains('.') {
        return None;
    }
    let start = segment
        .rfind(|c: char| !c.is_alphanumeric() && c != '_')
        .map(|i| i + 1)
        .unwrap_or(0);
    let partial = segment[start..].to_string();
    if partial.is_empty() || partial.chars().all(|c| c.is_alphanumeric() || c == '_') {
        Some(partial)
    } else {
        None
    }
}

pub fn parse_simple_member_expr(name: &str) -> Option<Expr> {
    parse_receiver_expr(name)
}

pub fn parse_receiver_expr(object_str: &str) -> Option<Expr> {
    let trimmed = object_str.trim();
    if trimmed.is_empty() {
        return None;
    }
    if trimmed.starts_with('[') || trimmed.ends_with(')') || trimmed.ends_with(']') {
        let synthetic = format!("func __syx_lsp_tmp() {{\n  return {trimmed}\n}}");
        let source = SourceFile::new("__syx_lsp_tmp.sy", synthetic);
        if let Ok(module) = Parser::parse(source) {
            for decl in &module.decls {
                if let Decl::Function { body, .. } = decl {
                    for stmt in body {
                        if let Stmt::Return {
                            value: Some(expr), ..
                        } = stmt
                        {
                            return Some(expr.clone());
                        }
                    }
                }
            }
        }
    }
    if trimmed.contains('.') || trimmed.chars().all(|c| c.is_alphanumeric() || c == '_') {
        let parts: Vec<&str> = trimmed.split('.').collect();
        let mut expr = Expr::Ident {
            span: Span::DUMMY,
            name: SmolStr::new(parts[0].trim_end_matches('(')),
        };
        for part in parts.iter().skip(1) {
            expr = Expr::Member {
                span: Span::DUMMY,
                object: Box::new(expr),
                name: SmolStr::new(*part),
            };
        }
        return Some(expr);
    }
    None
}

/// Byte offset in `source` of the last character of the receiver name before `.`.
pub fn member_receiver_name_end_byte(source: &str, line: usize, character: u32) -> Option<usize> {
    let line_text = source.lines().nth(line)?;
    let byte_col = lsp_character_to_byte(line_text, character);
    let (object_str, _) = member_access_context_text(line_text, byte_col)?;
    let dot_byte = if line_text.as_bytes().get(byte_col) == Some(&b'.') {
        byte_col
    } else {
        line_text[..byte_col.min(line_text.len())].rfind('.')? + 1
    };
    if object_str.len() > dot_byte {
        return None;
    }
    Some(lsp_line_byte_offset(source, line) + dot_byte - 1)
}

/// Resolve the static type of a member receiver for completion (original buffer + semantic).
pub fn receiver_type_at_position(
    semantic: &syx_semantic::SemanticAnalyzer,
    receiver: &Expr,
    source: &str,
    line: usize,
    character: u32,
) -> syx_semantic::TypeId {
    use syx_semantic::TypeId::*;

    // Prefer binding / symbol type for bare identifiers so Span::DUMMY receivers
    // (completion) are not confused with unrelated expr_types cache entries.
    if let Expr::Ident { name, .. } = receiver {
        let at = lsp_position_to_byte_offset(source, line, character) as u32;
        if let Some(bind_ty) = semantic.binding_type_before(name, at) {
            if !matches!(bind_ty, Unknown) {
                return bind_ty;
            }
        }
        if let Some(sym_ty) = semantic.symbol_type(name.as_str()) {
            if !matches!(sym_ty, Unknown) {
                return sym_ty;
            }
        }
    }

    let ty = semantic.type_of_expr(receiver);
    if !matches!(ty, Unknown) {
        return ty;
    }
    if let Expr::Ident { .. } = receiver {
        // Do not fall back to line-wide expr_types for identifiers — that often
        // picks an earlier Array/String literal on the same line or file.
        return Unknown;
    }
    if let Some(end_byte) = member_receiver_name_end_byte(source, line, character) {
        if let Some(cached) = semantic.type_of_expr_at_offset(end_byte as u32) {
            if !matches!(cached, Unknown) {
                return cached;
            }
        }
        let line_start = lsp_line_byte_offset(source, line) as u32;
        if let Some((_, best_ty)) = semantic
            .expr_types
            .iter()
            .filter(|(start, ty)| {
                **start >= line_start && **start <= end_byte as u32 && !matches!(ty, Unknown)
            })
            .max_by_key(|(start, _)| *start)
        {
            return best_ty.clone();
        }
    }
    let cursor = lsp_position_to_byte_offset(source, line, character) as u32;
    semantic.type_of_expr_at_offset(cursor).unwrap_or(Unknown)
}

const MEMBER_COMPLETION_PLACEHOLDER: &str = "__syx_lsp";

fn clean_member_partial(partial: &str) -> String {
    partial
        .strip_prefix(MEMBER_COMPLETION_PLACEHOLDER)
        .unwrap_or(partial)
        .to_string()
}

/// Resolve receiver AST + partial member prefix at an LSP position.
/// `original` is the editor buffer; `parsed` is optionally patched for parse (same byte offsets before the cursor).
pub fn receiver_for_completion(
    original: &str,
    parsed: &SourceFile,
    module: &Module,
    line: usize,
    character: u32,
) -> Option<(Expr, String)> {
    let line_text = original.lines().nth(line).unwrap_or("");
    let byte_col = lsp_character_to_byte(line_text, character);
    let byte_offset = lsp_position_to_byte_offset(original, line, character);
    let partial = clean_member_partial(&partial_member_prefix(original, byte_offset));

    if let Some((object_str, text_partial)) = interpolation_member_context(line_text, byte_col) {
        if let Some(expr) = parse_receiver_expr(&object_str) {
            let partial = if partial.is_empty() {
                clean_member_partial(&text_partial)
            } else {
                partial
            };
            return Some((expr, partial));
        }
    }

    if let Some(expr) = receiver_expr_at_offset(module, parsed.contents.as_ref(), byte_offset) {
        return Some((expr, partial));
    }
    // Cursor may sit on the dot while the AST only has a placeholder member name.
    if line_text.as_bytes().get(byte_col) == Some(&b'.') {
        if let Some((object_str, text_partial)) = member_access_context_text(line_text, byte_col) {
            if let Some(expr) = parse_receiver_expr(&object_str) {
                let partial = if partial.is_empty() {
                    clean_member_partial(&text_partial)
                } else {
                    partial
                };
                return Some((expr, partial));
            }
        }
    }

    if let Some((object_str, text_partial)) = member_access_context_text(line_text, byte_col) {
        if let Some(expr) = parse_receiver_expr(&object_str) {
            let partial = if partial.is_empty() {
                clean_member_partial(&text_partial)
            } else {
                partial
            };
            return Some((expr, partial));
        }
    }
    None
}

/// Insert a placeholder after `.` when the parser would reject incomplete member access.
pub fn patch_source_for_member_completion(source: &str, line: usize, character: u32) -> String {
    let line_text = source.lines().nth(line).unwrap_or("");
    let byte_col = lsp_character_to_byte(line_text, character);
    let on_dot = line_text.as_bytes().get(byte_col) == Some(&b'.');
    let prefix = &line_text[..byte_col.min(line_text.len())];
    let in_interp_member = interpolation_member_context(line_text, byte_col).is_some();
    let in_member_context = in_interp_member || on_dot || prefix.contains('.');
    if !in_member_context {
        return source.to_string();
    }

    let dot_end = if on_dot {
        byte_col + 1
    } else {
        prefix.rfind('.').map(|i| i + 1).unwrap_or(byte_col)
    };
    let partial: String = line_text[dot_end..]
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    if !partial.is_empty() {
        return source.to_string();
    }

    let insert_at = if on_dot {
        lsp_position_to_byte_offset(source, line, character) + 1
    } else {
        lsp_line_byte_offset(source, line) + dot_end
    };
    let mut patched = source.to_string();
    patched.insert_str(insert_at.min(patched.len()), MEMBER_COMPLETION_PLACEHOLDER);
    patched
}

fn extra_closing_parens_outside_strings(line: &str) -> String {
    let mut depth = 0i32;
    let mut in_str = false;
    let mut esc = false;
    let mut i = 0;
    let bytes = line.as_bytes();
    while i < bytes.len() {
        if esc {
            esc = false;
            i += 1;
            continue;
        }
        if in_str {
            if bytes[i] == b'\\' {
                if bytes.get(i + 1) == Some(&b'(') {
                    i += 2;
                    while i < bytes.len() {
                        match bytes[i] {
                            b'(' => {}
                            b')' => {
                                i += 1;
                                break;
                            }
                            _ => i += 1,
                        }
                    }
                    continue;
                }
                esc = true;
                i += 1;
                continue;
            }
            if bytes[i] == b'"' {
                in_str = false;
            }
            i += 1;
            continue;
        }
        match bytes[i] {
            b'"' => {
                in_str = true;
                i += 1;
            }
            b'(' => {
                depth += 1;
                i += 1;
            }
            b')' => {
                depth -= 1;
                i += 1;
            }
            _ => i += 1,
        }
    }
    if depth > 0 {
        ")".repeat(depth as usize)
    } else {
        String::new()
    }
}

/// Like [`patch_source_for_member_completion`], plus fixes that keep semantic analysis parseable.
pub fn patch_source_for_completion_semantic(source: &str, line: usize, character: u32) -> String {
    let orig_line = source.lines().nth(line).unwrap_or("");
    let orig_byte_col = lsp_character_to_byte(orig_line, character);
    let mut patched = patch_source_for_member_completion(source, line, character);
    if orig_line[..orig_byte_col.min(orig_line.len())].contains("\\(") {
        let before = &orig_line[..orig_byte_col.min(orig_line.len())];
        if let Some(open) = before.rfind("\\(") {
            let mut suffix = String::new();
            if interpolation_needs_close_paren(orig_line, open, orig_byte_col) {
                suffix.push(')');
            }
            let in_unclosed_string = orig_line[..orig_byte_col]
                .chars()
                .fold((false, false), |(in_str, esc), c| {
                    if esc {
                        return (in_str, false);
                    }
                    if c == '\\' {
                        return (in_str, true);
                    }
                    if c == '"' {
                        return (!in_str, false);
                    }
                    (in_str, false)
                })
                .0;
            if in_unclosed_string && !orig_line[orig_byte_col..].contains('"') {
                suffix.push('"');
            }
            if !suffix.is_empty() {
                let line_text = patched.lines().nth(line).unwrap_or("");
                let line_start = patched
                    .lines()
                    .take(line)
                    .map(|l| l.len() + 1)
                    .sum::<usize>();
                let line_end = line_start + line_text.len();
                patched.insert_str(line_end.min(patched.len()), &suffix);
                let updated_line = patched.lines().nth(line).unwrap_or("");
                let extra_paren = extra_closing_parens_outside_strings(updated_line);
                if !extra_paren.is_empty() {
                    let line_end = line_start + updated_line.len();
                    patched.insert_str(line_end.min(patched.len()), &extra_paren);
                }
            }
        }
    }
    let Some(line_text) = patched.lines().nth(line) else {
        return patched;
    };
    let trimmed = line_text.trim();
    if trimmed.starts_with("for ") && trimmed.contains(" in ") && !trimmed.contains('{') {
        let line_start = patched
            .lines()
            .take(line)
            .map(|l| l.len() + 1)
            .sum::<usize>();
        let line_end = line_start + line_text.len();
        patched.insert_str(line_end.min(patched.len()), " { }");
    }
    patched
}

#[cfg(test)]
mod tests {
    use super::*;
    use syx_parser::Parser;

    #[test]
    fn utf16_character_to_byte() {
        let line = "let xs = 1";
        assert_eq!(lsp_character_to_byte(line, 0), 0);
        assert_eq!(lsp_character_to_byte(line, 4), 4);
    }

    #[test]
    fn line_byte_offset_crlf() {
        let src = "a\r\nb\r\nc";
        assert_eq!(lsp_line_byte_offset(src, 0), 0);
        assert_eq!(lsp_line_byte_offset(src, 1), 3);
        assert_eq!(lsp_line_byte_offset(src, 2), 6);
        assert_eq!(lsp_position_to_byte_offset(src, 2, 0), 6);
    }

    #[test]
    fn receiver_finds_member_object() {
        let src = "func main() {\n  let xs = [1]\n  xs.map\n}";
        let source = SourceFile::new("t.sy", src);
        let module = Parser::parse(source.clone()).unwrap();
        let (expr, partial) =
            receiver_for_completion(src, &source, &module, 2, 7).expect("receiver");
        assert!(matches!(expr, Expr::Ident { name, .. } if name == "xs"));
        assert_eq!(partial, "ma");
    }

    #[test]
    fn patch_enables_parse_after_bare_dot() {
        let src = "func main() {\n  let xs = [1]\n  xs.\n}";
        let patched = patch_source_for_member_completion(src, 2, 5);
        let source = SourceFile::new("t.sy", patched);
        let module = Parser::parse(source).expect("patched source should parse");
        assert!(!module.decls.is_empty());
    }

    #[test]
    fn interpolation_member_context_self_dot() {
        let line = "    print(\"Hello, \\(self.";
        let col = line.len();
        let (obj, partial) = interpolation_member_context(line, col).expect("interpolation ctx");
        assert_eq!(obj, "self");
        assert_eq!(partial, "");
    }

    #[test]
    fn interpolation_identifier_context_name_partial() {
        let line = "    print(\"Hello, \\(na";
        let col = line.len();
        let partial = interpolation_identifier_context(line, col).expect("identifier ctx");
        assert_eq!(partial, "na");
    }

    #[test]
    fn interpolation_needs_close_paren_when_missing() {
        let line = "print(\"Hello, \\(na";
        assert!(interpolation_needs_close_paren(line, 13, line.len()));
    }

    #[test]
    fn interpolation_needs_close_paren_skips_existing() {
        let line = "print(\"Hello, \\(na)!\")";
        let open = line.find("\\(").unwrap();
        let col = open + 2 + 2; // after "na"
        assert!(!interpolation_needs_close_paren(line, open, col));
    }

    #[test]
    fn patch_interpolation_bare_identifier_parses() {
        let src = "async func main() {\n  let name: String = \"SYX\"\n  print(\"Hello, \\(na\n}";
        let line = src.lines().nth(2).unwrap();
        let character = line.len() as u32;
        let patched = patch_source_for_completion_semantic(src, 2, character);
        let source = SourceFile::new("t.sy", patched);
        let module = Parser::parse(source).expect("patched interpolation should parse");
        assert!(!module.decls.is_empty());
    }

    #[test]
    fn patch_interpolation_member_completion_closes_string() {
        let src = "print(\"Hello, \\(self.";
        let line = src;
        let character = line.len() as u32;
        let patched = patch_source_for_completion_semantic(src, 0, character);
        assert!(patched.contains("__syx_lsp"), "patched={patched}");
        assert!(
            patched.contains(")"),
            "expected closed interpolation: {patched}"
        );
    }

    #[test]
    fn patch_for_in_member_completion_adds_body() {
        let src = "async func main() {\n  let nums = [1]\n  for n in nums.\n}";
        let patched = patch_source_for_completion_semantic(src, 2, 17);
        assert!(patched.contains("for n in nums.__syx_lsp { }"));
        let source = syx_diagnostics::SourceFile::new("t.sy", patched);
        let module = syx_parser::Parser::parse(source).expect("for-in patch should parse");
        assert!(!module.decls.is_empty());
    }
}
