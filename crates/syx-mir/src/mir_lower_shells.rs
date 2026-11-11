//! SYX MIR type/dataclass/enum/varglobal shell lowering + merge helpers.
//! Extracted in S2-I204 (zero semantics; crate-private + public shell APIs).

use crate::flatten_blocks;
use crate::lower_expr;
use crate::mir_local_names_for_slots;
use crate::mir_local_types_for_slots;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_module_index::ModuleIndex;
use crate::mir_types::{MirBlock, MirFunction, MirInstr, MirModule, MirOp};
use indexmap::IndexSet;
use smol_str::SmolStr;
use std::cell::Cell;
use std::rc::Rc;
use syx_ast::{Decl, Expr, Literal, Module};
use syx_diagnostics::SyxResult;
use syx_stdlib::{host_fn_index, HostFnId};

pub(crate) fn mir_op(op: MirOp) -> MirInstr {
    MirInstr {
        dest: None,
        op,
        span: None,
    }
}

pub(crate) fn mir_shell_fn(name: SmolStr, param_count: u32, local_count: u32, instructions: Vec<MirInstr>) -> MirFunction {
    MirFunction {
        name,
        param_count,
        local_count,
        entry: 0,
        blocks: vec![MirBlock {
            id: 0,
            instructions: instructions.clone(),
            successors: vec![],
        }],
        instructions,
        local_names: vec![],
        local_types: vec![],
        init_body_pc: 0,
        init_wrap_pc: 0,
    }
}

/// Dataclass/Struct init/copy/describe/hash shells for native/WASM
/// (mirrors codegen type shells; W467 / ADR-811: Struct joined DataClass on MIR path).
pub fn lower_dataclass_shells(module: &Module) -> Vec<MirFunction> {
    let mut out = Vec::new();
    for decl in &module.decls {
        let (name, fields) = match decl {
            Decl::DataClass { name, fields, .. } | Decl::Struct { name, fields, .. } => {
                (name, fields)
            }
            _ => continue,
        };
        out.push(lower_dataclass_init_shell(name, fields));
        out.push(lower_dataclass_copy_shell(name, fields));
        out.push(lower_dataclass_describe_shell(name, fields));
        out.push(lower_dataclass_hash_into_shell(name, fields));
        // R-20: structural equality. MIR lowering emits `Call("{T}_==")` for any
        // struct / data class operand, so these shells are mandatory, not optional.
        out.push(lower_dataclass_eq_shell(name, fields));
        out.push(lower_dataclass_ne_shell(name, fields));
    }
    out
}

/// FinalClass variant init shells for native/WASM (W468 / ADR-812).
/// Mirrors codegen `Decl::FinalClass` DictBuild (`value`/`variant`/`__type`); ≠ Enum `tag`/`case`.
pub fn lower_finalclass_shells(module: &Module) -> Vec<MirFunction> {
    let mut out = Vec::new();
    for decl in &module.decls {
        let Decl::FinalClass { name, variants, .. } = decl else {
            continue;
        };
        for (variant, fields) in variants {
            let init_name = SmolStr::new(format!("{name}_{variant}_init"));
            let mut ins = Vec::new();
            if fields.len() == 1 {
                ins.push(mir_op(MirOp::LoadLocal(0)));
                ins.push(mir_op(MirOp::ConstStr("value".into())));
            } else {
                for (slot, (field, _)) in fields.iter().enumerate() {
                    ins.push(mir_op(MirOp::LoadLocal(slot as u32)));
                    ins.push(mir_op(MirOp::ConstStr(field.to_string())));
                }
            }
            ins.push(mir_op(MirOp::ConstStr(variant.to_string())));
            ins.push(mir_op(MirOp::ConstStr("variant".into())));
            ins.push(mir_op(MirOp::ConstStr(name.to_string())));
            ins.push(mir_op(MirOp::ConstStr("__type".into())));
            let entry_count = if fields.is_empty() {
                2
            } else if fields.len() == 1 {
                3
            } else {
                fields.len() + 2
            };
            ins.push(mir_op(MirOp::DictBuild(entry_count as u32)));
            ins.push(mir_op(MirOp::Return));
            out.push(mir_shell_fn(
                init_name,
                fields.len() as u32,
                fields.len() as u32,
                ins,
            ));
        }
    }
    out
}

/// Top-level VarGlobal init for native/WASM (W469 / ADR-813).
/// Emits **one** combined `__init_global` (declaration-order slots) — equivalent to AST's
/// multiple same-named units, but survives `compile_from_mir` name-dedupe.
pub fn lower_varglobal_inits(module: &Module) -> SyxResult<Vec<MirFunction>> {
    let mut globals = Vec::new();
    let mut any_init = false;
    for decl in &module.decls {
        if let Decl::VarGlobal { name, init, .. } = decl {
            globals.push(name.clone());
            if init.is_some() {
                any_init = true;
            }
        }
    }
    if !any_init {
        return Ok(vec![]);
    }

    let module_idx = ModuleIndex::from_module(module);
    let mut imports = IndexSet::new();
    for decl in &module.decls {
        if let Some(sym) = decl.import_symbol() {
            imports.insert(sym.clone());
        }
    }

    let init_name = SmolStr::new("__init_global");
    // High water mark so `__mir_closure_N` from inits never collide with
    // per-function counters that also start at 0 (S1-I4 CallClosure).
    let mut ctx = LowerCtx::new(
        init_name.clone(),
        module_idx,
        imports,
        globals,
        Rc::new(Cell::new(100_000u32)),
    );
    let entry = ctx.fresh_block();
    ctx.active_block = entry;

    let mut slot = 0u32;
    for decl in &module.decls {
        let Decl::VarGlobal { init, .. } = decl else {
            continue;
        };
        if let Some(expr) = init {
            let dest = ctx.fresh_reg();
            lower_expr(expr, dest, &mut ctx, entry);
            ctx.emit(
                entry,
                MirInstr {
                    dest: None,
                    op: MirOp::StoreGlobal(slot),
                    span: None,
                },
            );
        }
        slot += 1;
    }
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
    let mut out = std::mem::take(&mut ctx.pending_closures);
    let blocks = ctx.blocks;
    let instructions = flatten_blocks(&blocks);
    let local_count = ctx.locals.len() as u32;
    let local_names = mir_local_names_for_slots(&ctx.locals, local_count);
    let local_types = mir_local_types_for_slots(&ctx.locals, &ctx.local_types, local_count);
    out.push(MirFunction {
        name: init_name,
        param_count: 0,
        local_count,
        entry,
        blocks,
        instructions,
        local_names,
        local_types,
        init_body_pc: 0,
        init_wrap_pc: 0,
    });
    Ok(out)
}

/// Enum case init shells for native/WASM (mirrors codegen `{Enum}_{case}_init`).
pub fn lower_enum_shells(module: &Module) -> Vec<MirFunction> {
    let mut out = Vec::new();
    for decl in &module.decls {
        let Decl::Enum { name, cases, .. } = decl else {
            continue;
        };
        let mut next_tag = 0i64;
        for (case, value, fields) in cases {
            let tag = if let Some(expr) = value {
                match expr {
                    Expr::Literal {
                        value: Literal::Integer { value: v, .. },
                        ..
                    } => Some(*v as i64),
                    _ => None,
                }
            } else {
                None
            };
            let tag_val = tag.unwrap_or(next_tag);
            next_tag = tag_val + 1;
            let init_name = SmolStr::new(format!("{name}_{case}_init"));
            if fields.is_empty() {
                let ins = vec![
                    mir_op(MirOp::ConstI32(tag_val as i32)),
                    mir_op(MirOp::ConstStr("tag".into())),
                    mir_op(MirOp::ConstStr(case.to_string())),
                    mir_op(MirOp::ConstStr("case".into())),
                    mir_op(MirOp::ConstStr(name.to_string())),
                    mir_op(MirOp::ConstStr("__type".into())),
                    mir_op(MirOp::DictBuild(3)),
                    mir_op(MirOp::Return),
                ];
                out.push(mir_shell_fn(init_name, 0, 0, ins));
            } else {
                let mut ins = Vec::new();
                ins.push(mir_op(MirOp::ConstI32(tag_val as i32)));
                ins.push(mir_op(MirOp::ConstStr("tag".into())));
                ins.push(mir_op(MirOp::ConstStr(case.to_string())));
                ins.push(mir_op(MirOp::ConstStr("case".into())));
                for (slot, (field, _)) in fields.iter().enumerate() {
                    ins.push(mir_op(MirOp::LoadLocal(slot as u32)));
                    ins.push(mir_op(MirOp::ConstStr(field.to_string())));
                }
                ins.push(mir_op(MirOp::ConstStr(name.to_string())));
                ins.push(mir_op(MirOp::ConstStr("__type".into())));
                ins.push(mir_op(MirOp::DictBuild((fields.len() + 3) as u32)));
                ins.push(mir_op(MirOp::Return));
                out.push(mir_shell_fn(
                    init_name,
                    fields.len() as u32,
                    fields.len() as u32,
                    ins,
                ));
            }
        }
    }
    out
}

pub(crate) fn lower_dataclass_init_shell(
    name: &SmolStr,
    fields: &[(SmolStr, syx_ast::TypeExpr)],
) -> MirFunction {
    let mut ins = Vec::new();
    for (slot, (field, _)) in fields.iter().enumerate() {
        ins.push(mir_op(MirOp::LoadLocal(slot as u32)));
        ins.push(mir_op(MirOp::ConstStr(field.to_string())));
    }
    ins.push(mir_op(MirOp::ConstStr(name.to_string())));
    ins.push(mir_op(MirOp::ConstStr("__type".to_string())));
    ins.push(mir_op(MirOp::DictBuild((fields.len() + 1) as u32)));
    ins.push(mir_op(MirOp::Return));
    mir_shell_fn(
        SmolStr::new(format!("{name}_init")),
        fields.len() as u32,
        fields.len() as u32,
        ins,
    )
}

pub(crate) fn lower_dataclass_copy_shell(
    name: &SmolStr,
    fields: &[(SmolStr, syx_ast::TypeExpr)],
) -> MirFunction {
    let init_name = SmolStr::new(format!("{name}_init"));
    let mut ins = Vec::new();
    for (field, _) in fields {
        ins.push(mir_op(MirOp::LoadLocal(0)));
        ins.push(mir_op(MirOp::ConstStr(field.to_string())));
        ins.push(mir_op(MirOp::ObjectGet));
    }
    ins.push(mir_op(MirOp::Call(init_name)));
    ins.push(mir_op(MirOp::Return));
    mir_shell_fn(SmolStr::new(format!("{name}_copy")), 1, 1, ins)
}

/// R-20: structural equality shell `{T}_==`.
///
/// MIR lowering emits `Call("{T}_==")` for any struct / data class operand
/// (`mir_lower_expr_binary.rs`), but no shell was synthesized, so every comparison
/// failed at check time with `[C020] Unknown function 'Point_=='`.
///
/// Semantics: field-by-field structural equality over the runtime bag. Local 0 is the
/// left operand, local 1 the right operand, local 2 accumulates the running boolean.
pub(crate) fn lower_dataclass_eq_shell(
    name: &SmolStr,
    fields: &[(SmolStr, syx_ast::TypeExpr)],
) -> MirFunction {
    let mut ins = Vec::new();
    if fields.is_empty() {
        // No fields: same-type values are structurally equal.
        ins.push(mir_op(MirOp::ConstBool(true)));
        ins.push(mir_op(MirOp::Return));
        return mir_shell_fn(SmolStr::new(format!("{name}_==")), 2, 2, ins);
    }
    // acc = true
    ins.push(mir_op(MirOp::ConstBool(true)));
    ins.push(mir_op(MirOp::StoreLocal(2)));
    for (field, _) in fields {
        // acc = acc && (l[field] == r[field])
        ins.push(mir_op(MirOp::LoadLocal(2)));
        ins.push(mir_op(MirOp::LoadLocal(0)));
        ins.push(mir_op(MirOp::ConstStr(field.to_string())));
        ins.push(mir_op(MirOp::ObjectGet));
        ins.push(mir_op(MirOp::LoadLocal(1)));
        ins.push(mir_op(MirOp::ConstStr(field.to_string())));
        ins.push(mir_op(MirOp::ObjectGet));
        ins.push(mir_op(MirOp::ValueEq));
        ins.push(mir_op(MirOp::And));
        ins.push(mir_op(MirOp::StoreLocal(2)));
    }
    ins.push(mir_op(MirOp::LoadLocal(2)));
    ins.push(mir_op(MirOp::Return));
    mir_shell_fn(SmolStr::new(format!("{name}_==")), 2, 3, ins)
}

/// R-20: `{T}_!=` — negation of `{T}_==`.
pub(crate) fn lower_dataclass_ne_shell(
    name: &SmolStr,
    fields: &[(SmolStr, syx_ast::TypeExpr)],
) -> MirFunction {
    let eq_name = SmolStr::new(format!("{name}_=="));
    let mut ins = Vec::new();
    ins.push(mir_op(MirOp::LoadLocal(0)));
    ins.push(mir_op(MirOp::LoadLocal(1)));
    ins.push(mir_op(MirOp::Call(eq_name)));
    ins.push(mir_op(MirOp::Not));
    ins.push(mir_op(MirOp::Return));
    mir_shell_fn(SmolStr::new(format!("{name}_!=")), 2, 2, ins)
}

pub(crate) fn lower_dataclass_describe_shell(
    name: &SmolStr,
    fields: &[(SmolStr, syx_ast::TypeExpr)],
) -> MirFunction {
    // R-22: every field gets a `name: ` label, including the first. The previous version
    // only prefixed non-first fields, printing `Point(3, y: 4)`.
    let mut ins = vec![mir_op(MirOp::ConstStr(format!("{name}(")))];
    for (i, (field, _)) in fields.iter().enumerate() {
        let prefix = if i == 0 {
            format!("{field}: ")
        } else {
            format!(", {field}: ")
        };
        ins.push(mir_op(MirOp::ConstStr(prefix)));
        ins.push(mir_op(MirOp::ConcatStr));
        ins.push(mir_op(MirOp::LoadLocal(0)));
        ins.push(mir_op(MirOp::ConstStr(field.to_string())));
        ins.push(mir_op(MirOp::ObjectGet));
        ins.push(mir_op(MirOp::ToStr));
        ins.push(mir_op(MirOp::ConcatStr));
    }
    ins.push(mir_op(MirOp::ConstStr(")".to_string())));
    ins.push(mir_op(MirOp::ConcatStr));
    ins.push(mir_op(MirOp::Return));
    mir_shell_fn(SmolStr::new(format!("{name}_describe")), 1, 1, ins)
}

pub(crate) fn lower_dataclass_hash_into_shell(
    name: &SmolStr,
    fields: &[(SmolStr, syx_ast::TypeExpr)],
) -> MirFunction {
    let mut ins = Vec::new();
    for (field, _) in fields {
        ins.push(mir_op(MirOp::LoadLocal(1)));
        ins.push(mir_op(MirOp::LoadLocal(0)));
        ins.push(mir_op(MirOp::ConstStr(field.to_string())));
        ins.push(mir_op(MirOp::ObjectGet));
        ins.push(mir_op(MirOp::CallHost(host_fn_index(
            HostFnId::HasherCombine,
        ))));
        ins.push(mir_op(MirOp::StoreLocal(1)));
    }
    ins.push(mir_op(MirOp::LoadLocal(1)));
    ins.push(mir_op(MirOp::Return));
    mir_shell_fn(
        SmolStr::new(format!("{name}_hash_into")),
        2,
        2,
        ins,
    )
}

pub fn merge_mir_functions(mir: &mut MirModule, extra: Vec<MirFunction>) {
    for func in extra {
        if !mir.functions.iter().any(|f| f.name == func.name) {
            mir.functions.push(func);
        }
    }
}

/// R-21: merge synthesized dataclass/enum/finalclass shells, **replacing** any existing
/// function with the same name.
///
/// Why this differs from [`merge_mir_functions`]: `lower_class_init` runs first and emits a
/// generic `{Type}_init` with `param_count = 0` that builds only `{__type: "Type"}`. For a
/// struct or data class the memberwise shell from `lower_dataclass_init_shell` is the
/// correct one (`param_count = fields.len()`, builds every field). Plain name-presence
/// merging silently kept the generic zero-arg version, so every struct instance was
/// constructed with empty fields — which is why `p.x` read back `nil` on this path and
/// `p == r` compared `nil == nil` into `true`.
pub fn merge_mir_shell_functions(mir: &mut MirModule, extra: Vec<MirFunction>) {
    for func in extra {
        match mir.functions.iter_mut().find(|f| f.name == func.name) {
            Some(existing) => *existing = func,
            None => mir.functions.push(func),
        }
    }
}

/// Prefix merged user-dependency MIR symbols and relink internal calls (SYX 39.0).
pub fn prefix_user_dep_mir_batch(functions: &mut [MirFunction], module_stem: &str) {
    let prefix = format!("{module_stem}_");
    let mut rename = indexmap::IndexMap::new();
    for f in functions.iter() {
        if f.name.as_str() == "main" || f.name.starts_with(prefix.as_str()) {
            continue;
        }
        rename.insert(
            f.name.clone(),
            SmolStr::new(format!("{prefix}{}", f.name)),
        );
    }
    fn relink_instr(instr: &mut MirInstr, rename: &indexmap::IndexMap<SmolStr, SmolStr>) {
        match &mut instr.op {
            MirOp::Call(name) | MirOp::ActorCall(name) | MirOp::TaskSpawn(name) => {
                if let Some(new_name) = rename.get(name) {
                    *name = new_name.clone();
                }
            }
            MirOp::MakeClosure(name, _) => {
                if let Some(new_name) = rename.get(name) {
                    *name = new_name.clone();
                }
            }
            _ => {}
        }
    }
    for f in functions.iter_mut() {
        if let Some(new_name) = rename.get(&f.name) {
            f.name = new_name.clone();
        }
        for instr in &mut f.instructions {
            relink_instr(instr, &rename);
        }
        for block in &mut f.blocks {
            for instr in &mut block.instructions {
                relink_instr(instr, &rename);
            }
        }
    }
}

