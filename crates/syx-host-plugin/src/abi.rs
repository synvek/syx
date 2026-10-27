//! C ABI v1 / v2 / v2m for dynamic Host plugins (ADR-071; W1045 / ADR-1390; W1046 / ADR-1391).

use std::ffi::c_void;

pub const SYX_PLUGIN_API_VERSION: u32 = 1;
pub const SYX_PLUGIN_API_VERSION_V2: u32 = 2;
pub const SYX_PLUGIN_API_VERSION_V2M: u32 = 3;

pub const SYX_PLUGIN_BUF_UTF8: u32 = 0;
pub const SYX_PLUGIN_BUF_F64LE: u32 = 1;
/// W1047 / ADR-1392: 8-byte LE u64 host handle. Not f64-as-handle.
pub const SYX_PLUGIN_BUF_HANDLE: u32 = 2;
pub const SYX_PLUGIN_V2_OUT_CAP: usize = 65536;
pub const SYX_PLUGIN_V2M_GROW_INITIAL_CAP: usize = 8;

pub type SyxValueStack = c_void;

pub type SyxHostFn = unsafe extern "C" fn(*const u8, *mut u8, usize);

#[repr(C)]
pub struct SyxHostFnDesc {
    pub name: *const u8,
    pub return_type: *const u8,
    pub arity: u32,
    pub func: Option<SyxHostFn>,
}

#[repr(C)]
pub struct SyxPluginInfo {
    pub api_version: u32,
    pub plugin_id: *const u8,
    pub host_count: u32,
    pub hosts: *const SyxHostFnDesc,
}

#[repr(C)]
pub struct SyxPluginBuf {
    pub kind: u32,
    pub ptr: *mut u8,
    pub len: usize,
}

pub type SyxHostFnV2 = unsafe extern "C" fn(*const SyxPluginBuf, *mut SyxPluginBuf, usize) -> i32;

#[repr(C)]
pub struct SyxHostFnDescV2 {
    pub name: *const u8,
    pub return_type: *const u8,
    pub arity: u32,
    pub func: Option<SyxHostFnV2>,
}

#[repr(C)]
pub struct SyxPluginInfoV2 {
    pub api_version: u32,
    pub plugin_id: *const u8,
    pub host_count: u32,
    pub hosts: *const SyxHostFnDesc,
    pub v2_host_count: u32,
    pub v2_hosts: *const SyxHostFnDescV2,
}

#[repr(C)]
pub struct SyxPluginIov {
    pub count: u32,
    pub bufs: *mut SyxPluginBuf,
}

pub type SyxPluginReallocFn = unsafe extern "C" fn(*mut c_void, *mut u8, usize) -> *mut u8;

/// W1051 / ADR-1396: copy-out borrow of host handle (call-duration).
pub type SyxPluginBorrowFn =
    unsafe extern "C" fn(*mut c_void, u64, *mut SyxPluginBuf, *mut usize, *mut usize) -> i32;

/// W1051 / ADR-1396: pin row-major f64-le into the host handle table.
pub type SyxPluginPinFn =
    unsafe extern "C" fn(*mut c_void, usize, usize, *const u8, usize, *mut u64) -> i32;

/// W1055 / ADR-1400: drop a host handle (unknown / double drop → nonzero).
pub type SyxPluginDropFn = unsafe extern "C" fn(*mut c_void, u64) -> i32;

#[repr(C)]
pub struct SyxPluginAlloc {
    pub ctx: *mut c_void,
    pub realloc_fn: Option<SyxPluginReallocFn>,
    pub borrow_fn: Option<SyxPluginBorrowFn>,
    pub pin_fn: Option<SyxPluginPinFn>,
    pub drop_fn: Option<SyxPluginDropFn>,
}

pub type SyxHostFnV2m =
    unsafe extern "C" fn(*const SyxPluginIov, *mut SyxPluginIov, *mut SyxPluginAlloc) -> i32;

#[repr(C)]
pub struct SyxHostFnDescV2m {
    pub name: *const u8,
    pub return_type: *const u8,
    pub arity: u32,
    pub func: Option<SyxHostFnV2m>,
}

#[repr(C)]
pub struct SyxPluginInfoV2m {
    pub api_version: u32,
    pub plugin_id: *const u8,
    pub host_count: u32,
    pub hosts: *const SyxHostFnDesc,
    pub v2_host_count: u32,
    pub v2_hosts: *const SyxHostFnDescV2,
    pub v2m_host_count: u32,
    pub v2m_hosts: *const SyxHostFnDescV2m,
}

pub type SyxPluginInitFn = unsafe extern "C" fn() -> *const SyxPluginInfo;
