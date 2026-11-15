//! U-10: `while let Ident = expr` desugars to While+IfLet+Break.
//! Bounded probes only — Optional must become nil (no unbounded while).

use std::io::Write;

fn write_temp(src: &str) -> std::path::PathBuf {
    let mut f = tempfile::NamedTempFile::new().expect("temp");
    f.write_all(src.as_bytes()).expect("write");
    f.into_temp_path().keep().expect("keep")
}

#[test]
fn u10_while_let_bounded_optional_runs() {
    let path = write_temp(
        r#"
var n = 0
public func next() -> Int? {
    if n >= 3 {
        return nil
    }
    let v = n
    n = n + 1
    return v
}
public func main() {
    while let v = next() {
        print(v)
    }
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("while let next() must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["0", "1", "2"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u10_while_let_immediate_nil_skips_body() {
    let path = write_temp(
        r#"
public func empty() -> Int? {
    return nil
}
public func main() {
    while let v = empty() {
        print(v)
    }
    print("done")
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("while let nil must skip body");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["done"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u10_plain_while_still_runs() {
    let path = write_temp(
        r#"
public func main() {
    var i = 0
    while i < 3 {
        print(i)
        i = i + 1
    }
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("plain while must still run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["0", "1", "2"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u10_if_let_still_runs() {
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
            .expect("if let must still run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["7"]);
    let _ = std::fs::remove_file(&path);
}
