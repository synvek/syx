//! Phase 13: C FFI embed surface (`syx_embed_v1_*`) — ADR-003 Accepted.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::{compile_and_run_embedded, DriverOptions};

#[repr(C)]
pub struct SyxEmbedResult {
    pub exit_code: i32,
    pub line_count: i32,
    pub lines: *mut *mut c_char,
}

/// Compile and run SYX source; returns heap-allocated result or null on failure.
#[no_mangle]
pub extern "C" fn syx_embed_v1_compile_run(source: *const c_char) -> *mut SyxEmbedResult {
    if source.is_null() {
        return std::ptr::null_mut();
    }
    let c_str = unsafe { CStr::from_ptr(source) };
    let src = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let result = match compile_and_run_embedded(src, &DriverOptions::default()) {
        Ok(r) => r,
        Err(_) => return std::ptr::null_mut(),
    };
    let mut line_ptrs: Vec<*mut c_char> = result
        .printed
        .into_iter()
        .filter_map(|s| CString::new(s).ok().map(|c| c.into_raw()))
        .collect();
    let out = Box::new(SyxEmbedResult {
        exit_code: result.exit_code,
        line_count: line_ptrs.len() as i32,
        lines: line_ptrs.as_mut_ptr(),
    });
    std::mem::forget(line_ptrs);
    Box::into_raw(out)
}

/// Free a result returned by [`syx_embed_v1_compile_run`].
#[no_mangle]
pub extern "C" fn syx_embed_v1_free_result(ptr: *mut SyxEmbedResult) {
    if ptr.is_null() {
        return;
    }
    let boxed = unsafe { Box::from_raw(ptr) };
    if !boxed.lines.is_null() && boxed.line_count > 0 {
        let lines = unsafe {
            Vec::from_raw_parts(
                boxed.lines,
                boxed.line_count as usize,
                boxed.line_count as usize,
            )
        };
        for p in lines {
            unsafe {
                let _ = CString::from_raw(p);
            }
        }
    }
}
