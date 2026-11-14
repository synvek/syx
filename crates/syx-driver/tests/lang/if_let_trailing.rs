//! U-8: `if let` / `if case` RHS must not swallow then-brace as trailing closure.
//! Bounded probes only — no unbounded `while`.

use std::io::Write;

fn write_temp(src: &str) -> std::path::PathBuf {
    let mut f = tempfile::NamedTempFile::new().expect("temp");
    f.write_all(src.as_bytes()).expect("write");
    f.into_temp_path().keep().expect("keep")
}

#[test]
fn u8_if_let_call_rhs_runs() {
    let path = write_temp(
        r#"
public func f() -> Int? {
    return 7
}
public func main() {
    if let v = f() {
        print(v)
    }
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("if let v = f() must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["7"]);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u8_if_case_call_subject_runs() {
    let path = write_temp(
        r#"
public func f() -> Int {
    return 7
}
public func main() {
    if case 7 = f() {
        print("ok")
    }
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("if case 7 = f() must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["ok"]);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u8_if_let_paren_workaround_still_runs() {
    let path = write_temp(
        r#"
public func f() -> Int? {
    return 3
}
public func main() {
    if let v = (f()) {
        print(v)
    }
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("if let (f()) must still run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["3"]);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u8_plain_if_call_still_runs() {
    let path = write_temp(
        r#"
public func f() -> Bool {
    return true
}
public func main() {
    if f() {
        print(1)
    }
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("plain if f() must still run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["1"]);
    let _ = std::fs::remove_file(&path);
}
