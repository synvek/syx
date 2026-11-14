//! SYX MIR statement lowering.
//! Extracted in S2-I203 (zero semantics; crate-private).

use crate::binop_operator_symbol;
use crate::emit_integer_narrow_cast_if_needed;
use crate::extension_or_hosted_return_type;
use crate::infer_expr_init_type;
use crate::infer_for_in_elem_type;
use crate::infer_object_type_name;
use crate::infer_service_resolve_type_name;
use crate::lower_expr;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_lower_switch_pattern::{bind_pattern, lower_switch_pattern_match};
use crate::mir_module_index::mir_type_expr_name;
use crate::mir_types::{MirInstr, MirOp};
use crate::type_name_is_set;
use smol_str::SmolStr;
use syx_ast::{Expr, Literal, Stmt};

pub(crate) fn lower_stmt(stmt: &Stmt, ctx: &mut LowerCtx, block: u32) {
    let prev_span = ctx.current_span;
    ctx.current_span = Some(stmt_span_tuple(stmt));
    lower_stmt_inner(stmt, ctx, block);
    ctx.current_span = prev_span;
}

pub(crate) fn stmt_span_tuple(stmt: &Stmt) -> (u32, u32) {
    let span = match stmt {
        Stmt::Print { span, .. }
        | Stmt::Expr { span, .. }
        | Stmt::VarDecl { span, .. }
        | Stmt::Return { span, .. }
        | Stmt::If { span, .. }
        | Stmt::While { span, .. }
        | Stmt::ForIn { span, .. }
        | Stmt::Block { span, .. }
        | Stmt::Seq { span, .. }
        | Stmt::Break { span, .. }
        | Stmt::Continue { span, .. }
        | Stmt::Try { span, .. }
        | Stmt::Throw { span, .. }
        | Stmt::Switch { span, .. }
        | Stmt::IfCase { span, .. }
        | Stmt::IfLet { span, .. }
        | Stmt::GuardLet { span, .. }
        | Stmt::RepeatWhile { span, .. }
        | Stmt::MacroInvoke { span, .. } => *span,
    };
    (span.start, span.len)
}

pub(crate) fn lower_stmt_inner(stmt: &Stmt, ctx: &mut LowerCtx, block: u32) {
    match stmt {
        Stmt::Return { value: Some(v), .. } => {
            let dest = ctx.fresh_reg();
            if let Expr::Ident { name, .. } = v {
                ctx.skip_deinit_on_return = Some(name.clone());
            }
            lower_expr(v, dest, ctx, block);
            // W1058: do not emit scope-exit deinits on `return expr` — Call+Pop
            // would drop the result and leave ctor leftovers (rows/cols Int).
            // Params stay caller-owned; returned Ident is not dropped here.
            ctx.skip_deinit_on_return = None;
            // After exprs that split blocks (e.g. `??`), continue from active_block.
            let cur = ctx.active_block;
            ctx.emit(
                cur,
                MirInstr {
                    dest: None,
                    op: MirOp::Return,
                    span: None,
                },
            );
        }
        Stmt::Return { value: None, .. } => {
            let nil_reg = ctx.fresh_reg();
            ctx.emit(
                block,
                MirInstr {
                    dest: Some(nil_reg),
                    op: MirOp::ConstNil,
                    span: None,
                },
            );
            ctx.emit_scope_exit_deinits(block);
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Return,
                    span: None,
                },
            );
        }
        Stmt::Print { expr, .. } => {
            let dest = ctx.fresh_reg();
            lower_expr(expr, dest, ctx, block);
            let cur = ctx.active_block;
            ctx.emit(
                cur,
                MirInstr {
                    dest: None,
                    op: MirOp::Print,
                    span: None,
                },
            );
        }
        Stmt::Expr { expr, .. } => {
            let dest = ctx.fresh_reg();
            lower_expr(expr, dest, ctx, block);
            if !matches!(
                expr,
                Expr::Assign { .. } | Expr::MemberAssign { .. } | Expr::IndexAssign { .. }
            ) {
                let cur = ctx.active_block;
                ctx.emit(
                    cur,
                    MirInstr {
                        dest: None,
                        op: MirOp::Pop,
                        span: None,
                    },
                );
            }
        }
        Stmt::VarDecl { name, init, ty, .. } => {
            let slot = ctx.alloc_local(name);
            if let Some(type_expr) = ty {
                if let Some(type_name) = mir_type_expr_name(type_expr) {
                    if type_name_is_set(type_name.as_str()) {
                        ctx.prefer_set_build = true;
                    }
                    ctx.local_types.insert(name.clone(), type_name);
                }
            }
            if let Some(expr) = init {
                if let Expr::Borrow {
                    expr: inner,
                    mutable: true,
                    ..
                } = expr
                {
                    if let Expr::Ident { name: target, .. } = inner.as_ref() {
                        ctx.mut_ref_aliases.insert(name.clone(), target.clone());
                        return;
                    }
                }
                let dest = ctx.fresh_reg();
                lower_expr(expr, dest, ctx, block);
                if let Expr::Literal {
                    value: Literal::String(_),
                    ..
                } = expr
                {
                    ctx.local_types.insert(name.clone(), SmolStr::new("String"));
                }
                if let Expr::Literal {
                    value: Literal::Double { suffix, .. },
                    ..
                } = expr
                {
                    let s = suffix.as_deref().unwrap_or("");
                    let ty = if s == "f32" || s == "f" {
                        "Float32"
                    } else {
                        "Float64"
                    };
                    ctx.local_types.insert(name.clone(), SmolStr::new(ty));
                }
                if let Expr::Call { callee, args, .. } = expr {
                    if let Some(type_name) = infer_service_resolve_type_name(callee, args) {
                        ctx.local_types.insert(name.clone(), type_name);
                    }
                    if let Expr::Member {
                        object,
                        name: method,
                        ..
                    } = callee.as_ref()
                    {
                        let recv_ty = match object.as_ref() {
                            Expr::Ident {
                                name: type_name, ..
                            } => {
                                if ctx.module.class_types.contains_key(type_name)
                                    || ctx.module.enum_classes.contains(type_name)
                                    || ctx.module.dataclass_names.contains(type_name)
                                    || matches!(type_name.as_str(), "BigInt" | "Result")
                                {
                                    Some(type_name.clone())
                                } else {
                                    ctx.local_types.get(type_name).cloned()
                                }
                            }
                            _ => infer_object_type_name(object, ctx),
                        };
                        if let Some(type_name) = recv_ty {
                            if matches!(type_name.as_str(), "BigInt" | "Result") {
                                ctx.local_types.insert(name.clone(), type_name.clone());
                            }
                            if ctx.module.enum_classes.contains(&type_name) {
                                ctx.local_types.insert(name.clone(), type_name.clone());
                            }
                            if let Some(ret) =
                                extension_or_hosted_return_type(&type_name, method, ctx)
                            {
                                ctx.local_types.insert(name.clone(), ret);
                            }
                        }
                    }
                    if let Expr::Ident {
                        name: type_name, ..
                    } = callee.as_ref()
                    {
                        if ctx.module.dataclass_names.contains(type_name)
                            || ctx.module.actor_types.contains(type_name)
                            || ctx.module.class_types.contains_key(type_name)
                        {
                            ctx.local_types.insert(name.clone(), type_name.clone());
                        }
                    }
                }
                if let Expr::Binary { left, op, .. } = expr {
                    if let Some(sym) = binop_operator_symbol(*op) {
                        if let Some(left_ty) = infer_object_type_name(left, ctx) {
                            if let Some(ret) = ctx
                                .module
                                .extension_method_returns
                                .get(&(left_ty.clone(), SmolStr::new(sym)))
                            {
                                ctx.local_types.insert(name.clone(), ret.clone());
                            }
                        }
                    }
                }
                if !ctx.local_types.contains_key(name) {
                    if let Some(type_name) = infer_expr_init_type(expr, ctx) {
                        ctx.local_types.insert(name.clone(), type_name);
                    }
                }
                if let Some(to_name) = ty.as_ref().and_then(|t| mir_type_expr_name(t)) {
                    if let Some(from_name) = infer_expr_init_type(expr, ctx) {
                        let cur = ctx.active_block;
                        emit_integer_narrow_cast_if_needed(
                            from_name.as_str(),
                            to_name.as_str(),
                            dest,
                            ctx,
                            cur,
                        );
                    }
                }
            } else {
                let nil_reg = ctx.fresh_reg();
                ctx.emit(
                    block,
                    MirInstr {
                        dest: Some(nil_reg),
                        op: MirOp::ConstNil,
                        span: None,
                    },
                );
            }
            ctx.prefer_set_build = false;
            // `??` / other block-splitting inits leave the value in `active_block`.
            let cur = ctx.active_block;
            // R-31: `var q = p` for struct/dataclass must copy, not alias.
            // Skip copy when RHS is a fresh `{T}()` / `{T}_init` call — already unique,
            // and Call-dest register may not leave a stack value for `{T}_copy`.
            let rhs_is_fresh_init = matches!(
                init,
                Some(Expr::Call {
                    callee,
                    ..
                }) if matches!(
                    callee.as_ref(),
                    Expr::Ident { name: ty_name, .. }
                        if ctx.module.dataclass_names.contains(ty_name)
                            || ctx.module.class_types.contains_key(ty_name)
                            || ctx.module.actor_types.contains(ty_name)
                )
            );
            if !rhs_is_fresh_init {
                if let Some(ty) = ctx.local_types.get(name).cloned() {
                    ctx.emit_dataclass_assign_copy_if_needed(&ty, cur);
                }
            }
            ctx.emit(
                cur,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(slot),
                    span: None,
                },
            );
        }
        Stmt::IfLet {
            name,
            pattern,
            body,
            else_branch,
            ..
        } => {
            let dest = ctx.fresh_reg();
            lower_expr(pattern, dest, ctx, block);
            let cur = ctx.active_block;
            let end_block = ctx.fresh_block();
            let else_block = if else_branch.is_some() {
                ctx.fresh_block()
            } else {
                end_block
            };
            ctx.emit(
                cur,
                MirInstr {
                    dest: None,
                    op: MirOp::JumpIfNil(else_block),
                    span: None,
                },
            );
            let slot = ctx.alloc_local(name);
            ctx.emit(
                cur,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(slot),
                    span: None,
                },
            );
            ctx.active_block = cur;
            for s in body {
                let b = ctx.active_block;
                lower_stmt(s, ctx, b);
            }
            let then_end = ctx.active_block;
            ctx.emit(
                then_end,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(end_block),
                    span: None,
                },
            );
            ctx.link(then_end, end_block);
            if let Some(else_b) = else_branch {
                ctx.link(cur, else_block);
                ctx.active_block = else_block;
                for s in else_b {
                    let b = ctx.active_block;
                    lower_stmt(s, ctx, b);
                }
                let else_end = ctx.active_block;
                ctx.emit(
                    else_end,
                    MirInstr {
                        dest: None,
                        op: MirOp::Jump(end_block),
                        span: None,
                    },
                );
                ctx.link(else_end, end_block);
            } else {
                ctx.link(cur, end_block);
            }
            ctx.active_block = end_block;
        }
        Stmt::GuardLet {
            name,
            pattern,
            body,
            else_branch,
            ..
        } => {
            let dest = ctx.fresh_reg();
            lower_expr(pattern, dest, ctx, block);
            let cur = ctx.active_block;
            let end_block = ctx.fresh_block();
            let else_block = ctx.fresh_block();
            ctx.emit(
                cur,
                MirInstr {
                    dest: None,
                    op: MirOp::JumpIfNil(else_block),
                    span: None,
                },
            );
            let slot = ctx.alloc_local(name);
            ctx.emit(
                cur,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(slot),
                    span: None,
                },
            );
            ctx.active_block = cur;
            for s in body {
                let b = ctx.active_block;
                lower_stmt(s, ctx, b);
            }
            let then_end = ctx.active_block;
            ctx.emit(
                then_end,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(end_block),
                    span: None,
                },
            );
            ctx.link(then_end, end_block);
            ctx.link(cur, else_block);
            ctx.active_block = else_block;
            for s in else_branch {
                let b = ctx.active_block;
                lower_stmt(s, ctx, b);
            }
            let else_end = ctx.active_block;
            ctx.emit(
                else_end,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(end_block),
                    span: None,
                },
            );
            ctx.link(else_end, end_block);
            ctx.active_block = end_block;
        }
        Stmt::If {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            let cond_reg = ctx.fresh_reg();
            lower_expr(condition, cond_reg, ctx, block);
            // Condition may split blocks (e.g. nested `??`); emit branches from the
            // live block after lowering, not the stale entry `block`.
            let cond_block = ctx.active_block;
            let then_block = ctx.fresh_block();
            let merge_block = ctx.fresh_block();
            // Always emit an explicit Jump(then) after JumpIfFalse. While/ForIn
            // pre-allocate an exit block between the body and later then/merge
            // ids; without this jump, a true condition falls through into exit
            // and drops trailing body stmts (e.g. `j = j + 1`).
            let else_block = if else_branch.is_some() {
                let eb = ctx.fresh_block();
                ctx.emit(
                    cond_block,
                    MirInstr {
                        dest: None,
                        op: MirOp::JumpIfFalse(eb),
                        span: None,
                    },
                );
                ctx.emit(
                    cond_block,
                    MirInstr {
                        dest: None,
                        op: MirOp::Jump(then_block),
                        span: None,
                    },
                );
                ctx.link(cond_block, then_block);
                ctx.link(cond_block, eb);
                Some(eb)
            } else {
                ctx.emit(
                    cond_block,
                    MirInstr {
                        dest: None,
                        op: MirOp::JumpIfFalse(merge_block),
                        span: None,
                    },
                );
                ctx.emit(
                    cond_block,
                    MirInstr {
                        dest: None,
                        op: MirOp::Jump(then_block),
                        span: None,
                    },
                );
                ctx.link(cond_block, then_block);
                ctx.link(cond_block, merge_block);
                None
            };
            // Lower then/else from each branch entry; after nested control flow,
            // terminate from ctx.active_block (not the branch entry block).
            ctx.active_block = then_block;
            for s in then_branch {
                let b = ctx.active_block;
                lower_stmt(s, ctx, b);
            }
            let then_end = ctx.active_block;
            ctx.emit(
                then_end,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(merge_block),
                    span: None,
                },
            );
            ctx.link(then_end, merge_block);
            if let (Some(else_b), Some(eb)) = (else_branch, else_block) {
                ctx.active_block = eb;
                for s in else_b {
                    let b = ctx.active_block;
                    lower_stmt(s, ctx, b);
                }
                let else_end = ctx.active_block;
                ctx.emit(
                    else_end,
                    MirInstr {
                        dest: None,
                        op: MirOp::Jump(merge_block),
                        span: None,
                    },
                );
                ctx.link(else_end, merge_block);
            }
            ctx.active_block = merge_block;
        }
        Stmt::While {
            condition,
            body,
            label,
            ..
        } => {
            let loop_header = ctx.fresh_block();
            let body_block = ctx.fresh_block();
            let exit_block = ctx.fresh_block();
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(loop_header),
                    span: None,
                },
            );
            ctx.link(block, loop_header);
            let cond_reg = ctx.fresh_reg();
            lower_expr(condition, cond_reg, ctx, loop_header);
            ctx.emit(
                loop_header,
                MirInstr {
                    dest: None,
                    op: MirOp::JumpIfFalse(exit_block),
                    span: None,
                },
            );
            ctx.link(loop_header, body_block);
            ctx.link(loop_header, exit_block);
            // Same as If: body stmts must see active_block == body entry.
            // Otherwise VarDecl/Print (which store via active_block after ?? fix)
            // emit into the pre-loop block after Jump — dead code, loop locals stay nil.
            // U-7: continue → header, break → exit.
            ctx.loop_stack
                .push((loop_header, exit_block, label.clone()));
            ctx.active_block = body_block;
            for s in body {
                let b = ctx.active_block;
                lower_stmt(s, ctx, b);
            }
            let body_end = ctx.active_block;
            ctx.loop_stack.pop();
            ctx.emit(
                body_end,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(loop_header),
                    span: None,
                },
            );
            ctx.link(body_end, loop_header);
            ctx.active_block = exit_block;
        }
        Stmt::ForIn {
            name,
            iterable,
            body,
            label,
            ..
        } => {
            let dest = ctx.fresh_reg();
            lower_expr(iterable, dest, ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::MakeIterator,
                    span: None,
                },
            );
            let iter_slot = ctx.alloc_local(&SmolStr::new(format!("__iter_{name}")));
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(iter_slot),
                    span: None,
                },
            );
            let loop_header = ctx.fresh_block();
            let body_block = ctx.fresh_block();
            let exit_block = ctx.fresh_block();
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(loop_header),
                    span: None,
                },
            );
            ctx.link(block, loop_header);
            ctx.emit(
                loop_header,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(iter_slot),
                    span: None,
                },
            );
            ctx.emit(
                loop_header,
                MirInstr {
                    dest: None,
                    op: MirOp::IteratorNext,
                    span: None,
                },
            );
            ctx.emit(
                loop_header,
                MirInstr {
                    dest: None,
                    op: MirOp::JumpIfFalse(exit_block),
                    span: None,
                },
            );
            ctx.link(loop_header, body_block);
            ctx.link(loop_header, exit_block);
            let elem_slot = ctx.alloc_local(name);
            // R-34: bind for-in element type from Array/Set<T> so protocol
            // receivers (e.g. Shape) hit witness CallIndirect, not ObjectGet.
            if let Some(elem_ty) = infer_for_in_elem_type(iterable, ctx) {
                ctx.local_types.insert(name.clone(), elem_ty);
            }
            ctx.emit(
                body_block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(elem_slot),
                    span: None,
                },
            );
            ctx.emit(
                body_block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(iter_slot),
                    span: None,
                },
            );
            // U-7: continue → header (next), break → exit.
            ctx.loop_stack
                .push((loop_header, exit_block, label.clone()));
            ctx.active_block = body_block;
            for s in body {
                let b = ctx.active_block;
                lower_stmt(s, ctx, b);
            }
            let body_end = ctx.active_block;
            ctx.loop_stack.pop();
            ctx.emit(
                body_end,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(loop_header),
                    span: None,
                },
            );
            ctx.link(body_end, loop_header);
            ctx.active_block = exit_block;
        }
        Stmt::Block { stmts, .. } => {
            ctx.active_block = block;
            for s in stmts {
                let b = ctx.active_block;
                lower_stmt(s, ctx, b);
            }
        }
        Stmt::Seq { stmts, .. } => {
            ctx.active_block = block;
            for s in stmts {
                let b = ctx.active_block;
                lower_stmt(s, ctx, b);
            }
        }
        Stmt::Try {
            body,
            catch_name,
            catch_body,
            ..
        } => {
            let catch_block = ctx.fresh_block();
            let after_block = ctx.fresh_block();
            // SA-2-01: keep active_block aligned — Print/Expr emit to active_block, not only `block`.
            ctx.active_block = block;
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::BeginTry(catch_block),
                    span: None,
                },
            );
            for s in body {
                lower_stmt(s, ctx, ctx.active_block);
            }
            let body_end = ctx.active_block;
            ctx.emit(
                body_end,
                MirInstr {
                    dest: None,
                    op: MirOp::EndTry,
                    span: None,
                },
            );
            ctx.emit(
                body_end,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(after_block),
                    span: None,
                },
            );
            ctx.link(body_end, catch_block);
            ctx.link(body_end, after_block);
            ctx.active_block = catch_block;
            ctx.emit(
                catch_block,
                MirInstr {
                    dest: None,
                    op: MirOp::Catch,
                    span: None,
                },
            );
            let catch_slot = ctx.alloc_local(catch_name);
            ctx.emit(
                catch_block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(catch_slot),
                    span: None,
                },
            );
            for s in catch_body {
                lower_stmt(s, ctx, ctx.active_block);
            }
            let catch_end = ctx.active_block;
            ctx.emit(
                catch_end,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(after_block),
                    span: None,
                },
            );
            ctx.link(catch_end, after_block);
            ctx.active_block = after_block;
        }
        Stmt::Throw { expr, .. } => {
            let dest = ctx.fresh_reg();
            lower_expr(expr, dest, ctx, block);
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Throw,
                    span: None,
                },
            );
        }
        Stmt::Switch {
            subject,
            cases,
            default,
            ..
        } => {
            let subject_reg = ctx.fresh_reg();
            lower_expr(subject, subject_reg, ctx, block);
            let case_slot = ctx.alloc_local(&SmolStr::new("__switch_val"));
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(case_slot),
                    span: None,
                },
            );
            let end_block = ctx.fresh_block();
            let mut current_block = block;
            for (i, case) in cases.iter().enumerate() {
                lower_switch_pattern_match(&case.pattern, case_slot, ctx, current_block);
                let next_block = if i + 1 < cases.len() || default.is_some() {
                    ctx.fresh_block()
                } else {
                    end_block
                };
                ctx.emit(
                    current_block,
                    MirInstr {
                        dest: None,
                        op: MirOp::JumpIfFalse(next_block),
                        span: None,
                    },
                );
                bind_pattern(&case.pattern, case_slot, ctx, current_block);
                // S1-I5: `where` guard after bindings so `case .err(let e) where e.count > 0` works.
                if let Some(guard) = &case.guard {
                    let guard_reg = ctx.fresh_reg();
                    lower_expr(guard, guard_reg, ctx, current_block);
                    let guard_block = ctx.active_block;
                    ctx.emit(
                        guard_block,
                        MirInstr {
                            dest: None,
                            op: MirOp::JumpIfFalse(next_block),
                            span: None,
                        },
                    );
                    current_block = guard_block;
                }
                for s in &case.body {
                    lower_stmt(s, ctx, current_block);
                }
                ctx.emit(
                    current_block,
                    MirInstr {
                        dest: None,
                        op: MirOp::Jump(end_block),
                        span: None,
                    },
                );
                ctx.link(current_block, end_block);
                ctx.active_block = next_block;
                current_block = next_block;
            }
            if let Some(body) = default {
                for s in body {
                    lower_stmt(s, ctx, current_block);
                }
                ctx.emit(
                    current_block,
                    MirInstr {
                        dest: None,
                        op: MirOp::Jump(end_block),
                        span: None,
                    },
                );
                ctx.link(current_block, end_block);
            } else if current_block != end_block {
                // No default: last case already uses `end_block` as its failure
                // target; do not emit Jump(end) on end itself (infinite loop).
                ctx.emit(
                    current_block,
                    MirInstr {
                        dest: None,
                        op: MirOp::Jump(end_block),
                        span: None,
                    },
                );
                ctx.link(current_block, end_block);
            }
            ctx.active_block = end_block;
        }
        Stmt::IfCase {
            pattern,
            subject,
            body,
            else_branch,
            ..
        } => {
            let subject_reg = ctx.fresh_reg();
            lower_expr(subject, subject_reg, ctx, block);
            let case_slot = ctx.alloc_local(&SmolStr::new("__ifcase_val"));
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(case_slot),
                    span: None,
                },
            );
            lower_switch_pattern_match(pattern, case_slot, ctx, block);
            let fail_block = ctx.fresh_block();
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::JumpIfFalse(fail_block),
                    span: None,
                },
            );
            bind_pattern(pattern, case_slot, ctx, block);
            for s in body {
                lower_stmt(s, ctx, block);
            }
            let end_block = ctx.fresh_block();
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(end_block),
                    span: None,
                },
            );
            ctx.link(block, end_block);
            if let Some(else_b) = else_branch {
                ctx.active_block = fail_block;
                for s in else_b {
                    lower_stmt(s, ctx, fail_block);
                }
                ctx.emit(
                    fail_block,
                    MirInstr {
                        dest: None,
                        op: MirOp::Jump(end_block),
                        span: None,
                    },
                );
                ctx.link(fail_block, end_block);
            } else {
                ctx.link(fail_block, end_block);
            }
            ctx.active_block = end_block;
        }
        // U-7: `repeat { body } while cond` — body runs at least once.
        Stmt::RepeatWhile {
            body, condition, ..
        } => {
            let body_block = ctx.fresh_block();
            let cond_block = ctx.fresh_block();
            let exit_block = ctx.fresh_block();
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(body_block),
                    span: None,
                },
            );
            ctx.link(block, body_block);
            // continue re-checks condition; break exits.
            ctx.loop_stack.push((cond_block, exit_block, None));
            ctx.active_block = body_block;
            for s in body {
                let b = ctx.active_block;
                lower_stmt(s, ctx, b);
            }
            let body_end = ctx.active_block;
            ctx.loop_stack.pop();
            ctx.emit(
                body_end,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(cond_block),
                    span: None,
                },
            );
            ctx.link(body_end, cond_block);
            let cond_reg = ctx.fresh_reg();
            lower_expr(condition, cond_reg, ctx, cond_block);
            ctx.emit(
                cond_block,
                MirInstr {
                    dest: None,
                    op: MirOp::JumpIfFalse(exit_block),
                    span: None,
                },
            );
            ctx.emit(
                cond_block,
                MirInstr {
                    dest: None,
                    op: MirOp::Jump(body_block),
                    span: None,
                },
            );
            ctx.link(cond_block, exit_block);
            ctx.link(cond_block, body_block);
            ctx.active_block = exit_block;
        }
        Stmt::Break { label, .. } => {
            match ctx.resolve_loop_target(label.as_ref()) {
                Some((_, break_block)) => {
                    let cur = ctx.active_block;
                    ctx.emit(
                        cur,
                        MirInstr {
                            dest: None,
                            op: MirOp::Jump(break_block),
                            span: None,
                        },
                    );
                    ctx.link(cur, break_block);
                    // Dead continuation so fall-through after break does not rejoin.
                    ctx.active_block = ctx.fresh_block();
                }
                None => {
                    ctx.set_error_with_code(
                        Some("MIR001"),
                        if label.is_some() {
                            "break label does not match any enclosing loop"
                        } else {
                            "break outside of loop"
                        },
                    );
                }
            }
        }
        Stmt::Continue { label, .. } => match ctx.resolve_loop_target(label.as_ref()) {
            Some((continue_block, _)) => {
                let cur = ctx.active_block;
                ctx.emit(
                    cur,
                    MirInstr {
                        dest: None,
                        op: MirOp::Jump(continue_block),
                        span: None,
                    },
                );
                ctx.link(cur, continue_block);
                ctx.active_block = ctx.fresh_block();
            }
            None => {
                ctx.set_error_with_code(
                    Some("MIR001"),
                    if label.is_some() {
                        "continue label does not match any enclosing loop"
                    } else {
                        "continue outside of loop"
                    },
                );
            }
        },
        _ => {}
    }
}
