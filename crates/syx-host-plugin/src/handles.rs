//! Host-side opaque f64-buffer handles (W1047 / ADR-1392; W1048 shape; W1049 solve).
//! ≠ handle GC GA.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

use syx_runtime_core::Value;

use crate::v2::value_as_f64;

#[derive(Clone, Debug, PartialEq)]
pub struct HostBuf {
    pub nrows: usize,
    pub ncols: usize,
    pub data: Vec<f64>,
}

static NEXT_HANDLE: AtomicU64 = AtomicU64::new(1);
static TABLE: OnceLock<Mutex<HashMap<u64, HostBuf>>> = OnceLock::new();

fn table() -> &'static Mutex<HashMap<u64, HostBuf>> {
    TABLE.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn value_as_handle(v: &Value) -> Result<u64, String> {
    match v {
        Value::I64(x) if *x > 0 => Ok(*x as u64),
        Value::I32(x) if *x > 0 => Ok(*x as u64),
        Value::I16(x) if *x > 0 => Ok(*x as u64),
        Value::I8(x) if *x > 0 => Ok(*x as u64),
        Value::U64(x) if *x > 0 => Ok(*x),
        Value::U32(x) if *x > 0 => Ok(u64::from(*x)),
        Value::F64(_) | Value::F32(_) => Err("H004: handle must be integer id, not f64".into()),
        other => Err(format!("H004: expected handle I64, got {other:?}")),
    }
}

pub fn value_as_dim(v: &Value) -> Result<usize, String> {
    let n = match v {
        Value::I64(x) if *x > 0 => *x as usize,
        Value::I32(x) if *x > 0 => *x as usize,
        Value::I16(x) if *x > 0 => *x as usize,
        Value::U64(x) if *x > 0 => *x as usize,
        Value::U32(x) if *x > 0 => *x as usize,
        other => return Err(format!("H004: expected positive dim, got {other:?}")),
    };
    Ok(n)
}

pub fn pack_handle(id: u64) -> [u8; 8] {
    id.to_le_bytes()
}

pub fn unpack_handle(bytes: &[u8]) -> Result<u64, String> {
    let id = unpack_handle_bits(bytes)?;
    if id == 0 {
        return Err("H004: HANDLE id 0 is reserved".into());
    }
    Ok(id)
}

/// W1055 / ADR-1400: v2m HANDLE *out* may be 0 (hdrop success → `Int 0`).
/// Input packing still uses [`unpack_handle`] (id 0 reserved).
pub fn unpack_handle_out(bytes: &[u8]) -> Result<u64, String> {
    unpack_handle_bits(bytes)
}

fn unpack_handle_bits(bytes: &[u8]) -> Result<u64, String> {
    if bytes.len() != 8 {
        return Err(format!("H004: HANDLE payload len {} != 8", bytes.len()));
    }
    let mut n = [0u8; 8];
    n.copy_from_slice(bytes);
    Ok(u64::from_le_bytes(n))
}

fn insert_buf(buf: HostBuf) -> Result<i64, String> {
    if buf.nrows == 0 || buf.ncols == 0 {
        return Err("H004: matrix dims must be > 0".into());
    }
    if buf.data.len() != buf.nrows.saturating_mul(buf.ncols) {
        return Err(format!(
            "H004: data len {} != {}×{}",
            buf.data.len(),
            buf.nrows,
            buf.ncols
        ));
    }
    let id = NEXT_HANDLE.fetch_add(1, Ordering::Relaxed);
    table()
        .lock()
        .map_err(|_| "H004: handle table lock poisoned".to_string())?
        .insert(id, buf);
    Ok(id as i64)
}

pub fn pin_f64_values(values: &[Value]) -> Result<i64, String> {
    let mut data = Vec::with_capacity(values.len());
    for v in values {
        data.push(value_as_f64(v)?);
    }
    insert_buf(HostBuf {
        nrows: 1,
        ncols: data.len(),
        data,
    })
}

pub fn pin_shaped(values: &[Value], nrows: usize, ncols: usize) -> Result<i64, String> {
    let mut data = Vec::with_capacity(values.len());
    for v in values {
        data.push(value_as_f64(v)?);
    }
    insert_buf(HostBuf { nrows, ncols, data })
}

pub fn pin_zeros(nrows: usize, ncols: usize) -> Result<i64, String> {
    let n = nrows.saturating_mul(ncols);
    insert_buf(HostBuf {
        nrows,
        ncols,
        data: vec![0.0; n],
    })
}

pub fn pin_f64_le_bytes(nrows: usize, ncols: usize, bytes: &[u8]) -> Result<i64, String> {
    if bytes.len() % 8 != 0 {
        return Err(format!(
            "H004: pin f64-le len {} is not a multiple of 8",
            bytes.len()
        ));
    }
    let mut data = Vec::with_capacity(bytes.len() / 8);
    for c in bytes.chunks_exact(8) {
        let mut n = [0u8; 8];
        n.copy_from_slice(c);
        data.push(f64::from_le_bytes(n));
    }
    insert_buf(HostBuf { nrows, ncols, data })
}

pub fn get_buf(id: u64) -> Result<HostBuf, String> {
    let g = table()
        .lock()
        .map_err(|_| "H004: handle table lock poisoned".to_string())?;
    g.get(&id)
        .cloned()
        .ok_or_else(|| format!("H004: unknown handle {id}"))
}

pub fn sum_handle(id: u64) -> Result<f64, String> {
    Ok(get_buf(id)?.data.iter().copied().sum())
}

pub fn buf_to_values(id: u64) -> Result<Vec<Value>, String> {
    Ok(get_buf(id)?.data.into_iter().map(Value::F64).collect())
}

pub fn drop_handle(id: u64) -> Result<(), String> {
    let mut g = table()
        .lock()
        .map_err(|_| "H004: handle table lock poisoned".to_string())?;
    g.remove(&id)
        .ok_or_else(|| format!("H004: unknown handle {id}"))?;
    Ok(())
}

#[cfg(feature = "p-faer")]
pub fn mul_handles(left: u64, right: u64) -> Result<i64, String> {
    let a = get_buf(left)?;
    let b = get_buf(right)?;
    if a.ncols != b.nrows {
        return Err(format!(
            "H004: faer mul shape {}x{} vs {}x{}",
            a.nrows, a.ncols, b.nrows, b.ncols
        ));
    }
    let am = faer::Mat::<f64>::from_fn(a.nrows, a.ncols, |r, c| a.data[r * a.ncols + c]);
    let bm = faer::Mat::<f64>::from_fn(b.nrows, b.ncols, |r, c| b.data[r * b.ncols + c]);
    let cm = &am * &bm;
    let mut data = Vec::with_capacity(cm.nrows() * cm.ncols());
    for r in 0..cm.nrows() {
        for c in 0..cm.ncols() {
            data.push(cm[(r, c)]);
        }
    }
    insert_buf(HostBuf {
        nrows: cm.nrows(),
        ncols: cm.ncols(),
        data,
    })
}

#[cfg(not(feature = "p-faer"))]
pub fn mul_handles(_left: u64, _right: u64) -> Result<i64, String> {
    Err("H004: faer mul requires syx-host-plugin feature p-faer".into())
}

#[cfg(feature = "p-faer")]
pub fn solve_handles(left: u64, right: u64) -> Result<i64, String> {
    use faer::linalg::solvers::Solve;
    let a = get_buf(left)?;
    let b = get_buf(right)?;
    if a.nrows != a.ncols {
        return Err(format!(
            "H004: faer solve requires square A, got {}x{}",
            a.nrows, a.ncols
        ));
    }
    if b.nrows != a.nrows {
        return Err(format!(
            "H004: faer solve rhs {}x{} vs A {}x{}",
            b.nrows, b.ncols, a.nrows, a.ncols
        ));
    }
    let am = faer::Mat::<f64>::from_fn(a.nrows, a.ncols, |r, c| a.data[r * a.ncols + c]);
    let bm = faer::Mat::<f64>::from_fn(b.nrows, b.ncols, |r, c| b.data[r * b.ncols + c]);
    let lu = am.partial_piv_lu();
    let u = lu.U();
    let n = a.nrows;
    let mut max_abs = 0.0_f64;
    for i in 0..n {
        max_abs = max_abs.max(u[(i, i)].abs());
    }
    let tol = (1e-12_f64).max(1e-14 * max_abs);
    for i in 0..n {
        if u[(i, i)].abs() < tol {
            return Err(format!("H004: faer solve singular at pivot {i}"));
        }
    }
    let xm = lu.solve(&bm);
    let mut data = Vec::with_capacity(xm.nrows() * xm.ncols());
    for r in 0..xm.nrows() {
        for c in 0..xm.ncols() {
            data.push(xm[(r, c)]);
        }
    }
    insert_buf(HostBuf {
        nrows: xm.nrows(),
        ncols: xm.ncols(),
        data,
    })
}

#[cfg(not(feature = "p-faer"))]
pub fn solve_handles(_left: u64, _right: u64) -> Result<i64, String> {
    Err("H004: faer solve requires syx-host-plugin feature p-faer".into())
}

#[cfg(feature = "p-faer")]
fn binop_handles(
    left: u64,
    right: u64,
    op_name: &str,
    op: impl Fn(f64, f64) -> f64,
) -> Result<i64, String> {
    let a = get_buf(left)?;
    let b = get_buf(right)?;
    if a.nrows != b.nrows || a.ncols != b.ncols {
        return Err(format!(
            "H004: faer {op_name} shape {}x{} vs {}x{}",
            a.nrows, a.ncols, b.nrows, b.ncols
        ));
    }
    let am = faer::Mat::<f64>::from_fn(a.nrows, a.ncols, |r, c| a.data[r * a.ncols + c]);
    let bm = faer::Mat::<f64>::from_fn(b.nrows, b.ncols, |r, c| b.data[r * b.ncols + c]);
    let mut data = Vec::with_capacity(a.nrows * a.ncols);
    for r in 0..a.nrows {
        for c in 0..a.ncols {
            data.push(op(am[(r, c)], bm[(r, c)]));
        }
    }
    insert_buf(HostBuf {
        nrows: a.nrows,
        ncols: a.ncols,
        data,
    })
}

#[cfg(feature = "p-faer")]
pub fn add_handles(left: u64, right: u64) -> Result<i64, String> {
    binop_handles(left, right, "add", |x, y| x + y)
}

#[cfg(not(feature = "p-faer"))]
pub fn add_handles(_left: u64, _right: u64) -> Result<i64, String> {
    Err("H004: faer add requires syx-host-plugin feature p-faer".into())
}

#[cfg(feature = "p-faer")]
pub fn sub_handles(left: u64, right: u64) -> Result<i64, String> {
    binop_handles(left, right, "sub", |x, y| x - y)
}

#[cfg(not(feature = "p-faer"))]
pub fn sub_handles(_left: u64, _right: u64) -> Result<i64, String> {
    Err("H004: faer sub requires syx-host-plugin feature p-faer".into())
}

#[cfg(feature = "p-faer")]
pub fn hadamard_handles(left: u64, right: u64) -> Result<i64, String> {
    binop_handles(left, right, "hadamard", |x, y| x * y)
}

#[cfg(not(feature = "p-faer"))]
pub fn hadamard_handles(_left: u64, _right: u64) -> Result<i64, String> {
    Err("H004: faer hadamard requires syx-host-plugin feature p-faer".into())
}
