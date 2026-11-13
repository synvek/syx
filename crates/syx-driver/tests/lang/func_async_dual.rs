//! U-4: dual-accept `func…async` (postfix) while keeping `async func` (prefix).

use std::io::Write;

fn write_temp(src: &str) -> std::path::PathBuf {
    let mut f = tempfile::NamedTempFile::new().expect("temp");
    f.write_all(src.as_bytes()).expect("write");
    f.into_temp_path().keep().expect("keep")
}

#[test]
fn u4_postfix_func_async_check_and_run() {
    let path = write_temp(
        r#"
func f() async {
    print(1)
}
public func main() {
    f()
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("postfix func() async must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["1"]);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u4_postfix_func_async_arrow_run() {
    let path = write_temp(
        r#"
func f() async -> Int {
    return 2
}
public func main() {
    print(f())
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("postfix func() async -> T must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["2"]);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u4_prefix_async_func_still_runs() {
    let path = write_temp(
        r#"
async func f() {
    print(3)
}
public func main() {
    f()
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("prefix async func must still run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["3"]);
    let _ = std::fs::remove_file(&path);
}
