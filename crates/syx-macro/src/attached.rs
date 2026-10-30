use smol_str::SmolStr;
use syx_ast::{AccessLevel, Attribute, AttributeArg, Decl, Expr, Literal, Stmt, TypeExpr};
use syx_diagnostics::{Span, SyxResult};

use crate::attached_config::attached_body_template_for;
use crate::parse_fragment::{parse_attached_body, parse_attached_decls, parse_extension_member};

fn merge_witness_where(
    entry: &mut (syx_diagnostics::Span, Vec<Decl>, Vec<SmolStr>, Vec<syx_ast::WherePred>),
    type_params: &[SmolStr],
    user_where: &[syx_ast::WherePred],
    protocols: &[&str],
) {
    let mut wc = user_where.to_vec();
    if !type_params.is_empty() && wc.is_empty() {
        wc = syx_mono::witness_constraint_matrix(type_params, protocols);
    }
    for pred in wc {
        if !entry
            .3
            .iter()
            .any(|p| p.type_name == pred.type_name && p.constraint == pred.constraint)
        {
            entry.3.push(pred);
        }
    }
}

pub(crate) fn expand_attached_macros(module: &mut syx_ast::Module) -> SyxResult<()> {
    use syx_ast::{AccessLevel, Decl};

    apply_builtin_macro_attrs(module);

    let member_body_template =
        attached_body_template_for("member", "SYX_ATTACHED_MEMBER_BODY", "print(\"attached\")");
    let peer_body_template =
        attached_body_template_for("peer", "SYX_ATTACHED_PEER_BODY", "print(\"peer\")");
    let custom_body_template =
        attached_body_template_for("custom", "SYX_ATTACHED_CUSTOM_BODY", "print(\"custom\")");
    let accessor_body_template =
        attached_body_template_for("accessor", "SYX_ATTACHED_ACCESSOR_BODY", "return self.value");
    let deinit_body_template =
        attached_body_template_for("deinit", "SYX_ATTACHED_DEINIT_BODY", "print(\"deinit\")");
    let extension_body_template = attached_body_template_for(
        "extension",
        "SYX_ATTACHED_EXTENSION_BODY",
        "func describe() -> String { return \"attached\" }",
    );

    let mut extensions: indexmap::IndexMap<
        smol_str::SmolStr,
        (syx_diagnostics::Span, Vec<Decl>, Vec<SmolStr>, Vec<syx_ast::WherePred>),
    > = indexmap::IndexMap::new();
    for decl in &module.decls {
        let (attrs, name, span, fields, type_params, where_clause) = match decl {
            Decl::DataClass {
                attrs,
                name,
                span,
                fields,
                type_params,
                where_clause,
                ..
            } => (
                attrs,
                name,
                span,
                Some(fields.as_slice()),
                type_params.as_slice(),
                where_clause.as_slice(),
            ),
            Decl::Class { attrs, name, span, .. } => (attrs, name, span, None, &[][..], &[][..]),
            _ => continue,
        };
        if has_equatable_attr(attrs) {
            if let Some(fields) = fields {
                let entry = extensions
                    .entry(name.clone())
                    .or_insert_with(|| (*span, Vec::new(), Vec::new(), Vec::new()));
                merge_witness_where(entry, type_params, where_clause, &["Equatable"]);
                entry.1.push(synthesize_equatable(name, fields, *span, AccessLevel::Internal));
            }
        }
        if has_hashable_attr(attrs) {
            if let Some(fields) = fields {
                let entry = extensions
                    .entry(name.clone())
                    .or_insert_with(|| (*span, Vec::new(), Vec::new(), Vec::new()));
                merge_witness_where(entry, type_params, where_clause, &["Hashable"]);
                entry.1.extend(synthesize_hashable(name, fields, *span)?);
            }
        }
        if has_codable_attr(attrs) {
            if let Some(fields) = fields {
                let entry = extensions
                    .entry(name.clone())
                    .or_insert_with(|| (*span, Vec::new(), Vec::new(), Vec::new()));
                merge_witness_where(entry, type_params, where_clause, &["Codable"]);
                entry.1.extend(synthesize_codable(name, fields, *span, type_params, where_clause)?);
            }
        }
        for proto in attached_conformance_protocols(attrs) {
            if let Some(fields) = fields {
                let entry = extensions
                    .entry(name.clone())
                    .or_insert_with(|| (*span, Vec::new(), Vec::new(), Vec::new()));
                match proto.as_str() {
                    "Equatable" => {
                        merge_witness_where(entry, type_params, where_clause, &["Equatable"]);
                        entry.1.push(synthesize_equatable(name, fields, *span, AccessLevel::Public));
                    }
                    "Hashable" => {
                        merge_witness_where(entry, type_params, where_clause, &["Hashable"]);
                        entry.1.extend(synthesize_hashable(name, fields, *span)?);
                    }
                    "Codable" => {
                        merge_witness_where(entry, type_params, where_clause, &["Codable"]);
                        entry.1.extend(synthesize_codable(name, fields, *span, type_params, where_clause)?);
                    }
                    other => {
                        return Err(syx_diagnostics::SyxDiagnostic::error(format!(
                            "attached(conformance) unknown protocol '{other}' (Equatable/Hashable/Codable only)"
                        ))
                        .with_code("E005"));
                    }
                }
            }
        }
        if let Some(spec) = external_type_macro_attr(attrs) {
            if let Some(fields) = fields {
                let member = attached_member_names(attrs)
                    .into_iter()
                    .next()
                    .unwrap_or_else(|| smol_str::SmolStr::new("externalMember"));
                let entry = extensions.entry(name.clone()).or_insert_with(|| (*span, Vec::new(), Vec::new(), Vec::new()));
                let field_names: Vec<SmolStr> = fields.iter().map(|(n, _)| n.clone()).collect();
                if spec.type_name == "CodableMacro" {
                    entry
                        .1
                        .extend(crate::external::expand_external_type_decls(
                            &spec.type_name,
                            name,
                            member.as_str(),
                            &field_names,
                        )?);
                } else {
                    let body = crate::external::expand_external_type_macro(
                        &spec.type_name,
                        name,
                        member.as_str(),
                        &field_names,
                    )?;
                    entry.1.push(Decl::Function {
                        span: *span,
                        access: AccessLevel::Internal,
                        attrs: vec![],
                        name: member,
                        type_params: vec![],
                        params: vec![],
                        return_type: None,
                        throws_type: None,
                        effect: None,
                        where_clause: vec![],
                        body,
                        is_async: false,
                    is_override: false,
            is_final: false,
                    });
                }
                let _ = fields;
                let _ = spec;
            }
        }
        let members = attached_member_names(attrs);
        let peers = attached_peer_names(attrs);
        let customs = attached_custom_names(attrs);
        let extension_slots = attached_kind_names(attrs, "extension");
        let extension_conformances = attached_extension_conformances(attrs);
        if !extension_slots.is_empty() {
            let entry = extensions.entry(name.clone()).or_insert_with(|| (*span, Vec::new(), Vec::new(), Vec::new()));
            if !extension_conformances.is_empty() {
                entry.2.extend(extension_conformances);
            }
            for slot in extension_slots {
                let body_src = attached_body_for_named(attrs, "extension", &slot)
                    .or_else(|| attached_body_from_attr(attrs, "extension"))
                    .unwrap_or_else(|| extension_body_template.clone())
                    .replace("{type}", name.as_str())
                    .replace("{name}", slot.as_str());
                if body_src.trim_start().starts_with("func ") {
                    entry.1.push(parse_extension_member(&body_src)?);
                } else {
                    entry.1.extend(parse_attached_decls(&body_src)?);
                }
            }
        }
        if members.is_empty() && peers.is_empty() && customs.is_empty() {
            continue;
        }
        let entry = extensions.entry(name.clone()).or_insert_with(|| (*span, Vec::new(), Vec::new(), Vec::new()));
        for member in members {
            let body_src = attached_body_from_attr(attrs, "member")
                .unwrap_or_else(|| member_body_template.clone())
                .replace("{type}", name.as_str())
                .replace("{name}", member.as_str());
            entry.1.push(Decl::Function {
                span: *span,
                access: AccessLevel::Internal,
                attrs: vec![],
                name: member,
                type_params: vec![],
                params: vec![],
                return_type: None,
                throws_type: None,
                effect: None,
                where_clause: vec![],
                body: attached_body_stmts(*span, &body_src)?,
                is_async: false,
            is_override: false,
            is_final: false,
            });
        }
        for (peer_name, _strategy) in peers {
            let body_src = peer_body_template
                .replace("{type}", name.as_str())
                .replace("{name}", peer_name.as_str());
            entry.1.push(Decl::Function {
                span: *span,
                access: AccessLevel::Internal,
                attrs: vec![syx_ast::Attribute {
                    name: smol_str::SmolStr::new("peer"),
                    args: vec![syx_ast::AttributeArg::Ident(peer_name.clone())],
                }],
                name: peer_name,
                type_params: vec![],
                params: vec![],
                return_type: None,
                throws_type: None,
                effect: None,
                where_clause: vec![],
                body: attached_body_stmts(*span, &body_src)?,
                is_async: false,
            is_override: false,
            is_final: false,
            });
        }
        for custom in customs {
            let body_src = custom_body_template
                .replace("{type}", name.as_str())
                .replace("{name}", custom.as_str());
            entry.1.push(Decl::Function {
                span: *span,
                access: AccessLevel::Internal,
                attrs: vec![syx_ast::Attribute {
                    name: smol_str::SmolStr::new("custom"),
                    args: vec![syx_ast::AttributeArg::Ident(custom.clone())],
                }],
                name: custom,
                type_params: vec![],
                params: vec![],
                return_type: None,
                throws_type: None,
                effect: None,
                where_clause: vec![],
                body: attached_body_stmts(*span, &body_src)?,
                is_async: false,
            is_override: false,
            is_final: false,
            });
        }
    }
    for decl in &module.decls {
        if let Decl::Enum {
            attrs,
            name,
            cases,
            span,
            ..
        } = decl
        {
            if !has_codable_attr(attrs)
                && !attached_conformance_protocols(attrs)
                    .iter()
                    .any(|p| p.as_str() == "Codable")
            {
                continue;
            }
            let entry = extensions
                .entry(name.clone())
                .or_insert_with(|| (*span, Vec::new(), Vec::new(), Vec::new()));
            entry.1.extend(synthesize_enum_codable(name, cases, *span)?);
        }
    }
    for decl in &mut module.decls {
        match decl {
            Decl::Class { attrs, members, span, .. } => {
                expand_attached_deinit(attrs, members, *span, &deinit_body_template)?;
                expand_attached_accessors(members, *span, &accessor_body_template)?;
            }
            _ => {}
        }
    }
    for (type_name, (span, methods, conformances, where_clause)) in extensions {
        module.decls.push(Decl::Extension {
            span,
            type_name,
            conformances,
            where_clause,
            methods,
        });
    }
    Ok(())
}

pub(crate) fn attached_body_template(env_key: &str, default: &str) -> String {
    std::env::var(env_key).unwrap_or_else(|_| default.to_string())
}

fn attached_body_from_attr(attrs: &[syx_ast::Attribute], kind: &str) -> Option<String> {
    use syx_ast::AttributeArg;
    for attr in attrs {
        if attr.name.as_str() != "attached" {
            continue;
        }
        let ids: Vec<&str> = attr
            .args
            .iter()
            .filter_map(|a| match a {
                AttributeArg::Ident(id) => Some(id.as_str()),
                _ => None,
            })
            .collect();
        if ids.first() == Some(&kind) {
            for arg in &attr.args {
                if let AttributeArg::String(s) = arg {
                    return Some(s.clone());
                }
            }
        }
    }
    None
}

fn attached_body_stmts(span: syx_diagnostics::Span, template: &str) -> SyxResult<Vec<Stmt>> {
    parse_attached_body(template).or_else(|_| Ok(attached_print_body(span, template)))
}

fn attached_print_body(span: syx_diagnostics::Span, template: &str) -> Vec<Stmt> {
    use syx_ast::{Expr, Literal, Stmt};
    let text = template
        .trim()
        .trim_start_matches("print(")
        .trim_end_matches(')')
        .trim_matches('"')
        .trim_matches('\'')
        .to_string();
    vec![Stmt::Print {
        span,
        expr: Expr::Literal {
            span,
            value: Literal::String(text.into()),
        },
    }]
}

fn attached_member_names(attrs: &[syx_ast::Attribute]) -> Vec<smol_str::SmolStr> {
    attached_kind_names(attrs, "member")
}

fn attached_custom_names(attrs: &[syx_ast::Attribute]) -> Vec<smol_str::SmolStr> {
    attached_kind_names(attrs, "custom")
}

/// Phase 20: `@attached(peer, named, PeerImpl)` peer metadata table entries.
fn attached_peer_names(attrs: &[syx_ast::Attribute]) -> Vec<(smol_str::SmolStr, smol_str::SmolStr)> {
    attached_kind_names(attrs, "peer")
        .into_iter()
        .map(|name| (name, smol_str::SmolStr::new("named")))
        .collect()
}

fn attached_kind_names(attrs: &[syx_ast::Attribute], kind: &str) -> Vec<smol_str::SmolStr> {
    use syx_ast::AttributeArg;
    let mut out = Vec::new();
    for attr in attrs {
        if attr.name.as_str() != "attached" {
            continue;
        }
        let ids: Vec<&str> = attr
            .args
            .iter()
            .filter_map(|a| match a {
                AttributeArg::Ident(id) => Some(id.as_str()),
                _ => None,
            })
            .collect();
        if ids.first() == Some(&kind) {
            if let Some(name) = ids.last() {
                if *name != kind && *name != "named" && *name != "conformances" {
                    out.push(smol_str::SmolStr::new(*name));
                }
            }
        }
    }
    out
}

fn attached_extension_conformances(attrs: &[Attribute]) -> Vec<SmolStr> {
    use syx_ast::AttributeArg;
    let mut out = Vec::new();
    for attr in attrs {
        if attr.name.as_str() != "attached" {
            continue;
        }
        let ids: Vec<&str> = attr
            .args
            .iter()
            .filter_map(|a| match a {
                AttributeArg::Ident(id) => Some(id.as_str()),
                _ => None,
            })
            .collect();
        if ids.first() != Some(&"extension") {
            continue;
        }
        let mut after_conformances = false;
        for arg in &attr.args {
            match arg {
                AttributeArg::Ident(id) if id.as_str() == "conformances" => {
                    after_conformances = true;
                }
                AttributeArg::Ident(id)
                    if after_conformances && id.as_str() != "named" && id.as_str() != "extension" =>
                {
                    out.push(id.clone());
                }
                _ => {}
            }
        }
    }
    out
}

fn attached_body_for_named(
    attrs: &[Attribute],
    kind: &str,
    slot: &SmolStr,
) -> Option<String> {
    use syx_ast::AttributeArg;
    for attr in attrs {
        if attr.name.as_str() != "attached" {
            continue;
        }
        let ids: Vec<&str> = attr
            .args
            .iter()
            .filter_map(|a| match a {
                AttributeArg::Ident(id) => Some(id.as_str()),
                _ => None,
            })
            .collect();
        if ids.first() != Some(&kind) {
            continue;
        }
        if ids.last() != Some(&slot.as_str()) {
            continue;
        }
        for arg in &attr.args {
            if let AttributeArg::String(s) = arg {
                return Some(s.clone());
            }
        }
    }
    None
}

fn expand_attached_deinit(
    attrs: &[Attribute],
    members: &mut Vec<Decl>,
    span: Span,
    default_body: &str,
) -> SyxResult<()> {
    let slots = attached_kind_names(attrs, "deinit");
    if slots.is_empty() {
        return Ok(());
    }
    let body_src = attached_body_from_attr(attrs, "deinit")
        .or_else(|| slots.first().and_then(|s| attached_body_for_named(attrs, "deinit", s)))
        .unwrap_or_else(|| default_body.to_string());
    members.push(Decl::Deinit {
        span,
        access: AccessLevel::Internal,
        body: attached_body_stmts(span, &body_src)?,
    });
    Ok(())
}

fn expand_attached_accessors(
    members: &mut Vec<Decl>,
    span: Span,
    default_body: &str,
) -> SyxResult<()> {
    let mut generated = Vec::new();
    for member in members.iter() {
        let Decl::VarGlobal {
            attrs,
            name: prop,
            ty,
            ..
        } = member
        else {
            continue;
        };
        for acc in attached_kind_names(attrs, "accessor") {
            let body_src = attached_body_for_named(attrs, "accessor", &acc)
                .or_else(|| attached_body_from_attr(attrs, "accessor"))
                .unwrap_or_else(|| {
                    default_body
                        .replace("{name}", prop.as_str())
                        .replace("value", prop.as_str())
                });
            let return_type = if acc.as_str() == "set" {
                None
            } else {
                ty.clone()
            };
            let params = if acc.as_str() == "set" {
                vec![syx_ast::Param {
                    name: smol_str::SmolStr::new("newValue"),
                    external_label: syx_ast::ParamLabel::Same,
                    ty: ty.clone(),
                    default: None,
                    isolated: false,
                    sending: false,
                    attrs: vec![],
                }]
            } else {
                vec![]
            };
            generated.push(Decl::Function {
                span,
                access: AccessLevel::Internal,
                attrs: vec![Attribute {
                    name: smol_str::SmolStr::new("accessor"),
                    args: vec![AttributeArg::Ident(acc.clone())],
                }],
                name: acc,
                type_params: vec![],
                params,
                return_type,
                throws_type: None,
                effect: None,
                where_clause: vec![],
                body: attached_body_stmts(span, &body_src)?,
                is_async: false,
            is_override: false,
            is_final: false,
            });
        }
    }
    members.extend(generated);
    Ok(())
}

fn attached_member_name(attrs: &[syx_ast::Attribute]) -> Option<smol_str::SmolStr> {
    attached_member_names(attrs).into_iter().next()
}

fn has_equatable_attr(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|a| a.name.as_str() == "Equatable")
}

fn has_hashable_attr(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|a| a.name.as_str() == "Hashable")
}

fn has_codable_attr(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|a| a.name.as_str() == "Codable")
}

fn external_type_macro_attr(attrs: &[Attribute]) -> Option<crate::external::ExternalMacroSpec> {
    for attr in attrs {
        if matches!(
            attr.name.as_str(),
            "Equatable" | "Hashable" | "Codable" | "attached"
        ) {
            continue;
        }
        if let Some(spec) = crate::external::lookup_external_macro(&attr.name) {
            return Some(spec);
        }
    }
    None
}

fn synthesize_equatable(
    type_name: &SmolStr,
    fields: &[(SmolStr, syx_ast::TypeExpr)],
    span: Span,
    access: AccessLevel,
) -> Decl {
    use syx_ast::{BinOp, Param, ParamLabel};
    let mut body = Vec::new();
    if fields.is_empty() {
        body.push(Stmt::Return {
            span,
            value: Some(Expr::Literal {
                span,
                value: Literal::Bool(true),
            }),
        });
    } else {
        let first = &fields[0].0;
        body.push(Stmt::VarDecl {
            span,
            mutable: false,
            name: smol_str::SmolStr::new("eq"),
            ty: None,
            init: Some(Expr::Binary {
                span,
                left: Box::new(Expr::Member {
                    span,
                    object: Box::new(Expr::Ident {
                        span,
                        name: smol_str::SmolStr::new("self"),
                    }),
                    name: first.clone(),
                }),
                op: BinOp::Eq,
                right: Box::new(Expr::Member {
                    span,
                    object: Box::new(Expr::Ident {
                        span,
                        name: smol_str::SmolStr::new("other"),
                    }),
                    name: first.clone(),
                }),
            }),
        });
        for (fname, _) in fields.iter().skip(1) {
            body.push(Stmt::Expr {
                span,
                expr: Expr::Binary {
                    span,
                    left: Box::new(Expr::Ident {
                        span,
                        name: smol_str::SmolStr::new("eq"),
                    }),
                    op: BinOp::And,
                    right: Box::new(Expr::Binary {
                        span,
                        left: Box::new(Expr::Member {
                            span,
                            object: Box::new(Expr::Ident {
                                span,
                                name: smol_str::SmolStr::new("self"),
                            }),
                            name: fname.clone(),
                        }),
                        op: BinOp::Eq,
                        right: Box::new(Expr::Member {
                            span,
                            object: Box::new(Expr::Ident {
                                span,
                                name: smol_str::SmolStr::new("other"),
                            }),
                            name: fname.clone(),
                        }),
                    }),
                },
            });
        }
        body.push(Stmt::Return {
            span,
            value: Some(Expr::Ident {
                span,
                name: smol_str::SmolStr::new("eq"),
            }),
        });
    }
    Decl::Function {
        span,
        access,
        attrs: vec![],
        name: smol_str::SmolStr::new("=="),
        type_params: vec![],
        params: vec![Param {
            name: smol_str::SmolStr::new("other"),
            external_label: ParamLabel::Same,
            ty: Some(syx_ast::TypeExpr::Named {
                name: type_name.clone(),
                generics: vec![],
            }),
            default: None,
            isolated: false,
            sending: false,
            attrs: vec![],
        }],
        return_type: Some(syx_ast::TypeExpr::Named {
            name: smol_str::SmolStr::new("Bool"),
            generics: vec![],
        }),
        throws_type: None,
        effect: None,
        where_clause: vec![],
        body,
        is_async: false,
    is_override: false,
            is_final: false,
    }
}

fn synthesize_codable(
    type_name: &SmolStr,
    fields: &[(SmolStr, syx_ast::TypeExpr)],
    _span: Span,
    type_params: &[SmolStr],
    where_clause: &[syx_ast::WherePred],
) -> SyxResult<Vec<Decl>> {
    let mut witness_where = where_clause.to_vec();
    if !type_params.is_empty() && witness_where.is_empty() {
        witness_where = syx_mono::witness_constraint_matrix(type_params, &["Codable"]);
    }
    let _witness_where = &witness_where;
    let field_names: Vec<String> = fields.iter().map(|(n, _)| n.to_string()).collect();
    // Avoid heterogeneous dict literals (`{ "id": Int, "name": String }`) which are S014.
    // Encode the instance directly; `__syx_json_encode` walks object fields.
    let decode_args = field_names
        .iter()
        .map(|f| format!("{f}: obj.{f}"))
        .collect::<Vec<_>>()
        .join(", ");
    let src = "func encode() -> String {
  return __syx_json_encode(self)
}"
    .to_string();
    let mut decls = vec![parse_extension_member(&src)?];
    let decode_src = format!(
        "static func decode(text: String) -> {type_name} {{
  let obj = __syx_json_decode(text)
  return {type_name}({decode_args})
}}"
    );
    decls.push(parse_extension_member(&decode_src)?);
    Ok(decls)
}

fn synthesize_hashable(
    _type_name: &SmolStr,
    fields: &[(SmolStr, syx_ast::TypeExpr)],
    _span: Span,
) -> SyxResult<Vec<Decl>> {
    let combines = fields
        .iter()
        .map(|(fname, _)| format!("__syx_hasher_combine(hasher, self.{fname})"))
        .collect::<Vec<_>>()
        .join("\n");
    let body_src = if combines.is_empty() {
        "__syx_hasher_combine(hasher, self)".to_string()
    } else {
        combines
    };
    let src = format!(
        "func hash(into hasher: Hasher) {{
{body_src}
}}"
    );
    Ok(vec![parse_extension_member(&src)?])
}

fn attached_conformance_protocols(attrs: &[Attribute]) -> Vec<SmolStr> {
    use syx_ast::AttributeArg;
    let mut out = Vec::new();
    for attr in attrs {
        if attr.name.as_str() != "attached" {
            continue;
        }
        let ids: Vec<&str> = attr
            .args
            .iter()
            .filter_map(|a| match a {
                AttributeArg::Ident(id) => Some(id.as_str()),
                _ => None,
            })
            .collect();
        if ids.first() != Some(&"conformance") {
            continue;
        }
        for id in ids.iter().skip(1) {
            if *id != "named" && *id != "conformances" {
                out.push(smol_str::SmolStr::new(*id));
            }
        }
    }
    out
}

fn apply_builtin_macro_attrs(module: &mut syx_ast::Module) {
    use std::collections::HashMap;
    use syx_ast::{Attribute, AttributeArg, Decl, MacroImplementation};

    let mut builtins: HashMap<SmolStr, String> = HashMap::new();
    for decl in &module.decls {
        if let Decl::MacroDecl {
            name,
            implementation: MacroImplementation::BuiltIn { id },
            ..
        } = decl
        {
            builtins.insert(name.clone(), id.clone());
        }
    }
    if builtins.is_empty() {
        return;
    }
    for decl in &mut module.decls {
        let Decl::Class { members, .. } = decl else {
            continue;
        };
        for member in members.iter_mut() {
            let Decl::VarGlobal {
                attrs, name: prop, ..
            } = member
            else {
                continue;
            };
            let mut extra_attrs: Vec<Attribute> = Vec::new();
            for attr in attrs.iter() {
                let Some(id) = builtins.get(&attr.name) else {
                    continue;
                };
                if id != "tracked_accessor" {
                    continue;
                }
                let prop_s = prop.as_str();
                extra_attrs.push(Attribute {
                    name: smol_str::SmolStr::new("attached"),
                    args: vec![
                        AttributeArg::Ident(smol_str::SmolStr::new("accessor")),
                        AttributeArg::Ident(smol_str::SmolStr::new("named")),
                        AttributeArg::Ident(smol_str::SmolStr::new("get")),
                        AttributeArg::String(format!("return self.{prop_s}")),
                    ],
                });
                extra_attrs.push(Attribute {
                    name: smol_str::SmolStr::new("attached"),
                    args: vec![
                        AttributeArg::Ident(smol_str::SmolStr::new("accessor")),
                        AttributeArg::Ident(smol_str::SmolStr::new("named")),
                        AttributeArg::Ident(smol_str::SmolStr::new("set")),
                        AttributeArg::String(format!("self.{prop_s} = newValue")),
                    ],
                });
            }
            attrs.extend(extra_attrs);
        }
    }
}

/// Phase 129 M19: `@Codable` L2 associated-value enum full auto synthesis.
/// Fingerprint: `macro-codable-l2:associated-enum-auto`
fn synthesize_enum_codable(
    type_name: &SmolStr,
    cases: &[(SmolStr, Option<Expr>, Vec<(SmolStr, TypeExpr)>)],
    _span: Span,
) -> SyxResult<Vec<Decl>> {
    fn is_stringish(ty: &TypeExpr) -> bool {
        matches!(
            ty,
            TypeExpr::Named { name, .. } if matches!(name.as_str(), "String" | "str")
        )
    }

    let mut encode_arms = Vec::new();
    for (tag, (case, _, fields)) in cases.iter().enumerate() {
        if fields.is_empty() {
            // Wire JSON via string literal (heterogeneous dict literals are S014).
            encode_arms.push(format!(
                "  if (self.tag == {tag}) {{\n    return \"{{\\\"__v\\\":2,\\\"__tag\\\":\\\"{case}\\\"}}\"\n  }}"
            ));
            continue;
        }
        let mut stmts = Vec::new();
        for (i, (fname, ty)) in fields.iter().enumerate() {
            if !is_stringish(ty) {
                stmts.push(format!(
                    "    let __syx_av{i} = __syx_fmt_format(\"{{0}}\", [self.{fname}])"
                ));
            }
        }
        // {"__v":2,"__tag":"case","f":VAL...}
        let mut concat = String::new();
        for (i, (fname, ty)) in fields.iter().enumerate() {
            if i == 0 {
                if is_stringish(ty) {
                    concat = format!(
                        "\"{{\\\"__v\\\":2,\\\"__tag\\\":\\\"{case}\\\",\\\"{fname}\\\":\\\"\" + self.{fname} + \"\\\"\""
                    );
                } else {
                    concat = format!(
                        "\"{{\\\"__v\\\":2,\\\"__tag\\\":\\\"{case}\\\",\\\"{fname}\\\":\" + __syx_av{i}"
                    );
                }
            } else if is_stringish(ty) {
                concat = format!(
                    "{concat} + \",\\\"{fname}\\\":\\\"\" + self.{fname} + \"\\\"\""
                );
            } else {
                concat = format!("{concat} + \",\\\"{fname}\\\":\" + __syx_av{i}");
            }
        }
        stmts.push(format!("    let __syx_j = {concat} + \"}}\""));
        stmts.push("    return __syx_j".into());
        encode_arms.push(format!(
            "  if (self.tag == {tag}) {{\n{}\n  }}",
            stmts.join("\n")
        ));
    }
    let encode_src = format!(
        "func encode() -> String {{\n{arms}\n  return \"{{\\\"__v\\\":2,\\\"__tag\\\":\\\"unknown\\\"}}\"\n}}",
        arms = encode_arms.join("\n")
    );
    let mut decls = vec![parse_extension_member(&encode_src)?];
    let mut decode_arms = Vec::new();
    for (case, _, fields) in cases {
        if fields.is_empty() {
            decode_arms.push(format!(
                "  if (obj.__tag == \"{case}\") {{ return {type_name}.{case}() }}"
            ));
        } else {
            let args: Vec<String> = fields
                .iter()
                .map(|(f, _)| format!("{f}: obj.{f}"))
                .collect();
            decode_arms.push(format!(
                "  if (obj.__tag == \"{case}\") {{ return {type_name}.{case}({}) }}",
                args.join(", ")
            ));
        }
    }
    let fallback = match cases.first() {
        Some((c, _, fields)) if fields.is_empty() => format!("  return {type_name}.{c}()"),
        Some((c, _, fields)) => {
            let args: Vec<String> = fields
                .iter()
                .map(|(f, ty)| {
                    if is_stringish(ty) {
                        format!("{f}: \"\"")
                    } else {
                        format!("{f}: 0")
                    }
                })
                .collect();
            format!("  return {type_name}.{c}({})", args.join(", "))
        }
        None => format!("  return {type_name}.ok()"),
    };
    let decode_src = format!(
        "static func decode(text: String) -> {type_name} {{
  let obj = __syx_json_decode(text)
{arms}
{fallback}
}}",
        type_name = type_name,
        arms = decode_arms.join("\n"),
        fallback = fallback,
    );
    decls.push(parse_extension_member(&decode_src)?);
    Ok(decls)
}
