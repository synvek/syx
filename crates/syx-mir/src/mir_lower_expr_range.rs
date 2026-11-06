//! SYX MIR range lowering (bounds / contains / dict / HalfOpen·Closed·RangeDyn arms).
//! Extracted in S2-I195 (zero semantics; crate-private).

use crate::lower_expr;
use crate::mir_lower_ctx::LowerCtx;
use crate::mir_types::{MirInstr, MirOp};
use smol_str::SmolStr;
use syx_ast::Expr;
use syx_stdlib::PLATFORM_INT;

pub(crate) fn range_bounds_from_expr(expr: &Expr) -> Option<(i64, i64, bool)> {
    match expr {
        Expr::HalfOpenRange { start, end, .. } => Some((*start, *end, false)),
        Expr::ClosedRange { start, end, .. } => Some((*start, *end, true)),
        Expr::Group { expr, .. } => range_bounds_from_expr(expr),
        _ => None,
    }
}

pub(crate) fn lower_range_contains_mir(
    start: i64,
    end: i64,
    inclusive: bool,
    arg: &Expr,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    let val_slot = ctx.alloc_local(&SmolStr::new("__range_contains_val"));
    lower_expr(arg, ctx.fresh_reg(), ctx, block);
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::StoreLocal(val_slot),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::LoadLocal(val_slot),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::ConstI32(start as i32),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::Ge {
                kind: PLATFORM_INT.to_byte(),
            },
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::LoadLocal(val_slot),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::ConstI32(end as i32),
            span: None,
        },
    );
    let cmp_kind = PLATFORM_INT.to_byte();
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: if inclusive {
                MirOp::Le { kind: cmp_kind }
            } else {
                MirOp::Lt { kind: cmp_kind }
            },
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op: MirOp::And,
            span: None,
        },
    );
}

pub(crate) fn lower_range_dict_dyn(
    ctx: &mut LowerCtx,
    block: u32,
    dest: u32,
    start: &Expr,
    end: &Expr,
    inclusive: bool,
) {
    let pair_count = if inclusive { 3 } else { 2 };
    lower_expr(start, dest, ctx, block);
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::ConstStr("start".into()),
            span: None,
        },
    );
    lower_expr(end, dest, ctx, block);
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::ConstStr("end".into()),
            span: None,
        },
    );
    if inclusive {
        ctx.emit(
            block,
            MirInstr {
                dest: Some(dest),
                op: MirOp::ConstBool(true),
                span: None,
            },
        );
        ctx.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ConstStr("inclusive".into()),
                span: None,
            },
        );
    }
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op: MirOp::DictBuild(pair_count),
            span: None,
        },
    );
}

pub(crate) fn lower_range_dict(
    ctx: &mut LowerCtx,
    block: u32,
    dest: u32,
    start: i64,
    end: i64,
    inclusive: bool,
) {
    let pair_count = if inclusive { 3 } else { 2 };
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op: MirOp::ConstNumeric {
                kind: PLATFORM_INT.to_byte(),
                bits: start as u64,
            },
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::ConstStr("start".into()),
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op: MirOp::ConstNumeric {
                kind: PLATFORM_INT.to_byte(),
                bits: end as u64,
            },
            span: None,
        },
    );
    ctx.emit(
        block,
        MirInstr {
            dest: None,
            op: MirOp::ConstStr("end".into()),
            span: None,
        },
    );
    if inclusive {
        ctx.emit(
            block,
            MirInstr {
                dest: Some(dest),
                op: MirOp::ConstBool(true),
                span: None,
            },
        );
        ctx.emit(
            block,
            MirInstr {
                dest: None,
                op: MirOp::ConstStr("inclusive".into()),
                span: None,
            },
        );
    }
    ctx.emit(
        block,
        MirInstr {
            dest: Some(dest),
            op: MirOp::DictBuild(pair_count),
            span: None,
        },
    );
}

pub(crate) fn lower_expr_half_open_range(
    start: i64,
    end: i64,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    lower_range_dict(ctx, block, dest, start, end, false);
}

pub(crate) fn lower_expr_closed_range(
    start: i64,
    end: i64,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    lower_range_dict(ctx, block, dest, start, end, true);
}

pub(crate) fn lower_expr_range_dyn(
    start: &Expr,
    end: &Expr,
    inclusive: bool,
    dest: u32,
    ctx: &mut LowerCtx,
    block: u32,
) {
    lower_range_dict_dyn(ctx, block, dest, start, end, inclusive);
}
