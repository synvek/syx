use syx_ast::{Decl, Module};
use syx_diagnostics::{SourceFile, SyxResult};
use syx_parser::Parser;

pub fn generate_markdown(source: &SourceFile) -> SyxResult<String> {
    generate_markdown_sources(std::slice::from_ref(source))
}

/// Generate combined API markdown for one or more source files.
pub fn generate_markdown_sources(sources: &[SourceFile]) -> SyxResult<String> {
    let mut out = String::new();
    out.push_str("# SYX Documentation\n\n");
    let mut any_decls = false;
    for source in sources {
        let module = Parser::parse(source.clone())?;
        if sources.len() > 1 {
            out.push_str(&format!("## Source: `{}`\n\n", source.name));
        } else {
            out.push_str(&format!("Source: `{}`\n\n", source.name));
        }
        if !module.decls.is_empty() {
            any_decls = true;
        }
        render_module_body(source, &module, &mut out);
    }
    if !any_decls {
        out.push_str("_No public declarations found._\n");
    }
    Ok(out)
}

pub fn render_module(source: &SourceFile, module: &Module) -> String {
    let mut out = String::new();
    out.push_str("# SYX Documentation\n\n");
    out.push_str(&format!("Source: `{}`\n\n", source.name));
    let before = out.len();
    render_module_body(source, module, &mut out);
    if out.len() == before {
        out.push_str("_No public declarations found._\n");
    }
    out
}

fn render_module_body(source: &SourceFile, module: &Module, out: &mut String) {
    for decl in &module.decls {
        render_decl(source, decl, out, 2);
    }
}

fn render_decl(source: &SourceFile, decl: &Decl, out: &mut String, heading_level: usize) {
    let heading = |level: usize, title: &str| -> String {
        format!("{} {}\n\n", "#".repeat(level.max(2).min(6)), title)
    };
    match decl {
        Decl::Function {
            name,
            params,
            return_type,
            ..
        } => {
            out.push_str(&heading(heading_level, &format!("`{name}`")));
            if let Some(doc) = doc_comment_for(source, name.as_str()) {
                out.push_str(&doc);
                out.push('\n');
            } else {
                out.push_str("_No documentation comment._\n\n");
            }
            out.push_str("```syx\n");
            out.push_str("func ");
            out.push_str(name);
            out.push('(');
            for (i, p) in params.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(&p.name);
            }
            out.push(')');
            if let Some(ty) = return_type {
                out.push_str(" -> ");
                out.push_str(&type_name(ty));
            }
            out.push_str("\n```\n\n");
        }
        Decl::Struct { name, fields, .. } => {
            out.push_str(&heading(heading_level, &format!("struct `{name}`")));
            if let Some(doc) = doc_comment_for(source, name.as_str()) {
                out.push_str(&doc);
                out.push('\n');
            }
            for (field, ty) in fields {
                out.push_str(&format!("- `{field}`: `{}`\n", type_name(ty)));
            }
            out.push('\n');
        }
        Decl::Class {
            name,
            superclass,
            members,
            ..
        } => {
            let title = if let Some(super_name) = superclass {
                format!("class `{name}`: `{super_name}`")
            } else {
                format!("class `{name}`")
            };
            out.push_str(&heading(heading_level, &title));
            if let Some(doc) = doc_comment_for(source, name.as_str()) {
                out.push_str(&doc);
                out.push('\n');
            }
            for member in members {
                render_decl(source, member, out, heading_level + 1);
            }
        }
        Decl::DataClass { name, fields, .. } => {
            out.push_str(&heading(heading_level, &format!("data class `{name}`")));
            if let Some(doc) = doc_comment_for(source, name.as_str()) {
                out.push_str(&doc);
                out.push('\n');
            }
            for (field, ty) in fields {
                out.push_str(&format!("- `{field}`: `{}`\n", type_name(ty)));
            }
            out.push('\n');
        }
        Decl::Protocol {
            name,
            inherited,
            methods,
            ..
        } => {
            let title = if inherited.is_empty() {
                format!("protocol `{name}`")
            } else {
                format!(
                    "protocol `{name}`: {}",
                    inherited
                        .iter()
                        .map(|s| format!("`{s}`"))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            };
            out.push_str(&heading(heading_level, &title));
            if let Some(doc) = doc_comment_for(source, name.as_str()) {
                out.push_str(&doc);
                out.push('\n');
            }
            for method in methods {
                render_decl(source, method, out, heading_level + 1);
            }
        }
        Decl::Enum { name, cases, .. } => {
            out.push_str(&heading(heading_level, &format!("enum `{name}`")));
            if let Some(doc) = doc_comment_for(source, name.as_str()) {
                out.push_str(&doc);
                out.push('\n');
            }
            for (case_name, _, _) in cases {
                out.push_str(&format!("- `{case_name}`\n"));
            }
            out.push('\n');
        }
        Decl::Actor { name, members, .. } => {
            out.push_str(&heading(heading_level, &format!("actor `{name}`")));
            if let Some(doc) = doc_comment_for(source, name.as_str()) {
                out.push_str(&doc);
                out.push('\n');
            }
            for member in members {
                render_decl(source, member, out, heading_level + 1);
            }
        }
        Decl::Extension {
            type_name,
            methods,
            ..
        } => {
            out.push_str(&heading(
                heading_level,
                &format!("extension `{type_name}`"),
            ));
            for method in methods {
                render_decl(source, method, out, heading_level + 1);
            }
        }
        Decl::Import { path, alias, .. } => {
            let path_str = path
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(".");
            let display = alias.as_ref().map(|a| a.to_string()).unwrap_or(path_str);
            out.push_str(&format!("- imports `{display}`\n"));
        }
        Decl::Export { item, .. } => {
            render_decl(source, item, out, heading_level);
        }
        _ => {}
    }
}

fn type_name(ty: &syx_ast::TypeExpr) -> String {
    match ty {
        syx_ast::TypeExpr::Named { name, .. } => name.to_string(),
        syx_ast::TypeExpr::Array(inner) => format!("[{}]", type_name(inner)),
        _ => "Unknown".into(),
    }
}

fn doc_comment_for(source: &SourceFile, symbol: &str) -> Option<String> {
    let mut pending = Vec::new();
    for line in source.contents.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("///") {
            pending.push(trimmed.trim_start_matches("///").trim().to_string());
            continue;
        }
        if trimmed.contains(symbol) && !pending.is_empty() {
            return Some(pending.join("\n\n"));
        }
        if !trimmed.is_empty() && !trimmed.starts_with("//") {
            pending.clear();
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_function_stub() {
        let source = SourceFile::new(
            "t.sy",
            "/// Greets the user\nfunc main() {\n    print(\"hi\")\n}",
        );
        let md = generate_markdown(&source).unwrap();
        assert!(md.contains("## `main`"));
        assert!(md.contains("Greets the user"));
    }

    #[test]
    fn renders_class_and_multiple_sources() {
        let a = SourceFile::new(
            "a.sy",
            "/// Matrix type\npublic class Mat {\n    public func rows() -> Int { return 0 }\n}",
        );
        let b = SourceFile::new("b.sy", "/// Helper\nfunc helper() {}");
        let md = generate_markdown_sources(&[a, b]).unwrap();
        assert!(md.contains("class `Mat`"));
        assert!(md.contains("Matrix type"));
        assert!(md.contains("## Source: `a.sy`"));
        assert!(md.contains("## Source: `b.sy`"));
        assert!(md.contains("## `helper`") || md.contains("### `helper`") || md.contains("`helper`"));
    }
}
