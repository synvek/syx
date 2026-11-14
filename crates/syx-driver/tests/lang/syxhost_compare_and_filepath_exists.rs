//! CI: SyxHost host-call returns are Int (not SyxHost) so `h < 0` is numeric;
//! FilePath static/instance `exists` must not share one recursive MIR symbol.

use std::io::Write;

fn write_temp(src: &str) -> std::path::PathBuf {
    let mut f = tempfile::NamedTempFile::new().expect("temp");
    f.write_all(src.as_bytes()).expect("write");
    f.into_temp_path().keep().expect("keep")
}

#[test]
fn syxhost_tcp_connect_compare_runs() {
    let path = write_temp(
        r#"
func main() {
    let h = SyxHost.tcp_connect("127.0.0.1", 1)
    if h < 0 { print("neg") } else { print("ok") }
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("SyxHost.tcp_connect compare must run");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        vec!["neg"],
        "{printed:?}"
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn filepath_static_exists_no_stack_overflow() {
    let path = write_temp(
        r#"
import System
func main() {
    print(FilePath.exists(path: "/"))
}
"#,
    );
    let (code, printed) =
        syx_driver::run_file_printed(&path, &syx_driver::DriverOptions::default())
            .expect("FilePath.exists must run without stack overflow");
    assert_eq!(code, 0, "exit {code}, printed={printed:?}");
    assert_eq!(
        printed.iter().map(|s| s.trim()).collect::<Vec<_>>(),
        vec!["true"],
        "{printed:?}"
    );
    let _ = std::fs::remove_file(&path);
}
