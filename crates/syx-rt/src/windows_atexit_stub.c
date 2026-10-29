/* Satisfy Rust `std` references to `atexit` without pulling MSVC
 * `msvcrt.lib(utility.obj)`. That object needs `__vcrt_*` / `__acrt_*` from
 * static `libvcruntime` / `libucrt`, which conflict with the dynamic `/MD`
 * import libs Rust staticlibs expect.
 *
 * SYX native executables use `/ENTRY:syx_rt_start` and do not run the MSVC
 * CRT startup path that would call registered `atexit` handlers.
 */
typedef void (*syx_atexit_fn)(void);

int atexit(syx_atexit_fn func) {
  (void)func;
  return 0;
}
