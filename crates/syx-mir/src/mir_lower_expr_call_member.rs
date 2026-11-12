//! SYX MIR `Expr::Call` Member-callee lowering (bounded Call sub-face).
//! Extracted in S2-I174 (zero semantics; crate-private).

use crate::lower_expr;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_lower_expr_range::{lower_range_contains_mir, range_bounds_from_expr};
use crate::mir_module_index::remap_platform_int_extension_call;
use crate::mir_types::{MirInstr, MirOp};
use crate::{
    emit_hosted_collection_call, expr_as_i32_literal, expr_is_array_like, expr_is_dict_like,
    expr_is_float32_like, expr_is_float64_like, expr_is_set_like, expr_is_string_like,
    infer_object_type_name, infer_simd_element_from_array_arg, infer_simd_element_from_reduce_arg,
    infer_static_method_type_args, is_hosted_collection_method, is_platform_value_type,
    member_host_simd_lowers_own_args, method_id_mir_op, resolve_host_call,
    simd_member_lanes_element, type_name_dispatch_base, MemberCallKind,
};
use smol_str::SmolStr;
use syx_ast::{CallArg, Expr};
use syx_mono::mangle_name;
use syx_stdlib::{
    host_fn_index, host_fn_index_by_name, lookup_array_method, lookup_dict_method,
    lookup_float32_math_host, lookup_float64_math_host, lookup_string_method, HostFnId,
};

pub(crate) fn lower_expr_call_member(
    object: &Box<Expr>,
    name: &SmolStr,
    args: &[CallArg],
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
    member_kind: MemberCallKind,
    lower_args_early: bool,
) {
    if name == "contains" && args.len() == 1 {
        if let Some((start, end, inclusive)) = range_bounds_from_expr(object) {
            lower_range_contains_mir(start, end, inclusive, &args[0].value, dest, ctx, block);
            return;
        }
    }
    // R-29: any `super.method` → CallSuper (codegen parity); only `init` writes back self.
    if matches!(object.as_ref(), Expr::Super { .. }) {
        if let Some(class) = ctx.enclosing_class() {
            if let Some(super_class) = ctx.module.class_superclasses.get(&class).cloned() {
                ctx.lower_load_binding_raw(&SmolStr::new("self"), block);
                if !lower_args_early {
                    for arg in args {
                        lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
                    }
                }
                let fn_name = SmolStr::new(format!("{super_class}_{name}"));
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::CallSuper(fn_name),
                        span: None,
                    },
                );
                if name == "init" {
                    if let Some(&slot) = ctx.locals.get(&SmolStr::new("self")) {
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
                return;
            }
        }
    }
    if name == "hash" && args.len() == 1 && args[0].label.as_deref() == Some("into") {
        if let Some(type_name) = infer_object_type_name(object, ctx) {
            let hash_into_fn = SmolStr::new(format!("{type_name}_hash_into"));
            let hash_method_fn = SmolStr::new(format!("{type_name}_hash"));
            let call_fn = if ctx.module.function_params.contains_key(&hash_into_fn)
                || ctx.module.dataclass_names.contains(&type_name)
            {
                Some(hash_into_fn)
            } else if ctx.module.function_params.contains_key(&hash_method_fn)
                || ctx
                    .module
                    .class_vtables
                    .get(&type_name)
                    .is_some_and(|vt| vt.contains_key(name))
            {
                Some(hash_method_fn)
            } else {
                None
            };
            if let Some(hash_fn) = call_fn {
                lower_expr(object, ctx.fresh_reg(), ctx, block);
                lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::Call(hash_fn),
                        span: None,
                    },
                );
                if let Expr::Ident {
                    name: hasher_name, ..
                } = &args[0].value
                {
                    if let Some(&slot) = ctx.locals.get(hasher_name) {
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: None,
                                op: MirOp::StoreLocal(slot),
                                span: None,
                            },
                        );
                    } else if let Some(slot) = ctx.global_slot(hasher_name) {
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: None,
                                op: MirOp::StoreGlobal(slot),
                                span: None,
                            },
                        );
                    }
                }
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::ConstNil,
                        span: None,
                    },
                );
                return;
            }
            let base = type_name_dispatch_base(&type_name);
            if ctx
                .module
                .extension_methods
                .contains(&(base.clone(), name.clone()))
            {
                let ext_name = SmolStr::new(format!("{base}_{name}"));
                let is_type_name_call = matches!(
                    object.as_ref(),
                    Expr::Ident { name: recv, .. } if recv == &type_name
                );
                if !is_type_name_call {
                    lower_expr(object, ctx.fresh_reg(), ctx, block);
                }
                for arg in args {
                    lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
                }
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::Call(ext_name),
                        span: None,
                    },
                );
                return;
            }
            // Phase 46: no HasherCombine fallback.
            ctx.set_error_with_code(
                Some("C020"),
                format!(
                    "type '{type_name}' has no hash(into:) implementation; use a Core Hashable extension, data class + @Hashable, or define func hash(into:)"
                ),
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::ConstNil,
                    span: None,
                },
            );
            return;
        }
    }
    if let Some(type_name) = infer_object_type_name(object, ctx) {
        if type_name == "Hasher" {
            if name == "combine" && args.len() == 1 {
                lower_expr(object, ctx.fresh_reg(), ctx, block);
                lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::CallHost(host_fn_index(HostFnId::HasherCombine)),
                        span: None,
                    },
                );
                return;
            }
            if name == "finalize" && args.is_empty() {
                lower_expr(object, ctx.fresh_reg(), ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::CallHost(host_fn_index(HostFnId::HasherFinalize)),
                        span: None,
                    },
                );
                return;
            }
        }
    }
    if (name == "description" || name == "copy") && args.is_empty() {
        if let Some(type_name) = infer_object_type_name(object, ctx) {
            if ctx.module.dataclass_names.contains(&type_name) {
                lower_expr(object, ctx.fresh_reg(), ctx, block);
                let fn_name = if name == "description" {
                    SmolStr::new(format!("{type_name}_describe"))
                } else {
                    SmolStr::new(format!("{type_name}_copy"))
                };
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::Call(fn_name),
                        span: None,
                    },
                );
                return;
            }
        }
    }
    // SA-4-04: `SyxHost.f(...)` façades are thin `__syx_f` wrappers. Enum-name-only
    // inject (no Extension bodies) must not emit `Call SyxHost_f` → C020 on cold prelude.
    if let Expr::Ident {
        name: type_name, ..
    } = object.as_ref()
    {
        if type_name == "SyxHost" {
            let host_name = format!("__syx_{name}");
            if let Some(idx) = host_fn_index_by_name(&host_name) {
                if member_kind != MemberCallKind::ArgsEarly {
                    for arg in args {
                        lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
                    }
                }
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
        }
    }
    if let Some((host_id, receiver_first)) = resolve_host_call(object, name, ctx) {
        let simd_lowers_own = member_host_simd_lowers_own_args(object, name, ctx);
        // ADR-236: HTTPResponse hosts take receiver first (status/text/ok/…).
        if receiver_first {
            lower_expr(object, ctx.fresh_reg(), ctx, block);
        }
        if member_kind != MemberCallKind::ArgsEarly && !simd_lowers_own {
            for arg in args {
                lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
            }
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
            // Phase 127: pin operands at dest/dest+1 for SSA simd binop refill.
            lower_expr(&args[0].value, dest, ctx, block);
            if ctx.next_reg <= dest + 1 {
                ctx.next_reg = dest + 2;
            }
            if let Some(n) = expr_as_i32_literal(&args[1].value) {
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest + 1),
                        op: MirOp::ConstI32(n),
                        span: None,
                    },
                );
            } else {
                lower_expr(&args[1].value, dest + 1, ctx, block);
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
            lower_expr(&args[0].value, dest, ctx, block);
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
            lower_expr(&args[0].value, dest, ctx, block);
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
            lower_expr(&args[0].value, dest, ctx, block);
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
            lower_expr(&args[0].value, dest, ctx, block);
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
            lower_expr(&args[0].value, dest, ctx, block);
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
            lower_expr(&args[0].value, dest, ctx, block);
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
            lower_expr(&args[0].value, dest, ctx, block);
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
            lower_expr(&args[0].value, dest, ctx, block);
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
            lower_expr(&args[0].value, dest, ctx, block);
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
        if host_id == HostFnId::SimdI8Shuffle && args.len() == 2 {
            lower_expr(&args[0].value, dest, ctx, block);
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
        if host_id == HostFnId::SimdReduceAdd && args.len() == 1 {
            lower_expr(&args[0].value, dest, ctx, block);
            let (lanes, mut element) = simd_member_lanes_element(object);
            if let Some(inferred) = infer_simd_element_from_reduce_arg(&args[0].value, ctx) {
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
            if member_kind == MemberCallKind::ArgsEarly {
                lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
            }
            let (lanes, mut element) = simd_member_lanes_element(object);
            let inferred = infer_simd_element_from_array_arg(args, ctx);
            // SIMD2 defaults to F64 only when array element is the I32 fallback.
            if lanes != 2 || inferred != syx_stdlib::NumericKind::I32.to_byte() {
                element = inferred;
            }
            // Phase 117: SIMD8 guest packs as i16x8.
            if lanes == 8 {
                element = syx_stdlib::NumericKind::I16.to_byte();
            }
            if lanes == 16 {
                element = syx_stdlib::NumericKind::I8.to_byte();
            }
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
            if member_kind == MemberCallKind::ArgsEarly {
                lower_expr(&args[0].value, ctx.fresh_reg(), ctx, block);
                lower_expr(&args[2].value, ctx.fresh_reg(), ctx, block);
            }
            let (lanes, mut element) = simd_member_lanes_element(object);
            let inferred = infer_simd_element_from_array_arg(args, ctx);
            if lanes != 2 || inferred != syx_stdlib::NumericKind::I32.to_byte() {
                element = inferred;
            }
            if lanes == 8 {
                element = syx_stdlib::NumericKind::I16.to_byte();
            }
            if lanes == 16 {
                element = syx_stdlib::NumericKind::I8.to_byte();
            }
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
    if let Expr::Member {
        object: inner_obj,
        name: outer_name,
        ..
    } = object.as_ref()
    {
        if let Expr::Ident { name: mod_name, .. } = inner_obj.as_ref() {
            if ctx.imports.contains(mod_name)
                && outer_name.chars().next().is_some_and(|c| c.is_uppercase())
                && name.chars().next().is_some_and(|c| c.is_uppercase())
            {
                let init_fn = SmolStr::new(format!("{mod_name}_{outer_name}_{name}_init"));
                for arg in args {
                    lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
                }
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
        }
    }
    if let Expr::Ident { name: mod_name, .. } = object.as_ref() {
        // User enum cases (e.g. `enum Result { success(...) }`) must win over
        // the builtin Result/BigInt static-method shortcut below.
        if (ctx.module.enum_classes.contains(mod_name)
            || ctx.module.final_classes.contains(mod_name))
            && ctx
                .module
                .enum_case_tags
                .contains_key(&(mod_name.clone(), name.clone()))
        {
            let init_fn = SmolStr::new(format!("{mod_name}_{name}_init"));
            for arg in args {
                lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
            }
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
        if matches!(mod_name.as_str(), "Result" | "BigInt") {
            let qualified = SmolStr::new(format!("{mod_name}_{name}"));
            for arg in args {
                let r = ctx.fresh_reg();
                lower_expr(&arg.value, r, ctx, block);
            }
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::Call(qualified),
                    span: None,
                },
            );
            return;
        }
        if ctx
            .module
            .extension_methods
            .contains(&(mod_name.clone(), name.clone()))
        {
            let ext_fn = SmolStr::new(format!("{mod_name}_{name}"));
            for arg in args {
                let r = ctx.fresh_reg();
                lower_expr(&arg.value, r, ctx, block);
            }
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::Call(ext_fn),
                    span: None,
                },
            );
            return;
        }
        if ctx.imports.contains(mod_name) {
            if name.chars().next().is_some_and(|c| c.is_uppercase()) {
                let init_fn = SmolStr::new(format!("{mod_name}_{name}_init"));
                for arg in args {
                    lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
                }
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
            let qualified = SmolStr::new(format!("{mod_name}_{name}"));
            let type_args = infer_static_method_type_args(args, ctx);
            let call_name = if type_args.iter().all(|t| t.as_str() != "Unknown") {
                mangle_name(qualified.as_str(), &type_args)
            } else {
                qualified
            };
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::Call(call_name),
                    span: None,
                },
            );
            return;
        }
    }
    if expr_is_string_like(object, ctx) {
        if let Some(method) = lookup_string_method(name) {
            lower_expr(object, ctx.fresh_reg(), ctx, block);
            for arg in args {
                let r = ctx.fresh_reg();
                lower_expr(&arg.value, r, ctx, block);
            }
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: method_id_mir_op(method, "String", name.as_str(), ctx),
                    span: None,
                },
            );
            return;
        }
    }
    if expr_is_array_like(object, ctx) {
        emit_hosted_collection_call("Array", name, object, args, dest, ctx, block);
        return;
    }
    if expr_is_set_like(object, ctx) {
        emit_hosted_collection_call("Set", name, object, args, dest, ctx, block);
        return;
    }
    if expr_is_dict_like(object, ctx) {
        // Shared bag ops (count/keys/containsKey/…) use Dictionary MethodId
        // even on typed Object. Object-only APIs (valueForKey/put) fall through
        // to Object_* extensions below.
        let is_object = infer_object_type_name(object, ctx)
            .is_some_and(|t| type_name_dispatch_base(&t).as_str() == "Object");
        let shared_bag_op = lookup_dict_method(name).is_some()
            || is_hosted_collection_method("Dictionary", name.as_str());
        if !is_object || shared_bag_op {
            emit_hosted_collection_call("Dictionary", name, object, args, dest, ctx, block);
            return;
        }
    }
    if let Some(host_id) = lookup_float64_math_host(name) {
        if expr_is_float64_like(object, ctx)
            || infer_object_type_name(object, ctx).is_some_and(|t| {
                matches!(type_name_dispatch_base(&t).as_str(), "Float64" | "Double")
            })
        {
            lower_expr(object, ctx.fresh_reg(), ctx, block);
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
    }
    if let Some(host_id) = lookup_float32_math_host(name) {
        if expr_is_float32_like(object, ctx)
            || infer_object_type_name(object, ctx).is_some_and(|t| {
                matches!(type_name_dispatch_base(&t).as_str(), "Float32" | "Float")
            })
        {
            lower_expr(object, ctx.fresh_reg(), ctx, block);
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
    }
    if let Some(type_name) = infer_object_type_name(object, ctx) {
        let base = type_name_dispatch_base(&type_name);
        let is_type_name_call = matches!(
            object.as_ref(),
            Expr::Ident { name: recv, .. } if recv == &type_name
        );
        if is_type_name_call && name.chars().next().is_some_and(|c| c.is_uppercase()) {
            let nested_ty = SmolStr::new(format!("{base}_{name}"));
            if ctx.module.class_types.contains_key(&nested_ty) {
                for arg in args {
                    lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
                }
                let mut init_fn = SmolStr::new(format!("{nested_ty}_init"));
                if !ctx.module.callable_extensions.contains(&init_fn) {
                    for imp in &ctx.imports {
                        let prefixed = SmolStr::new(format!("{imp}_{nested_ty}_init"));
                        if ctx.module.callable_extensions.contains(&prefixed) {
                            init_fn = prefixed;
                            break;
                        }
                    }
                }
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
        }
        // R-34: protocol-typed receiver → witness ObjectGet + CallIndirect
        // (align AST emit_witness_protocol_call). Never Call("{P}_{m}") —
        // there is no Shape_area body; slots live on the instance.
        if ctx.module.protocol_names.contains(&base)
            && ctx
                .module
                .protocol_methods
                .get(&base)
                .is_some_and(|ms| ms.iter().any(|m| m == name))
        {
            let recv_name = SmolStr::new(format!("__witness_recv_{}", ctx.next_reg));
            let recv_slot = ctx.alloc_local(&recv_name);
            lower_expr(object, ctx.fresh_reg(), ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(recv_slot),
                    span: None,
                },
            );
            let mut arg_slots = Vec::new();
            for (i, arg) in args.iter().enumerate() {
                let an = SmolStr::new(format!("__witness_arg_{}_{i}", ctx.next_reg));
                let slot = ctx.alloc_local(&an);
                lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::StoreLocal(slot),
                        span: None,
                    },
                );
                arg_slots.push(slot);
            }
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(recv_slot),
                    span: None,
                },
            );
            for slot in &arg_slots {
                ctx.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::LoadLocal(*slot),
                        span: None,
                    },
                );
            }
            // Stack: [self, args..., recv]. ObjectGet consumes recv+key →
            // [self, args..., witness]. No Dup — an extra recv would remain
            // under the result and break `sum + s.area()` (R-34).
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(recv_slot),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstStr(format!("__witness_{base}_{name}")),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::ObjectGet,
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::CallIndirect((1 + args.len()) as u32),
                    span: None,
                },
            );
            return;
        }
        let ext_name = SmolStr::new(format!("{base}_{name}"));
        let is_type_name_call = matches!(
            object.as_ref(),
            Expr::Ident { name: recv, .. } if recv == &type_name
        );
        if ctx.module.actor_types.contains(&type_name) {
            lower_expr(object, ctx.fresh_reg(), ctx, block);
            for arg in args {
                lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
            }
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::ActorCall(ext_name),
                    span: None,
                },
            );
            return;
        }
        // S1-I7: `obj.callback(x)` when `callback` is a Closure field.
        if let Some(field_ty) = ctx
            .module
            .class_field_types
            .get(&base)
            .and_then(|m| m.get(name))
        {
            if field_ty.as_str() == "Closure" {
                for arg in args {
                    lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
                }
                lower_expr(object, dest, ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::ConstStr(name.to_string()),
                        span: None,
                    },
                );
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::ObjectGet,
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
        // R-43: class instance method present in class_vtables → ObjectGet(__vtable_*)
        // + CallIndirect (R-34 witness shape). VM already virtualizes Call via
        // resolve_virtual_method; wasm/LLVM need the slot at the call site.
        // Skip type-name statics, init/deinit, and platform value types (DI-safe).
        if !is_type_name_call
            && ctx.module.class_types.contains_key(&base)
            && !is_platform_value_type(base.as_str())
            && name.as_str() != "init"
            && name.as_str() != "deinit"
            && ctx
                .module
                .class_vtables
                .get(&base)
                .is_some_and(|vt| vt.contains_key(name))
        {
            let recv_name = SmolStr::new(format!("__vtable_recv_{}", ctx.next_reg));
            let recv_slot = ctx.alloc_local(&recv_name);
            lower_expr(object, ctx.fresh_reg(), ctx, block);
            if !matches!(object.as_ref(), Expr::Ident { .. }) {
                ctx.emit_unwrap_syx_ref(block);
            }
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(recv_slot),
                    span: None,
                },
            );
            let mut arg_slots = Vec::new();
            for (i, arg) in args.iter().enumerate() {
                let an = SmolStr::new(format!("__vtable_arg_{}_{i}", ctx.next_reg));
                let slot = ctx.alloc_local(&an);
                lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::StoreLocal(slot),
                        span: None,
                    },
                );
                arg_slots.push(slot);
            }
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(recv_slot),
                    span: None,
                },
            );
            for slot in &arg_slots {
                ctx.emit(
                    block,
                    MirInstr {
                        dest: None,
                        op: MirOp::LoadLocal(*slot),
                        span: None,
                    },
                );
            }
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(recv_slot),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstStr(format!("__vtable_{name}")),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::ObjectGet,
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::CallIndirect((1 + args.len()) as u32),
                    span: None,
                },
            );
            return;
        }
        if ctx.module.class_types.contains_key(&base)
            || ctx.module.dataclass_names.contains(&base)
            || ctx.module.enum_classes.contains(&base)
            || ctx
                .module
                .extension_methods
                .contains(&(base.clone(), name.clone()))
        {
            if !is_type_name_call {
                lower_expr(object, ctx.fresh_reg(), ctx, block);
                // Unwrap non-Ident receivers (e.g. `self.logger.tag()`) so
                // virtual dispatch sees Object.__type. Leave Ident cells as
                // MutRefCell so `self.field =` write-back updates the caller's cell.
                // S1-I7: never unwrap platform value types (`3.doubled()`).
                if ctx.module.class_types.contains_key(&base)
                    && !is_platform_value_type(base.as_str())
                    && !matches!(object.as_ref(), Expr::Ident { .. })
                {
                    ctx.emit_unwrap_syx_ref(block);
                }
            }
            for arg in args {
                lower_expr(&arg.value, ctx.fresh_reg(), ctx, block);
            }
            let mut call_name = ext_name.clone();
            if !ctx.module.callable_extensions.contains(&call_name) {
                for imp in &ctx.imports {
                    let prefixed = SmolStr::new(format!("{imp}_{base}_{name}"));
                    if ctx.module.callable_extensions.contains(&prefixed) {
                        call_name = prefixed;
                        break;
                    }
                }
            }
            call_name = remap_platform_int_extension_call(&call_name);
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: MirOp::Call(call_name),
                    span: None,
                },
            );
            return;
        }
    }
    if let Some(method) = lookup_array_method(name) {
        lower_expr(object, ctx.fresh_reg(), ctx, block);
        for arg in args {
            let r = ctx.fresh_reg();
            lower_expr(&arg.value, r, ctx, block);
        }
        ctx.emit(
            block,
            MirInstr {
                dest: Some(dest),
                op: method_id_mir_op(method, "Array", name.as_str(), ctx),
                span: None,
            },
        );
        return;
    }
    lower_expr(object, dest, ctx, block);
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::ConstStr(name.to_string()),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op: MirOp::ObjectGet,
            span: None,
        },
    );
}
