//! ADR-070 M4: JSON protocol for external macro expansion (Phase 52).

use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use syx_diagnostics::SyxResult;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MacroExpandRequest {
    pub api_version: u32,
    pub role: String,
    pub attachment: String,
    #[serde(default)]
    pub type_name: String,
    #[serde(default)]
    pub members: Vec<String>,
    #[serde(default)]
    pub syntax: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MacroExpandResponse {
    #[serde(default)]
    pub stmts: Vec<String>,
    #[serde(default)]
    pub decls: Vec<String>,
    #[serde(default)]
    pub expr: Option<String>,
}

pub fn request_to_json(req: &MacroExpandRequest) -> SyxResult<String> {
    serde_json::to_string(req).map_err(|e| {
        syx_diagnostics::SyxDiagnostic::error(format!("macro expand request encode: {e}"))
            .with_code("E004")
    })
}

pub fn response_from_json(text: &str) -> SyxResult<MacroExpandResponse> {
    serde_json::from_str(text).map_err(|e| {
        syx_diagnostics::SyxDiagnostic::error(format!("macro expand response decode: {e}"))
            .with_code("E004")
    })
}

pub trait ExternalMacroExpander: Send + Sync {
    fn expand(&self, req: &MacroExpandRequest) -> SyxResult<MacroExpandResponse>;
}

pub struct InProcessMacroExpander;

pub struct PluginMacroExpander;

impl ExternalMacroExpander for PluginMacroExpander {
    fn expand(&self, req: &MacroExpandRequest) -> SyxResult<MacroExpandResponse> {
        let json = request_to_json(req)?;
        let resp_json = syx_host_plugin::expand_macro_json(&json).map_err(|e| {
            syx_diagnostics::SyxDiagnostic::error(e).with_code("E004")
        })?;
        response_from_json(&resp_json)
    }
}

impl ExternalMacroExpander for InProcessMacroExpander {
    fn expand(&self, req: &MacroExpandRequest) -> SyxResult<MacroExpandResponse> {
        if req.api_version != 1 {
            return Err(syx_diagnostics::SyxDiagnostic::error(format!(
                "unsupported macro expand api_version {}",
                req.api_version
            ))
            .with_code("E004"));
        }
        match req.role.as_str() {
            "member" | "accessor" | "deinit" | "extension" | "conformance" | "statement"
            | "expression" | "declaration" | "freestanding" | "" => {}
            other => {
                return Err(syx_diagnostics::SyxDiagnostic::error(format!(
                    "unknown macro expand role '{other}'"
                ))
                .with_code("E004"));
            }
        }
        match req.role.as_str() {
            "accessor" => {
                let prop = req.members.first().map(String::as_str).unwrap_or("value");
                return Ok(MacroExpandResponse {
                    stmts: vec![],
                    decls: vec![
                        format!("func get() -> Int {{ return self.{prop} }}"),
                        format!("func set(newValue: Int) {{ self.{prop} = newValue }}"),
                    ],
                    expr: None,
                });
            }
            "deinit" => {
                return Ok(MacroExpandResponse {
                    stmts: vec!["print(\"deinit\")".into()],
                    decls: vec![],
                    expr: None,
                });
            }
            "extension" => {
                let member = req.members.first().map(String::as_str).unwrap_or("label");
                return Ok(MacroExpandResponse {
                    stmts: vec![],
                    decls: vec![format!(
                        "func {member}() -> String {{ return \"{member}:\" + self.description() }}"
                    )],
                    expr: None,
                });
            }
            _ => {}
        }
        match req.attachment.as_str() {
            "SmokeMemberMacro" => {
                let member = req.members.first().map(String::as_str).unwrap_or("externalMember");
                if req.role == "statement" || req.role == "freestanding" {
                    Ok(MacroExpandResponse {
                        stmts: vec![format!(
                            "print(\"external:{}\")",
                            req.type_name
                        )],
                        decls: vec![],
                        expr: None,
                    })
                } else {
                    Ok(MacroExpandResponse {
                        stmts: vec![format!(
                            "func {member}() {{ print(\"external:{}\") }}",
                            req.type_name
                        )],
                        decls: vec![],
                        expr: None,
                    })
                }
            }
            "CodableMacro" => {
                let fields: Vec<&str> = req.members.iter().map(String::as_str).collect();
                let decode_args = fields
                    .iter()
                    .map(|f| format!("{f}: obj.{f}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                Ok(MacroExpandResponse {
                    stmts: vec![],
                    decls: vec![
                        // Avoid heterogeneous dict literals (S014); encode the instance.
                        "func encode() -> String {\n  return __syx_json_encode(self)\n}"
                            .to_string(),
                        format!(
                            "static func decode(text: String) -> {} {{\n  let obj = __syx_json_decode(text)\n  return {}({decode_args})\n}}",
                            req.type_name, req.type_name
                        ),
                    ],
                    expr: None,
                })
            }
            // Phase 119 M9: production JSON-protocol external type (cdylib / in-process).
            "JsonPluginMacro" | "ProdMemberMacro" => {
                let member = req.members.first().map(String::as_str).unwrap_or("pluginOk");
                Ok(MacroExpandResponse {
                    stmts: vec![],
                    decls: vec![format!(
                        "func {member}() -> String {{ return \"plugin-prod:\" + \"{}\" }}",
                        req.type_name
                    )],
                    expr: None,
                })
            }
            other => Err(syx_diagnostics::SyxDiagnostic::error(format!(
                "unknown external macro type '{other}'"
            ))
            .with_code("E004")),
        }
    }
}

pub fn expand_via_protocol(
    expander: &dyn ExternalMacroExpander,
    req: &MacroExpandRequest,
) -> SyxResult<Vec<syx_ast::Stmt>> {
    use crate::parse_fragment::{parse_attached_body, parse_attached_decls};
    let resp = expander.expand(req)?;
    let mut out = Vec::new();
    for src in &resp.stmts {
        out.extend(parse_attached_body(src)?);
    }
    for decl in parse_attached_decls(&resp.decls.join("\n"))? {
        if let syx_ast::Decl::Function { body, .. } = decl {
            out.extend(body);
        }
    }
    Ok(out)
}

pub fn expand_decls_via_protocol(
    expander: &dyn ExternalMacroExpander,
    req: &MacroExpandRequest,
) -> SyxResult<Vec<syx_ast::Decl>> {
    
    let resp = expander.expand(req)?;
    if !resp.decls.is_empty() {
        let mut out = Vec::new();
        for src in &resp.decls {
            out.push(crate::parse_fragment::parse_extension_member(src)?);
        }
        return Ok(out);
    }
    if !resp.stmts.is_empty() {
        let mut decls = Vec::new();
        for src in &resp.stmts {
            let body = crate::parse_fragment::parse_attached_body(src)?;
            decls.push(syx_ast::Decl::Function {
                span: syx_diagnostics::Span::DUMMY,
                access: syx_ast::AccessLevel::Internal,
                attrs: vec![],
                name: SmolStr::new("externalMember"),
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
        return Ok(decls);
    }
    Ok(vec![])
}
