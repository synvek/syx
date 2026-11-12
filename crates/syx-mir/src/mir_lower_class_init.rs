//! SYX MIR class init / protocol witness / vtable lowering.
//! Extracted in S2-I206 (zero semantics; crate-private).

use crate::flatten_blocks;
use crate::lower_stmt;
use crate::mir_local_names_for_slots;
use crate::mir_local_types_for_slots;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_module_index::ModuleIndex;
use crate::mir_types::{MirFunction, MirInstr, MirModule, MirOp};
use crate::register_param_local_type;
use indexmap::IndexSet;
use smol_str::SmolStr;
use std::cell::Cell;
use std::rc::Rc;
use syx_ast::Stmt;
use syx_diagnostics::SyxResult;

pub(crate) fn emit_protocol_witnesses(
    ctx: &mut LowerCtx,
    block: u32,
    self_slot: u32,
    type_name: &SmolStr,
    conformances: &[SmolStr],
) {
    const PROTOCOL_METHODS: &[(&str, &[&str])] = &[
        ("Iterator", &["next"]),
        ("Comparable", &["compare"]),
        ("Equatable", &["equals"]),
    ];
    for protocol in conformances {
        let methods: Vec<SmolStr> = if let Some(user) = ctx.module.protocol_methods.get(protocol) {
            user.clone()
        } else {
            PROTOCOL_METHODS
                .iter()
                .find(|(proto_name, _)| protocol.as_str() == *proto_name)
                .map(|(_, ms)| ms.iter().map(|m| SmolStr::new(*m)).collect())
                .unwrap_or_default()
        };
        for method in &methods {
            let fn_name = SmolStr::new(format!("{type_name}_{method}"));
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(self_slot),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstFnIndex(fn_name.to_string()),
                    span: None,
                },
            );
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstStr(format!("__witness_{protocol}_{method}")),
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
            // R-45: ObjectSet leaves the updated object on the stack; write back
            // self so subsequent ArrayBuild / ctors do not alias stack garbage.
            ctx.emit(
                block,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreLocal(self_slot),
                    span: None,
                },
            );
        }
    }
}

pub(crate) fn emit_class_vtable_slots_mir(
    ctx: &mut LowerCtx,
    block: u32,
    self_slot: u32,
    type_name: &SmolStr,
) {
    let Some(vtable) = ctx.module.class_vtables.get(type_name).cloned() else {
        return;
    };
    for (method, fn_name) in vtable {
        ctx.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::LoadLocal(self_slot),
                span: None,
            },
        );
        ctx.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ConstFnIndex(fn_name.to_string()),
                span: None,
            },
        );
        ctx.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ConstStr(format!("__vtable_{method}")),
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
        // R-45: same StoreLocal hygiene as protocol witnesses (vtable ObjectSet).
        ctx.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::StoreLocal(self_slot),
                span: None,
            },
        );
    }
}

pub(crate) fn mir_instruction_count(ctx: &LowerCtx) -> u32 {
    ctx.blocks.iter().map(|b| b.instructions.len() as u32).sum()
}

pub(crate) fn emit_mut_ref_cell_wrap_mir(ctx: &mut LowerCtx, block: u32, self_slot: u32) {
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::LoadLocal(self_slot),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::ConstStr("__syx_ref".to_string()),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::ConstStr("MutRefCell".to_string()),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::ConstStr("__type".to_string()),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::DictBuild(2),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::StoreLocal(self_slot),
            span: None,
        },
    );
}

pub(crate) fn lower_class_init(
    mir: &mut MirModule,
    type_name: &SmolStr,
    fields: &[SmolStr],
    params: &[syx_ast::Param],
    body: &[Stmt],
    is_actor: bool,
    conformances: &[SmolStr],
    module_idx: &ModuleIndex,
    imports: &IndexSet<SmolStr>,
    globals: &[SmolStr],
    closure_ids: Rc<Cell<u32>>,
) -> SyxResult<()> {
    let init_name = SmolStr::new(format!("{type_name}_init"));
    let mut ctx = LowerCtx::new(
        init_name.clone(),
        module_idx.clone(),
        imports.clone(),
        globals.to_vec(),
        closure_ids,
    );
    // Phase 125: parity with codegen — init builds raw dict then wraps MutRefCell after body.
    ctx.mut_ref_params.clear();
    ctx.mut_ref_aliases.clear();
    for param in params {
        ctx.alloc_local(&param.name);
        register_param_local_type(&mut ctx, param);
    }
    let self_slot = ctx.alloc_local(&SmolStr::new("self"));
    let entry = ctx.fresh_block();
    ctx.active_block = entry;
    for field in fields {
        let param_slot = params
            .iter()
            .find(|p| p.name == *field)
            .and_then(|p| ctx.locals.get(&p.name).copied());
        if let Some(slot) = param_slot {
            ctx.emit(
                entry,
                MirInstr {
                    dest: None,
                    op: MirOp::LoadLocal(slot),
                    span: None,
                },
            );
        } else if let Some(v) = module_idx
            .dataclass_field_default_i64
            .get(&(type_name.clone(), field.clone()))
            .copied()
        {
            // R-35: apply `var field: Int = N` when init omits the field.
            ctx.emit(
                entry,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstI32(v as i32),
                    span: None,
                },
            );
        } else {
            ctx.emit(
                entry,
                MirInstr {
                    dest: None,
                    op: MirOp::ConstNil,
                    span: None,
                },
            );
        }
        ctx.emit(
            entry,
            MirInstr {
                dest: None,
                op: MirOp::ConstStr(field.to_string()),
                span: None,
            },
        );
    }
    ctx.emit(
        entry,
        MirInstr {
            dest: None,
            op: MirOp::ConstStr(type_name.to_string()),
            span: None,
        },
    );
    ctx.emit(
        entry,
        MirInstr {
            dest: None,
            op: MirOp::ConstStr("__type".to_string()),
            span: None,
        },
    );
    ctx.emit(
        entry,
        MirInstr {
            dest: None,
            op: MirOp::DictBuild((fields.len() + 1) as u32),
            span: None,
        },
    );
    ctx.emit(
        entry,
        MirInstr {
            dest: None,
            op: MirOp::StoreLocal(self_slot),
            span: None,
        },
    );
    ctx.local_types
        .insert(SmolStr::new("self"), type_name.clone());
    emit_class_vtable_slots_mir(&mut ctx, entry, self_slot, type_name);
    if is_actor {
        ctx.emit(
            entry,
            MirInstr {
                dest: None,
                op: MirOp::LoadLocal(self_slot),
                span: None,
            },
        );
        ctx.emit(
            entry,
            MirInstr {
                dest: None,
                op: MirOp::ActorRegister,
                span: None,
            },
        );
        ctx.emit(
            entry,
            MirInstr {
                dest: None,
                op: MirOp::StoreLocal(self_slot),
                span: None,
            },
        );
    }
    emit_protocol_witnesses(&mut ctx, entry, self_slot, type_name, conformances);
    let init_body_pc = mir_instruction_count(&ctx);
    for stmt in body {
        lower_stmt(stmt, &mut ctx, entry);
    }
    let init_wrap_pc = mir_instruction_count(&ctx);
    // Struct/dataclass values stay bare dicts; class/actor instances use MutRefCell.
    if is_actor || module_idx.class_types.contains_key(type_name) {
        emit_mut_ref_cell_wrap_mir(&mut ctx, entry, self_slot);
    }
    emit_protocol_witnesses(&mut ctx, entry, self_slot, type_name, conformances);
    ctx.emit(
        entry,
        MirInstr {
            dest: None,
            op: MirOp::LoadLocal(self_slot),
            span: None,
        },
    );
    ctx.emit(
        entry,
        MirInstr {
            dest: None,
            op: MirOp::Return,
            span: None,
        },
    );
    if let Some(d) = ctx.take_error_diagnostic() {
        return Err(d);
    }
    let blocks = ctx.blocks;
    let instructions = flatten_blocks(&blocks);
    let local_count = ctx.locals.len() as u32;
    let local_names = mir_local_names_for_slots(&ctx.locals, local_count);
    let local_types = mir_local_types_for_slots(&ctx.locals, &ctx.local_types, local_count);
    let closures = std::mem::take(&mut ctx.pending_closures);
    mir.functions.push(MirFunction {
        name: init_name,
        param_count: params.len() as u32,
        local_count,
        entry,
        blocks,
        instructions,
        local_names,
        local_types,
        init_body_pc,
        init_wrap_pc,
    });
    mir.functions.extend(closures);
    Ok(())
}
