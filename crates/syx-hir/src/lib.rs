//! Typed HIR thin slice (ADR-136 / M1-B01).
//!
//! Vertical slice: Suite-1 style arith / locals / call / string literals.
//! Full language surface is out of scope; backends should prefer MIR (+ type metadata).

use smol_str::SmolStr;
use syx_ast::{BinOp, Decl, Expr, Literal, Module, Stmt};
use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_mir::{encode_module, lower_module, MirModule};
use syx_mono::monomorphize_module;
use syx_semantic::{SemanticAnalyzer, TypeId};
use syx_stdlib::PLATFORM_INT;

fn ty_int() -> TypeId {
    TypeId::Numeric(PLATFORM_INT)
}

fn ty_object() -> TypeId {
    TypeId::Named(SmolStr::new("Object"))
}

/// Typed expression (Suite-1 subset).
#[derive(Debug, Clone)]
pub enum HirExpr {
    Int(i32, TypeId),
    Str(String, TypeId),
    Bool(bool, TypeId),
    Local(SmolStr, TypeId),
    Binary {
        op: BinOp,
        lhs: Box<HirExpr>,
        rhs: Box<HirExpr>,
        ty: TypeId,
    },
    Call {
        name: SmolStr,
        args: Vec<HirExpr>,
        ty: TypeId,
    },
}

/// Typed statement (Suite-1 subset).
#[derive(Debug, Clone)]
pub enum HirStmt {
    Expr(HirExpr),
    Let {
        name: SmolStr,
        value: HirExpr,
    },
    Return(Option<HirExpr>),
}

#[derive(Debug, Clone)]
pub struct HirFunction {
    pub name: SmolStr,
    pub params: Vec<(SmolStr, TypeId)>,
    pub body: Vec<HirStmt>,
    pub ret_ty: TypeId,
}

#[derive(Debug, Clone)]
pub struct HirModule {
    pub functions: Vec<HirFunction>,
    /// Monomorphized AST retained for MIR lowering (transitional B01 carrier).
    mono: Module,
}

impl HirModule {
    pub fn mono(&self) -> &Module {
        &self.mono
    }
}

/// Build HIR from a semantically analyzed module (Suite-1 shapes only).
pub fn from_semantic(module: &Module, semantic: &SemanticAnalyzer) -> SyxResult<HirModule> {
    let mono = monomorphize_module(module, &semantic.instantiations);
    let mut functions = Vec::new();
    for decl in &mono.decls {
        let Decl::Function {
            name,
            params,
            body,
            ..
        } = decl
        else {
            continue;
        };
        let mut hir_params = Vec::new();
        for p in params {
            let ty = p
                .ty
                .as_ref()
                .map(type_expr_fallback)
                .unwrap_or_else(ty_int);
            hir_params.push((p.name.clone(), ty));
        }
        let mut hir_body = Vec::new();
        for stmt in body {
            hir_body.push(stmt_to_hir(stmt)?);
        }
        functions.push(HirFunction {
            name: name.clone(),
            params: hir_params,
            body: hir_body,
            ret_ty: ty_int(),
        });
    }
    if functions.is_empty() {
        return Err(SyxDiagnostic::error(
            "syx-hir Suite-1 slice: no function decls to lower",
        ));
    }
    Ok(HirModule { functions, mono })
}

fn type_expr_fallback(te: &syx_ast::TypeExpr) -> TypeId {
    match te {
        syx_ast::TypeExpr::Named { name, .. } => match name.as_str() {
            "Int" => ty_int(),
            "String" => TypeId::String,
            "Bool" => TypeId::Bool,
            other => TypeId::Named(SmolStr::new(other)),
        },
        _ => ty_object(),
    }
}

fn stmt_to_hir(stmt: &Stmt) -> SyxResult<HirStmt> {
    match stmt {
        Stmt::Expr { expr, .. } => Ok(HirStmt::Expr(expr_to_hir(expr)?)),
        Stmt::Print { expr, .. } => Ok(HirStmt::Expr(HirExpr::Call {
            name: SmolStr::new("print"),
            args: vec![expr_to_hir(expr)?],
            ty: ty_object(),
        })),
        Stmt::VarDecl {
            name,
            init: Some(value),
            ..
        } => Ok(HirStmt::Let {
            name: name.clone(),
            value: expr_to_hir(value)?,
        }),
        Stmt::Return { value, .. } => Ok(HirStmt::Return(
            value.as_ref().map(expr_to_hir).transpose()?,
        )),
        other => Err(SyxDiagnostic::error(format!(
            "syx-hir Suite-1 slice: unsupported stmt {other:?}"
        ))),
    }
}

fn expr_to_hir(expr: &Expr) -> SyxResult<HirExpr> {
    match expr {
        Expr::Literal { value, .. } => match value {
            Literal::Integer { value, .. } => Ok(HirExpr::Int(*value as i32, ty_int())),
            Literal::String(s) => Ok(HirExpr::Str(s.clone(), TypeId::String)),
            Literal::Bool(b) => Ok(HirExpr::Bool(*b, TypeId::Bool)),
            _ => Err(SyxDiagnostic::error(
                "syx-hir Suite-1 slice: unsupported literal",
            )),
        },
        Expr::Ident { name, .. } => Ok(HirExpr::Local(name.clone(), ty_object())),
        Expr::Binary {
            op, left, right, ..
        } => Ok(HirExpr::Binary {
            op: *op,
            lhs: Box::new(expr_to_hir(left)?),
            rhs: Box::new(expr_to_hir(right)?),
            ty: ty_int(),
        }),
        Expr::Call { callee, args, .. } => {
            let name = match callee.as_ref() {
                Expr::Ident { name, .. } => name.clone(),
                _ => {
                    return Err(SyxDiagnostic::error(
                        "syx-hir Suite-1 slice: only simple calls",
                    ))
                }
            };
            let mut hir_args = Vec::new();
            for a in args {
                hir_args.push(expr_to_hir(&a.value)?);
            }
            Ok(HirExpr::Call {
                name,
                args: hir_args,
                ty: ty_object(),
            })
        }
        other => Err(SyxDiagnostic::error(format!(
            "syx-hir Suite-1 slice: unsupported expr {other:?}"
        ))),
    }
}

/// Lower HIR to canonical MIR (reuses `syx-mir::lower_module` on mono AST).
pub fn to_mir(hir: &HirModule) -> SyxResult<MirModule> {
    let mut mir = lower_module(&hir.mono)?;
    syx_mir::optimize(&mut mir);
    Ok(mir)
}

pub fn mir_hash(mir: &MirModule) -> String {
    use sha2::{Digest, Sha256};
    let digest = Sha256::digest(encode_module(mir));
    format!("{digest:x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use syx_diagnostics::SourceFile;
    use syx_frontend::{FrontendOptions, FrontendPipeline, StopAfter};

    #[test]
    fn suite1_arith_hir_mir_matches_pipeline() {
        let src = "func main() { print(10 + 5) }\n";
        let file = SourceFile::new("t.sy", src);
        let mut opts = FrontendOptions {
            stop_after: StopAfter::Mir,
            desugar_top_level: true,
            ..FrontendOptions::default()
        };
        opts.source = Some(file.clone());
        let art = FrontendPipeline::run(file, &opts).expect("pipeline");
        let pipeline_hash = art.mir_hash.expect("hash");
        let semantic = art.semantic.expect("semantic");
        let hir = from_semantic(&art.module, &semantic).expect("hir");
        assert_eq!(hir.functions.len(), 1);
        assert_eq!(hir.functions[0].name.as_str(), "main");
        let mir = to_mir(&hir).expect("mir");
        assert_eq!(mir_hash(&mir), pipeline_hash);
    }

    #[test]
    fn suite1_string_hir_mir_matches_pipeline() {
        let src = "func main() { print(\"a\" + \"b\") }\n";
        let file = SourceFile::new("t.sy", src);
        let mut opts = FrontendOptions {
            stop_after: StopAfter::Mir,
            desugar_top_level: true,
            ..FrontendOptions::default()
        };
        opts.source = Some(file.clone());
        let art = FrontendPipeline::run(file, &opts).expect("pipeline");
        let pipeline_hash = art.mir_hash.expect("hash");
        let semantic = art.semantic.expect("semantic");
        let hir = from_semantic(&art.module, &semantic).expect("hir");
        let mir = to_mir(&hir).expect("mir");
        assert_eq!(mir_hash(&mir), pipeline_hash);
    }
}
