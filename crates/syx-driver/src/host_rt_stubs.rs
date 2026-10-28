//! Host-toolchain stubs for AOT user entry points.
//!
//! `syx-rt::syx_rt_start` references `syx_user_init` / `syx_user_main`, which
//! real native programs provide via LLVM/C codegen. Host tools (`syx`, gates)
//! link `syx-rt` and may retain `syx_rt_start` (MSVC always; Linux rust-lld /
//! some macOS link configs also keep it), so those externals must resolve.
//!
//! Stubs live in `syx-driver` (not `syx-rt`) so they are **not** baked into
//! `libsyx_rt` / `syx_rt.lib` used when linking user AOT objects.

#[no_mangle]
pub extern "C" fn syx_user_init() {}

#[no_mangle]
pub extern "C" fn syx_user_main() -> i32 {
    0
}
