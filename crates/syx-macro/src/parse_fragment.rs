//! Parse attached macro body fragments into AST (ADR-070 M3).

use syx_ast::{Decl, Stmt};
use syx_diagnostics::{SourceFile, SyxResult};
use syx_parser::Parser;

/// Parse attached `body:` template as statement list.
pub fn parse_attached_body(src: &str) -> SyxResult<Vec<Stmt>> {
    let trimmed = src.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }
    let wrapped = if trimmed.starts_with("func ") || trimmed.starts_with("public func ") {
        trimmed.to_string()
    } else {
        format!("func __attached_body() {{\n{trimmed}\n}}")
    };
    let file = SourceFile::new("attached.sy", wrapped);
    let module = Parser::parse(file)?;
    for decl in module.decls {
        if let Decl::Function { body, .. } = decl {
            return Ok(body);
        }
    }
    Ok(Vec::new())
}

/// Parse one or more top-level declarations from attached macro templates.
pub fn parse_attached_decls(src: &str) -> SyxResult<Vec<Decl>> {
    let trimmed = src.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }
    let file = SourceFile::new("attached_decls.sy", trimmed.to_string());
    let module = Parser::parse(file)?;
    Ok(module.decls)
}

/// Parse a single extension member (`func` / `static func`).
pub fn parse_extension_member(src: &str) -> SyxResult<Decl> {
    let trimmed = src.trim();
    let wrapped = format!("extension __AttachedType {{\n{trimmed}\n}}");
    let file = SourceFile::new("attached_member.sy", wrapped);
    let module = Parser::parse(file)?;
    for decl in module.decls {
        if let Decl::Extension { methods, .. } = decl {
            if let Some(method) = methods.into_iter().next() {
                return Ok(method);
            }
        }
    }
    Err(syx_diagnostics::SyxDiagnostic::error(
        "failed to parse extension member fragment",
    ))
}
