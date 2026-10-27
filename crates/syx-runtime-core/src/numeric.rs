use syx_stdlib::{
    bit_width, int_range, is_float, is_integer, is_integer_narrowing, is_signed, is_unsigned,
    resolve_alias, type_name, NumericKind, PLATFORM_INT,
};

use crate::Value;

pub fn i128_from_lo_hi(lo: u64, hi: u64) -> i128 {
    ((hi as u128) << 64 | lo as u128) as i128
}

pub fn u128_from_lo_hi(lo: u64, hi: u64) -> u128 {
    (hi as u128) << 64 | lo as u128
}

pub fn lo_hi_from_i128(v: i128) -> (u64, u64) {
    let u = v as u128;
    (u as u64, (u >> 64) as u64)
}

pub fn lo_hi_from_u128(v: u128) -> (u64, u64) {
    (v as u64, (v >> 64) as u64)
}

pub fn value_from_numeric128(kind: NumericKind, lo: u64, hi: u64) -> Value {
    match resolve_alias(kind) {
        NumericKind::I128 => Value::I128(i128_from_lo_hi(lo, hi)),
        NumericKind::U128 => Value::U128(u128_from_lo_hi(lo, hi)),
        other => value_from_numeric(other, lo),
    }
}

pub fn value_from_numeric(kind: NumericKind, bits: u64) -> Value {
    match resolve_alias(kind) {
        NumericKind::I8 => Value::I8(bits as i8),
        NumericKind::I16 => Value::I16(bits as i16),
        NumericKind::I32 => Value::I32(bits as i32),
        NumericKind::I64 => Value::I64(bits as i64),
        NumericKind::I128 => Value::I128(bits as i64 as i128),
        NumericKind::U8 => Value::U8(bits as u8),
        NumericKind::U16 => Value::U16(bits as u16),
        NumericKind::U32 => Value::U32(bits as u32),
        NumericKind::U64 => Value::U64(bits),
        NumericKind::U128 => Value::U128(bits as u128),
        NumericKind::F32 | NumericKind::Float => Value::F32(f32::from_bits(bits as u32)),
        NumericKind::F64 | NumericKind::Double => Value::F64(f64::from_bits(bits)),
        NumericKind::Int | NumericKind::UInt => unreachable!(),
    }
}

pub fn numeric_bits(v: &Value) -> Option<(NumericKind, u64)> {
    match v {
        Value::I8(n) => Some((NumericKind::I8, *n as u64)),
        Value::I16(n) => Some((NumericKind::I16, *n as u64)),
        Value::I32(n) => Some((NumericKind::I32, *n as u64)),
        Value::I64(n) => Some((NumericKind::I64, *n as u64)),
        Value::I128(n) => {
            let (lo, _) = lo_hi_from_i128(*n);
            Some((NumericKind::I128, lo))
        }
        Value::U8(n) => Some((NumericKind::U8, *n as u64)),
        Value::U16(n) => Some((NumericKind::U16, *n as u64)),
        Value::U32(n) => Some((NumericKind::U32, *n as u64)),
        Value::U64(n) => Some((NumericKind::U64, *n)),
        Value::U128(n) => {
            let (lo, _) = lo_hi_from_u128(*n);
            Some((NumericKind::U128, lo))
        }
        Value::F32(n) => Some((NumericKind::F32, n.to_bits() as u64)),
        Value::F64(n) => Some((NumericKind::F64, n.to_bits())),
        _ => None,
    }
}

pub fn value_kind(v: &Value) -> Option<NumericKind> {
    numeric_bits(v).map(|(k, _)| k)
}

pub fn platform_int_value(n: i64) -> Value {
    match PLATFORM_INT {
        NumericKind::I32 => Value::I32(n as i32),
        NumericKind::I64 => Value::I64(n),
        _ => Value::I32(n as i32),
    }
}

impl Value {
    pub fn as_i64(&self) -> i64 {
        match self {
            Value::I8(v) => *v as i64,
            Value::I16(v) => *v as i64,
            Value::I32(v) => *v as i64,
            Value::I64(v) => *v,
            Value::I128(v) => *v as i64,
            Value::U8(v) => *v as i64,
            Value::U16(v) => *v as i64,
            Value::U32(v) => *v as i64,
            Value::U64(v) => *v as i64,
            Value::U128(v) => *v as i64,
            Value::F32(v) => *v as i64,
            Value::F64(v) => *v as i64,
            _ => 0,
        }
    }

    pub fn as_i128(&self) -> i128 {
        match self {
            Value::I128(v) => *v,
            Value::U128(v) => *v as i128,
            _ => self.as_i64() as i128,
        }
    }

    pub fn as_u128(&self) -> u128 {
        match self {
            Value::U128(v) => *v,
            Value::I128(v) => *v as u128,
            _ => self.as_i64() as u128,
        }
    }

    pub fn numeric_eq(a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::F32(x), Value::F32(y)) => x == y,
            (Value::F64(x), Value::F64(y)) => x == y,
            (Value::I128(x), Value::I128(y)) => x == y,
            (Value::U128(x), Value::U128(y)) => x == y,
            _ => a.as_i64() == b.as_i64(),
        }
    }
}

fn float_operands(a: &Value, b: &Value) -> bool {
    matches!(a, Value::F32(_) | Value::F64(_)) || matches!(b, Value::F32(_) | Value::F64(_))
}

fn as_f64_operand(v: &Value) -> f64 {
    match v {
        Value::F64(n) => *n,
        Value::F32(n) => *n as f64,
        _ => v.as_i64() as f64,
    }
}

pub fn numeric_add(kind: NumericKind, a: &Value, b: &Value) -> Option<Value> {
    let kind = resolve_alias(kind);
    if kind == NumericKind::I128 {
        return Some(Value::I128(a.as_i128().wrapping_add(b.as_i128())));
    }
    if kind == NumericKind::U128 {
        return Some(Value::U128(a.as_u128().wrapping_add(b.as_u128())));
    }
    // MIR often tags Add as platform-int; still honor float operands (W1064 Mat.mean).
    if is_float(kind) || float_operands(a, b) {
        let av = as_f64_operand(a);
        let bv = as_f64_operand(b);
        return Some(if kind == NumericKind::F32 {
            Value::F32((av + bv) as f32)
        } else {
            Value::F64(av + bv)
        });
    }
    if is_unsigned(kind) {
        let av = a.as_i64() as u64;
        let bv = b.as_i64() as u64;
        let r = av.wrapping_add(bv);
        return Some(value_from_numeric(kind, r));
    }
    let r = a.as_i64().wrapping_add(b.as_i64());
    Some(value_from_numeric(kind, r as u64))
}

pub fn numeric_sub(kind: NumericKind, a: &Value, b: &Value) -> Option<Value> {
    let kind = resolve_alias(kind);
    if kind == NumericKind::I128 {
        return Some(Value::I128(a.as_i128().wrapping_sub(b.as_i128())));
    }
    if kind == NumericKind::U128 {
        return Some(Value::U128(a.as_u128().wrapping_sub(b.as_u128())));
    }
    // MIR often tags Sub as platform-int; still honor float operands.
    if is_float(kind) || float_operands(a, b) {
        let av = as_f64_operand(a);
        let bv = as_f64_operand(b);
        return Some(if kind == NumericKind::F32 {
            Value::F32((av - bv) as f32)
        } else {
            Value::F64(av - bv)
        });
    }
    if is_unsigned(kind) {
        let av = a.as_i64() as u64;
        let bv = b.as_i64() as u64;
        let r = av.wrapping_sub(bv);
        return Some(value_from_numeric(kind, r));
    }
    let r = a.as_i64().wrapping_sub(b.as_i64());
    Some(value_from_numeric(kind, r as u64))
}

fn negate(kind: NumericKind, v: &Value) -> Option<Value> {
    let kind = resolve_alias(kind);
    if kind == NumericKind::I128 {
        return match v {
            Value::I128(n) => Some(Value::I128(n.wrapping_neg())),
            _ => Some(Value::I128(-v.as_i128())),
        };
    }
    if kind == NumericKind::U128 {
        return match v {
            Value::U128(n) => Some(Value::U128(n.wrapping_neg())),
            _ => Some(Value::U128((-v.as_i128()) as u128)),
        };
    }
    if is_float(kind) {
        return match v {
            Value::F32(n) => Some(Value::F32(-n)),
            Value::F64(n) => Some(Value::F64(-n)),
            _ => None,
        };
    }
    let n = v.as_i64();
    Some(value_from_numeric(kind, (-n) as u64))
}

pub fn numeric_mul(kind: NumericKind, a: &Value, b: &Value) -> Option<Value> {
    let kind = resolve_alias(kind);
    if kind == NumericKind::I128 {
        return Some(Value::I128(a.as_i128().wrapping_mul(b.as_i128())));
    }
    if kind == NumericKind::U128 {
        return Some(Value::U128(a.as_u128().wrapping_mul(b.as_u128())));
    }
    // MIR often tags Mul as platform-int; still honor float operands.
    if is_float(kind) || float_operands(a, b) {
        let av = as_f64_operand(a);
        let bv = as_f64_operand(b);
        if kind == NumericKind::F32 {
            return Some(Value::F32((av * bv) as f32));
        }
        return Some(Value::F64(av * bv));
    }
    let r = a.as_i64().wrapping_mul(b.as_i64());
    Some(value_from_numeric(kind, r as u64))
}

pub fn numeric_div(kind: NumericKind, a: &Value, b: &Value) -> Option<Value> {
    let kind = resolve_alias(kind);
    if kind == NumericKind::I128 {
        let bv = b.as_i128();
        if bv == 0 {
            return None;
        }
        return Some(Value::I128(a.as_i128().wrapping_div(bv)));
    }
    if kind == NumericKind::U128 {
        let bv = b.as_u128();
        if bv == 0 {
            return None;
        }
        return Some(Value::U128(a.as_u128().wrapping_div(bv)));
    }
    // MIR often tags Div as platform-int; still honor float operands (W1064 Mat.mean).
    if is_float(kind) || float_operands(a, b) {
        let av = as_f64_operand(a);
        let bv = as_f64_operand(b);
        if bv == 0.0 {
            return None;
        }
        if kind == NumericKind::F32 {
            return Some(Value::F32((av / bv) as f32));
        }
        return Some(Value::F64(av / bv));
    }
    let av = a.as_i64();
    let bv = b.as_i64();
    if bv == 0 {
        return None;
    }
    Some(value_from_numeric(kind, (av / bv) as u64))
}

pub fn numeric_mod(kind: NumericKind, a: &Value, b: &Value) -> Option<Value> {
    let kind = resolve_alias(kind);
    if kind == NumericKind::I128 {
        let bv = b.as_i128();
        if bv == 0 {
            return None;
        }
        return Some(Value::I128(a.as_i128().wrapping_rem(bv)));
    }
    if kind == NumericKind::U128 {
        let bv = b.as_u128();
        if bv == 0 {
            return None;
        }
        return Some(Value::U128(a.as_u128().wrapping_rem(bv)));
    }
    let av = a.as_i64();
    let bv = b.as_i64();
    if bv == 0 {
        return None;
    }
    Some(value_from_numeric(kind, (av % bv) as u64))
}

pub fn numeric_neg(kind: NumericKind, v: &Value) -> Option<Value> {
    negate(resolve_alias(kind), v)
}

fn shift_mask(kind: NumericKind) -> u32 {
    (bit_width(kind) as u32).saturating_sub(1)
}

pub fn numeric_bitand(kind: NumericKind, a: &Value, b: &Value) -> Option<Value> {
    let kind = resolve_alias(kind);
    if is_float(kind) {
        return None;
    }
    if kind == NumericKind::I128 {
        return Some(Value::I128(a.as_i128() & b.as_i128()));
    }
    if kind == NumericKind::U128 {
        return Some(Value::U128(a.as_u128() & b.as_u128()));
    }
    if is_unsigned(kind) {
        let r = (a.as_i64() as u64) & (b.as_i64() as u64);
        return Some(value_from_numeric(kind, r));
    }
    Some(value_from_numeric(kind, (a.as_i64() & b.as_i64()) as u64))
}

pub fn numeric_bitor(kind: NumericKind, a: &Value, b: &Value) -> Option<Value> {
    let kind = resolve_alias(kind);
    if is_float(kind) {
        return None;
    }
    if kind == NumericKind::I128 {
        return Some(Value::I128(a.as_i128() | b.as_i128()));
    }
    if kind == NumericKind::U128 {
        return Some(Value::U128(a.as_u128() | b.as_u128()));
    }
    if is_unsigned(kind) {
        let r = (a.as_i64() as u64) | (b.as_i64() as u64);
        return Some(value_from_numeric(kind, r));
    }
    Some(value_from_numeric(kind, (a.as_i64() | b.as_i64()) as u64))
}

pub fn numeric_bitxor(kind: NumericKind, a: &Value, b: &Value) -> Option<Value> {
    let kind = resolve_alias(kind);
    if is_float(kind) {
        return None;
    }
    if kind == NumericKind::I128 {
        return Some(Value::I128(a.as_i128() ^ b.as_i128()));
    }
    if kind == NumericKind::U128 {
        return Some(Value::U128(a.as_u128() ^ b.as_u128()));
    }
    if is_unsigned(kind) {
        let r = (a.as_i64() as u64) ^ (b.as_i64() as u64);
        return Some(value_from_numeric(kind, r));
    }
    Some(value_from_numeric(kind, (a.as_i64() ^ b.as_i64()) as u64))
}

pub fn numeric_bitnot(kind: NumericKind, v: &Value) -> Option<Value> {
    let kind = resolve_alias(kind);
    if is_float(kind) {
        return None;
    }
    if kind == NumericKind::I128 {
        return Some(Value::I128(!v.as_i128()));
    }
    if kind == NumericKind::U128 {
        return Some(Value::U128(!v.as_u128()));
    }
    if is_unsigned(kind) {
        let w = bit_width(kind);
        let mask = if w >= 64 {
            u64::MAX
        } else {
            (1u64 << w) - 1
        };
        return Some(value_from_numeric(kind, (!(v.as_i64() as u64)) & mask));
    }
    Some(value_from_numeric(kind, (!v.as_i64()) as u64))
}

pub fn numeric_shl(kind: NumericKind, a: &Value, b: &Value) -> Option<Value> {
    let kind = resolve_alias(kind);
    if is_float(kind) {
        return None;
    }
    let shift = (b.as_i64() as u32) & shift_mask(kind);
    if kind == NumericKind::I128 {
        return Some(Value::I128(a.as_i128().wrapping_shl(shift)));
    }
    if kind == NumericKind::U128 {
        return Some(Value::U128(a.as_u128().wrapping_shl(shift)));
    }
    if is_unsigned(kind) {
        return Some(value_from_numeric(
            kind,
            (a.as_i64() as u64).wrapping_shl(shift),
        ));
    }
    Some(value_from_numeric(
        kind,
        a.as_i64().wrapping_shl(shift) as u64,
    ))
}

pub fn numeric_shr(kind: NumericKind, a: &Value, b: &Value) -> Option<Value> {
    let kind = resolve_alias(kind);
    if is_float(kind) {
        return None;
    }
    let shift = (b.as_i64() as u32) & shift_mask(kind);
    if kind == NumericKind::I128 {
        return Some(Value::I128(a.as_i128().wrapping_shr(shift)));
    }
    if kind == NumericKind::U128 {
        return Some(Value::U128(a.as_u128().wrapping_shr(shift)));
    }
    if is_unsigned(kind) {
        return Some(value_from_numeric(
            kind,
            (a.as_i64() as u64).wrapping_shr(shift),
        ));
    }
    Some(value_from_numeric(
        kind,
        a.as_i64().wrapping_shr(shift) as u64,
    ))
}

pub fn numeric_cmp(kind: NumericKind, a: &Value, b: &Value) -> Option<std::cmp::Ordering> {
    let kind = resolve_alias(kind);
    if kind == NumericKind::I128 {
        return a.as_i128().partial_cmp(&b.as_i128());
    }
    if kind == NumericKind::U128 {
        return a.as_u128().partial_cmp(&b.as_u128());
    }
    if is_float(kind) {
        let av = match a {
            Value::F32(v) => *v as f64,
            Value::F64(v) => *v,
            _ => a.as_i64() as f64,
        };
        let bv = match b {
            Value::F32(v) => *v as f64,
            Value::F64(v) => *v,
            _ => b.as_i64() as f64,
        };
        return av.partial_cmp(&bv);
    }
    if is_unsigned(kind) {
        return (a.as_i64() as u64).partial_cmp(&(b.as_i64() as u64));
    }
    Some(a.as_i64().cmp(&b.as_i64()))
}

pub fn numeric_min(kind: NumericKind, a: &Value, b: &Value) -> Option<Value> {
    numeric_cmp(kind, a, b).and_then(|ord| match ord {
        std::cmp::Ordering::Greater => Some(b.clone()),
        _ => Some(a.clone()),
    })
}

pub fn numeric_max(kind: NumericKind, a: &Value, b: &Value) -> Option<Value> {
    numeric_cmp(kind, a, b).and_then(|ord| match ord {
        std::cmp::Ordering::Less => Some(b.clone()),
        _ => Some(a.clone()),
    })
}

pub fn numeric_abs(kind: NumericKind, v: &Value) -> Option<Value> {
    let kind = resolve_alias(kind);
    if is_float(kind) {
        return Some(match v {
            Value::F32(n) => Value::F32(n.abs()),
            Value::F64(n) => Value::F64(n.abs()),
            _ => v.clone(),
        });
    }
    if is_unsigned(kind) {
        return Some(v.clone());
    }
    if kind == NumericKind::I128 {
        let n = v.as_i128();
        return Some(Value::I128(if n < 0 { n.wrapping_neg() } else { n }));
    }
    let n = v.as_i64();
    Some(value_from_numeric(kind, n.unsigned_abs() as u64))
}

pub fn numeric_cast(from: NumericKind, to: NumericKind, v: &Value) -> Value {
    let from = resolve_alias(from);
    let to = resolve_alias(to);
    if from == to {
        return v.clone();
    }
    if to == NumericKind::I128 {
        return Value::I128(v.as_i128());
    }
    if to == NumericKind::U128 {
        return Value::U128(v.as_u128());
    }
    if from == NumericKind::I128 || from == NumericKind::U128 {
        if is_float(to) {
            let n = if from == NumericKind::I128 {
                v.as_i128() as f64
            } else {
                v.as_u128() as f64
            };
            return if to == NumericKind::F32 {
                Value::F32(n as f32)
            } else {
                Value::F64(n)
            };
        }
        return value_from_numeric(to, v.as_i128() as u64);
    }
    if is_float(from) && is_float(to) {
        let f = match v {
            Value::F32(n) => *n as f64,
            Value::F64(n) => *n,
            _ => v.as_i64() as f64,
        };
        return if to == NumericKind::F32 {
            Value::F32(f as f32)
        } else {
            Value::F64(f)
        };
    }
    if is_float(from) && is_integer(to) {
        let f = match v {
            Value::F64(n) => *n,
            Value::F32(n) => *n as f64,
            _ => v.as_i64() as f64,
        };
        if to == NumericKind::I128 {
            return Value::I128(f.trunc() as i128);
        }
        if to == NumericKind::U128 {
            return Value::U128(f.trunc() as u128);
        }
        return value_from_numeric(to, f.trunc() as i64 as u64);
    }
    if is_integer(from) && is_float(to) {
        let n = v.as_i128() as f64;
        return if to == NumericKind::F32 {
            Value::F32(n as f32)
        } else {
            Value::F64(n)
        };
    }
    value_from_numeric(to, v.as_i64() as u64)
}

/// Like [`numeric_cast`], but integer narrowing / float→int out of `int_range` is an error.
pub fn try_numeric_cast(
    from: NumericKind,
    to: NumericKind,
    v: &Value,
) -> Result<Value, String> {
    let from_r = resolve_alias(from);
    let to_r = resolve_alias(to);
    if is_integer_narrowing(from_r, to_r) {
        if let Some((lo, hi)) = int_range(to_r) {
            let n = v.as_i128();
            if n < lo || n > hi {
                return Err(format!(
                    "numeric value {n} out of range for {}",
                    type_name(to_r)
                ));
            }
        }
    }
    if is_float(from_r) && is_integer(to_r) {
        let f = match v {
            Value::F64(n) => *n,
            Value::F32(n) => *n as f64,
            _ => v.as_i64() as f64,
        };
        if !f.is_finite() {
            return Err(format!(
                "numeric value {f} out of range for {}",
                type_name(to_r)
            ));
        }
        let truncated = f.trunc();
        if let Some((lo, hi)) = int_range(to_r) {
            // Compare in f64 for magnitudes beyond exact i128 float representation.
            if truncated < lo as f64 || truncated > hi as f64 {
                return Err(format!(
                    "numeric value {truncated} out of range for {}",
                    type_name(to_r)
                ));
            }
        }
    }
    Ok(numeric_cast(from, to, v))
}

pub fn display_numeric(v: &Value) -> Option<String> {
    match v {
        Value::I8(n) => Some(n.to_string()),
        Value::I16(n) => Some(n.to_string()),
        Value::I32(n) => Some(n.to_string()),
        Value::I64(n) => Some(n.to_string()),
        Value::I128(n) => Some(n.to_string()),
        Value::U8(n) => Some(n.to_string()),
        Value::U16(n) => Some(n.to_string()),
        Value::U32(n) => Some(n.to_string()),
        Value::U64(n) => Some(n.to_string()),
        Value::U128(n) => Some(n.to_string()),
        Value::F32(n) => Some(n.to_string()),
        Value::F64(n) => Some(n.to_string()),
        _ => None,
    }
}
