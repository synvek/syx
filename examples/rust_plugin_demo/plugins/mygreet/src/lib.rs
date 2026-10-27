//! Minimal SYX Host plugin (Rust → C ABI v1).
//!
//! Exports `syx_plugin_init` and one Host: `__syx_plugin_mygreet_greet`.
//! See `crates/syx-host-plugin/include/syx_plugin.h`.

use std::ffi::CStr;
use std::os::raw::c_char;
use std::slice;
use std::sync::OnceLock;

const SYX_PLUGIN_API_VERSION: u32 = 1;

type SyxHostFn = unsafe extern "C" fn(*const c_char, *mut c_char, usize);

#[repr(C)]
struct SyxHostFnDesc {
    name: *const c_char,
    return_type: *const c_char,
    arity: u32,
    func: SyxHostFn,
}

// C ABI structs hold raw pointers; init runs once and the data is immutable afterward.
unsafe impl Sync for SyxHostFnDesc {}
unsafe impl Send for SyxHostFnDesc {}

#[repr(C)]
pub struct SyxPluginInfo {
    api_version: u32,
    plugin_id: *const c_char,
    host_count: u32,
    hosts: *const SyxHostFnDesc,
}

unsafe impl Sync for SyxPluginInfo {}
unsafe impl Send for SyxPluginInfo {}

/// `greet(name: String) -> String` → `"Hello, <name>!"`
unsafe extern "C" fn greet_host(input: *const c_char, out: *mut c_char, out_cap: usize) {
    if out.is_null() || out_cap == 0 {
        return;
    }
    let name = if input.is_null() {
        ""
    } else {
        // SAFETY: loader passes a NUL-terminated UTF-8 C string.
        CStr::from_ptr(input).to_str().unwrap_or("")
    };
    let msg = format!("Hello, {name}!");
    let bytes = msg.as_bytes();
    let copy_len = bytes.len().min(out_cap.saturating_sub(1));
    // SAFETY: `out` is a writable buffer of `out_cap` bytes from the host.
    let dest = slice::from_raw_parts_mut(out as *mut u8, out_cap);
    dest[..copy_len].copy_from_slice(&bytes[..copy_len]);
    dest[copy_len] = 0;
}

static PLUGIN: OnceLock<SyxPluginInfo> = OnceLock::new();
static HOST: OnceLock<SyxHostFnDesc> = OnceLock::new();

/// Required entry point — dlopen looks up this symbol.
#[no_mangle]
pub extern "C" fn syx_plugin_init() -> *const SyxPluginInfo {
    let host = HOST.get_or_init(|| SyxHostFnDesc {
        name: c"__syx_plugin_mygreet_greet".as_ptr(),
        return_type: c"String".as_ptr(),
        arity: 1,
        func: greet_host,
    });
    let info = PLUGIN.get_or_init(|| SyxPluginInfo {
        api_version: SYX_PLUGIN_API_VERSION,
        plugin_id: c"mygreet".as_ptr(),
        host_count: 1,
        hosts: host as *const SyxHostFnDesc,
    });
    info as *const SyxPluginInfo
}
