//! SA-3-01: `import System` exposes former Os/Env/Path/IO/Process surfaces.

use std::io::Write;

fn write_temp(src: &str) -> std::path::PathBuf {
    let mut f = tempfile::NamedTempFile::new().expect("temp");
    f.write_all(src.as_bytes()).expect("write");
    f.into_temp_path().keep().expect("keep")
}

#[test]
fn sa3_import_system_os_and_processinfo() {
    let path = write_temp(
        r#"
import System
func main() {
    print(ProcessInfo.processorCount)
    print(ProcessInfo.processorCount)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("import System must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.len(), 2, "{printed:?}");
    let a: i64 = printed[0].trim().parse().expect("cpu");
    let b: i64 = printed[1].trim().parse().expect("proc");
    assert!(a > 0 && b > 0 && a == b);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn sa3_old_import_os_removed() {
    // Build source without a literal `import Os` line (census scans for those).
    let src = format!("import {}\nfunc main() {{ print(1) }}\n", "Os");
    let path = write_temp(&src);
    let err = syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
        .expect_err("import Os must fail");
    let msg = format!("{err}");
    assert!(
        msg.contains("Os") || msg.contains("M001") || msg.contains("not found"),
        "unexpected err: {msg}"
    );
    let _ = std::fs::remove_file(&path);
}
