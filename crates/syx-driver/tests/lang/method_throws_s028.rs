//! U-20: type-member `throws` flag for S028 (≠ top-level-only U-19; ≠ MainActor S020).

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
fn u20_throws_method_throw_ok() {
    assert!(
        !has_error(
            r#"
class C {
    func f() throws {
        throw "x"
    }
}
public func main() {
    do {
        try C().f()
    } catch {
        print("c")
    }
}
"#,
            "S028"
        ),
        "throws method throw must not be S028"
    );
}

#[test]
fn u20_non_throws_method_is_s028() {
    assert!(
        has_error(
            r#"
class C {
    func f() {
        throw "x"
    }
}
public func main() {
    C().f()
}
"#,
            "S028"
        ),
        "non-throws method throw must be S028"
    );
}

#[test]
fn u20_throws_method_do_catch_runs() {
    let path = write_temp(
        r#"
class C {
    func f() throws {
        throw "x"
    }
}
public func main() {
    do {
        try C().f()
    } catch {
        print("caught")
    }
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("throws method + do/catch must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["caught"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u20_toplevel_throws_still_ok() {
    assert!(
        !has_error(
            r#"
func f() throws {
    throw "x"
}
public func main() {
    do {
        try f()
    } catch {
        print("c")
    }
}
"#,
            "S028"
        ),
        "top-level throws must stay green (U-19)"
    );
}

#[test]
fn u20_init_throws_ok() {
    assert!(
        !has_error(
            r#"
class C {
    init() throws {
        throw "x"
    }
}
public func main() {
    do {
        try C()
    } catch {
        print("c")
    }
}
"#,
            "S028"
        ),
        "init throws throw must not be S028"
    );
}
