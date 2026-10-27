//! SYX faerlin cdylib (W1050 F64LE mul; W1051 HANDLE mul; W1052 HANDLE solve;
//! W1053 / ADR-1398 HANDLE from/to; W1055 / ADR-1400 HANDLE drop;
//! W1056 / ADR-1401 F64LE packed solve; W1059 / ADR-1405 F64LE packed zeros;
//! W1104 / ADR-1450 F64LE packed add/sub;
//! W1105 / ADR-1451 HANDLE×cdylib add/sub;
//! W1109 / ADR-1455 F64LE packed hadamard;
//! W1110 / ADR-1456 HANDLE×cdylib hadamard).
//!
//! Real `dlopen` + `syx_plugin_init`.
//! v2 `__syx_plugin_faer_cdylib_mul`: packed F64LE
//!   [nrows_a, ncols_a, nrows_b, ncols_b, A_row_major..., B_row_major...]
//! v2 `__syx_plugin_faer_cdylib_solve`: same packing; square A + rhs; faer LU.
//! v2 `__syx_plugin_faer_cdylib_add` / `_sub` / `_hadamard`: same packing; same-shape element-wise.
//! v2m `__syx_plugin_faer_cdylib_hmul`: two HANDLE slots; host_ops borrow/pin.
//! v2m `__syx_plugin_faer_cdylib_hadd` / `_hsub` / `_hhadamard`: two HANDLE slots; same-shape element-wise.
//! v2m `__syx_plugin_faer_cdylib_hsolve`: square A + rhs b; faer partial_piv_lu.
//! v2m `__syx_plugin_faer_cdylib_hfrom`: packed [nrows, ncols, row_major…] → pin.
//! v2m `__syx_plugin_faer_cdylib_hto`: HANDLE → packed [nrows, ncols, row_major…].
//! v2m `__syx_plugin_faer_cdylib_hdrop`: HANDLE → host drop_fn (unknown/double drop fail).
//! v2 `__syx_plugin_faer_cdylib_zeros`: packed F64LE [nrows, ncols] → row-major zeros.
//! Shape / singular / unknown handle / bad packing → nonzero. ≠ Mat · ≠ faer product GA.

use std::ffi::c_void;
use std::os::raw::c_char;
use std::slice;
use std::sync::OnceLock;

const SYX_PLUGIN_API_VERSION_V2: u32 = 2;
const SYX_PLUGIN_API_VERSION_V2M: u32 = 3;
const SYX_PLUGIN_BUF_F64LE: u32 = 1;
const SYX_PLUGIN_BUF_HANDLE: u32 = 2;

type SyxHostFnV2 = unsafe extern "C" fn(*const SyxPluginBuf, *mut SyxPluginBuf, usize) -> i32;
type SyxHostFnV2m =
    unsafe extern "C" fn(*const SyxPluginIov, *mut SyxPluginIov, *mut SyxPluginAlloc) -> i32;
type SyxPluginReallocFn = unsafe extern "C" fn(*mut c_void, *mut u8, usize) -> *mut u8;
type SyxPluginBorrowFn =
    unsafe extern "C" fn(*mut c_void, u64, *mut SyxPluginBuf, *mut usize, *mut usize) -> i32;
type SyxPluginPinFn =
    unsafe extern "C" fn(*mut c_void, usize, usize, *const u8, usize, *mut u64) -> i32;
type SyxPluginDropFn = unsafe extern "C" fn(*mut c_void, u64) -> i32;

#[repr(C)]
struct SyxPluginBuf {
    kind: u32,
    ptr: *mut u8,
    len: usize,
}

#[repr(C)]
struct SyxPluginIov {
    count: u32,
    bufs: *mut SyxPluginBuf,
}

#[repr(C)]
struct SyxPluginAlloc {
    ctx: *mut c_void,
    realloc_fn: Option<SyxPluginReallocFn>,
    borrow_fn: Option<SyxPluginBorrowFn>,
    pin_fn: Option<SyxPluginPinFn>,
    drop_fn: Option<SyxPluginDropFn>,
}

#[repr(C)]
struct SyxHostFnDescV2 {
    name: *const c_char,
    return_type: *const c_char,
    arity: u32,
    func: SyxHostFnV2,
}

unsafe impl Sync for SyxHostFnDescV2 {}
unsafe impl Send for SyxHostFnDescV2 {}

#[repr(C)]
struct SyxHostFnDescV2m {
    name: *const c_char,
    return_type: *const c_char,
    arity: u32,
    func: SyxHostFnV2m,
}

unsafe impl Sync for SyxHostFnDescV2m {}
unsafe impl Send for SyxHostFnDescV2m {}

#[repr(C)]
struct SyxPluginInfoV2m {
    api_version: u32,
    plugin_id: *const c_char,
    host_count: u32,
    hosts: *const u8,
    v2_host_count: u32,
    v2_hosts: *const SyxHostFnDescV2,
    v2m_host_count: u32,
    v2m_hosts: *const SyxHostFnDescV2m,
}

unsafe impl Sync for SyxPluginInfoV2m {}
unsafe impl Send for SyxPluginInfoV2m {}

fn read_f64_le(bytes: &[u8], off: usize) -> Option<f64> {
    let chunk = bytes.get(off..off + 8)?;
    Some(f64::from_le_bytes(chunk.try_into().ok()?))
}

fn write_f64_le(dest: &mut [u8], off: usize, v: f64) -> bool {
    let Some(slot) = dest.get_mut(off..off + 8) else {
        return false;
    };
    slot.copy_from_slice(&v.to_le_bytes());
    true
}

fn read_u64_le(bytes: &[u8]) -> Option<u64> {
    if bytes.len() != 8 {
        return None;
    }
    let mut n = [0u8; 8];
    n.copy_from_slice(bytes);
    Some(u64::from_le_bytes(n))
}

fn f64_as_dim(v: f64) -> Option<usize> {
    if !v.is_finite() || v < 1.0 {
        return None;
    }
    let n = v as usize;
    if n as f64 != v {
        return None;
    }
    Some(n)
}

fn mat_from_row_major(nrows: usize, ncols: usize, bytes: &[u8]) -> Option<faer::Mat<f64>> {
    if bytes.len() != nrows.checked_mul(ncols)? * 8 {
        return None;
    }
    Some(faer::Mat::<f64>::from_fn(nrows, ncols, |r, c| {
        read_f64_le(bytes, (r * ncols + c) * 8).unwrap_or(0.0)
    }))
}

/// Packed F64LE mul via faer. Returns 0 on success, nonzero on error (no silent zero).
unsafe extern "C" fn faer_cdylib_mul(
    in_buf: *const SyxPluginBuf,
    out_buf: *mut SyxPluginBuf,
    out_cap: usize,
) -> i32 {
    if in_buf.is_null() || out_buf.is_null() {
        return 1;
    }
    let inn = &*in_buf;
    let out = &mut *out_buf;
    if out.ptr.is_null() || inn.ptr.is_null() {
        return 1;
    }
    if inn.kind != SYX_PLUGIN_BUF_F64LE || (inn.len % 8) != 0 {
        return 1;
    }
    let bytes = slice::from_raw_parts(inn.ptr, inn.len);
    let n = inn.len / 8;
    if n < 4 {
        return 1;
    }
    let nrows_a = read_f64_le(bytes, 0).unwrap_or(0.0) as usize;
    let ncols_a = read_f64_le(bytes, 8).unwrap_or(0.0) as usize;
    let nrows_b = read_f64_le(bytes, 16).unwrap_or(0.0) as usize;
    let ncols_b = read_f64_le(bytes, 24).unwrap_or(0.0) as usize;
    if nrows_a == 0 || ncols_a == 0 || nrows_b == 0 || ncols_b == 0 {
        return 1;
    }
    if ncols_a != nrows_b {
        return 1;
    }
    let a_len = nrows_a.checked_mul(ncols_a).unwrap_or(0);
    let b_len = nrows_b.checked_mul(ncols_b).unwrap_or(0);
    if a_len == 0 || b_len == 0 || 4 + a_len + b_len != n {
        return 1;
    }
    let mut a_data = Vec::with_capacity(a_len);
    let mut b_data = Vec::with_capacity(b_len);
    for i in 0..a_len {
        let Some(v) = read_f64_le(bytes, (4 + i) * 8) else {
            return 1;
        };
        a_data.push(v);
    }
    for i in 0..b_len {
        let Some(v) = read_f64_le(bytes, (4 + a_len + i) * 8) else {
            return 1;
        };
        b_data.push(v);
    }
    let am = faer::Mat::<f64>::from_fn(nrows_a, ncols_a, |r, c| a_data[r * ncols_a + c]);
    let bm = faer::Mat::<f64>::from_fn(nrows_b, ncols_b, |r, c| b_data[r * ncols_b + c]);
    let cm = &am * &bm;
    let out_n = cm.nrows() * cm.ncols();
    let need = out_n * 8;
    if need > out_cap {
        return 1;
    }
    let dest = slice::from_raw_parts_mut(out.ptr, out_cap);
    let mut off = 0usize;
    for r in 0..cm.nrows() {
        for c in 0..cm.ncols() {
            if !write_f64_le(dest, off, cm[(r, c)]) {
                return 1;
            }
            off += 8;
        }
    }
    out.kind = SYX_PLUGIN_BUF_F64LE;
    out.len = need;
    0
}

/// Packed F64LE solve via faer LU (W1056 / ADR-1401). Same packing as mul.
unsafe extern "C" fn faer_cdylib_solve(
    in_buf: *const SyxPluginBuf,
    out_buf: *mut SyxPluginBuf,
    out_cap: usize,
) -> i32 {
    use faer::linalg::solvers::Solve;
    if in_buf.is_null() || out_buf.is_null() {
        return 1;
    }
    let inn = &*in_buf;
    let out = &mut *out_buf;
    if out.ptr.is_null() || inn.ptr.is_null() {
        return 1;
    }
    if inn.kind != SYX_PLUGIN_BUF_F64LE || (inn.len % 8) != 0 {
        return 1;
    }
    let bytes = slice::from_raw_parts(inn.ptr, inn.len);
    let n = inn.len / 8;
    if n < 4 {
        return 1;
    }
    let nrows_a = read_f64_le(bytes, 0).unwrap_or(0.0) as usize;
    let ncols_a = read_f64_le(bytes, 8).unwrap_or(0.0) as usize;
    let nrows_b = read_f64_le(bytes, 16).unwrap_or(0.0) as usize;
    let ncols_b = read_f64_le(bytes, 24).unwrap_or(0.0) as usize;
    if nrows_a == 0 || ncols_a == 0 || nrows_b == 0 || ncols_b == 0 {
        return 1;
    }
    if nrows_a != ncols_a || nrows_b != nrows_a {
        return 1;
    }
    let a_len = nrows_a.checked_mul(ncols_a).unwrap_or(0);
    let b_len = nrows_b.checked_mul(ncols_b).unwrap_or(0);
    if a_len == 0 || b_len == 0 || 4 + a_len + b_len != n {
        return 1;
    }
    let mut a_data = Vec::with_capacity(a_len);
    let mut b_data = Vec::with_capacity(b_len);
    for i in 0..a_len {
        let Some(v) = read_f64_le(bytes, (4 + i) * 8) else {
            return 1;
        };
        a_data.push(v);
    }
    for i in 0..b_len {
        let Some(v) = read_f64_le(bytes, (4 + a_len + i) * 8) else {
            return 1;
        };
        b_data.push(v);
    }
    let am = faer::Mat::<f64>::from_fn(nrows_a, ncols_a, |r, c| a_data[r * ncols_a + c]);
    let bm = faer::Mat::<f64>::from_fn(nrows_b, ncols_b, |r, c| b_data[r * ncols_b + c]);
    let lu = am.partial_piv_lu();
    let u = lu.U();
    let mut max_abs = 0.0_f64;
    for i in 0..nrows_a {
        max_abs = max_abs.max(u[(i, i)].abs());
    }
    let tol = (1e-12_f64).max(1e-14 * max_abs);
    for i in 0..nrows_a {
        if u[(i, i)].abs() < tol {
            return 1;
        }
    }
    let xm = lu.solve(&bm);
    let out_n = xm.nrows() * xm.ncols();
    let need = out_n * 8;
    if need > out_cap {
        return 1;
    }
    let dest = slice::from_raw_parts_mut(out.ptr, out_cap);
    let mut off = 0usize;
    for r in 0..xm.nrows() {
        for c in 0..xm.ncols() {
            if !write_f64_le(dest, off, xm[(r, c)]) {
                return 1;
            }
            off += 8;
        }
    }
    out.kind = SYX_PLUGIN_BUF_F64LE;
    out.len = need;
    0
}

/// Packed F64LE same-shape element-wise add/sub (W1104 / ADR-1450).
/// Packing matches mul: [nrows_a, ncols_a, nrows_b, ncols_b, A..., B...];
/// requires nrows_a==nrows_b && ncols_a==ncols_b.
unsafe fn faer_cdylib_binop(
    in_buf: *const SyxPluginBuf,
    out_buf: *mut SyxPluginBuf,
    out_cap: usize,
    op: impl Fn(f64, f64) -> f64,
) -> i32 {
    if in_buf.is_null() || out_buf.is_null() {
        return 1;
    }
    let inn = &*in_buf;
    let out = &mut *out_buf;
    if out.ptr.is_null() || inn.ptr.is_null() {
        return 1;
    }
    if inn.kind != SYX_PLUGIN_BUF_F64LE || (inn.len % 8) != 0 {
        return 1;
    }
    let bytes = slice::from_raw_parts(inn.ptr, inn.len);
    let n = inn.len / 8;
    if n < 4 {
        return 1;
    }
    let nrows_a = read_f64_le(bytes, 0).unwrap_or(0.0) as usize;
    let ncols_a = read_f64_le(bytes, 8).unwrap_or(0.0) as usize;
    let nrows_b = read_f64_le(bytes, 16).unwrap_or(0.0) as usize;
    let ncols_b = read_f64_le(bytes, 24).unwrap_or(0.0) as usize;
    if nrows_a == 0 || ncols_a == 0 || nrows_b == 0 || ncols_b == 0 {
        return 1;
    }
    if nrows_a != nrows_b || ncols_a != ncols_b {
        return 1;
    }
    let a_len = nrows_a.checked_mul(ncols_a).unwrap_or(0);
    let b_len = nrows_b.checked_mul(ncols_b).unwrap_or(0);
    if a_len == 0 || b_len == 0 || 4 + a_len + b_len != n {
        return 1;
    }
    let mut a_data = Vec::with_capacity(a_len);
    let mut b_data = Vec::with_capacity(b_len);
    for i in 0..a_len {
        let Some(v) = read_f64_le(bytes, (4 + i) * 8) else {
            return 1;
        };
        a_data.push(v);
    }
    for i in 0..b_len {
        let Some(v) = read_f64_le(bytes, (4 + a_len + i) * 8) else {
            return 1;
        };
        b_data.push(v);
    }
    let need = a_len * 8;
    if need > out_cap {
        return 1;
    }
    let dest = slice::from_raw_parts_mut(out.ptr, out_cap);
    let mut off = 0usize;
    for i in 0..a_len {
        if !write_f64_le(dest, off, op(a_data[i], b_data[i])) {
            return 1;
        }
        off += 8;
    }
    out.kind = SYX_PLUGIN_BUF_F64LE;
    out.len = need;
    0
}

unsafe extern "C" fn faer_cdylib_add(
    in_buf: *const SyxPluginBuf,
    out_buf: *mut SyxPluginBuf,
    out_cap: usize,
) -> i32 {
    faer_cdylib_binop(in_buf, out_buf, out_cap, |x, y| x + y)
}

unsafe extern "C" fn faer_cdylib_sub(
    in_buf: *const SyxPluginBuf,
    out_buf: *mut SyxPluginBuf,
    out_cap: usize,
) -> i32 {
    faer_cdylib_binop(in_buf, out_buf, out_cap, |x, y| x - y)
}

unsafe extern "C" fn faer_cdylib_hadamard(
    in_buf: *const SyxPluginBuf,
    out_buf: *mut SyxPluginBuf,
    out_cap: usize,
) -> i32 {
    faer_cdylib_binop(in_buf, out_buf, out_cap, |x, y| x * y)
}

/// Packed F64LE zeros (W1059 / ADR-1405). In: [nrows, ncols]; out: row-major zeros.
unsafe extern "C" fn faer_cdylib_zeros(
    in_buf: *const SyxPluginBuf,
    out_buf: *mut SyxPluginBuf,
    out_cap: usize,
) -> i32 {
    if in_buf.is_null() || out_buf.is_null() {
        return 1;
    }
    let inn = &*in_buf;
    let out = &mut *out_buf;
    if out.ptr.is_null() || inn.ptr.is_null() {
        return 1;
    }
    if inn.kind != SYX_PLUGIN_BUF_F64LE || (inn.len % 8) != 0 {
        return 1;
    }
    let bytes = slice::from_raw_parts(inn.ptr, inn.len);
    if inn.len < 16 {
        return 1;
    }
    let Some(nrows) = f64_as_dim(read_f64_le(bytes, 0).unwrap_or(0.0)) else {
        return 1;
    };
    let Some(ncols) = f64_as_dim(read_f64_le(bytes, 8).unwrap_or(0.0)) else {
        return 1;
    };
    let cells = match nrows.checked_mul(ncols) {
        Some(c) if c > 0 => c,
        _ => return 1,
    };
    let need = cells * 8;
    if need > out_cap {
        return 1;
    }
    let dest = slice::from_raw_parts_mut(out.ptr, out_cap);
    let mut off = 0usize;
    for _ in 0..cells {
        if !write_f64_le(dest, off, 0.0) {
            return 1;
        }
        off += 8;
    }
    out.kind = SYX_PLUGIN_BUF_F64LE;
    out.len = need;
    0
}

unsafe fn faer_cdylib_hbinop(
    in_iov: *const SyxPluginIov,
    out_iov: *mut SyxPluginIov,
    alloc: *mut SyxPluginAlloc,
    op: fn(f64, f64) -> f64,
) -> i32 {
    if in_iov.is_null() || out_iov.is_null() || alloc.is_null() {
        return 1;
    }
    let inn = &*in_iov;
    let out = &mut *out_iov;
    let ops = &*alloc;
    let Some(borrow) = ops.borrow_fn else {
        return 1;
    };
    if inn.count != 2 || inn.bufs.is_null() || out.count == 0 || out.bufs.is_null() {
        return 1;
    }
    let slots = slice::from_raw_parts(inn.bufs, 2);
    if slots[0].kind != SYX_PLUGIN_BUF_HANDLE || slots[1].kind != SYX_PLUGIN_BUF_HANDLE {
        return 1;
    }
    let a_id = read_u64_le(slice::from_raw_parts(slots[0].ptr, slots[0].len)).unwrap_or(0);
    let b_id = read_u64_le(slice::from_raw_parts(slots[1].ptr, slots[1].len)).unwrap_or(0);
    if a_id == 0 || b_id == 0 {
        return 1;
    }
    let mut abuf = SyxPluginBuf {
        kind: 0,
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    let mut bbuf = SyxPluginBuf {
        kind: 0,
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    let mut nrows_a = 0usize;
    let mut ncols_a = 0usize;
    let mut nrows_b = 0usize;
    let mut ncols_b = 0usize;
    if borrow(ops.ctx, a_id, &mut abuf, &mut nrows_a, &mut ncols_a) != 0 {
        return 1;
    }
    if borrow(ops.ctx, b_id, &mut bbuf, &mut nrows_b, &mut ncols_b) != 0 {
        return 1;
    }
    if nrows_a == 0 || ncols_a == 0 || nrows_a != nrows_b || ncols_a != ncols_b {
        return 1;
    }
    let a_bytes = slice::from_raw_parts(abuf.ptr, abuf.len);
    let b_bytes = slice::from_raw_parts(bbuf.ptr, bbuf.len);
    let cells = nrows_a.checked_mul(ncols_a).unwrap_or(0);
    if cells == 0 || a_bytes.len() != cells * 8 || b_bytes.len() != cells * 8 {
        return 1;
    }
    let mut packed = vec![0u8; cells * 8];
    let mut off = 0usize;
    for i in 0..cells {
        let Some(av) = read_f64_le(a_bytes, i * 8) else {
            return 1;
        };
        let Some(bv) = read_f64_le(b_bytes, i * 8) else {
            return 1;
        };
        if !write_f64_le(&mut packed, off, op(av, bv)) {
            return 1;
        }
        off += 8;
    }
    pin_handle_out(ops, out, nrows_a, ncols_a, &packed)
}

unsafe extern "C" fn faer_cdylib_hadd(
    in_iov: *const SyxPluginIov,
    out_iov: *mut SyxPluginIov,
    alloc: *mut SyxPluginAlloc,
) -> i32 {
    faer_cdylib_hbinop(in_iov, out_iov, alloc, |x, y| x + y)
}

unsafe extern "C" fn faer_cdylib_hsub(
    in_iov: *const SyxPluginIov,
    out_iov: *mut SyxPluginIov,
    alloc: *mut SyxPluginAlloc,
) -> i32 {
    faer_cdylib_hbinop(in_iov, out_iov, alloc, |x, y| x - y)
}

unsafe extern "C" fn faer_cdylib_hhadamard(
    in_iov: *const SyxPluginIov,
    out_iov: *mut SyxPluginIov,
    alloc: *mut SyxPluginAlloc,
) -> i32 {
    faer_cdylib_hbinop(in_iov, out_iov, alloc, |x, y| x * y)
}

unsafe extern "C" fn faer_cdylib_hmul(
    in_iov: *const SyxPluginIov,
    out_iov: *mut SyxPluginIov,
    alloc: *mut SyxPluginAlloc,
) -> i32 {
    if in_iov.is_null() || out_iov.is_null() || alloc.is_null() {
        return 1;
    }
    let inn = &*in_iov;
    let out = &mut *out_iov;
    let ops = &*alloc;
    let Some(borrow) = ops.borrow_fn else {
        return 1;
    };
    if inn.count != 2 || inn.bufs.is_null() || out.count == 0 || out.bufs.is_null() {
        return 1;
    }
    let slots = slice::from_raw_parts(inn.bufs, 2);
    if slots[0].kind != SYX_PLUGIN_BUF_HANDLE || slots[1].kind != SYX_PLUGIN_BUF_HANDLE {
        return 1;
    }
    let a_id = read_u64_le(slice::from_raw_parts(slots[0].ptr, slots[0].len)).unwrap_or(0);
    let b_id = read_u64_le(slice::from_raw_parts(slots[1].ptr, slots[1].len)).unwrap_or(0);
    if a_id == 0 || b_id == 0 {
        return 1;
    }
    let mut abuf = SyxPluginBuf {
        kind: 0,
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    let mut bbuf = SyxPluginBuf {
        kind: 0,
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    let mut nrows_a = 0usize;
    let mut ncols_a = 0usize;
    let mut nrows_b = 0usize;
    let mut ncols_b = 0usize;
    if borrow(ops.ctx, a_id, &mut abuf, &mut nrows_a, &mut ncols_a) != 0 {
        return 1;
    }
    if borrow(ops.ctx, b_id, &mut bbuf, &mut nrows_b, &mut ncols_b) != 0 {
        return 1;
    }
    if ncols_a != nrows_b || nrows_a == 0 || ncols_b == 0 {
        return 1;
    }
    let a_bytes = slice::from_raw_parts(abuf.ptr, abuf.len);
    let b_bytes = slice::from_raw_parts(bbuf.ptr, bbuf.len);
    let Some(am) = mat_from_row_major(nrows_a, ncols_a, a_bytes) else {
        return 1;
    };
    let Some(bm) = mat_from_row_major(nrows_b, ncols_b, b_bytes) else {
        return 1;
    };
    let cm = &am * &bm;
    let mut packed = vec![0u8; cm.nrows() * cm.ncols() * 8];
    let mut off = 0usize;
    for r in 0..cm.nrows() {
        for c in 0..cm.ncols() {
            if !write_f64_le(&mut packed, off, cm[(r, c)]) {
                return 1;
            }
            off += 8;
        }
    }
    pin_handle_out(ops, out, cm.nrows(), cm.ncols(), &packed)
}

unsafe fn pin_handle_out(
    ops: &SyxPluginAlloc,
    out: &mut SyxPluginIov,
    nrows: usize,
    ncols: usize,
    packed: &[u8],
) -> i32 {
    let Some(pin) = ops.pin_fn else {
        return 1;
    };
    let mut out_id = 0u64;
    if pin(
        ops.ctx,
        nrows,
        ncols,
        packed.as_ptr(),
        packed.len(),
        &mut out_id,
    ) != 0
        || out_id == 0
    {
        return 1;
    }
    if out.bufs.is_null() {
        return 1;
    }
    let dest = &mut *out.bufs;
    if dest.ptr.is_null() {
        return 1;
    }
    let need = 8usize;
    let ptr = if dest.len < need {
        match ops.realloc_fn {
            Some(realloc) => realloc(ops.ctx, dest.ptr, need),
            None => return 1,
        }
    } else {
        dest.ptr
    };
    if ptr.is_null() {
        return 1;
    }
    slice::from_raw_parts_mut(ptr, need).copy_from_slice(&out_id.to_le_bytes());
    dest.kind = SYX_PLUGIN_BUF_HANDLE;
    dest.ptr = ptr;
    dest.len = need;
    0
}

unsafe extern "C" fn faer_cdylib_hsolve(
    in_iov: *const SyxPluginIov,
    out_iov: *mut SyxPluginIov,
    alloc: *mut SyxPluginAlloc,
) -> i32 {
    use faer::linalg::solvers::Solve;
    if in_iov.is_null() || out_iov.is_null() || alloc.is_null() {
        return 1;
    }
    let inn = &*in_iov;
    let out = &mut *out_iov;
    let ops = &*alloc;
    let Some(borrow) = ops.borrow_fn else {
        return 1;
    };
    if inn.count != 2 || inn.bufs.is_null() || out.count == 0 || out.bufs.is_null() {
        return 1;
    }
    let slots = slice::from_raw_parts(inn.bufs, 2);
    if slots[0].kind != SYX_PLUGIN_BUF_HANDLE || slots[1].kind != SYX_PLUGIN_BUF_HANDLE {
        return 1;
    }
    let a_id = read_u64_le(slice::from_raw_parts(slots[0].ptr, slots[0].len)).unwrap_or(0);
    let b_id = read_u64_le(slice::from_raw_parts(slots[1].ptr, slots[1].len)).unwrap_or(0);
    if a_id == 0 || b_id == 0 {
        return 1;
    }
    let mut abuf = SyxPluginBuf {
        kind: 0,
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    let mut bbuf = SyxPluginBuf {
        kind: 0,
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    let mut nrows_a = 0usize;
    let mut ncols_a = 0usize;
    let mut nrows_b = 0usize;
    let mut ncols_b = 0usize;
    if borrow(ops.ctx, a_id, &mut abuf, &mut nrows_a, &mut ncols_a) != 0 {
        return 1;
    }
    if borrow(ops.ctx, b_id, &mut bbuf, &mut nrows_b, &mut ncols_b) != 0 {
        return 1;
    }
    if nrows_a == 0 || nrows_a != ncols_a || nrows_b != nrows_a || ncols_b == 0 {
        return 1;
    }
    let a_bytes = slice::from_raw_parts(abuf.ptr, abuf.len);
    let b_bytes = slice::from_raw_parts(bbuf.ptr, bbuf.len);
    let Some(am) = mat_from_row_major(nrows_a, ncols_a, a_bytes) else {
        return 1;
    };
    let Some(bm) = mat_from_row_major(nrows_b, ncols_b, b_bytes) else {
        return 1;
    };
    let lu = am.partial_piv_lu();
    let u = lu.U();
    let n = nrows_a;
    let mut max_abs = 0.0_f64;
    for i in 0..n {
        max_abs = max_abs.max(u[(i, i)].abs());
    }
    let tol = (1e-12_f64).max(1e-14 * max_abs);
    for i in 0..n {
        if u[(i, i)].abs() < tol {
            return 1;
        }
    }
    let xm = lu.solve(&bm);
    let mut packed = vec![0u8; xm.nrows() * xm.ncols() * 8];
    let mut off = 0usize;
    for r in 0..xm.nrows() {
        for c in 0..xm.ncols() {
            if !write_f64_le(&mut packed, off, xm[(r, c)]) {
                return 1;
            }
            off += 8;
        }
    }
    pin_handle_out(ops, out, xm.nrows(), xm.ncols(), &packed)
}

unsafe fn write_f64le_out(ops: &SyxPluginAlloc, out: &mut SyxPluginIov, packed: &[u8]) -> i32 {
    if out.bufs.is_null() {
        return 1;
    }
    let dest = &mut *out.bufs;
    if dest.ptr.is_null() {
        return 1;
    }
    let need = packed.len();
    let ptr = if dest.len < need {
        match ops.realloc_fn {
            Some(realloc) => realloc(ops.ctx, dest.ptr, need),
            None => return 1,
        }
    } else {
        dest.ptr
    };
    if ptr.is_null() {
        return 1;
    }
    slice::from_raw_parts_mut(ptr, need).copy_from_slice(packed);
    dest.kind = SYX_PLUGIN_BUF_F64LE;
    dest.ptr = ptr;
    dest.len = need;
    0
}

unsafe extern "C" fn faer_cdylib_hfrom(
    in_iov: *const SyxPluginIov,
    out_iov: *mut SyxPluginIov,
    alloc: *mut SyxPluginAlloc,
) -> i32 {
    if in_iov.is_null() || out_iov.is_null() || alloc.is_null() {
        return 1;
    }
    let inn = &*in_iov;
    let out = &mut *out_iov;
    let ops = &*alloc;
    if inn.count != 1 || inn.bufs.is_null() || out.count == 0 || out.bufs.is_null() {
        return 1;
    }
    let slot = &*inn.bufs;
    if slot.kind != SYX_PLUGIN_BUF_F64LE
        || slot.ptr.is_null()
        || slot.len < 16
        || (slot.len % 8) != 0
    {
        return 1;
    }
    let bytes = slice::from_raw_parts(slot.ptr, slot.len);
    let n = slot.len / 8;
    let Some(nrows) = f64_as_dim(read_f64_le(bytes, 0).unwrap_or(0.0)) else {
        return 1;
    };
    let Some(ncols) = f64_as_dim(read_f64_le(bytes, 8).unwrap_or(0.0)) else {
        return 1;
    };
    let cells = match nrows.checked_mul(ncols) {
        Some(c) if c > 0 => c,
        _ => return 1,
    };
    if 2 + cells != n {
        return 1;
    }
    let data = &bytes[16..];
    pin_handle_out(ops, out, nrows, ncols, data)
}

unsafe extern "C" fn faer_cdylib_hto(
    in_iov: *const SyxPluginIov,
    out_iov: *mut SyxPluginIov,
    alloc: *mut SyxPluginAlloc,
) -> i32 {
    if in_iov.is_null() || out_iov.is_null() || alloc.is_null() {
        return 1;
    }
    let inn = &*in_iov;
    let out = &mut *out_iov;
    let ops = &*alloc;
    let Some(borrow) = ops.borrow_fn else {
        return 1;
    };
    if inn.count != 1 || inn.bufs.is_null() || out.count == 0 || out.bufs.is_null() {
        return 1;
    }
    let slot = &*inn.bufs;
    if slot.kind != SYX_PLUGIN_BUF_HANDLE {
        return 1;
    }
    let id = read_u64_le(slice::from_raw_parts(slot.ptr, slot.len)).unwrap_or(0);
    if id == 0 {
        return 1;
    }
    let mut buf = SyxPluginBuf {
        kind: 0,
        ptr: std::ptr::null_mut(),
        len: 0,
    };
    let mut nrows = 0usize;
    let mut ncols = 0usize;
    if borrow(ops.ctx, id, &mut buf, &mut nrows, &mut ncols) != 0 {
        return 1;
    }
    if nrows == 0 || ncols == 0 || buf.ptr.is_null() {
        return 1;
    }
    let data = slice::from_raw_parts(buf.ptr, buf.len);
    let cells = match nrows.checked_mul(ncols) {
        Some(c) if c > 0 => c,
        _ => return 1,
    };
    if data.len() != cells * 8 {
        return 1;
    }
    let mut packed = vec![0u8; 16 + data.len()];
    if !write_f64_le(&mut packed, 0, nrows as f64) || !write_f64_le(&mut packed, 8, ncols as f64) {
        return 1;
    }
    packed[16..].copy_from_slice(data);
    write_f64le_out(ops, out, &packed)
}

unsafe extern "C" fn faer_cdylib_hdrop(
    in_iov: *const SyxPluginIov,
    out_iov: *mut SyxPluginIov,
    alloc: *mut SyxPluginAlloc,
) -> i32 {
    if in_iov.is_null() || out_iov.is_null() || alloc.is_null() {
        return 1;
    }
    let inn = &*in_iov;
    let out = &mut *out_iov;
    let ops = &*alloc;
    let Some(drop) = ops.drop_fn else {
        return 1;
    };
    if inn.count != 1 || inn.bufs.is_null() || out.count == 0 || out.bufs.is_null() {
        return 1;
    }
    let slot = &*inn.bufs;
    if slot.kind != SYX_PLUGIN_BUF_HANDLE {
        return 1;
    }
    let id = read_u64_le(slice::from_raw_parts(slot.ptr, slot.len)).unwrap_or(0);
    if id == 0 {
        return 1;
    }
    if drop(ops.ctx, id) != 0 {
        return 1;
    }
    write_i64_handle_out(ops, out, 0)
}

unsafe fn write_i64_handle_out(ops: &SyxPluginAlloc, out: &mut SyxPluginIov, id: u64) -> i32 {
    if out.bufs.is_null() {
        return 1;
    }
    let dest = &mut *out.bufs;
    if dest.ptr.is_null() {
        return 1;
    }
    let need = 8usize;
    let ptr = if dest.len < need {
        match ops.realloc_fn {
            Some(realloc) => realloc(ops.ctx, dest.ptr, need),
            None => return 1,
        }
    } else {
        dest.ptr
    };
    if ptr.is_null() {
        return 1;
    }
    slice::from_raw_parts_mut(ptr, need).copy_from_slice(&id.to_le_bytes());
    dest.kind = SYX_PLUGIN_BUF_HANDLE;
    dest.ptr = ptr;
    dest.len = need;
    0
}

static V2_HOSTS: OnceLock<[SyxHostFnDescV2; 6]> = OnceLock::new();
static V2M_HOSTS: OnceLock<[SyxHostFnDescV2m; 8]> = OnceLock::new();
static PLUGIN: OnceLock<SyxPluginInfoV2m> = OnceLock::new();

/// Required entry point — dlopen looks up this symbol.
#[no_mangle]
pub extern "C" fn syx_plugin_init() -> *const core::ffi::c_void {
    let v2 = V2_HOSTS.get_or_init(|| {
        [
            SyxHostFnDescV2 {
                name: c"__syx_plugin_faer_cdylib_mul".as_ptr(),
                return_type: c"Array".as_ptr(),
                arity: 1,
                func: faer_cdylib_mul,
            },
            SyxHostFnDescV2 {
                name: c"__syx_plugin_faer_cdylib_solve".as_ptr(),
                return_type: c"Array".as_ptr(),
                arity: 1,
                func: faer_cdylib_solve,
            },
            SyxHostFnDescV2 {
                name: c"__syx_plugin_faer_cdylib_zeros".as_ptr(),
                return_type: c"Array".as_ptr(),
                arity: 1,
                func: faer_cdylib_zeros,
            },
            SyxHostFnDescV2 {
                name: c"__syx_plugin_faer_cdylib_add".as_ptr(),
                return_type: c"Array".as_ptr(),
                arity: 1,
                func: faer_cdylib_add,
            },
            SyxHostFnDescV2 {
                name: c"__syx_plugin_faer_cdylib_sub".as_ptr(),
                return_type: c"Array".as_ptr(),
                arity: 1,
                func: faer_cdylib_sub,
            },
            SyxHostFnDescV2 {
                name: c"__syx_plugin_faer_cdylib_hadamard".as_ptr(),
                return_type: c"Array".as_ptr(),
                arity: 1,
                func: faer_cdylib_hadamard,
            },
        ]
    });
    let v2m = V2M_HOSTS.get_or_init(|| {
        [
            SyxHostFnDescV2m {
                name: c"__syx_plugin_faer_cdylib_hmul".as_ptr(),
                return_type: c"Int".as_ptr(),
                arity: 2,
                func: faer_cdylib_hmul,
            },
            SyxHostFnDescV2m {
                name: c"__syx_plugin_faer_cdylib_hsolve".as_ptr(),
                return_type: c"Int".as_ptr(),
                arity: 2,
                func: faer_cdylib_hsolve,
            },
            SyxHostFnDescV2m {
                name: c"__syx_plugin_faer_cdylib_hfrom".as_ptr(),
                return_type: c"Int".as_ptr(),
                arity: 1,
                func: faer_cdylib_hfrom,
            },
            SyxHostFnDescV2m {
                name: c"__syx_plugin_faer_cdylib_hto".as_ptr(),
                return_type: c"Array".as_ptr(),
                arity: 1,
                func: faer_cdylib_hto,
            },
            SyxHostFnDescV2m {
                name: c"__syx_plugin_faer_cdylib_hdrop".as_ptr(),
                return_type: c"Int".as_ptr(),
                arity: 1,
                func: faer_cdylib_hdrop,
            },
            SyxHostFnDescV2m {
                name: c"__syx_plugin_faer_cdylib_hadd".as_ptr(),
                return_type: c"Int".as_ptr(),
                arity: 2,
                func: faer_cdylib_hadd,
            },
            SyxHostFnDescV2m {
                name: c"__syx_plugin_faer_cdylib_hsub".as_ptr(),
                return_type: c"Int".as_ptr(),
                arity: 2,
                func: faer_cdylib_hsub,
            },
            SyxHostFnDescV2m {
                name: c"__syx_plugin_faer_cdylib_hhadamard".as_ptr(),
                return_type: c"Int".as_ptr(),
                arity: 2,
                func: faer_cdylib_hhadamard,
            },
        ]
    });
    let info = PLUGIN.get_or_init(|| SyxPluginInfoV2m {
        api_version: SYX_PLUGIN_API_VERSION_V2M,
        plugin_id: c"faerlin".as_ptr(),
        host_count: 0,
        hosts: std::ptr::null(),
        v2_host_count: 6,
        v2_hosts: v2.as_ptr(),
        v2m_host_count: 8,
        v2m_hosts: v2m.as_ptr(),
    });
    let _ = SYX_PLUGIN_API_VERSION_V2;
    info as *const SyxPluginInfoV2m as *const core::ffi::c_void
}
