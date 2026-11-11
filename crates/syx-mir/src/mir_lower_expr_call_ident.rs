//! SYX MIR `Expr::Call` Ident-callee lowering (bounded Call sub-face).
//! Extracted in S2-I173 (zero semantics; crate-private).

use crate::lower_expr;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_types::{MirInstr, MirOp};
use crate::{
    expr_as_i32_literal, expr_is_string_like, infer_generic_arg_type_name,
    infer_service_resolve_type_name, infer_simd_element_from_array_arg,
    infer_simd_element_from_reduce_arg, simd_reduce_add_shape,
};
use smol_str::SmolStr;
use syx_ast::{CallArg, Expr};
use syx_mono::mangle_name;
use syx_stdlib::{host_fn_id_by_name, host_fn_index, host_fn_index_by_name, HostFnId};

pub(crate) fn lower_expr_call_ident(
    callee: &Expr,
    name: &SmolStr,
    args: &[CallArg],
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
    lower_args_early: bool,
    unbox_targets: Vec<(SmolStr, u32)>,
) {
    // Builtin `Set` is `Value::Set`, not a class instance.
    // Empty `Set()` must lower like `[]` under `: Set<T>` → SetBuild(0).
    if name.as_str() == "Set" && args.is_empty() {
        ctx.emit(
            block,
            MirInstr {
                dest: Some(dest),
                op: MirOp::SetBuild(0),
                span: None,
            },
        );
        return;
    }
    if name == "print" {
        if !lower_args_early {
            for arg in args {
                let r = ctx.fresh_reg();
                lower_expr(&arg.value, r, ctx, block);
            }
        }
        for _ in args {
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Print,
                    span: None,
                },
            );
        }
        return;
    }
    if name == "reflect" {
        if let Some(arg) = args.first() {
            if let Expr::TypeSelf { type_name, .. } = &arg.value {
                ctx.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::ConstStr(type_name.to_string()),
                        span: None,
                    },
                );
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::CallHost(host_fn_index(HostFnId::ReflectType)),
                        span: None,
                    },
                );
                return;
            }
        }
    }
    if let Some(host_id) = host_fn_id_by_name(name) {
        if host_id == HostFnId::SimdShuffle && args.len() == 2 {
            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
            let mode = expr_as_i32_literal(&args[1].value).unwrap_or(0);
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::SimdShuffle { mode },
                    span: None,
                },
            );
            return;
        }
        if host_id == HostFnId::SimdI8Shuffle && args.len() == 2 {
            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
            let mode = expr_as_i32_literal(&args[1].value).unwrap_or(0);
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::SimdI8Shuffle { mode },
                    span: None,
                },
            );
            return;
        }
        if matches!(
            host_id,
            HostFnId::SimdAnd
                | HostFnId::SimdOr
                | HostFnId::SimdXor
                | HostFnId::SimdShl
                | HostFnId::SimdShr
                | HostFnId::SimdMin
                | HostFnId::SimdMax
                | HostFnId::SimdF32Add
                | HostFnId::SimdF32Sub
                | HostFnId::SimdF32Mul
                | HostFnId::SimdF32Div
                | HostFnId::SimdF64Add
                | HostFnId::SimdF64Sub
                | HostFnId::SimdF64Mul
                | HostFnId::SimdF64Div
                | HostFnId::SimdI16Add
                | HostFnId::SimdI16Sub
                | HostFnId::SimdI16Mul
                | HostFnId::SimdI8Add
                | HostFnId::SimdI8Sub
                | HostFnId::SimdI8Mul
                | HostFnId::SimdI8And
                | HostFnId::SimdI8Or
                | HostFnId::SimdI8Xor
                | HostFnId::SimdI8Min
                | HostFnId::SimdI8Max
                | HostFnId::SimdI8UMin
                | HostFnId::SimdI8UMax
                | HostFnId::SimdI8Eq
                | HostFnId::SimdI8Ne
                | HostFnId::SimdI8Le
                | HostFnId::SimdI8Ge
                | HostFnId::SimdI8AvgrU
                | HostFnId::SimdI8Popcnt
                | HostFnId::SimdI8Bitmask
                | HostFnId::SimdI8AllTrue
                | HostFnId::SimdI8AnyTrue
                | HostFnId::SimdI8Lt
                | HostFnId::SimdI8Gt
                | HostFnId::SimdI8AddSat
                | HostFnId::SimdI8SubSat
                | HostFnId::SimdI8Shl
                | HostFnId::SimdI8Shr
                | HostFnId::SimdI64Add
        ) && args.len() == 2
        {
            let v_reg = dest;
            if ctx.next_reg <= dest + 1 {
                ctx.next_reg = dest + 2;
            }
            let a_reg = dest + 1;
            lower_expr(&args[0].value, v_reg, ctx, block);
            if let Some(n) = expr_as_i32_literal(&args[1].value) {
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(a_reg),
                        op: MirOp::ConstI32(n),
                        span: None,
                    },
                );
            } else {
                lower_expr(&args[1].value, a_reg, ctx, block);
            }
            let mir_op = match host_id {
                HostFnId::SimdAnd => MirOp::SimdAnd,
                HostFnId::SimdOr => MirOp::SimdOr,
                HostFnId::SimdXor => MirOp::SimdXor,
                HostFnId::SimdShl => MirOp::SimdShl,
                HostFnId::SimdShr => MirOp::SimdShr,
                HostFnId::SimdMin => MirOp::SimdMin,
                HostFnId::SimdMax => MirOp::SimdMax,
                HostFnId::SimdF32Add => MirOp::SimdF32Add,
                HostFnId::SimdF32Sub => MirOp::SimdF32Sub,
                HostFnId::SimdF32Mul => MirOp::SimdF32Mul,
                HostFnId::SimdF32Div => MirOp::SimdF32Div,
                HostFnId::SimdF64Add => MirOp::SimdF64Add,
                HostFnId::SimdF64Sub => MirOp::SimdF64Sub,
                HostFnId::SimdF64Mul => MirOp::SimdF64Mul,
                HostFnId::SimdF64Div => MirOp::SimdF64Div,
                HostFnId::SimdI16Add => MirOp::SimdI16Add,
                HostFnId::SimdI16Sub => MirOp::SimdI16Sub,
                HostFnId::SimdI16Mul => MirOp::SimdI16Mul,
                HostFnId::SimdI8Add => MirOp::SimdI8Add,
                HostFnId::SimdI8Sub => MirOp::SimdI8Sub,
                HostFnId::SimdI8Mul => MirOp::SimdI8Mul,
                HostFnId::SimdI8And => MirOp::SimdI8And,
                HostFnId::SimdI8Or => MirOp::SimdI8Or,
                HostFnId::SimdI8Xor => MirOp::SimdI8Xor,
                HostFnId::SimdI8Min => MirOp::SimdI8Min,
                HostFnId::SimdI8Max => MirOp::SimdI8Max,
                HostFnId::SimdI8UMin => MirOp::SimdI8UMin,
                HostFnId::SimdI8UMax => MirOp::SimdI8UMax,
                HostFnId::SimdI8Eq => MirOp::SimdI8Eq,
                HostFnId::SimdI8Ne => MirOp::SimdI8Ne,
                HostFnId::SimdI8Le => MirOp::SimdI8Le,
                HostFnId::SimdI8Ge => MirOp::SimdI8Ge,
                HostFnId::SimdI8AvgrU => MirOp::SimdI8AvgrU,
                HostFnId::SimdI8Lt => MirOp::SimdI8Lt,
                HostFnId::SimdI8Gt => MirOp::SimdI8Gt,
                HostFnId::SimdI8AddSat => MirOp::SimdI8AddSat,
                HostFnId::SimdI8SubSat => MirOp::SimdI8SubSat,
                HostFnId::SimdI8Shl => MirOp::SimdI8Shl,
                HostFnId::SimdI8Shr => MirOp::SimdI8Shr,
                HostFnId::SimdI64Add => MirOp::SimdI64Add,
                _ => unreachable!(),
            };
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: mir_op,
                    span: None,
                },
            );
            return;
        }
        if host_id == HostFnId::SimdI8Not && args.len() == 1 {
            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::SimdI8Not,
                    span: None,
                },
            );
            return;
        }
        if host_id == HostFnId::SimdI8Abs && args.len() == 1 {
            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::SimdI8Abs,
                    span: None,
                },
            );
            return;
        }
        if host_id == HostFnId::SimdI8Neg && args.len() == 1 {
            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::SimdI8Neg,
                    span: None,
                },
            );
            return;
        }
        if host_id == HostFnId::SimdI8Popcnt && args.len() == 1 {
            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::SimdI8Popcnt,
                    span: None,
                },
            );
            return;
        }
        if host_id == HostFnId::SimdI8Bitmask && args.len() == 1 {
            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::SimdI8Bitmask,
                    span: None,
                },
            );
            return;
        }
        if host_id == HostFnId::SimdI8AllTrue && args.len() == 1 {
            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::SimdI8AllTrue,
                    span: None,
                },
            );
            return;
        }
        if host_id == HostFnId::SimdI8AnyTrue && args.len() == 1 {
            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::SimdI8AnyTrue,
                    span: None,
                },
            );
            return;
        }
        if host_id == HostFnId::SimdI8Extract && args.len() == 2 {
            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
            if let Some(n) = expr_as_i32_literal(&args[1].value) {
                ctx.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::ConstI32(n),
                        span: None,
                    },
                );
            } else {
                lower_expr(&args[1].value, ctx.fresh_reg(), ctx, block);
            }
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::SimdI8Extract,
                    span: None,
                },
            );
            return;
        }
        if host_id == HostFnId::SimdI8Replace && args.len() == 3 {
            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
            if let Some(n) = expr_as_i32_literal(&args[1].value) {
                ctx.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::ConstI32(n),
                        span: None,
                    },
                );
            } else {
                lower_expr(&args[1].value, ctx.fresh_reg(), ctx, block);
            }
            lower_expr(&args[2].value, ctx.fresh_reg(), ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::SimdI8Replace,
                    span: None,
                },
            );
            return;
        }
        if host_id == HostFnId::SimdNot && args.len() == 1 {
            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::SimdNot,
                    span: None,
                },
            );
            return;
        }
        if host_id == HostFnId::SimdReduceAdd && args.len() == 1 {
            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
            let (lanes, mut element) =
                simd_reduce_add_shape(callee, ctx.func_name.as_str());
            if let Some(inferred) =
                infer_simd_element_from_reduce_arg(&args[0].value, ctx)
            {
                element = inferred;
            }
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::SimdReduceAdd { lanes, element },
                    span: None,
                },
            );
            return;
        }
        if host_id == HostFnId::SimdLoadFromArray && args.len() >= 1 {
            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
            let lanes = args
                .get(2)
                .and_then(|a| expr_as_i32_literal(&a.value))
                .unwrap_or(4) as u8;
            let element = infer_simd_element_from_array_arg(args, ctx);
            let element = if lanes == 2
                && element == syx_stdlib::NumericKind::I32.to_byte()
            {
                // Bare SIMD2 defaults to f64 when element cannot be inferred.
                syx_stdlib::NumericKind::F64.to_byte()
            } else if lanes == 8 {
                syx_stdlib::NumericKind::I16.to_byte()
            } else if lanes == 16 {
                // Phase 129: free-call `__syx_simd_load_from_array(..., 16)` → i8x16.
                syx_stdlib::NumericKind::I8.to_byte()
            } else {
                element
            };
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::SimdLoad { lanes, element },
                    span: None,
                },
            );
            return;
        }
        if host_id == HostFnId::SimdStoreToArray && args.len() >= 3 {
            lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
            lower_expr(&args[2].value, ctx.fresh_reg(), ctx, block);
            let lanes = args
                .get(3)
                .and_then(|a| expr_as_i32_literal(&a.value))
                .map(|n| n as u8)
                .unwrap_or(4);
            let element = if lanes == 2 {
                let inferred = infer_simd_element_from_array_arg(args, ctx);
                if inferred == syx_stdlib::NumericKind::I32.to_byte() {
                    syx_stdlib::NumericKind::F64.to_byte()
                } else {
                    inferred
                }
            } else if lanes == 8 {
                syx_stdlib::NumericKind::I16.to_byte()
            } else if lanes == 16 {
                syx_stdlib::NumericKind::I8.to_byte()
            } else {
                infer_simd_element_from_array_arg(args, ctx)
            };
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::SimdStore { lanes, element },
                    span: None,
                },
            );
            return;
        }
        if host_id == HostFnId::ServiceResolve
            || matches!(
                callee,
                Expr::Ident { name, .. } if name.as_str() == "resolve"
            )
        {
            if let Some(type_name) = infer_service_resolve_type_name(callee, args) {
                ctx.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::ConstStr(type_name.to_string()),
                        span: None,
                    },
                );
                let host = if matches!(
                    callee,
                    Expr::Ident { name, .. } if name.as_str() == "resolve"
                ) {
                    host_fn_index(HostFnId::ServiceResolveDynamic)
                } else {
                    host_fn_index(host_id)
                };
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::CallHost(host),
                        span: None,
                    },
                );
                return;
            }
        }
        for arg in args {
            lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
        }
        ctx.emit(
            block,
            MirInstr {
                dest: Some(dest),
                op: MirOp::CallHost(host_fn_index(host_id)),
                span: None,
            },
        );
        return;
    }
    if let Some(idx) = host_fn_index_by_name(name) {
        ctx.emit(
            block,
            MirInstr {
                dest: Some(dest),
                op: MirOp::CallHost(idx),
                span: None,
            },
        );
        return;
    }
    // R-32: `Int(String)` → CallHost(IntFromString); do not Call(Int_init).
    if name.as_str() == "Int"
        && args.len() == 1
        && expr_is_string_like(&args[0].value, ctx)
    {
        if !lower_args_early {
            let r = ctx.fresh_reg();
            lower_expr(&args[0].value, r, ctx, block);
        }
        ctx.emit(
            block,
            MirInstr {
                dest: Some(dest),
                op: MirOp::CallHost(host_fn_index(HostFnId::IntFromString)),
                span: None,
            },
        );
        return;
    }
    if ctx.module.dataclass_names.contains(name)
        || ctx.module.actor_types.contains(name)
        || ctx.module.class_types.contains_key(name)
    {
        // R-35: Ident `Type(...)` is classified ArgsEarly (non-Member), which
        // skips the empty-args pre-pass — still pad memberwise defaults here.
        let mut fields = ctx
            .module
            .dataclass_fields
            .get(name)
            .cloned()
            .unwrap_or_default();
        if fields.is_empty() {
            for (ty, field) in ctx.module.dataclass_field_default_i64.keys() {
                if ty == name && !fields.iter().any(|f| f == field) {
                    fields.push(field.clone());
                }
            }
            if let Some(m) = ctx.module.class_field_types.get(name) {
                for field in m.keys() {
                    if !fields.iter().any(|f| f == field) {
                        fields.push(field.clone());
                    }
                }
            }
        }
        if !fields.is_empty() {
            // S1-I2 / R-35: stack-materialize defaults (dest: None).
            for field in &fields {
                let arg = args
                    .iter()
                    .find(|a| a.label.as_deref() == Some(field.as_str()))
                    .or_else(|| {
                        let idx = fields.iter().position(|f| f == field)?;
                        args.get(idx).filter(|a| a.label.is_none())
                    });
                if let Some(arg) = arg {
                    if !lower_args_early {
                        let r = ctx.fresh_reg();
                        lower_expr(&arg.value, r, ctx, block);
                    }
                } else if let Some(v) = ctx
                    .module
                    .dataclass_field_default_i64
                    .get(&(name.clone(), field.clone()))
                    .copied()
                {
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::ConstI32(v as i32),
                            span: None,
                        },
                    );
                } else {
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::ConstNil,
                            span: None,
                        },
                    );
                }
            }
        } else if !lower_args_early {
            for arg in args {
                let r = ctx.fresh_reg();
                lower_expr(&arg.value, r, ctx, block);
            }
        }
        let init_fn = SmolStr::new(format!("{name}_init"));
        ctx.emit(
            block,
            MirInstr {
                dest: Some(dest),
                op: MirOp::Call(init_fn),
                span: None,
            },
        );
        return;
    }
    if name.as_str() == "resolve" {
        if let Some(type_name) = infer_service_resolve_type_name(callee, args) {
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstStr(type_name.to_string()),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::CallHost(host_fn_index(HostFnId::ServiceResolveDynamic)),
                    span: None,
                },
            );
            return;
        }
    }
    // S1-I4: `let f = { … }; f(x)` — value call, not named function.
    let not_named_fn = !ctx.module.function_params.contains_key(name)
        && !ctx.module.generic_templates.contains(name)
        && host_fn_id_by_name(name).is_none()
        && host_fn_index_by_name(name).is_none();
    if not_named_fn {
        if let Some(&slot) = ctx.locals.get(name) {
            if !lower_args_early {
                for arg in args {
                    let r = ctx.fresh_reg();
                    lower_expr(&arg.value, r, ctx, block);
                }
            }
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(slot),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::CallClosure,
                    span: None,
                },
            );
            return;
        }
        if let Some(slot) = ctx.global_slot(name) {
            if !lower_args_early {
                for arg in args {
                    let r = ctx.fresh_reg();
                    lower_expr(&arg.value, r, ctx, block);
                }
            }
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadGlobal(slot),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::CallClosure,
                    span: None,
                },
            );
            return;
        }
    }
    let call_name = if ctx.module.generic_templates.contains(name) {
        // Prefer local binding types (ADR-324: Ident ≠ type name).
        let mut type_args: Vec<SmolStr> = args
            .iter()
            .map(|a| infer_generic_arg_type_name(&a.value, ctx))
            .collect();
        // S1-I5: pad to declared arity (`compose<A,B,C>` + 2 closures).
        let arity = ctx
            .module
            .generic_template_arity
            .get(name)
            .copied()
            .unwrap_or(type_args.len().max(1));
        let fill = type_args
            .iter()
            .find(|t| t.as_str() != "Unknown")
            .cloned()
            .unwrap_or_else(|| SmolStr::new("Unknown"));
        while type_args.len() < arity {
            type_args.push(fill.clone());
        }
        if type_args.len() > arity {
            type_args.truncate(arity);
        }
        mangle_name(name.as_str(), &type_args)
    } else {
        name.clone()
    };
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op: MirOp::Call(call_name),
            span: None,
        },
    );
    for (target, cell_slot) in unbox_targets {
        ctx.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::LoadLocal(cell_slot),
                span: None,
            },
        );
        ctx.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ConstStr("__syx_ref".into()),
                span: None,
            },
        );
        ctx.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ObjectGet,
                span: None,
            },
        );
        let resolved = ctx.resolve_binding_name(&target).clone();
        if let Some(&slot) = ctx.locals.get(&resolved) {
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(slot),
                    span: None,
                },
            );
        }
    }
}
