/// Fixed-width and platform numeric kinds — single source of truth for the compiler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NumericKind {
    I8 = 0,
    I16 = 1,
    I32 = 2,
    I64 = 3,
    U8 = 4,
    U16 = 5,
    U32 = 6,
    U64 = 7,
    Int = 8,
    UInt = 9,
    F32 = 10,
    F64 = 11,
    I128 = 12,
    U128 = 13,
    /// Alias of `F32`. Diagnostics keep the name the programmer wrote.
    Float = 14,
    /// Alias of `F64`. Diagnostics keep the name the programmer wrote.
    Double = 15,
}

#[cfg(target_pointer_width = "32")]
pub const PLATFORM_INT: NumericKind = NumericKind::I32;
#[cfg(target_pointer_width = "64")]
pub const PLATFORM_INT: NumericKind = NumericKind::I64;

#[cfg(target_pointer_width = "32")]
pub const PLATFORM_UINT: NumericKind = NumericKind::U32;
#[cfg(target_pointer_width = "64")]
pub const PLATFORM_UINT: NumericKind = NumericKind::U64;

/// Resolve `Int` / `UInt` and `Float` / `Double` to their concrete kinds.
pub fn resolve_alias(kind: NumericKind) -> NumericKind {
    match kind {
        NumericKind::Int => PLATFORM_INT,
        NumericKind::UInt => PLATFORM_UINT,
        NumericKind::Float => NumericKind::F32,
        NumericKind::Double => NumericKind::F64,
        other => other,
    }
}

/// Map a SYX type name to a numeric kind, if it is a numeric builtin.
pub fn from_type_name(name: &str) -> Option<NumericKind> {
    match name {
        "Int8" => Some(NumericKind::I8),
        "Int16" => Some(NumericKind::I16),
        "Int32" => Some(NumericKind::I32),
        "Int64" => Some(NumericKind::I64),
        "Int128" => Some(NumericKind::I128),
        "UInt8" => Some(NumericKind::U8),
        "UInt16" => Some(NumericKind::U16),
        "UInt32" => Some(NumericKind::U32),
        "UInt64" => Some(NumericKind::U64),
        "UInt128" => Some(NumericKind::U128),
        "Int" => Some(NumericKind::Int),
        "UInt" => Some(NumericKind::UInt),
        "Float32" => Some(NumericKind::F32),
        "Float" => Some(NumericKind::Float),
        "Float64" => Some(NumericKind::F64),
        "Double" => Some(NumericKind::Double),
        _ => None,
    }
}

/// Canonical SYX display name for a resolved numeric kind.
pub fn type_name(kind: NumericKind) -> &'static str {
    match resolve_alias(kind) {
        NumericKind::I8 => "Int8",
        NumericKind::I16 => "Int16",
        NumericKind::I32 => "Int32",
        NumericKind::I64 => "Int64",
        NumericKind::I128 => "Int128",
        NumericKind::U8 => "UInt8",
        NumericKind::U16 => "UInt16",
        NumericKind::U32 => "UInt32",
        NumericKind::U64 => "UInt64",
        NumericKind::U128 => "UInt128",
        NumericKind::F32 | NumericKind::Float => "Float32",
        NumericKind::F64 | NumericKind::Double => "Float64",
        NumericKind::Int | NumericKind::UInt => "Int", // unreachable after resolve
    }
}

/// Display name preserving alias names (`Int`, `UInt`, `Float`, `Double`).
pub fn type_name_with_alias(kind: NumericKind) -> &'static str {
    match kind {
        NumericKind::Int => "Int",
        NumericKind::UInt => "UInt",
        NumericKind::Float => "Float",
        NumericKind::Double => "Double",
        NumericKind::F32 => "Float32",
        NumericKind::F64 => "Float64",
        other => type_name(other),
    }
}

pub fn is_signed(kind: NumericKind) -> bool {
    matches!(
        resolve_alias(kind),
        NumericKind::I8
            | NumericKind::I16
            | NumericKind::I32
            | NumericKind::I64
            | NumericKind::I128
    )
}

pub fn is_unsigned(kind: NumericKind) -> bool {
    matches!(
        resolve_alias(kind),
        NumericKind::U8 | NumericKind::U16 | NumericKind::U32 | NumericKind::U64 | NumericKind::U128
    )
}

pub fn is_float(kind: NumericKind) -> bool {
    matches!(resolve_alias(kind), NumericKind::F32 | NumericKind::F64)
}

pub fn is_integer(kind: NumericKind) -> bool {
    is_signed(kind) || is_unsigned(kind)
}

/// Bit width of a resolved numeric kind.
pub fn bit_width(kind: NumericKind) -> u8 {
    match resolve_alias(kind) {
        NumericKind::I8 | NumericKind::U8 => 8,
        NumericKind::I16 | NumericKind::U16 => 16,
        NumericKind::I32 | NumericKind::U32 => 32,
        NumericKind::I64 | NumericKind::U64 => 64,
        NumericKind::I128 | NumericKind::U128 => 128,
        NumericKind::F32 | NumericKind::Float => 32,
        NumericKind::F64 | NumericKind::Double => 64,
        NumericKind::Int | NumericKind::UInt => unreachable!("alias must be resolved"),
    }
}

/// Inclusive min/max for integer kinds (used for literal range checks).
pub fn int_range(kind: NumericKind) -> Option<(i128, i128)> {
    if is_float(kind) {
        return None;
    }
    let w = bit_width(kind);
    if w == 128 {
        if is_unsigned(kind) {
            return Some((0, i128::MAX));
        }
        return Some((i128::MIN, i128::MAX));
    }
    if is_unsigned(kind) {
        Some((0, (1i128 << w) - 1))
    } else {
        let min = -(1i128 << (w - 1));
        let max = (1i128 << (w - 1)) - 1;
        Some((min, max))
    }
}

/// Default integer literal type (platform `Int` alias — not the resolved width).
///
/// Keep the alias so diagnostics/hover say `Int` rather than `Int64`/`Int32`.
/// Callers that need a concrete width must `resolve_alias`.
pub fn default_integer_kind() -> NumericKind {
    NumericKind::Int
}

/// Default floating literal type.
pub fn default_float_kind() -> NumericKind {
    NumericKind::F64
}

/// Encode an AST `f64` literal value as `ConstNumeric` bits for the target float kind.
pub fn float_literal_bits(kind: NumericKind, value: f64) -> u64 {
    match resolve_alias(kind) {
        NumericKind::F32 => (value as f32).to_bits() as u64,
        NumericKind::F64 => value.to_bits(),
        other => panic!("float_literal_bits called with non-float kind: {other:?}"),
    }
}

/// Result kind for binary numeric promotion (Swift-like widening).
pub fn widen_for_binary(a: NumericKind, b: NumericKind) -> Option<NumericKind> {
    let a = resolve_alias(a);
    let b = resolve_alias(b);
    if is_float(a) || is_float(b) {
        return Some(NumericKind::F64);
    }
    if is_signed(a) && is_unsigned(b) || is_unsigned(a) && is_signed(b) {
        return None; // requires explicit cast
    }
    let wa = bit_width(a);
    let wb = bit_width(b);
    if wa >= wb {
        Some(a)
    } else {
        Some(b)
    }
}

/// Whether `from` can implicitly convert to `to` (widening only).
pub fn is_implicit_convertible(from: NumericKind, to: NumericKind) -> bool {
    let from = resolve_alias(from);
    let to = resolve_alias(to);
    if from == to {
        return true;
    }
    if is_float(from) && is_float(to) {
        return bit_width(from) <= bit_width(to);
    }
    if is_integer(from) && is_float(to) {
        return true;
    }
    if is_integer(from) && is_integer(to) {
        if is_unsigned(from) && is_signed(to) {
            return bit_width(from) < bit_width(to);
        }
        if is_signed(from) && is_unsigned(to) {
            return false;
        }
        return bit_width(from) <= bit_width(to);
    }
    false
}

/// Same-signedness integer narrowing (`Int` → `Int8`, `UInt16` → `UInt8`).
/// Float↔int and signed↔unsigned mixes are never narrowing here.
pub fn is_integer_narrowing(from: NumericKind, to: NumericKind) -> bool {
    let from = resolve_alias(from);
    let to = resolve_alias(to);
    if !is_integer(from) || !is_integer(to) || from == to {
        return false;
    }
    if is_signed(from) && is_signed(to) {
        return bit_width(from) > bit_width(to);
    }
    if is_unsigned(from) && is_unsigned(to) {
        return bit_width(from) > bit_width(to);
    }
    false
}

/// Parse a literal suffix string (e.g. `"i8"`, `"u32"`, `"f64"`).
pub fn from_suffix(suffix: &str) -> Option<NumericKind> {
    match suffix {
        "i8" => Some(NumericKind::I8),
        "i16" => Some(NumericKind::I16),
        "i32" => Some(NumericKind::I32),
        "i64" => Some(NumericKind::I64),
        "i128" => Some(NumericKind::I128),
        "u8" => Some(NumericKind::U8),
        "u16" => Some(NumericKind::U16),
        "u32" => Some(NumericKind::U32),
        "u64" => Some(NumericKind::U64),
        "u128" => Some(NumericKind::U128),
        "f32" | "float" => Some(NumericKind::F32),
        "f64" | "double" => Some(NumericKind::F64),
        _ => None,
    }
}

impl NumericKind {
    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
            0 => Some(NumericKind::I8),
            1 => Some(NumericKind::I16),
            2 => Some(NumericKind::I32),
            3 => Some(NumericKind::I64),
            4 => Some(NumericKind::U8),
            5 => Some(NumericKind::U16),
            6 => Some(NumericKind::U32),
            7 => Some(NumericKind::U64),
            8 => Some(NumericKind::Int),
            9 => Some(NumericKind::UInt),
            10 => Some(NumericKind::F32),
            11 => Some(NumericKind::F64),
            12 => Some(NumericKind::I128),
            13 => Some(NumericKind::U128),
            14 => Some(NumericKind::Float),
            15 => Some(NumericKind::Double),
            _ => None,
        }
    }

    pub fn to_byte(self) -> u8 {
        self as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f32_literal_bits_from_ast_f64() {
        let bits = float_literal_bits(NumericKind::F32, 3.4);
        assert_eq!(f32::from_bits(bits as u32), 3.4f32);
    }

    #[test]
    fn platform_int_is_64_on_64bit() {
        #[cfg(target_pointer_width = "64")]
        assert_eq!(resolve_alias(NumericKind::Int), NumericKind::I64);
        #[cfg(target_pointer_width = "32")]
        assert_eq!(resolve_alias(NumericKind::Int), NumericKind::I32);
    }

    #[test]
    fn default_integer_literal_keeps_int_alias() {
        assert_eq!(default_integer_kind(), NumericKind::Int);
        assert_eq!(type_name_with_alias(default_integer_kind()), "Int");
    }

    #[test]
    fn float_aliases_keep_source_names() {
        assert_eq!(from_type_name("Double"), Some(NumericKind::Double));
        assert_eq!(from_type_name("Float"), Some(NumericKind::Float));
        assert_eq!(from_type_name("Float64"), Some(NumericKind::F64));
        assert_eq!(from_type_name("Float32"), Some(NumericKind::F32));
        assert_eq!(type_name_with_alias(NumericKind::Double), "Double");
        assert_eq!(type_name_with_alias(NumericKind::Float), "Float");
        assert_eq!(resolve_alias(NumericKind::Double), NumericKind::F64);
        assert_eq!(resolve_alias(NumericKind::Float), NumericKind::F32);
        assert!(is_implicit_convertible(NumericKind::Double, NumericKind::F64));
        assert!(is_implicit_convertible(NumericKind::F64, NumericKind::Double));
    }

    #[test]
    fn widen_signed_same_family() {
        assert_eq!(
            widen_for_binary(NumericKind::I8, NumericKind::I32),
            Some(NumericKind::I32)
        );
    }

    #[test]
    fn mixed_signed_unsigned_requires_cast() {
        assert_eq!(widen_for_binary(NumericKind::I32, NumericKind::U32), None);
    }
}
