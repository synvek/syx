fn main() {
    println!("LLVM_SYS_211_PREFIX={:?}", std::env::var("LLVM_SYS_211_PREFIX"));
    println!("SYX_LLVM_PREFIX={:?}", std::env::var("SYX_LLVM_PREFIX"));
    if let Ok(p) = std::env::var("LLVM_SYS_211_PREFIX") {
        let cfg = std::path::Path::new(&p).join("bin").join("llvm-config.exe");
        println!("llvm-config exists? {} at {}", cfg.is_file(), cfg.display());
        if cfg.is_file() {
            let o = std::process::Command::new(&cfg).arg("--version").output().unwrap();
            println!("version={}", String::from_utf8_lossy(&o.stdout));
        }
    }
}
