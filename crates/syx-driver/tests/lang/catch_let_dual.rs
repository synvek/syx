//! U-6: dual-accept Swift-style `catch let e` while keeping `catch e` / `catch (e)` / bare.
//! U-19: throw sites live in `throws` helpers (S028); catch-let surface unchanged.

use std::io::Write;

fn write_temp(src: &str) -> std::path::PathBuf {
    let mut f = tempfile::NamedTempFile::new().expect("temp");
    f.write_all(src.as_bytes()).expect("write");
    f.into_temp_path().keep().expect("keep")
}

#[test]
fn u6_catch_let_runs() {
    let path = write_temp(
        r#"
func boom() throws {
    throw "let"
}
public func main() {
    do {
        try boom()
    } catch let e {
        print(e)
    }
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("catch let e must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["let"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u6_catch_let_typed_runs() {
    let path = write_temp(
        r#"
func boom() throws {
    throw "typed"
}
public func main() {
    do {
        try boom()
    } catch let e: String {
        print(e)
    }
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("catch let e: String must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["typed"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u6_catch_parenless_still_runs() {
    let path = write_temp(
        r#"
func boom() throws {
    throw "name"
}
public func main() {
    do {
        try boom()
    } catch e {
        print(e)
    }
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("catch e must still run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["name"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u6_catch_paren_still_runs() {
    let path = write_temp(
        r#"
func boom() throws {
    throw "paren"
}
public func main() {
    do {
        try boom()
    } catch (e) {
        print(e)
    }
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("catch (e) must still run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["paren"]
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u6_catch_bare_still_runs() {
    let path = write_temp(
        r#"
func boom() throws {
    throw "bare"
}
public func main() {
    do {
        try boom()
    } catch {
        print(error)
    }
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("bare catch must still run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        ["bare"]
    );
    let _ = std::fs::remove_file(&path);
}
