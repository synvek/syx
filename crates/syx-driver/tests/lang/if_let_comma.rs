//! U-14: restore comma `if let x = a, let y = b[, cond]` → nested IfLet / trailing If.
//! Bounded probes only — no loops.

use std::io::Write;

fn write_temp(src: &str) -> std::path::PathBuf {
    let mut f = tempfile::NamedTempFile::new().expect("temp");
    f.write_all(src.as_bytes()).expect("write");
    f.into_temp_path().keep().expect("keep")
}

#[test]
fn u14_comma_both_some_enters_body() {
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
            .expect("comma if let both some must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["1", "2"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u14_comma_first_nil_takes_else() {
    let path = write_temp(
        r#"
public func main() {
    let a: Int? = nil
    let b: Int? = 2
    if let x = a, let y = b {
        print("no")
    } else {
        print("else")
    }
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("first nil must take else");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["else"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u14_comma_second_nil_takes_else() {
    let path = write_temp(
        r#"
public func main() {
    let a: Int? = 1
    let b: Int? = nil
    if let x = a, let y = b {
        print("no")
    } else {
        print("else")
    }
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("second nil must take else");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["else"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u14_comma_trailing_cond_true() {
    let path = write_temp(
        r#"
public func main() {
    let a: Int? = 3
    if let x = a, x > 0 {
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
fn u14_comma_trailing_cond_false_takes_else() {
    let path = write_temp(
        r#"
public func main() {
    let a: Int? = 3
    if let x = a, x < 0 {
        print("no")
    } else {
        print("else")
    }
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("trailing cond false must take else");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["else"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u14_single_if_let_still_runs() {
    let path = write_temp(
        r#"
public func main() {
    let a: Int? = 7
    if let x = a { print(x) }
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("single if let must still run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["7"]);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u14_plain_if_still_runs() {
    let path = write_temp(
        r#"
public func main() {
    if true { print(1) }
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("plain if must still run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["1"]);
    let _ = std::fs::remove_file(&path);
}
