use syx_stdlib::{bit_width, is_float, is_signed, resolve_alias, NumericKind};

use crate::Value;

pub fn simd_from_array(lanes: u8, element: NumericKind, items: &[Value]) -> Option<Value> {
    let mut element = resolve_alias(element);
    // Phase 117: SIMD8 guest pilot packs as i16x8 (wasm simd128 width).
    if lanes == 8 {
        element = NumericKind::I16;
    }
    if lanes == 16 {
        element = NumericKind::I8;
    }
    if bit_width(element) > 64 {
        return None;
    }
    // Phase 111/113: `f32x4` + `f64x2` pilots; other float widths remain unsupported.
    if is_float(element)
        && !(element == NumericKind::F32 || (element == NumericKind::F64 && lanes == 2))
    {
        return None;
    }
    let lane_bytes = (bit_width(element) / 8) as usize;
    let total = lanes as usize * lane_bytes;
    if items.len() != lanes as usize || total > 64 {
        return None;
    }
    let mut bits = [0u8; 64];
    for (i, item) in items.iter().enumerate() {
        let raw = scalar_to_u64(element, item)?;
        let off = i * lane_bytes;
        for b in 0..lane_bytes {
            bits[off + b] = ((raw >> (b * 8)) & 0xFF) as u8;
        }
    }
    Some(Value::Simd {
        lanes,
        element,
        bits,
    })
}

pub fn simd_reduce_add(v: &Value) -> Option<Value> {
    let Value::Simd { lanes, element, bits } = v else {
        return None;
    };
    let element = resolve_alias(*element);
    if is_float(element) {
        let mut sum = 0.0f64;
        for lane in 0..*lanes {
            sum += read_float_lane(element, bits, lane)?;
        }
        return Some(if element == NumericKind::F32 {
            Value::F32(sum as f32)
        } else {
            Value::F64(sum)
        });
    }
    if is_signed(element) {
        let mut sum: i128 = 0;
        for lane in 0..*lanes {
            sum = sum.wrapping_add(read_signed_lane(element, bits, lane)? as i128);
        }
        return Some(if element == NumericKind::I128 {
            Value::I128(sum)
        } else if element == NumericKind::I16 || element == NumericKind::I8 {
            // Promote narrow lane sums to platform Int for print/interop (SIMD8 pilot).
            super::numeric::platform_int_value(sum as i64)
        } else {
            super::numeric::value_from_numeric(element, sum as u64)
        });
    }
    let mut sum: u128 = 0;
    for lane in 0..*lanes {
        sum = sum.wrapping_add(read_unsigned_lane(element, bits, lane)?);
    }
    Some(if element == NumericKind::U128 {
        Value::U128(sum)
    } else {
        super::numeric::value_from_numeric(element, sum as u64)
    })
}

pub fn simd_binop(
    op: SimdBinOp,
    a: &Value,
    b: &Value,
) -> Option<Value> {
    let Value::Simd {
        lanes: la,
        element: ea,
        bits: ba,
    } = a
    else {
        return None;
    };
    let Value::Simd {
        lanes: lb,
        element: eb,
        bits: bb,
    } = b
    else {
        return None;
    };
    if la != lb || ea != eb {
        return None;
    }
    let element = resolve_alias(*ea);
    let mut out = [0u8; 64];
    for lane in 0..*la {
        if is_float(element) {
            let av = read_float_lane(element, ba, lane)?;
            let bv = read_float_lane(element, bb, lane)?;
            let r = match op {
                SimdBinOp::Add => av + bv,
                SimdBinOp::Sub => av - bv,
                SimdBinOp::Mul => av * bv,
                SimdBinOp::Div => av / bv,
            };
            write_float_lane(element, &mut out, lane, r)?;
        } else if is_signed(element) {
            let av = read_signed_lane(element, ba, lane)? as i128;
            let bv = read_signed_lane(element, bb, lane)? as i128;
            let r = match op {
                SimdBinOp::Add => av.wrapping_add(bv),
                SimdBinOp::Sub => av.wrapping_sub(bv),
                SimdBinOp::Mul => av.wrapping_mul(bv),
                SimdBinOp::Div => {
                    if bv == 0 {
                        return None;
                    }
                    av / bv
                }
            };
            write_signed_lane(element, &mut out, lane, r)?;
        } else {
            let av = read_unsigned_lane(element, ba, lane)?;
            let bv = read_unsigned_lane(element, bb, lane)?;
            let r = match op {
                SimdBinOp::Add => av.wrapping_add(bv),
                SimdBinOp::Sub => av.wrapping_sub(bv),
                SimdBinOp::Mul => av.wrapping_mul(bv),
                SimdBinOp::Div => {
                    if bv == 0 {
                        return None;
                    }
                    av / bv
                }
            };
            write_unsigned_lane(element, &mut out, lane, r)?;
        }
    }
    Some(Value::Simd {
        lanes: *la,
        element,
        bits: out,
    })
}

pub fn simd_broadcast(lanes: u8, element: NumericKind, scalar: &Value) -> Option<Value> {
    let element = resolve_alias(element);
    let mut items = Vec::with_capacity(lanes as usize);
    for _ in 0..lanes {
        items.push(scalar.clone());
    }
    simd_from_array(lanes, element, &items)
}

pub fn simd_equals(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (
            Value::Simd {
                lanes: la,
                element: ea,
                bits: ba,
            },
            Value::Simd {
                lanes: lb,
                element: eb,
                bits: bb,
            },
        ) => la == lb && ea == eb && ba == bb,
        _ => false,
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SimdBinOp {
    Add,
    Sub,
    Mul,
    Div,
}

pub fn simd_div(a: &Value, b: &Value) -> Option<Value> {
    simd_binop(SimdBinOp::Div, a, b)
}

pub fn simd_min(a: &Value, b: &Value) -> Option<Value> {
    lanewise_cmp(a, b, |av, bv| if av < bv { av } else { bv })
}

pub fn simd_max(a: &Value, b: &Value) -> Option<Value> {
    lanewise_cmp(a, b, |av, bv| if av > bv { av } else { bv })
}

/// Phase 130: unsigned lane-wise min (i8x16.min_u semantics regardless of element signedness).
pub fn simd_umin(a: &Value, b: &Value) -> Option<Value> {
    lanewise_ucmp(a, b, |av, bv| if av < bv { av } else { bv })
}

/// Phase 130: unsigned lane-wise max (i8x16.max_u semantics regardless of element signedness).
pub fn simd_umax(a: &Value, b: &Value) -> Option<Value> {
    lanewise_ucmp(a, b, |av, bv| if av > bv { av } else { bv })
}

/// Phase 131: signed lane-wise abs (`i8x16.abs`; INT_MIN stays INT_MIN).
pub fn simd_abs(v: &Value) -> Option<Value> {
    let Value::Simd {
        lanes,
        element,
        bits,
    } = v
    else {
        return None;
    };
    let element = resolve_alias(*element);
    if is_float(element) {
        return None;
    }
    let mut out = [0u8; 64];
    for lane in 0..*lanes {
        if is_signed(element) {
            let x = read_signed_lane(element, bits, lane)?;
            write_signed_lane(element, &mut out, lane, x.wrapping_abs() as i128)?;
        } else {
            let x = read_unsigned_lane(element, bits, lane)?;
            write_unsigned_lane(element, &mut out, lane, x)?;
        }
    }
    Some(Value::Simd {
        lanes: *lanes,
        element,
        bits: out,
    })
}

/// Phase 132: signed lane-wise neg (`i8x16.neg`; INT_MIN wraps).
pub fn simd_neg(v: &Value) -> Option<Value> {
    let Value::Simd {
        lanes,
        element,
        bits,
    } = v
    else {
        return None;
    };
    let element = resolve_alias(*element);
    if is_float(element) {
        return None;
    }
    let mut out = [0u8; 64];
    for lane in 0..*lanes {
        if is_signed(element) {
            let x = read_signed_lane(element, bits, lane)?;
            write_signed_lane(element, &mut out, lane, x.wrapping_neg() as i128)?;
        } else {
            let x = read_unsigned_lane(element, bits, lane)?;
            write_unsigned_lane(element, &mut out, lane, (0u128).wrapping_sub(x))?;
        }
    }
    Some(Value::Simd {
        lanes: *lanes,
        element,
        bits: out,
    })
}

/// Phase 130: unsigned lane compare helper for `umin`/`umax` (always read as unsigned).
fn lanewise_ucmp(
    a: &Value,
    b: &Value,
    pick: fn(u128, u128) -> u128,
) -> Option<Value> {
    let Value::Simd {
        lanes: la,
        element: ea,
        bits: ba,
    } = a
    else {
        return None;
    };
    let Value::Simd {
        lanes: lb,
        element: eb,
        bits: bb,
    } = b
    else {
        return None;
    };
    if la != lb || ea != eb {
        return None;
    }
    let element = resolve_alias(*ea);
    if is_float(element) {
        return None;
    }
    let mut out = [0u8; 64];
    for lane in 0..*la {
        let av = read_unsigned_lane(element, ba, lane)?;
        let bv = read_unsigned_lane(element, bb, lane)?;
        let r = pick(av, bv);
        write_unsigned_lane(element, &mut out, lane, r)?;
    }
    Some(Value::Simd {
        lanes: *la,
        element,
        bits: out,
    })
}

fn lanewise_cmp(
    a: &Value,
    b: &Value,
    pick: fn(i128, i128) -> i128,
) -> Option<Value> {
    let Value::Simd {
        lanes: la,
        element: ea,
        bits: ba,
    } = a
    else {
        return None;
    };
    let Value::Simd {
        lanes: lb,
        element: eb,
        bits: bb,
    } = b
    else {
        return None;
    };
    if la != lb || ea != eb {
        return None;
    }
    let element = resolve_alias(*ea);
    if is_float(element) {
        return None;
    }
    let mut out = [0u8; 64];
    for lane in 0..*la {
        let av = if is_signed(element) {
            read_signed_lane(element, ba, lane)? as i128
        } else {
            read_unsigned_lane(element, ba, lane)? as i128
        };
        let bv = if is_signed(element) {
            read_signed_lane(element, bb, lane)? as i128
        } else {
            read_unsigned_lane(element, bb, lane)? as i128
        };
        let r = pick(av, bv);
        if is_signed(element) {
            write_signed_lane(element, &mut out, lane, r)?;
        } else {
            write_unsigned_lane(element, &mut out, lane, r as u128)?;
        }
    }
    Some(Value::Simd {
        lanes: *la,
        element,
        bits: out,
    })
}

pub fn simd_eq(a: &Value, b: &Value) -> Option<Value> {
    let Value::Simd {
        lanes,
        element,
        bits: ba,
    } = a
    else {
        return None;
    };
    let Value::Simd {
        lanes: lb,
        element: eb,
        bits: bb,
    } = b
    else {
        return None;
    };
    if lanes != lb || element != eb {
        return None;
    }
    let element = resolve_alias(*element);
    let mut out = [0u8; 64];
    for lane in 0..*lanes {
        let eq = if is_float(element) {
            read_float_lane(element, ba, lane)? == read_float_lane(element, bb, lane)?
        } else if is_signed(element) {
            read_signed_lane(element, ba, lane)? == read_signed_lane(element, bb, lane)?
        } else {
            read_unsigned_lane(element, ba, lane)? == read_unsigned_lane(element, bb, lane)?
        };
        let mask = if eq { lane_mask(element) } else { 0 };
        write_unsigned_lane(element, &mut out, lane, mask as u128)?;
    }
    Some(Value::Simd {
        lanes: *lanes,
        element,
        bits: out,
    })
}

pub fn simd_lt(a: &Value, b: &Value) -> Option<Value> {
    let Value::Simd {
        lanes,
        element,
        bits: ba,
    } = a
    else {
        return None;
    };
    let Value::Simd {
        lanes: lb,
        element: eb,
        bits: bb,
    } = b
    else {
        return None;
    };
    if lanes != lb || element != eb {
        return None;
    }
    let element = resolve_alias(*element);
    let mut out = [0u8; 64];
    for lane in 0..*lanes {
        let lt = if is_float(element) {
            read_float_lane(element, ba, lane)? < read_float_lane(element, bb, lane)?
        } else if is_signed(element) {
            read_signed_lane(element, ba, lane)? < read_signed_lane(element, bb, lane)?
        } else {
            read_unsigned_lane(element, ba, lane)? < read_unsigned_lane(element, bb, lane)?
        };
        let mask = if lt { lane_mask(element) } else { 0 };
        write_unsigned_lane(element, &mut out, lane, mask as u128)?;
    }
    Some(Value::Simd {
        lanes: *lanes,
        element,
        bits: out,
    })
}

pub fn simd_gt(a: &Value, b: &Value) -> Option<Value> {
    let Value::Simd {
        lanes,
        element,
        bits: ba,
    } = a
    else {
        return None;
    };
    let Value::Simd {
        lanes: lb,
        element: eb,
        bits: bb,
    } = b
    else {
        return None;
    };
    if lanes != lb || element != eb {
        return None;
    }
    let element = resolve_alias(*element);
    let mut out = [0u8; 64];
    for lane in 0..*lanes {
        let gt = if is_float(element) {
            read_float_lane(element, ba, lane)? > read_float_lane(element, bb, lane)?
        } else if is_signed(element) {
            read_signed_lane(element, ba, lane)? > read_signed_lane(element, bb, lane)?
        } else {
            read_unsigned_lane(element, ba, lane)? > read_unsigned_lane(element, bb, lane)?
        };
        let mask = if gt { lane_mask(element) } else { 0 };
        write_unsigned_lane(element, &mut out, lane, mask as u128)?;
    }
    Some(Value::Simd {
        lanes: *lanes,
        element,
        bits: out,
    })
}

/// Phase 127: lane-wise signed saturating add (`i8x16.add_sat_s` guest pilot).
pub fn simd_add_sat(a: &Value, b: &Value) -> Option<Value> {
    lanewise_signed_sat_binop(a, b, |av, bv| av.saturating_add(bv))
}

/// Phase 127: lane-wise signed saturating sub (`i8x16.sub_sat_s` guest pilot).
pub fn simd_sub_sat(a: &Value, b: &Value) -> Option<Value> {
    lanewise_signed_sat_binop(a, b, |av, bv| av.saturating_sub(bv))
}

/// Phase 128: fixed-mode `i8x16` shuffle MVP (`i8x16.shuffle` guest pilot).
/// mode 0 = reverse, mode 1 = swap pairs, else identity.
pub fn simd_shuffle_i8(v: &Value, mode: i32) -> Option<Value> {
    let Value::Simd {
        lanes,
        element,
        bits,
    } = v
    else {
        return None;
    };
    if *lanes != 16 {
        return None;
    }
    let element = resolve_alias(*element);
    if element != NumericKind::I8 {
        return None;
    }
    let mut lane_vals = [0i8; 16];
    for i in 0..16 {
        lane_vals[i] = read_signed_lane(element, bits, i as u8)? as i8;
    }
    let shuffled = match mode {
        0 => {
            let mut out = [0i8; 16];
            for i in 0..16 {
                out[i] = lane_vals[15 - i];
            }
            out
        }
        1 => {
            let mut out = [0i8; 16];
            for i in 0..8 {
                out[i * 2] = lane_vals[i * 2 + 1];
                out[i * 2 + 1] = lane_vals[i * 2];
            }
            out
        }
        _ => lane_vals,
    };
    let mut out_bits = [0u8; 64];
    for (i, val) in shuffled.iter().enumerate() {
        write_signed_lane(element, &mut out_bits, i as u8, *val as i128)?;
    }
    Some(Value::Simd {
        lanes: *lanes,
        element,
        bits: out_bits,
    })
}

fn lanewise_signed_sat_binop(
    a: &Value,
    b: &Value,
    op: fn(i8, i8) -> i8,
) -> Option<Value> {
    let Value::Simd {
        lanes: la,
        element: ea,
        bits: ba,
    } = a
    else {
        return None;
    };
    let Value::Simd {
        lanes: lb,
        element: eb,
        bits: bb,
    } = b
    else {
        return None;
    };
    if la != lb || ea != eb {
        return None;
    }
    let element = resolve_alias(*ea);
    if !is_signed(element) || is_float(element) {
        return None;
    }
    let mut out = [0u8; 64];
    for lane in 0..*la {
        let av = read_signed_lane(element, ba, lane)? as i8;
        let bv = read_signed_lane(element, bb, lane)? as i8;
        let r = op(av, bv) as i128;
        write_signed_lane(element, &mut out, lane, r)?;
    }
    Some(Value::Simd {
        lanes: *la,
        element,
        bits: out,
    })
}

pub fn simd_ne(a: &Value, b: &Value) -> Option<Value> {
    let eq = simd_eq(a, b)?;
    simd_not(&eq)
}

pub fn simd_le(a: &Value, b: &Value) -> Option<Value> {
    let Value::Simd {
        lanes,
        element,
        bits: ba,
    } = a
    else {
        return None;
    };
    let Value::Simd {
        lanes: lb,
        element: eb,
        bits: bb,
    } = b
    else {
        return None;
    };
    if lanes != lb || element != eb {
        return None;
    }
    let element = resolve_alias(*element);
    let mut out = [0u8; 64];
    for lane in 0..*lanes {
        let le = if is_float(element) {
            read_float_lane(element, ba, lane)? <= read_float_lane(element, bb, lane)?
        } else if is_signed(element) {
            read_signed_lane(element, ba, lane)? <= read_signed_lane(element, bb, lane)?
        } else {
            read_unsigned_lane(element, ba, lane)? <= read_unsigned_lane(element, bb, lane)?
        };
        let mask = if le { lane_mask(element) } else { 0 };
        write_unsigned_lane(element, &mut out, lane, mask as u128)?;
    }
    Some(Value::Simd {
        lanes: *lanes,
        element,
        bits: out,
    })
}

pub fn simd_ge(a: &Value, b: &Value) -> Option<Value> {
    let Value::Simd {
        lanes,
        element,
        bits: ba,
    } = a
    else {
        return None;
    };
    let Value::Simd {
        lanes: lb,
        element: eb,
        bits: bb,
    } = b
    else {
        return None;
    };
    if lanes != lb || element != eb {
        return None;
    }
    let element = resolve_alias(*element);
    let mut out = [0u8; 64];
    for lane in 0..*lanes {
        let ge = if is_float(element) {
            read_float_lane(element, ba, lane)? >= read_float_lane(element, bb, lane)?
        } else if is_signed(element) {
            read_signed_lane(element, ba, lane)? >= read_signed_lane(element, bb, lane)?
        } else {
            read_unsigned_lane(element, ba, lane)? >= read_unsigned_lane(element, bb, lane)?
        };
        let mask = if ge { lane_mask(element) } else { 0 };
        write_unsigned_lane(element, &mut out, lane, mask as u128)?;
    }
    Some(Value::Simd {
        lanes: *lanes,
        element,
        bits: out,
    })
}

/// Phase 136: unsigned rounding average (`i8x16.avgr_u`: `(a + b + 1) / 2` per lane).
pub fn simd_avgr_u(a: &Value, b: &Value) -> Option<Value> {
    lanewise_ucmp(a, b, |av, bv| av.wrapping_add(bv).wrapping_add(1) / 2)
}

/// Phase 137: lane-wise population count (`i8x16.popcnt`).
pub fn simd_popcnt(v: &Value) -> Option<Value> {
    let Value::Simd {
        lanes,
        element,
        bits,
    } = v
    else {
        return None;
    };
    let element = resolve_alias(*element);
    if is_float(element) {
        return None;
    }
    let mut out = [0u8; 64];
    for lane in 0..*lanes {
        let x = read_unsigned_lane(element, bits, lane)? as u8;
        write_unsigned_lane(element, &mut out, lane, x.count_ones() as u128)?;
    }
    Some(Value::Simd {
        lanes: *lanes,
        element,
        bits: out,
    })
}

/// Phase 138: MSB bitmask (`i8x16.bitmask`) — returns lane MSBs as u16 mask.
pub fn simd_bitmask(v: &Value) -> Option<Value> {
    let Value::Simd {
        lanes,
        element,
        bits,
    } = v
    else {
        return None;
    };
    let element = resolve_alias(*element);
    if is_float(element) {
        return None;
    }
    let lane_count = (*lanes).min(16) as u32;
    let mut mask: u32 = 0;
    for lane in 0..lane_count {
        let x = read_signed_lane(element, bits, lane as u8)? as i8;
        if x < 0 {
            mask |= 1 << lane;
        }
    }
    Some(Value::I32(mask as i32))
}

/// Phase 139: `i8x16.all_true` — true when every lane is non-zero.
pub fn simd_all_true(v: &Value) -> Option<Value> {
    let Value::Simd {
        lanes,
        element,
        bits,
    } = v
    else {
        return None;
    };
    let element = resolve_alias(*element);
    if is_float(element) {
        return None;
    }
    let lane_count = (*lanes).min(16) as u8;
    for lane in 0..lane_count {
        let x = read_unsigned_lane(element, bits, lane)? as u8;
        if x == 0 {
            return Some(Value::Bool(false));
        }
    }
    Some(Value::Bool(true))
}

/// Phase 139: `v128.any_true` on i8x16 — true when any lane is non-zero.
pub fn simd_any_true(v: &Value) -> Option<Value> {
    let Value::Simd {
        lanes,
        element,
        bits,
    } = v
    else {
        return None;
    };
    let element = resolve_alias(*element);
    if is_float(element) {
        return None;
    }
    let lane_count = (*lanes).min(16) as u8;
    for lane in 0..lane_count {
        let x = read_unsigned_lane(element, bits, lane)? as u8;
        if x != 0 {
            return Some(Value::Bool(true));
        }
    }
    Some(Value::Bool(false))
}

pub fn simd_not(v: &Value) -> Option<Value> {
    let Value::Simd {
        lanes,
        element,
        bits,
    } = v
    else {
        return None;
    };
    let mut out = [0u8; 64];
    for i in 0..bits.len() {
        out[i] = !bits[i];
    }
    Some(Value::Simd {
        lanes: *lanes,
        element: *element,
        bits: out,
    })
}

pub fn simd_shl(v: &Value, amount: i32) -> Option<Value> {
    let Value::Simd {
        lanes,
        element,
        bits,
    } = v
    else {
        return None;
    };
    let element = resolve_alias(*element);
    if is_float(element) {
        return None;
    }
    let shift = (amount as u32) & 31;
    let mut out = [0u8; 64];
    for lane in 0..*lanes {
        if is_signed(element) {
            let x = read_signed_lane(element, bits, lane)?;
            let shifted = (x as i32).wrapping_shl(shift) as i128;
            write_signed_lane(element, &mut out, lane, shifted)?;
        } else {
            let x = read_unsigned_lane(element, bits, lane)?;
            let shifted = (x as u32).wrapping_shl(shift) as u128;
            write_unsigned_lane(element, &mut out, lane, shifted)?;
        }
    }
    Some(Value::Simd {
        lanes: *lanes,
        element,
        bits: out,
    })
}

pub fn simd_shr(v: &Value, amount: i32) -> Option<Value> {
    let Value::Simd {
        lanes,
        element,
        bits,
    } = v
    else {
        return None;
    };
    let element = resolve_alias(*element);
    if is_float(element) {
        return None;
    }
    let shift = (amount as u32) & 31;
    let mut out = [0u8; 64];
    for lane in 0..*lanes {
        if is_signed(element) {
            let x = read_signed_lane(element, bits, lane)?;
            let shifted = (x as i32).wrapping_shr(shift) as i128;
            write_signed_lane(element, &mut out, lane, shifted)?;
        } else {
            let x = read_unsigned_lane(element, bits, lane)?;
            let shifted = (x as u32).wrapping_shr(shift) as u128;
            write_unsigned_lane(element, &mut out, lane, shifted)?;
        }
    }
    Some(Value::Simd {
        lanes: *lanes,
        element,
        bits: out,
    })
}

fn simd_bitwise(a: &Value, b: &Value, op: fn(u8, u8) -> u8) -> Option<Value> {
    let Value::Simd {
        lanes,
        element,
        bits: ba,
    } = a
    else {
        return None;
    };
    let Value::Simd {
        lanes: lb,
        element: eb,
        bits: bb,
    } = b
    else {
        return None;
    };
    if lanes != lb || element != eb {
        return None;
    }
    let mut out = [0u8; 64];
    for i in 0..ba.len() {
        out[i] = op(ba[i], bb[i]);
    }
    Some(Value::Simd {
        lanes: *lanes,
        element: *element,
        bits: out,
    })
}

pub fn simd_and(a: &Value, b: &Value) -> Option<Value> {
    simd_bitwise(a, b, |x, y| x & y)
}

pub fn simd_or(a: &Value, b: &Value) -> Option<Value> {
    simd_bitwise(a, b, |x, y| x | y)
}

pub fn simd_xor(a: &Value, b: &Value) -> Option<Value> {
    simd_bitwise(a, b, |x, y| x ^ y)
}

pub fn simd_load_from_array(array: &Value, lanes: u8, element: NumericKind, offset: usize) -> Option<Value> {
    let array = match array {
        Value::HeapSlot { .. } => Value::resolve_heap_slot(array),
        other => other.clone(),
    };
    let Value::Array(items) = array else {
        return None;
    };
    let slice = items.get(offset..offset + lanes as usize)?;
    simd_from_array(lanes, element, slice)
}

pub fn simd_store_to_array(array: &mut Value, offset: usize, simd: &Value) -> Option<()> {
    let simd = match simd {
        Value::HeapSlot { .. } => Value::resolve_heap_slot(simd),
        other => other.clone(),
    };
    let Value::Simd { lanes, element, bits } = &simd else {
        return None;
    };
    // Phase 129: ArrayBuild often yields HeapSlot; match loadFromArray resolve path.
    let array_outer = array.clone();
    let mut resolved = match &array_outer {
        Value::HeapSlot { .. } => Value::resolve_heap_slot(&array_outer),
        other => other.clone(),
    };
    let Value::Array(items) = &mut resolved else {
        return None;
    };
    let element = resolve_alias(*element);
    let end = offset + *lanes as usize;
    if items.len() < end {
        items.resize(end, Value::Nil);
    }
    for lane in 0..*lanes {
        let idx = offset + lane as usize;
        items[idx] = if is_float(element) {
            let f = read_float_lane(element, bits, lane)?;
            if element == NumericKind::F32 {
                Value::F32(f as f32)
            } else {
                Value::F64(f)
            }
        } else if is_signed(element) {
            super::numeric::value_from_numeric(element, read_signed_lane(element, bits, lane)? as u64)
        } else {
            super::numeric::value_from_numeric(element, read_unsigned_lane(element, bits, lane)? as u64)
        };
    }
    *array = if matches!(array_outer, Value::HeapSlot { .. }) {
        Value::write_back_heap_slot(array_outer, resolved)
    } else {
        resolved
    };
    Some(())
}

fn lane_mask(element: NumericKind) -> u64 {
    if is_float(element) {
        if element == NumericKind::F32 {
            (-1i32) as u32 as u64
        } else {
            u64::MAX
        }
    } else {
        let w = bit_width(element);
        if w >= 64 {
            u64::MAX
        } else {
            (1u64 << w) - 1
        }
    }
}

fn lane_bytes(element: NumericKind) -> usize {
    (bit_width(element) / 8) as usize
}

fn scalar_to_u64(element: NumericKind, v: &Value) -> Option<u64> {
    match element {
        NumericKind::I8 => v.as_i64().try_into().ok().map(|n: i8| n as u64),
        NumericKind::I16 => v.as_i64().try_into().ok().map(|n: i16| n as u64),
        NumericKind::I32 => v.as_i64().try_into().ok().map(|n: i32| n as u64),
        NumericKind::I64 => Some(v.as_i64() as u64),
        NumericKind::U8 => v.as_i64().try_into().ok().map(|n: u8| n as u64),
        NumericKind::U16 => v.as_i64().try_into().ok().map(|n: u16| n as u64),
        NumericKind::U32 => v.as_i64().try_into().ok().map(|n: u32| n as u64),
        NumericKind::U64 => Some(v.as_i64() as u64),
        NumericKind::F32 => match v {
            Value::F32(n) => Some(n.to_bits() as u64),
            _ => Some((v.as_i64() as f32).to_bits() as u64),
        },
        NumericKind::F64 => match v {
            Value::F64(n) => Some(n.to_bits()),
            _ => Some((v.as_i64() as f64).to_bits()),
        },
        _ => None,
    }
}

fn read_unsigned_lane(element: NumericKind, bits: &[u8; 64], lane: u8) -> Option<u128> {
    let lb = lane_bytes(element);
    let off = lane as usize * lb;
    let mut raw = 0u128;
    for b in 0..lb {
        raw |= (bits[off + b] as u128) << (b * 8);
    }
    Some(raw)
}

fn read_signed_lane(element: NumericKind, bits: &[u8; 64], lane: u8) -> Option<i64> {
    let w = bit_width(element);
    let raw = read_unsigned_lane(element, bits, lane)? as i128;
    let sign = 1i128 << (w - 1);
    let masked = if raw & sign != 0 {
        raw | (!0i128 << w)
    } else {
        raw
    };
    Some(masked as i64)
}

fn read_float_lane(element: NumericKind, bits: &[u8; 64], lane: u8) -> Option<f64> {
    let raw = scalar_to_u64(element, &super::numeric::value_from_numeric(element, read_unsigned_lane(element, bits, lane)? as u64))?;
    match element {
        NumericKind::F32 => Some(f32::from_bits(raw as u32) as f64),
        NumericKind::F64 => Some(f64::from_bits(raw)),
        _ => None,
    }
}

fn write_unsigned_lane(element: NumericKind, bits: &mut [u8; 64], lane: u8, value: u128) -> Option<()> {
    let lb = lane_bytes(element);
    let off = lane as usize * lb;
    for b in 0..lb {
        bits[off + b] = ((value >> (b * 8)) & 0xFF) as u8;
    }
    Some(())
}

fn write_signed_lane(element: NumericKind, bits: &mut [u8; 64], lane: u8, value: i128) -> Option<()> {
    write_unsigned_lane(element, bits, lane, value as u128)
}

fn write_float_lane(element: NumericKind, bits: &mut [u8; 64], lane: u8, value: f64) -> Option<()> {
    let raw = if element == NumericKind::F32 {
        (value as f32).to_bits() as u64
    } else {
        value.to_bits()
    };
    write_unsigned_lane(element, bits, lane, raw as u128)
}

/// Phase 125: extract one signed lane from a SIMD vector as platform Int.
pub fn simd_extract_lane(v: &Value, lane: i32) -> Option<Value> {
    let Value::Simd { lanes, element, bits } = v else {
        return None;
    };
    let lane = lane as u8;
    if lane >= *lanes {
        return None;
    }
    let element = resolve_alias(*element);
    if is_signed(element) {
        let n = read_signed_lane(element, bits, lane)?;
        return Some(super::numeric::platform_int_value(n));
    }
    let n = read_unsigned_lane(element, bits, lane)?;
    Some(super::numeric::platform_int_value(n as i64))
}

/// Phase 125: replace one lane in a SIMD vector (scalar widened/truncated to lane width).
pub fn simd_replace_lane(v: &Value, lane: i32, scalar: &Value) -> Option<Value> {
    let Value::Simd { lanes, element, bits } = v else {
        return None;
    };
    let lane = lane as u8;
    if lane >= *lanes {
        return None;
    }
    let element = resolve_alias(*element);
    let mut bits = *bits;
    if is_float(element) {
        let f = scalar.as_i64() as f64;
        write_float_lane(element, &mut bits, lane, f)?;
    } else if is_signed(element) {
        write_signed_lane(element, &mut bits, lane, scalar.as_i64() as i128)?;
    } else {
        let raw = scalar_to_u64(element, scalar)?;
        write_unsigned_lane(element, &mut bits, lane, raw as u128)?;
    }
    Some(Value::Simd {
        lanes: *lanes,
        element,
        bits,
    })
}