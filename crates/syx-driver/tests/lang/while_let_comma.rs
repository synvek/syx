//! U-31: comma `while let x = a, let y = b[, cond]` → nested IfLet / trailing If + break.
//! Bounded probes only — Optional must become nil (no unbounded while).

use std::io::Write;

fn write_temp(src: &str) -> std::path::PathBuf {
    let mut f = tempfile::NamedTempFile::new().expect("temp");
    f.write_all(src.as_bytes()).expect("write");
    f.into_temp_path().keep().expect("keep")
}

#[test]
fn u31_comma_both_some_enters_body() {
    let path = write_temp(
        r#"
var step = 0
public func left() -> Int? {
    if step >= 1 { return nil }
    return 1
}
public func right() -> Int? {
    return 2
}
public func main() {
    while let x = left(), let y = right() {
        print(x)
        print(y)
        step = step + 1
    }
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("comma while let both some must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["1", "2"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u31_comma_first_nil_skips_body() {
    let path = write_temp(
        r#"
public func empty() -> Int? { return nil }
public func right() -> Int? { return 2 }
public func main() {
    while let x = empty(), let y = right() {
        print("no")
    }
    print("done")
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("first nil must skip body");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["done"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u31_comma_second_nil_skips_body() {
    let path = write_temp(
        r#"
public func left() -> Int? { return 1 }
public func empty() -> Int? { return nil }
public func main() {
    while let x = left(), let y = empty() {
        print("no")
    }
    print("done")
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("second nil must skip body");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["done"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u31_comma_trailing_cond_true() {
    let path = write_temp(
        r#"
var n = 0
public func next() -> Int? {
    if n >= 1 { return nil }
    let v = 3
    n = n + 1
    return v
}
public func main() {
    while let x = next(), x > 0 {
        print(x)
    }
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("trailing cond true must enter");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["3"]);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u31_comma_trailing_cond_false_skips_body() {
    let path = write_temp(
        r#"
public func once() -> Int? { return 3 }
public func main() {
    while let x = once(), x < 0 {
        print("no")
    }
    print("done")
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("trailing cond false must skip body");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["done"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u31_single_while_let_still_runs() {
    let path = write_temp(
        r#"
var n = 0
public func next() -> Int? {
    if n >= 1 { return nil }
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
            .expect("single while let must still run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["0"]);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u31_if_let_comma_still_runs() {
    let path = write_temp(
        r#"
public func main() {
    let a: Int? = 1
    let b: Int? = 2
    if let x = a, let y = b {
        print(x)
        print(y)
    }
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("comma if let must still run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["1", "2"]
    );
    let _ = std::fs::remove_file(&path);
}
