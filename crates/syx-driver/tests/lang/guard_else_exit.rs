//! U-18: `guard let` else must exit → S027 (≠ MainActor S020; ≠ bare guard If).

use syx_diagnostics::SourceFile;
use syx_parser::Parser;
use syx_semantic::SemanticAnalyzer;

use std::io::Write;

fn write_temp(src: &str) -> std::path::PathBuf {
    let mut f = tempfile::NamedTempFile::new().expect("temp");
    f.write_all(src.as_bytes()).expect("write");
    f.into_temp_path().keep().expect("keep")
}

fn has_error(src: &str, code: &str) -> bool {
    let file = SourceFile::new("test.sy", src);
    let module = Parser::parse(file).unwrap();
    match SemanticAnalyzer::analyze(&module) {
        Err(d) => d.code.as_deref() == Some(code),
        Ok(a) => a.errors.iter().any(|e| e.code.as_deref() == Some(code)),
    }
}

#[test]
fn u18_print_only_else_is_s027() {
    assert!(
        has_error(
            r#"
public func main() {
    let a: Int? = nil
    guard let x = a else {
        print("e")
    }
    print("after")
}
"#,
            "S027"
        ),
        "print-only else must be S027"
    );
}

#[test]
fn u18_empty_else_is_s027() {
    assert!(
        has_error(
            r#"
public func main() {
    let a: Int? = nil
    guard let x = a else {
    }
    print("after")
}
"#,
            "S027"
        ),
        "empty else must be S027"
    );
}

#[test]
fn u18_return_else_ok_and_binds() {
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
            .expect("return else must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(printed.iter().map(|s| s.trim()).collect::<Vec<_>>(), ["7"]);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u18_throw_else_ok() {
    assert!(
        !has_error(
            r#"
public func f() throws {
    let a: Int? = nil
    guard let x = a else {
        throw "nil"
    }
    print(x)
}
public func main() {
    do {
        try f()
    } catch {
        print("caught")
    }
}
"#,
            "S027"
        ),
        "throw else must not be S027"
    );
}

#[test]
fn u18_break_else_ok() {
    assert!(
        !has_error(
            r#"
public func main() {
    let a: Int? = nil
    while true {
        guard let x = a else {
            break
        }
        print(x)
    }
    print("done")
}
"#,
            "S027"
        ),
        "break else must not be S027"
    );
}

#[test]
fn u18_continue_else_ok() {
    assert!(
        !has_error(
            r#"
public func main() {
    var i = 0
    while i < 2 {
        i = i + 1
        let a: Int? = nil
        guard let x = a else {
            continue
        }
        print(x)
    }
    print("done")
}
"#,
            "S027"
        ),
        "continue else must not be S027"
    );
}

#[test]
fn u18_comma_guard_let_else_must_exit() {
    assert!(
        has_error(
            r#"
public func main() {
    let a: Int? = 1
    let b: Int? = nil
    guard let x = a, let y = b else {
        print("else")
    }
    print("after")
}
"#,
            "S027"
        ),
        "comma guard let print-only else must be S027"
    );
}

#[test]
fn u18_comma_guard_let_return_ok() {
    let path = write_temp(
        r#"
public func main() {
    let a: Int? = 1
    let b: Int? = 2
    guard let x = a, let y = b else {
        return
    }
    print(x)
    print(y)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("comma guard let return else must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["1", "2"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u18_bare_guard_empty_else_not_s027() {
    // U-17 bare guard → If; U-18 non-claim.
    assert!(
        !has_error(
            r#"
public func main() {
    guard false else {
    }
    print("after")
}
"#,
            "S027"
        ),
        "bare guard must not emit S027"
    );
}
