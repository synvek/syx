#![allow(dead_code)]
#![allow(unreachable_patterns)]
#![allow(unused_imports)]
use syx_ast::{
    ClosureBody,
    AccessLevel, Attribute, AttributeArg, BinOp, CompoundAssignOp, Decl, Expr, FunctionEffect,
    Literal, MacroImplementation, Module, Stmt, StringPart, UnaryOp,
};
use syx_diagnostics::{SourceFile, SyxDiagnostic, SyxResult};
use syx_parser::Parser;

pub fn format_source(source: &SourceFile) -> SyxResult<String> {
    let module = Parser::parse(source.clone())?;
    format_module(&module)
}

/// ADR-142 / M1-E04: lossless CST subset formatter (trivia-preserving).
///
/// Used by [`format_source_auto`] (W873 CST-first) and explicit CST-only callers.
pub fn format_source_cst(source: &SourceFile) -> SyxResult<String> {
    let root = syx_cst::parse_cst_subset(source)?;
    Ok(syx_cst::format_cst(&root))
}

/// W873 / ADR-1217: CST-first with AST fallback on `CST001`.
///
/// - `SYX_FMT_AST=1` → force AST (rollback / negative fixtures).
/// - else try CST; on `CST001` fall back to AST; other errors propagate.
/// - `SYX_FMT_CST` is ignored (superseded by default CST-first).
pub fn format_source_auto(source: &SourceFile) -> SyxResult<String> {
    match std::env::var("SYX_FMT_AST") {
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true") => return format_source(source),
        _ => {}
    }
    match format_source_cst(source) {
        Ok(s) => Ok(s),
        Err(e) if e.code.as_deref() == Some("CST001") => format_source(source),
        Err(e) => Err(e),
    }
}

/// W683 / C-06: dry-run report (never writes disk).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DryRunReport {
    pub formatted: String,
    /// `true` when formatted output differs from input contents.
    pub would_change: bool,
}

/// Format without writing; assert idempotency `format(format(x)) == format(x)`.
///
/// Uses [`format_source_auto`] (W873: CST-first + CST001→AST).
/// Failures (F001 / parse) return `Err` — caller must not write.
pub fn format_dry_run(source: &SourceFile) -> SyxResult<DryRunReport> {
    let once = format_source_auto(source)?;
    let again_src = SourceFile::new(source.name.clone(), once.as_str());
    let twice = format_source_auto(&again_src)?;
    if once != twice {
        return Err(SyxDiagnostic::error(
            "syx-fmt: dry-run idempotency failed (format(format(x)) != format(x))",
        )
        .with_code("F002"));
    }
    Ok(DryRunReport {
        would_change: once.as_str() != source.contents.as_ref(),
        formatted: once,
    })
}

/// W872 / ADR-1216: CST-only dry-run (no env — safe for parallel tests).
///
/// Failures (CST001 / parse) return `Err` — caller must not write.
pub fn format_dry_run_cst(source: &SourceFile) -> SyxResult<DryRunReport> {
    let once = format_source_cst(source)?;
    let again_src = SourceFile::new(source.name.clone(), once.as_str());
    let twice = format_source_cst(&again_src)?;
    if once != twice {
        return Err(SyxDiagnostic::error(
            "syx-fmt: CST dry-run idempotency failed (format_cst(format_cst(x)) != format_cst(x))",
        )
        .with_code("F002"));
    }
    Ok(DryRunReport {
        would_change: once.as_str() != source.contents.as_ref(),
        formatted: once,
    })
}

pub fn format_module(module: &Module) -> SyxResult<String> {
    let mut out = String::new();
    for decl in &module.decls {
        format_decl(decl, &mut out)?;
        out.push('\n');
    }
    Ok(out)
}

fn unsupported(what: &str) -> SyxDiagnostic {
    SyxDiagnostic::error(format!(
        "syx-fmt: unsupported {what} (M0-A02 fail-closed; refusing to rewrite source)"
    ))
    .with_code("F001")
}

/// Parser desugar of `is <Type>` in switch/if-case patterns uses `Ident("__switch_subject")`.
fn is_desugar_is_pattern(pattern: &Expr) -> bool {
    matches!(
        pattern,
        Expr::Is { expr, .. } if matches!(
            expr.as_ref(),
            Expr::Ident { name, .. } if name.as_str() == "__switch_subject"
        )
    )
}

fn format_decl(decl: &Decl, out: &mut String) -> SyxResult<()> {
    format_decl_at(decl, out, 0)
}

fn format_attrs(attrs: &[Attribute], out: &mut String, pad: &str) -> SyxResult<()> {
    for a in attrs {
        // W900: `static` is a keyword attr from parser desugar — emit as keyword, not `@static`.
        if a.name.as_str() == "static" {
            continue;
        }
        out.push_str(pad);
        out.push('@');
        out.push_str(&a.name);
        if !a.args.is_empty() {
            out.push('(');
            for (i, arg) in a.args.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                match arg {
                    AttributeArg::Ident(s) => out.push_str(s),
                    AttributeArg::String(s) => {
                        out.push('"');
                        out.push_str(s);
                        out.push('"');
                    }
                }
            }
            out.push(')');
        }
        out.push('\n');
    }
    Ok(())
}

fn attrs_has_static(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|a| a.name.as_str() == "static")
}

fn format_access(access: AccessLevel, out: &mut String) {
    match access {
        AccessLevel::Public => out.push_str("public "),
        AccessLevel::Private => out.push_str("private "),
        AccessLevel::FilePrivate => out.push_str("fileprivate "),
        AccessLevel::Internal => {}
    }
}

/// W685–W687 / C-06: Class/Init/Extension/Protocol/Actor/Deinit + access honesty.
fn format_decl_at(decl: &Decl, out: &mut String, indent: usize) -> SyxResult<()> {
    let pad = "    ".repeat(indent);
    match decl {
        Decl::Function {
            name,
            params,
            body,
            is_async,
            return_type,
            is_override,
            is_final,
            type_params,
            throws_type,
            where_clause,
            effect,
            attrs,
            access,
            ..
        } => {
            // W771–W774 / C-06: Function type_params + throws + effect + where 首片。
            format_attrs(attrs, out, &pad)?;
            out.push_str(&pad);
            format_access(*access, out);
            if *is_override {
                out.push_str("override ");
            }
            if *is_final {
                out.push_str("final ");
            }
            // W900: emit desugared `@static` as `static` keyword (idempotent).
            if attrs_has_static(attrs) {
                out.push_str("static ");
            }
            if *is_async {
                out.push_str("async ");
            }
            out.push_str("func ");
            out.push_str(name);
            if !type_params.is_empty() {
                out.push('<');
                for (i, tp) in type_params.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(tp);
                }
                out.push('>');
            }
            out.push('(');
            format_params(params, out)?;
            out.push(')');
            if let Some(ty) = return_type {
                out.push_str(" -> ");
                format_type_expr(ty, out)?;
            }
            if throws_type.is_some() {
                out.push_str(" throws");
            }
            if let Some(eff) = effect {
                match eff {
                    FunctionEffect::Pure => out.push_str(" requires Pure"),
                    FunctionEffect::Io => out.push_str(" requires IO"),
                }
            }
            if !where_clause.is_empty() {
                out.push_str(" where ");
                for (i, pred) in where_clause.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(&pred.type_name);
                    out.push_str(": ");
                    out.push_str(&pred.constraint);
                }
            }
            out.push_str(" {\n");
            for stmt in body {
                format_stmt(stmt, out, indent + 1)?;
            }
            out.push_str(&pad);
            out.push_str("}\n");
            Ok(())
        }
        Decl::Class {
            name,
            type_params,
            type_param_defaults,
            superclass,
            conformances,
            members,
            attrs,
            access,
            ..
        } => {
            format_attrs(attrs, out, &pad)?;
            out.push_str(&pad);
            format_access(*access, out);
            out.push_str("class ");
            out.push_str(name);
            if !type_params.is_empty() {
                out.push('<');
                for (i, p) in type_params.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(p);
                    if let Some(Some(def)) = type_param_defaults.get(i) {
                        out.push_str(" = ");
                        format_type_expr(def, out)?;
                    }
                }
                out.push('>');
            }
            let mut after_colon = false;
            if let Some(super_name) = superclass {
                out.push_str(": ");
                out.push_str(super_name);
                after_colon = true;
            }
            for c in conformances {
                if after_colon {
                    out.push_str(", ");
                } else {
                    out.push_str(": ");
                    after_colon = true;
                }
                out.push_str(c);
            }
            out.push_str(" {\n");
            for m in members {
                format_decl_at(m, out, indent + 1)?;
            }
            out.push_str(&pad);
            out.push_str("}\n");
            Ok(())
        }
        Decl::Init {
            params,
            body,
            access,
            failable,
            throws,
            ..
        } => {
            out.push_str(&pad);
            format_access(*access, out);
            out.push_str("init");
            if *failable {
                out.push('?');
            }
            out.push('(');
            format_params(params, out)?;
            out.push(')');
            if *throws {
                out.push_str(" throws");
            }
            out.push_str(" {\n");
            for stmt in body {
                format_stmt(stmt, out, indent + 1)?;
            }
            out.push_str(&pad);
            out.push_str("}\n");
            Ok(())
        }
        Decl::Deinit { body, access, .. } => {
            out.push_str(&pad);
            format_access(*access, out);
            out.push_str("deinit {\n");
            for stmt in body {
                format_stmt(stmt, out, indent + 1)?;
            }
            out.push_str(&pad);
            out.push_str("}\n");
            Ok(())
        }
        Decl::Actor {
            name,
            type_params,
            conformances,
            members,
            access,
            ..
        } => {
            out.push_str(&pad);
            format_access(*access, out);
            out.push_str("actor ");
            out.push_str(name);
            if !type_params.is_empty() {
                out.push('<');
                for (i, p) in type_params.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(p);
                }
                out.push('>');
            }
            if !conformances.is_empty() {
                out.push_str(": ");
                for (i, c) in conformances.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(c);
                }
            }
            out.push_str(" {\n");
            for m in members {
                format_decl_at(m, out, indent + 1)?;
            }
            out.push_str(&pad);
            out.push_str("}\n");
            Ok(())
        }
        Decl::Import { path, alias, .. } => {
            out.push_str(&pad);
            out.push_str("import ");
            for (i, seg) in path.iter().enumerate() {
                if i > 0 {
                    out.push('.');
                }
                out.push_str(seg);
            }
            if let Some(a) = alias {
                out.push_str(" as ");
                out.push_str(a);
            }
            out.push('\n');
            Ok(())
        }
        Decl::Export { item, .. } => {
            out.push_str(&pad);
            out.push_str("export ");
            // Item is formatted at indent 0 so its first line continues after "export ".
            let mut buf = String::new();
            format_decl_at(item, &mut buf, 0)?;
            out.push_str(&buf);
            Ok(())
        }
        Decl::ExportList { names, .. } => {
            out.push_str(&pad);
            out.push_str("export { ");
            for (i, name) in names.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(name);
            }
            out.push_str(" }\n");
            Ok(())
        }
        Decl::VarGlobal {
            name,
            init,
            mutable,
            ty,
            will_set,
            did_set,
            computed_getter,
            computed_setter,
            is_lazy,
            is_override,
            attrs,
            access,
            ..
        } => {
            // W693: willSet/didSet；W695: get-only；W696: lazy；W697: get+set；W698: override.
            // W874: lazy + computed get/get+set uses the same layout (prefix `lazy `).
            format_attrs(attrs, out, &pad)?;
            out.push_str(&pad);
            format_access(*access, out);
            if *is_override {
                out.push_str("override ");
            }
            if *is_lazy {
                out.push_str("lazy ");
            }
            if *mutable {
                out.push_str("var ");
            } else {
                out.push_str("let ");
            }
            out.push_str(name);
            if let Some(t) = ty {
                out.push_str(": ");
                format_type_expr(t, out)?;
            }
            if let Some(getter) = computed_getter {
                // Computed get / get+set: mutually exclusive with init/observers.
                out.push_str(" {\n");
                out.push_str(&pad);
                out.push_str("    get {\n");
                for stmt in getter {
                    format_stmt(stmt, out, indent + 2)?;
                }
                out.push_str(&pad);
                out.push_str("    }\n");
                if let Some(stmts) = computed_setter {
                    out.push_str(&pad);
                    out.push_str("    set {\n");
                    for stmt in stmts {
                        format_stmt(stmt, out, indent + 2)?;
                    }
                    out.push_str(&pad);
                    out.push_str("    }\n");
                }
                out.push_str(&pad);
                out.push_str("}\n");
                return Ok(());
            }
            // Setter without getter is ill-formed.
            if computed_setter.is_some() {
                return Err(unsupported("computed setter without getter"));
            }
            if let Some(expr) = init {
                out.push_str(" = ");
                format_expr(expr, out, indent)?;
            }
            out.push('\n');
            if let Some(stmts) = will_set {
                out.push_str(&pad);
                out.push_str("    willSet {\n");
                for stmt in stmts {
                    format_stmt(stmt, out, indent + 2)?;
                }
                out.push_str(&pad);
                out.push_str("    }\n");
            }
            if let Some(stmts) = did_set {
                out.push_str(&pad);
                out.push_str("    didSet {\n");
                for stmt in stmts {
                    format_stmt(stmt, out, indent + 2)?;
                }
                out.push_str(&pad);
                out.push_str("    }\n");
            }
            Ok(())
        }
        Decl::Struct {
            name,
            fields,
            members,
            access,
            ..
        } => {
            out.push_str(&pad);
            format_access(*access, out);
            out.push_str("struct ");
            out.push_str(name);
            out.push_str(" {\n");
            if members.is_empty() {
                for (field, ty) in fields {
                    out.push_str(&pad);
                    out.push_str("    ");
                    out.push_str(field);
                    out.push_str(": ");
                    format_type_expr(ty, out)?;
                    out.push('\n');
                }
            } else {
                for member in members {
                    format_decl_at(member, out, indent + 1)?;
                }
            }
            out.push_str(&pad);
            out.push_str("}\n");
            Ok(())
        }
        Decl::Extension {
            type_name,
            conformances,
            where_clause,
            methods,
            ..
        } => {
            // W686–W766 / C-06: Extension + where 首片（WherePred 直印 type_name: constraint）。
            out.push_str(&pad);
            out.push_str("extension ");
            out.push_str(type_name);
            if !conformances.is_empty() {
                out.push_str(": ");
                for (i, c) in conformances.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(c);
                }
            }
            if !where_clause.is_empty() {
                out.push_str(" where ");
                for (i, pred) in where_clause.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(&pred.type_name);
                    out.push_str(": ");
                    out.push_str(&pred.constraint);
                }
            }
            out.push_str(" {\n");
            for m in methods {
                format_decl_at(m, out, indent + 1)?;
            }
            out.push_str(&pad);
            out.push_str("}\n");
            Ok(())
        }
        Decl::Protocol {
            name,
            access,
            inherited,
            associated_types,
            methods,
            ..
        } => {
            // W686–W687 / W765 / C-06: Protocol + associatedtype 首片（哨兵 Any 省略 bound）。
            out.push_str(&pad);
            format_access(*access, out);
            out.push_str("protocol ");
            out.push_str(name);
            if !inherited.is_empty() {
                out.push_str(": ");
                for (i, p) in inherited.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(p);
                }
            }
            out.push_str(" {\n");
            let inner = "    ".repeat(indent + 1);
            for (assoc_name, constraint) in associated_types {
                out.push_str(&inner);
                out.push_str("associatedtype ");
                out.push_str(assoc_name);
                let is_any_sentinel = matches!(
                    constraint,
                    syx_ast::TypeExpr::Named { name, generics, .. }
                        if name.as_str() == "Any" && generics.is_empty()
                );
                if !is_any_sentinel {
                    out.push_str(": ");
                    format_type_expr(constraint, out)?;
                }
                out.push('\n');
            }
            for m in methods {
                format_decl_at(m, out, indent + 1)?;
            }
            out.push_str(&pad);
            out.push_str("}\n");
            Ok(())
        }
        Decl::Enum {
            name,
            access,
            attrs,
            type_params,
            cases,
            ..
        } => {
            // W688–W767 / C-06: Enum + type_params 首片（SmolStr 直印 <T[, …]>）。
            format_attrs(attrs, out, &pad)?;
            out.push_str(&pad);
            format_access(*access, out);
            out.push_str("enum ");
            out.push_str(name);
            if !type_params.is_empty() {
                out.push('<');
                for (i, tp) in type_params.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(tp);
                }
                out.push('>');
            }
            out.push_str(" {\n");
            for (case_name, value, fields) in cases {
                out.push_str(&pad);
                out.push_str("    ");
                out.push_str(case_name);
                if !fields.is_empty() {
                    out.push('(');
                    for (i, (field, ty)) in fields.iter().enumerate() {
                        if i > 0 {
                            out.push_str(", ");
                        }
                        out.push_str(field);
                        out.push_str(": ");
                        format_type_expr(ty, out)?;
                    }
                    out.push(')');
                }
                if let Some(v) = value {
                    out.push_str(" = ");
                    format_expr(v, out, indent)?;
                }
                out.push('\n');
            }
            out.push_str(&pad);
            out.push_str("}\n");
            Ok(())
        }
        Decl::TypeAlias {
            name,
            access,
            ty,
            ..
        } => {
            out.push_str(&pad);
            format_access(*access, out);
            out.push_str("typealias ");
            out.push_str(name);
            out.push_str(" = ");
            format_type_expr(ty, out)?;
            out.push('\n');
            Ok(())
        }
        Decl::DataClass {
            name,
            access,
            attrs,
            type_params,
            where_clause,
            fields,
            ..
        } => {
            // W689–W769 / C-06: DataClass + type_params + where 首片（WherePred 直印，`)` 后）。
            format_attrs(attrs, out, &pad)?;
            out.push_str(&pad);
            format_access(*access, out);
            out.push_str("data class ");
            out.push_str(name);
            if !type_params.is_empty() {
                out.push('<');
                for (i, tp) in type_params.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(tp);
                }
                out.push('>');
            }
            out.push('(');
            for (i, (field, ty)) in fields.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(field);
                out.push_str(": ");
                format_type_expr(ty, out)?;
            }
            out.push(')');
            if !where_clause.is_empty() {
                out.push_str(" where ");
                for (i, pred) in where_clause.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(&pred.type_name);
                    out.push_str(": ");
                    out.push_str(&pred.constraint);
                }
            }
            out.push('\n');
            Ok(())
        }
        Decl::FinalClass {
            name,
            access,
            type_params,
            variants,
            ..
        } => {
            // W690–W770 / C-06: FinalClass + type_params 首片（`<T[, …]>`，name 后、`{` 前）。
            out.push_str(&pad);
            format_access(*access, out);
            out.push_str("final class ");
            out.push_str(name);
            if !type_params.is_empty() {
                out.push('<');
                for (i, tp) in type_params.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(tp);
                }
                out.push('>');
            }
            out.push_str(" {\n");
            for (variant_name, fields) in variants {
                out.push_str(&pad);
                out.push_str("    ");
                out.push_str(variant_name);
                if !fields.is_empty() {
                    out.push('(');
                    for (i, (field, ty)) in fields.iter().enumerate() {
                        if i > 0 {
                            out.push_str(", ");
                        }
                        out.push_str(field);
                        out.push_str(": ");
                        format_type_expr(ty, out)?;
                    }
                    out.push(')');
                }
                out.push('\n');
            }
            out.push_str(&pad);
            out.push_str("}\n");
            Ok(())
        }
        Decl::GlobalActor { name, access, .. } => {
            // W691 / C-06: GlobalActor 首片（parser 空体 MVP）。
            out.push_str(&pad);
            format_access(*access, out);
            out.push_str("global actor ");
            out.push_str(name);
            out.push_str(" {\n");
            out.push_str(&pad);
            out.push_str("}\n");
            Ok(())
        }
        Decl::Subscript {
            params,
            return_type,
            getter,
            setter,
            ..
        } => {
            // W692 / C-06: Subscript get-only；W694：补 setter（get+set）。
            out.push_str(&pad);
            out.push_str("subscript(");
            format_params(params, out)?;
            out.push_str(") -> ");
            format_type_expr(return_type, out)?;
            out.push_str(" {\n");
            out.push_str(&pad);
            out.push_str("    get {\n");
            for stmt in getter {
                format_stmt(stmt, out, indent + 2)?;
            }
            out.push_str(&pad);
            out.push_str("    }\n");
            if let Some(stmts) = setter {
                out.push_str(&pad);
                out.push_str("    set {\n");
                for stmt in stmts {
                    format_stmt(stmt, out, indent + 2)?;
                }
                out.push_str(&pad);
                out.push_str("    }\n");
            }
            out.push_str(&pad);
            out.push_str("}\n");
            Ok(())
        }
        Decl::OperatorFunc {
            op,
            params,
            return_type,
            body,
            attrs,
            access,
            ..
        } => {
            // W699 / C-06: OperatorFunc 首片（镜像 Function，无 override/final/async）。
            format_attrs(attrs, out, &pad)?;
            out.push_str(&pad);
            format_access(*access, out);
            out.push_str("operator func ");
            out.push_str(op);
            out.push('(');
            format_params(params, out)?;
            out.push(')');
            if let Some(ty) = return_type {
                out.push_str(" -> ");
                format_type_expr(ty, out)?;
            }
            out.push_str(" {\n");
            for stmt in body {
                format_stmt(stmt, out, indent + 1)?;
            }
            out.push_str(&pad);
            out.push_str("}\n");
            Ok(())
        }
        Decl::MacroDecl {
            name,
            implementation,
            attrs,
            access,
            ..
        } => {
            match implementation {
                // W704 / C-06: BuiltIn MacroDecl — `macro Name = #builtInMacro("id")`.
                MacroImplementation::BuiltIn { id } => {
                    format_attrs(attrs, out, &pad)?;
                    out.push_str(&pad);
                    format_access(*access, out);
                    out.push_str("macro ");
                    out.push_str(name);
                    out.push_str(" = #builtInMacro(\"");
                    out.push_str(id);
                    out.push_str("\")\n");
                    Ok(())
                }
                // W700–W706 / C-06: User MacroDecl；非空 decl_body 递归 format_decl.
                MacroImplementation::User {
                    params,
                    body,
                    decl_body,
                } => {
                    format_attrs(attrs, out, &pad)?;
                    out.push_str(&pad);
                    format_access(*access, out);
                    out.push_str("#macro ");
                    out.push_str(name);
                    out.push('(');
                    for (i, p) in params.iter().enumerate() {
                        if i > 0 {
                            out.push_str(", ");
                        }
                        out.push_str(p);
                    }
                    out.push(')');
                    out.push_str(" {\n");
                    for decl in decl_body {
                        format_decl_at(decl, out, indent + 1)?;
                    }
                    for stmt in body {
                        format_stmt(stmt, out, indent + 1)?;
                    }
                    out.push_str(&pad);
                    out.push_str("}\n");
                    Ok(())
                }
                MacroImplementation::External { module, type_name } => {
                    // W705 / C-06: External MacroDecl — `macro Name = #externalMacro(module: "…", type: "…")`.
                    format_attrs(attrs, out, &pad)?;
                    out.push_str(&pad);
                    format_access(*access, out);
                    out.push_str("macro ");
                    out.push_str(name);
                    out.push_str(" = #externalMacro(module: \"");
                    out.push_str(module);
                    out.push_str("\", type: \"");
                    out.push_str(type_name);
                    out.push_str("\")\n");
                    Ok(())
                }
            }
        }
        Decl::MacroSiteInvoke { name, args, .. } => {
            // W701 / C-06: MacroSiteInvoke — 模块级 `#name(args)`.
            out.push_str(&pad);
            format_macro_invoke(name, args, out)?;
            out.push('\n');
            Ok(())
        }
        Decl::TopLevelStmt { stmt, .. } => {
            // W703 / C-06: TopLevelStmt 首片 — 模块顶层 Stmt（indent=0）.
            format_stmt(stmt, out, 0)
        }
        // Keep catch-all for future Decl variants (fail-closed F001).
        #[allow(unreachable_patterns)]
        other => Err(unsupported(&format!(
            "declaration {:?}",
            std::mem::discriminant(other)
        ))),
    }
}

/// Shared `#name(args)` printer for MacroSiteInvoke / Expr·Stmt MacroInvoke (W702).
fn format_macro_invoke(name: &str, args: &[Expr], out: &mut String) -> SyxResult<()> {
    out.push('#');
    out.push_str(name);
    out.push('(');
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        format_expr(arg, out, 0)?;
    }
    out.push(')');
    Ok(())
}

fn format_params(params: &[syx_ast::Param], out: &mut String) -> SyxResult<()> {
    for (i, p) in params.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        format_param(p, out)?;
    }
    Ok(())
}

fn format_param(p: &syx_ast::Param, out: &mut String) -> SyxResult<()> {
    // W775 / C-06: parameter attributes inline (`@Name[(args)] ` before label/name).
    for a in &p.attrs {
        out.push('@');
        out.push_str(&a.name);
        if !a.args.is_empty() {
            out.push('(');
            for (i, arg) in a.args.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                match arg {
                    AttributeArg::Ident(s) => out.push_str(s),
                    AttributeArg::String(s) => {
                        out.push('"');
                        out.push_str(s);
                        out.push('"');
                    }
                }
            }
            out.push(')');
        }
        out.push(' ');
    }
    // W776 / C-06: isolated/sending keywords (after attrs, before label/name).
    if p.isolated {
        out.push_str("isolated ");
    }
    if p.sending {
        out.push_str("sending ");
    }
    match &p.external_label {
        syx_ast::ParamLabel::Same => {}
        syx_ast::ParamLabel::Named(label) => {
            out.push_str(label);
            out.push(' ');
        }
        syx_ast::ParamLabel::Omitted => {
            out.push_str("_ ");
        }
    }
    out.push_str(&p.name);
    if let Some(ty) = &p.ty {
        out.push_str(": ");
        format_type_expr(ty, out)?;
    }
    if let Some(default) = &p.default {
        out.push_str(" = ");
        format_expr(default, out, 0)?;
    }
    Ok(())
}

fn format_type_expr(ty: &syx_ast::TypeExpr, out: &mut String) -> SyxResult<()> {
    match ty {
        syx_ast::TypeExpr::Named { name, generics, .. } => {
            out.push_str(name);
            if !generics.is_empty() {
                out.push('<');
                for (i, g) in generics.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    format_type_expr(g, out)?;
                }
                out.push('>');
            }
            Ok(())
        }
        syx_ast::TypeExpr::Dictionary { key, value } => {
            out.push_str("Dictionary<");
            format_type_expr(key, out)?;
            out.push_str(", ");
            format_type_expr(value, out)?;
            out.push('>');
            Ok(())
        }
        syx_ast::TypeExpr::Array(inner) => {
            out.push('[');
            format_type_expr(inner, out)?;
            out.push(']');
            Ok(())
        }
        syx_ast::TypeExpr::Void => {
            out.push_str("Void");
            Ok(())
        }
        // W732 / C-06: Tuple type for Seq destructure annotations — `(Int, Int)` / `(x: Int, y: Int)`.
        syx_ast::TypeExpr::Tuple { elements } => {
            out.push('(');
            for (i, (label, t)) in elements.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                if let Some(l) = label {
                    out.push_str(l);
                    out.push_str(": ");
                }
                format_type_expr(t, out)?;
            }
            out.push(')');
            Ok(())
        }
        // W782 / C-06: TypeExpr::Optional first slice — postfix `T?`.
        syx_ast::TypeExpr::Optional(inner) => {
            format_type_expr(inner, out)?;
            out.push('?');
            Ok(())
        }
        // W783 / C-06: TypeExpr::OpaqueSome first slice — `some T`.
        syx_ast::TypeExpr::OpaqueSome(inner) => {
            out.push_str("some ");
            format_type_expr(inner, out)
        }
        // W784 / C-06: TypeExpr::OpaqueAny first slice — `any T`.
        syx_ast::TypeExpr::OpaqueAny(inner) => {
            out.push_str("any ");
            format_type_expr(inner, out)
        }
        // W785 / C-06: TypeExpr::Function first slice — `(T, U) -> Ret` / `()`.
        syx_ast::TypeExpr::Function { params, returns } => {
            out.push('(');
            for (i, p) in params.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                format_type_expr(p, out)?;
            }
            out.push(')');
            if let Some(ret) = returns {
                out.push_str(" -> ");
                format_type_expr(ret, out)?;
            }
            Ok(())
        }
        // W786 / C-06: TypeExpr::Reference first slice — `&T` / `&mut T` / `&'a T`.
        syx_ast::TypeExpr::Reference(inner, mutable, lifetime) => {
            out.push('&');
            if let Some(lt) = lifetime {
                out.push_str(lt);
                out.push(' ');
            }
            if *mutable {
                out.push_str("mut ");
            }
            format_type_expr(inner, out)
        }
        other => Err(unsupported(&format!("type {:?}", std::mem::discriminant(other)))),
    }
}

fn format_stmt(stmt: &Stmt, out: &mut String, indent: usize) -> SyxResult<()> {
    let pad = "    ".repeat(indent);
    match stmt {
        Stmt::Print { expr, .. } => {
            out.push_str(&pad);
            out.push_str("print(");
            format_expr(expr, out, indent)?;
            out.push_str(")\n");
            Ok(())
        }
        Stmt::Return { value: Some(v), .. } => {
            out.push_str(&pad);
            out.push_str("return ");
            format_expr(v, out, indent)?;
            out.push('\n');
            Ok(())
        }
        Stmt::Return { value: None, .. } => {
            out.push_str(&pad);
            out.push_str("return\n");
            Ok(())
        }
        Stmt::VarDecl {
            name,
            ty,
            init,
            mutable,
            ..
        } => {
            out.push_str(&pad);
            if *mutable {
                out.push_str("var ");
            } else {
                out.push_str("let ");
            }
            out.push_str(name);
            if let Some(t) = ty {
                out.push_str(": ");
                format_type_expr(t, out)?;
            }
            if let Some(v) = init {
                out.push_str(" = ");
                format_expr(v, out, indent)?;
            }
            out.push('\n');
            Ok(())
        }
        Stmt::Expr { expr, .. } => {
            out.push_str(&pad);
            format_expr(expr, out, indent)?;
            out.push('\n');
            Ok(())
        }
        Stmt::If {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            out.push_str(&pad);
            out.push_str("if ");
            format_expr(condition, out, indent)?;
            if then_branch.len() == 1 && !matches!(then_branch[0], Stmt::Block { .. }) {
                out.push('\n');
                format_stmt(&then_branch[0], out, indent)?;
            } else {
                out.push_str(" {\n");
                for s in then_branch {
                    format_stmt(s, out, indent + 1)?;
                }
                out.push_str(&pad);
                out.push_str("}");
            }
            if let Some(else_b) = else_branch {
                if else_b.len() == 1 && !matches!(else_b[0], Stmt::Block { .. }) {
                    out.push_str(" else\n");
                    format_stmt(&else_b[0], out, indent)?;
                    out.push('\n');
                } else {
                    out.push_str(" else {\n");
                    for s in else_b {
                        format_stmt(s, out, indent + 1)?;
                    }
                    out.push_str(&pad);
                    out.push_str("}\n");
                }
            } else if then_branch.len() != 1 || matches!(then_branch[0], Stmt::Block { .. }) {
                out.push('\n');
            }
            Ok(())
        }
        Stmt::While { condition, body, .. } => {
            out.push_str(&pad);
            out.push_str("while ");
            format_expr(condition, out, indent)?;
            if body.len() == 1 && !matches!(body[0], Stmt::Block { .. }) {
                out.push('\n');
                format_stmt(&body[0], out, indent)?;
            } else {
                out.push_str(" {\n");
                for s in body {
                    format_stmt(s, out, indent + 1)?;
                }
                out.push_str(&pad);
                out.push_str("}\n");
            }
            Ok(())
        }
        Stmt::ForIn {
            name,
            iterable,
            body,
            is_async_iter,
            ..
        } => {
            out.push_str(&pad);
            if *is_async_iter {
                out.push_str("for await ");
            } else {
                out.push_str("for ");
            }
            out.push_str(name);
            out.push_str(" in ");
            format_expr(iterable, out, indent)?;
            out.push_str(" {\n");
            for s in body {
                format_stmt(s, out, indent + 1)?;
            }
            out.push_str(&pad);
            out.push_str("}\n");
            Ok(())
        }
        Stmt::Block { stmts, .. } => {
            out.push_str(&pad);
            out.push_str("{\n");
            for s in stmts {
                format_stmt(s, out, indent + 1)?;
            }
            out.push_str(&pad);
            out.push_str("}\n");
            Ok(())
        }
        Stmt::Break { .. } => {
            out.push_str(&pad);
            out.push_str("break\n");
            Ok(())
        }
        Stmt::Continue { .. } => {
            out.push_str(&pad);
            out.push_str("continue\n");
            Ok(())
        }
        Stmt::MacroInvoke { name, args, .. } => {
            // W702 / C-06: Stmt MacroInvoke — `#name(args)` 语句位.
            out.push_str(&pad);
            format_macro_invoke(name, args, out)?;
            out.push('\n');
            Ok(())
        }
        Stmt::Throw { expr, .. } => {
            // W707 / C-06: Throw 首片 — `throw <expr>`.
            out.push_str(&pad);
            out.push_str("throw ");
            format_expr(expr, out, indent)?;
            out.push('\n');
            Ok(())
        }
        Stmt::Try {
            body,
            catch_name,
            catch_type,
            catch_body,
            ..
        } => {
            // W708 / C-06: Try 首片 — `try { … } catch (name[: Type]) { … }`.
            out.push_str(&pad);
            out.push_str("try {\n");
            for s in body {
                format_stmt(s, out, indent + 1)?;
            }
            out.push_str(&pad);
            out.push_str("} catch (");
            out.push_str(catch_name);
            if let Some(ty) = catch_type {
                out.push_str(": ");
                format_type_expr(ty, out)?;
            }
            out.push_str(") {\n");
            for s in catch_body {
                format_stmt(s, out, indent + 1)?;
            }
            out.push_str(&pad);
            out.push_str("}\n");
            Ok(())
        }
        Stmt::Switch {
            subject,
            cases,
            default,
            ..
        } => {
            // W719 / W733–W761 / C-06: Switch — Literal | HalfOpenRange | ClosedRange |
            // VariantPattern | TuplePattern | Ident | Binary | Call | Member | Index |
            // OptionalChain | ForceUnwrap | KeyPath | Array | Unary | Await | Cast |
            // OptionalCast | TypeSelf | Super | Borrow | MacroInvoke | Closure | TaskBlock |
            // Assign | CompoundAssign | MemberAssign | IndexAssign | Is [+ where].
            // Still reject: other patterns (non-whitelist).
            // Desugar Is must print `is <Type>` — bare format_expr leaks __switch_subject.
            for c in cases {
                let allowed = matches!(
                    c.pattern,
                    Expr::Literal { .. }
                        | Expr::HalfOpenRange { .. }
                        | Expr::ClosedRange { .. }
                        | Expr::VariantPattern { .. }
                        | Expr::TuplePattern { .. }
                        | Expr::Ident { .. }
                        | Expr::Binary { .. }
                        | Expr::Call { .. }
                        | Expr::Member { .. }
                        | Expr::Index { .. }
                        | Expr::OptionalChain { .. }
                        | Expr::ForceUnwrap { .. }
                        | Expr::KeyPath { .. }
                        | Expr::Array { .. }
                        | Expr::Unary { .. }
                        | Expr::Await { .. }
                        | Expr::Cast { .. }
                        | Expr::OptionalCast { .. }
                        | Expr::TypeSelf { .. }
                        | Expr::Super { .. }
                        | Expr::Borrow { .. }
                        | Expr::MacroInvoke { .. }
                        | Expr::Closure { .. }
                        | Expr::TaskBlock { .. }
                        | Expr::Assign { .. }
                        | Expr::CompoundAssign { .. }
                        | Expr::MemberAssign { .. }
                        | Expr::IndexAssign { .. }
                        | Expr::Is { .. }
                );
                if !allowed {
                    return Err(unsupported("switch case pattern"));
                }
            }
            out.push_str(&pad);
            out.push_str("switch ");
            format_expr(subject, out, indent)?;
            out.push_str(" {\n");
            for c in cases {
                out.push_str(&pad);
                out.push_str("case ");
                if let Expr::Is { target, .. } = &c.pattern {
                    if is_desugar_is_pattern(&c.pattern) {
                        out.push_str("is ");
                        format_type_expr(target, out)?;
                    } else {
                        // W761 / C-06: non-desugar Is — `y is Int` via format_expr.
                        format_expr(&c.pattern, out, indent)?;
                    }
                } else {
                    format_expr(&c.pattern, out, indent)?;
                }
                if let Some(g) = &c.guard {
                    out.push_str(" where ");
                    format_expr(g, out, indent)?;
                }
                out.push_str(":\n");
                for s in &c.body {
                    format_stmt(s, out, indent + 1)?;
                }
            }
            if let Some(def) = default {
                out.push_str(&pad);
                out.push_str("default:\n");
                for s in def {
                    format_stmt(s, out, indent + 1)?;
                }
            }
            out.push_str(&pad);
            out.push_str("}\n");
            Ok(())
        }
        Stmt::IfLet {
            name,
            pattern,
            body,
            else_branch,
            ..
        } => {
            // W720 / C-06: IfLet 首片 — `if let name = pattern { … } [else { … }]`（恒花括号）.
            out.push_str(&pad);
            out.push_str("if let ");
            out.push_str(name);
            out.push_str(" = ");
            format_expr(pattern, out, indent)?;
            out.push_str(" {\n");
            for s in body {
                format_stmt(s, out, indent + 1)?;
            }
            out.push_str(&pad);
            out.push('}');
            if let Some(else_b) = else_branch {
                out.push_str(" else {\n");
                for s in else_b {
                    format_stmt(s, out, indent + 1)?;
                }
                out.push_str(&pad);
                out.push_str("}\n");
            } else {
                out.push('\n');
            }
            Ok(())
        }
        Stmt::GuardLet {
            name,
            pattern,
            body,
            else_branch,
            ..
        } => {
            // W721 / C-06: GuardLet 首片 — `guard let name = pattern else { … }\n{ … }`（先 else 后 body）.
            out.push_str(&pad);
            out.push_str("guard let ");
            out.push_str(name);
            out.push_str(" = ");
            format_expr(pattern, out, indent)?;
            out.push_str(" else {\n");
            for s in else_branch {
                format_stmt(s, out, indent + 1)?;
            }
            out.push_str(&pad);
            out.push_str("}\n");
            out.push_str(&pad);
            out.push_str("{\n");
            for s in body {
                format_stmt(s, out, indent + 1)?;
            }
            out.push_str(&pad);
            out.push_str("}\n");
            Ok(())
        }
        Stmt::RepeatWhile {
            body,
            condition,
            ..
        } => {
            // W722 / C-06: RepeatWhile 首片 — `repeat { … } while cond`（恒花括号）.
            out.push_str(&pad);
            out.push_str("repeat {\n");
            for s in body {
                format_stmt(s, out, indent + 1)?;
            }
            out.push_str(&pad);
            out.push_str("} while ");
            format_expr(condition, out, indent)?;
            out.push('\n');
            Ok(())
        }
        Stmt::IfCase {
            pattern,
            subject,
            body,
            else_branch,
            ..
        } => {
            // W723–W724 / W762–W764 / C-06: IfCase — Variant|Tuple|Literal|Range|Is.
            if !matches!(
                pattern,
                Expr::VariantPattern { .. }
                    | Expr::TuplePattern { .. }
                    | Expr::Literal { .. }
                    | Expr::HalfOpenRange { .. }
                    | Expr::ClosedRange { .. }
                    | Expr::Is { .. }
            ) {
                return Err(unsupported("if case pattern"));
            }
            out.push_str(&pad);
            out.push_str("if case ");
            if let Expr::Is { target, .. } = pattern {
                if is_desugar_is_pattern(pattern) {
                    // W764: desugar `is Dog` — do not leak __switch_subject.
                    out.push_str("is ");
                    format_type_expr(target, out)?;
                } else {
                    format_expr(pattern, out, indent)?;
                }
            } else {
                format_expr(pattern, out, indent)?;
            }
            out.push_str(" = ");
            format_expr(subject, out, indent)?;
            out.push_str(" {\n");
            for s in body {
                format_stmt(s, out, indent + 1)?;
            }
            out.push_str(&pad);
            out.push('}');
            if let Some(else_b) = else_branch {
                out.push_str(" else {\n");
                for s in else_b {
                    format_stmt(s, out, indent + 1)?;
                }
                out.push_str(&pad);
                out.push_str("}\n");
            } else {
                out.push('\n');
            }
            Ok(())
        }
        // W732 / C-06: Stmt::Seq 元组解构首片 — 识别 `__tuple_*` desugar，重建表面 `let`/`var (a, b) [= rhs]`.
        Stmt::Seq { stmts, .. } => match match_tuple_destructure_seq(stmts) {
            Some((mutable, names, ty, init)) => {
                out.push_str(&pad);
                if mutable {
                    out.push_str("var ");
                } else {
                    out.push_str("let ");
                }
                out.push('(');
                for (i, n) in names.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    out.push_str(n);
                }
                out.push(')');
                if let Some(t) = ty {
                    out.push_str(": ");
                    format_type_expr(t, out)?;
                }
                if let Some(v) = init {
                    out.push_str(" = ");
                    format_expr(v, out, indent)?;
                }
                out.push('\n');
                Ok(())
            }
            None => Err(unsupported("statement Seq")),
        },
        other => Err(unsupported(&format!("statement {:?}", std::mem::discriminant(other)))),
    }
}

/// Recognize parser desugar of `let`/`var (a, b[, …]) [: Ty] [= rhs]` → Seq of `__tuple_*` + Member binds.
fn match_tuple_destructure_seq<'a>(
    stmts: &'a [Stmt],
) -> Option<(
    bool,
    Vec<&'a str>,
    Option<&'a syx_ast::TypeExpr>,
    Option<&'a Expr>,
)> {
    if stmts.len() < 2 {
        return None;
    }
    let (tmp_name, ty, init) = match &stmts[0] {
        Stmt::VarDecl {
            name,
            ty,
            init,
            mutable: false,
            ..
        } if name.starts_with("__tuple_") => (name.as_str(), ty.as_ref(), init.as_ref()),
        _ => return None,
    };
    let bindings = &stmts[1..];
    let mut names = Vec::with_capacity(bindings.len());
    let mut bind_mutable: Option<bool> = None;
    for (i, s) in bindings.iter().enumerate() {
        match s {
            Stmt::VarDecl {
                name,
                ty: None,
                init:
                    Some(Expr::Member {
                        object,
                        name: idx,
                        ..
                    }),
                mutable,
                ..
            } => {
                let ok_obj = matches!(
                    object.as_ref(),
                    Expr::Ident { name: n, .. } if n.as_str() == tmp_name
                );
                let expected = i.to_string();
                if !ok_obj || idx.as_str() != expected {
                    return None;
                }
                if let Some(m) = bind_mutable {
                    if m != *mutable {
                        return None;
                    }
                } else {
                    bind_mutable = Some(*mutable);
                }
                names.push(name.as_str());
            }
            _ => return None,
        }
    }
    Some((bind_mutable.unwrap_or(false), names, ty, init))
}

fn format_expr(expr: &Expr, out: &mut String, indent: usize) -> SyxResult<()> {
    match expr {
        Expr::Literal { value, .. } => {
            match value {
                Literal::String(s) => {
                    out.push('"');
                    out.push_str(s);
                    out.push('"');
                }
                Literal::Integer { value, suffix } => {
                    out.push_str(&value.to_string());
                    if let Some(s) = suffix {
                        out.push_str(s.as_str());
                    }
                }
                Literal::BigInt(text) => {
                    // Lexer stores decimal digits without `n`; reconstruct surface form.
                    out.push_str(text);
                    out.push('n');
                }
                Literal::Double { value, suffix } => {
                    out.push_str(&value.to_string());
                    if let Some(s) = suffix {
                        out.push_str(s.as_str());
                    }
                }
                Literal::Bool(v) => out.push_str(if *v { "true" } else { "false" }),
                Literal::Nil => out.push_str("nil"),
                Literal::Interpolated(parts) => {
                    // Surface: "text\(expr)…" — Text 直印（与 String 同诚实）；Expr → \( + format_expr + )
                    out.push('"');
                    for part in parts {
                        match part {
                            StringPart::Text(t) => out.push_str(t),
                            StringPart::Expr(e) => {
                                out.push('\\');
                                out.push('(');
                                format_expr(e, out, indent)?;
                                out.push(')');
                            }
                        }
                    }
                    out.push('"');
                }
                Literal::Regex(pattern) => {
                    // Lexer stores body without flanking `/`.
                    out.push('/');
                    out.push_str(pattern);
                    out.push('/');
                }
                Literal::Dict(entries) => {
                    out.push('{');
                    for (i, (k, v)) in entries.iter().enumerate() {
                        if i > 0 {
                            out.push_str(", ");
                        }
                        format_expr(k, out, indent)?;
                        out.push_str(": ");
                        format_expr(v, out, indent)?;
                    }
                    out.push('}');
                }
                #[allow(unreachable_patterns)]
                other => {
                    return Err(unsupported(&format!(
                        "literal {:?}",
                        std::mem::discriminant(other)
                    )));
                }
            }
            Ok(())
        }
        Expr::Ident { name, .. } => {
            out.push_str(name);
            Ok(())
        }
        Expr::Binary { left, op, right, .. } => {
            format_expr(left, out, indent)?;
            out.push(' ');
            out.push_str(match op {
                BinOp::Add => "+",
                BinOp::Sub => "-",
                BinOp::Mul => "*",
                BinOp::Div => "/",
                // W777 / C-06: BinOp::Mod first slice.
                BinOp::Mod => "%",
                BinOp::Eq => "==",
                BinOp::NotEq => "!=",
                BinOp::IdenticalEq => "===",
                BinOp::IdenticalNe => "!==",
                BinOp::Lt => "<",
                // W778 / C-06: BinOp::LtEq first slice.
                BinOp::LtEq => "<=",
                BinOp::Gt => ">",
                // W779 / C-06: BinOp::GtEq first slice.
                BinOp::GtEq => ">=",
                BinOp::And => "&&",
                BinOp::Or => "||",
                BinOp::BitAnd => "&",
                BinOp::BitOr => "|",
                BinOp::BitXor => "^",
                BinOp::Shl => "<<",
                BinOp::Shr => ">>",
                // W780 / C-06: BinOp::NilCoalesce first slice (Binary residual closed).
                BinOp::NilCoalesce => "??",
                other => {
                    return Err(unsupported(&format!(
                        "binop {:?}",
                        std::mem::discriminant(other)
                    )));
                }
            });
            out.push(' ');
            format_expr(right, out, indent)
        }
        Expr::Unary { op, expr, .. } => {
            // W781 / C-06: UnaryOp::Try first slice — postfix `expr?` (same AST as prefix `try expr`).
            if matches!(op, UnaryOp::Try) {
                format_expr(expr, out, indent)?;
                out.push('?');
                return Ok(());
            }
            out.push_str(match op {
                UnaryOp::Neg => "-",
                UnaryOp::Not => "!",
                UnaryOp::BitNot => "~",
                other => {
                    return Err(unsupported(&format!(
                        "unary {:?}",
                        std::mem::discriminant(other)
                    )));
                }
            });
            format_expr(expr, out, indent)
        }
        Expr::Call { callee, args, .. } => {
            format_expr(callee, out, indent)?;
            out.push('(');
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                if let Some(label) = &arg.label {
                    out.push_str(label);
                    out.push_str(": ");
                }
                format_expr(&arg.value, out, indent)?;
            }
            out.push(')');
            Ok(())
        }
        Expr::Member { object, name, .. } => {
            format_expr(object, out, indent)?;
            out.push('.');
            out.push_str(name);
            Ok(())
        }
        Expr::Index { object, index, .. } => {
            format_expr(object, out, indent)?;
            out.push('[');
            format_expr(index, out, indent)?;
            out.push(']');
            Ok(())
        }
        Expr::Array { elements, .. } => {
            out.push('[');
            for (i, el) in elements.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                format_expr(el, out, indent)?;
            }
            out.push(']');
            Ok(())
        }
        Expr::Assign { name, value, .. } => {
            out.push_str(name);
            out.push_str(" = ");
            format_expr(value, out, indent)
        }
        Expr::MemberAssign {
            object,
            name,
            value,
            ..
        } => {
            format_expr(object, out, indent)?;
            out.push('.');
            out.push_str(name);
            out.push_str(" = ");
            format_expr(value, out, indent)
        }
        Expr::Group { expr, .. } => {
            out.push('(');
            format_expr(expr, out, indent)?;
            out.push(')');
            Ok(())
        }
        Expr::MacroInvoke { name, args, .. } => {
            // W702 / C-06: Expr MacroInvoke — `#name(args)` 表达式位.
            format_macro_invoke(name, args, out)
        }
        Expr::Closure { params, body, .. } => {
            // W709 / C-06: Closure 首片 — `{ (params) in … }`；不重建 parser 丢弃的 `-> Ret`.
            let pad = "    ".repeat(indent);
            out.push_str("{ (");
            format_params(params, out)?;
            out.push_str(") in");
            match body {
                ClosureBody::Expr(e) => {
                    out.push(' ');
                    format_expr(e, out, indent)?;
                    out.push_str(" }");
                }
                ClosureBody::Block(stmts) => {
                    out.push('\n');
                    for s in stmts {
                        format_stmt(s, out, indent + 1)?;
                    }
                    out.push_str(&pad);
                    out.push('}');
                }
            }
            Ok(())
        }
        Expr::Await { expr, .. } => {
            // W710 / C-06: Await 首片 — `await <expr>`.
            out.push_str("await ");
            format_expr(expr, out, indent)
        }
        Expr::ForceUnwrap { expr, .. } => {
            // W711 / C-06: ForceUnwrap 首片 — `<expr>!`.
            format_expr(expr, out, indent)?;
            out.push('!');
            Ok(())
        }
        Expr::Cast { expr, target, .. } => {
            // W712 / C-06: Cast 首片 — `<expr> as <Type>`.
            format_expr(expr, out, indent)?;
            out.push_str(" as ");
            format_type_expr(target, out)
        }
        Expr::OptionalCast { expr, target, .. } => {
            // W713 / C-06: OptionalCast 首片 — `<expr> as? <Type>`.
            format_expr(expr, out, indent)?;
            out.push_str(" as? ");
            format_type_expr(target, out)
        }
        Expr::Is { expr, target, .. } => {
            // W714 / C-06: Is 首片 — `<expr> is <Type>`.
            format_expr(expr, out, indent)?;
            out.push_str(" is ");
            format_type_expr(target, out)
        }
        Expr::TypeSelf { type_name, .. } => {
            // W715 / C-06: TypeSelf 首片 — `<Type>.self`.
            out.push_str(type_name.as_str());
            out.push_str(".self");
            Ok(())
        }
        Expr::OptionalChain { object, name, .. } => {
            // W716 / C-06: OptionalChain 首片 — `<expr>?.<name>`.
            format_expr(object, out, indent)?;
            out.push_str("?.");
            out.push_str(name.as_str());
            Ok(())
        }
        Expr::HalfOpenRange { start, end, .. } => {
            // W717 / C-06: HalfOpenRange 首片 — `<i64>..<i64>`.
            out.push_str(&start.to_string());
            out.push_str("..<");
            out.push_str(&end.to_string());
            Ok(())
        }
        Expr::ClosedRange { start, end, .. } => {
            // W718 / C-06: ClosedRange 首片 — `<i64>...<i64>`.
            out.push_str(&start.to_string());
            out.push_str("...");
            out.push_str(&end.to_string());
            Ok(())
        }
        Expr::VariantPattern {
            variant,
            bindings,
            ..
        } => {
            // W723 / C-06: VariantPattern — `.name` / `.name(let x)` / labeled binds.
            out.push('.');
            out.push_str(variant);
            if !bindings.is_empty() {
                out.push('(');
                for (i, (label, name)) in bindings.iter().enumerate() {
                    if i > 0 {
                        out.push_str(", ");
                    }
                    if let Some(lab) = label {
                        out.push_str(lab);
                        out.push_str(": ");
                    }
                    out.push_str("let ");
                    out.push_str(name);
                }
                out.push(')');
            }
            Ok(())
        }
        Expr::TuplePattern { bindings, .. } => {
            // W724 / C-06: TuplePattern 首片 — `(a, b)`（不重建 parser 丢弃的 let）.
            out.push('(');
            for (i, name) in bindings.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(name);
            }
            out.push(')');
            Ok(())
        }
        Expr::TaskBlock {
            detached,
            body,
            ..
        } => {
            // W725 / C-06: TaskBlock 首片 — `Task {…}` / `Task.detached {…}`.
            let pad = "    ".repeat(indent);
            if *detached {
                out.push_str("Task.detached {");
            } else {
                out.push_str("Task {");
            }
            out.push('\n');
            for s in body {
                format_stmt(s, out, indent + 1)?;
            }
            out.push_str(&pad);
            out.push('}');
            Ok(())
        }
        Expr::Borrow {
            mutable,
            expr,
            ..
        } => {
            // W726 / C-06: Borrow 首片 — `&expr` / `&mut expr`.
            if *mutable {
                out.push_str("&mut ");
            } else {
                out.push('&');
            }
            format_expr(expr, out, indent)
        }
        Expr::KeyPath {
            root_type,
            property,
            optional,
            ..
        } => {
            // W727 / C-06: KeyPath 首片 — `\Root.prop` / `\Root?.prop`.
            out.push('\\');
            out.push_str(root_type);
            if *optional {
                out.push('?');
            }
            out.push('.');
            out.push_str(property);
            Ok(())
        }
        Expr::CompoundAssign {
            name,
            op,
            value,
            ..
        } => {
            // W728 / C-06: CompoundAssign 首片 — `name op= value`.
            out.push_str(name);
            out.push(' ');
            out.push_str(match op {
                CompoundAssignOp::Add => "+=",
                CompoundAssignOp::Sub => "-=",
                CompoundAssignOp::Mul => "*=",
                CompoundAssignOp::Div => "/=",
                CompoundAssignOp::Mod => "%=",
                CompoundAssignOp::BitAnd => "&=",
                CompoundAssignOp::BitOr => "|=",
                CompoundAssignOp::BitXor => "^=",
                CompoundAssignOp::Shl => "<<=",
                CompoundAssignOp::Shr => ">>=",
            });
            out.push(' ');
            format_expr(value, out, indent)
        }
        Expr::IndexAssign {
            object,
            index,
            value,
            ..
        } => {
            // W729 / C-06: IndexAssign 首片 — `object[index] = value`.
            format_expr(object, out, indent)?;
            out.push('[');
            format_expr(index, out, indent)?;
            out.push_str("] = ");
            format_expr(value, out, indent)
        }
        Expr::Tuple { elements, .. } => {
            // W730 / C-06: Tuple 字面首片 — `(a, b)` / `(x: 1, y: 2)`.
            out.push('(');
            for (i, (label, expr)) in elements.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                if let Some(name) = label {
                    out.push_str(name);
                    out.push_str(": ");
                }
                format_expr(expr, out, indent)?;
            }
            out.push(')');
            Ok(())
        }
        Expr::Super { .. } => {
            // W731 / C-06: Super 首片 — `super`（Member/Call 承接 .speak / .init）。
            out.push_str("super");
            Ok(())
        }
        other => Err(unsupported(&format!("expression {:?}", std::mem::discriminant(other)))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_function() {
        let source = SourceFile::new("t.sy", "func main() { print(\"hi\") }");
        let formatted = format_source(&source).unwrap();
        assert!(formatted.contains("func main"));
    }

    #[test]
    fn formats_if_and_while() {
        let source = SourceFile::new(
            "t.sy",
            "func main() { if (true) { print(1) } while (false) { break } }",
        );
        let formatted = format_source(&source).unwrap();
        assert!(formatted.contains("if (true)"));
        assert!(formatted.contains("while (false)"));
    }

    #[test]
    fn m0_a02_rejects_subscript_setter_decl_fail_closed() {
        // W693: observers supported; subscript setter remains M0-A02 F001.
        let source = SourceFile::new(
            "t.sy",
            "class Box {\n    subscript(i: Int) -> Int {\n        get { return 0 }\n        set { print(1) }\n    }\n}\nfunc main() { print(1) }",
        );
        let err = format_source(&source).expect_err("subscript setter must fail closed");
        assert!(
            err.message.contains("unsupported") || err.code.as_deref() == Some("F001"),
            "{err:?}"
        );
    }
}
