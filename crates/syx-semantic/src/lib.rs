//! SYX semantic analysis crate root (hub + reexports + unit tests).

mod host_fn_capability;
mod import_resolver;
mod type_checker;
mod semantic_types;
mod semantic_type_codec;
mod feature_gates;
mod semantic_helpers;
mod semantic_analyzer;
mod semantic_collect_decls;
mod semantic_analyze_expr;
mod semantic_analyze_stmt;
mod semantic_analyze_decl;
mod semantic_analyze_call;
mod semantic_analyze_type_member;
mod semantic_concurrency;
mod semantic_ide;
mod semantic_infer_host;

use type_checker::is_compatible;

/// Cap on diagnostics retained in collecting mode (LSP / `analyze_collecting`).
/// Prevents pathological files from flooding the editor while still showing many errors at once.
pub const MAX_COLLECTED_DIAGNOSTICS: usize = 64;

pub use semantic_types::{TypeId, Symbol, Scope, ScopeError};
pub use feature_gates::*;
pub use semantic_analyzer::SemanticAnalyzer;
pub use semantic_type_codec::{
    type_name_to_id,
    type_expr_to_id,
    type_expr_to_id_with_aliases,
    type_id_collection_base,
    type_name_dispatch_base,
    normalize_type_arg_name,
    demangle_mono_name,
    user_facing_type_name,
    user_facing_mir_fn_name,
    user_facing_runtime_type_name,
    type_id_to_ir_name,
    type_id_to_name,
    type_id_to_diag_name,
    simd_lanes_from_type_name,
};

pub(crate) use semantic_helpers::{
    attached_conformance_protocols, attr_has_ident_arg, default_assoc_type_for, expr_span, has_attr,
    is_forbidden_internal_import, is_retired_collection_methodcall, module_first_task_detach_span,
    module_uses_task_detach, retired_ops_hint, type_collection_surface, type_expr_uses_param,
};
pub use semantic_helpers::expr_span_start;

#[cfg(test)]
mod tests {
    use super::*;
    use syx_ast::{
        AccessLevel, CallArg, Decl, Expr, Literal, Module, Param, ParamLabel, Stmt, TypeExpr,
    };
    use syx_diagnostics::Span;

    #[test]
    fn analyzes_simple_main() {
        let module = Module {
            name: "main".into(),
            decls: vec![Decl::Function {
                span: Span::DUMMY,
                access: AccessLevel::Internal,
                attrs: vec![],
                name: "main".into(),
                type_params: vec![],
                params: vec![],
                return_type: None,
                throws_type: None,
                effect: None,
                where_clause: vec![],
                body: vec![Stmt::Print {
                    span: Span::DUMMY,
                    expr: Expr::Literal {
                        span: Span::DUMMY,
                        value: Literal::String("hi".into()),
                    },
                }],
                is_async: false,
            is_override: false,
            is_final: false,
            }],
        };
        SemanticAnalyzer::analyze(&module).unwrap();
    }

    #[test]
    fn rejects_undefined_function_call() {
        let module = Module {
            name: "main".into(),
            decls: vec![Decl::Function {
                span: Span::DUMMY,
                access: AccessLevel::Internal,
                attrs: vec![],
                name: "main".into(),
                type_params: vec![],
                params: vec![],
                return_type: None,
                throws_type: None,
                effect: None,
                where_clause: vec![],
                body: vec![Stmt::Expr {
                    span: Span::DUMMY,
                    expr: Expr::Call {
                        span: Span::DUMMY,
                        callee: Box::new(Expr::Ident {
                            span: Span::DUMMY,
                            name: "join".into(),
                        }),
                        args: vec![CallArg {
                            label: None,
                            value: Expr::Literal {
                                span: Span::DUMMY,
                                value: Literal::Nil,
                            },
                        }],
                    },
                }],
                is_async: false,
            is_override: false,
            is_final: false,
            }],
        };
        let err = SemanticAnalyzer::analyze(&module).unwrap_err();
        assert_eq!(err.code.as_deref(), Some("S002"));
        assert!(err.message.contains("Undefined function 'join'"));
    }

    fn module_with_main_call(callee: &str, args: Vec<CallArg>) -> Module {
        Module {
            name: "main".into(),
            decls: vec![
                Decl::Function {
                    span: Span::DUMMY,
                    access: AccessLevel::Internal,
                    attrs: vec![],
                    name: callee.into(),
                    type_params: vec![],
                    params: vec![],
                    return_type: None,
                    throws_type: None,
                    effect: None,
                    where_clause: vec![],
                    body: vec![],
                    is_async: false,
                    is_override: false,
                    is_final: false,
                },
                Decl::Function {
                    span: Span::DUMMY,
                    access: AccessLevel::Internal,
                    attrs: vec![],
                    name: "main".into(),
                    type_params: vec![],
                    params: vec![],
                    return_type: None,
                    throws_type: None,
                    effect: None,
                    where_clause: vec![],
                    body: vec![Stmt::Expr {
                        span: Span::DUMMY,
                        expr: Expr::Call {
                            span: Span::DUMMY,
                            callee: Box::new(Expr::Ident {
                                span: Span::DUMMY,
                                name: callee.into(),
                            }),
                            args,
                        },
                    }],
                    is_async: false,
                    is_override: false,
                    is_final: false,
                },
            ],
        }
    }

    fn labeled_param(name: &str) -> Param {
        Param {
            name: name.into(),
            external_label: ParamLabel::Same,
            ty: Some(TypeExpr::Named {
                name: "Int".into(),
                generics: vec![],
            }),
            default: None,
            isolated: false,
            sending: false,
            attrs: vec![],
        }
    }

    fn omitted_param(name: &str) -> Param {
        Param {
            name: name.into(),
            external_label: ParamLabel::Omitted,
            ty: Some(TypeExpr::Named {
                name: "Int".into(),
                generics: vec![],
            }),
            default: None,
            isolated: false,
            sending: false,
            attrs: vec![],
        }
    }

    fn external_param(ext: &str, internal: &str) -> Param {
        Param {
            name: internal.into(),
            external_label: ParamLabel::Named(ext.into()),
            ty: Some(TypeExpr::Named {
                name: "Int".into(),
                generics: vec![],
            }),
            default: None,
            isolated: false,
            sending: false,
            attrs: vec![],
        }
    }

    fn analyze_call_errors(params: Vec<Param>, args: Vec<CallArg>) -> Vec<String> {
        let mut module = module_with_main_call("f", args);
        module.decls[0] = Decl::Function {
            span: Span::DUMMY,
            access: AccessLevel::Internal,
            attrs: vec![],
            name: "f".into(),
            type_params: vec![],
            params,
            return_type: None,
            throws_type: None,
            effect: None,
            where_clause: vec![],
            body: vec![],
            is_async: false,
            is_override: false,
            is_final: false,
        };
        let analyzer = SemanticAnalyzer::analyze_collecting(&module);
        analyzer
            .errors
            .iter()
            .filter_map(|e| e.code.as_deref().map(String::from))
            .collect()
    }

    #[test]
    fn p139_requires_labels_when_not_all_omitted() {
        let codes = analyze_call_errors(
            vec![labeled_param("x")],
            vec![CallArg {
                label: None,
                value: Expr::Literal {
                    span: Span::DUMMY,
                    value: Literal::Integer {
                        value: 1,
                        suffix: None,
                    },
                },
            }],
        );
        assert!(codes.iter().any(|c| c == "S042"));
    }

    #[test]
    fn p139_underscore_param_is_positional_only() {
        let bad = analyze_call_errors(
            vec![omitted_param("x")],
            vec![CallArg {
                label: Some("x".into()),
                value: Expr::Literal {
                    span: Span::DUMMY,
                    value: Literal::Integer {
                        value: 1,
                        suffix: None,
                    },
                },
            }],
        );
        assert!(bad.iter().any(|c| c == "S042"));
        let ok = analyze_call_errors(
            vec![omitted_param("x")],
            vec![CallArg {
                label: None,
                value: Expr::Literal {
                    span: Span::DUMMY,
                    value: Literal::Integer {
                        value: 1,
                        suffix: None,
                    },
                },
            }],
        );
        assert!(!ok.iter().any(|c| c == "S042"));
    }

    #[test]
    fn p139_external_label_differs_from_internal() {
        let bad = analyze_call_errors(
            vec![external_param("to", "x")],
            vec![CallArg {
                label: Some("x".into()),
                value: Expr::Literal {
                    span: Span::DUMMY,
                    value: Literal::Integer {
                        value: 1,
                        suffix: None,
                    },
                },
            }],
        );
        assert!(bad.iter().any(|c| c == "S042"));
        let ok = analyze_call_errors(
            vec![external_param("to", "x")],
            vec![CallArg {
                label: Some("to".into()),
                value: Expr::Literal {
                    span: Span::DUMMY,
                    value: Literal::Integer {
                        value: 1,
                        suffix: None,
                    },
                },
            }],
        );
        assert!(!ok.iter().any(|c| c == "S042"));
    }
}
