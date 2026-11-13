//! U-7: MIR lowering for `break` / `continue` / `repeat-while` (run, not check-only).

use std::io::Write;

fn write_temp(src: &str) -> std::path::PathBuf {
    let mut f = tempfile::NamedTempFile::new().expect("temp");
    f.write_all(src.as_bytes()).expect("write");
    f.into_temp_path().keep().expect("keep")
}

#[test]
fn u7_while_break_runs() {
    let path = write_temp(
        r#"
public func main() {
    var i = 0
    while i < 5 {
        i = i + 1
        if i == 2 { break }
    }
    print(i)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("while+break must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["2"]);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u7_for_continue_runs() {
    let path = write_temp(
        r#"
public func main() {
    var s = 0
    for i in 0..<5 {
        if i == 2 { continue }
        s = s + i
    }
    print(s)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("for+continue must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["8"]);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u7_repeat_while_runs() {
    let path = write_temp(
        r#"
public func main() {
    var n = 0
    repeat {
        n = n + 1
    } while n < 3
    print(n)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("repeat-while must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["3"]);
    let _ = std::fs::remove_file(&path);
}
