//! U-29: nested enum/tuple MemberAssign → S060 (enum↔enum / enum→tuple / tuple→enum).

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
fn u29_let_enum_to_enum_is_s060() {
    assert!(
        has_error(
            r#"
enum Inner { case wrap(payload: Int) }
enum Outer { case nest(inner: Inner) }
public func main() {
    let o = Outer.nest(inner: Inner.wrap(payload: 1))
    o.inner.payload = 9
}
"#,
            "S060"
        ),
        "let enum→enum nested assign must be S060"
    );
}

#[test]
fn u29_let_enum_to_tuple_is_s060() {
    assert!(
        has_error(
            r#"
enum E { case pair(t: (Int, Int)) }
public func main() {
    let e = E.pair(t: (1, 2))
    e.t.0 = 3
}
"#,
            "S060"
        ),
        "let enum→tuple nested assign must be S060"
    );
}

#[test]
fn u29_let_tuple_to_enum_is_s060() {
    assert!(
        has_error(
            r#"
enum E { case wrap(payload: Int) }
public func main() {
    let t = (E.wrap(payload: 1), 2)
    t.0.payload = 9
}
"#,
            "S060"
        ),
        "let tuple→enum nested assign must be S060"
    );
}

#[test]
fn u29_let_ident_enum_still_s060() {
    assert!(
        has_error(
            r#"
enum Box { case wrap(payload: Int) }
public func main() {
    let e = Box.wrap(payload: 1)
    e.payload = 9
}
"#,
            "S060"
        ),
        "let Ident enum must still be S060"
    );
}

#[test]
fn u29_let_dataclass_to_enum_still_s060() {
    assert!(
        has_error(
            r#"
enum E { case wrap(payload: Int) }
struct W { var e: E }
public func main() {
    let w = W(e: E.wrap(payload: 1))
    w.e.payload = 9
}
"#,
            "S060"
        ),
        "let dataclass→enum must still be S060"
    );
}

#[test]
fn u29_var_enum_to_enum_ok() {
    assert!(
        !has_error(
            r#"
enum Inner { case wrap(payload: Int) }
enum Outer { case nest(inner: Inner) }
public func main() {
    var o = Outer.nest(inner: Inner.wrap(payload: 1))
    o.inner.payload = 9
}
"#,
            "S060"
        ),
        "var enum→enum must not be S060"
    );
}

#[test]
fn u29_var_enum_to_enum_runs() {
    let path = write_temp(
        r#"
enum Inner { case wrap(payload: Int) }
enum Outer { case nest(inner: Inner) }
public func main() {
    var o = Outer.nest(inner: Inner.wrap(payload: 1))
    o.inner.payload = 9
    print(o.inner.payload)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("var enum→enum must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["9"]
    );
    let _ = std::fs::remove_file(&path);
}
