//! SYX MIR `Expr::TypeSelf` lowering.
//! Extracted in S2-I193 (zero semantics; crate-private).

use crate::mir_lower_ctx::LowerCtx;
use crate::mir_types::{MirInstr, MirOp};
use smol_str::SmolStr;
use syx_stdlib::{host_fn_index, HostFnId};

pub(crate) fn lower_expr_type_self(type_name: &SmolStr, dest: u32, ctx: &mut LowerCtx, block: u32) {
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
}
