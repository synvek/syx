//! U-27: `let` tuple MemberAssign → S060 (positional / labeled / nested; var stays green).

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
fn u27_let_tuple_positional_is_s060() {
    assert!(
        has_error(
            r#"
public func main() {
    let t = (1, 2)
    t.0 = 3
}
"#,
            "S060"
        ),
        "let tuple .0= must be S060"
    );
}

#[test]
fn u27_let_tuple_labeled_is_s060() {
    assert!(
        has_error(
            r#"
public func main() {
    let t = (x: 1, y: 2)
    t.x = 3
}
"#,
            "S060"
        ),
        "let tuple .x= must be S060"
    );
}

#[test]
fn u27_let_nested_tuple_is_s060() {
    assert!(
        has_error(
            r#"
struct Box { var pair: (Int, Int) }
public func main() {
    let b = Box(pair: (1, 2))
    b.pair.0 = 3
}
"#,
            "S060"
        ),
        "let nested struct.tuple.0= must be S060"
    );
}

#[test]
fn u27_var_tuple_ok() {
    assert!(
        !has_error(
            r#"
public func main() {
    var t = (1, 2)
    t.0 = 3
}
"#,
            "S060"
        ),
        "var tuple .0= must not be S060"
    );
}

#[test]
fn u27_let_struct_still_s060() {
    assert!(
        has_error(
            r#"
struct Point { var x: Int }
public func main() {
    let p = Point(x: 1)
    p.x = 3
}
"#,
            "S060"
        ),
        "let Ident struct member must still be S060"
    );
}

#[test]
fn u27_var_tuple_runs() {
    let path = write_temp(
        r#"
public func main() {
    var t = (1, 2)
    t.0 = 3
    print(t.0)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("var tuple member must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["3"]
    );
    let _ = std::fs::remove_file(&path);
}
