fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    if target_os == "linux" {
        println!("cargo:rustc-link-arg=-Wl,--export-dynamic");
    }
    // Phase 124: libsyx_gui dlopen / copy side-car retired. SynUI is linked via syx-vm path dep.

    // `full-native` statically links both wasmtime (whose `helpers.c` defines
    // `__jit_debug_register_code` / `__jit_debug_descriptor` as non-weak
    // `__declspec(dllexport)` on Windows) and the LLVM static archives
    // (`JITLoaderGDB.cpp.obj` defines the same two symbols). On ELF/Mach-O both
    // definitions are weak and merge; MSVC link.exe reports LNK2005/LNK1169
    // instead. Neither JIT registration is reachable from this binary (wasmtime
    // runs Cranelift, syx-llvm only emits objects), so letting the first
    // definition win is safe. Same remedy as `native_lld_argv` in syx-rt.
    if target_os == "windows" && std::env::var_os("CARGO_FEATURE_FULL_NATIVE").is_some() {
        println!("cargo:rustc-link-arg-bins=/FORCE:MULTIPLE");
    }
}
