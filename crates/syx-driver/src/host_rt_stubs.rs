//! Host-toolchain stubs for AOT user entry points.
//!
//! `syx-rt::syx_rt_start` references `syx_user_init` / `syx_user_main`, which
//! real native programs provide via LLVM/C codegen. On Windows MSVC, `#[no_mangle]`
//! symbols from `syx-rt` are retained when linking host tools (`syx.exe`), so those
//! externals must resolve. macOS/Linux dead-strip unused `syx_rt_start` and do not
//! need these.
//!
//! Stubs live in `syx-driver` (not `syx-rt`) so they are **not** baked into
//! `libsyx_rt` / `syx_rt.lib` used when linking user AOT objects.

#[no_mangle]
pub extern "C" fn syx_user_init() {}

#[no_mangle]
pub extern "C" fn syx_user_main() -> i32 {
    0
}
