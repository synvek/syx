//! Phase 47: Result algebraic HostFn registry.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultHostOp {
    Ok,
    Err,
    IsOk,
    IsErr,
}

#[derive(Debug, Clone, Copy)]
pub struct ResultHostEntry {
    pub name: &'static str,
    pub op: ResultHostOp,
    pub return_surface: &'static str,
}

const fn rh(name: &'static str, op: ResultHostOp, ret: &'static str) -> ResultHostEntry {
    ResultHostEntry {
        name,
        op,
        return_surface: ret,
    }
}

pub static RESULT_HOST_TABLE: &[ResultHostEntry] = &[
    rh("__syx_result_ok", ResultHostOp::Ok, "Result"),
    rh("__syx_result_err", ResultHostOp::Err, "Result"),
    rh("__syx_result_is_ok", ResultHostOp::IsOk, "Bool"),
    rh("__syx_result_is_err", ResultHostOp::IsErr, "Bool"),
];

pub fn lookup_result_host(name: &str) -> Option<&'static ResultHostEntry> {
    RESULT_HOST_TABLE.iter().find(|e| e.name == name)
}

pub fn result_host_names() -> impl Iterator<Item = &'static str> {
    RESULT_HOST_TABLE.iter().map(|e| e.name)
}
