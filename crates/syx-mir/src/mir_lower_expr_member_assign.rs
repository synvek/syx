//! SYX MIR `Expr::MemberAssign` lowering.
//! Extracted in S2-I176 (zero semantics; crate-private).

use crate::expr_is_dict_like;
use crate::infer_object_type_name;
use crate::is_static_type_member_call;
use crate::lower_expr;
use crate::lower_stmt;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_types::{MirInstr, MirOp};
use smol_str::SmolStr;
use syx_ast::Expr;

pub(crate) fn lower_expr_member_assign(
    object: &Box<Expr>,
    name: &SmolStr,
    value: &Box<Expr>,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    if let Expr::Ident {
        name: type_name, ..
    } = object.as_ref()
    {
        if is_static_type_member_call(object, ctx) {
            let set_method = syx_ast::var_global_setter_method(name);
            let setter_fn = SmolStr::new(format!("{type_name}_{set_method}"));
            let has_local = ctx
                .module
                .property_setters
                .contains(&(type_name.clone(), name.clone()));
            let has_static = ctx
                .module
                .static_class_methods
                .contains(&(type_name.clone(), set_method.clone()));
            let has_merged = ctx.module.callable_extensions.contains(&setter_fn);
            if has_local || has_static || has_merged {
                lower_expr(value, ctx.fresh_reg(), ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::Call(setter_fn),
                        span: None,
                    },
                );
                return;
            }
        }
    }
    if let Some(type_name) = infer_object_type_name(object, ctx) {
        if let Some(setter) = ctx.module.property_setter_fn(&type_name, name) {
            if !is_static_type_member_call(object, ctx) {
                lower_expr(object, ctx.fresh_reg(), ctx, block);
                lower_expr(value, ctx.fresh_reg(), ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::Call(setter),
                        span: None,
                    },
                );
                return;
            }
        }
    }
    if let Expr::Ident {
        name: type_name, ..
    } = object.as_ref()
    {
        if let Some((global_name, mutable)) = ctx.module.static_var_global(type_name, name) {
            if mutable {
                if let Some(slot) = ctx.global_slot(&global_name) {
                    let observer = ctx.module.lookup_property_observer(type_name, name);
                    let saved_locals = ctx.locals.clone();
                    if let Some((will_set, did_set)) = observer {
                        let old_slot = ctx.alloc_local(&SmolStr::new("oldValue"));
                        let new_slot = ctx.alloc_local(&SmolStr::new("newValue"));
                        if did_set.is_some() {
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
                                    dest: None,
                                    op: MirOp::StoreLocal(old_slot),
                                    span: None,
                                },
                            );
                        }
                        if let Some(stmts) = will_set {
                            ctx.lower_rhs_for_field_store(value, block);
                            ctx.emit(
                                block,
                                MirInstr {
                                    dest: None,
                                    op: MirOp::StoreLocal(new_slot),
                                    span: None,
                                },
                            );
                            for stmt in stmts {
                                lower_stmt(&stmt, ctx, block);
                            }
                        }
                        lower_expr(value, dest, ctx, block);
                        ctx.emit(
                            block,
                            MirInstr {
                                dest: None,
                                op: MirOp::StoreGlobal(slot),
                                span: None,
                            },
                        );
                        if let Some(stmts) = did_set {
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
                                    dest: None,
                                    op: MirOp::StoreLocal(new_slot),
                                    span: None,
                                },
                            );
                            for stmt in stmts {
                                lower_stmt(&stmt, ctx, block);
                            }
                        }
                        ctx.locals = saved_locals;
                        return;
                    }
                    lower_expr(value, dest, ctx, block);
                    ctx.emit(
                        block,
                        MirInstr {
                            dest: None,
                            op: MirOp::StoreGlobal(slot),
                            span: None,
                        },
                    );
                    return;
                }
            }
        }
    }
    let obj_type = if let Expr::Ident { name: obj_name, .. } = object.as_ref() {
        ctx.receiver_class_type(obj_name)
    } else {
        None
    };
    let obj_reg = ctx.fresh_reg();
    // Keep MutRefCell handles raw so write-back can update `__syx_ref`.
    if let Expr::Ident { name: obj_name, .. } = object.as_ref() {
        ctx.lower_load_binding_raw(obj_name, block);
    } else {
        lower_expr(object, obj_reg, ctx, block);
    }
    let tmp_slot = ctx.alloc_local(&SmolStr::new(format!("__obs_{obj_reg}")));
    let needs_writeback = if let Expr::Ident { name: obj_name, .. } = object.as_ref() {
        ctx.member_assign_unwraps_object(object) && ctx.is_class_instance_binding(obj_name)
    } else {
        false
    };
    let inner_write_slot =
        needs_writeback.then(|| ctx.alloc_local(&SmolStr::new(format!("__syx_inner_{obj_reg}"))));
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::StoreLocal(tmp_slot),
            span: None,
        },
    );
    let observer = obj_type
        .as_ref()
        .and_then(|ty| ctx.module.lookup_property_observer(ty, name));
    let suppress_observers = ctx.in_class_init()
        && matches!(object.as_ref(), Expr::Ident { name: obj_name, .. } if obj_name.as_str() == "self");
    let saved_locals = ctx.locals.clone();
    let run_observers = observer.as_ref().filter(|_| !suppress_observers);
    if let Some((will_set, did_set)) = run_observers {
        let old_slot = ctx.alloc_local(&SmolStr::new("oldValue"));
        let new_slot = ctx.alloc_local(&SmolStr::new("newValue"));
        if did_set.is_some() {
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(tmp_slot),
                    span: None,
                },
            );
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
                    dest: None,
                    op: MirOp::ObjectGet,
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(old_slot),
                    span: None,
                },
            );
        }
        if let Some(stmts) = will_set {
            ctx.lower_rhs_for_field_store(value, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(new_slot),
                    span: None,
                },
            );
            for stmt in stmts {
                lower_stmt(stmt, ctx, block);
            }
        }
        ctx.emit(
            block,
            MirInstr {
                dest: Some(obj_reg),
                op: MirOp::LoadLocal(tmp_slot),
                span: None,
            },
        );
        if !(ctx.in_class_init()
            && matches!(object.as_ref(), Expr::Ident { name: obj_name, .. } if obj_name.as_str() == "self"))
            && !expr_is_dict_like(object, ctx)
        {
            ctx.emit_unwrap_syx_ref(block);
        }
        ctx.lower_rhs_for_field_store(value, block);
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
                dest: None,
                op: MirOp::ObjectSet,
                span: None,
            },
        );
        if let Some(inner_slot) = inner_write_slot {
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(inner_slot),
                    span: None,
                },
            );
        }
        if let Some(stmts) = did_set {
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(tmp_slot),
                    span: None,
                },
            );
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
                    dest: None,
                    op: MirOp::ObjectGet,
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(new_slot),
                    span: None,
                },
            );
            for stmt in stmts {
                lower_stmt(stmt, ctx, block);
            }
        }
    } else {
        ctx.emit(
            block,
            MirInstr {
                dest: Some(obj_reg),
                op: MirOp::LoadLocal(tmp_slot),
                span: None,
            },
        );
        if !(ctx.in_class_init()
            && matches!(object.as_ref(), Expr::Ident { name: obj_name, .. } if obj_name.as_str() == "self"))
            && !expr_is_dict_like(object, ctx)
        {
            ctx.emit_unwrap_syx_ref(block);
        }
        ctx.lower_rhs_for_field_store(value, block);
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
                dest: None,
                op: MirOp::ObjectSet,
                span: None,
            },
        );
        if let Some(inner_slot) = inner_write_slot {
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(inner_slot),
                    span: None,
                },
            );
        }
    }
    ctx.locals = saved_locals;
    if let Expr::Ident { name: obj_name, .. } = object.as_ref() {
        if let Some(inner_slot) = inner_write_slot {
            ctx.emit_mut_ref_cell_writeback_from_inner(obj_name, inner_slot, block);
        } else if ctx.should_unwrap_class_local(obj_name) && !ctx.in_class_init() {
            ctx.lower_store_binding(obj_name, block);
        } else if let Some(&slot) = ctx.locals.get(obj_name) {
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
