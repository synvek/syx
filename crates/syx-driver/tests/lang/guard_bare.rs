//! U-17: bare `guard cond else { E }` → If(cond, then=[], else=E).
//! Bounded probes only — no unbounded loops.

use std::io::Write;

fn write_temp(src: &str) -> std::path::PathBuf {
    let mut f = tempfile::NamedTempFile::new().expect("temp");
    f.write_all(src.as_bytes()).expect("write");
    f.into_temp_path().keep().expect("keep")
}

#[test]
fn u17_guard_true_continues() {
    let path = write_temp(
        r#"
public func main() {
    guard true else {
        print("else")
        return
    }
    print(1)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("guard true must continue");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["1"]);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u17_guard_false_takes_else() {
    let path = write_temp(
        r#"
public func main() {
    guard false else {
        print("else")
        return
    }
    print("no")
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("guard false must take else");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["else"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u17_guard_expr_cond_continues() {
    let path = write_temp(
        r#"
public func main() {
    let n = 3
    guard n > 0 else {
        print("else")
        return
    }
    print(n)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("guard expr cond must continue");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["3"]);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u17_single_guard_let_still_runs() {
    let path = write_temp(
        r#"
public func main() {
    let a: Int? = 7
    guard let x = a else {
        return
    }
    print(x)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("single guard let must still run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["7"]);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u17_comma_guard_let_still_runs() {
    let path = write_temp(
        r#"
public func main() {
    let a: Int? = 1
    let b: Int? = 2
    guard let x = a, let y = b else {
        print("else")
        return
    }
    print(x)
    print(y)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("comma guard let must still run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["1", "2"]
    );
    let _ = std::fs::remove_file(&path);
}
