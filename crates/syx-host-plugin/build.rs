fn main() {
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-arg=-Wl,--export-dynamic");
    }
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-arg=-Wl,-export_dynamic");
    }
}
