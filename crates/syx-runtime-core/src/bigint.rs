//! Phase 46/48: BigInt MVP — i128 fast path + decimal limb strings beyond i128.

use std::collections::HashMap;

use crate::value::Value;

pub const BIGINT_TYPE: &str = "BigInt";

pub fn bigint_from_i128(v: i128) -> Value {
    Value::Object(HashMap::from([
        ("__type".into(), Value::Str(BIGINT_TYPE.into())),
        ("value".into(), Value::Str(v.to_string().into())),
    ]))
}

fn bigint_wide(s: &str) -> Value {
    Value::Object(HashMap::from([
        ("__type".into(), Value::Str(BIGINT_TYPE.into())),
        ("value".into(), Value::Str(s.into())),
        ("wide".into(), Value::Bool(true)),
    ]))
}

fn is_decimal_int(s: &str) -> bool {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return false;
    }
    let body = trimmed.strip_prefix('-').unwrap_or(trimmed);
    !body.is_empty() && body.chars().all(|c| c.is_ascii_digit())
}

pub fn bigint_from_string(s: &str) -> Option<Value> {
    let trimmed = s.trim();
    if !is_decimal_int(trimmed) {
        return None;
    }
    if let Ok(v) = trimmed.parse::<i128>() {
        return Some(bigint_from_i128(v));
    }
    Some(bigint_wide(trimmed))
}

fn decimal_digits(v: &Value) -> Option<String> {
    match v {
        Value::Object(map) | Value::Dict(map) => map.get("value").and_then(|inner| match inner {
            Value::Str(s) => Some(s.to_string()),
            Value::I64(n) => Some(n.to_string()),
            Value::I32(n) => Some(n.to_string()),
            Value::I128(n) => Some(n.to_string()),
            _ => None,
        }),
        Value::I64(n) => Some(n.to_string()),
        Value::I32(n) => Some(n.to_string()),
        Value::I128(n) => Some(n.to_string()),
        Value::Str(s) => Some(s.to_string()),
        _ => None,
    }
}

fn parse_i128(v: &Value) -> Option<i128> {
    decimal_digits(v)?.parse().ok()
}

fn normalize_decimal(s: &str) -> (bool, String) {
    let neg = s.starts_with('-');
    let mag = s.strip_prefix('-').unwrap_or(s).trim_start_matches('0');
    let mag = if mag.is_empty() {
        "0".to_string()
    } else {
        mag.to_string()
    };
    (neg, mag)
}

fn cmp_decimal_strings(a: &str, b: &str) -> i32 {
    let (neg_a, ma) = normalize_decimal(a);
    let (neg_b, mb) = normalize_decimal(b);
    match (neg_a, neg_b) {
        (true, false) => -1,
        (false, true) => 1,
        (true, true) => cmp_decimal_strings(&mb, &ma),
        (false, false) => {
            if ma.len() != mb.len() {
                ma.len().cmp(&mb.len()) as i32
            } else {
                ma.cmp(&mb) as i32
            }
        }
    }
}

fn add_decimal_magnitude(a: &str, b: &str) -> String {
    let a = a.trim_start_matches('0');
    let a = if a.is_empty() { "0" } else { a };
    let b = b.trim_start_matches('0');
    let b = if b.is_empty() { "0" } else { b };
    let mut out = Vec::new();
    let mut carry = 0u32;
    let mut i = a.len();
    let mut j = b.len();
    while i > 0 || j > 0 || carry > 0 {
        let da = if i > 0 {
            i -= 1;
            a.as_bytes()[i] - b'0'
        } else {
            0
        };
        let db = if j > 0 {
            j -= 1;
            b.as_bytes()[j] - b'0'
        } else {
            0
        };
        let sum = da as u32 + db as u32 + carry;
        out.push((sum % 10) as u8 + b'0');
        carry = sum / 10;
    }
    out.reverse();
    String::from_utf8(out).unwrap_or_else(|_| "0".into())
}

fn sub_decimal_magnitude(a: &str, b: &str) -> Option<String> {
    if a.len() < b.len() || (a.len() == b.len() && a < b) {
        return None;
    }
    let mut out = Vec::new();
    let mut borrow = 0i32;
    let mut i = a.len();
    let mut j = b.len();
    while i > 0 {
        i -= 1;
        let mut da = a.as_bytes()[i] as i32 - borrow;
        let db = if j > 0 {
            j -= 1;
            b.as_bytes()[j] as i32
        } else {
            0
        };
        if da < db {
            da += 10;
            borrow = 1;
        } else {
            borrow = 0;
        }
        out.push((da - db) as u8 + b'0');
    }
    while out.last() == Some(&b'0') && out.len() > 1 {
        out.pop();
    }
    out.reverse();
    Some(String::from_utf8(out).unwrap_or_else(|_| "0".into()))
}

fn add_decimal_strings(a: &str, b: &str) -> String {
    let neg_a = a.starts_with('-');
    let neg_b = b.starts_with('-');
    let ma = a.strip_prefix('-').unwrap_or(a);
    let mb = b.strip_prefix('-').unwrap_or(b);
    match (neg_a, neg_b) {
        (false, false) => add_decimal_magnitude(ma, mb),
        (true, true) => format!("-{}", add_decimal_magnitude(ma, mb)),
        (true, false) => {
            if ma.len() > mb.len() || (ma.len() == mb.len() && ma > mb) {
                format!("-{}", sub_decimal_magnitude(ma, mb).unwrap_or_else(|| "0".into()))
            } else if ma == mb {
                "0".into()
            } else {
                sub_decimal_magnitude(mb, ma).unwrap_or_else(|| "0".into())
            }
        }
        (false, true) => {
            if ma.len() > mb.len() || (ma.len() == mb.len() && ma > mb) {
                sub_decimal_magnitude(ma, mb).unwrap_or_else(|| "0".into())
            } else if ma == mb {
                "0".into()
            } else {
                format!("-{}", sub_decimal_magnitude(mb, ma).unwrap_or_else(|| "0".into()))
            }
        }
    }
}

fn mul_decimal_magnitude(a: &str, b: &str) -> String {
    if a == "0" || b == "0" {
        return "0".into();
    }
    let mut acc = vec![0u32; a.len() + b.len()];
    let ab = a.as_bytes();
    let bb = b.as_bytes();
    for (i, &da) in ab.iter().rev().enumerate() {
        let da = (da - b'0') as u32;
        for (j, &db) in bb.iter().rev().enumerate() {
            let db = (db - b'0') as u32;
            let pos = i + j;
            acc[pos] += da * db;
        }
    }
    for i in 0..acc.len() - 1 {
        acc[i + 1] += acc[i] / 10;
        acc[i] %= 10;
    }
    while acc.len() > 1 && *acc.last().unwrap() == 0 {
        acc.pop();
    }
    acc.into_iter()
        .rev()
        .map(|d| (d as u8 + b'0') as char)
        .collect()
}

fn mul_decimal_strings(a: &str, b: &str) -> String {
    let neg = a.starts_with('-') ^ b.starts_with('-');
    let ma = a.strip_prefix('-').unwrap_or(a);
    let mb = b.strip_prefix('-').unwrap_or(b);
    let mag = mul_decimal_magnitude(ma, mb);
    if neg && mag != "0" {
        format!("-{mag}")
    } else {
        mag
    }
}

pub fn bigint_add(a: &Value, b: &Value) -> Option<Value> {
    let as_str = decimal_digits(a)?;
    let bs_str = decimal_digits(b)?;
    if let (Ok(av), Ok(bv)) = (as_str.parse::<i128>(), bs_str.parse::<i128>()) {
        if let Some(sum) = av.checked_add(bv) {
            return Some(bigint_from_i128(sum));
        }
    }
    let sum = add_decimal_strings(&as_str, &bs_str);
    if let Ok(v) = sum.parse::<i128>() {
        Some(bigint_from_i128(v))
    } else {
        Some(bigint_wide(&sum))
    }
}

pub fn bigint_sub(a: &Value, b: &Value) -> Option<Value> {
    let as_str = decimal_digits(a)?;
    let bs_str = decimal_digits(b)?;
    if let (Ok(av), Ok(bv)) = (as_str.parse::<i128>(), bs_str.parse::<i128>()) {
        if let Some(diff) = av.checked_sub(bv) {
            return Some(bigint_from_i128(diff));
        }
    }
    let neg_b = if bs_str.starts_with('-') {
        bs_str.strip_prefix('-').unwrap_or(&bs_str).to_string()
    } else {
        format!("-{bs_str}")
    };
    bigint_add(a, &bigint_from_string(&neg_b)?)
}

pub fn bigint_mul(a: &Value, b: &Value) -> Option<Value> {
    let as_str = decimal_digits(a)?;
    let bs_str = decimal_digits(b)?;
    if let (Ok(av), Ok(bv)) = (as_str.parse::<i128>(), bs_str.parse::<i128>()) {
        if let Some(prod) = av.checked_mul(bv) {
            return Some(bigint_from_i128(prod));
        }
    }
    let prod = mul_decimal_strings(&as_str, &bs_str);
    if let Ok(v) = prod.parse::<i128>() {
        Some(bigint_from_i128(v))
    } else {
        Some(bigint_wide(&prod))
    }
}

pub fn bigint_cmp(a: &Value, b: &Value) -> Option<i32> {
    let as_str = decimal_digits(a)?;
    let bs_str = decimal_digits(b)?;
    if let (Ok(av), Ok(bv)) = (as_str.parse::<i128>(), bs_str.parse::<i128>()) {
        return Some(av.cmp(&bv) as i32);
    }
    Some(cmp_decimal_strings(&as_str, &bs_str))
}

pub fn bigint_description(v: &Value) -> String {
    decimal_digits(v)
        .map(|n| format!("BigInt({n})"))
        .unwrap_or_else(|| "BigInt(invalid)".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wide_add_smoke() {
        let a = bigint_from_string("999999999999999999999999999999").unwrap();
        let b = bigint_from_i128(1);
        let sum = bigint_add(&a, &b).unwrap();
        assert!(bigint_description(&sum).contains("1000000000000000000000000000000"));
    }

    #[test]
    fn wide_from_string_smoke() {
        let v = bigint_from_string("99999999999999999999999999999999999999999").unwrap();
        assert!(bigint_description(&v).contains("99999999999999999999999999999999999999999"));
    }
}
