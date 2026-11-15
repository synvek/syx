//! Shared native AOT runner for driver integration tests.
//! Soft-skips when the toolchain lacks `--features full-native` (L006).

use std::fs;
use std::process::Command;

use syx_driver::{build_native_executable, DriverOptions};

/// Build + run under `SYX_NATIVE_STRICT=1`.
/// Returns `None` when LLVM/LLD full-native is unavailable (honest L006 refuse).
pub fn try_run_native_strict(src: &str, label: &str) -> Option<(i32, String)> {
    // Refuse before MIR/LLVM work: L006 is only raised deep in the native pipeline.
    if !cfg!(feature = "full-native") {
        eprintln!("skip native AOT ({label}): toolchain built without --features full-native");
        return None;
    }
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_native_aot_{label}_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), src).unwrap();
    fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
    let exe = dir.join("app");
    match build_native_executable(&dir, &DriverOptions::default(), &exe) {
        Ok(_) => {}
        Err(e) => {
            let msg = format!("{e}");
            if msg.contains("L006") || msg.contains("full-native") {
                eprintln!("skip native AOT ({label}): {msg}");
                return None;
            }
            panic!("native build failed ({label}): {e}");
        }
    }
    let output = Command::new(&exe).output().expect("run native exe");
    Some((
        output.status.code().unwrap_or(1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
    ))
}
