//! U-15: restore `while case pattern = subject` → While+IfCase+Break (paper U-12).
//! Bounded probes only — subject must eventually mismatch (no unbounded while).

use std::io::Write;

fn write_temp(src: &str) -> std::path::PathBuf {
    let mut f = tempfile::NamedTempFile::new().expect("temp");
    f.write_all(src.as_bytes()).expect("write");
    f.into_temp_path().keep().expect("keep")
}

#[test]
fn u15_while_case_bounded_sequence_runs() {
    let path = write_temp(
        r#"
var n = 0
public func next() -> Int {
    let v = n
    n = n + 1
    return v
}
public func main() {
    while case 0 = next() {
        print("hit")
    }
    print("done")
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("while case bounded must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["hit", "done"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u15_while_case_immediate_mismatch_skips_body() {
    let path = write_temp(
        r#"
public func always() -> Int {
    return 9
}
public func main() {
    while case 0 = always() {
        print("no")
    }
    print("done")
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("while case mismatch must skip body");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["done"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u15_while_case_multi_match_then_exit() {
    let path = write_temp(
        r#"
var n = 0
public func next() -> Int {
    // Returns 1 three times, then 0 → while case 1 exits after three hits.
    if n >= 3 {
        return 0
    }
    n = n + 1
    return 1
}
public func main() {
    while case 1 = next() {
        print(n)
    }
    print("done")
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("while case multi-match must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["1", "2", "3", "done"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u15_plain_while_still_runs() {
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
fn u15_if_case_still_runs() {
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
            .expect("if case must still run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["ok"]);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u15_while_let_still_runs() {
    let path = write_temp(
        r#"
var n = 0
public func next() -> Int? {
    if n >= 2 {
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
            .expect("while let must still run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["0", "1"]
    );
    let _ = std::fs::remove_file(&path);
}
