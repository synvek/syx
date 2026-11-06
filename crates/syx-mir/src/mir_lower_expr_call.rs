//! SYX MIR `Expr::Call` shell lowering (ArgsEarly + Ident/Member dispatch + keypath fallback).
//! Extracted in S2-I189 (zero semantics; crate-private).

use crate::classify_member_call;
use crate::host_simd_lowers_own_args;
use crate::keypath_from_callee;
use crate::lower_expr;
use crate::member_host_simd_lowers_own_args;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_lower_expr_call_ident::lower_expr_call_ident;
use crate::mir_lower_expr_call_member::lower_expr_call_member;
use crate::mir_types::{MirInstr, MirOp};
use crate::MemberCallKind;
use smol_str::SmolStr;
use syx_ast::{CallArg, Expr, TypeExpr};
use syx_stdlib::{host_fn_index, HostFnId};

pub(crate) fn lower_expr_call(
    callee: &Box<Expr>,
    args: &[CallArg],
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    let member_kind = classify_member_call(callee, ctx);
    let lower_args_early = member_kind == MemberCallKind::ArgsEarly;
    let mut unbox_targets: Vec<(SmolStr, u32)> = Vec::new();
    if lower_args_early {
        let skip_early = matches!(
            callee.as_ref(),
            Expr::Ident { name, .. } if host_simd_lowers_own_args(name)
        ) || matches!(
            callee.as_ref(),
            Expr::Member { object, name, .. }
                if member_host_simd_lowers_own_args(object, name, ctx)
        );
        if !skip_early {
            let callee_ident = match callee.as_ref() {
                Expr::Ident { name, .. } => Some(name.clone()),
                _ => None,
            };
            let param_types = callee_ident
                .as_ref()
                .and_then(|n| ctx.module.function_params.get(n))
                .cloned()
                .unwrap_or_default();
            for (i, arg) in args.iter().enumerate() {
                let is_mut_param = param_types
                    .get(i)
                    .is_some_and(|t| matches!(t, Some(TypeExpr::Reference(_, true, _))));
                if is_mut_param {
                    if let Expr::Borrow {
                        expr,
                        mutable: true,
                        ..
                    } = &arg.value
                    {
                        if let Expr::Ident { name: target, .. } = expr.as_ref() {
                            let cell_slot =
                                ctx.alloc_local(&SmolStr::new(format!("__syx_cell_{i}")));
                            let r = ctx.fresh_reg();
                            ctx.lower_mut_ref_cell(r, target, block);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: None,
                                    op: MirOp::StoreLocal(cell_slot),
                                    span: None,
                                },
                            );
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: None,
                                    op: MirOp::LoadLocal(cell_slot),
                                    span: None,
                                },
                            );
                            unbox_targets.push((target.clone(), cell_slot));
                            continue;
                        }
                    }
                }
                let r = ctx.fresh_reg();
                lower_expr(&arg.value, r, ctx, block);
            }
        }
    }
    match callee.as_ref() {
        Expr::Ident { name, .. } => {
            lower_expr_call_ident(
                callee.as_ref(),
                name,
                args,
                dest,
                ctx,
                block,
                lower_args_early,
                unbox_targets,
            );
        }
        Expr::Member { object, name, .. } => {
            lower_expr_call_member(
                object,
                name,
                args,
                dest,
                ctx,
                block,
                member_kind,
                lower_args_early,
            );
        }
        _ => {
            if let Some((root_type, property, optional)) = keypath_from_callee(callee) {
                if let Some(arg) = args.first() {
                    lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::ConstStr(root_type.to_string()),
                            span: None,
                        },
                    );
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::ConstStr(property.to_string()),
                            span: None,
                        },
                    );
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::ConstBool(optional),
                            span: None,
                        },
                    );
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: Some(dest),
                            op: MirOp::CallHost(host_fn_index(HostFnId::KeyPathGet)),
                            span: None,
                        },
                    );
                    return;
                }
            }
            lower_expr(callee, dest, ctx, block);
        }
    }
}
