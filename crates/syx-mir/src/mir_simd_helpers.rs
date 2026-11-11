//! SYX MIR SIMD lowering helpers.
//! Extracted in S2-I209 (zero semantics; crate-private).

use crate::mir_host_cast_helpers::resolve_host_call;
use crate::mir_lower_ctx::LowerCtx;
use syx_ast::{CallArg, Expr, Literal};
use syx_stdlib::{from_type_name, host_fn_id_by_name, HostFnId};

fn expr_type_ident_name(expr: &Expr) -> Option<&str> {
    match expr {
        Expr::Ident { name, .. } => Some(name.as_str()),
        _ => None,
    }
}

fn simd_element_from_type_suffix(suffix: &str) -> u8 {
    match suffix {
        "Float64" | "Double" | "f64" => syx_stdlib::NumericKind::F64.to_byte(),
        "Float32" | "Float" | "f32" | "f" => syx_stdlib::NumericKind::F32.to_byte(),
        "Int16" | "i16" => syx_stdlib::NumericKind::I16.to_byte(),
        "Int64" | "i64" => syx_stdlib::NumericKind::I64.to_byte(),
        "UInt64" | "u64" => syx_stdlib::NumericKind::U64.to_byte(),
        "Int8" | "i8" => syx_stdlib::NumericKind::I8.to_byte(),
        _ => syx_stdlib::NumericKind::I32.to_byte(),
    }
}

pub(crate) fn simd_member_lanes_element(object: &Expr) -> (u8, u8) {
    expr_type_ident_name(object)
        .map(simd_lanes_element_from_type)
        .unwrap_or((4, syx_stdlib::NumericKind::I32.to_byte()))
}

fn simd_lanes_element_from_type(type_name: &str) -> (u8, u8) {
    if let Some(open) = type_name.find('<') {
        let base = &type_name[..open];
        let elem = type_name[open + 1..].trim_end_matches('>');
        let lanes = base
            .strip_prefix("SIMD")
            .and_then(|s| s.parse().ok())
            .unwrap_or(4);
        return (lanes, simd_element_from_type_suffix(elem));
    }
    if type_name == "SIMD2" {
        (2, syx_stdlib::NumericKind::F64.to_byte())
    } else if type_name == "SIMD8" {
        (8, syx_stdlib::NumericKind::I16.to_byte())
    } else if type_name == "SIMD16" {
        (16, syx_stdlib::NumericKind::I8.to_byte())
    } else {
        (4, syx_stdlib::NumericKind::I32.to_byte())
    }
}

pub(crate) fn infer_simd_element_from_array_arg(args: &[CallArg], ctx: &LowerCtx) -> u8 {
    let Some(first) = args.first() else {
        return syx_stdlib::NumericKind::I32.to_byte();
    };
    if let Expr::Array { elements, .. } = &first.value {
        if let Some(el) = elements.first() {
            if let Some(element) = infer_simd_element_from_expr(el) {
                return element;
            }
        }
    }
    if let Expr::Ident { name, .. } = &first.value {
        if let Some(ty) = ctx.local_types.get(name.as_str()) {
            if ty.starts_with("Array<") && ty.ends_with('>') {
                let inner = ty.trim_start_matches("Array<").trim_end_matches('>');
                return simd_element_from_type_suffix(inner);
            }
        }
    }
    // Phase 117: `loadFromArray(storeToArray(...), …)` — peek store’s array/simd args.
    if let Expr::Call {
        callee,
        args: store_args,
        ..
    } = &first.value
    {
        if let Expr::Member { name, .. } = callee.as_ref() {
            if name.as_str() == "storeToArray" && !store_args.is_empty() {
                let from_arr = infer_simd_element_from_array_arg(store_args, ctx);
                if from_arr != syx_stdlib::NumericKind::I32.to_byte() {
                    return from_arr;
                }
                if let Some(simd_arg) = store_args.get(2) {
                    if let Some(el) = infer_simd_element_from_reduce_arg(&simd_arg.value, ctx) {
                        return el;
                    }
                }
            }
        }
    }
    syx_stdlib::NumericKind::I32.to_byte()
}

pub(crate) fn infer_simd_element_from_reduce_arg(arg: &Expr, ctx: &LowerCtx) -> Option<u8> {
    match arg {
        Expr::Call { callee, args, .. } => {
            if let Expr::Member { object, name, .. } = callee.as_ref() {
                match name.as_str() {
                    "addF32" | "subF32" | "mulF32" | "divF32" => {
                        return Some(syx_stdlib::NumericKind::F32.to_byte());
                    }
                    "addF64" | "subF64" | "mulF64" | "divF64" => {
                        return Some(syx_stdlib::NumericKind::F64.to_byte());
                    }
                    "addI16" => {
                        return Some(syx_stdlib::NumericKind::I16.to_byte());
                    }
                    "addI64" => {
                        return Some(syx_stdlib::NumericKind::I64.to_byte());
                    }
                    "addSat" | "subSat" | "add" | "sub" | "mul" | "eq" | "lt" | "gt" | "min"
                    | "max" | "and" | "or" | "xor" | "not" | "shl" | "shr"
                        if !args.is_empty() =>
                    {
                        if let Some(type_name) = expr_type_ident_name(object) {
                            let (_, element) = simd_lanes_element_from_type(type_name);
                            return Some(element);
                        }
                    }
                    "loadFromArray" if !args.is_empty() => {
                        return Some(infer_simd_element_from_array_arg(args, ctx));
                    }
                    "fromArray" if !args.is_empty() => {
                        return Some(infer_simd_element_from_array_arg(args, ctx));
                    }
                    _ => {}
                }
                if let Some(type_name) = expr_type_ident_name(object) {
                    if let Some((_, element)) = parse_simd_type(type_name) {
                        return Some(element.to_byte());
                    }
                }
            }
            None
        }
        Expr::Ident { .. } => infer_expr_simd_type(arg, ctx).map(|(_, k)| k.to_byte()),
        _ => None,
    }
}

fn infer_simd_element_from_expr(expr: &Expr) -> Option<u8> {
    match expr {
        Expr::Literal {
            value: Literal::Double { suffix, .. },
            ..
        } => {
            if let Some(s) = suffix.as_deref() {
                if s == "f32" || s == "f" {
                    return Some(syx_stdlib::NumericKind::F32.to_byte());
                }
                if s == "f64" {
                    return Some(syx_stdlib::NumericKind::F64.to_byte());
                }
            }
            Some(syx_stdlib::NumericKind::F32.to_byte())
        }
        Expr::Literal {
            value: Literal::Integer { suffix, .. },
            ..
        } => {
            let ty = suffix
                .as_deref()
                .unwrap_or(if cfg!(target_pointer_width = "64") {
                    "i64"
                } else {
                    "i32"
                });
            Some(simd_element_from_type_suffix(ty))
        }
        _ => None,
    }
}

pub(crate) fn simd_reduce_add_shape(callee: &Expr, func_name: &str) -> (u8, u8) {
    if let Expr::Call { callee: inner, .. } = callee {
        if let Expr::Member { object, .. } = inner.as_ref() {
            if let Some(name) = expr_type_ident_name(object) {
                return simd_lanes_element_from_type(name);
            }
        }
    }
    if let Some(type_name) = func_name.strip_suffix("_reduceAdd") {
        return simd_lanes_element_from_type(type_name);
    }
    (4, syx_stdlib::NumericKind::I32.to_byte())
}

/// Host SIMD calls that lower their own args onto the MIR stack; skip ArgsEarly pre-pass.
///
/// Note: `SIMD*_fromArray` is a **stdlib facade** (`Call SIMD4_fromArray`), not a direct
/// CallHost that packs lanes itself. Skipping ArgsEarly for it drops the array literal and
/// yields VM `SIMD operation failed` on the subsequent `SimdAdd`. Keep ArgsEarly for fromArray.
pub(crate) fn host_simd_lowers_own_args(name: &str) -> bool {
    host_fn_id_by_name(name).is_some_and(|h| {
        matches!(
            h,
            HostFnId::SimdShuffle
                | HostFnId::SimdAnd
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
                | HostFnId::SimdI8Not
                | HostFnId::SimdI8Min
                | HostFnId::SimdI8Max
                | HostFnId::SimdI8UMin
                | HostFnId::SimdI8UMax
                | HostFnId::SimdI8Abs
                | HostFnId::SimdI8Neg
                | HostFnId::SimdI8Extract
                | HostFnId::SimdI8Replace
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
                | HostFnId::SimdI8Shuffle
                | HostFnId::SimdI8Shl
                | HostFnId::SimdI8Shr
                | HostFnId::SimdI64Add
                | HostFnId::SimdReduceAdd
                | HostFnId::SimdNot
                | HostFnId::SimdLoadFromArray
                | HostFnId::SimdStoreToArray
        )
    })
}

/// Static SIMD member calls that lower their own args; skip ArgsEarly dupes.
/// `fromArray` is intentionally excluded — see `host_simd_lowers_own_args`.
pub(crate) fn member_host_simd_lowers_own_args(
    object: &Expr,
    method: &str,
    ctx: &LowerCtx,
) -> bool {
    resolve_host_call(object, method, ctx).is_some_and(|(h, _)| {
        matches!(
            h,
            HostFnId::SimdLoadFromArray
                | HostFnId::SimdStoreToArray
                | HostFnId::SimdShuffle
                | HostFnId::SimdAnd
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
                | HostFnId::SimdI8Not
                | HostFnId::SimdI8Min
                | HostFnId::SimdI8Max
                | HostFnId::SimdI8UMin
                | HostFnId::SimdI8UMax
                | HostFnId::SimdI8Abs
                | HostFnId::SimdI8Neg
                | HostFnId::SimdI8Extract
                | HostFnId::SimdI8Replace
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
                | HostFnId::SimdI8Shuffle
                | HostFnId::SimdI8Shl
                | HostFnId::SimdI8Shr
                | HostFnId::SimdI64Add
                | HostFnId::SimdReduceAdd
                | HostFnId::SimdNot
        )
    })
}

fn parse_simd_type(type_name: &str) -> Option<(u8, syx_stdlib::NumericKind)> {
    let rest = type_name.strip_prefix("SIMD")?;
    let (lanes_str, inner) = rest.split_once('<')?;
    let lanes: u8 = lanes_str.parse().ok()?;
    let elem = inner.trim_end_matches('>').trim();
    from_type_name(elem).map(|k| (lanes, k))
}

pub(crate) fn infer_expr_simd_type(
    expr: &Expr,
    ctx: &LowerCtx,
) -> Option<(u8, syx_stdlib::NumericKind)> {
    if let Expr::Ident { name, .. } = expr {
        return ctx
            .local_types
            .get(name)
            .and_then(|t| parse_simd_type(t.as_str()));
    }
    None
}
