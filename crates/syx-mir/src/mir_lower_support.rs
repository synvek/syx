//! SYX MIR lower support helpers (flatten / locals / keypath / opaque).
//! Extracted in S2-I212 (zero semantics; crate-private).

use crate::mir_lower_ctx::LowerCtx;
use crate::mir_types::{MirBlock, MirInstr, MirOp};
use smol_str::SmolStr;
use syx_ast::Expr;

pub(crate) fn flatten_blocks(blocks: &[MirBlock]) -> Vec<MirInstr> {
    let mut block_start = indexmap::IndexMap::new();
    let mut out = Vec::new();
    for block in blocks {
        block_start.insert(block.id, out.len());
        out.extend(block.instructions.iter().cloned());
    }
    resolve_jump_targets(&mut out, &block_start);
    out
}


fn resolve_jump_targets(instructions: &mut [MirInstr], block_start: &indexmap::IndexMap<u32, usize>) {
    for instr in instructions {
        match &mut instr.op {
            MirOp::Jump(t)
            | MirOp::JumpIfFalse(t)
            | MirOp::JumpIfNil(t)
            | MirOp::BeginTry(t)
            | MirOp::BindResult(t) => {
                if let Some(&pc) = block_start.get(t) {
                    *t = pc as u32;
                }
            }
            _ => {}
        }
    }
}


pub(crate) fn mir_local_names_for_slots(
    locals: &indexmap::IndexMap<SmolStr, u32>,
    local_count: u32,
) -> Vec<SmolStr> {
    let mut names = (0..local_count)
        .map(|i| SmolStr::new(format!("local_{i}")))
        .collect::<Vec<_>>();
    for (name, slot) in locals {
        if (*slot as usize) < names.len() {
            names[*slot as usize] = name.clone();
        }
    }
    names
}


pub(crate) fn mir_local_types_for_slots(
    locals: &indexmap::IndexMap<SmolStr, u32>,
    local_types: &indexmap::IndexMap<SmolStr, SmolStr>,
    local_count: u32,
) -> Vec<SmolStr> {
    let mut types = (0..local_count)
        .map(|_| SmolStr::new("Unknown"))
        .collect::<Vec<_>>();
    for (name, slot) in locals {
        if let Some(ty) = local_types.get(name) {
            if (*slot as usize) < types.len() {
                types[*slot as usize] = ty.clone();
            }
        }
    }
    types
}


pub(crate) fn expr_is_opaque_operand(expr: &Expr, ctx: &LowerCtx) -> bool {
    match expr {
        Expr::Call { callee, .. } => matches!(
            callee.as_ref(),
            Expr::Ident { name, .. } if ctx.module.opaque_functions.contains(name)
        ),
        _ => false,
    }
}


pub(crate) fn keypath_from_callee(expr: &Expr) -> Option<(&SmolStr, &SmolStr, bool)> {
    match expr {
        Expr::KeyPath {
            root_type,
            property,
            optional,
            ..
        } => Some((root_type, property, *optional)),
        Expr::Group { expr, .. } => keypath_from_callee(expr),
        _ => None,
    }
}

