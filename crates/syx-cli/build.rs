fn main() {
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-arg=-Wl,--export-dynamic");
    }
    // Phase 124: libsyx_gui dlopen / copy side-car retired. SynUI is linked via syx-vm path dep.
}
