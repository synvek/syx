//! SYX MIR top-level `Expr::Member` lowering.
//! Extracted in S2-I175 (zero semantics; crate-private).

use crate::class_name_from_mangled;
use crate::lower_expr;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_types::{MirInstr, MirOp};
use crate::{
    collection_method_mir_op, expr_is_array_like, expr_is_dict_like, expr_is_set_like,
    expr_is_string_like, extension_call_op, infer_object_type_name, is_hosted_collection_method,
    is_static_type_member_call, method_id_mir_op, type_name_dispatch_base,
};
use smol_str::SmolStr;
use syx_ast::Expr;
use syx_stdlib::{
    lookup_array_method, lookup_dict_method, lookup_set_method, lookup_string_method,
};

pub(crate) fn lower_expr_member(
    object: &Box<Expr>,
    name: &SmolStr,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    if let Expr::Ident {
        name: type_name, ..
    } = object.as_ref()
    {
        if let Some((global_name, _)) = ctx.module.static_var_global(type_name, name) {
            if let Some(slot) = ctx.global_slot(&global_name) {
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::LoadGlobal(slot),
                        span: None,
                    },
                );
                return;
            }
        }
        // Type-level `Type.prop` for `@static __get_prop` (static let / computed get).
        // Do not lower `Type` as a value — that yields Unknown variable.
        if is_static_type_member_call(object, ctx) {
            let get_method = syx_ast::var_global_getter_method(name);
            let getter_fn = SmolStr::new(format!("{type_name}_{get_method}"));
            let has_local_getter = ctx
                .module
                .property_getters
                .contains(&(type_name.clone(), name.clone()));
            let has_static_getter = ctx
                .module
                .static_class_methods
                .contains(&(type_name.clone(), get_method.clone()));
            let has_merged_getter = ctx.module.callable_extensions.contains(&getter_fn);
            if has_local_getter || has_static_getter || has_merged_getter {
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::Call(getter_fn),
                        span: None,
                    },
                );
                return;
            }
        }
    }
    // Class methods keep untyped `self`; recover the owning type from the mangled name
    // so stored fields like `Counter.count` win over `Object.count` / Array.count.
    let member_type_name = infer_object_type_name(object, ctx).or_else(|| {
        matches!(object.as_ref(), Expr::Ident { name, .. } if name.as_str() == "self")
            .then(|| class_name_from_mangled(ctx.func_name.as_str()))
            .flatten()
    });
    if let Some(type_name) = member_type_name.as_ref() {
        if let Some(getter) = ctx.module.property_getter_fn(type_name, name) {
            // Instance computed get needs the receiver; type-level handled above.
            if !is_static_type_member_call(object, ctx) {
                lower_expr(object, ctx.fresh_reg(), ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::Call(getter),
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
        if ctx
            .module
            .extension_methods
            .contains(&(type_name.clone(), name.clone()))
        {
            let ext_fn = SmolStr::new(format!("{type_name}_{name}"));
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
        let nested_ty = SmolStr::new(format!("{type_name}_{name}"));
        if ctx.module.class_types.contains_key(&nested_ty) {
            let init_fn = SmolStr::new(format!("{nested_ty}_init"));
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
        if ctx.module.enum_classes.contains(type_name)
            || ctx.module.final_classes.contains(type_name)
        {
            let init_fn = SmolStr::new(format!("{type_name}_{name}_init"));
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
    if name == "description" {
        if let Some(type_name) = infer_object_type_name(object, ctx) {
            if ctx.module.dataclass_names.contains(&type_name) {
                lower_expr(object, dest, ctx, block);
                let desc_fn = SmolStr::new(format!("{type_name}_describe"));
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::Call(desc_fn),
                        span: None,
                    },
                );
                return;
            }
            let base = type_name_dispatch_base(&type_name);
            if ctx
                .module
                .extension_methods
                .contains(&(base.clone(), SmolStr::new("description")))
            {
                lower_expr(object, ctx.fresh_reg(), ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: extension_call_op(&base, "description", ctx),
                        span: None,
                    },
                );
                return;
            }
            // Phase 46: description requires Core extension for builtins (dataclass unchanged above).
        }
    }
    if name == "description" && expr_is_array_like(object, ctx) {
        lower_expr(object, ctx.fresh_reg(), ctx, block);
        ctx.emit(
            block,
            MirInstr {
                dest: Some(dest),
                op: collection_method_mir_op("Array", "description", ctx),
                span: None,
            },
        );
        return;
    }
    if name == "debugDescription" && expr_is_array_like(object, ctx) {
        lower_expr(object, ctx.fresh_reg(), ctx, block);
        ctx.emit(
            block,
            MirInstr {
                dest: Some(dest),
                op: collection_method_mir_op("Array", "debugDescription", ctx),
                span: None,
            },
        );
        return;
    }
    if name == "debugDescription" {
        if let Some(type_name) = infer_object_type_name(object, ctx) {
            let base = type_name_dispatch_base(&type_name);
            if ctx
                .module
                .extension_methods
                .contains(&(base.clone(), SmolStr::new("debugDescription")))
            {
                lower_expr(object, ctx.fresh_reg(), ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: extension_call_op(&base, "debugDescription", ctx),
                        span: None,
                    },
                );
                return;
            }
        }
    }
    if name == "copy" {
        if let Some(type_name) = infer_object_type_name(object, ctx) {
            if ctx.module.dataclass_names.contains(&type_name) {
                lower_expr(object, dest, ctx, block);
                let copy_fn = SmolStr::new(format!("{type_name}_copy"));
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::Call(copy_fn),
                        span: None,
                    },
                );
                return;
            }
        }
    }
    if expr_is_string_like(object, ctx) {
        if let Some(method) = lookup_string_method(name) {
            lower_expr(object, ctx.fresh_reg(), ctx, block);
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
        if is_hosted_collection_method("Array", name.as_str()) {
            lower_expr(object, ctx.fresh_reg(), ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: collection_method_mir_op("Array", name.as_str(), ctx),
                    span: None,
                },
            );
            return;
        }
        if let Some(method) = lookup_array_method(name) {
            lower_expr(object, ctx.fresh_reg(), ctx, block);
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
    }
    if expr_is_set_like(object, ctx) {
        if is_hosted_collection_method("Set", name.as_str()) {
            lower_expr(object, ctx.fresh_reg(), ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: collection_method_mir_op("Set", name.as_str(), ctx),
                    span: None,
                },
            );
            return;
        }
        if let Some(method) = lookup_set_method(name) {
            lower_expr(object, ctx.fresh_reg(), ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(dest),
                    op: method_id_mir_op(method, "Set", name.as_str(), ctx),
                    span: None,
                },
            );
            return;
        }
    }
    if expr_is_dict_like(object, ctx) {
        let is_object = infer_object_type_name(object, ctx)
            .is_some_and(|t| type_name_dispatch_base(&t).as_str() == "Object");
        let shared_bag_op = lookup_dict_method(name).is_some()
            || is_hosted_collection_method("Dictionary", name.as_str())
            || matches!(name.as_str(), "description" | "debugDescription");
        if !is_object || shared_bag_op {
            if name == "description" || name == "debugDescription" {
                lower_expr(object, ctx.fresh_reg(), ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: collection_method_mir_op("Dictionary", name.as_str(), ctx),
                        span: None,
                    },
                );
                return;
            }
            if is_hosted_collection_method("Dictionary", name.as_str()) {
                lower_expr(object, ctx.fresh_reg(), ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: collection_method_mir_op("Dictionary", name.as_str(), ctx),
                        span: None,
                    },
                );
                return;
            }
            if let Some(method) = lookup_dict_method(name) {
                lower_expr(object, ctx.fresh_reg(), ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: method_id_mir_op(method, "Dictionary", name.as_str(), ctx),
                        span: None,
                    },
                );
                return;
            }
        }
    }
    if let Some(type_name) = infer_object_type_name(object, ctx) {
        let base = type_name_dispatch_base(&type_name);
        if ctx.module.protocol_names.contains(&base) {
            if ctx
                .module
                .protocol_methods
                .get(&base)
                .is_some_and(|ms| ms.iter().any(|m| m == name))
            {
                lower_expr(object, ctx.fresh_reg(), ctx, block);
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(dest),
                        op: MirOp::Call(SmolStr::new(format!("{base}_{name}"))),
                        span: None,
                    },
                );
                return;
            }
        }
    }
    // R-35: do not steal stored fields like `Counter.count` via Array/String
    // method-name fallbacks (those only apply to collection-typed receivers).
    let stored_field_shadow = member_type_name
        .as_ref()
        .is_some_and(|t| ctx.module.has_own_stored_field(t, name));
    if !stored_field_shadow {
        if let Some(method) = lookup_array_method(name) {
            lower_expr(object, ctx.fresh_reg(), ctx, block);
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
        if let Some(method) = lookup_string_method(name) {
            lower_expr(object, ctx.fresh_reg(), ctx, block);
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
