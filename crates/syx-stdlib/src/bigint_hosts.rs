//! Phase 46: BigInt HostFn registry.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BigIntHostOp {
    FromString,
    FromI128,
    Add,
    Sub,
    Mul,
    Cmp,
    Description,
}

#[derive(Debug, Clone, Copy)]
pub struct BigIntHostEntry {
    pub name: &'static str,
    pub op: BigIntHostOp,
    pub return_surface: &'static str,
}

const fn bh(name: &'static str, op: BigIntHostOp, ret: &'static str) -> BigIntHostEntry {
    BigIntHostEntry {
        name,
        op,
        return_surface: ret,
    }
}

pub static BIGINT_HOST_TABLE: &[BigIntHostEntry] = &[
    bh("__syx_bigint_from_string", BigIntHostOp::FromString, "BigInt"),
    bh("__syx_bigint_from_i128", BigIntHostOp::FromI128, "BigInt"),
    bh("__syx_bigint_add", BigIntHostOp::Add, "BigInt"),
    bh("__syx_bigint_sub", BigIntHostOp::Sub, "BigInt"),
    bh("__syx_bigint_mul", BigIntHostOp::Mul, "BigInt"),
    bh("__syx_bigint_cmp", BigIntHostOp::Cmp, "Int"),
    bh("__syx_bigint_description", BigIntHostOp::Description, "String"),
];

pub fn lookup_bigint_host(name: &str) -> Option<&'static BigIntHostEntry> {
    BIGINT_HOST_TABLE.iter().find(|e| e.name == name)
}

pub fn bigint_host_names() -> impl Iterator<Item = &'static str> {
    BIGINT_HOST_TABLE.iter().map(|e| e.name)
}
