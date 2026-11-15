//! U-9: block statements tolerate optional `;` separators.
//! Bounded probes only — no unbounded `while`.

use std::io::Write;

fn write_temp(src: &str) -> std::path::PathBuf {
    let mut f = tempfile::NamedTempFile::new().expect("temp");
    f.write_all(src.as_bytes()).expect("write");
    f.into_temp_path().keep().expect("keep")
}

#[test]
fn u9_if_block_then_semicolon_stmt_runs() {
    let path = write_temp(
        r#"
public func main() {
    if true { }; print(1)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("if { }; print must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["1"]);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u9_double_semicolon_tolerated() {
    let path = write_temp(
        r#"
public func main() {
    if true { } ;; print(2)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("double semicolon must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["2"]);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u9_no_semicolon_if_still_runs() {
    let path = write_temp(
        r#"
public func main() {
    if true { print(1) }
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("no-semicolon if must still run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["1"]);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u9_let_semicolon_then_stmt_runs() {
    let path = write_temp(
        r#"
public func main() {
    let x = 1; print(x)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("let x = 1; print must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["1"]);
    let _ = std::fs::remove_file(&path);
}
