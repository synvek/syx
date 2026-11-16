//! U-26: Ch15/Ch28 must not claim init-order / access-level as S020 (tree emits none).

use syx_diagnostics::SourceFile;
use syx_parser::Parser;
use syx_semantic::SemanticAnalyzer;

use std::io::Write;

fn write_temp(src: &str) -> std::path::PathBuf {
    let mut f = tempfile::NamedTempFile::new().expect("temp");
    f.write_all(src.as_bytes()).expect("write");
    f.into_temp_path().keep().expect("keep")
}

fn semantic_has(src: &str, code: &str) -> bool {
    let file = SourceFile::new("test.sy", src);
    let module = Parser::parse(file).unwrap();
    match SemanticAnalyzer::analyze(&module) {
        Err(d) => d.code.as_deref() == Some(code),
        Ok(a) => a.errors.iter().any(|e| e.code.as_deref() == Some(code)),
    }
}

fn check_err_contains(src: &str, code: &str) -> bool {
    let path = write_temp(src);
    let hit =
        match syx_driver::check_file_with_options(&path, &syx_driver::DriverOptions::default()) {
            Ok(_) => false,
            Err(diags) => diags
                .iter()
                .any(|d| d.code.as_deref() == Some(code) || format!("{d:?}").contains(code)),
        };
    let _ = std::fs::remove_file(&path);
    hit
}

#[test]
fn u26_init_order_not_s020() {
    let src = r#"
class Base {
    var n: Int
    init(n: Int) { self.n = n }
}
class Child: Base {
    var m: Int
    init(n: Int, m: Int) {
        print(self.n)
        self.m = m
        super.init(n: n)
    }
}
public func main() {
    let _ = Child(n: 1, m: 2)
}
"#;
    assert!(
        !semantic_has(src, "S020"),
        "init-order probe must not emit S020"
    );
    assert!(
        !check_err_contains(src, "S020"),
        "init-order check path must not emit S020"
    );
}

#[test]
fn u26_access_level_not_s020() {
    let src = r#"
internal class Box {
    public var n: Int
    init(n: Int) { self.n = n }
}
public func main() {
    let b = Box(n: 1)
    print(b.n)
}
"#;
    assert!(
        !semantic_has(src, "S020"),
        "access-level probe must not emit S020"
    );
    assert!(
        !check_err_contains(src, "S020"),
        "access-level check path must not emit S020"
    );
}

#[test]
fn u26_mutating_still_s060() {
    let src = r#"
public func main() {
    let a: Array<Int> = []
    a.append(1)
}
"#;
    assert!(
        semantic_has(src, "S060"),
        "mutating on let must still be S060"
    );
    assert!(
        !semantic_has(src, "S020"),
        "mutating on let must not be S020"
    );
}

#[test]
fn u26_await_still_a002() {
    let src = r#"
func f() -> Int {
    return await g()
}
async func g() -> Int { return 1 }
public func main() { print(1) }
"#;
    assert!(
        check_err_contains(src, "A002"),
        "await outside async must still be A002"
    );
    assert!(
        !check_err_contains(src, "S020"),
        "await outside async must not be S020"
    );
}
