//! Phase 45: per-type numeric HostFn registry (`__syx_i8_min`, …).

use crate::collection_hosts::lookup_collection_host;
use crate::numeric::NumericKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericHostOp {
    Min,
    Max,
    Abs,
    Add,
    AddAssign,
    SubAssign,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Compare,
}

#[derive(Debug, Clone, Copy)]
pub struct NumericHostEntry {
    pub name: &'static str,
    pub kind: NumericKind,
    pub op: NumericHostOp,
    pub return_surface: &'static str,
}

const fn nh(
    name: &'static str,
    kind: NumericKind,
    op: NumericHostOp,
    return_surface: &'static str,
) -> NumericHostEntry {
    NumericHostEntry {
        name,
        kind,
        op,
        return_surface,
    }
}

/// Host slug for manifest surface (canonical types only; aliases delegate in Core).
pub fn host_slug(surface: &str) -> Option<&'static str> {
    Some(match surface {
        "Int8" => "i8",
        "Int16" => "i16",
        "Int32" => "i32",
        "Int64" => "i64",
        "Int128" => "i128",
        "UInt8" => "u8",
        "UInt16" => "u16",
        "UInt32" => "u32",
        "UInt64" => "u64",
        "UInt128" => "u128",
        "Int" => "int",
        "UInt" => "uint",
        "Float32" => "f32",
        "Float64" => "f64",
        _ => return None,
    })
}

pub static NUMERIC_HOST_TABLE: &[NumericHostEntry] = &[
    nh("__syx_i8_min", NumericKind::I8, NumericHostOp::Min, "Int8"),
    nh("__syx_i8_max", NumericKind::I8, NumericHostOp::Max, "Int8"),
    nh("__syx_i8_abs", NumericKind::I8, NumericHostOp::Abs, "Int8"),
    nh("__syx_i16_min", NumericKind::I16, NumericHostOp::Min, "Int16"),
    nh("__syx_i16_max", NumericKind::I16, NumericHostOp::Max, "Int16"),
    nh("__syx_i16_abs", NumericKind::I16, NumericHostOp::Abs, "Int16"),
    nh("__syx_i32_min", NumericKind::I32, NumericHostOp::Min, "Int32"),
    nh("__syx_i32_max", NumericKind::I32, NumericHostOp::Max, "Int32"),
    nh("__syx_i32_abs", NumericKind::I32, NumericHostOp::Abs, "Int32"),
    nh("__syx_i64_min", NumericKind::I64, NumericHostOp::Min, "Int64"),
    nh("__syx_i64_max", NumericKind::I64, NumericHostOp::Max, "Int64"),
    nh("__syx_i64_abs", NumericKind::I64, NumericHostOp::Abs, "Int64"),
    nh("__syx_i128_min", NumericKind::I128, NumericHostOp::Min, "Int128"),
    nh("__syx_i128_max", NumericKind::I128, NumericHostOp::Max, "Int128"),
    nh("__syx_i128_abs", NumericKind::I128, NumericHostOp::Abs, "Int128"),
    nh("__syx_int_min", NumericKind::Int, NumericHostOp::Min, "Int"),
    nh("__syx_int_max", NumericKind::Int, NumericHostOp::Max, "Int"),
    nh("__syx_int_abs", NumericKind::Int, NumericHostOp::Abs, "Int"),
    nh("__syx_int_add", NumericKind::Int, NumericHostOp::Add, "Int"),
    nh("__syx_int_add_assign", NumericKind::Int, NumericHostOp::AddAssign, "Int"),
    nh("__syx_int_sub_assign", NumericKind::Int, NumericHostOp::SubAssign, "Int"),
    nh("__syx_int_eq", NumericKind::Int, NumericHostOp::Eq, "Bool"),
    nh("__syx_int_ne", NumericKind::Int, NumericHostOp::Ne, "Bool"),
    nh("__syx_int_lt", NumericKind::Int, NumericHostOp::Lt, "Bool"),
    nh("__syx_int_le", NumericKind::Int, NumericHostOp::Le, "Bool"),
    nh("__syx_int_gt", NumericKind::Int, NumericHostOp::Gt, "Bool"),
    nh("__syx_int_ge", NumericKind::Int, NumericHostOp::Ge, "Bool"),
    nh("__syx_int_compare", NumericKind::Int, NumericHostOp::Compare, "Int"),
    nh("__syx_i8_compare", NumericKind::I8, NumericHostOp::Compare, "Int"),
    nh("__syx_i16_compare", NumericKind::I16, NumericHostOp::Compare, "Int"),
    nh("__syx_i32_compare", NumericKind::I32, NumericHostOp::Compare, "Int"),
    nh("__syx_i64_compare", NumericKind::I64, NumericHostOp::Compare, "Int"),
    nh("__syx_i128_compare", NumericKind::I128, NumericHostOp::Compare, "Int"),
    nh("__syx_u8_compare", NumericKind::U8, NumericHostOp::Compare, "Int"),
    nh("__syx_u16_compare", NumericKind::U16, NumericHostOp::Compare, "Int"),
    nh("__syx_u32_compare", NumericKind::U32, NumericHostOp::Compare, "Int"),
    nh("__syx_u64_compare", NumericKind::U64, NumericHostOp::Compare, "Int"),
    nh("__syx_u128_compare", NumericKind::U128, NumericHostOp::Compare, "Int"),
    nh("__syx_uint_compare", NumericKind::UInt, NumericHostOp::Compare, "Int"),
    nh("__syx_f32_compare", NumericKind::F32, NumericHostOp::Compare, "Int"),
    nh("__syx_f64_compare", NumericKind::F64, NumericHostOp::Compare, "Int"),
    nh("__syx_u8_min", NumericKind::U8, NumericHostOp::Min, "UInt8"),
    nh("__syx_u8_max", NumericKind::U8, NumericHostOp::Max, "UInt8"),
    nh("__syx_u16_min", NumericKind::U16, NumericHostOp::Min, "UInt16"),
    nh("__syx_u16_max", NumericKind::U16, NumericHostOp::Max, "UInt16"),
    nh("__syx_u32_min", NumericKind::U32, NumericHostOp::Min, "UInt32"),
    nh("__syx_u32_max", NumericKind::U32, NumericHostOp::Max, "UInt32"),
    nh("__syx_u64_min", NumericKind::U64, NumericHostOp::Min, "UInt64"),
    nh("__syx_u64_max", NumericKind::U64, NumericHostOp::Max, "UInt64"),
    nh("__syx_u128_min", NumericKind::U128, NumericHostOp::Min, "UInt128"),
    nh("__syx_u128_max", NumericKind::U128, NumericHostOp::Max, "UInt128"),
    nh("__syx_uint_min", NumericKind::UInt, NumericHostOp::Min, "UInt"),
    nh("__syx_uint_max", NumericKind::UInt, NumericHostOp::Max, "UInt"),
    nh("__syx_f32_min", NumericKind::F32, NumericHostOp::Min, "Float32"),
    nh("__syx_f32_max", NumericKind::F32, NumericHostOp::Max, "Float32"),
    nh("__syx_f64_min", NumericKind::F64, NumericHostOp::Min, "Float64"),
    nh("__syx_f64_max", NumericKind::F64, NumericHostOp::Max, "Float64"),
];

pub fn lookup_numeric_host(name: &str) -> Option<&'static NumericHostEntry> {
    NUMERIC_HOST_TABLE.iter().find(|e| e.name == name)
}

pub fn numeric_host_names() -> impl Iterator<Item = &'static str> {
    NUMERIC_HOST_TABLE.iter().map(|e| e.name)
}

pub fn host_return_type_by_name(name: &str) -> Option<&'static str> {
    if let Some(e) = lookup_numeric_host(name) {
        return Some(e.return_surface);
    }
    match name {
        "__syx_value_description" | "__syx_value_debug_description" => Some("String"),
        "__syx_value_compare" => Some("Int"),
        "__syx_bigint_from_string" | "__syx_bigint_add" | "__syx_bigint_sub" | "__syx_bigint_mul" => {
            Some("BigInt")
        }
        "__syx_bigint_cmp" => Some("Int"),
        "__syx_bigint_description" => Some("String"),
        _ => lookup_collection_host(name).map(|e| e.return_surface),
    }
}
