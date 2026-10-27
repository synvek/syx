//! ABI v2/v2m f64 little-endian pack helpers (W1045 / ADR-1390; W1046 / ADR-1391).
//! W1051 / ADR-1396: host_ops borrow/pin via alloc trailing fields.
//! W1055 / ADR-1400: host_ops drop via alloc trailing drop_fn.
//! ≠ faer / ≠ handle-table GA.

use std::ffi::c_void;

use syx_runtime_core::Value;

use crate::abi::{SyxPluginBuf, SYX_PLUGIN_BUF_F64LE};
use crate::handles::{drop_handle, get_buf, pin_f64_le_bytes, HostBuf};

/// Call-local host_ops context. `borrows` keeps cloned buffers alive for the call.
pub struct HostOpsCtx {
    pub out: Vec<u8>,
    pub borrows: Vec<Box<HostBuf>>,
}

pub fn value_as_f64(v: &Value) -> Result<f64, String> {
    match v {
        Value::F64(x) => Ok(*x),
        Value::F32(x) => Ok(f64::from(*x)),
        Value::I64(x) => Ok(*x as f64),
        Value::I32(x) => Ok(f64::from(*x)),
        Value::I16(x) => Ok(f64::from(*x)),
        Value::I8(x) => Ok(f64::from(*x)),
        Value::U32(x) => Ok(f64::from(*x)),
        Value::U16(x) => Ok(f64::from(*x)),
        Value::U8(x) => Ok(f64::from(*x)),
        other => Err(format!("H004: v2 f64 slot expected numeric, got {other:?}")),
    }
}

pub fn pack_f64_le(values: &[Value]) -> Result<Vec<u8>, String> {
    let mut out = Vec::with_capacity(values.len() * 8);
    for v in values {
        out.extend_from_slice(&value_as_f64(v)?.to_le_bytes());
    }
    Ok(out)
}

pub fn unpack_f64_le(bytes: &[u8]) -> Result<Vec<Value>, String> {
    if bytes.len() % 8 != 0 {
        return Err(format!(
            "H004: v2 f64 out len {} is not a multiple of 8",
            bytes.len()
        ));
    }
    Ok(bytes
        .chunks_exact(8)
        .map(|c| {
            let mut n = [0u8; 8];
            n.copy_from_slice(c);
            Value::F64(f64::from_le_bytes(n))
        })
        .collect())
}

pub fn sum_f64_array(values: &[Value]) -> Result<f64, String> {
    let mut acc = 0.0;
    for v in values {
        acc += value_as_f64(v)?;
    }
    Ok(acc)
}

pub fn dot_f64_arrays(a: &[Value], b: &[Value]) -> Result<f64, String> {
    if a.len() != b.len() {
        return Err("H004: v2probe_dot length mismatch".into());
    }
    let mut acc = 0.0;
    for (x, y) in a.iter().zip(b.iter()) {
        acc += value_as_f64(x)? * value_as_f64(y)?;
    }
    Ok(acc)
}

/// Growable out: start at `initial_cap`, realloc (resize) if packed bytes exceed it.
pub fn grow_copy_f64(values: &[Value], initial_cap: usize) -> Result<Vec<u8>, String> {
    let packed = pack_f64_le(values)?;
    let mut buf = vec![0u8; initial_cap];
    if packed.len() > buf.len() {
        buf.resize(packed.len(), 0);
    }
    if packed.len() > buf.len() {
        return Err("H004: v2 grow realloc failed".into());
    }
    buf[..packed.len()].copy_from_slice(&packed);
    buf.truncate(packed.len());
    Ok(buf)
}

pub unsafe extern "C" fn host_realloc(ctx: *mut c_void, _ptr: *mut u8, new_cap: usize) -> *mut u8 {
    if ctx.is_null() {
        return std::ptr::null_mut();
    }
    let ops = &mut *(ctx as *mut HostOpsCtx);
    ops.out.resize(new_cap, 0);
    ops.out.as_mut_ptr()
}

/// Copy-out borrow: clone HostBuf so the table lock is not held across the plugin call.
pub unsafe extern "C" fn host_borrow(
    ctx: *mut c_void,
    id: u64,
    buf: *mut SyxPluginBuf,
    nrows: *mut usize,
    ncols: *mut usize,
) -> i32 {
    if ctx.is_null() || buf.is_null() || nrows.is_null() || ncols.is_null() {
        return 1;
    }
    let Ok(cloned) = get_buf(id) else {
        return 1;
    };
    let ops = &mut *(ctx as *mut HostOpsCtx);
    ops.borrows.push(Box::new(cloned));
    let owned = ops.borrows.last().expect("just pushed");
    (*buf).kind = SYX_PLUGIN_BUF_F64LE;
    (*buf).ptr = owned.data.as_ptr() as *mut u8;
    (*buf).len = owned.data.len() * 8;
    *nrows = owned.nrows;
    *ncols = owned.ncols;
    0
}

pub unsafe extern "C" fn host_pin(
    _ctx: *mut c_void,
    nrows: usize,
    ncols: usize,
    ptr: *const u8,
    len: usize,
    out_id: *mut u64,
) -> i32 {
    if ptr.is_null() || out_id.is_null() {
        return 1;
    }
    let bytes = std::slice::from_raw_parts(ptr, len);
    let Ok(id) = pin_f64_le_bytes(nrows, ncols, bytes) else {
        return 1;
    };
    *out_id = id as u64;
    0
}

/// W1055 / ADR-1400: drop host handle. Unknown / double drop → 1.
pub unsafe extern "C" fn host_drop(_ctx: *mut c_void, id: u64) -> i32 {
    if drop_handle(id).is_err() {
        1
    } else {
        0
    }
}
