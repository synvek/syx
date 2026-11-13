//! U-0: FilePath.joining keeps FilePath through chain + reassign (R-47 residual).

use std::io::Write;

fn write_temp(src: &str) -> std::path::PathBuf {
    let mut f = tempfile::NamedTempFile::new().expect("temp");
    f.write_all(src.as_bytes()).expect("write");
    f.into_temp_path().keep().expect("keep")
}

#[test]
fn u0_filepath_joining_chain_and_reassign() {
    let path = write_temp(
        r#"
import System
public func main() {
    let a = FilePath(rawValue: "/tmp")
    let b = a.joining(other: "x")
    let c = b.joining(other: "y").normalized()
    print(b.rawValue())
    print(c.rawValue())
    var p = FilePath(rawValue: "/a")
    p = p.joining(other: "b")
    print(p.rawValue())
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("FilePath joining chain/reassign must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        vec!["/tmp/x", "/tmp/x/y", "/a/b"],
        "{printed:?}"
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn u0_local_struct_joining_chain_and_reassign() {
    let path = write_temp(
        r#"
struct Path {
    var tag: String
    public func joining(other: String) -> Path {
        return Path(tag: self.tag + "/" + other)
    }
    public func normalized() -> Path {
        return Path(tag: self.tag)
    }
}
public func main() {
    let a = Path(tag: "/tmp")
    let b = a.joining(other: "x")
    let c = b.joining(other: "y").normalized()
    print(c.tag)
    var p = Path(tag: "/a")
    p = p.joining(other: "b")
    print(p.tag)
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("local struct joining chain/reassign must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        vec!["/tmp/x/y", "/a/b"],
        "{printed:?}"
    );
    let _ = std::fs::remove_file(&path);
}
